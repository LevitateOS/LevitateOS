// The Help page: one page for each of the three apps, owned by the frame (the
// frame knows which app it is, so the app adds nothing). It says what the app
// is for, the few ideas a person needs, how to do what the app can do (each
// ending in a link to the page where it is done), what the agent is, and what
// to do when something is wrong. Everything on it is a sentence that is true of
// the app as built; what is not built is left out (see the "Not done" list of
// the app's README before adding a sentence).
#pragma once

#include "frame.h"
#include "page.h"

#include <QList>
#include <QPair>
#include <QStringList>

// One idea a person needs. `first` is its first sentence alone: the page a
// person sees the first time (firstrun.*) takes `first` of the ideas that are
// marked for it.
struct HelpIdea {
    QString title, first, more;
    bool forFirstRun = false;
};

// A thing the app can do, or a trouble, with the page that is its way on.
// `target` is a rail key ("next"), `Frame::activityKey()`, `Frame::settingsKey()`,
// or a page that is not on the rail and that the app has offered with
// `Frame::addHelpTarget`; with no target the row has no link.
struct HelpRow {
    QString title, words, target, page; // `page`: how the link names it ("The next system")
};

struct HelpText {
    QString name, purpose, never, agentInThisApp;
    QList<HelpIdea> ideas;
    QList<HelpRow> how, wrong;
    // The other two apps, one line each.
    QList<QPair<Frame::App, QString>> others;
};

HelpText helpText(Frame::App app);

class HelpPage : public Page {
    Q_OBJECT
public:
    explicit HelpPage(Frame::App app, QWidget *parent = nullptr);

    // Draws the page for the pages that exist now: `targets` are the rail's keys
    // and the pages the app offered besides. A thing to do whose target is not
    // among them is left out, so no link leads nowhere. The frame calls it each
    // time the page is shown.
    void draw(const QStringList &targets);

    // For a check: the words on the page (labels and buttons).
    QString pageText() const;
    // The link or button with exactly these words, as a click would; false if
    // there is none on the page.
    bool press(const QString &words);
    // The words of every link on the page, in order.
    QStringList links() const;
    // Scrolls down by about a screen; false when it was at the end already.
    bool scrollDown();

signals:
    // A link: a page of the app, Activity or Settings.
    void go(const QString &target);
    // "Open Recipe" and the like.
    void startApp(Frame::App app);
    // "Show the first page again".
    void firstPageAgain();

private:
    Frame::App app_;
    QStringList targets_;
};
