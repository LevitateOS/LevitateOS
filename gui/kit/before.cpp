#include "before.h"

#include "parts.h"

#include <QAbstractButton>
#include <QFormLayout>
#include <QHBoxLayout>
#include <QLabel>
#include <QMouseEvent>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QScrollBar>
#include <QShortcut>
#include <QVBoxLayout>

namespace {

// One place to choose: the whole card is the click target, the radio button
// in it is what the keyboard and a screen reader reach.
class ChoiceCard : public Card {
public:
    ChoiceCard(bool on, QRadioButton *radio, QWidget *parent = nullptr)
        : Card(on ? Card::On : Card::Plain, parent)
        , radio_(radio)
    {
        setCursor(radio ? Qt::PointingHandCursor : Qt::ArrowCursor);
        // The ones not picked sit back on the window's colour.
        if (!on)
            setBackgroundRole(QPalette::Window);
        column()->setContentsMargins(12, 10, 12, 10);
        column()->setSpacing(2);
    }
protected:
    void mouseReleaseEvent(QMouseEvent *event) override
    {
        if (radio_ && event->button() == Qt::LeftButton && rect().contains(event->position().toPoint()))
            radio_->click();
        Card::mouseReleaseEvent(event);
    }
private:
    QRadioButton *radio_;
};

QHBoxLayout *bullet(const QString &sentence)
{
    auto *row = new QHBoxLayout;
    row->setSpacing(8);
    auto *dot = dim(text(QStringLiteral("•")));
    row->addWidget(dot, 0, Qt::AlignTop);
    row->addWidget(text(sentence, true), 1);
    return row;
}

} // namespace

BeforePage::BeforePage(QWidget *parent)
    : Page(parent)
{
    // Escape is Cancel while focus is anywhere on the page.
    auto *escape = new QShortcut(Qt::Key_Escape, this);
    escape->setContext(Qt::WidgetWithChildrenShortcut);
    connect(escape, &QShortcut::activated, this, &BeforePage::cancelled);
}

void BeforePage::show(const Content &content)
{
    // Drawing again (a new choice) leaves the person where they were.
    const int scrolled = scroll_ ? scroll_->verticalScrollBar()->value() : 0;
    content_ = content;
    QVBoxLayout *column = fresh();
    column->setContentsMargins(28, 20, 28, 16);

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(content.title));
    top->addWidget(dim(text(tr("Nothing has started. This is what will happen if you go on."))));
    column->addLayout(top);

    // The sections scroll; Cancel and the button under them do not.
    scroll_ = new QScrollArea;
    scroll_->setWidgetResizable(true);
    scroll_->setFrameShape(QFrame::NoFrame);
    auto *inner = new QWidget;
    auto *sections = new QVBoxLayout(inner);
    sections->setContentsMargins(0, 0, 4, 0);
    sections->setSpacing(14);

    if (!content.choices.isEmpty()) {
        auto *panel = new Panel;
        panel->column()->addWidget(strong(content.whereTitle));
        auto *cards = new QHBoxLayout;
        cards->setSpacing(14);
        const bool several = content.choices.size() > 1;
        for (const Choice &choice : content.choices) {
            const bool on = !several || choice.key == content.chosen;
            QRadioButton *radio = nullptr;
            if (several) {
                radio = new QRadioButton(choice.name);
                radio->setChecked(choice.key == content.chosen);
                QFont bold = radio->font();
                bold.setBold(true);
                radio->setFont(bold);
                const QString key = choice.key;
                connect(radio, &QRadioButton::clicked, this, [this, key] { choose(key); });
            }
            auto *card = new ChoiceCard(on, radio);
            QVBoxLayout *inside = card->column();
            inside->addWidget(radio ? static_cast<QWidget *>(radio) : strong(choice.name));
            const int indent = several ? 24 : 0;
            for (const QString &line : {choice.first, choice.second}) {
                if (line.isEmpty())
                    continue;
                auto *words = dim(text(line, true));
                words->setContentsMargins(indent, 0, 0, 0);
                inside->addWidget(words);
            }
            inside->addStretch();
            cards->addWidget(card, 1);
        }
        panel->column()->addLayout(cards);
        sections->addWidget(panel);
    }

    if (!content.happens.isEmpty()) {
        auto *panel = new Panel;
        panel->column()->addWidget(strong(content.happensTitle));
        for (const QString &sentence : content.happens)
            panel->column()->addLayout(bullet(sentence));
        sections->addWidget(panel);
    }

    if (!content.table.isEmpty() || !content.tableNote.isEmpty()) {
        auto *panel = new Panel;
        panel->column()->addWidget(strong(content.tableTitle));
        if (!content.table.isEmpty()) {
            auto *form = new QFormLayout;
            form->setHorizontalSpacing(16);
            form->setVerticalSpacing(7);
            form->setLabelAlignment(Qt::AlignLeft | Qt::AlignTop);
            for (const auto &row : content.table)
                form->addRow(dim(text(row.first)), text(row.second, true));
            panel->column()->addLayout(form);
        }
        if (!content.tableNote.isEmpty())
            panel->column()->addWidget(dim(text(content.tableNote, true)));
        sections->addWidget(panel);
    }
    sections->addStretch();
    scroll_->setWidget(inner);
    column->addWidget(scroll_, 1);
    scroll_->verticalScrollBar()->setValue(scrolled);

    auto *buttons = new QHBoxLayout;
    buttons->setSpacing(8);
    QLabel *foot = text(content.foot, true);
    if (!content.linkWords.isEmpty()) {
        foot->setText(QStringLiteral("%1 <a href=\"foot\">%2</a>")
                          .arg(content.foot.toHtmlEscaped(), content.linkWords.toHtmlEscaped()));
        foot->setTextFormat(Qt::RichText);
        foot->setTextInteractionFlags(Qt::TextBrowserInteraction);
        connect(foot, &QLabel::linkActivated, this, &BeforePage::linkClicked);
    }
    dim(foot);
    buttons->addWidget(foot, 1);
    auto *cancel = new QPushButton(tr("Cancel"));
    connect(cancel, &QPushButton::clicked, this, &BeforePage::cancelled);
    buttons->addWidget(cancel);
    auto *go = new QPushButton(content.go);
    go->setDefault(true);
    connect(go, &QPushButton::clicked, this, [this] { emit confirmed(content_.chosen); });
    buttons->addWidget(go);
    column->addLayout(buttons);
    go->setFocus();
}

void BeforePage::choose(const QString &key)
{
    if (content_.choices.size() < 2 || key == content_.chosen)
        return;
    bool known = false;
    for (const Choice &choice : content_.choices)
        known = known || choice.key == key;
    if (!known)
        return;
    // This page draws the new card at once; the app may give new content in
    // answer, which draws it once more with its own words.
    Content again = content_;
    again.chosen = key;
    show(again);
    emit choiceChanged(key);
}

QString BeforePage::pageText() const
{
    // The page's current body: the old one is deleted later and must not be read.
    QWidget *body = scroll_ ? scroll_->parentWidget() : nullptr;
    if (!body)
        return QString();
    QStringList words;
    for (const QLabel *label : body->findChildren<QLabel *>())
        words << label->text();
    for (const QAbstractButton *button : body->findChildren<QAbstractButton *>())
        words << button->text();
    return words.join(QLatin1Char('\n'));
}

QStringList BeforePage::choiceNames() const
{
    QStringList names;
    for (const Choice &choice : content_.choices)
        names << choice.name;
    return names;
}
