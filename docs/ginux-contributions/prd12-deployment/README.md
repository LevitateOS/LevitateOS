# PRD12 actual deployment fixture

The Recipe CLI case
`target_metadata::cli_target_metadata_survives_publication_reimport_and_erofs_production`
extracts a package, retains/reuses checkpoints, publishes/reimports/composes its
metadata, inspects the real target tar and invokes the actual distro-builder
EROFS producer. It requires `RECIPE_TEST_BUILDER` and `RECIPE_TEST_REPO_ROOT`.
Set `RECIPE_TEST_ARTIFACTS` to an existing private evidence directory to retain
its exact input, manifests, products and `prepared/target.erofs`.

Deployment is a separate real consumer of **that same EROFS image**. A host
kernel without EROFS zstd cannot substitute an uncompressed build or fsck
extraction for this check. `vm-init.c` runs the actual recstrap binary inside a
disposable diskless libvirt guest, with all installer guards enabled, on a fresh
3-GiB-limit tmpfs (the tiny data fixture does not allocate 3 GiB). It checks every
required mode/owner, setuid/setgid/sticky bits, hardlink/symlink identity,
capability bytes, payload and the producer's deliberately normalized epoch
mtime, then unmounts and powers off. No network, disk or host-clock mutation.

Build from the workspace, using already prepared kernel artifacts:

```sh
cc -O2 -Wall -Wextra -Werror \
  docs/ginux-contributions/prd12-deployment/vm-init.c -o .artifacts/prd12-vm-init
python3 docs/ginux-contributions/prd12-deployment/build-vm.py \
  --output .artifacts/prd12-vm \
  --image /absolute/path/to/the/retained/prepared/target.erofs \
  --recstrap /absolute/path/to/the/exact/recstrap \
  --controller .artifacts/prd12-vm-init \
  --kernel .artifacts/kernel/levitate/current/staging/boot/vmlinuz \
  --name levitate-prd12-metadata
virsh --connect qemu:///session create .artifacts/prd12-vm/domain.xml
```

The producer creates `inputs.json` with every exact input/library/kernel hash,
`domain.xml`, initramfs and kernel snapshot. Run as the session's owner; generated
output directories and console log must be writable by that user. On SELinux
hosts the generated kernel/initramfs need the same `virt_content_t` content label
as the other read-only VM boot artifacts. This affects only generated fixtures.

Wait for `PRD12_VM PASS` and guest disappearance from
`virsh --connect qemu:///session list --all`. A created/running domain is not a
passing receipt. Retain the unedited console and exact input hashes. Compare its
printed recstrap/image hashes with the selected producer result before reusing
a receipt. The PID-1 controller refuses execution without its dedicated guest
kernel marker. Do not execute it as a host program.

These are metadata/backend acceptance guests, not distro installation or boot
acceptance. No kernel build or source-publication decision is implied.
