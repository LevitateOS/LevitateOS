# TEAM_303: Investigate sys_read loop

## Symptom Description
The shell seems to be stuck in a `sys_read` loop, or at least performing the same syscalls repeatedly.

## Hypotheses
1. [TBD]

## Evidence Gathered
- User reported "there is the same syscalls everywhere".
- Terminal output shows sequential `SYS_READ` and `SYS_WRITE` calls that look like normal character-by-character shell input, but maybe they are happening too fast or without user input.

## Progress
- [ ] Read `TEAM_302` log.
- [ ] Check `git diff`.
- [ ] Analyze `sys_read` implementation.
- [ ] Analyze shell input handling.
