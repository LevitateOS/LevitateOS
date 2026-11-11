#include "map.h"

#include "mess.h"
#include "words.h"

#include <QHelpEvent>
#include <QMouseEvent>
#include <QPainter>
#include <QToolTip>

#include <limits>

DiskMap::DiskMap(QWidget *parent)
    : QWidget(parent)
{
    setMinimumSize(320, 240);
    setMouseTracking(true);
}

void DiskMap::setTree(std::shared_ptr<Node> tree)
{
    tree_ = std::move(tree);
    selected_ = nullptr;
    top_ = nullptr;
    layOut();
    update();
}

void DiskMap::showFolder(Node *folder)
{
    top_ = folder;
    selected_ = nullptr;
    layOut();
    update();
}

void DiskMap::select(Node *node)
{
    selected_ = node;
    update();
}

void DiskMap::relay()
{
    layOut();
    update();
}

void DiskMap::resizeEvent(QResizeEvent *)
{
    layOut();
}

void DiskMap::layOut()
{
    tiles_.clear();
    titles_.clear();
    where_.clear();
    Node *root = top_ ? top_ : tree_.get();
    if (root && root->bytes > 0)
        lay(root, QRectF(rect()), 0);
}

// A squarified layout: the children of a folder are placed in rows along the
// shorter side, each row grown for as long as its rectangles get squarer.
void DiskMap::lay(Node *node, QRectF area, int depth)
{
    if (area.width() < 3 || area.height() < 3)
        return;
    where_.insert(node, area);
    const bool whole = !node->dir || node->children.empty() || (depth > 0 && node->kind >= 0) || depth > 8
                       || area.width() * area.height() < 700;
    if (whole) {
        tiles_.push_back({area, node, depth});
        return;
    }
    QRectF rest = depth ? area.adjusted(1, 1, -1, -1) : area;
    // The biggest folders get a strip with their name on it.
    if (depth >= 1 && depth <= 6 && rest.width() > 110 && rest.height() > 64) {
        const double strip = QFontMetricsF(font()).height() + 3;
        titles_.push_back({QRectF(rest.left(), rest.top(), rest.width(), strip), node, depth});
        rest.setTop(rest.top() + strip);
    }
    double left = double(node->bytes);
    size_t first = 0;
    const auto &items = node->children;
    while (first < items.size() && left > 0 && rest.width() >= 1 && rest.height() >= 1) {
        const bool alongHeight = rest.width() >= rest.height(); // the row is a column on the left
        const double side = alongHeight ? rest.height() : rest.width();
        const double scale = rest.width() * rest.height() / left;
        double rowArea = 0, best = std::numeric_limits<double>::max();
        size_t last = first;
        while (last < items.size() && items[last]->bytes > 0) {
            const double grown = rowArea + double(items[last]->bytes) * scale;
            const double thickness = grown / side;
            double worst = 0;
            for (size_t i = first; i <= last; ++i) {
                const double length = double(items[i]->bytes) * scale / thickness;
                worst = std::max(worst, std::max(length / thickness, thickness / length));
            }
            if (worst > best)
                break;
            best = worst;
            rowArea = grown;
            ++last;
        }
        if (last == first)
            break;
        const double thickness = rowArea / side;
        double offset = 0, rowBytes = 0;
        for (size_t i = first; i < last; ++i) {
            const double length = double(items[i]->bytes) * scale / thickness;
            const QRectF tile = alongHeight ? QRectF(rest.left(), rest.top() + offset, thickness, length)
                                            : QRectF(rest.left() + offset, rest.top(), length, thickness);
            lay(items[i].get(), tile, depth + 1);
            offset += length;
            rowBytes += double(items[i]->bytes);
        }
        if (alongHeight)
            rest.setLeft(rest.left() + thickness);
        else
            rest.setTop(rest.top() + thickness);
        left -= rowBytes;
        first = last;
    }
}

bool DiskMap::loud(const Node *node) const
{
    return node->kind >= 0;
}

QColor DiskMap::fillOf(const Node *node, int depth) const
{
    if (node->kind >= 0)
        return messKinds().at(node->kind).colour;
    // Quiet, with a little variation so neighbours can be told apart.
    QColor fill = palette().color(QPalette::WindowText);
    fill.setAlphaF(0.07f + 0.035f * float((depth + (qHash(node->name) & 3)) % 4));
    return fill;
}

