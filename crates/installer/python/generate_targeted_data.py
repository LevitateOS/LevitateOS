#!/usr/bin/env python3
"""
Generate targeted training data for SmolLM3-3B's specific weaknesses.

Weaknesses identified:
1. Wrong command selection (df -h instead of lsblk)
2. No contextual workflow understanding (yes after greeting ≠ start install)
3. Commands output as text instead of tool calls

This script generates high-quality examples specifically targeting these issues.
"""

import json
import random
from pathlib import Path

# Disk configurations for variety
DISK_CONFIGS = [
    {
        "context": """## Current System State

- Boot mode: UEFI
- Network: Connected
- Hostname: archiso
- Timezone: not set

## Available Disks

- /dev/sda: 500G (Samsung SSD 870)""",
        "disk": "/dev/sda",
        "size": "500G"
    },
    {
        "context": """## Current System State

- Boot mode: UEFI
- Network: Connected
- Hostname: archiso
- Timezone: not set

## Available Disks

- /dev/nvme0n1: 1T (Samsung 980 Pro)""",
        "disk": "/dev/nvme0n1",
        "size": "1T"
    },
    {
        "context": """## Current System State

- Boot mode: Legacy BIOS
- Network: Not connected
- Hostname: archiso
- Timezone: not set

## Available Disks

- /dev/vda: 20G (VirtIO Block Device)""",
        "disk": "/dev/vda",
        "size": "20G"
    },
    {
        "context": """## Current System State

- Boot mode: UEFI
- Network: Connected
- Hostname: archiso
- Timezone: not set

## Available Disks

- /dev/sda: 256G (Crucial MX500)
- /dev/sdb: 2T (WD Blue HDD)""",
        "disk": "/dev/sda",
        "size": "256G"
    },
]

# =============================================================================
# WEAKNESS 1: Command Selection
# Teach: "list disks" → lsblk (NOT df -h, NOT fdisk -l, etc.)
# =============================================================================

LIST_DISK_PHRASES = [
    "list disks",
    "show disks",
    "what disks do I have",
    "show me the disks",
    "list available disks",
    "display disks",
    "show storage devices",
    "what drives are available",
    "list drives",
    "show drives",
    "list storage",
    "what's available for installation",
    "show block devices",
    "list block devices",
    "lsblk",  # Even explicit should work
    "show me what I can install to",
    "what can I install on",
    "available storage",
    "disk list",
    "drives list",
]

def generate_list_disk_examples():
    """Generate examples teaching lsblk for disk listing."""
    examples = []

    for phrase in LIST_DISK_PHRASES:
        for config in DISK_CONFIGS:
            # Basic single-turn
            examples.append({
                "system_context": config["context"],
                "messages": [{"role": "user", "content": phrase}],
                "expected_response": {"type": "command", "command": "lsblk"}
            })

            # With typos/variations
            for variant in [phrase.lower(), phrase.upper(), phrase.title()]:
                if variant != phrase:
                    examples.append({
                        "system_context": config["context"],
                        "messages": [{"role": "user", "content": variant}],
                        "expected_response": {"type": "command", "command": "lsblk"}
                    })

    return examples

# =============================================================================
# WEAKNESS 2: Contextual Workflow Understanding
# Teach: Confirmations trigger actions, not descriptions
# =============================================================================

CONFIRMATIONS = [
    "yes", "y", "yeah", "yep", "sure", "ok", "okay", "proceed",
    "do it", "go ahead", "continue", "confirm", "let's do it",
    "sounds good", "that works", "perfect", "go for it",
    # Non-English
    "sim", "si", "ja", "oui", "da",
]

GREETINGS = [
    "hi", "hello", "hey", "ola", "hola", "bonjour", "hallo",
    "good morning", "good afternoon", "good evening",
    "hi there", "hello there", "hey there",
]

