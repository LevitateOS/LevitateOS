#include "pages.h"

#include "parts.h"
#include "run.h"

#include <QDialogButtonBox>
#include <functional>
#include <QFontDatabase>
#include <QDateTime>
#include <QFormLayout>
#include <QGridLayout>
#include <QGroupBox>
#include <QLocale>
#include <QScrollArea>
#include <QJsonArray>
#include <QPlainTextEdit>
#include <QProgressBar>
#include <QPushButton>
#include <QVBoxLayout>

namespace {

QPushButton *primary(const QString &words)
{
    auto *button = new QPushButton(words);
    button->setDefault(true);
    return button;
}

// One thing a person can do: its name, one line on what it does, and its
// button. When recab would refuse it, the line is recab's reason and the
// button is off.
QPushButton *act(QVBoxLayout *group, const QString &name, const QString &line, const QString &button,
                 const ActionVerdict &verdict, bool main = false)
{
    if (group->count() > 1)
        group->addWidget(rule());
    auto *row = new QHBoxLayout;
    row->setSpacing(20);
    auto *words = new QVBoxLayout;
    words->setSpacing(1);
    QLabel *title = strong(name);
    if (!verdict.ready)
        dim(title);
    words->addWidget(title);
    words->addWidget(dim(text(verdict.ready ? line : verdict.reason, true)));
    row->addLayout(words, 1);
    QPushButton *push = main && verdict.ready ? primary(button) : new QPushButton(button);
    push->setMinimumWidth(170);
    push->setEnabled(verdict.ready);
    row->addWidget(push);
    group->addLayout(row);
    return push;
}

// A way to another page: underlined, in the link colour, and reachable by Tab.
ClickLabel *link(const QString &words)
{
    auto *label = new ClickLabel(words);
    QFont font = label->font();
    font.setUnderline(true);
    label->setFont(font);
    QPalette palette = label->palette();
    palette.setColor(QPalette::WindowText, palette.color(QPalette::Link));
    label->setPalette(palette);
    return label;
}

// One of the two systems, as a card: a picture, what it is, which release,
// and a few plain lines about it.
Card *card(Card::Tone tone, Glyph::Kind picture, const QString &title, const QString &under, const QStringList &lines,
           const QString &quiet = QString())
{
    auto *box = new Card(tone);
    auto *row = new QHBoxLayout;
    row->setSpacing(16);
    row->addWidget(new Glyph(picture, tone == Card::On), 0, Qt::AlignTop);
    auto *words = new QVBoxLayout;
    words->setSpacing(2);
    words->addWidget(strong(title));
    words->addWidget(dim(text(under, true)));
    words->addSpacing(4);
    for (const QString &line : lines)
        words->addWidget(text(QStringLiteral("• ") + line, true));
    if (!quiet.isEmpty())
        words->addWidget(dim(text(QStringLiteral("• ") + quiet, true)));
    words->addStretch();
    row->addLayout(words, 1);
    box->column()->addLayout(row);
    return box;
}

} // namespace

// ---------------------------------------------------------------- The system

void SystemPage::showProblem(const QString &problem, bool asAdministrator)
{
    QVBoxLayout *column = fresh();
    column->addWidget(heading(tr("The system could not be read")));
    column->addWidget(text(problem, true));
    auto *buttons = new QHBoxLayout;
    auto *again = new QPushButton(tr("Try again"));
    connect(again, &QPushButton::clicked, this, &SystemPage::readAgain);
    buttons->addWidget(again);
    if (asAdministrator) {
        // recab reads the boot entries, which this person may not.
        auto *root = primary(tr("Read it as administrator"));
        connect(root, &QPushButton::clicked, this, &SystemPage::readAsAdministrator);
        buttons->addWidget(root);
    } else {
        again->setDefault(true);
    }
    buttons->addStretch();
    column->addLayout(buttons);
    column->addStretch();
}

static void lifecyclePage(QVBoxLayout *column,const Status &s,QObject *owner,std::function<void(const QJsonObject&)> request) {
    auto *group=new QGroupBox(QObject::tr("Software, updates and recovery"));
    group->setCheckable(true);
    auto *outer=new QVBoxLayout(group);auto *body=new QWidget;auto *actions=new QVBoxLayout(body);outer->addWidget(body);
    if(!s.unusable.isEmpty())actions->addWidget(text(s.unusable,true));
    if(!s.updatePhase.isEmpty())actions->addWidget(text(QObject::tr("Update: %1 %2").arg(s.updatePhase,s.downloadProgress),true));
    if(!s.updateError.isEmpty())actions->addWidget(text(s.updateError,true));
    for(const auto &v:s.operations){const auto op=v.toObject();auto *button=new QPushButton(QFontMetrics(group->font()).elidedText(op["label"].toString(),Qt::ElideMiddle,600));button->setEnabled(op["verdict"]=="ready");button->setToolTip(op["label"].toString()+"\n"+op["reason"].toString());actions->addWidget(button);QObject::connect(button,&QPushButton::clicked,owner,[request,op]{request(op);});}
    auto *start=new QPushButton(QObject::tr("Reboot"));actions->addWidget(start);QObject::connect(start,&QPushButton::clicked,owner,[request]{request(QJsonObject{{"id","reboot"},{"verdict","ready"}});});
    const bool expanded=!s.unusable.isEmpty() || !s.clashes.isEmpty() || s.state=="interrupted";
    group->setChecked(expanded);body->setVisible(expanded);QObject::connect(group,&QGroupBox::toggled,body,&QWidget::setVisible);
    column->addWidget(group);
}

void SystemPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
    if(!s.operations.isEmpty())lifecyclePage(column,s,this,[this](const QJsonObject &op){emit operation(op);});
    const bool next = s.otherRole == QLatin1String("next");
    const QString running = s.name(s.runningRelease);
    const QStringList titles = s.changeTitles();

    // What is true now, in one line and one sentence.
    if (s.state == QLatin1String("failed") || s.state == QLatin1String("interrupted")) {
        auto *verdict = new Verdict;
        verdict->setFrameShape(QFrame::NoFrame);
        if (s.state == QLatin1String("failed")) {
            verdict->column()->addWidget(heading(tr("The next system did not start, so the computer came back")));
            verdict->column()->addWidget(text(
                tr("You are on %1, exactly as it was. The next system still holds its changes; nothing in it was "
                   "thrown away.")
                    .arg(running),
                true));
        } else {
            verdict->column()->addWidget(heading(tr("Preparing the next system was interrupted")));
            verdict->column()->addWidget(text(
                tr("It stopped while %1. The next system cannot be started in this state. The running system was "
                   "not touched.")
                    .arg(s.interruptedDuring),
                true));
        }
        column->addWidget(verdict);
    } else {
        QString title, under;
        if (s.state == QLatin1String("trial_pending")) {
            title = tr("The next system starts at the next restart");
            under = tr("That is set: any restart starts it, whoever makes it. Until then you stay on the running "
                       "system, and you can cancel.");
        } else if (s.state == QLatin1String("ready")) {
            title = tr("The next system is ready");
            under = tr("It is checked and closed. Restart to start using it; until then you stay on the running system.");
        } else if (next && !s.clashes.isEmpty()) {
            title = tr("The update clashes with one of your changes");
            under = tr("Nothing was chosen for you. The next system cannot be finished until that is settled.");
        } else if (s.state == QLatin1String("on_trial")) {
            title = tr("This system is on trial");
            under = tr("It started, and is not kept yet: a restart now goes back to the system from before. recab "
                       "keeps it once start-up has finished properly.");
        } else if (s.state == QLatin1String("working")) {
            title = tr("The next system is being worked on");
            under = tr("A recab command is %1 right now, started outside this window. This page reads again until "
                       "it has finished.")
                        .arg(s.interruptedDuring.isEmpty() ? tr("working on the next system") : s.interruptedDuring);
        } else if (next) {
            title = counted(s.changes.size(), tr("The next system has %1 change"), tr("The next system has %1 changes"));
            under = (s.changes.size() == 1 ? tr("It takes effect when you restart into it.")
                                           : tr("They take effect when you restart into it."))
                    + tr(" The running system is untouched until then.");
        } else if (s.trialOutcome == QLatin1String("kept")) {
            title = tr("You are on the new system");
            under = tr("It started properly, so it is kept, and locked like the one before. Your changes are in effect.");
        } else {
            title = tr("Your system is locked");
            under = tr("It cannot be changed while it runs. A change is made in the next system, beside it, and "
                       "takes effect when you restart.");
        }
        auto *top = new QVBoxLayout;
        top->setSpacing(1);
        top->addWidget(heading(title));
        top->addWidget(dim(text(under, true)));
        column->addLayout(top);
    }

    // The failed start is about one thing: handing it to the agent.
    if (s.state == QLatin1String("failed")) {
        QStringList facts;
        facts << (s.trialLogLines ? tr("What failed: %1; its log, %2 lines").arg(s.trialDetail).arg(s.trialLogLines)
                                  : tr("What failed: %1").arg(s.trialDetail));
        QStringList differs = titles;
        if (s.otherRelease != s.runningRelease)
            differs.prepend(tr("the update to %1").arg(s.otherRelease));
        facts << tr("The next system differs from the running one by: %1").arg(differs.join(tr("; ")));
        facts << tr("The running system starts on the same computer");
        auto *prompt = new PromptPanel(
            tr("The next system stopped during start-up on %1 and the computer fell back to %2.").arg(s.trialWhen, running),
            facts,
            tr("The next system starts to the login screen with every change in it working. If one of them cannot "
               "work, say which and leave it out."));
        auto *buttons = new QHBoxLayout;
        auto *again = new QPushButton(tr("Try the same again"));
        auto *fixNow = primary(tr("Let the agent fix it"));
        connect(again, &QPushButton::clicked, this, &SystemPage::retry);
        connect(fixNow, &QPushButton::clicked, this, [this, prompt] { emit fix(prompt->prompt()); });
        buttons->addWidget(again);
        buttons->addStretch();
        buttons->addWidget(fixNow);
        prompt->column()->addLayout(buttons);
        column->addWidget(prompt);
        auto *more = new QHBoxLayout;
        more->setSpacing(18);
        more->addWidget(dim(new QLabel(tr("The rest is on:"))));
        auto *nextLink = link(tr("The next system"));
        connect(nextLink, &ClickLabel::clicked, this, &SystemPage::openNext);
        more->addWidget(nextLink);
        more->addStretch();
        column->addLayout(more);
        column->addStretch();
        return;
    }

    // The two systems, side by side.
    auto *pair = new QHBoxLayout;
    pair->setSpacing(14);
    QStringList runningLines{tr("Running now"), tr("Locked")};
    if (next)
        runningLines << (s.state == QLatin1String("interrupted") ? tr("Untouched") : tr("Your way back"));
    else if (s.state == QLatin1String("on_trial"))
        runningLines = QStringList{tr("Running now"), tr("On trial: not kept yet")};
    else if (!s.runningHolds.isEmpty())
        runningLines << tr("With: %1").arg(s.runningHolds.join(tr("; ")));
    else
        runningLines << tr("Kept: it started properly");
    pair->addWidget(card(Card::On, Glyph::Lock, tr("The running system"), running, runningLines), 1);

    if (next) {
        const bool plus = !titles.isEmpty();
        const QString under = !plus ? s.name(s.otherRelease)
                                     : tr("%1, plus:").arg(s.name(s.otherRelease));
        if (s.state == QLatin1String("interrupted"))
            pair->addWidget(card(Card::Open, Glyph::Plus, tr("The next system"), under, titles,
                                 tr("Incomplete: cannot be started")), 1);
        else if (s.otherState == QLatin1String("ready"))
            pair->addWidget(card(Card::Ready, Glyph::Check, tr("The next system"), under,
                                 titles + QStringList{tr("Ready: starts when you restart into it")}), 1);
        else
            pair->addWidget(card(Card::Open, Glyph::Plus, tr("The next system"), under,
                                 plus ? titles : QStringList{tr("No change yet: exactly the running system")},
                                 tr("Open: not finished, cannot be started yet")), 1);
    } else if (s.otherRole == QLatin1String("previous")) {
        // The real recab does not name the release the other system holds.
        pair->addWidget(card(Card::Plain, Glyph::Before, tr("The system from before"),
                             s.otherRelease.isEmpty() ? tr("As it was before your last change") : s.name(s.otherRelease),
                             {tr("You can go back to it")},
                             tr("Its place becomes the next system when you make a change")), 1);
    } else if (s.otherRole == QLatin1String("copy")) {
        pair->addWidget(card(Card::Plain, Glyph::Check, tr("The second copy"), tr("Exactly the running system"),
                             {tr("Nothing has been changed since it was made")},
                             tr("It becomes the next system when you make a change")), 1);
    } else {
        pair->addWidget(card(Card::Plain, Glyph::Plus, tr("No other system yet"),
                             tr("The place beside the running system is free"), {},
                             tr("It becomes the next system when you make a change")), 1);
    }
    column->addLayout(pair);

    // The one thing to do next, with recab's verdict on it. The rest is on
    // the two pages that hold it, so nothing is offered twice.
    auto *group = new Panel;
    QVBoxLayout *list = group->column();
    list->addWidget(strong(tr("What to do next")));
    const QString updateName = s.updateRelease.isEmpty() ? QString() : s.name(s.updateRelease);
    bool toNext = true;

    if (s.state == QLatin1String("working")) {
        list->addWidget(dim(text(tr("Nothing can be started while that command runs. Two commands never work on the "
                                    "next system at once."),
                                 true)));
    } else if (s.state == QLatin1String("on_trial")) {
        toNext = false;
        connect(act(list, tr("Keep this system"),
                    tr("Makes it the system that every restart starts, now, without waiting for start-up to be "
                       "judged."),
                    tr("Keep it"), s.can("keep"), true),
                &QPushButton::clicked, this, &SystemPage::keep);
    } else if (s.state == QLatin1String("interrupted")) {
        connect(act(list, tr("Continue where it stopped"),
                    tr("recab looks at what is really there and carries on. Nothing that was finished is done again."),
                    tr("Continue"), s.can("recover"), true),
                &QPushButton::clicked, this, &SystemPage::recover);
    } else if (s.state == QLatin1String("trial_pending")) {
        connect(act(list, tr("Restart now"),
                    tr("If the next system does not start properly, the computer comes back to the running system "
                       "by itself. Its changes are not lost."),
                    tr("Restart"), s.can("restart"), true),
                &QPushButton::clicked, this, &SystemPage::restart);
    } else if (s.state == QLatin1String("ready")) {
        connect(act(list, tr("Restart into the next system"),
                    tr("If it does not start properly, the computer comes back to the running system by itself. Its "
                       "changes are not lost."),
                    tr("Restart into it"), s.can("restart"), true),
                &QPushButton::clicked, this, &SystemPage::restart);
    } else if (next && !s.clashes.isEmpty()) {
        connect(act(list, tr("Settle the clash"),
                    tr("%1 was changed by you and by the update. An agent can settle it, or you decide.")
                        .arg(s.clashes.first().path),
                    tr("Settle…"), ActionVerdict{true, QString()}, true),
                &QPushButton::clicked, this, &SystemPage::settle);
    } else if (next) {
        connect(act(list, tr("Finish and restart"),
                    tr("The next system is checked and closed. Then you restart into it."), tr("Finish…"),
                    s.can("finalize"), true),
                &QPushButton::clicked, this, &SystemPage::finish);
        if (!updateName.isEmpty())
            connect(act(list, tr("Add the update to %1").arg(updateName),
                        tr("It is merged into this same next system. Your changes are kept, or you are asked where "
                           "they clash."),
                        tr("Add the update"), s.can("update")),
                    &QPushButton::clicked, this, &SystemPage::addUpdate);
    } else {
        toNext = false;
        connect(act(list, tr("Change the system"),
                    tr("Say what you want. An agent makes the change in the next system, beside the running one."),
                    tr("Make a change…"), s.can("change"), true),
                &QPushButton::clicked, this, &SystemPage::ask);
        if (!updateName.isEmpty())
            connect(act(list, tr("Update to %1").arg(updateName),
                        tr("A newer %1 is available, %2. It goes into the next system like any other change.")
                            .arg(s.distro, bytesText(s.updateBytes)),
                        tr("Add the update"), s.can("update")),
                    &QPushButton::clicked, this, &SystemPage::addUpdate);
    }
    // The way to the pages that hold the rest.
    auto *more = new QHBoxLayout;
    more->setSpacing(18);
    more->addWidget(dim(new QLabel(tr("The rest is on:"))));
    if (toNext) {
        auto *nextLink = link(tr("The next system"));
        connect(nextLink, &ClickLabel::clicked, this, &SystemPage::openNext);
        more->addWidget(nextLink);
    }
    auto *backLink = link(tr("Getting back"));
    connect(backLink, &ClickLabel::clicked, this, &SystemPage::openBack);
    more->addWidget(backLink);
    more->addStretch();
    list->addLayout(more);
    column->addWidget(group);
    column->addStretch();
}

