// What Codex and Claude Code have in common: a run is a series of turns, each
// one a start of the agent that ends with a last message. That message may end
// with a line saying the agent needs the person (ASK, RUN, RUN-AS-ADMINISTRATOR);
// the answer starts the next turn. How one turn is started and read is the
// business of the class that derives from this one.
#pragma once

#include "adapter.h"

#include <QHash>
#include <QMap>
#include <QSet>
#include <QStringList>

class Child;

class Conversation : public Adapter {
public:
    void begin(const Request &request) override;
    void answered(const QString &id, const QString &answer, const QByteArray &line) override;
    void stop() override;

    // A line of the set-aside convention: what the agent decided about a place.
    struct Claim {
        enum Kind { SetAside, Keep, Cannot } kind = SetAside;
        QString path, why;
    };

    // What the agent's last message asks for, if anything.
    struct Wish {
        enum Kind { Nothing, Ask, Run, Administrator } kind = Nothing;
        QString text;          // the question, or the command
        QStringList choices;
        QString because;
        QString rest;          // the message without the convention lines
        QList<Claim> claims;   // SET-ASIDE, KEEP and CANNOT lines, only when the request has `set_aside`
    };
    Wish readWish(const QString &message) const;

protected:
    explicit Conversation(Session *session);

    // The text the agent is given once: how to ask the person for anything.
    QString contract() const;
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
    // Carries out what the agent decided about places, and writes down what
    // happened for the agent's next turn.
    void settle(const QList<Claim> &claims);
    void place(const QString &path, const QString &outcome, const QString &why, qint64 bytes);
    void ask(const Wish &wish, const QStringList &words);
    void execute(const Pending &pending);

    Child *helper_ = nullptr;
    bool waiting_ = false;
    int turns_ = 0, questions_ = 0;
    Pending pending_;
    QStringList planned_;
    QMap<int, QString> stepState_;
    // What the helper did for the agent since its last turn, said to it at the next one.
    QString report_;
    // Places of `ask_first` the person was asked about in this run, with the last answer.
    QHash<QString, QString> asked_;
    QSet<QString> dealtWith_;
};
