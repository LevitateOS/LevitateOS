#include "work.h"

#include "parts.h"
#include "run.h"
#include "words.h"

#include <QFontDatabase>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QPlainTextEdit>
#include <QProgressBar>
#include <QPushButton>

void WorkPage::begin(const QString &title, const QString &subtitle, Run *run)
{
    run_ = run;
    marks_.clear();
    titles_.clear();
    column_ = fresh();

    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(title));
    top->addWidget(dim(text(subtitle, true)));
    header->addLayout(top, 1);
    auto *side = new QVBoxLayout;
    side->setSpacing(6);
    stop_ = new QPushButton(tr("Stop"));
    connect(stop_, &QPushButton::clicked, this, [this, run] {
        stop_->setEnabled(false);
        run->stop();
    });
    side->addWidget(stop_, 0, Qt::AlignRight);
    stopNote_ = dim(text(tr("What is done is kept; you can continue later")));
    side->addWidget(stopNote_);
    header->addLayout(side);
    column_->addLayout(header);

    auto *panel = new Panel;
    steps_ = new QHBoxLayout;
    steps_->setSpacing(10);
    panel->column()->addLayout(steps_);
    bar_ = new QProgressBar;
    bar_->setRange(0, 0);
    bar_->setTextVisible(false);
    bar_->setFixedHeight(8);
    panel->column()->addWidget(bar_);
    state_ = dim(text(tr("Starting"), true));
    panel->column()->addWidget(state_);
    column_->addWidget(panel);

    column_->addWidget(strong(tr("Output")));
    output_ = new QPlainTextEdit;
    output_->setReadOnly(true);
    output_->setMaximumBlockCount(5000);
    output_->setLineWrapMode(QPlainTextEdit::NoWrap);
    output_->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
    column_->addWidget(output_, 1);

    connect(run, &Run::event, this, [this, run](const QJsonObject &event) {
        if (run == run_)
            onEvent(event);
    });
    connect(run, &Run::output, this, [this, run](const QString &line) {
        if (run == run_)
            output_->appendPlainText(line);
    });
    connect(run, &Run::finished, this, [this, run](bool ok) {
        if (run == run_)
            onFinished(ok);
        run->deleteLater();
    });
    emit activity(title);
}

void WorkPage::onEvent(const QJsonObject &event)
{
    const QString kind = event["event"].toString();
    if (kind == QLatin1String("plan")) {
        int number = 0;
        for (const QJsonValue &value : event["steps"].toArray()) {
            const QJsonArray pair = value.toArray();
            const QString id = pair.at(0).toString();
            auto *stepMark = new StepMark(++number);
            marks_.insert(id, stepMark);
            titles_.insert(id, pair.at(1).toString());
            steps_->addWidget(stepMark);
            QLabel *label = text(pair.at(1).toString());
            label->setMinimumWidth(110);
            steps_->addWidget(label);
        }
        steps_->addStretch();
    } else if (kind == QLatin1String("step")) {
        const QString id = event["step"].toString();
        const QString outcome = event["status"].toString();
        StepMark *stepMark = marks_.value(id);
        if (!stepMark)
            return;
        if (outcome == QLatin1String("running")) {
            stepMark->setState(StepMark::Running);
            bar_->setRange(0, 0);
            state_->setText(titles_.value(id));
            emit activity(titles_.value(id));
        } else if (outcome == QLatin1String("skipped")) {
            stepMark->setState(StepMark::Skipped);
        } else if (outcome == QLatin1String("failed")) {
            stepMark->setState(StepMark::Failed);
        } else {
            stepMark->setState(StepMark::Done);
        }
    } else if (kind == QLatin1String("progress")) {
        const qint64 done = event["done"].toInteger(), total = event["total"].toInteger();
        const bool bytes = event["unit"] == QLatin1String("bytes");
        bar_->setRange(0, int(bytes ? total / 1024 : total));
        bar_->setValue(int(bytes ? done / 1024 : done));
        const QString step = titles_.value(event["step"].toString());
        state_->setText(bytes ? tr("%1: %2 of %3").arg(step, bytesText(done), bytesText(total))
                              : tr("%1: %2 of %3").arg(step).arg(done).arg(total));
    } else if (kind == QLatin1String("say")) {
        state_->setText(event["text"].toString());
        output_->appendPlainText(event["text"].toString());
    } else if (kind == QLatin1String("agent")) {
        output_->appendPlainText(tr("agent: %1").arg(event["summary"].toString()));
    }
}

void WorkPage::onFinished(bool ok)
{
    emit activity(QString());
    const bool stoppedByUser = !stop_->isEnabled();
    const QString reason = run_->failure()["reason"].toString();
    if (ok || stoppedByUser) {
        emit done(ok);
        return;
    }
    bar_->hide();
    stop_->hide();
    stopNote_->hide();
    auto *verdict = new Verdict;
    verdict->setFrameShape(QFrame::NoFrame);
    verdict->column()->addWidget(strong(tr("It stopped")));
    verdict->column()->addWidget(
        text(reason.isEmpty() ? tr("The output below ends with the reason. The running system was not touched.") : reason,
             true));
    auto *buttons = new QHBoxLayout;
    auto *leave = new QPushButton(tr("Back"));
    connect(leave, &QPushButton::clicked, this, &WorkPage::back);
    buttons->addWidget(leave);
    buttons->addStretch();
    verdict->column()->addLayout(buttons);
    column_->insertWidget(1, verdict);
    emit done(false);
}
