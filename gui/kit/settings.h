// The Settings page: one page, the same in all three apps. It reads what
// `levitate-agent --status` and `--settings` report and changes things only by
// asking the helper (`--set`, `--sign-in`, `--sign-out`), which is the only
// writer of its settings file. The frame owns the page; an app does not build it.
#pragma once

#include "page.h"

#include <QJsonObject>
#include <QPointer>
#include <QSize>

class QButtonGroup;
class QLabel;
class QWidget;
class Run;

class SettingsPage : public Page {
    Q_OBJECT
public:
    // `app`: this window's app as Activity names it ("Recab"); "Try it"
    // asks the agent as that app.
    explicit SettingsPage(const QString &app, QWidget *parent = nullptr);

    // Reads again without holding the window up; the first time the page says so.
    void read();
    // Whether the page was ever asked to read (the frame reads it again only then).
    bool started() const { return started_; }

    // For a check: what the page knows and shows.
    bool loaded() const { return loaded_; }
    QString problem() const { return problem_; }
    // Anything is being read, changed or asked.
    bool busy() const;
    // What the helper last reported.
    QString agentInForce() const { return settings_["agent"].toString(); }
    QString commandsInForce() const { return settings_["run_commands"].toString(); }
    // The words on the page: labels, buttons, the choice and the radio buttons.
    QString pageText() const;
    // The checked radio button's agent: its id ("codex", "claude") or "other".
    QString checkedAgent() const;
    // The sentence shown in place of a refusal or a failure, or empty.
    QString notice() const { return notice_; }
    // As a click would: the button or radio button with these words (false if none is shown and enabled).
    bool press(const QString &words);
    // As choosing in the list would: "Ask each time" or "Allowed".
    void pickCommands(const QString &words);
    // As choosing in the list would: the notices row, "Yes" or "No".
    void pickNotices(bool on);
    // What the helper last reported about notices.
    bool noticesInForce() const { return settings_["notices"].toBool(true); }
    // What the file dialog hands over: written with `--set agent=PATH`.
    void chooseProgram(const QString &path);

signals:
    // The helper's answer to a change may differ from before: the frame asks
    // its status again for the line at the bottom.
    void changed();
    // "Try it" ended; the run is in the helper's record, so Activity can read it.
    void ran();

private:
    void render();
    void readDone();
    void set(const QString &assignment, const QString &where);
    void account(bool signIn);
    bool chosen(const QString &id) const { return settings_["agent"].toString() == id; }
    void tryIt();
    QWidget *agentRow(const QJsonObject &agent, QButtonGroup *group);
    QWidget *otherRow(QButtonGroup *group);

    QString app_, program_;
    Run *statusRun_ = nullptr, *settingsRun_ = nullptr, *op_ = nullptr, *trying_ = nullptr;
    QJsonObject status_, settings_;
    bool started_ = false, loaded_ = false, again_ = false, statusOk_ = false, settingsOk_ = false;
    QString problem_;
    // The sentence shown in place, and where: "codex", "other", "commands".
    QString notice_, noticeWhere_;
    // A sign in or out that is going on (the agent's id), the lines it printed.
    QString accountOf_, accountVerb_;
    QStringList accountLines_;
    QPointer<QLabel> accountLabel_;
    bool confirmingOut_ = false, stopped_ = false;
    // "Try it": what came back, and whether that is a failure.
    QString answer_;
    bool answerFailed_ = false;
    int scroll_ = 0;
};

// Makes a window open at the size it had: the size is read now (`first` the
// first time) and kept when the window closes, under organisation
// "LevitateOS" and one key per app. With `remember` false (the picture modes)
// the window is only given `first`, and nothing is read or written.
void rememberWindowSize(QWidget *window, const QString &app, QSize first, bool remember);
