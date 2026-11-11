// What the stories of the three apps check about Help and the first page: the
// rail has Help under Settings; the page for this app; every link on it goes
// where it says; the first page in the three states an agent can be in
// (signed in, not signed in, not installed), with the stand-in Codex and the
// built levitate-agent; and going on to the app. The checks are the same in
// every app; the app says which page each of its targets is. Only the picture
// modes use it.
#pragma once

#include "frame.h"

#include <QString>
#include <QStringList>

#include <functional>

class QWidget;

namespace helpstory {

using Save = std::function<void(QWidget &, const QString &)>;
using Expect = std::function<void(const QString &, bool)>;

struct Plan {
    Frame::App app = Frame::Recipe;
    QWidget *window = nullptr;
    Frame *frame = nullptr;
    // The rail key the app opens on, and the pages Help leads to that the app
    // shows itself, by key (a rail key, or one given to `addHelpTarget`): is that
    // page the one on screen now?
    QString firstEntry;
    std::function<bool(const QString &key)> pageShown;
    // Waits until the window has finished reading what a page asked for.
    std::function<void()> idle;
    // `folder`: where the pictures go (and a scratch home); `fixtures`: gui/agent/fixtures.
    QString folder, fixtures;
};

// `prefix` names the pictures (`PREFIX-help`, `PREFIX-firstrun-...`).
void run(const Plan &plan, const Save &save, const Expect &expect, const QString &prefix);

} // namespace helpstory
