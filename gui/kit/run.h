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
    void start();
    void stop();
    QJsonObject failure() const { return failure_; }
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
};