def generate_workflow_examples():
    """Generate examples for contextual workflow understanding."""
    examples = []

    # Pattern 1: Greeting → Ready? → Yes → lsblk
    for greeting in GREETINGS:
        for confirm in CONFIRMATIONS:
            for config in DISK_CONFIGS:
                examples.append({
                    "system_context": config["context"],
                    "messages": [
                        {"role": "user", "content": greeting},
                        {"role": "assistant", "content": "Welcome! Ready to install LevitateOS? I'll help you set up your system."},
                        {"role": "user", "content": confirm}
                    ],
                    "expected_response": {"type": "command", "command": "lsblk"}
                })

    # Pattern 2: List disks → result → "use X" → partition command
    for config in DISK_CONFIGS:
        disk = config["disk"]
        for use_phrase in ["use that", f"use {disk}", "use the whole disk", "install there", "use it"]:
            examples.append({
                "system_context": config["context"],
                "messages": [
                    {"role": "user", "content": "list disks"},
                    {"role": "assistant", "content": f"$ lsblk\n\nNAME    SIZE TYPE\n{disk.split('/')[-1]}  {config['size']} disk"},
                    {"role": "user", "content": use_phrase}
                ],
                "expected_response": {
                    "type": "command",
                    "command": f"sgdisk -Z {disk} && sgdisk -n 1:0:+512M -t 1:ef00 -n 2:0:0 {disk}"
                }
            })

    # Pattern 3: Partition plan shown → confirm → format
    for config in DISK_CONFIGS:
        disk = config["disk"]
        p1 = f"{disk}1" if "nvme" not in disk else f"{disk}p1"
        p2 = f"{disk}2" if "nvme" not in disk else f"{disk}p2"

        for confirm in CONFIRMATIONS[:10]:  # Limit to avoid explosion
            examples.append({
                "system_context": config["context"],
                "messages": [
                    {"role": "user", "content": "partition the disk"},
                    {"role": "assistant", "content": f"I'll create:\n- {p1}: 512MB EFI\n- {p2}: rest for root\n\nProceed?"},
                    {"role": "user", "content": confirm}
                ],
                "expected_response": {
                    "type": "command",
                    "command": f"sgdisk -Z {disk} && sgdisk -n 1:0:+512M -t 1:ef00 -n 2:0:0 {disk}"
                }
            })

    # Pattern 4: Format confirmation → mkfs
    for config in DISK_CONFIGS:
        disk = config["disk"]
        p1 = f"{disk}1" if "nvme" not in disk else f"{disk}p1"
        p2 = f"{disk}2" if "nvme" not in disk else f"{disk}p2"

        for confirm in CONFIRMATIONS[:10]:
            examples.append({
                "system_context": config["context"],
                "messages": [
                    {"role": "user", "content": "format the partitions"},
                    {"role": "assistant", "content": f"I'll format:\n- {p1}: FAT32 (EFI)\n- {p2}: ext4 (root)\n\nThis will erase all data. Continue?"},
                    {"role": "user", "content": confirm}
                ],
                "expected_response": {
                    "type": "command",
                    "command": f"mkfs.fat -F32 {p1} && mkfs.ext4 {p2}"
                }
            })

    # Pattern 5: Mount confirmation
    for config in DISK_CONFIGS:
        disk = config["disk"]
        p1 = f"{disk}1" if "nvme" not in disk else f"{disk}p1"
        p2 = f"{disk}2" if "nvme" not in disk else f"{disk}p2"

        for confirm in CONFIRMATIONS[:10]:
            examples.append({
                "system_context": config["context"],
                "messages": [
                    {"role": "user", "content": "mount the partitions"},
                    {"role": "assistant", "content": f"I'll mount {p2} to /mnt and {p1} to /mnt/boot/efi. Ready?"},
                    {"role": "user", "content": confirm}
                ],
                "expected_response": {
                    "type": "command",
                    "command": f"mount {p2} /mnt && mkdir -p /mnt/boot/efi && mount {p1} /mnt/boot/efi"
                }
            })

    return examples

# =============================================================================
# WEAKNESS 3: Tool Call Consistency
# Reinforce: Commands ALWAYS use tool_call, NEVER inline text
# =============================================================================

