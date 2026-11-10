#include "page.h"

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
