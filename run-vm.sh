#!/bin/bash
# Boot LevitateOS ISO with shared folder for development

echo "=== Inside VM, run: ==="
echo "mkdir /mnt/share && mount -t 9p -o trans=virtio share /mnt/share"
echo "/mnt/share/target/release/levitate-installer"
echo "========================"
echo ""

qemu-system-x86_64 \
    -enable-kvm \
    -m 4G \
    -cdrom LevitateOS-1.0-x86_64.iso \
    -boot d \
    -virtfs local,path=.,mount_tag=share,security_model=mapped-xattr \
    -serial file:vm.log
