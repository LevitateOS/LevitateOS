// The two pages: where the mess is, and what the agent is given about it.
#pragma once

#include "mess.h"
#include "page.h"
#include "scan.h"

#include <QList>

#include <memory>

class DiskMap;
class PromptPanel;
class QLabel;
class QPushButton;
class QTreeWidget;
class QTreeWidgetItem;

// The scanned disk as a map and a list of what looks like mess.
class MessPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void showScanning(const QStringList &roots);
    void showCount(qint64 files, qint64 bytes);
    // `freed` is what the last hand-over to the agent gave back, if any.
    void show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QStringList &roots, qint64 freeBytes,
              qint64 freed, const QString &home);
    // The findings that are ticked.
    QList<Finding> chosen() const;
    // Points at a file or folder: on the map, in the line under it, in the list.
    void pick(Node *node);
    // Puts what is pointed at on the list, as something the agent looks into.
    void addPicked();
    // The biggest folder the rules say nothing about, or null.
    Node *biggestUnnamed() const;

signals:
    void rescan();
    void handOver();

private:
    void fillList();
    void updateFoot();
    const Finding *findingAround(Node *node) const;

    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    QString home_;
    Node *picked_ = nullptr;
    QLabel *progress_ = nullptr, *head_ = nullptr, *foot_ = nullptr, *where_ = nullptr;
    QPushButton *add_ = nullptr, *hand_ = nullptr;
    QTreeWidget *list_ = nullptr;
    DiskMap *map_ = nullptr;
};

// What the agent is given: the issue, the facts (the list), the expected result.
class HandPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const QList<Finding> &chosen, qint64 scannedBytes, const QStringList &roots);
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
