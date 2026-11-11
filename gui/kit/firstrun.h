// The page a person sees the first time an app opens: what the app is for, the
// things worth knowing first (Help's ideas that are marked for it), whether an
// agent is ready, and one button that goes on to the app. The frame owns it and
// shows it in place of the app's own page, once per person and per app; Help
// says how to see it again. It asks the agent helper for its status itself,
// without holding anything up: the app's own first read does not wait for it.
#pragma once

#include "frame.h"
#include "page.h"

class Run;

class FirstRunPage : public Page {
    Q_OBJECT
public:
    explicit FirstRunPage(Frame::App app, QWidget *parent = nullptr);

    // Whether this person has not seen the page for this app yet, and noting
    // that it was shown. Both use QSettings (organisation "LevitateOS", file
    // `firstrun`, one key per app) and do nothing in a picture run, where a
    // story asks for the page itself: `wanted` is then false, `markSeen` writes
    // nothing.
    static bool wanted(Frame::App app);
    static void markSeen(Frame::App app);

    // Asks the helper for the agent's state and draws the page when it answers
    // (it says "asking" until then). Called each time the page is shown.
    void read();

    // What the helper said about the agent that is chosen.
    enum Agent { Asking, Ready, NotSignedIn, NotInstalled, SignInUnknown, CouldNotAsk };
    Agent agent() const { return agent_; }

    // For a check.
    QString pageText() const;
    bool press(const QString &words);
    bool busy() const { return asking_ != nullptr; }

signals:
    // The one button: go on to the app.
    void goOn();
    // The links: Settings (to sign in or choose an agent), Help.
    void openSettings();
    void openHelp();

private:
    void draw();

    Frame::App app_;
    Run *asking_ = nullptr;
    Agent agent_ = Asking;
    QString agentName_, problem_;
};
