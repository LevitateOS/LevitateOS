#include "parts.h"

#include <QFileInfo>
#include <QFormLayout>
#include <QKeyEvent>
#include <QPainter>
#include <QPainterPath>
#include <QPlainTextEdit>
#include <QSvgRenderer>
#include <QVBoxLayout>

QLabel *text(const QString &words, bool wrap)
{
    auto *label = new QLabel(words);
    label->setWordWrap(wrap);
    label->setTextInteractionFlags(Qt::TextSelectableByMouse);
    return label;
}

QLabel *dim(QLabel *label)
{
    QPalette palette = label->palette();
    QColor colour = palette.color(QPalette::WindowText);
    colour.setAlphaF(0.66f);
    palette.setColor(QPalette::WindowText, colour);
    label->setPalette(palette);
    return label;
}

QLabel *strong(const QString &words)
{
    auto *label = new QLabel(words);
    QFont font = label->font();
    font.setBold(true);
    label->setFont(font);
    return label;
}

QLabel *heading(const QString &words)
{
    auto *label = new QLabel(words);
    QFont font = label->font();
    font.setPointSizeF(font.pointSizeF() * 1.55);
    font.setWeight(QFont::DemiBold);
    label->setFont(font);
    return label;
}

namespace {
const QColor wrong(0xc2, 0x3b, 0x33);

class Rule : public QWidget {
public:
    Rule() { setFixedHeight(1); }
protected:
    void paintEvent(QPaintEvent *) override
    {
        QColor colour = palette().color(QPalette::WindowText);
        colour.setAlphaF(0.13f);
        QPainter(this).fillRect(rect(), colour);
    }
};
} // namespace

QWidget *rule()
{
    return new Rule;
}

Card::Card(Tone tone, QWidget *parent)
    : Panel(parent)
    , tone_(tone)
{
    if (tone != Plain)
        setFrameShape(QFrame::NoFrame);
}

void Card::paintEvent(QPaintEvent *event)
{
    Panel::paintEvent(event);
    if (tone_ == Plain)
        return;
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    QPen pen(palette().color(QPalette::Highlight), 1.5);
    if (tone_ == Open)
        pen = QPen(QColor(0xd9, 0x9a, 0x2b), 1.5, Qt::DashLine);
    else if (tone_ == Ready)
        pen.setColor(QColor(0x4f, 0xa8, 0x66));
    else if (tone_ == Wrong)
        pen.setColor(wrong);
    painter.setPen(pen);
    painter.setBrush(Qt::NoBrush);
    painter.drawRoundedRect(QRectF(rect()).adjusted(0.75, 0.75, -0.75, -0.75), 4, 4);
}

Glyph::Glyph(Kind kind, bool filled, QWidget *parent)
    : QWidget(parent)
    , kind_(kind)
    , filled_(filled)
{
    setFixedSize(52, 52);
}

