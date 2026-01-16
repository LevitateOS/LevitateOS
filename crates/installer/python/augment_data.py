#!/usr/bin/env python3
"""
Convert conversation templates to training snapshots.

Reads: conversations/templates.jsonl (templates with placeholders)
Writes: training/augmented_dataset.jsonl (training snapshots)

Each template is expanded into multiple variations based on:
- Disk types (SATA, NVMe, VirtIO)
- Boot modes (UEFI, Legacy BIOS)
- Single vs multi-disk configurations
- Various user inputs (hostname, username, timezone)
"""

import json
import re
import sys
import random
from pathlib import Path
from typing import Optional
from dataclasses import dataclass, field
from itertools import product

# ============================================================================
# System Context Variations
# ============================================================================

DISK_CONFIGS = [
    # (device, size, model, type_name, type_explanation)
    ("sda", "256G", "Samsung 860 EVO", "SATA SSD", "SATA is a common interface for SSDs and hard drives. Your drive connects via a SATA cable."),
    ("sda", "500G", "Samsung 870 EVO", "SATA SSD", "SATA is a common interface for SSDs and hard drives. Your drive connects via a SATA cable."),
    ("sda", "1T", "Crucial MX500", "SATA SSD", "SATA is a common interface for SSDs and hard drives. Your drive connects via a SATA cable."),
    ("sda", "500G", "Seagate Barracuda", "SATA HDD", "SATA is a common interface for hard drives. This is a spinning disk drive - reliable but slower than SSDs."),
    ("nvme0n1", "256G", "WD Blue SN570", "NVMe SSD", "NVMe is a fast storage interface. Your drive plugs directly into the motherboard and is much faster than SATA drives."),
    ("nvme0n1", "500G", "Samsung 980", "NVMe SSD", "NVMe is a fast storage interface. Your drive plugs directly into the motherboard and is much faster than SATA drives."),
    ("nvme0n1", "1T", "Samsung 990 PRO", "NVMe SSD", "NVMe is a high-performance storage interface. This is a top-tier drive - very fast!"),
    ("nvme0n1", "2T", "WD Black SN850X", "NVMe SSD", "NVMe is a high-performance storage interface. This is a top-tier drive with lots of space!"),
    ("vda", "20G", "VirtIO Block Device", "virtual drive", "VirtIO is a virtualized storage interface - you're running in a virtual machine. It works just like a regular drive."),
    ("vda", "40G", "VirtIO Block Device", "virtual drive", "VirtIO is a virtualized storage interface - you're running in a virtual machine. It works just like a regular drive."),
    ("vda", "100G", "VirtIO Block Device", "virtual drive", "VirtIO is a virtualized storage interface - you're running in a virtual machine. It works just like a regular drive."),
]

SECONDARY_DISK_CONFIGS = [
    ("sdb", "1T", "WD Blue HDD"),
    ("sdb", "2T", "Seagate Barracuda"),
    ("nvme1n1", "500G", "Crucial P3"),
    ("sdb", "500G", "Kingston A400"),
]

BOOT_MODES = ["UEFI", "Legacy BIOS"]

HOSTNAMES = ["mypc", "laptop", "desktop", "workstation", "devbox", "homepc", "linuxbox", "levitate-pc"]
USERNAMES = ["user", "admin", "dev", "me", "main"]
TIMEZONES = [
    ("new york", "America/New_York"),
    ("los angeles", "America/Los_Angeles"),
    ("chicago", "America/Chicago"),
    ("london", "Europe/London"),
    ("berlin", "Europe/Berlin"),
    ("tokyo", "Asia/Tokyo"),
    ("sydney", "Australia/Sydney"),
    ("utc", "UTC"),
]

FILESYSTEMS = [
    ("ext4", "ext4 is the default Linux filesystem - reliable, fast, and well-tested. Great for most users.", ""),
    ("btrfs", "btrfs supports snapshots, compression, and copy-on-write. Great for advanced users who want easy backups.", "Look into 'snapper' or 'timeshift' for btrfs snapshots."),
    ("xfs", "xfs is excellent for large files and high-performance workloads. Used by many servers.", ""),
]

# ============================================================================
# Partition naming helpers
# ============================================================================

