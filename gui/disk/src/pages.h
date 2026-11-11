// The mess list, and what the agent is given about it.
#pragma once

#include "disk.h"
#include "mess.h"
#include "page.h"
#include "scan.h"

#include <QList>

#include <memory>

class PromptPanel;
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

// What the agent is given: the issue, the facts (the list), the expected result.
class HandPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    // `holding` is the folder the agent sets what it clears aside in.
    void show(const QList<Finding> &chosen, qint64 scannedBytes, const QStringList &roots, const QString &holding);
    // Any other hand-over: a title, a line under it, and the three parts.
    void show(const QString &title, const QString &subtitle, const QString &issue, const QStringList &facts,
              const QString &expected);
    // The text as it would be sent now, with the person's edits.
    QString prompt() const;
signals:
    void back();
    void send(const QString &prompt);
private:
    PromptPanel *prompt_ = nullptr;
};
