// The Claude Code adapter: runs `claude -p` once for each turn with
// `--output-format stream-json`, turns what it prints into the apps' events,
// and carries the person's answers back by resuming the same session.
//
// Written from Anthropic's documentation (see README.md for the lines it
// rests on). It has never run against a real Claude Code.
#pragma once

#include "conversation.h"

#include <QJsonObject>
#include <QMap>
#include <QSet>

class Child;
class QTimer;

class Claude : public Conversation {
public:
    explicit Claude(Session *session);

protected:
    QString firstPrompt(const QString &who, const QString &prompt) const override;
    void launch(const QString &prompt, bool first) override;
    void endAgent() override;

private:
    struct Tool {
        QString name;
        QJsonObject input;
    };

    void readOutput();
    void onLine(const QByteArray &line);
    void onAssistant(const QJsonObject &message);
    void onUser(const QJsonObject &message);
    void onToolUse(const QJsonObject &block);
    void onToolResult(const QJsonObject &block);
    void onResult(const QJsonObject &message);
    void onExit();

    Child *child_ = nullptr;
    QTimer *grace_ = nullptr;
    QByteArray buffer_, errors_;
    QString sessionId_, lastText_, resultText_, assistantError_;
    bool completed_ = false;
    QMap<QString, Tool> tools_;
    QSet<QString> denied_, finished_;
};