def get_partition_suffix(disk: str, part_num: int) -> str:
    """Get partition name based on disk type."""
    if disk.startswith("nvme") or disk.startswith("mmcblk"):
        return f"{disk}p{part_num}"
    else:
        return f"{disk}{part_num}"

def get_boot_mount_path(boot_mode: str) -> str:
    """Get boot mount path based on boot mode."""
    if boot_mode == "UEFI":
        return "boot/efi"
    else:
        return "boot"

def get_partition_cmd(disk: str, boot_mode: str) -> str:
    """Get partition command based on boot mode."""
    if boot_mode == "UEFI":
        return f"sgdisk -Z /dev/{disk} && sgdisk -n 1:0:+512M -t 1:ef00 -n 2:0:0 -t 2:8300 /dev/{disk}"
    else:
        return f"parted -s /dev/{disk} mklabel msdos mkpart primary ext4 1MiB 512MiB mkpart primary ext4 512MiB 100%"

def get_format_cmd(boot_part: str, root_part: str, boot_mode: str, fs: str = "ext4") -> str:
    """Get format command based on boot mode and filesystem."""
    if boot_mode == "UEFI":
        return f"mkfs.fat -F32 /dev/{boot_part} && mkfs.{fs} /dev/{root_part}"
    else:
        return f"mkfs.ext4 /dev/{boot_part} && mkfs.{fs} /dev/{root_part}"

def get_bootloader_cmd(disk: str, boot_mode: str) -> str:
    """Get bootloader install command based on boot mode."""
    if boot_mode == "UEFI":
        return "arch-chroot /mnt bootctl install"
    else:
        return f"arch-chroot /mnt grub-install --target=i386-pc /dev/{disk} && arch-chroot /mnt grub-mkconfig -o /boot/grub/grub.cfg"

# ============================================================================
# System State Tracking
# ============================================================================

@dataclass
class DiskPartition:
    name: str
    size: str
    fstype: Optional[str] = None
    mountpoint: Optional[str] = None

@dataclass
class Disk:
    device: str
    size: str
    model: str
    partitions: list = field(default_factory=list)

