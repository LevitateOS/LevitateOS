# TEAM_019: Extract llm-toolkit as Reusable Submodule

## Goal
Extract generic LLM training tools from `/python` into `llm-toolkit` git submodule that can be shared between installer and recipe projects.

## Context
- `/installer/python` has full LLM training pipeline (installer-specific)
- `/python` was a partial extraction attempt (3 files)
- Need shared toolkit for both installer and recipe LoRA training

## Plan
1. Rename `/python` → `/llm-toolkit`
2. Create generic `evaluate.py` (extract from installer's evaluate_lora.py)
3. Add README.md
4. Initialize as git submodule

## Progress
- [x] Rename directory
- [x] Create evaluate.py
- [x] Create README.md
- [x] Initialize git repo
- [x] Verify tools work

## Result
All tools verified working:
- `train_lora.py --help` ✓
- `generate_data.py --help` ✓
- `llm_server.py --help` ✓
- `evaluate.py --help` ✓

Git repo initialized with initial commit (27ab00b).

## Files
- `/llm-toolkit/train_lora.py` - LoRA training
- `/llm-toolkit/generate_data.py` - Data utilities
- `/llm-toolkit/llm_server.py` - HTTP inference
- `/llm-toolkit/evaluate.py` - Model evaluation (NEW)
