// What Codex and Claude Code have in common: a run is a series of turns, each
// one a start of the agent that ends with a last message. That message may end
// with a line saying the agent needs the person (ASK, RUN, RUN-AS-ADMINISTRATOR);
// the answer starts the next turn. How one turn is started and read is the
// business of the class that derives from this one.
#pragma once

#include "adapter.h"

#include <QMap>
#include <QStringList>

class Child;

class Conversation : public Adapter {
public:
    void begin(const Request &request) override;
    void answered(const QString &id, const QString &answer, const QByteArray &line) override;
    void stop() override;

    // What the agent's last message asks for, if anything.
    struct Wish {
        enum Kind { Nothing, Ask, Run, Administrator } kind = Nothing;
        QString text;          // the question, or the command
        QStringList choices;
        QString because;
        QString rest;          // the message without the convention lines
    };
    static Wish readWish(const QString &message);

protected:
    explicit Conversation(Session *session);

    // The text the agent is given once: how to ask the person for anything.
    static QString contract();
    // The first prompt of the run. `who` says which window it comes from.
    virtual QString firstPrompt(const QString &who, const QString &prompt) const = 0;
    // Starts the agent for one turn. It later calls `turnEnded` or `fail`.
    // `first` is true for the first turn of the run.
    virtual void launch(const QString &prompt, bool first) = 0;
    // Ends what `launch` started.
    virtual void endAgent() = 0;

    // The turn ended well; `message` is the agent's last word.
    void turnEnded(const QString &message);
    void fail(const QString &reason);
    void say(const QString &text);
    void output(const QString &text);
    void did(const QString &kind, const QString &what, const QJsonValue &exitCode, bool administrator, bool asked,
             const QJsonValue &approved);
    // The agent's list of steps. `states` holds `done`, `running` or an empty
    // string for each text. A `plan` is sent only when the wording changes.
    void plan(const QStringList &texts, const QStringList &states);
    // The reason a turn failed, made plainer for a person who is not signed in.
    static QString plainReason(const QString &reason, const QString &agentName, const QString &signInWords);
    static QStringList cappedLines(const QString &text, int most);

    Request request_;
    bool over_ = false;

private:
    struct Pending {
        QString id;
        Wish wish;
        QStringList words;
    };

    void turn(const QString &prompt);
    void ask(const Wish &wish, const QStringList &words);
    void execute(const Pending &pending);

    Child *helper_ = nullptr;
    bool waiting_ = false;
    int turns_ = 0, questions_ = 0;
    Pending pending_;
    QStringList planned_;
    QMap<int, QString> stepState_;
};
