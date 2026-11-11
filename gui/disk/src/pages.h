// The mess list, and what the agent is given about it.
#pragma once

#include "disk.h"
#include "mess.h"
#include "page.h"
#include "scan.h"

#include <QList>

#include <memory>

class PromptPanel;
class QShortcut;
class QLabel;
class QPushButton;
class QTreeWidget;
class QVBoxLayout;

// What looks like mess, as a list to tick, and what is known about the place
// chosen. The map is on the Browse page; this page is the list.
class MessPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void showScanning(const QStringList &roots);
    void showCount(qint64 files, qint64 bytes);
    // `space` is the room on the disk the home folder is on, measured.
    void show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QStringList &roots, const Space &space,
              const QString &home, const QStringList &never, qint64 now);
    // The list changed (a place was added, taken off or never suggested again).
    void setFound(const QList<Finding> &found, const QStringList &never);
    // The findings that are ticked.
    QList<Finding> chosen() const;
    // Chooses a place of the list, if it is on it.
    void pick(Node *node);
    // Unticks what the agent asks about first.
    void tickOnlySure();
    Node *pointed() const { return picked_; }

signals:
    void rescan();
    void handOver();
    void openFolder(const QString &path);
    void showInBrowse(Node *node);
    void takeOffRequested(Node *node);
    void neverRequested(Node *node);
    void suggestAgain(const QString &path);

private:
    void fillList();
    void updateFoot();
    void updatePanel();
    void updateTold();

    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    QStringList roots_, never_;
    QString home_;
    qint64 now_ = 0;
    Space space_;
    Node *picked_ = nullptr;
    QLabel *progress_ = nullptr, *head_ = nullptr, *foot_ = nullptr, *what_ = nullptr, *why_ = nullptr;
    QPushButton *show_ = nullptr, *open_ = nullptr, *takeOff_ = nullptr, *never_button_ = nullptr, *hand_ = nullptr;
    QWidget *panel_ = nullptr;
    QVBoxLayout *told_ = nullptr;
    QTreeWidget *list_ = nullptr;
};

// Which agent the person chose, as the helper says it (`levitate-agent --settings`): the
// hand-over page words itself for it, because who moves the places depends on it.
struct AgentChoice {
    enum Kind {
        Unknown, // the helper could not be asked
        Helper,  // Codex or Claude Code, run by the helper, which moves what the agent names
        Other    // a program chosen by its path: it is given the request as it is and moves the places itself
    };
    Kind kind = Unknown;
    bool commandsAllowed = false; // Settings let the agent run commands in the home folder without asking
    QString name;                 // "Codex", "Claude Code", or the program's file name
    QString problem;              // why it could not be asked
};

// What the agent is given: the issue, the facts (the list), the expected result.
class HandPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // What this page says before anything starts, and what the agent is given.
    struct Ask {
        QString title;       // a question
        QStringList happens; // plain sentences
        QString issue;
        QStringList facts;
        QString expected;    // the person may change it
        QString go;          // the one button that hands it over
    };
    // The mess: `holding` is the folder the agent sets what it clears aside in;
    // `freeNow` is the room on the disk now (-1 when it could not be read).
    void show(const QList<Finding> &chosen, qint64 scannedBytes, const QStringList &roots, const QString &holding,
              qint64 freeNow, const QString &home, const AgentChoice &agent);
    // Any other hand-over.
    void show(const Ask &ask);
    // Every label and button, for a check.
    QString pageText() const;
    // The text as it would be sent now, with the person's edits.
    QString prompt() const;
signals:
    void back();
    void send(const QString &prompt);
private:
    PromptPanel *prompt_ = nullptr;
    QShortcut *escape_ = nullptr;
};