qint64 DiskMap::sizeOf(const Node *node) const
{
    return node->bytes;
}

QString DiskMap::tipOf(const Node *node) const
{
    return node->path() + QLatin1Char('\n') + bytesText(node->bytes);
}

void DiskMap::decorate(QPainter &, const Node *, const QRectF &) const
{
}

void DiskMap::paintEvent(QPaintEvent *)
{
    QPainter painter(this);
    const QColor ground = palette().color(QPalette::Base);
    painter.fillRect(rect(), ground);
    const QColor ink = palette().color(QPalette::WindowText);
    for (const Tile &tile : tiles_) {
        const QRectF inner = tile.rect.adjusted(0.5, 0.5, -0.5, -0.5);
        painter.fillRect(inner, fillOf(tile.node, tile.depth));
        // Mess is always named when there is room; the rest only when it is large.
        const bool mess = loud(tile.node);
        if (inner.width() > 90 && inner.height() > (mess ? 22 : 60)) {
            QColor words = mess ? QColor(0x14, 0x18, 0x16) : ink;
            if (!mess)
                words.setAlphaF(0.6f);
            painter.setPen(words);
            QFont small = font();
            small.setPointSizeF(small.pointSizeF() * 0.9);
            painter.setFont(small);
            const QFontMetricsF metrics(small);
            const QRectF room = inner.adjusted(6, 4, -6, -4);
            const QString name = metrics.elidedText(tile.node->name, Qt::ElideMiddle, room.width());
            // Two lines when they fit, otherwise the name alone.
            const bool both = room.height() >= 2 * metrics.lineSpacing();
            painter.drawText(room, Qt::AlignLeft | Qt::AlignTop,
                             both ? name + QLatin1Char('\n') + bytesText(sizeOf(tile.node)) : name);
        }
        decorate(painter, tile.node, inner);
    }
    QFont small = font();
    small.setPointSizeF(small.pointSizeF() * 0.9);
    painter.setFont(small);
    const QFontMetricsF metrics(small);
    QColor words = ink;
    words.setAlphaF(0.75f);
    painter.setPen(words);
    for (const Tile &title : titles_) {
        const QRectF room = title.rect.adjusted(4, 0, -4, 0);
        const QString size = bytesText(sizeOf(title.node));
        const double sizeWidth = metrics.horizontalAdvance(size) + 10;
        // A root is named by its whole path; that is too long for a strip.
        QString name = title.node->name;
        if (title.depth == 1 && !name.section(QLatin1Char('/'), -1).isEmpty())
            name = name.section(QLatin1Char('/'), -1);
        painter.drawText(room, Qt::AlignLeft | Qt::AlignVCenter,
                         metrics.elidedText(name, Qt::ElideMiddle, room.width() - sizeWidth));
        painter.drawText(room, Qt::AlignRight | Qt::AlignVCenter, size);
    }
    if (selected_ && where_.contains(selected_)) {
        // Dark under light, so the outline shows on any colour.
        const QRectF outline = where_.value(selected_).adjusted(1.5, 1.5, -1.5, -1.5);
        painter.setPen(QPen(ground, 4));
        painter.drawRect(outline);
        painter.setPen(QPen(ink, 2));
        painter.drawRect(outline);
    }
}

Node *DiskMap::at(const QPointF &point) const
{
    // The innermost name strip wins over the folder around it.
    for (auto title = titles_.rbegin(); title != titles_.rend(); ++title)
        if (title->rect.contains(point))
            return title->node;
    for (const Tile &tile : tiles_)
        if (tile.rect.contains(point))
            return tile.node;
    return nullptr;
}

void DiskMap::mousePressEvent(QMouseEvent *event)
{
    if (Node *node = at(event->position())) {
        selected_ = node;
        update();
        emit picked(node);
    }
}

bool DiskMap::event(QEvent *event)
{
    if (event->type() == QEvent::ToolTip) {
        auto *help = static_cast<QHelpEvent *>(event);
        if (Node *node = at(help->pos()))
            QToolTip::showText(help->globalPos(), tipOf(node), this);
        else
            QToolTip::hideText();
        return true;
    }
    return QWidget::event(event);
}