// ---------------------------------------------------------------- Ask

void AskPage::show(const Status &s, const QString &request)
{
    QVBoxLayout *column = fresh();
    auto *backLink = new ClickLabel(tr("‹ Back"));
    dim(backLink);
    connect(backLink, &ClickLabel::clicked, this, &AskPage::back);
    column->addWidget(backLink, 0, Qt::AlignLeft);

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("What do you want to change?")));
    top->addWidget(dim(text(tr("Say it in your own words. An agent works out how and makes the change in the next "
                               "system. The running system is not touched."),
                            true)));
    column->addLayout(top);

    auto *wish = new QPlainTextEdit(request);
    wish->setAccessibleName(tr("The change you want"));
    wish->setPlaceholderText(tr("For example: my graphics card is an NVIDIA RTX 4070 and I want NVIDIA's own driver."));
    wish->setFixedHeight(74);
    column->addWidget(wish);

    QStringList facts{tr("The running system: %1").arg(s.name(s.runningRelease))};
    if (s.otherRole == QLatin1String("next"))
        facts << (s.changes.isEmpty() ? tr("The next system exists and holds no changes yet")
                                      : tr("The next system already holds: %1").arg(s.changeTitles().join(tr("; "))));
    else
        facts << tr("The next system does not exist yet; it starts as a copy of the running one");
    if (!s.updateRelease.isEmpty())
        facts << tr("A newer release is available and not added: %1").arg(s.name(s.updateRelease));
    if (s.real) {
        // What this recab offers for it, so the agent need not guess.
        facts << tr("The next system is made with `recab prepare`, changed with `recab change install|update|remove "
                    "RECIPE` and `recab edit PATH`, and closed with `recab finalize`; `recab status --json` says "
                    "where things stand");
        facts << tr("Every recab command that changes something takes `--actor agent`, so the list of changes "
                    "says who made each one");
        if (!s.preparationId.isEmpty())
            facts << tr("The open preparation is %1, revision %2").arg(s.preparationId).arg(s.preparationRevision);
        facts << tr("This system runs from slot %1; the other is slot %2").arg(s.runningSlot, s.otherSlot);
    }
    auto *prompt = new PromptPanel(
        tr("The person asks for the change written above."), facts,
        tr("After a restart the change is in effect and the system starts as before. Nothing else changes."));
    column->addWidget(prompt);
    column->addStretch();

    auto *buttons = new QHBoxLayout;
    auto *cancel = new QPushButton(tr("Cancel"));
    auto *hand = primary(tr("Hand it to the agent"));
    connect(cancel, &QPushButton::clicked, this, &AskPage::back);
    connect(hand, &QPushButton::clicked, this, [this, wish, prompt] {
        const QString words = wish->toPlainText().trimmed();
        if (!words.isEmpty())
            emit handOver(tr("The request:\n%1\n\n%2").arg(words, prompt->prompt()));
    });
    buttons->addStretch();
    buttons->addWidget(cancel);
    buttons->addWidget(hand);
    column->addLayout(buttons);
}

