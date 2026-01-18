# TEAM_032: Rename levitateiso to leviso submodule

## Status: COMPLETE

## Objective
Rename `levitateiso/` to `leviso/` and convert it to a git submodule pointing to `git@github.com:LevitateOS/leviso.git`

## Steps
1. Remove build artifacts (target/)
2. Initialize git repo in levitateiso/
3. Push contents to remote repo
4. Remove local levitateiso/ directory
5. Add as submodule named leviso/

## Files
- levitateiso/ -> leviso/ (submodule)
