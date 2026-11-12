#include "activity.h"

#include "parts.h"
#include "run.h"
#include "words.h"

#include <QComboBox>
#include <QDateTime>
#include <QFileInfo>
#include <QFontDatabase>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonDocument>
#include <QJsonObject>
#include <QListWidget>
#include <QPainter>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QScrollArea>
#include <QScrollBar>
#include <QStyledItemDelegate>
#include <QTimer>
#include <QVBoxLayout>
#include <QLineEdit>
#include <QShowEvent>

namespace {

enum Role { IdRole = Qt::UserRole, TitleRole, StateRole, MoodRole, MetaRole };

QLabel *mono(const QString &words)
{
    auto *label = new QLabel(words);
    label->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
    label->setTextInteractionFlags(Qt::TextSelectableByMouse);
    label->setWordWrap(true);
    return label;
}

QString firstLine(const QString &words)
{
    return words.section(QLatin1Char('\n'), 0, 0).trimmed();
}

// "today at 14:10", "yesterday at 09:30", "9 November".
QString whenText(const QString &iso)
{
    const QDateTime when = QDateTime::fromString(iso, Qt::ISODateWithMs).toLocalTime();
    if (!when.isValid())
        return QString();
    const QLocale locale;
    const QDate today = QDate::currentDate();
    const QString clock = locale.toString(when.time(), QLocale::ShortFormat);
    if (when.date() == today)
        return QObject::tr("today at %1").arg(clock);
    if (when.date() == today.addDays(-1))
        return QObject::tr("yesterday at %1").arg(clock);
    return locale.toString(when.date(), when.date().year() == today.year() ? QStringLiteral("d MMMM")
                                                                            : QStringLiteral("d MMMM yyyy"));
}

QString appText(const QString &app)
{
    return app.isEmpty() ? QObject::tr("An app that did not say which") : app;
}

QString agentWords(const QString &agent)
{
    if (agent == QLatin1String("codex"))
        return QStringLiteral("Codex");
    if (agent == QLatin1String("claude"))
        return QStringLiteral("Claude Code");
    return agent.isEmpty() ? QObject::tr("The agent") : QFileInfo(agent).fileName();
}

QString titleOf(const QJsonObject &run)
{
    const QString title = run["title"].toString().trimmed();
    return title.isEmpty() ? QObject::tr("A request without a title") : title;
}

struct State {
    QString words;
    Mood mood;
};

// One line of state in words.
State stateOf(const QJsonObject &run)
{
    const QString state = run["state"].toString();
    const QString summary = firstLine(run["summary"].toString());
    if (state == QLatin1String("waiting"))
        return {QObject::tr("Waits for an answer"), Mood::Warn};
    if (state == QLatin1String("running"))
        return {QObject::tr("Running now"), Mood::Plain};
    if (state == QLatin1String("done"))
        return {summary.isEmpty() ? QObject::tr("Done") : QObject::tr("Done: %1").arg(summary), Mood::Ok};
    if (state == QLatin1String("failed"))
        return {summary.isEmpty() ? QObject::tr("Failed") : QObject::tr("Failed: %1").arg(summary), Mood::Bad};
    return {QObject::tr("Stopped before it finished"), Mood::Plain};
}

// The prompt the apps send has up to four headed parts; this reads them.
struct Part {
    QString heading, body;
};

QList<Part> partsOf(const QString &prompt)
{
    static const QStringList headings{"The request:", "The issue:", "The facts:", "The expected result:"};
    QList<Part> parts;
    for (const QString &line : prompt.split(QLatin1Char('\n'))) {
        if (headings.contains(line.trimmed())) {
            parts << Part{line.trimmed().chopped(1), QString()};
        } else if (!parts.isEmpty()) {
            parts.last().body += line + QLatin1Char('\n');
        }
    }
    for (Part &part : parts)
        part.body = part.body.trimmed();
    return parts;
}

// One row of the list: the title, one line of state, "App · when".
class RunDelegate : public QStyledItemDelegate {
public:
    using QStyledItemDelegate::QStyledItemDelegate;
    QSize sizeHint(const QStyleOptionViewItem &option, const QModelIndex &) const override
    {
        // Three lines and some air, whatever the font is.
        return {100, 3 * (QFontMetrics(option.font).height() + 1) + 18};
    }
    void paint(QPainter *painter, const QStyleOptionViewItem &option, const QModelIndex &index) const override
    {
        painter->save();
        const QRect r = option.rect;
        const QColor ink = option.palette.color(QPalette::Text);
        if (option.state & QStyle::State_Selected) {
            QColor tint = option.palette.color(QPalette::Highlight);
            tint.setAlphaF(0.16f);
            painter->fillRect(r, tint);
            painter->fillRect(QRect(r.left(), r.top(), 3, r.height()), option.palette.color(QPalette::Highlight));
        } else if (option.state & QStyle::State_MouseOver) {
            QColor tint = ink;
            tint.setAlphaF(0.06f);
            painter->fillRect(r, tint);
        }
        if (index.row() > 0) {
            QColor line = ink;
            line.setAlphaF(0.13f);
            painter->fillRect(QRect(r.left(), r.top(), r.width(), 1), line);
        }
        const QFontMetrics metrics(option.font);
        const int left = r.left() + 14, width = r.width() - 28, line = metrics.height() + 1;
        int y = r.top() + 8;
        QFont bold = option.font;
        bold.setBold(true);
        painter->setFont(bold);
        painter->setPen(ink);
        painter->drawText(QRect(left, y, width, line), Qt::AlignVCenter,
                          QFontMetrics(bold).elidedText(index.data(TitleRole).toString(), Qt::ElideRight, width));
        y += line + 1;
        painter->setFont(option.font);
        painter->setPen(moodColour(option.palette, Mood(index.data(MoodRole).toInt())));
        painter->drawText(QRect(left, y, width, line), Qt::AlignVCenter,
                          metrics.elidedText(index.data(StateRole).toString(), Qt::ElideRight, width));
        y += line + 1;
        painter->setPen(moodColour(option.palette, Mood::Plain));
        painter->drawText(QRect(left, y, width, line), Qt::AlignVCenter,
                          metrics.elidedText(index.data(MetaRole).toString(), Qt::ElideRight, width));
        painter->restore();
    }
};

Panel *group(const QString &title, QVBoxLayout **inside = nullptr)
{
    auto *panel = new Panel;
    panel->column()->setSpacing(6);
    panel->column()->addWidget(strong(title));
    if (inside)
        *inside = panel->column();
    return panel;
}

} // namespace

