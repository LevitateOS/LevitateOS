#!/usr/bin/env python3
"""Explicit x86_64 release operations. No system builds, re-signing or VM runs.

GitHub CLI owns remote release operations; distro-builder owns cryptography and
transport; Recipe/Ginux owns public payload acquisition. Never use an index key
in an acquisition process. Only small public metadata is committed/deployed.
"""
import argparse
import base64
import contextlib
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import tempfile
import time
import tomllib

REPOSITORY = 'LevitateOS/LevitateOS'
METADATA = Path('release/updates/site')
MAX_SITE = 256 * 1024 * 1024
SECRET_ENV = ('GH_TOKEN', 'GITHUB_TOKEN', 'RECAB_INDEX_KEY_BASE64',
              'RECAB_LEVITATE_INDEX_KEY_BASE64', 'RECAB_ACORN_INDEX_KEY_BASE64')


def require(ok, message):
    if not ok:
        raise RuntimeError(message)


def run(*args, **kwargs):
    return subprocess.run([str(a) for a in args], check=True, **kwargs)


def sha(path):
    with path.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def load(path):
    require(path.is_file() and not path.is_symlink() and path.stat().st_size <= 1024 * 1024,
            f'missing, unsafe or oversized metadata: {path}')
    return json.loads(path.read_bytes())


