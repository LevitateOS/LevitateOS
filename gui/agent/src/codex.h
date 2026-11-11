// The Codex adapter: runs `codex exec` once for each turn, turns what it
// prints into the apps' events, and carries the person's answers back by
// resuming the same session.
#pragma once

#include "conversation.h"

#include <QSet>

class Child;

class Codex : public Conversation {
public:
    explicit Codex(Session *session);

protected:
    QString firstPrompt(const QString &who, const QString &prompt) const override;
    void launch(const QString &prompt, bool first) override;
    void endAgent() override;

private:
    void readOutput();
    void onLine(const QByteArray &line);
    void onItem(const QString &phase, const QJsonObject &item);
    void onTodo(const QJsonObject &item);
    void onExit();

    Child *child_ = nullptr;
    QByteArray buffer_, errors_;
    QString threadId_, lastMessage_, errorMessage_;
    bool completed_ = false;
    QSet<QString> started_, finished_;
};