ActivityPage::ActivityPage(const QString &app, QWidget *parent)
    : Page(parent)
    , app_(app)
    , program_(qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT", "levitate-agent"))
{
    // The list is read again at a modest pace, and only while the page is on
    // screen, so the counts and the states follow runs of other windows.
    poll_ = new QTimer(this);
    poll_->setInterval(2000);
    connect(poll_, &QTimer::timeout, this, [this] {
        if (isVisible())
            read();
    });
    // Many events can come in one breath (the replay of a run that has been
    // going): the detail is drawn once for them.
    redraw_ = new QTimer(this);
    redraw_->setSingleShot(true);
    redraw_->setInterval(60);
    connect(redraw_, &QTimer::timeout, this, [this] {
        if (attached_ && live_)
            showDetail();
    });
}

void ActivityPage::showEvent(QShowEvent *event)
{
    Page::showEvent(event);
    poll_->start();
    read();
    if (loaded_ && !chosen_.isEmpty() && !attached_)
        fetchDetail();
}

void ActivityPage::hideEvent(QHideEvent *event)
{
    Page::hideEvent(event);
    poll_->stop();
    // Nobody looks: the run is let go of, and goes on.
    detach();
}

void ActivityPage::setOwnRun(const QString &id)
{
    own_ = id;
    if (loaded_ && !lines_.isEmpty())
        showDetail();
}

void ActivityPage::read()
{
    if (listing_)
        return;
    if (!loaded_ && problem_.isEmpty())
        showLoading(tr("Activity"), tr("Asking %1 for the runs it kept.").arg(QFileInfo(program_).fileName()));
    listing_ = new Run(program_, {"--list"}, this);
    connect(listing_, &Run::finished, this, [this](bool ok) {
        Run *run = listing_;
        listing_ = nullptr;
        run->deleteLater();
        const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
        if (!ok || !answer.contains("runs")) {
            problem_ = tr("`%1 --list` failed: %2")
                           .arg(QFileInfo(program_).fileName(),
                                run->problem().isEmpty() ? tr("it did not answer with a list of runs") : run->problem());
            runs_ = {};
            loaded_ = false;
            waiting_ = running_ = 0;
            emit counts(0, false);
            render();
            return;
        }
        const bool same = loaded_ && problem_.isEmpty() && runs_ == answer["runs"].toArray();
        problem_.clear();
        loaded_ = true;
        runs_ = answer["runs"].toArray();
        waiting_ = running_ = 0;
        for (const QJsonValue &value : runs_) {
            const QString state = value["state"].toString();
            waiting_ += state == QLatin1String("waiting");
            running_ += state == QLatin1String("running");
        }
        emit counts(waiting_ + running_, waiting_ > 0);
        // A list that reads as before changes nothing on screen (and leaves a
        // half-typed answer alone).
        if (!same)
            render();
    });
    listing_->start();
}

QJsonArray ActivityPage::filtered() const
{
    if (!onlyThis_)
        return runs_;
    QJsonArray only;
    for (const QJsonValue &run : runs_)
        if (run["app"].toString().compare(app_, Qt::CaseInsensitive) == 0)
            only << run;
    return only;
}

void ActivityPage::showOnlyThisApp(bool only)
{
    onlyThis_ = only;
    render();
}

void ActivityPage::choose(const QString &id)
{
    if (id == chosen_ || !list_)
        return;
    for (int row = 0; row < list_->count(); ++row)
        if (list_->item(row)->data(IdRole).toString() == id)
            list_->setCurrentRow(row);
}

void ActivityPage::chooseTitle(const QString &title)
{
    if (!list_)
        return;
    for (int row = 0; row < list_->count(); ++row)
        if (list_->item(row)->data(TitleRole).toString() == title)
            list_->setCurrentRow(row);
}

void ActivityPage::render()
{
    if (list_)
        listScroll_ = list_->verticalScrollBar()->value();
    rememberTyping();
    list_ = nullptr;
    detail_ = nullptr;
    stepLabel_ = tailLabel_ = nullptr;
    typing_ = nullptr;
    QVBoxLayout *column = fresh();

    if (!problem_.isEmpty()) {
        column->addWidget(heading(tr("Activity could not be read")));
        column->addWidget(text(problem_, true));
        auto *buttons = new QHBoxLayout;
        auto *again = new QPushButton(tr("Try again"));
        again->setDefault(true);
        connect(again, &QPushButton::clicked, this, &ActivityPage::read);
        buttons->addWidget(again);
        buttons->addStretch();
        column->addLayout(buttons);
        column->addStretch();
        return;
    }

    const QJsonArray shown = filtered();
    bool present = false;
    for (const QJsonValue &run : shown)
        present |= run["id"].toString() == chosen_;
    if (!present)
        chosen_ = shown.isEmpty() ? QString() : shown.first()["id"].toString();
    if (attachedId_ != chosen_)
        detach();

    auto *both = new QHBoxLayout;
    both->setSpacing(16);
    auto *left = new QWidget;
    left->setFixedWidth(330);
    auto *side = new QVBoxLayout(left);
    side->setContentsMargins(0, 0, 0, 0);
    side->setSpacing(10);
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Activity")));
    top->addWidget(dim(text(tr("From all three apps. A run goes on when its window closes."), true)));
    side->addLayout(top);

    int waits = 0, runs = 0;
    for (const QJsonValue &run : shown) {
        waits += run["state"].toString() == QLatin1String("waiting");
        runs += run["state"].toString() == QLatin1String("running");
    }
    auto *chips = new QHBoxLayout;
    chips->setSpacing(6);
    if (waits)
        chips->addWidget(new Chip(waits == 1 ? tr("1 waits for an answer") : tr("%1 wait for an answer").arg(waits),
                                  Mood::Warn));
    if (runs)
        chips->addWidget(new Chip(tr("%1 running").arg(runs)));
    if (!waits && !runs)
        chips->addWidget(dim(text(tr("Nothing waits or runs."))));
    chips->addStretch();
    side->addLayout(chips);

    auto *filter = new QHBoxLayout;
    filter->addWidget(dim(text(tr("Show"))));
    auto *which = new QComboBox;
    which->setAccessibleName(tr("Show runs from"));
    which->addItems({tr("All apps"), tr("This app (%1)").arg(app_)});
    which->setCurrentIndex(onlyThis_ ? 1 : 0);
    connect(which, &QComboBox::activated, this, [this](int index) { showOnlyThisApp(index == 1); });
    filter->addWidget(which, 1);
    side->addLayout(filter);

    list_ = new QListWidget;
    list_->setAccessibleName(tr("Runs, newest first"));
    list_->setItemDelegate(new RunDelegate(list_));
    list_->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
    list_->setMouseTracking(true);
    for (const QJsonValue &value : shown) {
        const QJsonObject run = value.toObject();
        const State state = stateOf(run);
        const QString when = whenText(run["started"].toString());
        auto *item = new QListWidgetItem(list_);
        item->setData(IdRole, run["id"].toString());
        item->setData(TitleRole, titleOf(run));
        item->setData(StateRole, state.words);
        item->setData(MoodRole, int(state.mood));
        item->setData(MetaRole, when.isEmpty() ? appText(run["app"].toString())
                                               : tr("%1 · %2").arg(appText(run["app"].toString()), when));
        // Read out by a screen reader as one sentence.
        item->setData(Qt::AccessibleTextRole, tr("%1. %2. %3").arg(titleOf(run), state.words, item->data(MetaRole).toString()));
        if (run["id"].toString() == chosen_)
            list_->setCurrentItem(item);
    }
    list_->verticalScrollBar()->setValue(listScroll_);
    connect(list_, &QListWidget::currentItemChanged, this, [this](QListWidgetItem *now) {
        if (!now || now->data(IdRole).toString() == chosen_)
            return;
        chosen_ = now->data(IdRole).toString();
        lines_ = {};
        typed_.clear();
        detach();
        fetchDetail();
    });
    if (shown.isEmpty()) {
        list_->hide();
        side->addWidget(dim(text(onlyThis_ ? tr("This app has asked the agent for nothing yet.")
                                           : tr("Nothing has been asked of the agent yet. What you ask it to do in "
                                                "Recipe, Recab or Recdirstat is listed here."),
                                 true)));
    } else {
        side->addWidget(list_, 1);
    }
    if (shown.isEmpty())
        side->addStretch();
    // Said once, so that nobody looks here for an install or a restart.
    side->addWidget(dim(text(tr("Only the agent's runs are kept. Installs and restarts are not listed."), true)));
    both->addWidget(left);

    detail_ = new QScrollArea;
    detail_->setWidgetResizable(true);
    detail_->setFrameShape(QFrame::NoFrame);
    both->addWidget(detail_, 1);
    column->addLayout(both, 1);
    // A run that is being followed has more than the record knows.
    if (attached_ && live_ && attachedId_ == chosen_)
        showDetail();
    else
        fetchDetail();
}

void ActivityPage::fetchDetail()
{
    if (chosen_.isEmpty()) {
        lines_ = {};
        detailId_.clear();
        showDetail();
        return;
    }
    if (showing_)
        return; // when it ends it looks at what is chosen by then
    if (lines_.isEmpty() && detail_) {
        auto *wait = new QLabel(tr("Reading the run…"));
        dim(wait);
        detail_->setWidget(wait);
    }
    const QString id = chosen_;
    showing_ = new Run(program_, {"--show", id}, this);
    connect(showing_, &Run::finished, this, [this, id](bool ok) {
        Run *run = showing_;
        showing_ = nullptr;
        run->deleteLater();
        if (id != chosen_) {
            fetchDetail();
            return;
        }
        const QJsonDocument document = QJsonDocument::fromJson(run->answer());
        detailBroken_ = !ok || !document.isArray() || document.array().isEmpty();
        lines_ = detailBroken_ ? QJsonArray() : document.array();
        detailId_ = id;
        // Followed already (an event came while the record was read): that is the fuller picture.
        if (!(attached_ && live_ && attachedId_ == id))
            showDetail();
        QJsonObject summary;
        for (const QJsonValue &listed : runs_)
            if (listed["id"].toString() == id)
                summary = listed.toObject();
        if (!detailBroken_)
            follow(summary);
    });
    showing_->start();
}

namespace {

// What the agent did, one row each.
QWidget *didRow(const QJsonObject &did)
{
    const QString kind = did["kind"].toString(), what = did["what"].toString();
    const bool administrator = did["as_administrator"].toBool();
    // Records from before `asked` existed cannot say whether anyone was asked.
    const bool known = did.contains("asked");
    const bool asked = did["asked"].toBool();
    const QJsonValue approved = did["approved"];
    const QJsonValue code = did["exit_code"];

    QString verb = kind == QLatin1String("write") ? QObject::tr("changed")
                   : kind == QLatin1String("search") ? QObject::tr("searched for")
                                                     : QObject::tr("ran");
    QString how;
    Mood mood = Mood::Plain;
    if (kind == QLatin1String("command") && known && asked && approved.isBool() && !approved.toBool()) {
        verb = QObject::tr("wanted to run");
        how = administrator ? QObject::tr("as administrator; you refused, so it was not run")
                            : QObject::tr("you refused, so it was not run");
        mood = Mood::Warn;
    } else if (known && asked) {
        how = administrator ? QObject::tr("as administrator, you approved") : QObject::tr("you approved, run as you");
    } else if (known) {
        // A search is the agent's own; a command or a file change ran or was made inside its sandbox.
        how = kind == QLatin1String("search") ? QObject::tr("the agent's own, nobody was asked")
                                              : QObject::tr("in the agent's sandbox");
    }
    if (kind == QLatin1String("command") && known && asked && approved.toBool()) {
        if (code.isNull()) {
            how += QObject::tr("; it did not finish");
            mood = Mood::Bad;
        } else if (code.toInt() != 0) {
            how += QObject::tr("; it exited with %1").arg(code.toInt());
            mood = Mood::Bad;
        }
    } else if (kind == QLatin1String("command") && !asked && !code.isNull() && code.toInt() != 0) {
        how += QObject::tr("; it exited with %1").arg(code.toInt());
        mood = Mood::Bad;
    }

    auto *row = new QWidget;
    auto *line = new QHBoxLayout(row);
    line->setContentsMargins(0, 3, 0, 3);
    line->setSpacing(8);
    line->addWidget(dim(text(verb)));
    auto *thing = new ElidedLabel(what);
    if (kind != QLatin1String("search"))
        thing->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
    line->addWidget(thing, 1);
    if (!how.isEmpty()) {
        QLabel *words = text(how);
        QPalette p = words->palette();
        p.setColor(QPalette::WindowText, moodColour(p, mood));
        words->setPalette(p);
        line->addWidget(words);
    }
    return row;
}

} // namespace

// A run that is going and can be reached is followed: attached to, with what
// it said so far replayed from its record. One that is not shown or not going
// is let go of (the run itself is never touched by that).
void ActivityPage::follow(const QJsonObject &summary)
{
    const QString state = summary["state"].toString();
    const bool going = state == QLatin1String("running") || state == QLatin1String("waiting");
    if (!going || !summary["attachable"].toBool() || !isVisible() || endedIds_.contains(chosen_)) {
        if (attachedId_ != chosen_)
            detach();
        return;
    }
    if (attached_ && attachedId_ == chosen_)
        return;
    detach();
    attachedId_ = chosen_;
    liveLines_ = QJsonArray{lines_.first()};
    tail_.clear();
    stepNow_.clear();
    stepTitles_.clear();
    asked_.clear();
    sentFor_.clear();
    answeredWords_.clear();
    stopProblem_.clear();
    live_ = true;
    const QString id = chosen_;
    attached_ = Run::attach(program_, id, this);
    connect(attached_, &Run::event, this, &ActivityPage::onLive);
    connect(attached_, &Run::output, this, [this](const QString &line) {
        if (line.isEmpty())
            return;
        tail_ << line;
        while (tail_.size() > 3)
            tail_.removeFirst();
        if (tailLabel_)
            tailLabel_->setText(tail_.join(QLatin1Char('\n')));
    });
    connect(attached_, &Run::finished, this, [this, id](bool) {
        if (attached_ && attachedId_ == id)
            liveEnded();
    });
    attached_->start();
    showDetail();
}

void ActivityPage::detach()
{
    live_ = false;
    attachedId_.clear();
    asked_.clear();
    if (!attached_)
        return;
    Run *run = attached_;
    attached_ = nullptr;
    // The run goes on; only this page stops listening.
    run->release();
}

void ActivityPage::liveEnded()
{
    const QString id = attachedId_;
    endedIds_ << id;
    Run *run = attached_;
    attached_ = nullptr;
    attachedId_.clear();
    live_ = false;
    asked_.clear();
    run->deleteLater();
    // Its record is complete now: read it and the list again.
    lines_ = {};
    read();
    if (id == chosen_)
        fetchDetail();
}

void ActivityPage::onLive(const QJsonObject &event)
{
    const QString kind = event["event"].toString();
    if (kind == QLatin1String("plan")) {
        stepTitles_.clear();
        for (const QJsonValue &pair : event["steps"].toArray())
            stepTitles_.insert(pair.toArray().at(0).toString(), pair.toArray().at(1).toString());
        return;
    }
    if (kind == QLatin1String("step")) {
        if (event["status"].toString() == QLatin1String("running")) {
            stepNow_ = stepTitles_.value(event["step"].toString());
            if (stepLabel_)
                stepLabel_->setText(stepNow_);
        }
        return;
    }
    liveLines_ << event;
    if (kind == QLatin1String("question")) {
        asked_ = event["id"].toVariant().toString();
        sentFor_.clear();
        answeredWords_.clear();
        read(); // the list says "waits" now
    } else if (kind == QLatin1String("answer")) {
        asked_.clear();
        sentFor_.clear();
        typed_.clear();
        answeredWords_ = tr("Answered: %1.").arg(event["answer"].toString());
        read();
    }
    redraw_->start();
}

void ActivityPage::rememberTyping()
{
    typingFocused_ = typing_ && typing_->hasFocus();
    if (typing_)
        typed_ = typing_->text();
}

QStringList ActivityPage::answerButtons() const
{
    QStringList words;
    if (detail_ && detail_->widget())
        if (QWidget *card = detail_->widget()->findChild<QWidget *>(QStringLiteral("question")))
            for (QPushButton *button : card->findChildren<QPushButton *>())
                words << button->text();
    return words;
}

bool ActivityPage::answer(const QString &words)
{
    if (!attached_ || asked_.isEmpty() || !detail_ || !detail_->widget() || sentFor_ == asked_)
        return false;
    QWidget *card = detail_->widget()->findChild<QWidget *>(QStringLiteral("question"));
    if (!card)
        return false;
    for (QPushButton *button : card->findChildren<QPushButton *>()) {
        if (button->text() == words && button->isEnabled()) {
            button->click();
            return true;
        }
    }
    if (QLineEdit *line = card->findChild<QLineEdit *>()) {
        line->setText(words);
        for (QPushButton *button : card->findChildren<QPushButton *>())
            if (button->isEnabled()) {
                button->click();
                return true;
            }
    }
    return false;
}

bool ActivityPage::offersStop() const
{
    if (!detail_ || !detail_->widget())
        return false;
    for (QPushButton *button : detail_->widget()->findChildren<QPushButton *>())
        if (button->text() == tr("Stop") && button->isEnabled())
            return true;
    return false;
}

bool ActivityPage::stopChosen()
{
    if (stopping_ || chosen_.isEmpty() || !offersStop())
        return false;
    stopProblem_.clear();
    stopping_ = new Run(program_, {"--stop", chosen_}, this);
    connect(stopping_, &Run::finished, this, [this](bool ok) {
        Run *run = stopping_;
        stopping_ = nullptr;
        if (!ok)
            stopProblem_ = tr("It could not be stopped: %1").arg(run->problem().isEmpty() ? tr("the helper did not say why") : run->problem());
        run->deleteLater();
        read(); // an attached page learns of the end by itself; one that is not, here
        if (!ok && detail_)
            showDetail();
    });
    stopping_->start();
    showDetail();
    return true;
}

void ActivityPage::showDetail()
{
    if (!detail_)
        return;
    rememberTyping();
    detailScroll_ = detail_->verticalScrollBar()->value();
    stepLabel_ = tailLabel_ = nullptr;
    typing_ = nullptr;
    auto *page = new QWidget;
    auto *column = new QVBoxLayout(page);
    column->setContentsMargins(0, 0, 4, 0);
    column->setSpacing(12);
    detail_->setWidget(page);
    // The scroll range exists only after layout.
    QMetaObject::invokeMethod(detail_, [this] { if (detail_) detail_->verticalScrollBar()->setValue(detailScroll_); },
                              Qt::QueuedConnection);

    if (chosen_.isEmpty()) {
        column->addWidget(dim(text(tr("Choose a run to see what was asked and what it did."), true)));
        column->addStretch();
        return;
    }
    const bool following = attached_ && live_ && attachedId_ == chosen_;
    const QJsonArray &lines = following ? liveLines_ : lines_;
    if (!following && (detailBroken_ || lines_.isEmpty())) {
        column->addWidget(strong(tr("This run could not be read")));
        column->addWidget(dim(text(tr("%1 --show did not give its record.").arg(QFileInfo(program_).fileName()), true)));
        column->addStretch();
        return;
    }

    QJsonObject summary;
    for (const QJsonValue &run : runs_)
        if (run["id"].toString() == chosen_)
            summary = run.toObject();
    const QJsonObject request = lines.first().toObject();

    // Walk the record once.
    QList<QJsonObject> dids;
    QString said, failure, endState, endTime;
    QJsonObject question;
    for (int at = 1; at < lines.size(); ++at) {
        const QJsonObject line = lines[at].toObject();
        const QString event = line["event"].toString();
        if (line["record"].toString() == QLatin1String("end")) {
            endState = line["state"].toString();
            endTime = line["t"].toString();
        }
        if (event == QLatin1String("did"))
            dids << line;
        else if (event == QLatin1String("say") && !line["text"].toString().isEmpty())
            said = line["text"].toString();
        else if (event == QLatin1String("agent") && !line["summary"].toString().isEmpty())
            said = line["summary"].toString();
        else if (event == QLatin1String("failed"))
            failure = line["reason"].toString();
        if (event == QLatin1String("question"))
            question = line;
        else if (event == QLatin1String("answer"))
            question = QJsonObject();
    }

    // Where it stands. The record's own end beats a list that is a moment old;
    // a run that is followed knows from what it said whether it waits.
    QString state = summary["state"].toString();
    if (!endState.isEmpty())
        state = endState;
    else if (following)
        state = question.isEmpty() ? QStringLiteral("running") : QStringLiteral("waiting");
    const bool waiting = state == QLatin1String("waiting"), running = state == QLatin1String("running");
    const bool mine = (waiting || running) && !own_.isEmpty() && request["id"].toString() == own_;
    const bool canStop = (waiting || running) && (following || summary["attachable"].toBool());

    // Who, when, and where it stands.
    const QString agent = agentWords(request["agent"].toString());
    const QString ended = whenText(endTime.isEmpty() ? summary["ended"].toString() : endTime);
    QString standing;
    if (waiting)
        standing = tr("%1 waits for an answer").arg(agent);
    else if (running)
        standing = tr("%1 is working on it").arg(agent);
    else if (state == QLatin1String("done"))
        standing = ended.isEmpty() ? tr("Done") : tr("Done, %1").arg(ended);
    else if (state == QLatin1String("failed"))
        standing = ended.isEmpty() ? tr("Failed") : tr("Failed, %1").arg(ended);
    else
        standing = ended.isEmpty() ? tr("Stopped") : tr("Stopped, %1").arg(ended);
    const QString asked = whenText(request["t"].toString());
    auto *head = new QHBoxLayout;
    auto *names = new QVBoxLayout;
    names->setSpacing(1);
    QLabel *title = heading(titleOf(request));
    title->setWordWrap(true);
    QFont smaller = title->font();
    smaller.setPointSizeF(smaller.pointSizeF() / 1.55 * 1.3);
    title->setFont(smaller);
    names->addWidget(title);
    names->addWidget(dim(text(asked.isEmpty() ? tr("Asked in %1 · %2").arg(appText(request["app"].toString()), standing)
                                              : tr("Asked in %1 %2 · %3").arg(appText(request["app"].toString()), asked, standing),
                              true)));
    head->addLayout(names, 1);
    auto *buttons = new QVBoxLayout;
    buttons->setSpacing(6);
    if (mine) {
        auto *go = new QPushButton(tr("Go to it"));
        go->setDefault(true);
        connect(go, &QPushButton::clicked, this, &ActivityPage::goToRun);
        buttons->addWidget(go);
    }
    if (canStop) {
        auto *stop = new QPushButton(stopping_ ? tr("Stopping…") : tr("Stop"));
        stop->setEnabled(!stopping_);
        stop->setAccessibleDescription(tr("Ends this run. What it already did stays done."));
        connect(stop, &QPushButton::clicked, this, [this] { stopChosen(); });
        buttons->addWidget(stop);
    }
    head->addLayout(buttons);
    column->addLayout(head);
    if (!stopProblem_.isEmpty()) {
        QLabel *problem = text(stopProblem_, true);
        QPalette p = problem->palette();
        p.setColor(QPalette::WindowText, moodColour(p, Mood::Bad));
        problem->setPalette(p);
        column->addWidget(problem);
    }

    // What was asked.
    {
        QVBoxLayout *inside;
        column->addWidget(group(tr("What you asked"), &inside));
        const QString prompt = request["prompt"].toString();
        const QList<Part> parts = partsOf(prompt);
        auto *grid = new QGridLayout;
        grid->setHorizontalSpacing(14);
        grid->setVerticalSpacing(5);
        grid->setColumnMinimumWidth(0, 96);
        grid->setColumnStretch(1, 1);
        int shown = 0;
        bool cut = false;
        auto line = [&](const QString &name, const QString &body) {
            QString words = body;
            if (words.size() > 320) {
                words = words.left(320).trimmed() + QStringLiteral("…");
                cut = true;
            }
            grid->addWidget(dim(text(name)), shown, 0, Qt::AlignTop);
            grid->addWidget(text(words, true), shown, 1);
            ++shown;
        };
        if (parts.isEmpty()) {
            line(tr("The request"), prompt);
        } else {
            for (const Part &part : parts) {
                if (part.heading == QLatin1String("The request") || part.heading == QLatin1String("The issue"))
                    line(part.heading, part.body);
                else if (part.heading == QLatin1String("The expected result"))
                    line(tr("Expected"), part.body);
            }
        }
        // Whatever the lines above leave out is behind the link.
        const bool folded = cut || (!parts.isEmpty() && shown < parts.size());
        inside->addLayout(grid);
        if (folded) {
            auto *link = new ClickLabel(tr("The whole prompt"));
            QPalette colours = link->palette();
            colours.setColor(QPalette::WindowText, colours.color(QPalette::Link));
            link->setPalette(colours);
            QFont underlined = link->font();
            underlined.setUnderline(true);
            link->setFont(underlined);
            auto *all = new QPlainTextEdit(prompt);
            all->setReadOnly(true);
            all->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
            all->setFixedHeight(160);
            all->hide();
            connect(link, &ClickLabel::clicked, this, [all] { all->setVisible(!all->isVisible()); });
            inside->addWidget(link);
            inside->addWidget(all);
        }
    }

    // What it does now: only for a run that is followed and is not waiting.
    if (following && running) {
        QVBoxLayout *inside;
        column->addWidget(group(tr("What it does now"), &inside));
        if (!answeredWords_.isEmpty())
            inside->addWidget(dim(text(answeredWords_, true)));
        stepLabel_ = text(stepNow_.isEmpty() ? tr("Working on it.") : stepNow_, true);
        inside->addWidget(stepLabel_);
        tailLabel_ = mono(tail_.join(QLatin1Char('\n')));
        inside->addWidget(tailLabel_);
    }

    // What it did.
    {
        QVBoxLayout *inside;
        column->addWidget(group(tr("What it did"), &inside));
        if (dids.isEmpty()) {
            inside->addWidget(dim(text(running || waiting ? tr("Nothing yet.")
                                       : state == QLatin1String("done") ? tr("It did not run a command or change a file.")
                                                                         : tr("Nothing was recorded before it ended."),
                                       true)));
        }
        const int most = 60;
        for (int at = 0; at < dids.size() && at < most; ++at) {
            if (at > 0)
                inside->addWidget(rule());
            inside->addWidget(didRow(dids[at]));
        }
        if (dids.size() > most)
            inside->addWidget(dim(text(tr("and %1 more").arg(dids.size() - most))));
    }

    // What it waits for, and the way to answer it.
    if (waiting && !question.isEmpty()) {
        auto *card = new Card(Card::Open);
        card->setObjectName(QStringLiteral("question"));
        column->addWidget(card);
        const QString kind = question["kind"].toString();
        const QString words = question["text"].toString();
        const bool command = kind == QLatin1String("administrator") || kind == QLatin1String("command");
        card->column()->setSpacing(6);
        if (command) {
            card->column()->addWidget(strong(kind == QLatin1String("administrator")
                                                 ? tr("It wants to run this as administrator")
                                                 : tr("It wants to run this as you, outside its sandbox")));
            card->column()->addWidget(mono(question["command"].toString()));
            const int why = words.indexOf(QLatin1String("\n\nWhy: "));
            if (why >= 0)
                card->column()->addWidget(text(words.mid(why + 7), true));
        } else {
            card->column()->addWidget(strong(tr("It asks")));
            card->column()->addWidget(text(words, true));
        }
        QStringList choices;
        for (const QJsonValue &choice : question["choices"].toArray())
            choices << choice.toString();
        const QString id = question["id"].toVariant().toString();
        if (!following) {
            card->column()->addWidget(dim(text(tr("It waits for an answer. This page cannot reach it at the moment."), true)));
        } else if (sentFor_ == id) {
            card->column()->addWidget(dim(text(tr("Your answer was sent. Waiting for the agent to take it."), true)));
        } else {
            auto *row = new QHBoxLayout;
            if (choices.isEmpty()) {
                auto *line = new QLineEdit;
                line->setAccessibleName(tr("Your answer"));
                typing_ = line;
                if (typedFor_ == id)
                    line->setText(typed_);
                typedFor_ = id;
                auto *send = new QPushButton(tr("Answer"));
                send->setDefault(true);
                auto give = [this, line, id] {
                    const QString answer = line->text().trimmed();
                    if (answer.isEmpty() || !attached_)
                        return;
                    attached_->reply(id, answer);
                    sentFor_ = id;
                    typed_.clear();
                    showDetail();
                };
                connect(send, &QPushButton::clicked, this, give);
                connect(line, &QLineEdit::returnPressed, send, &QPushButton::click);
                row->addWidget(line, 1);
                row->addWidget(send);
                if (typingFocused_)
                    line->setFocus();
            } else {
                for (const QString &choice : choices) {
                    auto *button = new QPushButton(choice);
                    connect(button, &QPushButton::clicked, this, [this, id, choice] {
                        if (!attached_ || sentFor_ == id)
                            return;
                        attached_->reply(id, choice);
                        sentFor_ = id;
                        showDetail();
                    });
                    row->addWidget(button);
                }
                row->addStretch();
            }
            card->column()->addLayout(row);
            if (command)
                card->column()->addWidget(dim(text(tr("Approve runs it now. Refuse tells the agent you did not allow it."), true)));
        }
    }

    // What it said, and how it ended.
    if (!said.isEmpty() && !waiting) {
        QVBoxLayout *inside;
        column->addWidget(group(tr("What it said last"), &inside));
        inside->addWidget(text(said, true));
    }
    if (state == QLatin1String("failed")) {
        auto *verdict = new Verdict;
        verdict->setFrameShape(QFrame::NoFrame);
        verdict->column()->addWidget(strong(tr("It failed")));
        const QString reason = failure.isEmpty() ? summary["summary"].toString() : failure;
        verdict->column()->addWidget(text(reason.isEmpty() ? tr("It did not say why.") : reason, true));
        column->addWidget(verdict);
    } else if (state == QLatin1String("stopped")) {
        QVBoxLayout *inside;
        column->addWidget(group(tr("How it ended"), &inside));
        inside->addWidget(text(tr("It was stopped before it finished: someone pressed Stop, or the computer restarted. "
                                  "What it had already done stays done."),
                               true));
    }
    column->addStretch();
}

QStringList ActivityPage::listedTitles() const
{
    QStringList titles;
    if (list_)
        for (int row = 0; row < list_->count(); ++row)
            titles << list_->item(row)->data(TitleRole).toString();
    return titles;
}

QStringList ActivityPage::listedStates() const
{
    QStringList states;
    if (list_)
        for (int row = 0; row < list_->count(); ++row)
            states << list_->item(row)->data(StateRole).toString();
    return states;
}

QString ActivityPage::detailText() const
{
    QStringList all;
    if (detail_ && detail_->widget())
        for (QLabel *label : detail_->widget()->findChildren<QLabel *>())
            all << label->text();
    return all.join(QLatin1Char('\n'));
}

bool ActivityPage::offersGoTo() const
{
    if (!detail_ || !detail_->widget())
        return false;
    for (QPushButton *button : detail_->widget()->findChildren<QPushButton *>())
        if (button->text() == tr("Go to it"))
            return true;
    return false;
}
