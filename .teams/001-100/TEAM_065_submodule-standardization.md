# TEAM_065: Submodule Documentation Standardization

## Objective
Standardize all git submodules (excluding linux - upstream kernel) with:
- MIT LICENSE
- README.md
- .gitignore
- CLAUDE.md
- Basic GitHub Actions CI

## Submodules to Process
1. website - needs LICENSE, CI
2. recipe - needs CLAUDE.md, CI
3. llm-toolkit - needs LICENSE, CLAUDE.md, CI
4. docs-content - needs README, LICENSE, CI
5. recipes - needs LICENSE, CLAUDE.md, .gitignore
6. leviso - needs README, LICENSE, CI
7. docs-tui - needs README, LICENSE, CLAUDE.md, CI
8. install-tests - needs README, LICENSE, CLAUDE.md, CI
9. stage3 - needs README, LICENSE, CLAUDE.md, CI

**SKIP: linux** (upstream kernel repository)

## Progress
- [x] Step 1: Create team file
- [x] Step 2: Add LICENSE to all 8 submodules
- [x] Step 3: Add missing READMEs (5 submodules)
- [x] Step 4: Add missing CLAUDE.md (6 submodules)
- [x] Step 5: Add .gitignore to recipes
- [x] Step 6: Add GitHub Actions CI (8 submodules)
- [x] Step 7: Commit in each submodule
- [x] Step 8: Update parent repo

## Log
- Started: 2026-01-20
- Completed: 2026-01-20
