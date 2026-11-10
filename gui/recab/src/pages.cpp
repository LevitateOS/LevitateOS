#include "pages.h"

#include "parts.h"
#include "run.h"

#include <QDialogButtonBox>
#include <QFontDatabase>
#include <QFormLayout>
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

void SystemPage::show(const Status &s)
{
    QVBoxLayout *column = fresh();
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
        if (s.state == QLatin1String("ready") || s.state == QLatin1String("trial_pending")) {
            title = tr("The next system is ready");
            under = tr("It is checked and closed. Restart to start using it; until then you stay on the running system.");
        } else if (next && !s.clashes.isEmpty()) {
            title = tr("The update clashes with one of your changes");
            under = tr("Nothing was chosen for you. The next system cannot be finished until that is settled.");
        } else if (next && !s.changesKnown) {
            // The real recab names the last change, not how many there are.
            title = tr("The next system is open");
            under = tr("What is changed in it takes effect when you restart into it. The running system is "
                       "untouched until then.");
        } else if (next) {
            title = counted(s.changes.size(), tr("The next system has %1 change"), tr("The next system has %1 changes"));
            under = tr("They take effect when you restart into it. The running system is untouched until then.");
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
        auto *drop = new QPushButton(tr("Discard the next system…"));
        auto *fixNow = primary(tr("Let the agent fix it"));
        connect(again, &QPushButton::clicked, this, &SystemPage::retry);
        connect(drop, &QPushButton::clicked, this, &SystemPage::discard);
        connect(fixNow, &QPushButton::clicked, this, [this, prompt] { emit fix(prompt->prompt()); });
        buttons->addWidget(again);
        buttons->addWidget(drop);
        buttons->addStretch();
        buttons->addWidget(fixNow);
        prompt->column()->addLayout(buttons);
        column->addWidget(prompt);
        column->addStretch();
        return;
    }

    // The two systems, side by side.
    auto *pair = new QHBoxLayout;
    pair->setSpacing(14);
    QStringList runningLines{tr("Running now"), tr("Locked")};
    if (next)
        runningLines << (s.state == QLatin1String("interrupted") ? tr("Untouched") : tr("Your way back"));
    else if (!s.runningHolds.isEmpty())
        runningLines << tr("With: %1").arg(s.runningHolds.join(tr("; ")));
    else
        runningLines << tr("Kept: it started properly");
    pair->addWidget(card(Card::On, Glyph::Lock, tr("The running system"), running, runningLines), 1);

    if (next) {
        const bool plus = !titles.isEmpty();
        const QString under = !plus ? s.name(s.otherRelease)
                              : s.changesKnown ? tr("%1, plus:").arg(s.name(s.otherRelease))
                                               : tr("%1; the last change:").arg(s.name(s.otherRelease));
        if (s.state == QLatin1String("interrupted"))
            pair->addWidget(card(Card::Open, Glyph::Plus, tr("The next system"), under, titles,
                                 tr("Incomplete: cannot be started")), 1);
        else if (s.otherState == QLatin1String("ready"))
            pair->addWidget(card(Card::Ready, Glyph::Check, tr("The next system"), under,
                                 titles + QStringList{tr("Ready: starts when you restart into it")}), 1);
        else
            pair->addWidget(card(Card::Open, Glyph::Plus, tr("The next system"), under, titles,
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

    // What can be done, with recab's verdict on each.
    auto *group = new Panel;
    QVBoxLayout *list = group->column();
    list->addWidget(strong(tr("What you can do")));
    const QString updateName = s.updateRelease.isEmpty() ? QString() : s.name(s.updateRelease);

    if (s.state == QLatin1String("interrupted")) {
        connect(act(list, tr("Continue where it stopped"),
                    tr("recab looks at what is really there and carries on. Nothing that was finished is done again."),
                    tr("Continue"), s.can("recover"), true),
                &QPushButton::clicked, this, &SystemPage::recover);
        connect(act(list, tr("Discard the next system"),
                    tr("Gives up the half-made change. It does not bring the system from before back: its place was "
                       "already reused."),
                    tr("Discard…"), s.can("discard")),
                &QPushButton::clicked, this, &SystemPage::discard);
    } else if (s.state == QLatin1String("ready") || s.state == QLatin1String("trial_pending")) {
        connect(act(list, tr("Restart into the next system"),
                    tr("If it does not start properly, the computer comes back to the running system by itself. Its "
                       "changes are not lost."),
                    tr("Restart into it"), s.can("restart"), true),
                &QPushButton::clicked, this, &SystemPage::restart);
        connect(act(list, tr("Change something else"),
                    tr("Opens the next system again. It must be finished again before a restart."),
                    tr("Make a change…"), s.can("change")),
                &QPushButton::clicked, this, &SystemPage::ask);
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
        connect(act(list, tr("Change something else"),
                    tr("Say what you want; the agent adds it to this same next system. Nothing is copied again."),
                    tr("Make a change…"), s.can("change")),
                &QPushButton::clicked, this, &SystemPage::ask);
    } else {
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
        if (s.otherRole == QLatin1String("previous"))
            connect(act(list, tr("Go back to %1").arg(s.otherRelease.isEmpty() ? tr("the system from before") : s.name(s.otherRelease)), tr("Restart into the system from before."),
                        tr("Go back…"), s.can("go_back")),
                    &QPushButton::clicked, this, &SystemPage::goBack);
    }
    column->addWidget(group);
    column->addStretch();

    if (next && s.state != QLatin1String("interrupted")) {
        auto *drop = new ClickLabel(tr("Discard the next system's changes…"));
        dim(drop);
        connect(drop, &ClickLabel::clicked, this, &SystemPage::discard);
        column->addWidget(drop, 0, Qt::AlignLeft);
    }
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
