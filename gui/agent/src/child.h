// A program started on the person's behalf, and the reading of its command line.
#pragma once

#include <QProcess>
#include <QStringList>

// A child that leads its own process group. The agent starts programs of its
// own; stopping only the child would leave those running.
class Child : public QProcess {
public:
    using QProcess::QProcess;
    void startInGroup();
    // Asks the whole group to leave, then makes it.
    void endGroup();

private:
    qint64 group_ = 0;
};

struct Words {
    bool ok = false;
    QStringList list;
    QString problem;
};

// One command line into arguments. Nothing runs through a shell, so a line
// that needs one (pipes, redirects, chaining, substitution) is refused rather
// than half-understood.
Words splitCommand(const QString &line);