@dataclass
class SystemState:
    boot_mode: str = "UEFI"
    network: str = "Connected"
    hostname: str = "archiso"
    timezone: str = "not set"
    disks: list = field(default_factory=list)
    users: list = field(default_factory=list)

    def to_context(self) -> str:
        lines = ["## Current System State", ""]
        lines.append(f"- Boot mode: {self.boot_mode}")
        lines.append(f"- Network: {self.network}")
        lines.append(f"- Hostname: {self.hostname}")
        lines.append(f"- Timezone: {self.timezone}")
        lines.append("")
        lines.append("## Available Disks")
        lines.append("")

        for disk in self.disks:
            lines.append(f"- {disk.device}: {disk.size} ({disk.model})")
            for part in disk.partitions:
                fs_str = f" [{part.fstype}]" if part.fstype else ""
                mount_str = f" mounted at {part.mountpoint}" if part.mountpoint else ""
                lines.append(f"  - /dev/{part.name}: {part.size}{fs_str}{mount_str}")

        mounts = []
        for disk in self.disks:
            for part in disk.partitions:
                if part.mountpoint:
                    mounts.append((f"/dev/{part.name}", part.mountpoint))

        if mounts:
            lines.append("")
            lines.append("## Current Mounts")
            for dev, mnt in mounts:
                lines.append(f"- {dev} on {mnt}")

        if self.users:
            lines.append("")
            lines.append(f"## Existing Users: {', '.join(self.users)}")

        return "\n".join(lines)

    def apply_command(self, command: str) -> None:
        if "sgdisk" in command or "parted" in command:
            match = re.search(r'/dev/(sd[a-z]|nvme\d+n\d+|vd[a-z])', command)
            if match:
                device = f"/dev/{match.group(1)}"
                disk = self._find_disk(device)
                if disk:
                    if "-Z" in command or "mklabel" in command:
                        disk.partitions = []
                    part_matches = re.findall(r'-n\s*(\d+):([^:]*):([^\s]+)', command)
                    for part_num, start, end in part_matches:
                        if "nvme" in device or "mmcblk" in device:
                            part_name = f"{match.group(1)}p{part_num}"
                        else:
                            part_name = f"{match.group(1)}{part_num}"
                        size = end[1:] if end.startswith("+") else ("remaining" if end == "0" else end)
                        disk.partitions.append(DiskPartition(name=part_name, size=size))
                    # Handle parted mkpart
                    if "mkpart" in command:
                        disk.partitions = [
                            DiskPartition(name=get_partition_suffix(match.group(1), 1), size="512M"),
                            DiskPartition(name=get_partition_suffix(match.group(1), 2), size="remaining"),
                        ]

        if "mkfs" in command:
            for m in re.finditer(r'mkfs\.fat[^\s]*\s+/dev/(\S+)', command):
                self._set_fstype(m.group(1), "vfat")
            for m in re.finditer(r'mkfs\.ext4\s+/dev/(\S+)', command):
                self._set_fstype(m.group(1), "ext4")
            for m in re.finditer(r'mkfs\.btrfs\s+/dev/(\S+)', command):
                self._set_fstype(m.group(1), "btrfs")
            for m in re.finditer(r'mkfs\.xfs\s+/dev/(\S+)', command):
                self._set_fstype(m.group(1), "xfs")

        for m in re.finditer(r'mount\s+/dev/(\S+)\s+(/\S+)', command):
            self._set_mountpoint(m.group(1), m.group(2))

        if "umount" in command:
            for disk in self.disks:
                for part in disk.partitions:
                    part.mountpoint = None

        m = re.search(r"echo\s+['\"]([^'\"]+)['\"]\s*>\s*/mnt/etc/hostname", command)
        if m:
            self.hostname = m.group(1)

        m = re.search(r'/usr/share/zoneinfo/(\S+)\s+/mnt/etc/localtime', command)
        if m:
            self.timezone = m.group(1)

        m = re.search(r'useradd\s+.*\s+(\w+)\s*$', command)
        if m and m.group(1) not in self.users:
            self.users.append(m.group(1))

    def _find_disk(self, device: str) -> Optional[Disk]:
        for disk in self.disks:
            if disk.device == device:
                return disk
        return None

    def _set_fstype(self, part_name: str, fstype: str) -> None:
        for disk in self.disks:
            for part in disk.partitions:
                if part.name == part_name:
                    part.fstype = fstype

    def _set_mountpoint(self, part_name: str, mountpoint: str) -> None:
        for disk in self.disks:
            for part in disk.partitions:
                if part.name == part_name:
                    part.mountpoint = mountpoint

# ============================================================================
# Template Expansion
# ============================================================================

def fill_placeholders(text: str, ctx: dict) -> str:
    """Replace placeholders in text with context values."""
    for key, value in ctx.items():
        text = text.replace(f"{{{key}}}", str(value))
    return text

