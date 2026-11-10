// The Codex adapter: runs `codex exec` once for each turn, turns what it
// prints into the apps' events, and carries the person's answers back by
// resuming the same session.
#pragma once

#include "adapter.h"

#include <QMap>
#include <QSet>
#include <QStringList>

class Child;

class Codex : public Adapter {
public:
    explicit Codex(Session *session);
    void begin(const Request &request) override;
    void answered(const QString &id, const QString &answer, const QByteArray &line) override;
    void inputClosed() override;
    void stop() override;

    // What the agent's last message asks for, if anything.
    struct Wish {
        enum Kind { Nothing, Ask, Run, Administrator } kind = Nothing;
        QString text;          // the question, or the command
        QStringList choices;
        QString because;
        QString rest;          // the message without the convention lines
    };

private:
    // What waits for the person.
    struct Pending {
        QString id;
        Wish wish;
        QStringList words;
    };

    void turn(const QString &prompt);
    void readOutput();
    void onLine(const QByteArray &line);
    void onItem(const QString &phase, const QJsonObject &item);
    void onTodo(const QJsonObject &item);
    void onExit();
    void afterTurn();
    void ask(const Wish &wish, const QStringList &words);
    void execute(const Pending &pending);
    void fail(const QString &reason);
    void say(const QString &text);
    void did(const QString &kind, const QString &what, const QJsonValue &exitCode, bool administrator, bool approved);
    void output(const QString &text);

    Child *child_ = nullptr;
    Child *helper_ = nullptr;
    QByteArray buffer_, errors_;
    QString threadId_, lastMessage_, errorMessage_;
    bool completed_ = false, waiting_ = false, over_ = false;
    int turns_ = 0, questions_ = 0;
    Pending pending_;
    QSet<QString> started_, finished_;
    QStringList planned_;
    QMap<int, QString> stepState_;
};
