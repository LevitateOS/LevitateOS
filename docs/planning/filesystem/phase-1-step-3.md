# Phase 1 Step 3: Prepare Test Disk Image

**Phase:** 1 (Discovery)  
**Step:** 3  
**UoW Count:** 1 (this file)

---

## Goal

Format `tinyos_disk.img` as FAT32 with test files for verification.

---

## Tasks

1. Backup existing disk image (if needed)
2. Create fresh FAT32 image
3. Add test files
4. Verify with host tools

---

## Commands

```bash
# Create fresh 16MB FAT32 image
dd if=/dev/zero of=tinyos_disk.img bs=1M count=16

# Format as FAT32
mkfs.vfat -F 32 -n LEVITATE tinyos_disk.img

# Mount and add test files
mkdir -p /tmp/levitate_mount
sudo mount -o loop tinyos_disk.img /tmp/levitate_mount

# Create test content
echo "Hello from LevitateOS!" | sudo tee /tmp/levitate_mount/hello.txt
sudo mkdir /tmp/levitate_mount/testdir
echo "Nested file content" | sudo tee /tmp/levitate_mount/testdir/nested.txt
echo "Binary test" | sudo tee /tmp/levitate_mount/test.bin

# Unmount
sudo umount /tmp/levitate_mount
rmdir /tmp/levitate_mount

# Verify
file tinyos_disk.img
# Expected: DOS/MBR boot sector, ... FAT (32 bit)
```

---

## Expected Disk Contents

```
/
├── hello.txt      (23 bytes)
├── test.bin       (12 bytes)
└── testdir/
    └── nested.txt (20 bytes)
```

---

## Exit Criteria

- [ ] Disk image is FAT32 formatted
- [ ] Contains test files at known paths
- [ ] `file` command confirms FAT32

→ **Next:** Phase 2 (Design)
