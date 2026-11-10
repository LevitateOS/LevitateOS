#include "window.h"

#include "pages.h"

#include "refresh.h"

#include <QTimer>
#include <QLabel>
#include <QLocale>
#include <QMessageBox>
#include <QPushButton>
#include <QStackedWidget>
#include <QStatusBar>
#include <QTime>
#include <QToolButton>

Window::Window(QWidget *parent)
    : QMainWindow(parent)
{
    setWindowTitle(tr("System"));

    stack_ = new QStackedWidget;
    system_ = new SystemPage;
    ask_ = new AskPage;
    work_ = new WorkPage;
    clash_ = new ClashPage;
    proposed_ = new ProposedPage;
    for (QWidget *page : {static_cast<QWidget *>(system_), static_cast<QWidget *>(ask_), static_cast<QWidget *>(work_),
                          static_cast<QWidget *>(clash_), static_cast<QWidget *>(proposed_)})
        stack_->addWidget(page);
    setCentralWidget(stack_);

    source_ = new QLabel(tr("Read from recab on this machine"));
    activity_ = new QLabel(tr("Agent: %1").arg(client_.agentName()));
    statusBar()->addWidget(source_);
    statusBar()->addPermanentWidget(activity_);
    statusBar()->setSizeGripEnabled(false);

    connect(system_, &SystemPage::ask, this, [this] { showAsk(); });
    connect(system_, &SystemPage::addUpdate, this, [this] { addUpdate(); });
    connect(system_, &SystemPage::goBack, this, [this] { goBack(); });
    connect(system_, &SystemPage::keep, this, &Window::keep);
    connect(system_, &SystemPage::restartLater, this, &Window::restartLater);
    connect(system_, &SystemPage::cancelRestart, this, &Window::cancelRestart);
    connect(system_, &SystemPage::discard, this, [this] { discard(); });
    connect(system_, &SystemPage::reset, this, [this] { reset(); });
    connect(system_, &SystemPage::finish, this, &Window::finish);
    connect(system_, &SystemPage::restart, this, &Window::restart);
    connect(system_, &SystemPage::retry, this, &Window::restart);
    connect(system_, &SystemPage::recover, this, &Window::recover);
    connect(system_, &SystemPage::settle, this, &Window::showClash);
    connect(system_, &SystemPage::fix, this,
            [this](const QString &prompt) { fixWithAgent(prompt, tr("The agent is changing the next system")); });

    connect(ask_, &AskPage::back, this, &Window::showMain);
    connect(ask_, &AskPage::handOver, this, [this](const QString &prompt) { handOver(prompt); });

    connect(work_, &WorkPage::back, this, &Window::showMain);
    connect(work_, &WorkPage::activity, this, [this](const QString &words) {
        activity_->setText(words.isEmpty() ? tr("Agent: %1").arg(client_.agentName()) : words);
    });
    connect(work_, &WorkPage::done, this, [this](bool ok) {
        run_ = nullptr;
        auto then = then_;
        then_ = nullptr;
        reload([then, ok] {
            if (then)
                then(ok);
        });
    });

    connect(clash_, &ClashPage::later, this, &Window::showMain);
    connect(clash_, &ClashPage::byHand, this, &Window::decideByHand);
    connect(clash_, &ClashPage::fix, this,
            [this](const QString &prompt) { fixWithAgent(prompt, tr("The agent is settling the clash")); });

    connect(proposed_, &ProposedPage::accept, this, &Window::acceptProposal);
    connect(proposed_, &ProposedPage::askAgain, this, &Window::showClash);
    connect(proposed_, &ProposedPage::byHand, this, &Window::decideByHand);

    connect(system_, &SystemPage::readAgain, this, [this] { reload([this] { showMain(); }); });
    connect(system_, &SystemPage::readAsAdministrator, this, [this] {
        asRoot_ = true;
        reload([this] { showMain(); });
    });

    // F5, and coming back to the window after a while: an agent or a terminal
    // may have changed the system in the meantime.
    refresher_ = new Refresher(this, 20);
    connect(refresher_, &Refresher::asked, this, [this](bool byHand) {
        if (stack_->currentWidget() == system_ || byHand)
            refresh();
    });
    auto *again = new QToolButton;
    again->setText(tr("Read again"));
    again->setToolTip(tr("Read the system again (F5)"));
    again->setAutoRaise(true);
    connect(again, &QToolButton::clicked, this, &Window::refresh);
    statusBar()->addWidget(again);

    reload([this] { showMain(); });
}

void Window::refresh()
{
    if (busy())
        return;
    // Only the main page is drawn from the state alone; a page being filled
    // in is left as it is.
    const bool main = stack_->currentWidget() == system_;
    reload([this, main] {
        if (main)
            showMain();
    });
}