def generate_variations(template: dict) -> list:
    """Generate all variations of a template."""
    variations = []

    # Determine which variations this template needs
    needs_secondary = "{SECONDARY_DISK}" in json.dumps(template) or "{BIGGER" in json.dumps(template)
    needs_filesystem = "{REQUESTED_FS}" in json.dumps(template)

    # Check if template is text-only (no commands)
    turns = template.get("turns", [])
    is_text_only = all(turn.get("type") == "text" for turn in turns)

    # Reduce variations to prevent overfitting
    # Keep variations similar for text/command to maintain ~50% balance
    num_disk_samples = 2

    # Sample disk configs - keep it minimal for diversity without repetition
    disk_samples = random.sample(DISK_CONFIGS, min(num_disk_samples, len(DISK_CONFIGS)))
    boot_mode_samples = BOOT_MODES

    for disk_config in disk_samples:
        disk, size, model, type_name, type_explanation = disk_config

        for boot_mode in boot_mode_samples:
            # Skip BIOS with NVMe (rare/unrealistic)
            if boot_mode == "Legacy BIOS" and disk.startswith("nvme"):
                continue

            boot_part = get_partition_suffix(disk, 1)
            root_part = get_partition_suffix(disk, 2)

            # Base context
            ctx = {
                "PRIMARY_DISK": disk,
                "DISK_SIZE": size,
                "DISK_MODEL": model,
                "DISK_TYPE_NAME": type_name,
                "DISK_TYPE_EXPLANATION": type_explanation,
                "BOOT_MODE": boot_mode,
                "BOOT_PARTITION": boot_part,
                "ROOT_PARTITION": root_part,
                "BOOT_MOUNT_PATH": get_boot_mount_path(boot_mode),
                "PARTITION_DISK_CMD": get_partition_cmd(disk, boot_mode),
                "FORMAT_CMD": get_format_cmd(boot_part, root_part, boot_mode, "ext4"),
                "BOOTLOADER_INSTALL": get_bootloader_cmd(disk, boot_mode),
                "PARTITION_LAYOUT_DESC": "EFI and root partitions" if boot_mode == "UEFI" else "boot and root partitions",
            }

            # User inputs - pick random samples
            hostname = random.choice(HOSTNAMES)
            username = random.choice(USERNAMES)
            tz_input, tz_path = random.choice(TIMEZONES)

            ctx.update({
                "HOSTNAME": hostname,
                "HOSTNAME_ALT": random.choice([h for h in HOSTNAMES if h != hostname]),
                "USERNAME": username,
                "TIMEZONE": tz_path,
                "TIMEZONE_INPUT": tz_input,
            })

            # Secondary disk if needed
            if needs_secondary:
                sec = random.choice(SECONDARY_DISK_CONFIGS)
                sec_disk, sec_size, sec_model = sec
                sec_boot_part = get_partition_suffix(sec_disk, 1)
                sec_root_part = get_partition_suffix(sec_disk, 2)

                # Determine which is bigger
                def parse_size(s):
                    if s.endswith("T"):
                        return float(s[:-1]) * 1000
                    elif s.endswith("G"):
                        return float(s[:-1])
                    return 0

                if parse_size(sec_size) > parse_size(size):
                    bigger_disk, bigger_size = sec_disk, sec_size
                    bigger_boot, bigger_root = sec_boot_part, sec_root_part
                else:
                    bigger_disk, bigger_size = disk, size
                    bigger_boot, bigger_root = boot_part, root_part

                ctx.update({
                    "SECONDARY_DISK": sec_disk,
                    "SECONDARY_SIZE": sec_size,
                    "SECONDARY_MODEL": sec_model,
                    "BIGGER_DISK": bigger_disk,
                    "BIGGER_SIZE": bigger_size,
                    "BIGGER_BOOT_PART": bigger_boot,
                    "BIGGER_ROOT_PART": bigger_root,
                    "PARTITION_BIGGER_DISK_CMD": get_partition_cmd(bigger_disk, boot_mode),
                    "FORMAT_BIGGER_DISK_CMD": get_format_cmd(bigger_boot, bigger_root, boot_mode, "ext4"),
                    "MULTI_DISK_ADVICE": f"The {sec_model} ({sec_size}) and the {model} ({size}). SSDs are faster for the OS, HDDs are better for bulk storage.",
                    "USER_DISK_CHOICE": "SSD" if "SSD" in model else "faster one",
                    "INITIAL_CHOICE": "big one",
                    "CHANGED_CHOICE": "SSD" if "SSD" in model else "smaller one",
                    "INITIAL_CHOICE_RESPONSE": f"The {sec_size} drive? The SSD would be faster for the OS though.",
                })

            # Filesystem if needed
            if needs_filesystem:
                for fs_name, fs_explanation, fs_tip in FILESYSTEMS:
                    fs_ctx = ctx.copy()
                    fs_ctx.update({
                        "REQUESTED_FS": fs_name,
                        "FS_EXPLANATION": fs_explanation,
                        "FS_POST_INSTALL_TIP": fs_tip,
                        "FORMAT_CMD_CUSTOM_FS": get_format_cmd(boot_part, root_part, boot_mode, fs_name),
                    })
                    variations.append(fs_ctx)
            else:
                variations.append(ctx)

    # Add some context-specific placeholders
    for ctx in variations:
        ctx["NEXT_STEP_SUGGESTION"] = "partition and format the disk" if not ctx.get("_partitioned") else "install the system"

    return variations

