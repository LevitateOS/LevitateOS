#include "refresh.h"

#include <QAction>
#include <QEvent>
#include <QWidget>

Refresher::Refresher(QWidget *window, int afterSeconds)
    : QObject(window)
    , window_(window)
    , after_(afterSeconds)
{
    auto *action = new QAction(tr("Read again"), window);
    action->setShortcuts({QKeySequence(Qt::Key_F5), QKeySequence(Qt::CTRL | Qt::Key_R)});
    window->addAction(action);
    connect(action, &QAction::triggered, this, [this] { emit asked(true); });
    window->installEventFilter(this);
    age_.start();
}

bool Refresher::eventFilter(QObject *watched, QEvent *event)
{
    if (watched == window_ && event->type() == QEvent::WindowActivate && after_ > 0
        && age_.elapsed() >= qint64(after_) * 1000)
        emit asked(false);
    return QObject::eventFilter(watched, event);
}
