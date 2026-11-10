#include "page.h"

#include "parts.h"

#include <QProgressBar>
#include <QVBoxLayout>

Page::Page(QWidget *parent)
    : QWidget(parent)
    , outer_(new QVBoxLayout(this))
{
    outer_->setContentsMargins(0, 0, 0, 0);
}

QVBoxLayout *Page::fresh()
{
    if (body_) {
        // A click inside the old body may still be on the stack.
        body_->hide();
        outer_->removeWidget(body_);
        body_->deleteLater();
    }
    body_ = new QWidget(this);
    outer_->addWidget(body_);
    auto *column = new QVBoxLayout(body_);
    column->setContentsMargins(28, 20, 28, 16);
    column->setSpacing(14);
    return column;
}

void Page::showLoading(const QString &title, const QString &words)
{
    QVBoxLayout *column = fresh();
    column->addWidget(heading(title));
    column->addWidget(dim(text(words, true)));
    auto *bar = new QProgressBar;
    bar->setRange(0, 0);
    bar->setTextVisible(false);
    bar->setFixedHeight(8);
    bar->setMaximumWidth(360);
    column->addWidget(bar);
    column->addStretch();
}
