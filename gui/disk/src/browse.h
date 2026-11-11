// Browse: the folder being looked at, as rows beside the map the app draws,
// with what the scan recorded about a chosen place and what can be done with
// it. Everything is read from the scan; the page changes nothing on the disk.
#pragma once

#include "mess.h"
#include "page.h"
#include "scan.h"

#include <QList>

#include <memory>

class DiskMap;
class QLabel;
class QLineEdit;
class QHBoxLayout;
class QPushButton;
class QTabBar;
class QTreeWidget;
class QTreeWidgetItem;

class BrowsePage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    enum class View { Folders, Biggest, Unused };

    // `home`: the folder to start in; `now`: seconds, for "not used for a year".
    void show(std::shared_ptr<Node> tree, const QList<Finding> &found, const QString &home, qint64 now);
    // The list of mess changed (something was added, taken off, never suggested).
    void setFound(const QList<Finding> &found);

    void goTo(Node *folder);
    void goIn();
    void up();
    void setView(View view);
    void setSearch(const QString &words);
    // Points at a place: on the map, in the rows, in the panel.
    void pick(Node *node);

    Node *folder() const { return folder_; }
    Node *pointed() const { return pointed_; }
    View view() const { return view_; }
    // The names of the rows now listed, in order, for a check.
    QStringList listed() const;
    // The row for a place, or -1.
    int rowOf(Node *node) const;

signals:
    void addRequested(Node *node);
    void takeOffRequested(Node *node);
    void neverRequested(Node *node);
    void openFolder(const QString &path);
    void said(const QString &words);
    void showMess();
    void handOver();

private:
    void fill();
    void drawPanel();
    void drawPath();
    void drawStrip();
    void drawLegend();
    Node *resolve(const QString &path) const;

    std::shared_ptr<Node> tree_;
    QList<Finding> found_;
    QString home_;
    qint64 now_ = 0;
    Node *folder_ = nullptr, *pointed_ = nullptr;
    View view_ = View::Folders;
    QString search_;
    bool filling_ = false;

    QPushButton *up_ = nullptr;
    QTabBar *views_ = nullptr;
    QLabel *legend_ = nullptr;
    QLabel *path_ = nullptr, *what_ = nullptr, *note_ = nullptr, *stripText_ = nullptr;
    QLineEdit *searchBox_ = nullptr;
    QTreeWidget *rows_ = nullptr;
    DiskMap *map_ = nullptr;
    QPushButton *goIn_ = nullptr, *open_ = nullptr, *copy_ = nullptr, *add_ = nullptr, *takeOff_ = nullptr,
                *never_ = nullptr, *listButton_ = nullptr, *handButton_ = nullptr;
};