class Publisher:
    def __init__(self, args):
        self.args = args
        self.repo = args.repo_root.resolve(strict=True)
        self.builder = args.builder.resolve(strict=True) if args.builder else None

    def build(self, *args):
        require(self.builder, 'supply --builder or use cargo xtask release')
        # gh tokens stay with gh, never Recipe or the metadata signing process.
        env = {k: v for k, v in os.environ.items() if k not in SECRET_ENV}
        result = run(self.builder, '--repo-root', self.repo, 'release', *args,
                     env=env, stdout=subprocess.PIPE, text=True)
        return json.loads(result.stdout)

    def api(self, path, data=None, missing=False):
        command = ['gh', 'api', '--hostname', 'github.com', '-H', 'X-GitHub-Api-Version: 2026-03-10',
                   f'repos/{REPOSITORY}/{path}']
        if data is not None:
            command += ['--method', 'POST' if path == 'releases' else 'PATCH', '--input', '-']
        result = subprocess.run(command, input=json.dumps(data) if data is not None else None,
                                text=True, capture_output=True)
        if missing and result.returncode and 'HTTP 404' in result.stderr:
            return None
        require(result.returncode == 0, f'GitHub request failed for {path}: {result.stderr[:2048]}')
        return json.loads(result.stdout)

    def assets(self, release_id):
        value = run('gh', 'api', '--hostname', 'github.com', '--paginate', '--slurp',
                    f'repos/{REPOSITORY}/releases/{release_id}/assets?per_page=100',
                    stdout=subprocess.PIPE, text=True)
        return [a for page in json.loads(value.stdout) for a in page]

    def inspect(self, distro, directory):
        return self.build('inspect-publication', distro, directory)

    def remote(self, distro, directory, representative=False):
        return self.build('verify-remote', distro, directory,
                          *(['--representative'] if representative else []))

    def config(self, distro):
        require(distro in ('levitate', 'acorn'), 'only levitate/acorn x86_64 releases are supported')
        config = tomllib.loads((self.repo / f'distro/variants/{distro}/distro.toml').read_text())
        release = config['release']
        require(release.get('production') and release.get('source'),
                f'{distro} production origin/root/delegation is not configured; defaults remain disabled')
        require(release['channel'] == 'stable', 'production release identity must use stable channel')
        require(release['production']['metadata_base'] == 'https://levitateos.github.io/LevitateOS/updates/'
                and release['production']['asset_prefix'] == f'https://github.com/{REPOSITORY}/releases/download/',
                'confirm and configure the approved canonical Pages/Release origins first')
        return config

    def immutable_release(self, record, draft=False):
        value = self.api('releases/tags/' + record['tag'], missing=draft)
        if value is not None:
            require(value['tag_name'] == record['tag'] and value['target_commitish'] == record['source_commit'],
                    'existing release tag/source conflicts with publication')
            if not value['draft']:
                require(value.get('immutable') is True, 'payload release must be immutable')
                target = self.api('git/ref/tags/' + record['tag'])['object']
                if target['type'] == 'tag':
                    target = self.api('git/tags/' + target['sha'])['object']
                require(target['type'] == 'commit' and target['sha'] == record['source_commit'],
                        'immutable tag does not identify the recorded source commit')
        return value

    def upload(self, distro, prepared):
        self.config(distro)
        record = self.inspect(distro, prepared)
        # Verify every local part and the whole signed artifact before any API mutation.
        manifest = load(prepared / 'release.json')
        objects = {}
        for artifact in manifest['artifacts'].values():
            whole = hashlib.sha256()
            total = 0
            for part in record['entry']['parts'][artifact['path']]:
                name = part['url'].rsplit('/', 1)[1]
                path = prepared / 'assets' / name
                require(path.is_file() and not path.is_symlink() and path.stat().st_size == part['size']
                        and sha(path) == part['sha256'], 'local transport part was altered')
                objects[name] = path
                with path.open('rb') as f:
                    while chunk := f.read(128 * 1024):
                        whole.update(chunk)
                        total += len(chunk)
            require(total == artifact['size'] and whole.hexdigest() == artifact['sha256'],
                    'transport assembly differs from release-role-signed artifact')
        for name in ('publication.json', 'release.json', 'release.sig', 'release-keys.json'):
            path = prepared / name
            require(path.is_file() and not path.is_symlink(), 'missing public publication metadata')
            objects[name] = path
        require(len(objects) <= 1000, 'publication exceeds GitHub asset count')
        settings = self.api('immutable-releases')
        require(settings.get('enabled') is True, 'enable repository immutable releases before uploading')
        release = self.immutable_release(record, draft=True)
        if release is None:
            release = self.api('releases', {'tag_name': record['tag'], 'target_commitish': record['source_commit'],
                'name': record['tag'], 'body': f"recab release {record['id']}; discovery is managed by signed update metadata.",
                'draft': True, 'prerelease': True})
        existing = {a['name']: a for a in self.assets(release['id'])}
        require(set(existing) <= set(objects), 'release has unexpected assets; refusing to alter it')
        for name, path in objects.items():
            if name not in existing:
                require(release['draft'], 'immutable release is missing a required asset')
                run('gh', 'release', 'upload', record['tag'], path, '--repo', 'https://github.com/' + REPOSITORY)
        actual = {a['name']: a for a in self.assets(release['id'])}
        require(set(actual) == set(objects), 'uploaded asset set differs')
        for name, path in objects.items():
            asset = actual[name]
            require(asset['state'] == 'uploaded' and asset['size'] == path.stat().st_size
                    and asset.get('digest') == 'sha256:' + sha(path),
                    'server asset size/digest conflicts; never clobber or repair a published asset')
        if release['draft']:
            self.api(f"releases/{release['id']}", {'draft': False, 'make_latest': 'false'})
        self.immutable_release(record)
        # Public, token-free acquisition through Recipe before it becomes discoverable.
        self.remote(distro, prepared)
        print(json.dumps({'payload_tag': record['tag'], 'release_id': record['id'],
                          'public_payloads_verified': True, 'production_enabled': False}))

    def download_metadata(self, distro, tag, output):
        require(re.fullmatch(r'os-(levitate|acorn)-x86_64-(base|desktop)-[1-9][0-9]*-[0-9a-f]{16}', tag),
                'invalid explicit payload tag')
        output.mkdir(parents=True, exist_ok=True)
        # Only four bounded public objects; payload images stay on Releases.
        release = self.api('releases/tags/' + tag)
        assets = {a['name']: a for a in self.assets(release['id'])}
        for name in ('publication.json', 'release.json', 'release.sig', 'release-keys.json'):
            require(name in assets and assets[name]['size'] <= 1024 * 1024,
                    'missing/oversized public release metadata')
            path = output / name
            with path.open('wb') as file:
                run('gh', 'api', '--hostname', 'github.com', f"repos/{REPOSITORY}/releases/assets/{assets[name]['id']}",
                    '-H', 'Accept: application/octet-stream', stdout=file)
            require(path.stat().st_size == assets[name]['size']
                    and 'sha256:' + sha(path) == assets[name].get('digest'), 'downloaded metadata digest differs')
        record = self.inspect(distro, output)
        expected = os.environ.get('RECAB_EXPECTED_PUBLICATION_SHA256')
        if expected:
            require(sha(output / 'publication.json') == expected, 'publication changed after secret-free public verification')
        require(record['tag'] == tag, 'downloaded publication tag differs')
        self.immutable_release(record)
        return record

    @contextlib.contextmanager
    def index_key(self, distro):
        env_name = 'RECAB_' + distro.upper() + '_INDEX_KEY_BASE64'
        value = os.environ.pop(env_name, None)
        require(value, f'missing protected environment secret {env_name}')
        # This CLI is Linux x86_64 only. Private key bytes live in runner tmpfs,
        # outside checkout, payload, metadata, logs and Actions artifacts.
        require(Path('/dev/shm').is_dir() and any(line.split()[4] == '/dev/shm' and line.split(' - ')[1].split()[0] == 'tmpfs' for line in Path('/proc/self/mountinfo').read_text().splitlines()), 'private tmpfs is required for index signing')
        with tempfile.TemporaryDirectory(prefix='recab-index-', dir='/dev/shm') as tmp:
            path = Path(tmp) / 'index.pk8'
            fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
            try:
                secret = base64.b64decode(value, validate=True)
                require(0 < len(secret) <= 16384, 'invalid private index key size')
                with os.fdopen(fd, 'wb') as file:
                    fd = None
                    file.write(secret)
                del secret, value
                yield path
            finally:
                if fd is not None:
                    os.close(fd)

    def feed(self, distro, audience):
        return self.repo / METADATA / 'updates' / distro / audience / 'x86_64'

    def renew(self, distro, audience, prepared=None):
        self.config(distro)
        with self.index_key(distro) as key:
            value = self.build('publish', distro, prepared or 'refresh', self.feed(distro, audience),
                               audience, '--index-key', key)
        if value['authority_expires_at'] <= int(time.time()) + 30 * 86400:
            print(f'::warning::{distro} root authorization expires within 30 days; renew offline')
        print(json.dumps({'distro': distro, 'audience': audience, **value}))

    def signing_batch(self, operation, distro=None, tag=None):
        # An explicit branch and non-force push implement a stale-base refusal.
        require(os.environ.get('GITHUB_REF') == 'refs/heads/' + os.environ.get('RECAB_DEFAULT_BRANCH', ''),
                'metadata signing runs only on the configured default branch')
        require(os.environ.get('GITHUB_REPOSITORY') == REPOSITORY, 'wrong publishing repository')
        # Each distro key may sign candidate and stable in a single refresh batch.
        # Materialize it once, then keep key paths only until both operations finish.
        if operation == 'publish':
            require(distro and tag, 'publish requires distro and immutable payload tag')
            with tempfile.TemporaryDirectory(prefix='recab-publication-') as tmp:
                prepared = Path(tmp)
                self.download_metadata(distro, tag, prepared)
                self.renew(distro, 'candidate', prepared)
        elif operation == 'promote':
            require(os.environ.get('RECAB_STABLE_VALIDATED') == 'true',
                    'stable promotion is disabled pending installed public-origin lifecycle validation')
            candidate = self.feed(distro, 'candidate')
            index = load(candidate / 'index.json')
            entries = [e for e in index['releases'] if load(candidate / 'releases' / e['release']['manifest']['sha256'] / 'publication.json')['tag'] == tag]
            require(len(entries) == 1, 'promote requires one exact current candidate ID')
            record = load(candidate / 'releases' / entries[0]['release']['manifest']['sha256'] / 'publication.json')
            evidence = self.repo / 'release/updates/validation' / (record['id'] + '.json')
            # Evidence is an explicit operator record pointing to the actual named
            # run; this guard is not itself runtime validation or a signature role.
            approval = load(evidence)
            require(approval.get('release_id') == record['id'] and approval.get('public_origin') is True
                    and approval.get('x86_64_lifecycle_passed') is True and approval.get('run_manifest_sha256')
                    and approval.get('source_commit') == record['source_commit'], 'missing matching installed validation record')
            run_file = evidence.with_suffix('.run.json')
            report = load(run_file)
            require(sha(run_file) == approval['run_manifest_sha256'] and report.get('status') == 'success'
                    and report.get('target_name') == 'ab-update' and report.get('target_kind') == 'scenario'
                    and report.get('distro_id') == distro and report.get('install_mode') == 'ab'
                    and report.get('fingerprint') and report.get('finished_at_utc'), 'validation record does not bind a successful actual ab-update run')
            require(sha(candidate / 'releases' / record['id'] / 'publication.json') == os.environ.get('RECAB_EXPECTED_PUBLICATION_SHA256'), 'candidate differs from the publicly verified immutable publication')
            self.renew(distro, 'stable', candidate / 'releases' / record['id'])
        else:
            targets = []
            for item in ('levitate', 'acorn'):
                for audience in ('candidate', 'stable'):
                    if (self.feed(item, audience) / 'index.json').is_file():
                        targets.append((item, audience))
            require(targets, 'no current feeds to refresh; explicit candidate publication must bootstrap them')
            for item in ('levitate', 'acorn'):
                selected = [a for d, a in targets if d == item]
                if not selected:
                    continue
                self.config(item)
                with self.index_key(item) as key:
                    for audience in selected:
                        value = self.build('publish', item, 'refresh', self.feed(item, audience), audience, '--index-key', key)
                        print(json.dumps({'distro': item, 'audience': audience, **value}))
                        if value['authority_expires_at'] <= int(time.time()) + 30 * 86400:
                            print(f'::warning::{item} root authorization expires within 30 days; renew offline')

    def export_site(self, output):
        require(not output.exists(), 'site export destination must be new')
        source = self.repo / METADATA
        require(source.is_dir(), 'no production metadata tree; publish candidate first')
        require(all(not p.is_symlink() for p in (self.repo / 'release', self.repo / 'release/updates', source)), 'Pages metadata roots cannot be symlinks')
        total = 0
        for item in source.rglob('*'):
            require(not item.is_symlink(), 'Pages metadata cannot contain symlinks')
            if any(p.startswith('.') for p in item.relative_to(source).parts):
                continue
            if item.is_file():
                require(item.suffix in ('.json', '.sig'), 'Pages may contain only public metadata')
                total += item.stat().st_size
                require(total <= MAX_SITE, 'Pages metadata exceeds 256 MiB safety limit')
        shutil.copytree(source, output, ignore=lambda _, names: [n for n in names if n.startswith('.')])
        (output / '.nojekyll').write_text('')
        print(json.dumps({'metadata_bytes': total, 'production_enabled': False}))

    def commit_metadata(self):
        branch = os.environ.get('RECAB_DEFAULT_BRANCH')
        require(branch in ('master', 'main'), 'invalid default branch')
        before = run('git', '-C', self.repo, 'rev-parse', 'HEAD', capture_output=True, text=True).stdout.strip()
        run('git', '-C', self.repo, 'fetch', 'origin', branch)
        current = run('git', '-C', self.repo, 'rev-parse', 'origin/' + branch, capture_output=True, text=True).stdout.strip()
        require(before == current, 'default branch advanced; rerun publication against its latest metadata')
        run('git', '-C', self.repo, 'add', '--', METADATA)
        dirty = subprocess.run(['git', '-C', str(self.repo), 'diff', '--cached', '--quiet', '--', str(METADATA)])
        if dirty.returncode == 1:
            run('git', '-C', self.repo, '-c', 'user.name=github-actions[bot]',
                '-c', 'user.email=41898282+github-actions[bot]@users.noreply.github.com',
                'commit', '-m', 'Refresh signed recab discovery metadata [skip ci]', '--', METADATA)
            run('git', '-C', self.repo, 'push', 'origin', 'HEAD:refs/heads/' + branch)
        else:
            require(dirty.returncode == 0, 'cannot inspect staged metadata')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo-root', type=Path, default=Path(__file__).resolve().parents[2])
    parser.add_argument('--builder', type=Path)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('prepare', help='verify/split existing signed artifacts, without signing or building')
    p.add_argument('distro', choices=['levitate', 'acorn']);p.add_argument('bundle', type=Path)
    p.add_argument('root', type=Path);p.add_argument('output', type=Path)
    p.add_argument('--source-commit', required=True);p.add_argument('--part-bytes', type=int, default=1024**3)
    for name in ('upload', 'verify'):
        p = sub.add_parser(name);p.add_argument('distro', choices=['levitate', 'acorn']);p.add_argument('prepared', type=Path)
    p = sub.add_parser('fetch-publication');p.add_argument('distro', choices=['levitate', 'acorn']);p.add_argument('tag');p.add_argument('output', type=Path)
    for name in ('publish', 'promote'):
        p = sub.add_parser(name);p.add_argument('distro', choices=['levitate', 'acorn']);p.add_argument('tag')
    sub.add_parser('refresh')
    p = sub.add_parser('monitor');p.add_argument('--representative', action='store_true')
    sub.add_parser('commit-metadata')
    p = sub.add_parser('export-site');p.add_argument('output', type=Path)
    args = parser.parse_args();publisher = Publisher(args)
    require(sys.platform == 'linux' and os.uname().machine == 'x86_64', 'Linux x86_64 release tooling required')
    if args.command == 'prepare':
        publisher.config(args.distro)
        print(json.dumps(publisher.build('transport', args.distro, args.bundle.resolve(), args.root.resolve(),
                                        args.output.resolve(), args.source_commit, args.part_bytes)))
    elif args.command == 'upload':publisher.upload(args.distro, args.prepared.resolve())
    elif args.command == 'fetch-publication':print(json.dumps(publisher.download_metadata(args.distro,args.tag,args.output.resolve())))
    elif args.command == 'verify':print(json.dumps(publisher.remote(args.distro, args.prepared.resolve())))
    elif args.command in ('publish', 'promote', 'refresh'):
        publisher.signing_batch(args.command, getattr(args, 'distro', None), getattr(args, 'tag', None))
    elif args.command == 'monitor':
        count = 0
        for distro in ('levitate', 'acorn'):
            for audience in ('candidate', 'stable'):
                feed = publisher.feed(distro, audience)
                if (feed / 'index.json').is_file():
                    publisher.config(distro)
                    for attempt in range(3):
                        try:
                            value = publisher.build('verify-online', distro, feed, audience,
                                                    *(['--representative'] if args.representative else []))
                            print(json.dumps(value));break
                        except subprocess.CalledProcessError:
                            if attempt == 2:raise
                            time.sleep(10)
                    count += 1
        require(count, 'no published feeds to monitor')
    elif args.command == 'export-site':publisher.export_site(args.output.resolve())
    else:publisher.commit_metadata()


if __name__ == '__main__':
    signal.signal(signal.SIGTERM, lambda *_: sys.exit(143))
    try:
        main()
    except (RuntimeError, subprocess.CalledProcessError, ValueError, OSError) as error:
        print(f'Error: {error}', file=sys.stderr)
        sys.exit(1)