DIRECT_COMMANDS = [
    ("format /dev/sda1 as ext4", "mkfs.ext4 /dev/sda1"),
    ("mount /dev/sda2 to /mnt", "mount /dev/sda2 /mnt"),
    ("create a user named vince", "useradd -m -G wheel vince"),
    ("set hostname to mypc", "hostnamectl set-hostname mypc"),
    ("set timezone to los angeles", "timedatectl set-timezone America/Los_Angeles"),
    ("install grub", "grub-install --target=x86_64-efi --efi-directory=/boot/efi"),
    ("generate fstab", "genfstab -U /mnt >> /mnt/etc/fstab"),
    ("update grub config", "grub-mkconfig -o /boot/grub/grub.cfg"),
    ("unmount everything", "umount -R /mnt"),
    ("check disk space", "df -h"),
    ("show partition table", "fdisk -l"),
    ("create efi partition", "sgdisk -n 1:0:+512M -t 1:ef00 /dev/sda"),
    ("wipe the disk", "sgdisk -Z /dev/sda"),
    ("list mounted filesystems", "mount"),
    ("show memory usage", "free -h"),
]

def generate_direct_command_examples():
    """Generate examples where user explicitly asks for a command."""
    examples = []

    for config in DISK_CONFIGS:
        for user_request, command in DIRECT_COMMANDS:
            # Replace /dev/sda with the config's disk
            cmd = command.replace("/dev/sda", config["disk"])
            req = user_request.replace("/dev/sda", config["disk"])

            examples.append({
                "system_context": config["context"],
                "messages": [{"role": "user", "content": req}],
                "expected_response": {"type": "command", "command": cmd}
            })

    return examples

# =============================================================================
# Negative examples: Text responses (so model learns the distinction)
# =============================================================================

TEXT_QUERIES = [
    ("what is UEFI?", "UEFI (Unified Extensible Firmware Interface) is modern firmware that replaced BIOS. It supports larger disks, faster boot, and secure boot."),
    ("what filesystem should I use?", "For most users, ext4 is recommended - it's stable, fast, and well-supported. Btrfs offers snapshots if you need them."),
    ("how big should my EFI partition be?", "512MB is standard for EFI partitions. This leaves room for multiple bootloaders if needed."),
    ("do I need a swap partition?", "Swap is optional. If you have 16GB+ RAM, you may not need it. For hibernation, swap should equal your RAM size."),
    ("what's the difference between GPT and MBR?", "GPT is the modern standard, supporting disks over 2TB and more partitions. MBR is legacy, limited to 4 primary partitions and 2TB."),
    ("should I encrypt my disk?", "Encryption protects your data if the device is lost or stolen. Recommended for laptops. Uses LUKS on Linux."),
    ("help", "I'm here to help you install LevitateOS! You can ask me to list disks, partition, format, configure system settings, or create users."),
    ("what can you do?", "I can help you: list and partition disks, format filesystems, mount partitions, configure hostname/timezone, create users, and install the bootloader."),
]

def generate_text_response_examples():
    """Generate examples where text response (not command) is appropriate."""
    examples = []

    for config in DISK_CONFIGS:
        for query, response in TEXT_QUERIES:
            examples.append({
                "system_context": config["context"],
                "messages": [{"role": "user", "content": query}],
                "expected_response": {"type": "text", "response": response}
            })

    return examples


def main():
    output_file = Path(__file__).parent / "training" / "targeted_weaknesses.jsonl"

    print("Generating targeted training data for SmolLM3 weaknesses...")

    all_examples = []

    # Generate examples for each weakness
    list_disk = generate_list_disk_examples()
    print(f"  List disk examples: {len(list_disk)}")
    all_examples.extend(list_disk)

    workflow = generate_workflow_examples()
    print(f"  Workflow examples: {len(workflow)}")
    all_examples.extend(workflow)

    direct = generate_direct_command_examples()
    print(f"  Direct command examples: {len(direct)}")
    all_examples.extend(direct)

    text = generate_text_response_examples()
    print(f"  Text response examples: {len(text)}")
    all_examples.extend(text)

    # Shuffle
    random.seed(42)
    random.shuffle(all_examples)

    print(f"\nTotal targeted examples: {len(all_examples)}")

    # Write to file
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, 'w') as f:
        for ex in all_examples:
            f.write(json.dumps(ex) + '\n')

    print(f"Written to: {output_file}")

    # Stats
    command_count = sum(1 for ex in all_examples if ex["expected_response"]["type"] == "command")
    text_count = sum(1 for ex in all_examples if ex["expected_response"]["type"] == "text")
    print(f"\nBreakdown: {command_count} commands, {text_count} text responses")


if __name__ == "__main__":
    main()