// ---------------------------------------------------------------- A clash

void ClashPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
    if (s.clashes.isEmpty()) {
        column->addWidget(heading(tr("Nothing clashes")));
        column->addStretch();
        return;
    }
    const Clash clash = s.clashes.first();

    auto *verdict = new Verdict;
    verdict->setFrameShape(QFrame::NoFrame);
    verdict->column()->addWidget(heading(tr("The update clashes with one of your changes")));
    verdict->column()->addWidget(text(
        s.clashes.size() == 1
            ? tr("%1 was added to the next system. One file was changed by you and by the update. Nothing was chosen "
                 "for you.")
                  .arg(s.name(s.otherRelease))
            : tr("%1 was added to the next system. %2 files were changed by you and by the update. Nothing was "
                 "chosen for you. This is the first; the others follow.")
                  .arg(s.name(s.otherRelease))
                  .arg(s.clashes.size()),
        true));
    column->addWidget(verdict);

    auto *prompt = new PromptPanel(
        tr("The update and a local change both altered %1. The next system cannot be finished until there is one "
           "result.")
            .arg(clash.path),
        {tr("In the next system now: %1").arg(clash.local), tr("From the update: %1").arg(clash.release),
         tr("The file as %1 shipped it").arg(s.name(s.runningRelease))},
        tr("One %1 that the program reading it accepts. What I changed stays as I set it. The update's own changes are "
           "kept where they do not undo that.")
            .arg(clash.path));
    auto *buttons = new QHBoxLayout;
    auto *hand = new QPushButton(tr("Decide by hand…"));
    auto *wait = new QPushButton(tr("Later"));
    auto *fixNow = primary(tr("Let the agent fix it"));
    connect(hand, &QPushButton::clicked, this, &ClashPage::byHand);
    connect(wait, &QPushButton::clicked, this, &ClashPage::later);
    connect(fixNow, &QPushButton::clicked, this, [this, prompt] { emit fix(prompt->prompt()); });
    buttons->addWidget(hand);
    buttons->addStretch();
    buttons->addWidget(wait);
    buttons->addWidget(fixNow);
    prompt->column()->addLayout(buttons);
    column->addWidget(prompt);
    column->addStretch();
}

// ---------------------------------------------------------------- What the agent did

void ProposedPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("The agent resolved the clash")));
    top->addWidget(dim(text(
        tr("Its result is in the next system, not in the running one. Nothing is final until you finish and restart."),
        true)));
    column->addLayout(top);

    auto *did = new Panel;
    did->column()->addWidget(strong(tr("What it did")));
    did->column()->addWidget(text(s.proposalSummary, true));
    if (!s.proposalScope.isEmpty())
        did->column()->addWidget(dim(text(s.proposalScope, true)));
    column->addWidget(did);

    auto *against = new Panel;
    against->column()->addWidget(strong(tr("Against what you expected")));
    auto *form = new QFormLayout;
    form->setHorizontalSpacing(24);
    form->setVerticalSpacing(7);
    for (const Check &check : s.checks) {
        QString words;
        if (check.verdict == QLatin1String("yes"))
            words = check.note.isEmpty() ? tr("Yes") : tr("Yes: %1").arg(check.note);
        else if (check.verdict == QLatin1String("no"))
            words = check.note.isEmpty() ? tr("No") : tr("No: %1").arg(check.note);
        else
            words = check.note.isEmpty() ? tr("Not known") : tr("Not known: %1").arg(check.note);
        QLabel *value = check.verdict == QLatin1String("yes") ? strong(words) : dim(text(words));
        form->addRow(text(check.expected), value);
    }
    against->column()->addLayout(form);
    column->addWidget(against);
    column->addStretch();

    auto *buttons = new QHBoxLayout;
    auto *again = new QPushButton(tr("Ask again…"));
    auto *hand = new QPushButton(tr("Decide by hand…"));
    auto *yes = primary(tr("Accept"));
    connect(again, &QPushButton::clicked, this, &ProposedPage::askAgain);
    connect(hand, &QPushButton::clicked, this, &ProposedPage::byHand);
    connect(yes, &QPushButton::clicked, this, &ProposedPage::accept);
    buttons->addWidget(again);
    buttons->addWidget(hand);
    buttons->addStretch();
    buttons->addWidget(yes);
    column->addLayout(buttons);
}

// ---------------------------------------------------------------- A question

ConfirmDialog::ConfirmDialog(const QString &title, const QString &question, const QStringList &paragraphs,
                             const QString &cost, const QString &yes, QWidget *parent)
    : QDialog(parent)
{
    setWindowTitle(title);
    auto *column = new QVBoxLayout(this);
    column->setContentsMargins(22, 20, 22, 18);
    column->setSpacing(12);
    QLabel *asked = strong(question);
    QFont larger = asked->font();
    larger.setPointSizeF(larger.pointSizeF() * 1.2);
    asked->setFont(larger);
    column->addWidget(asked);
    for (const QString &paragraph : paragraphs) {
        QLabel *words = text(paragraph, true);
        words->setMinimumWidth(460);
        column->addWidget(words);
    }
    if (!cost.isEmpty())
        column->addWidget(dim(text(cost, true)));
    column->addStretch();
    auto *buttons = new QDialogButtonBox(QDialogButtonBox::Cancel);
    buttons->addButton(yes, QDialogButtonBox::AcceptRole);
    buttons->button(QDialogButtonBox::Cancel)->setDefault(true);
    connect(buttons, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttons, &QDialogButtonBox::rejected, this, &QDialog::reject);
    column->addWidget(buttons);
}

// ---------------------------------------------------------------- Shared by the two pages

