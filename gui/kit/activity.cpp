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
#include <QVBoxLayout>

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
}

void ActivityPage::setOwnRun(qint64 pid)
{
    own_ = pid;
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
    list_ = nullptr;
    detail_ = nullptr;
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
    top->addWidget(dim(text(tr("From all three apps. Kept when a window closes."), true)));
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
        fetchDetail();
    });
    if (shown.isEmpty()) {
        list_->hide();
        side->addWidget(dim(text(onlyThis_ ? tr("This app has asked the agent for nothing yet.")
                                           : tr("Nothing has been asked of the agent yet. What you ask it to do in "
                                                "Recipe, System or Disk is listed here."),
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
        showDetail();
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

void ActivityPage::showDetail()
{
    if (!detail_)
        return;
    auto *page = new QWidget;
    auto *column = new QVBoxLayout(page);
    column->setContentsMargins(0, 0, 4, 0);
    column->setSpacing(12);
    detail_->setWidget(page);

    if (chosen_.isEmpty()) {
        column->addWidget(dim(text(tr("Choose a run to see what was asked and what it did."), true)));
        column->addStretch();
        return;
    }
    if (detailBroken_ || lines_.isEmpty()) {
        column->addWidget(strong(tr("This run could not be read")));
        column->addWidget(dim(text(tr("%1 --show did not give its record.").arg(QFileInfo(program_).fileName()), true)));
        column->addStretch();
        return;
    }

    QJsonObject summary;
    for (const QJsonValue &run : runs_)
        if (run["id"].toString() == chosen_)
            summary = run.toObject();
    const QJsonObject request = lines_.first().toObject();
    const QString state = summary["state"].toString();
    const bool waiting = state == QLatin1String("waiting"), running = state == QLatin1String("running");
    const bool mine = (waiting || running) && own_ > 0 && request["pid"].toInteger() == own_;

    // Walk the record once.
    QList<QJsonObject> dids;
    QString said, failure;
    QJsonObject question;
    for (int at = 1; at < lines_.size(); ++at) {
        const QJsonObject line = lines_[at].toObject();
        const QString event = line["event"].toString();
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

    // Who, when, and where it stands.
    const QString agent = agentWords(request["agent"].toString());
    const QString ended = whenText(summary["ended"].toString());
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
    if (mine) {
        auto *go = new QPushButton(tr("Go to it"));
        go->setDefault(true);
        connect(go, &QPushButton::clicked, this, &ActivityPage::goToRun);
        head->addWidget(go, 0, Qt::AlignTop);
    }
    column->addLayout(head);

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

    // What it waits for.
    if (waiting && !question.isEmpty()) {
        auto *card = new Card(Card::Open);
        column->addWidget(card);
        const QString kind = question["kind"].toString();
        const QString words = question["text"].toString();
        card->column()->setSpacing(6);
        if (kind == QLatin1String("administrator") || kind == QLatin1String("command")) {
            card->column()->addWidget(strong(kind == QLatin1String("administrator")
                                                 ? tr("It wants to run this as administrator")
                                                 : tr("It wants to run this as you")));
            card->column()->addWidget(mono(question["command"].toString()));
            const int why = words.indexOf(QLatin1String("\n\nWhy: "));
            if (why >= 0)
                card->column()->addWidget(text(words.mid(why + 7), true));
        } else {
            card->column()->addWidget(strong(tr("It asks")));
            card->column()->addWidget(text(words, true));
            QStringList choices;
            for (const QJsonValue &choice : question["choices"].toArray())
                choices << choice.toString();
            if (!choices.isEmpty())
                card->column()->addWidget(dim(text(tr("It offers: %1").arg(choices.join(QStringLiteral(", "))), true)));
        }
        if (mine) {
            card->column()->addWidget(text(tr("It waits in this window. Answer it there."), true));
        } else {
            const QString app = request["app"].toString();
            card->column()->addWidget(dim(text(
                app.isEmpty() ? tr("Only the window that started it can answer, because a run ends when its window "
                                   "closes. It cannot be answered from here.")
                              : tr("Only the window that started it, in %1, can answer, because a run ends when its "
                                   "window closes. It cannot be answered from here.")
                                    .arg(app),
                true)));
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
        inside->addWidget(text(tr("It was stopped before it finished, for example because the window that started it "
                                  "closed. What it had already done stays done."),
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