def convert_template_with_context(template: dict, ctx: dict) -> list:
    """Convert a template to training snapshots using a specific context."""
    snapshots = []

    # Build initial system state
    state = SystemState()
    state.boot_mode = ctx.get("BOOT_MODE", "UEFI")

    disk = Disk(
        device=f"/dev/{ctx['PRIMARY_DISK']}",
        size=ctx["DISK_SIZE"],
        model=ctx["DISK_MODEL"]
    )
    state.disks.append(disk)

    # Add secondary disk if present
    if "SECONDARY_DISK" in ctx:
        disk2 = Disk(
            device=f"/dev/{ctx['SECONDARY_DISK']}",
            size=ctx["SECONDARY_SIZE"],
            model=ctx["SECONDARY_MODEL"]
        )
        state.disks.append(disk2)

    messages = []

    for turn in template.get("turns", []):
        user_content = fill_placeholders(turn.get("user", ""), ctx)
        response_type = turn.get("type", "text")
        command = fill_placeholders(turn.get("command", ""), ctx)
        response_text = fill_placeholders(turn.get("response", ""), ctx)

        messages.append({"role": "user", "content": user_content})

        if response_type == "command":
            expected = {"type": "command", "command": command}
        else:
            expected = {"type": "text", "response": response_text}

        snapshot = {
            "system_context": state.to_context(),
            "messages": list(messages),
            "expected_response": expected
        }
        snapshots.append(snapshot)

        # Generate assistant message for history
        if response_type == "command":
            assistant_content = f"$ {command}"
            state.apply_command(command)
        else:
            assistant_content = response_text

        messages.append({"role": "assistant", "content": assistant_content})

    return snapshots

# ============================================================================
# Template Truncation - Generate shorter versions from multi-turn templates
# ============================================================================