void Window::reload(std::function<void()> then)
{
    if (reading_)
        return;
    reading_ = client_.read(asRoot_);
    if (!read_) {
        system_->showLoading(tr("Reading the system"),
                             tr("Asking recab what is running, what is beside it and what can be done."));
        stack_->setCurrentWidget(system_);
    }
    source_->setText(tr("Reading the system…"));
    connect(reading_, &Run::finished, this, [this, then](bool ok) {
        Run *run = reading_;
        reading_ = nullptr;
        run->deleteLater();
        QString problem;
        Status status;
        bool permission = false;
        if (!client_.understand(run, ok, &status, &problem, &permission)) {
            source_->setText(tr("The system could not be read"));
            system_->showProblem(problem, permission && !asRoot_);
            stack_->setCurrentWidget(system_);
            asRoot_ = false; // a refused password is not asked for again unasked
            return;
        }
        status_ = status;
        read_ = true;
        refresher_->fresh();
        source_->setText(tr("Read from recab at %1").arg(QLocale().toString(QTime::currentTime(), QLocale::ShortFormat)));
        if (then)
            then();
        // A command started elsewhere is working: look again until it is done.
        if (status_.state == QLatin1String("working"))
            QTimer::singleShot(2000, this, &Window::refresh);
    });
    reading_->start();
}

bool Window::agentAsks() const
{
    return run_ && work_->asking();
}

void Window::answerAgent(const QString &words)
{
    work_->answer(words);
}

void Window::showMain()
{
    system_->show(status_);
    stack_->setCurrentWidget(system_);
}

void Window::showAsk(const QString &request)
{
    ask_->show(status_, request);
    stack_->setCurrentWidget(ask_);
}

void Window::showClash()
{
    clash_->show(status_);
    stack_->setCurrentWidget(clash_);
}

ConfirmDialog *Window::reuseQuestion()
{
    const QString before = status_.otherRelease.isEmpty() ? tr("the system from before")
                                                          : status_.name(status_.otherRelease);
    return new ConfirmDialog(
        tr("The system is locked"), tr("This change goes into the next system"),
        {tr("The running system is locked, so it is not changed. The change is made in the next system and takes "
            "effect when you restart."),
         tr("The next system starts as an exact copy of the running one. Only what differs is copied.")},
        tr("It takes the place of %1. After this you can no longer go back to %1; the running system stays your "
           "way back.")
            .arg(before),
        tr("Prepare the next system"), this);
}

// Run one long thing on the work page, then read the system again.
void Window::work(Run *run, const QString &title, const QString &subtitle, std::function<void(bool)> then)
{
    run_ = run;
    then_ = std::move(then);
    work_->begin(title, subtitle, run);
    stack_->setCurrentWidget(work_);
    run->start();
}

// Short changes with no page of their own, one after another.
void Window::chain(QList<Run *> runs)
{
    if (runs.isEmpty()) {
        reload([this] { showMain(); });
        return;
    }
    Run *run = runs.takeFirst();
    setEnabled(false);
    ++quiet_;
    connect(run, &Run::finished, this, [this, run, runs](bool ok) {
        setEnabled(true);
        --quiet_;
        const QString reason = run->failure()["reason"].toString();
        run->deleteLater();
        if (!ok) {
            for (Run *left : runs)
                left->deleteLater();
            QMessageBox::warning(this, tr("System"),
                                 reason.isEmpty() ? tr("The change was not made.") : reason);
            reload([this] { showMain(); });
            return;
        }
        chain(runs);
    });
    run->start();
}

void Window::handOver(const QString &prompt, bool askFirst)
{
    if (askFirst && status_.otherRole == QLatin1String("previous")) {
        QScopedPointer<ConfirmDialog> question(reuseQuestion());
        if (question->exec() != QDialog::Accepted)
            return;
    }
    work(client_.agent(prompt), tr("Preparing the next system"),
         tr("The running system is not touched. You can keep working."), [this](bool) { showMain(); });
}

void Window::fixWithAgent(const QString &prompt, const QString &title)
{
    work(client_.agent(prompt), title, tr("It works in the next system only. The running system is not touched."),
         [this](bool ok) {
             if (ok && !status_.proposalPath.isEmpty()) {
                 proposed_->show(status_);
                 stack_->setCurrentWidget(proposed_);
             } else {
                 showMain();
             }
         });
}

void Window::addUpdate(bool askFirst)
{
    if (askFirst && status_.otherRole == QLatin1String("previous")) {
        QScopedPointer<ConfirmDialog> question(reuseQuestion());
        if (question->exec() != QDialog::Accepted)
            return;
    }
    work(client_.change({"update", "apply"}), tr("Adding the update to the next system"),
         tr("The running system is not touched. You can keep working."), [this](bool) {
             if (!status_.clashes.isEmpty())
                 showClash();
             else
                 showMain();
         });
}