void Glyph::paintEvent(QPaintEvent *)
{
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    QColor ground = palette().color(QPalette::WindowText);
    ground.setAlphaF(0.09f);
    QColor ink = palette().color(QPalette::WindowText);
    ink.setAlphaF(0.72f);
    if (filled_) {
        ground = palette().color(QPalette::Highlight);
        ink = palette().color(QPalette::HighlightedText);
    }
    painter.setPen(Qt::NoPen);
    painter.setBrush(ground);
    painter.drawRoundedRect(rect(), 11, 11);

    // The pictures are drawn in a 24 by 24 box in the middle of the tile.
    painter.translate((width() - 24) / 2.0, (height() - 24) / 2.0);
    painter.setPen(QPen(ink, 1.8, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
    painter.setBrush(Qt::NoBrush);
    switch (kind_) {
    case Lock: {
        painter.drawRoundedRect(QRectF(5.5, 11, 13, 9.5), 2, 2);
        QPainterPath shackle;
        shackle.moveTo(8.5, 11);
        shackle.lineTo(8.5, 8);
        shackle.arcTo(QRectF(8.5, 4.5, 7, 7), 180, -180);
        shackle.lineTo(15.5, 11);
        painter.drawPath(shackle);
        break;
    }
    case Plus:
        painter.drawRoundedRect(QRectF(4, 4, 16, 16), 3, 3);
        painter.drawLine(QPointF(12, 8), QPointF(12, 16));
        painter.drawLine(QPointF(8, 12), QPointF(16, 12));
        break;
    case Before:
        painter.drawArc(QRectF(4.5, 4.5, 15, 15), 150 * 16, -300 * 16);
        painter.drawPolyline(QPolygonF{QPointF(3.2, 4.6), QPointF(5.3, 8.4), QPointF(9.2, 6.6)});
        painter.drawPolyline(QPolygonF{QPointF(12, 8.5), QPointF(12, 12), QPointF(14.6, 13.6)});
        break;
    case Check:
        painter.drawPolyline(QPolygonF{QPointF(5, 12.5), QPointF(10, 17.5), QPointF(19, 7)});
        break;
    case Cross:
        painter.drawLine(QPointF(6, 6), QPointF(18, 18));
        painter.drawLine(QPointF(18, 6), QPointF(6, 18));
        break;
    }
}

void Verdict::paintEvent(QPaintEvent *event)
{
    Panel::paintEvent(event);
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    painter.setPen(QPen(wrong, 1.5));
    painter.setBrush(Qt::NoBrush);
    painter.drawRoundedRect(QRectF(rect()).adjusted(0.75, 0.75, -0.75, -0.75), 4, 4);
}

Panel::Panel(QWidget *parent)
    : QFrame(parent)
{
    setFrameShape(QFrame::StyledPanel);
    setBackgroundRole(QPalette::Base);
    setAutoFillBackground(true);
    auto *layout = new QVBoxLayout(this);
    layout->setContentsMargins(18, 14, 18, 14);
    layout->setSpacing(10);
}

QVBoxLayout *Panel::column()
{
    return static_cast<QVBoxLayout *>(layout());
}

ClickLabel::ClickLabel(const QString &words, QWidget *parent)
    : QLabel(words, parent)
{
    setCursor(Qt::PointingHandCursor);
    setFocusPolicy(Qt::TabFocus);
}

void ClickLabel::mouseReleaseEvent(QMouseEvent *event)
{
    if (event->button() == Qt::LeftButton && rect().contains(event->pos()))
        emit clicked();
}

void ClickLabel::keyPressEvent(QKeyEvent *event)
{
    if (event->key() == Qt::Key_Return || event->key() == Qt::Key_Enter || event->key() == Qt::Key_Space)
        emit clicked();
    else
        QLabel::keyPressEvent(event);
}

void ClickLabel::focusInEvent(QFocusEvent *event)
{
    QFont underlined = font();
    underlined.setUnderline(true);
    setFont(underlined);
    QLabel::focusInEvent(event);
}

void ClickLabel::focusOutEvent(QFocusEvent *event)
{
    QFont plain = font();
    plain.setUnderline(false);
    setFont(plain);
    QLabel::focusOutEvent(event);
}

Tile::Tile(const QString &name, const QString &iconPath, int size, QWidget *parent)
    : QWidget(parent)
    , letter_(name.left(1))
    , iconPath_(QFileInfo::exists(iconPath) ? iconPath : QString())
{
    setFixedSize(size, size);
}

void Tile::paintEvent(QPaintEvent *)
{
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    if (!iconPath_.isEmpty()) {
        QSvgRenderer(iconPath_).render(&painter, rect());
        return;
    }
    QColor ground = palette().color(QPalette::WindowText);
    ground.setAlphaF(0.09f);
    painter.setPen(Qt::NoPen);
    painter.setBrush(ground);
    painter.drawRoundedRect(rect(), width() / 4.5, width() / 4.5);
    QFont font = painter.font();
    font.setPixelSize(width() * 0.44);
    font.setWeight(QFont::DemiBold);
    painter.setFont(font);
    painter.setPen(palette().color(QPalette::Link));
    painter.drawText(rect(), Qt::AlignCenter, letter_);
}

PromptPanel::PromptPanel(const QString &issue, const QStringList &facts, const QString &expected, QWidget *parent)
    : Panel(parent)
    , issue_(issue)
    , facts_(facts)
    , expected_(new QPlainTextEdit(expected))
{
    auto *title = new QVBoxLayout;
    title->setSpacing(0);
    title->addWidget(strong(tr("What the agent is given")));
    title->addWidget(dim(text(tr("You can change the expected result: it says what should be true afterwards."))));
    column()->addLayout(title);
    auto *form = new QFormLayout;
    form->setHorizontalSpacing(16);
    form->setVerticalSpacing(8);
    form->addRow(strong(tr("The issue")), text(issue, true));
    form->addRow(strong(tr("The facts")), text(QStringLiteral("• ") + facts.join(QStringLiteral("\n• ")), true));
    expected_->setAccessibleName(tr("The expected result"));
    expected_->setFixedHeight(66);
    form->addRow(strong(tr("The expected result")), expected_);
    column()->addLayout(form);
}

QString PromptPanel::prompt() const
{
    return tr("The issue:\n%1\n\nThe facts:\n- %2\n\nThe expected result:\n%3\n")
        .arg(issue_, facts_.join(QStringLiteral("\n- ")), expected_->toPlainText().trimmed());
}

StepMark::StepMark(int number, QWidget *parent)
    : QWidget(parent)
    , number_(number)
{
    setFixedSize(26, 26);
}

void StepMark::setState(State state)
{
    state_ = state;
    update();
}

void StepMark::paintEvent(QPaintEvent *)
{
    QPainter painter(this);
    painter.setRenderHint(QPainter::Antialiasing);
    const QRectF circle = QRectF(rect()).adjusted(1.5, 1.5, -1.5, -1.5);
    const QColor accent = palette().color(QPalette::Highlight);
    QColor quiet = palette().color(QPalette::WindowText);
    quiet.setAlphaF(0.5f);

    QColor line = quiet, fill = Qt::transparent, ink = quiet;
    QString mark = QString::number(number_);
    switch (state_) {
    case Pending:
        break;
    case Running:
        line = accent;
        ink = palette().color(QPalette::WindowText);
        break;
    case Done:
        line = fill = accent;
        ink = palette().color(QPalette::HighlightedText);
        mark.clear();
        break;
    case Skipped:
        mark = QStringLiteral("–");
        break;
    case Failed:
        line = fill = wrong;
        ink = Qt::white;
        mark.clear();
        break;
    }
    painter.setPen(QPen(line, state_ == Running ? 2.0 : 1.2));
    painter.setBrush(fill);
    painter.drawEllipse(circle);
    QFont font = painter.font();
    font.setPixelSize(12);
    font.setBold(true);
    painter.setFont(font);
    painter.setPen(ink);
    if (!mark.isEmpty()) {
        painter.drawText(rect(), Qt::AlignCenter, mark);
        return;
    }
    // Drawn, not typed: the desktop's font may have no tick or cross.
    painter.setPen(QPen(ink, 2, Qt::SolidLine, Qt::RoundCap, Qt::RoundJoin));
    const QPointF c = circle.center();
    if (state_ == Done)
        painter.drawPolyline(QPolygonF{c + QPointF(-5, 0.5), c + QPointF(-1.5, 4), c + QPointF(5, -3.5)});
    else {
        painter.drawLine(c + QPointF(-4, -4), c + QPointF(4, 4));
        painter.drawLine(c + QPointF(4, -4), c + QPointF(-4, 4));
    }
}