namespace {

QString whenText(qint64 unixTime)
{
    return QLocale().toString(QDateTime::fromSecsSinceEpoch(unixTime), QLocale::ShortFormat);
}

// "You" for what this window asked for (`--actor person`), "The agent" for
// the agent, the caller's own label for anyone else, nothing when recab has none.
QString whoText(const QString &actor)
{
    if (actor.isEmpty())
        return {};
    if (actor == QLatin1String("person"))
        return QObject::tr("You");
    if (actor.startsWith(QLatin1String("agent")))
        return QObject::tr("The agent");
    return actor;
}

void colour(QLabel *label, Mood mood)
{
    QPalette palette = label->palette();
    palette.setColor(QPalette::WindowText, moodColour(palette, mood));
    label->setPalette(palette);
}

// A button that follows recab's verdict; a refusal is its tooltip and, when
// `why` is given, one more line for the page to show under the buttons.
QPushButton *push(QHBoxLayout *row, const QString &words, const ActionVerdict &verdict, bool main = false,
                  QStringList *why = nullptr)
{
    QPushButton *button = main && verdict.ready ? primary(words) : new QPushButton(words);
    button->setEnabled(verdict.ready);
    if (!verdict.ready) {
        button->setToolTip(verdict.reason);
        if (why && !verdict.reason.isEmpty())
            *why << QObject::tr("%1 cannot be done now: %2").arg(QString(words).remove(QChar(0x2026)), verdict.reason);
    }
    row->addWidget(button);
    return button;
}

// The change names in a few words: "htop, nvidia-driver".
QString joined(const QStringList &words)
{
    return words.join(QObject::tr(", "));
}

// Everything in the next system, one row for each change: what, who, when.
// Who and when are shown only when recab gives them for some change.
QWidget *changeList(const Status &s)
{
    bool who = false, when = false;
    for (const ChangeItem &change : s.changes) {
        who |= !whoText(change.actor).isEmpty();
        when |= change.when > 0;
    }
    auto *inner = new QWidget;
    auto *grid = new QGridLayout(inner);
    grid->setContentsMargins(0, 0, 0, 0);
    grid->setHorizontalSpacing(20);
    grid->setVerticalSpacing(7);
    grid->setColumnStretch(0, 1);
    int row = 0;
    if (who || when) {
        grid->addWidget(dim(text(QObject::tr("Change"))), row, 0);
        if (who)
            grid->addWidget(dim(text(QObject::tr("Made by"))), row, 1);
        if (when)
            grid->addWidget(dim(text(QObject::tr("When"))), row, 2);
        ++row;
    }
    for (const ChangeItem &change : s.changes) {
        grid->addWidget(new ElidedLabel(change.what), row, 0);
        if (who)
            grid->addWidget(text(whoText(change.actor)), row, 1);
        if (when)
            grid->addWidget(text(change.when > 0 ? whenText(change.when) : QString()), row, 2);
        ++row;
    }
    auto *panel = new Panel;
    if (s.changes.size() > 6) {
        // A long list scrolls inside its panel and does not push the page out.
        auto *scroll = new QScrollArea;
        scroll->setWidgetResizable(true);
        scroll->setFrameShape(QFrame::NoFrame);
        scroll->setFixedHeight(190);
        scroll->setWidget(inner);
        panel->column()->addWidget(scroll);
    } else {
        panel->column()->addWidget(inner);
    }
    return panel;
}

} // namespace

// ---------------------------------------------------------------- The next system

void NextPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
    if(!s.operations.isEmpty())lifecyclePage(column,s,this,[this](const QJsonObject &op){emit operation(op);});
    column->setSpacing(12);
    const bool next = s.otherRole == QLatin1String("next");
    const QString state = s.state;
    const bool clash = next && state == QLatin1String("preparing") && !s.clashes.isEmpty();
    const qsizetype held = s.changes.size();
    const QString running = s.name(s.runningRelease);
    const QString holds = held == 0 ? tr("It holds no change")
                                    : counted(held, tr("It holds %1 change"), tr("It holds %1 changes"));

    // What is true now, in a heading and a sentence: one for each state.
    QString title, under;
    if (state == QLatin1String("on_trial")) {
        title = tr("You are on the next system, on trial");
        under = tr("It started, and is not kept yet: a restart now goes back to the system from before. recab keeps "
                   "it once start-up has finished properly.");
    } else if (!next) {
        title = tr("There is no next system yet");
        if (s.otherRole == QLatin1String("previous"))
            under = tr("It is made when you make a change: a copy of the running system, in the place that holds the "
                       "system from before. After that you can no longer go back to that system; the running system "
                       "stays your way back.");
        else if (s.otherRole == QLatin1String("copy"))
            under = tr("The place beside the running system holds a copy of it. A change goes into that copy, which "
                       "then becomes the next system.");
        else
            under = tr("It is made when you make a change: a copy of the running system, beside it.");
        under += tr(" The running system is locked and is not changed.");
    } else if (state == QLatin1String("working")) {
        title = tr("The next system is being worked on");
        under = tr("A recab command is %1 right now, started outside this window. This page reads again until it has "
                   "finished.")
                    .arg(s.interruptedDuring.isEmpty() ? tr("working on the next system") : s.interruptedDuring);
    } else if (state == QLatin1String("interrupted")) {
        title = tr("Preparing the next system was interrupted");
        under = tr("It stopped while %1. The next system cannot be started in this state. The running system was not "
                   "touched.")
                    .arg(s.interruptedDuring);
    } else if (state == QLatin1String("failed")) {
        title = tr("The next system did not start, so the computer came back");
        under = tr("You are on %1, exactly as it was. The next system still holds its changes; nothing in it was "
                   "thrown away.")
                    .arg(running);
    } else if (state == QLatin1String("trial_pending")) {
        title = tr("The next system starts at the next restart");
        under = tr("%1 and is finished. The restart into it is set: any restart starts it, whoever makes it. Until "
                   "then nothing in it is in effect, and you can cancel.")
                    .arg(holds);
    } else if (state == QLatin1String("ready")) {
        title = tr("The next system is ready");
        under = tr("%1 and is finished: checked and closed, so it can be started. Nothing in it is in effect until "
                   "the computer restarts into it.")
                    .arg(holds);
    } else if (clash) {
        title = tr("The update clashes with one of your changes");
        under = tr("Nothing was chosen for you. The next system cannot be finished until that is settled.");
    } else {
        title = held == 0 ? tr("The next system holds no changes yet")
                          : counted(held, tr("The next system holds %1 change"), tr("The next system holds %1 changes"));
        under = held == 0 ? tr("It is the running system, unchanged. It is open: not finished, and nothing in it is "
                               "in effect.")
                          : tr("It is the running system plus what is listed here. It is open: not finished, and "
                               "nothing in it is in effect.");
    }
    auto *head = new QHBoxLayout;
    head->setSpacing(20);
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(title));
    top->addWidget(dim(text(under, true)));
    head->addLayout(top, 1);
    // Whatever state the next system is in, it can be made the running
    // system again: every change given up, the place an exact copy.
    if (next && state != QLatin1String("working")) {
        auto *drop = new QPushButton(tr("Reset to the running system…"));
        drop->setEnabled(s.can("discard").ready);
        drop->setToolTip(s.can("discard").ready
                             ? tr("Gives up every change in the next system and makes it an exact copy of the running "
                                  "system again. The running system is not touched.")
                             : s.can("discard").reason);
        connect(drop, &QPushButton::clicked, this, &NextPage::reset);
        head->addWidget(drop, 0, Qt::AlignTop);
    }
    column->addLayout(head);

    // No next system: the one way to make it, and nothing else to say.
    if (!next && state != QLatin1String("on_trial")) {
        auto *panel = new Panel;
        auto *buttons = new QHBoxLayout;
        QStringList why;
        connect(push(buttons, tr("Make a change…"), s.can("change"), true, &why), &QPushButton::clicked, this,
                &NextPage::ask);
        connect(push(buttons, tr("Install software…"), s.can("change")), &QPushButton::clicked, this,
                &NextPage::installSoftware);
        buttons->addStretch();
        panel->column()->addLayout(buttons);
        if (!why.isEmpty())
            panel->column()->addWidget(dim(text(why.first(), true)));
        column->addWidget(panel);
        column->addStretch();
        return;
    }

    // What it holds, and what will differ once it is started.
    if (next) {
        auto *both = new QHBoxLayout;
        both->setSpacing(14);
        auto *left = new QVBoxLayout;
        left->setSpacing(8);
        left->addWidget(strong(tr("Everything in it")));
        if (held == 0)
            left->addWidget(dim(text(state == QLatin1String("preparing")
                                         ? tr("Nothing yet: it is exactly the running system.")
                                         : tr("recab lists no finished change in it."),
                                     true)));
        else
            left->addWidget(changeList(s));
        if (state == QLatin1String("working")) {
            // Said once, here: nothing can be started while a command works.
            left->addWidget(dim(text(tr("Nothing can be started while that command runs. Two commands never work on "
                                        "the next system at once."),
                                     true)));
        } else {
            auto *buttons = new QHBoxLayout;
            connect(push(buttons, tr("Make a change…"), s.can("change")), &QPushButton::clicked, this, &NextPage::ask);
            connect(push(buttons, tr("Install software…"), s.can("change")), &QPushButton::clicked, this,
                    &NextPage::installSoftware);
            buttons->addStretch();
            left->addLayout(buttons);
        }
        left->addStretch();
        both->addLayout(left, 3);

        const bool differs = state == QLatin1String("preparing") || state == QLatin1String("ready")
                             || state == QLatin1String("trial_pending") || state == QLatin1String("failed");
        if (differs) {
            auto *right = new Panel;
            auto *facts = new QGridLayout;
            facts->setHorizontalSpacing(14);
            facts->setVerticalSpacing(6);
            facts->setColumnMinimumWidth(0, 78);
            facts->setColumnStretch(1, 1);
            int row = 0;
            auto fact = [&](const QString &name, const QString &words) {
                facts->addWidget(dim(text(name)), row, 0, Qt::AlignTop);
                facts->addWidget(text(words, true), row, 1);
                ++row;
            };
            QString starts = tr("the next system");
            if (!s.otherSlot.isEmpty())
                starts += s.otherDevice.isEmpty() ? tr(", in slot %1").arg(s.otherSlot)
                                                  : tr(", in slot %1 (%2)").arg(s.otherSlot, s.otherDevice);
            starts += state == QLatin1String("trial_pending") ? tr(": that restart is set")
                                                              : tr(", when you restart into it");
            fact(tr("Starts"), starts);
            if (!s.otherRelease.isEmpty() && s.otherRelease != s.runningRelease)
                fact(tr("Release"), s.name(s.otherRelease));
            QStringList added, updated, removed, software, files, others;
            for (const ChangeItem &change : s.changes) {
                if (change.action == QLatin1String("install"))
                    added << change.package;
                else if (change.action == QLatin1String("update"))
                    updated << change.package;
                else if (change.action == QLatin1String("remove"))
                    removed << change.package;
                else if (change.action == QLatin1String("edit"))
                    files << change.package;
                else if (change.kind == QLatin1String("software"))
                    software << change.what;
                else if (change.kind == QLatin1String("file"))
                    files << change.what;
                else if (change.kind != QLatin1String("update"))
                    others << change.what;
            }
            QStringList said;
            if (!added.isEmpty())
                said << tr("added: %1").arg(joined(added));
            if (!updated.isEmpty())
                said << tr("updated: %1").arg(joined(updated));
            if (!removed.isEmpty())
                said << tr("removed: %1").arg(joined(removed));
            if (!software.isEmpty())
                said << joined(software);
            fact(tr("Software"), said.isEmpty() ? tr("no change") : said.join(tr("; ")));
            fact(tr("Files"), files.isEmpty() ? tr("no file edited")
                                              : (s.changes.first().action.isEmpty() ? joined(files)
                                                                                    : tr("edited: %1").arg(joined(files))));
            if (!others.isEmpty())
                fact(tr("Other"), joined(others));
            fact(tr("Your files"), tr("the same: home is shared by both systems"));
            right->column()->addWidget(strong(tr("What will differ after the restart")));
            right->column()->addLayout(facts);
            right->column()->addWidget(dim(text(tr("If it does not start properly, the computer comes back to the "
                                                   "running system by itself, and the changes are kept for another "
                                                   "try."),
                                                true)));
            auto *disk = link(tr("See the two systems as blocks in Disk"));
            connect(disk, &ClickLabel::clicked, this, &NextPage::openDisk);
            right->column()->addWidget(disk, 0, Qt::AlignLeft);
            right->column()->addStretch();
            both->addWidget(right, 2);
        }
        column->addLayout(both, 1);
    }

    if (state == QLatin1String("working")) {
        column->addStretch();
        return;
    }

    // The three steps, the one that is current marked, and what fits it.
    int current = 0;
    if (state == QLatin1String("ready") || state == QLatin1String("trial_pending"))
        current = 1;
    else if (state == QLatin1String("on_trial") || state == QLatin1String("failed"))
        current = 2;
    QString after = tr("It is on trial until start-up has finished properly, then it is kept. If it does not start "
                       "properly, the computer comes back by itself.");
    if (state == QLatin1String("on_trial"))
        after = tr("It started and is not kept yet. Keep it, or go back to the system from before. recab keeps it once "
                   "start-up has finished properly.");
    else if (state == QLatin1String("failed"))
        after = tr("It did not start properly, so the computer came back to the running system. The changes are kept.");
    const QString steps[3][2] = {
        {tr("1. Finish"), state == QLatin1String("interrupted")
                              ? tr("It stopped half way. Continue where it stopped, or give it up.")
                              : tr("It is checked and closed, so it can be started. Until then a restart cannot start "
                                   "it.")},
        {tr("2. Choose when"), tr("Restart now, or let it start at the next restart, whoever makes it. Until then you "
                                  "can cancel.")},
        {tr("3. After the restart"), after}};

    auto *panel = new Panel;
    panel->column()->setSpacing(12);
    auto *cols = new QHBoxLayout;
    cols->setSpacing(22);
    for (int i = 0; i < 3; ++i) {
        auto *mark = new StepMark(i + 1);
        mark->setState(i < current ? StepMark::Done
                                   : i == current ? (state == QLatin1String("failed") ? StepMark::Failed
                                                                                      : StepMark::Running)
                                                  : StepMark::Pending);
        auto *words = new QVBoxLayout;
        words->setSpacing(1);
        QLabel *name = strong(steps[i][0]);
        if (i != current)
            dim(name);
        words->addWidget(name);
        words->addWidget(dim(text(steps[i][1], true)));
        words->addStretch();
        auto *one = new QHBoxLayout;
        one->setSpacing(10);
        one->addWidget(mark, 0, Qt::AlignTop);
        one->addLayout(words, 1);
        cols->addLayout(one, 1);
    }
    panel->column()->addLayout(cols);

    auto *buttons = new QHBoxLayout;
    QStringList why;
    if (state == QLatin1String("on_trial")) {
        connect(push(buttons, tr("Keep it"), s.can("keep"), true, &why), &QPushButton::clicked, this, &NextPage::keep);
        connect(push(buttons, tr("Go back…"), s.can("go_back"), false, &why), &QPushButton::clicked, this,
                &NextPage::goBack);
    } else if (state == QLatin1String("interrupted")) {
        connect(push(buttons, tr("Continue where it stopped"), s.can("recover"), true, &why), &QPushButton::clicked,
                this, &NextPage::recover);
        connect(push(buttons, tr("Discard…"), s.can("discard"), false, &why), &QPushButton::clicked, this,
                &NextPage::discard);
    } else if (state == QLatin1String("trial_pending")) {
        connect(push(buttons, tr("Restart now"), s.can("restart"), true, &why), &QPushButton::clicked, this,
                &NextPage::restart);
        connect(push(buttons, tr("Cancel the set restart"), s.can("keep"), false, &why), &QPushButton::clicked, this,
                &NextPage::cancelRestart);
    } else if (state == QLatin1String("ready")) {
        connect(push(buttons, tr("Restart now"), s.can("restart"), true, &why), &QPushButton::clicked, this,
                &NextPage::restart);
        connect(push(buttons, tr("Start it at the next restart"), s.can("restart"), false, &why), &QPushButton::clicked,
                this, &NextPage::restartLater);
    } else if (state == QLatin1String("failed")) {
        connect(push(buttons, tr("Try the same again"), s.can("restart"), true, &why), &QPushButton::clicked, this,
                &NextPage::restart);
        auto *agent = new QPushButton(tr("Hand it to the agent…"));
        connect(agent, &QPushButton::clicked, this, &NextPage::openOverview);
        buttons->addWidget(agent);
    } else if (clash) {
        auto *settle = primary(tr("Settle the clash…"));
        connect(settle, &QPushButton::clicked, this, &NextPage::settle);
        buttons->addWidget(settle);
    } else {
        connect(push(buttons, tr("Finish…"), s.can("finalize"), true, &why), &QPushButton::clicked, this,
                &NextPage::finish);
    }
    auto *last = new QHBoxLayout;
    last->setSpacing(20);
    QLabel *because = dim(text(why.isEmpty() ? QString() : why.first(), true));
    last->addWidget(because, 1);
    last->addLayout(buttons);
    panel->column()->addLayout(last);
    column->addWidget(panel);
    column->addStretch(next ? 0 : 1);
}

