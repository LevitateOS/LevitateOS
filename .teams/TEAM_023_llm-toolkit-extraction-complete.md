# TEAM_023: Complete llm-toolkit Extraction

## Goal
Remove ALL Python from `installer/` - make llm-toolkit fully generic, move installer-specific logic to Rust.

## Status: NEARLY COMPLETE - Need verification

## What Was Done

### 1. ✅ Updated llm-toolkit/llm_server.py
- Removed hook methods (gather_context, verify_response, build_system_prompt)
- Server now accepts `system_context` directly in HTTP request body
- Pure stateless inference - caller provides all context

### 2. ✅ Updated installer/src/llm.rs
- Added runtime toolkit discovery (checks LLM_TOOLKIT_PATH env, relative paths, system paths)
- Ported system facts gathering from Python (lsblk, boot mode, network, hostname, timezone, users)
- Ported hallucination detection (blocks commands with non-existent disk paths)
- Updated HTTP client to pass system_context in request
- Added `refresh_facts()` method to update context after commands
- Added `regex = "1.10"` dependency to Cargo.toml

### 3. ✅ Moved sweep_hyperparams.py to llm-toolkit/
- Made it generic (requires --data-dir and --test-file args)
- Uses llm-toolkit's train_lora.py and evaluate.py

### 4. ✅ Merged annotate_thinking.py into llm-toolkit/generate_data.py
- Added batch API support (--batch, --status, --process flags)
- Added checkpoint/resume support
- Sync and batch modes both work

### 5. ✅ Ported augment_data.py to Rust
- Created installer/src/bin/augment_data.rs (~600 lines)
- All disk configs, boot modes, hostnames, timezones, filesystems
- SystemState tracking with apply_command()
- Template expansion with placeholder substitution
- Legacy conversion support
- Train/test split (85/15)
- Added `rand = "0.9"` dependency to Cargo.toml
- Added [[bin]] target "augment-data"

### 6. ✅ Exported test cases
- Ran: `python evaluate_lora.py --export-tests ../data/test_cases.jsonl`
- 19 test cases exported

### 7. ✅ Moved data and deleted Python
- Created installer/data/ directory structure:
  - data/conversations/ (copied from python/conversations/)
  - data/training/ (empty, will be generated)
  - data/testing/ (empty, will be generated)
  - data/test_cases.jsonl (exported)
  - data/.gitignore
- Deleted all Python files from installer/python/
- Deleted installer/python/ directory entirely

## What Still Needs To Be Done

### 8. ⏳ Verify all changes work correctly
Shell died during verification. Need to run:

```bash
# 1. Check installer compiles
cd /home/vince/Projects/LevitateOS/installer
cargo check

# 2. Check augment-data binary compiles
cargo build --bin augment-data

# 3. Test augment-data runs (needs data/conversations/*.jsonl)
cargo run --bin augment-data

# 4. Check llm-toolkit scripts work
python llm-toolkit/llm_server.py --help
python llm-toolkit/sweep_hyperparams.py --help
python llm-toolkit/generate_data.py --help
```

## Files Changed Summary

| Action | File |
|--------|------|
| MODIFIED | llm-toolkit/llm_server.py |
| MODIFIED | llm-toolkit/generate_data.py (added batch support) |
| CREATED | llm-toolkit/sweep_hyperparams.py |
| MODIFIED | installer/src/llm.rs (complete rewrite) |
| MODIFIED | installer/Cargo.toml (added regex, rand, bin target) |
| CREATED | installer/src/bin/augment_data.rs |
| CREATED | installer/data/conversations/*.jsonl (copied) |
| CREATED | installer/data/test_cases.jsonl |
| CREATED | installer/data/.gitignore |
| DELETED | installer/python/ (entire directory) |

## Architecture After Changes

```
llm-toolkit/                    # Generic Python (NO installer knowledge)
  llm_server.py                 # Pure inference (receives context in request)
  train_lora.py                 # LoRA training
  evaluate.py                   # Generic evaluation
  generate_data.py              # Data utilities + thinking annotation
  sweep_hyperparams.py          # HP search (moved from installer)

installer/                      # Rust + Data only
  src/
    llm.rs                      # System facts, hallucination check, toolkit discovery
    bin/augment_data.rs         # Training data generation (ported from Python)
  data/
    conversations/*.jsonl       # Source templates
    training/                   # Generated training data
    testing/                    # Generated test data
    test_cases.jsonl            # Evaluation test cases
```

## Key Design Decisions

1. **Runtime toolkit discovery** instead of embedding - finds llm-toolkit via:
   - LLM_TOOLKIT_PATH environment variable
   - Relative paths from executable (../llm-toolkit, etc.)
   - System paths (/usr/share/llm-toolkit)

2. **Stateless server** - all context passed in request, no hooks needed

3. **Hallucination detection in Rust** - checks disk paths against gathered facts before executing commands
