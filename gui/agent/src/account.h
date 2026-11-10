// `--sign-in` and `--sign-out`: the agent's own login, run for the person.
#pragma once

#include "settings.h"

// Runs `codex login` (or `codex logout`) and passes what it prints through, a
// line at a time, on standard error (where the apps read progress). Returns
// its exit code. A chosen program that is not Codex has no sign-in to run.
int account(const Settings &settings, bool signIn);
