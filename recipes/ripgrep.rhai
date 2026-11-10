// Upstream's static x86_64 Linux musl release works on both OS variants.
// The archive digest is published with the release; payload digests below are
// from that verified archive. No host package manager or compiler is involved.
const RECIPE_API = 2;
const NEEDS = #{
    outputs: #{ "package": "directory" },
    network: ["github.com:443", "release-assets.githubusercontent.com:443"],
    network_max_bytes: 8388608,
    timeout_secs: 300,
};

let ctx = #{
    name: "ripgrep",
    version: "15.2.0",
    description: "Fast recursive text search with rg",
    sha256: "33e15bcf1624b25cdd2a55813a47a2f95dbe126268203e76aa6a585d1e7b149c",
    package_root: join_path(RUNTIME.paths.output, "package"),
    files: [
        #{ source: "rg", target: "bin/rg", mode: 0o755,
           sha256: "e62198eb19b136b88c330af83647b5a962cb99b6b1f066758568f12de1974849" },
        #{ source: "doc/rg.1", target: "share/man/man1/rg.1", mode: 0o644,
           sha256: "2b0db3d436bc1b79c2da556bb75ef1797c00476208c1fe04e0f6e012690912bf" },
        #{ source: "COPYING", target: "share/licenses/ripgrep/COPYING", mode: 0o644,
           sha256: "01c266bced4a434da0051174d6bee16a4c82cf634e2679b6155d40d75012390f" },
        #{ source: "LICENSE-MIT", target: "share/licenses/ripgrep/LICENSE-MIT", mode: 0o644,
           sha256: "0f96a83840e146e43c0ec96a22ec1f392e0680e6c1226e6f3ba87e0740af850f" },
        #{ source: "UNLICENSE", target: "share/licenses/ripgrep/UNLICENSE", mode: 0o644,
           sha256: "7e12e5df4bae12cb21581ba157ced20e1986a0508dd10d0e8a4ab9a4cf94e85c" },
    ],
};
ctx.release = "ripgrep-" + ctx.version + "-x86_64-unknown-linux-musl";
ctx.archive = join_path(RUNTIME.paths.build, ctx.release + ".tar.gz");
ctx.unpacked = join_path(RUNTIME.paths.build, "unpacked");
ctx.source = join_path(ctx.unpacked, ctx.release);
ctx.url = "https://github.com/BurntSushi/ripgrep/releases/download/" +
    ctx.version + "/" + ctx.release + ".tar.gz";

fn is_acquired(ctx) {
    if !is_file(ctx.archive) { return needs_work(ctx, "download ripgrep " + ctx.version); }
    verify_sha256(ctx.archive, ctx.sha256);
    ready(ctx)
}

fn acquire(ctx) {
    download(ctx.url, ctx.archive, ctx.sha256);
    ctx
}

fn is_built(ctx) {
    for file in ctx.files {
        let path = join_path(ctx.source, file.source);
        if !is_file(path) { return needs_work(ctx, "extract ripgrep " + ctx.version); }
        verify_sha256(path, file.sha256);
    }
    ready(ctx)
}

fn build(ctx) {
    verify_sha256(ctx.archive, ctx.sha256);
    if exists(ctx.unpacked) { rm(ctx.unpacked); }
    mkdir(ctx.unpacked);
    extract(ctx.archive, ctx.unpacked);
    ctx
}

fn is_installed(ctx) {
    for file in ctx.files {
        let path = join_path(ctx.package_root, file.target);
        if !is_file(path) { return needs_work(ctx, "install ripgrep " + ctx.version); }
        verify_sha256(path, file.sha256);
    }
    ctx.outputs["package"] = ctx.package_root;
    ready(ctx)
}

fn install(ctx) {
    if exists(ctx.package_root) { rm(ctx.package_root); }
    for file in ctx.files {
        let path = join_path(ctx.package_root, file.target);
        mkdir(dirname(path));
        copy_file(join_path(ctx.source, file.source), path);
        chmod(path, file.mode);
    }
    ctx.outputs["package"] = ctx.package_root;
    ctx
}

fn remove(ctx) {
    if exists(ctx.package_root) { rm(ctx.package_root); }
    ctx.outputs = #{};
    ctx
}
