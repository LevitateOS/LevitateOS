// Runs one command and reads its events. Shared by the front ends.
#pragma once

#include <QJsonObject>
#include <QObject>
#include <QProcess>

// One running command that changes something (`recipe`, `recab`, an agent).
// It reports the JSON events the command writes to stderr, one object per
// line; any other line is passed on as output.
class Run : public QObject {
    Q_OBJECT
public:
    Run(const QString &program, const QStringList &arguments, QObject *parent);
    // Text the command reads on its standard input, then end of input.
    void setInput(const QByteArray &input) { input_ = input; }
    // For an agent: the prompt as one JSON line, `{"prompt": "...", "app":
    // "...", "title": "...", "may_write": [...]}`, with the input left open so
    // its questions can be answered. `app` and `title` are what the Activity
    // page lists the run under; either may be left out. `mayWrite` names the
    // folders (full paths, inside the home folder) the agent may write in
    // although it asks before running commands.
    void setPrompt(const QString &prompt, const QString &app = QString(), const QString &title = QString(),
                   const QStringList &mayWrite = QStringList());
    // A client of a run that is going, `levitate-agent --attach ID`, with the
    // input left open so that its questions can be answered. It reads the
    // events as the record holds them (with their time, and the `answer`s).
    static Run *attach(const QString &agent, const QString &id, QObject *parent);
    // The process id while it runs, else 0.
    qint64 pid() const { return process_.processId(); }
    // The id of the agent run this is a client of: the helper names it in its
    // first line, so it is known a moment after start (empty for a program
    // that is not levitate-agent, and until then). `idKnown` says when.
    QString id() const { return id_; }
    // Whether the prompt route was used (an agent run, not a recipe or recab).
    bool isAgent() const { return agent_; }
    // Whether it is `levitate-agent --attach` (it follows a run, it did not start one).
    bool isAttach() const { return attach_; }
    // Whether the process is going.
    bool running() const { return process_.state() == QProcess::Running; }
    // The exit code of the process once it has finished.
    int exitCode() const { return exitCode_; }
    // The run was stopped (by anyone): the helper's exit code 143.
    bool wasStopped() const { return finishedAlready_ && agent_ && exitCode_ == 143; }
    // Answers a `question` event: one JSON line, `{"question": id, "answer": "..."}`.
    void reply(const QString &question, const QString &answer);
    void start();
    // Ends it. An agent run whose id is known is stopped for real, with
    // `levitate-agent --stop ID` (the run is not the window's, so ending the
    // helper that relays for it would not do); anything else is asked to end.
    void stop();
    // Lets go without ending anything: the input is closed and the helper told
    // to hang up, which is how a client leaves a run that goes on (the run
    // lives in a process of its own). Nothing of this object is heard after.
    void release();
    bool released() const { return released_; }
    ~Run() override;
    QJsonObject failure() const { return failure_; }
    // Why it failed, in one text: the reason it gave, else what it wrote
    // that was not an event, else that it could not be started.
    QString problem() const;
    // What the command wrote on its standard output, complete once finished.
    QByteArray answer() const { return answer_; }

signals:
    void event(const QJsonObject &event);
    void output(const QString &line);
    void finished(bool ok);
    void idKnown(const QString &id);
    // The helper left the run (exit code 75: it waits or works without us).
    // That is not a failure, and not an end either.
    void carriesOn();

private:
    void drain();
    QProcess process_;
    QByteArray pending_, input_, answer_;
    QJsonObject failure_;
    QStringList said_;
    QString id_;
    bool open_ = false, agent_ = false, attach_ = false, released_ = false, finishedAlready_ = false;
    int exitCode_ = -1;
};
