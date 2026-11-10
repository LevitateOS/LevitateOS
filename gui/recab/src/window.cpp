#include "window.h"

#include "pages.h"

#include <QLabel>
#include <QMessageBox>
#include <QPushButton>
#include <QStackedWidget>
#include <QStatusBar>

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
    connect(system_, &SystemPage::discard, this, [this] { discard(); });
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
        reload();
        auto then = then_;
        then_ = nullptr;
        if (then)
            then(ok);
    });

    connect(clash_, &ClashPage::later, this, &Window::showMain);
    connect(clash_, &ClashPage::byHand, this, &Window::decideByHand);
    connect(clash_, &ClashPage::fix, this,
            [this](const QString &prompt) { fixWithAgent(prompt, tr("The agent is settling the clash")); });

    connect(proposed_, &ProposedPage::accept, this, &Window::acceptProposal);
    connect(proposed_, &ProposedPage::askAgain, this, &Window::showClash);
    connect(proposed_, &ProposedPage::byHand, this, &Window::decideByHand);

    if (reload())
        showMain();
}

bool Window::reload()
{
    QString problem;
    if (!client_.load(&status_, &problem)) {
        system_->showProblem(problem);
        stack_->setCurrentWidget(system_);
        return false;
    }
    return true;
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
    const QString before = status_.name(status_.otherRelease);
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
        if (reload())
            showMain();
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
            if (reload())
                showMain();
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
    chain({client_.change({"set-next"}), client_.reboot()});
}

void Window::goBack(bool askFirst)
{
    if (askFirst) {
        ConfirmDialog question(
            tr("Go back"), tr("Go back to %1?").arg(status_.name(status_.otherRelease)),
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

void Window::acceptProposal()
{
    chain({client_.change({"resolve", status_.proposalPath, "--proposal"})});
}

void Window::decideByHand()
{
    if (status_.clashes.isEmpty())
        return;
    const Clash clash = status_.clashes.first();
    QMessageBox question(this);
    question.setWindowTitle(tr("Decide by hand"));
    question.setText(tr("Which %1 goes into the next system?").arg(clash.path));
    question.setInformativeText(tr("Mine: %1.\nThe update's: %2.").arg(clash.local, clash.release));
    QPushButton *mine = question.addButton(tr("Keep mine"), QMessageBox::AcceptRole);
    QPushButton *theirs = question.addButton(tr("Take the update's"), QMessageBox::AcceptRole);
    question.addButton(QMessageBox::Cancel);
    question.exec();
    if (question.clickedButton() == mine)
        chain({client_.change({"resolve", clash.path, "--local"})});
    else if (question.clickedButton() == theirs)
        chain({client_.change({"resolve", clash.path, "--release"})});
}

void Window::stopRun()
{
    if (run_)
        run_->stop();
}
