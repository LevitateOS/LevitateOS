// What the Before page says for each thing System does to the computer. The
// words come from the pages and from what recab reported (its state, its
// verdicts and reasons, the `destructive` mark on an operation); nothing here
// is worked out about the lifecycle. It does not say how long a command takes
// or how much it changes, because recab reports neither.
#pragma once

#include "before.h"
#include "client.h"
#include "model.h"

// How long recab waits for start-up to finish before a trial fails: the
// default of `recab trial --timeout`, TRIAL_TIMEOUT_SECS (900) in
// tools/recab/src/init_system.rs.
inline constexpr int recabTrialMinutes = 15;

namespace beforewords {

// Finishing the next system: checked and closed, no restart.
BeforePage::Content finish(const RecabClient &client, const Status &status);
// Restarting into the next system. With the restart set already only the
// restart is left.
BeforePage::Content restartNow(const RecabClient &client, const Status &status);
// Setting the next system to start at the next restart, without restarting.
BeforePage::Content restartLater(const RecabClient &client, const Status &status);
// Taking a set restart back.
BeforePage::Content cancelRestart(const RecabClient &client, const Status &status);
// Keeping the running system, which is on trial.
BeforePage::Content keep(const RecabClient &client, const Status &status);
// Going back to the system from before, and restarting into it.
BeforePage::Content goBack(const RecabClient &client, const Status &status);
// Giving up the next system's changes and copying the running system again.
BeforePage::Content reset(const RecabClient &client, const Status &status);
// Giving up the next system.
BeforePage::Content discard(const RecabClient &client, const Status &status);
// Carrying on where preparing stopped.
BeforePage::Content recover(const RecabClient &client, const Status &status);
// Adding a release to the next system: from the one recab found, or from a
// file (`fromFile`).
BeforePage::Content addRelease(const RecabClient &client, const Status &status, bool fromFile);
// A request handed to the agent while the place of the system from before
// would be reused: the question that has to be answered before it starts.
BeforePage::Content reuse(const RecabClient &client, const Status &status);
// Restarting the computer alone (the recovery list's "Reboot").
BeforePage::Content restartComputer(const RecabClient &client, const Status &status);
// One of recab's operations by its own words (its label, its `destructive`
// mark), for those the pages above do not cover.
BeforePage::Content operation(const RecabClient &client, const Status &status, const QJsonObject &op);
// The two release steps that change the next system, by operation id
// (`update_cancel`, `update_recover`). Empty title when the id is not one.
BeforePage::Content releaseStep(const RecabClient &client, const Status &status, const QString &id);

// Adopting the original release's baseline from the release file at `manifest`
// (`adopt_baseline`), which a merge blocked on a missing baseline waits for.
BeforePage::Content adoptBaseline(const RecabClient &client, const Status &status, const QString &manifest);
// Reproducing one package through Recipe for the release being added
// (`update_reproduce`), which a merge blocked on a package rebuild waits for.
BeforePage::Content reproduce(const RecabClient &client, const Status &status, const QString &package);
// Replacing a failed system that recab keeps no preparation record for
// (`replace_failed`): recab's own label for it and its `destructive` mark.
BeforePage::Content replaceFailed(const RecabClient &client, const Status &status, const QJsonObject &op);

} // namespace beforewords