void Window::finish()
{
    work(client_.change({"finalize"}), tr("Finishing the next system"),
         tr("It is checked and closed, so it can be started."), [this](bool) { showMain(); });
}

void Window::recover()
{
    work(client_.change({"recover"}), tr("Continuing where it stopped"),
         tr("recab looks at what is really there and carries on."), [this](bool) { showMain(); });
}

void Window::restart()
{
    // When the restart is set already, only the restart is left.
    if (status_.state == QLatin1String("trial_pending"))
        chain({client_.reboot()});
    else
        chain({client_.change({"set-next"}), client_.reboot()});
}

void Window::restartLater()
{
    chain({client_.change({"set-next"})});
}

// recab's commit of the running system takes a set restart back.
void Window::cancelRestart()
{
    chain({client_.change({"commit"})});
}

void Window::keep()
{
    work(client_.change({"commit"}), tr("Keeping this system"),
         tr("It becomes the system that every restart starts."), [this](bool) { showMain(); });
}

void Window::goBack(bool askFirst)
{
    if (askFirst) {
        ConfirmDialog question(
            tr("Go back"), tr("Go back to %1?").arg(status_.otherRelease.isEmpty() ? tr("the system from before")
                                                                              : status_.name(status_.otherRelease)),
            {tr("The computer restarts into the system from before. The system you are running now is kept beside "
                "it, so you can return.")},
            QString(), tr("Restart into it"), this);
        if (question.exec() != QDialog::Accepted)
            return;
    }
    chain({client_.change({"rollback"}), client_.reboot()});
}

void Window::discard(bool askFirst)
{
    if (askFirst) {
        ConfirmDialog question(
            tr("Discard"), tr("Discard the next system?"),
            {tr("Its changes are given up. The running system stays exactly as it is.")},
            tr("The system from before does not come back: its place was already reused."), tr("Discard"), this);
        if (question.exec() != QDialog::Accepted)
            return;
    }
    chain({client_.change({"discard"})});
}

// recab has no single command for this: discarding forgets the next system
// but leaves its files as they are; preparing again copies the running system
// over them. Both together are the reset.
void Window::reset(bool askFirst)
{
    if (askFirst) {
        ConfirmDialog question(
            tr("Reset"), tr("Reset the next system to the running one?"),
            {tr("Every change in the next system is given up."),
             tr("It becomes an exact copy of the running system again. Only what differs is copied.")},
            tr("The running system is not touched. A system from before does not come back."), tr("Reset"), this);
        if (question.exec() != QDialog::Accepted)
            return;
    }
    Run *drop = client_.change({"discard"});
    setEnabled(false);
    ++quiet_;
    connect(drop, &Run::finished, this, [this, drop](bool ok) {
        setEnabled(true);
        --quiet_;
        const QString reason = drop->problem();
        drop->deleteLater();
        if (!ok) {
            QMessageBox::warning(this, tr("System"), reason.isEmpty() ? tr("The next system was not reset.") : reason);
            reload([this] { showMain(); });
            return;
        }
        // Read first, so the copy is asked for as a new preparation. The
        // copy is the long part and gets the work page.
        reload([this] {
            work(client_.change({"prepare"}), tr("Making the next system a copy of the running one"),
                 tr("Only what differs is copied. The running system is not touched."), [this](bool) { showMain(); });
        });
    });
    drop->start();
}

void Window::acceptProposal()
{
    chain({client_.change({"resolve", status_.proposalPath, "--proposal"})});
}

// One question for each file that clashes, then all the answers at once.
void Window::decideByHand()
{
    QList<Run *> runs;
    int number = 0;
    for (const Clash &clash : status_.clashes) {
        QMessageBox question(this);
        question.setWindowTitle(status_.clashes.size() == 1
                                    ? tr("Decide by hand")
                                    : tr("Decide by hand (%1 of %2)").arg(++number).arg(status_.clashes.size()));
        question.setText(tr("Which %1 goes into the next system?").arg(clash.path));
        question.setInformativeText(tr("Mine: %1.\nThe update's: %2.").arg(clash.local, clash.release));
        QPushButton *mine = question.addButton(tr("Keep mine"), QMessageBox::AcceptRole);
        QPushButton *theirs = question.addButton(tr("Take the update's"), QMessageBox::AcceptRole);
        question.addButton(QMessageBox::Cancel);
        question.exec();
        if (question.clickedButton() == mine)
            runs << client_.change({"resolve", clash.path, "--local"});
        else if (question.clickedButton() == theirs)
            runs << client_.change({"resolve", clash.path, "--release"});
        else
            break; // what was decided so far is still carried out
    }
    if (!runs.isEmpty())
        chain(runs);
}

void Window::stopRun()
{
    if (run_)
        run_->stop();
}