// ---------------------------------------------------------------- Getting back

namespace {

// What the start-up list calls each system, which one is the running system,
// and how to make the choice stay. Only what recab's source or status says
// (the page's README names where each detail comes from).
QStringList backSteps(const Status &s)
{
    const QString running = s.menuName(s.runningSlot), other = s.menuName(s.otherSlot);
    const bool named = !running.isEmpty() && !other.isEmpty();
    const bool previous = s.otherRole == QLatin1String("previous");
    QStringList steps;
    steps << QObject::tr("Restart the computer and watch for the list of systems at start-up. recab's loader.conf keeps it "
                         "up for 5 seconds.");
    QString pick;
    if (s.state == QLatin1String("on_trial") && previous) {
        pick = named ? QObject::tr("Pick \"%1\". That is the system from before. The running system, \"%2\", is on "
                                   "trial and is not kept.")
                           .arg(other, running)
                     : QObject::tr("Pick the system from before. The running system is on trial and is not kept.");
    } else if (named) {
        pick = QObject::tr("Pick \"%1\". That is the running system today, and it is kept.").arg(running);
        if (previous)
            pick += QObject::tr(" If it is this system that breaks later, pick \"%1\" instead: it holds the system "
                                "from before.")
                        .arg(other);
        else if (s.otherRole == QLatin1String("next"))
            pick += QObject::tr(" \"%1\" holds the next system.").arg(other);
        else if (s.otherRole == QLatin1String("copy"))
            pick += QObject::tr(" \"%1\" holds a copy of it.").arg(other);
    } else {
        pick = QObject::tr("Pick the running system, the one that is kept.");
        if (previous)
            pick += QObject::tr(" If it is the running system that breaks later, pick the system from before instead.");
    }
    steps << pick;
    steps << QObject::tr("To make that choice stay, run \"recab commit\" as administrator once it has started. It makes "
                         "the running system the one that every restart starts.");
    return steps;
}

// The entries recab keeps for the two systems, when it says.
QString backEntries(const Status &s)
{
    if (s.runningEntry.isEmpty() || s.otherEntry.isEmpty())
        return {};
    QStringList both{s.runningEntry, s.otherEntry};
    both.sort(); // slot A first, whichever is running
    return QObject::tr("recab's entries for them, on the boot partition: %1 and %2.").arg(both[0], both[1]);
}

} // namespace

QString backStepsText(const Status &s)
{
    QString out = QObject::tr("If the desktop does not start\n\n");
    out += QObject::tr("A next system that fails to start properly is left by itself: the computer restarts into the "
                       "system it came from. If that does not happen, or a kept system breaks later:\n\n");
    const QStringList steps = backSteps(s);
    for (int i = 0; i < steps.size(); ++i)
        out += QStringLiteral("%1. %2\n\n").arg(i + 1).arg(steps[i]);
    const QString entries = backEntries(s);
    if (!entries.isEmpty())
        out += entries + QStringLiteral("\n\n");
    out += QObject::tr("These names change with every restart into a new system, so this was written for today: "
                       "%1, read from recab at %2.\n")
               .arg(s.name(s.runningRelease), QLocale().toString(QDateTime::currentDateTime(), QLocale::ShortFormat));
    return out;
}

void BackPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
    column->setSpacing(12);
    const bool onTrial = s.state == QLatin1String("on_trial");
    const bool next = s.otherRole == QLatin1String("next");

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Getting back")));
    top->addWidget(dim(text(tr("What you can turn back from here, and what to do when the desktop does not start at "
                               "all."),
                            true)));
    column->addLayout(top);

    // Three rows: each says what is true now and offers what recab allows.
    auto *rows = new Panel;
    QVBoxLayout *list = rows->column();
    list->setSpacing(8);
    auto row = [&](const QString &name, const QString &line, Mood mood, const QString &button, bool enabled,
                   bool main, auto &&then) {
        if (list->count() > 0)
            list->addWidget(rule());
        auto *one = new QHBoxLayout;
        one->setSpacing(20);
        auto *words = new QVBoxLayout;
        words->setSpacing(1);
        QLabel *title = strong(name);
        words->addWidget(title);
        QLabel *said = text(line, true);
        if (mood == Mood::Plain)
            dim(said);
        else
            colour(said, mood);
        words->addWidget(said);
        one->addLayout(words, 1);
        QPushButton *b = main && enabled ? primary(button) : new QPushButton(button);
        b->setMinimumWidth(190);
        b->setEnabled(enabled);
        one->addWidget(b);
        list->addLayout(one);
        connect(b, &QPushButton::clicked, this, then);
    };

    // Go back to the system from before.
    const bool previous = s.otherRole == QLatin1String("previous");
    const bool canBack = s.can("go_back").ready && (previous || onTrial);
    QString back;
    if (canBack)
        back = onTrial ? tr("Restarts into the system from before. This system is left as it is, so its changes can "
                            "be tried again.")
                       : tr("Restarts into the system from before. The system you are running now is kept beside it, "
                            "so you can return.");
    else if (next)
        back = tr("There is none now: its place holds the next system.");
    else if (s.otherRole == QLatin1String("copy"))
        back = tr("There is none: the other place holds a copy of the running system.");
    else
        back = s.can("go_back").reason;
    row(tr("Go back to the system from before"), back, Mood::Plain, tr("Go back…"), canBack, false,
        [this] { emit goBack(); });

    // Keep the running system.
    QString kept;
    if (onTrial) {
        kept = tr("It started, and is not kept yet. Keep it now, or recab keeps it once start-up has finished "
                  "properly.");
    } else if (s.trialOutcome == QLatin1String("kept") && !s.trialWhen.isEmpty()) {
        kept = tr("Kept already: it started properly on %1.").arg(s.trialWhen);
    } else if (s.trialOutcome == QLatin1String("failed") && !s.trialWhen.isEmpty()) {
        kept = tr("Kept already: the computer came back to it on %1, when the next system did not start properly.")
                   .arg(s.trialWhen);
    } else {
        kept = tr("Kept already.");
    }
    row(tr("Keep the running system"), kept, onTrial ? Mood::Plain : Mood::Ok, tr("Keep it"),
        onTrial && s.can("keep").ready, true, [this] { emit keep(); });

    // Cancel a set restart.
    const bool set = s.state == QLatin1String("trial_pending");
    row(tr("Cancel a set restart"),
        set ? tr("A restart into the next system is set: any restart starts it. Cancelling leaves the next system "
                 "ready, and the next restart starts the running system.")
            : tr("None is set. Once the next system is finished and chosen, any restart starts it, until you cancel "
                 "here."),
        Mood::Plain, tr("Cancel it"), set && s.can("keep").ready, false, [this] { emit cancelRestart(); });
    column->addWidget(rows);

    auto *both = new QHBoxLayout;
    both->setSpacing(14);

    // If the desktop does not start.
    auto *stuck = new Card(Card::Open);
    stuck->column()->setSpacing(8);
    auto *stuckTop = new QHBoxLayout;
    stuckTop->addWidget(strong(tr("If the desktop does not start")), 1);
    auto *save = new QPushButton(tr("Save as a file…"));
    connect(save, &QPushButton::clicked, this, &BackPage::saveSteps);
    stuckTop->addWidget(save);
    stuck->column()->addLayout(stuckTop);
    stuck->column()->addWidget(text(tr("A next system that fails to start is left by itself: the computer comes back. "
                                       "If that does not happen, or a kept system breaks later:"),
                                    true));
    const QStringList steps = backSteps(s);
    for (int i = 0; i < steps.size(); ++i) {
        auto *one = new QHBoxLayout;
        one->setSpacing(10);
        auto *mark = new StepMark(i + 1);
        one->addWidget(mark, 0, Qt::AlignTop);
        one->addWidget(text(steps[i], true), 1);
        stuck->column()->addLayout(one);
    }
    const QString entries = backEntries(s);
    // The names stay; which of the two is the running one is what changes.
    const QString today = tr("Which of the two is the running system changes with every restart into a new system. "
                             "This page shows how it is now.");
    stuck->column()->addWidget(dim(text(entries.isEmpty() ? today : today + QLatin1Char(' ') + entries, true)));
    stuck->column()->addStretch();
    both->addWidget(stuck, 3);

    // What going back does not turn back, and how the running system started.
    auto *right = new QVBoxLayout;
    right->setSpacing(10);
    auto *kept2 = new Panel;
    kept2->column()->setSpacing(6);
    kept2->column()->addWidget(strong(tr("What going back does not turn back")));
    for (const QString &line :
         {tr("Your files. Home is shared by both systems."),
          tr("What is kept in /var, which both systems share. A program that changed its own data for a newer "
             "version may not read it with the older one.")})
        kept2->column()->addWidget(text(QStringLiteral("• ") + line, true));
    right->addWidget(kept2);

    auto *started = new Panel;
    started->column()->setSpacing(6);
    started->column()->addWidget(strong(tr("How the running system started")));
    auto *facts = new QGridLayout;
    facts->setHorizontalSpacing(14);
    facts->setVerticalSpacing(6);
    facts->setColumnMinimumWidth(0, 76);
    facts->setColumnStretch(1, 1);
    int at = 0;
    auto fact = [&](const QString &name, const QString &words) {
        facts->addWidget(dim(text(name)), at, 0, Qt::AlignTop);
        facts->addWidget(text(words, true), at, 1);
        ++at;
    };
    if (onTrial)
        fact(tr("Start-up"), tr("started, and not kept yet"));
    if (s.trialOutcome.isEmpty()) {
        fact(tr("Last trial"), tr("recab holds no record of one"));
    } else {
        QString outcome = tr("kept");
        if (s.trialOutcome == QLatin1String("failed"))
            outcome = tr("did not start properly, so the computer came back");
        else if (s.trialOutcome == QLatin1String("other"))
            outcome = tr("kept, but it was the other system that was tried");
        fact(tr("Last trial"), s.trialWhen.isEmpty() ? outcome : tr("%1, %2").arg(s.trialWhen, outcome));
        if (!s.trialDetail.isEmpty())
            fact(tr("recab says"), s.trialDetail);
    }
    started->column()->addLayout(facts);
    right->addWidget(started);
    right->addStretch();
    both->addLayout(right, 2);
    column->addLayout(both, 1);
}
