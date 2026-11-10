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
    // "...", "title": "..."}`, with the input left open so its questions can
    // be answered. `app` and `title` are what the Activity page lists the run
    // under; either may be left out.
    void setPrompt(const QString &prompt, const QString &app = QString(), const QString &title = QString());
    // The process id while it runs, else 0.
    qint64 pid() const { return process_.processId(); }
    // Answers a `question` event: one JSON line, `{"question": id, "answer": "..."}`.
    void reply(const QString &question, const QString &answer);
    void start();
    void stop();
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

private:
    void drain();
    QProcess process_;
    QByteArray pending_, input_, answer_;
    QJsonObject failure_;
    QStringList said_;
    bool open_ = false;
};