def generate_truncated_templates(template: dict, min_turns: int = 1, skip_if_short: int = 3, max_truncations: int = 3) -> list:
    """
    Generate truncated versions of a multi-turn template.

    Instead of ALL truncations (which causes data explosion), we sample a few
    strategically chosen lengths to maintain diversity without overfitting.

    Args:
        template: The conversation template
        min_turns: Minimum number of turns to generate (default 1)
        skip_if_short: Don't truncate templates with this many turns or fewer (default 3)
        max_truncations: Maximum number of truncated versions to create (default 3)

    This adds diversity without causing massive data inflation.
    """
    turns = template.get("turns", [])

    # Don't truncate already-short templates
    if len(turns) <= skip_if_short:
        return [template]

    template_id = template.get("id", "unknown")
    template_desc = template.get("desc", "")

    # Choose strategic truncation points: short, medium, and full
    # This gives diversity without explosion
    n = len(turns)
    if n <= 6:
        # For medium templates, just use a couple points
        lengths = [1, n]
    else:
        # For long templates, sample: early, middle, full
        lengths = [1, n // 2, n]

    # Limit to max_truncations
    lengths = lengths[:max_truncations]

    truncated = []
    for length in lengths:
        truncated_template = {
            "id": f"{template_id}_t{length}",
            "desc": f"{template_desc} (truncated to {length} turns)",
            "turns": turns[:length],
        }
        # Preserve any metadata
        if "_legacy" in template:
            truncated_template["_legacy"] = template["_legacy"]
        truncated.append(truncated_template)

    return truncated


# ============================================================================
# Main
# ============================================================================

def convert_legacy_conversation(conv: dict) -> dict:
    """Convert old hardcoded conversation to template format with placeholders."""

    # Extract disk info from system_context
    ctx = conv.get("system_context", "")

    # Find primary disk: "- /dev/sda: 500G (Samsung SSD 870)"
    disk_match = re.search(r'-\s*/dev/(sd[a-z]|nvme\d+n\d+|vd[a-z]):\s*(\S+)\s*\(([^)]+)\)', ctx)
    if not disk_match:
        return None  # Skip if no disk found

    primary_disk = disk_match.group(1)
    disk_size = disk_match.group(2)
    disk_model = disk_match.group(3)

    # Determine partition names
    if primary_disk.startswith("nvme") or primary_disk.startswith("mmcblk"):
        boot_part = f"{primary_disk}p1"
        root_part = f"{primary_disk}p2"
    else:
        boot_part = f"{primary_disk}1"
        root_part = f"{primary_disk}2"

    # Check for secondary disk
    secondary_match = re.findall(r'-\s*/dev/(sd[a-z]|nvme\d+n\d+|vd[a-z]):\s*(\S+)\s*\(([^)]+)\)', ctx)
    has_secondary = len(secondary_match) > 1

    # Create replacement map
    replacements = {
        f"/dev/{primary_disk}": "/dev/{PRIMARY_DISK}",
        f"/dev/{boot_part}": "/dev/{BOOT_PARTITION}",
        f"/dev/{root_part}": "/dev/{ROOT_PARTITION}",
        disk_size: "{DISK_SIZE}",
        disk_model: "{DISK_MODEL}",
    }

    if has_secondary:
        sec_disk = secondary_match[1][0]
        sec_size = secondary_match[1][1]
        sec_model = secondary_match[1][2]
        if sec_disk.startswith("nvme") or sec_disk.startswith("mmcblk"):
            sec_boot = f"{sec_disk}p1"
            sec_root = f"{sec_disk}p2"
        else:
            sec_boot = f"{sec_disk}1"
            sec_root = f"{sec_disk}2"
        replacements.update({
            f"/dev/{sec_disk}": "/dev/{SECONDARY_DISK}",
            f"/dev/{sec_boot}": "/dev/{SECONDARY_BOOT}",
            f"/dev/{sec_root}": "/dev/{SECONDARY_ROOT}",
            sec_size: "{SECONDARY_SIZE}",
            sec_model: "{SECONDARY_MODEL}",
        })

    def apply_replacements(text: str) -> str:
        for old, new in sorted(replacements.items(), key=lambda x: -len(x[0])):
            text = text.replace(old, new)
        return text

    # Convert turns
    new_turns = []
    for turn in conv.get("turns", []):
        new_turn = {
            "user": turn.get("user", ""),
            "type": turn.get("type", "text"),
        }
        if turn.get("command"):
            new_turn["command"] = apply_replacements(turn["command"])
        if turn.get("response"):
            new_turn["response"] = apply_replacements(turn["response"])
        new_turns.append(new_turn)

    return {
        "id": conv.get("id", "legacy"),
        "desc": conv.get("desc", ""),
        "turns": new_turns,
        "_legacy": True,  # Mark as converted
        "_original_disk": primary_disk,
        "_original_size": disk_size,
        "_original_model": disk_model,
    }


def main():
    random.seed(42)  # Reproducible

    script_dir = Path(__file__).parent
    conv_dir = script_dir / "conversations"
    output_file = script_dir / "training" / "augmented_dataset.jsonl"
    test_output_file = script_dir / "testing" / "test_dataset.jsonl"

    # What percentage of templates to hold out for testing
    TEST_SPLIT = 0.15  # 15% for testing

    # Load ALL conversation files (templates and legacy batch files)
    templates = []
    legacy_count = 0

    for conv_file in sorted(conv_dir.glob("*.jsonl")):
        print(f"Loading {conv_file.name}...")
        with open(conv_file) as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue
                try:
                    conv = json.loads(line)

                    # Check if it's legacy format (has system_context with hardcoded disks)
                    ctx = conv.get("system_context", "")
                    is_legacy = "system_context" in conv and ("/dev/sd" in ctx or "/dev/nvme" in ctx or "/dev/vd" in ctx)
                    if is_legacy:
                        converted = convert_legacy_conversation(conv)
                        if converted:
                            templates.append(converted)
                            legacy_count += 1
                    else:
                        # It's already a template
                        templates.append(conv)
                except json.JSONDecodeError as e:
                    print(f"  WARNING: Invalid JSON: {e}", file=sys.stderr)

    print(f"\nLoaded {len(templates)} templates ({legacy_count} converted from legacy format)")

    # Split templates into train and test sets BEFORE expansion
    # This ensures test templates are truly unseen (not just different variations)
    random.shuffle(templates)
    split_idx = int(len(templates) * (1 - TEST_SPLIT))
    train_templates = templates[:split_idx]
    test_templates = templates[split_idx:]

    print(f"Split: {len(train_templates)} train, {len(test_templates)} test ({TEST_SPLIT*100:.0f}% held out)")

    # Generate truncated versions of multi-turn templates
    # This multiplies data by creating shorter versions of long conversations
    expanded_train = []
    for template in train_templates:
        truncated = generate_truncated_templates(template, min_turns=1)
        expanded_train.extend(truncated)

    expanded_test = []
    for template in test_templates:
        truncated = generate_truncated_templates(template, min_turns=1)
        expanded_test.extend(truncated)

    print(f"Expanded: {len(expanded_train)} train templates, {len(expanded_test)} test templates")

    # Generate training snapshots
    train_snapshots = []
    for template in expanded_train:
        variations = generate_variations(template)
        for ctx in variations:
            snapshots = convert_template_with_context(template, ctx)
            train_snapshots.extend(snapshots)

    # Generate test snapshots (use different random variations)
    random.seed(123)  # Different seed for test variations
    test_snapshots = []
    for template in expanded_test:
        variations = generate_variations(template)
        for ctx in variations:
            snapshots = convert_template_with_context(template, ctx)
            test_snapshots.extend(snapshots)

    # Write training output
    output_file.parent.mkdir(exist_ok=True)
    with open(output_file, "w") as f:
        for snapshot in train_snapshots:
            f.write(json.dumps(snapshot) + "\n")

    # Write test output
    test_output_file.parent.mkdir(exist_ok=True)
    with open(test_output_file, "w") as f:
        for snapshot in test_snapshots:
            f.write(json.dumps(snapshot) + "\n")

    print(f"\n{'='*60}")
    print(f"TRAINING SET: {len(train_snapshots)} snapshots")
    print(f"Output: {output_file}")

    # Training stats
    text_count = sum(1 for s in train_snapshots if s["expected_response"]["type"] == "text")
    cmd_count = sum(1 for s in train_snapshots if s["expected_response"]["type"] == "command")
    print(f"  Response types: Text {text_count} ({100*text_count/len(train_snapshots):.1f}%), Command {cmd_count} ({100*cmd_count/len(train_snapshots):.1f}%)")

    print(f"\n{'='*60}")
    print(f"TEST SET: {len(test_snapshots)} snapshots (held out, unseen)")
    print(f"Output: {test_output_file}")

    # Test stats
    text_count = sum(1 for s in test_snapshots if s["expected_response"]["type"] == "text")
    cmd_count = sum(1 for s in test_snapshots if s["expected_response"]["type"] == "command")
    print(f"  Response types: Text {text_count} ({100*text_count/len(test_snapshots):.1f}%), Command {cmd_count} ({100*cmd_count/len(test_snapshots):.1f}%)")

    # Combined stats
    all_snapshots = train_snapshots + test_snapshots
    print(f"\n{'='*60}")
    print(f"COMBINED STATS")

    lengths = [len(s["messages"]) for s in all_snapshots]
    short = sum(1 for l in lengths if l <= 2)
    medium = sum(1 for l in lengths if 3 <= l <= 6)
    long_ = sum(1 for l in lengths if 7 <= l <= 12)
    very_long = sum(1 for l in lengths if l > 12)
    print(f"\nConversation lengths:")
    print(f"  1-2 messages: {short} ({100*short/len(all_snapshots):.1f}%)")
    print(f"  3-6 messages: {medium} ({100*medium/len(all_snapshots):.1f}%)")
    print(f"  7-12 messages: {long_} ({100*long_/len(all_snapshots):.1f}%)")
    print(f"  13+ messages: {very_long} ({100*very_long/len(all_snapshots):.1f}%)")

    # Boot mode distribution
    uefi_count = sum(1 for s in all_snapshots if "UEFI" in s["system_context"])
    bios_count = sum(1 for s in all_snapshots if "Legacy BIOS" in s["system_context"])
    print(f"\nBoot modes:")
    print(f"  UEFI: {uefi_count} ({100*uefi_count/len(all_snapshots):.1f}%)")
    print(f"  Legacy BIOS: {bios_count} ({100*bios_count/len(all_snapshots):.1f}%)")

if __name__ == "__main__":
    main()
