// The disk as a map: every rectangle's area is the room that thing takes.
// Mess is drawn in the colour of its kind; everything else stays quiet. The
// biggest folders carry their name and size, so the map says where things are.
#pragma once

#include "scan.h"

#include <QHash>
#include <QWidget>

#include <memory>
#include <vector>

class DiskMap : public QWidget {
    Q_OBJECT
public:
    explicit DiskMap(QWidget *parent = nullptr);
    void setTree(std::shared_ptr<Node> tree);
    // Any file or folder of the tree; it is outlined.
    void select(Node *node);
    Node *selected() const { return selected_; }
    // Lays the map out again, after the kinds of nodes changed.
    void relay();

signals:
    void picked(Node *node);

protected:
    void paintEvent(QPaintEvent *event) override;
    void resizeEvent(QResizeEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    bool event(QEvent *event) override;

private:
    struct Tile {
        QRectF rect;
        Node *node;
        int depth;
    };
    void layOut();
    void lay(Node *node, QRectF rect, int depth);
    Node *at(const QPointF &point) const;

    std::shared_ptr<Node> tree_;
    std::vector<Tile> tiles_;  // what is drawn as one rectangle
    std::vector<Tile> titles_; // the name strips of the biggest folders
    QHash<Node *, QRectF> where_;
    Node *selected_ = nullptr;
};
