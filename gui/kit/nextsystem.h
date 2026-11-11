// Whether a next system is being prepared and what it holds, for the strip
// under every page of every app ("The next system holds 2 changes."). System
// has the whole status already and feeds the strip itself; Recipe and Disk
// ask recab with this.
#pragma once

#include <QByteArray>
#include <QObject>
#include <QStringList>

class Run;

class NextSystem : public QObject {
    Q_OBJECT
public:
    // `recab`: the program to run, `recab status --json`.
    NextSystem(const QString &recab, QObject *parent = nullptr);
    // Asks again, without holding anything up. Answers with `known`.
    void read();
    // The last answer: how many changes the next system holds, and their
    // titles ("htop, installed"). Zero when there is no next system, when it
    // holds nothing, or when the status could not be read.
    QStringList titles() const { return titles_; }

    // recab's status (the real one: `lifecycle.other` is "next" and
    // `preparation.changes` lists the changes; none while the running system is on
    // trial: the other one is the system from before). Returns false when the text is
    // not a status this reads. Public so that a check can feed it a status.
    static bool parse(const QByteArray &json, QStringList *titles);

signals:
    // Asked for, and answered; `titles()` is current.
    void known();

private:
    QString recab_;
    Run *running_ = nullptr;
    QStringList titles_;
};
