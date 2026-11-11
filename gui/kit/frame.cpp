#include "frame.h"

#include "activity.h"
#include "firstrun.h"
#include "help.h"
#include "look.h"
#include "parts.h"
#include "run.h"
#include "settings.h"

#include <QAbstractButton>
#include <QFileInfo>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
#include <QCloseEvent>
#include <QKeyEvent>
#include <QLabel>
#include <QPainter>
#include <QProcess>
#include <QPushButton>
#include <QStackedWidget>
#include <QStyle>
#include <QStyleOptionFocusRect>
#include <QTimer>
#include <QVBoxLayout>

namespace {

// The paper the rail and the strip sit on: the window colour pulled a little
// towards the text colour, so it is a shade lighter on a dark desktop and a
// shade darker on a light one, whatever the palette is.
QColor shade(const QPalette &palette, qreal by)
{
    const QColor ground = palette.color(QPalette::Window), ink = palette.color(QPalette::WindowText);
    return QColor::fromRgbF(ground.redF() + (ink.redF() - ground.redF()) * by,
                            ground.greenF() + (ink.greenF() - ground.greenF()) * by,
                            ground.blueF() + (ink.blueF() - ground.blueF()) * by);
}

// The pages the frame owns; the app's `follow` leaves them showing. "firstrun" has
// no rail entry: it is marked while the page before the app's own shows.
bool sharedKey(const QString &key)
{
    return key == QLatin1String("activity") || key == QLatin1String("settings") || key == QLatin1String("help")
        || key == QLatin1String("firstrun");
}

} // namespace

// The rail has the three apps and the entries in it; each is one of these.
// Up and Down move between the entries, Tab and Shift+Tab as everywhere else.
class RailButton : public QAbstractButton {
    Q_OBJECT
public:
    enum Kind { AppSwitch, Entry };
    RailButton(Kind kind, const QString &title, QWidget *parent = nullptr)
        : QAbstractButton(parent)
        , kind_(kind)
    {
        setText(title);
        setCheckable(kind == Entry);
        setFocusPolicy(Qt::StrongFocus);
        setCursor(Qt::PointingHandCursor);
        setAttribute(Qt::WA_Hover);
        setAccessibleName(title);
        setFixedHeight(kind == AppSwitch ? 28 : 32);
        setSizePolicy(QSizePolicy::Expanding, QSizePolicy::Fixed);
    }
    // The app this window is: marked, and it does nothing when chosen.
    void setCurrent(bool current)
    {
        current_ = current;
        update();
    }
    void setBadge(int count, bool warn)
    {
        count_ = count;
        warn_ = warn;
        setAccessibleName(count > 0 ? tr("%1, %2").arg(text()).arg(count) : text());
        update();
    }
    int badge() const { return count_; }
    bool warns() const { return warn_; }
    QSize sizeHint() const override { return {100, height()}; }

signals:
    void stepped(int by);

protected:
    void keyPressEvent(QKeyEvent *event) override
    {
        if (event->key() == Qt::Key_Down || event->key() == Qt::Key_Up) {
            emit stepped(event->key() == Qt::Key_Down ? 1 : -1);
            return;
        }
        if (event->key() == Qt::Key_Return || event->key() == Qt::Key_Enter) {
            click();
            return;
        }
        QAbstractButton::keyPressEvent(event);
    }
    void paintEvent(QPaintEvent *) override
    {
        QPainter painter(this);
        painter.setRenderHint(QPainter::Antialiasing);
        const QPalette &p = palette();
        const QRectF box = QRectF(rect()).adjusted(0.5, 0.5, -0.5, -0.5);
        QColor ink = p.color(QPalette::WindowText);
        if (kind_ == AppSwitch) {
            // The current app is a raised button, the others are words.
            if (current_) {
                painter.setPen(QPen(p.color(QPalette::Mid).lighter(150), 1));
                painter.setBrush(p.color(QPalette::Button));
                painter.drawRoundedRect(box, 5, 5);
            } else if (underMouse()) {
                QColor tint = ink;
                tint.setAlphaF(0.07f);
                painter.setPen(Qt::NoPen);
                painter.setBrush(tint);
                painter.drawRoundedRect(box, 5, 5);
            }
            QFont font = this->font();
            font.setPointSizeF(font.pointSizeF() * 0.95);
            font.setBold(current_);
            painter.setFont(font);
            if (!current_)
                ink.setAlphaF(0.7f);
            painter.setPen(ink);
            painter.drawText(rect(), Qt::AlignCenter, text());
        } else {
            if (isChecked()) {
                QColor tint = p.color(QPalette::Highlight);
                tint.setAlphaF(0.16f);
                painter.setBrush(tint);
                tint.setAlphaF(0.75f);
                painter.setPen(QPen(tint, 1));
                painter.drawRoundedRect(box, 5, 5);
            } else if (underMouse()) {
                QColor tint = ink;
                tint.setAlphaF(0.07f);
                painter.setPen(Qt::NoPen);
                painter.setBrush(tint);
                painter.drawRoundedRect(box, 5, 5);
            }
            painter.setPen(ink);
            painter.setFont(font());
            int right = width() - 10;
            if (count_ > 0) {
                const QString number = QString::number(count_);
                const int wide = qMax(20, fontMetrics().horizontalAdvance(number) + 12);
                const QRect pill(width() - 10 - wide, (height() - 18) / 2, wide, 18);
                painter.setPen(Qt::NoPen);
                const QColor fill = warn_ ? moodColour(p, Mood::Warn) : p.color(QPalette::Highlight);
                painter.setBrush(fill);
                painter.drawRoundedRect(pill, 9, 9);
                // Dark ink on a light fill, light ink on a dark one: amber is light on the dark look and brown on the light one.
                QFont bold = font();
                bold.setPointSizeF(bold.pointSizeF() * 0.93);
                bold.setBold(true);
                painter.setFont(bold);
                painter.setPen(!warn_ ? p.color(QPalette::HighlightedText)
                                      : fill.lightness() > 140 ? QColor(0x1c, 0x13, 0x00) : QColor(0xff, 0xff, 0xff));
                painter.drawText(pill, Qt::AlignCenter, number);
                right = pill.left() - 8;
            }
            painter.setFont(font());
            painter.setPen(ink);
            painter.drawText(QRect(10, 0, right - 10, height()), Qt::AlignVCenter | Qt::AlignLeft,
                             fontMetrics().elidedText(text(), Qt::ElideRight, right - 10));
        }
        if (hasFocus()) {
            QStyleOptionFocusRect focus;
            focus.initFrom(this);
            focus.rect = rect().adjusted(1, 1, -1, -1);
            focus.backgroundColor = palette().color(QPalette::Window);
            style()->drawPrimitive(QStyle::PE_FrameFocusRect, &focus, &painter, this);
        }
    }

private:
    Kind kind_;
    bool current_ = false, warn_ = false;
    int count_ = 0;
};

// "The next system holds 2 changes.", what they are, and the button.
class Strip : public QWidget {
    Q_OBJECT
public:
    explicit Strip(QWidget *parent = nullptr)
        : QWidget(parent)
    {
        auto *row = new QHBoxLayout(this);
        row->setContentsMargins(18, 8, 16, 8);
        row->setSpacing(12);
        sentence_ = strong(QString());
        detail_ = new ElidedLabel(QString());
        QPalette p = detail_->palette();
        p.setColor(QPalette::WindowText, moodColour(p, Mood::Plain));
        detail_->setPalette(p);
        button_ = new QPushButton(tr("Finish and restart…"));
        button_->setAccessibleDescription(tr("Goes to where the next system is finished"));
        row->addWidget(sentence_);
        row->addWidget(detail_, 1);
        row->addWidget(button_);
        connect(button_, &QPushButton::clicked, this, &Strip::pressed);
    }
    void say(const QString &sentence, const QString &detail)
    {
        sentence_->setText(sentence);
        detail_->setText(detail);
        detail_->setToolTip(detail);
    }
    QString sentence() const { return sentence_->text(); }

signals:
    void pressed();

protected:
    void paintEvent(QPaintEvent *) override
    {
        QPainter painter(this);
        painter.fillRect(rect(), palette().color(QPalette::Base));
        QColor line = palette().color(QPalette::WindowText);
        line.setAlphaF(0.13f);
        painter.fillRect(QRect(0, 0, width(), 1), line);
        // Amber on the left edge: something here is not finished.
        painter.fillRect(QRect(0, 1, 3, height() - 1), moodColour(palette(), Mood::Warn));
    }

private:
    QLabel *sentence_;
    ElidedLabel *detail_;
    QPushButton *button_;
};

namespace {

// "htop, installed" is a change's title; the strip names what changed.
QString nameOf(QString title)
{
    static const QStringList tails{", by the agent", ", installed", ", updated", ", removed", ", edited"};
    bool cut = true;
    while (cut) {
        cut = false;
        for (const QString &tail : tails) {
            if (title.endsWith(tail)) {
                title.chop(tail.size());
                cut = true;
            }
        }
    }
    return title;
}

// A line down the side of the rail.
class Divider : public QWidget {
public:
    Divider() { setFixedWidth(1); }
protected:
    void paintEvent(QPaintEvent *) override
    {
        QColor colour = palette().color(QPalette::WindowText);
        colour.setAlphaF(0.13f);
        QPainter(this).fillRect(rect(), colour);
    }
};

struct AppInfo {
    const char *name, *program, *variable;
};
const AppInfo apps[] = {{"Recipe", "recipe-gui", "LEVITATE_APP_RECIPE"},
                        {"System", "recab-gui", "LEVITATE_APP_SYSTEM"},
                        {"Disk", "disk-gui", "LEVITATE_APP_DISK"}};

QWidget *rail(QWidget *parent)
{
    auto *widget = new QWidget(parent);
    widget->setFixedWidth(196);
    widget->setAutoFillBackground(true);
    QPalette p = widget->palette();
    p.setColor(QPalette::Window, shade(p, 0.07));
    widget->setPalette(p);
    return widget;
}

} // namespace

Frame::Frame(App app, QWidget *parent)
    : QWidget(parent)
    , app_(app)
{
    auto *outer = new QVBoxLayout(this);
    outer->setContentsMargins(0, 0, 0, 0);
    outer->setSpacing(0);
    auto *shell = new QHBoxLayout;
    shell->setSpacing(0);

    // ---- the rail
    QWidget *left = rail(this);
    auto *railColumn = new QVBoxLayout(left);
    railColumn->setContentsMargins(8, 8, 8, 8);
    railColumn->setSpacing(2);
    left->setAccessibleName(tr("%1 sections").arg(QString::fromLatin1(apps[app].name)));

    appsRow_ = new QWidget;
    auto *three = new QHBoxLayout(appsRow_);
    three->setContentsMargins(0, 0, 0, 0);
    three->setSpacing(4);
    for (int at = 0; at < 3; ++at) {
        auto *button = new RailButton(RailButton::AppSwitch, tr(apps[at].name));
        button->setCurrent(at == app);
        button->setAccessibleDescription(at == app ? tr("This window") : tr("Opens %1").arg(tr(apps[at].name)));
        connect(button, &QAbstractButton::clicked, this, [this, at] { startApp(App(at)); });
        three->addWidget(button);
        order_ << button;
        connect(button, &RailButton::stepped, this, [this, button](int by) { step(button, by); });
    }
    railColumn->addWidget(appsRow_);
    railColumn->addSpacing(4);
    railColumn->addWidget(rule());
    railColumn->addSpacing(4);

    appEntries_ = new QWidget;
    auto *own = new QVBoxLayout(appEntries_);
    own->setContentsMargins(0, 0, 0, 0);
    own->setSpacing(2);
    railColumn->addWidget(appEntries_);
    railColumn->addStretch();
    // The shared entries, below the gap.
    auto *activity = new RailButton(RailButton::Entry, tr("Activity"));
    activity->setAccessibleDescription(tr("What the agent was asked, from all three apps"));
    entries_.insert(activityKey(), activity);
    order_ << activity;
    railColumn->addWidget(activity);
    connect(activity, &QAbstractButton::clicked, this, [this] { choose(activityKey()); });
    connect(activity, &RailButton::stepped, this, [this, activity](int by) { step(activity, by); });
    auto *settings = new RailButton(RailButton::Entry, tr("Settings"));
    settings->setAccessibleDescription(tr("The agent and the windows, the same in all three apps"));
    entries_.insert(settingsKey(), settings);
    order_ << settings;
    railColumn->addWidget(settings);
    connect(settings, &QAbstractButton::clicked, this, [this] { choose(settingsKey()); });
    connect(settings, &RailButton::stepped, this, [this, settings](int by) { step(settings, by); });
    auto *help = new RailButton(RailButton::Entry, tr("Help"));
    help->setAccessibleDescription(tr("How %1 works and where to look when something is wrong").arg(tr(apps[app].name)));
    entries_.insert(helpKey(), help);
    order_ << help;
    railColumn->addWidget(help);
    connect(help, &QAbstractButton::clicked, this, [this] { choose(helpKey()); });
    connect(help, &RailButton::stepped, this, [this, help](int by) { step(help, by); });
    shell->addWidget(left);
    shell->addWidget(new Divider);

    // ---- the middle
    auto *middle = new QVBoxLayout;
    middle->setSpacing(0);
    stack_ = new QStackedWidget;
    activity_ = new ActivityPage(QString::fromLatin1(apps[app].name));
    stack_->addWidget(new QWidget); // the app's content takes this place
    stack_->addWidget(activity_);
    settings_ = new SettingsPage(QString::fromLatin1(apps[app].name));
    stack_->addWidget(settings_);
    help_ = new HelpPage(app);
    stack_->addWidget(help_);
    firstRun_ = new FirstRunPage(app);
    stack_->addWidget(firstRun_);
    middle->addWidget(stack_, 1);
    strip_ = new Strip;
    strip_->hide();
    connect(strip_, &Strip::pressed, this, &Frame::finishRequested);
    middle->addWidget(strip_);
    shell->addLayout(middle, 1);
    outer->addLayout(shell, 1);

    // ---- the status line
    outer->addWidget(rule());
    auto *line = new QWidget;
    line->setFixedHeight(28);
    auto *status = new QHBoxLayout(line);
    status->setContentsMargins(12, 0, 12, 0);
    status->setSpacing(10);
    status_ = new ElidedLabel(QString());
    {
        QPalette p = status_->palette();
        p.setColor(QPalette::WindowText, moodColour(p, Mood::Plain));
        status_->setPalette(p);
    }
    status_->setSizePolicy(QSizePolicy::Preferred, QSizePolicy::Preferred);
    statusExtras_ = new QHBoxLayout;
    statusExtras_->setSpacing(6);
    agent_ = dim(new QLabel);
    status->addWidget(status_, 0);
    status->addLayout(statusExtras_);
    status->addStretch();
    status->addWidget(agent_);
    outer->addWidget(line);

    flashTimer_ = new QTimer(this);
    flashTimer_->setSingleShot(true);
    connect(flashTimer_, &QTimer::timeout, this, [this] {
        flashWords_.clear();
        status_->setText(statusWords_);
        status_->setToolTip(statusWords_);
    });

    connect(activity_, &ActivityPage::counts, this, [this](int active, bool waits) {
        setBadge(activityKey(), active, waits);
    });
    connect(activity_, &ActivityPage::goToRun, this, &Frame::goToRun);
    // A change in Settings shows in the status line at once; "Try it" is a run for Activity.
    connect(settings_, &SettingsPage::changed, this, &Frame::readAgentStatus);
    connect(settings_, &SettingsPage::ran, activity_, &ActivityPage::read);
    connect(help_, &HelpPage::go, this, &Frame::helpGo);
    connect(help_, &HelpPage::startApp, this, &Frame::startApp);
    connect(help_, &HelpPage::firstPageAgain, this, &Frame::showFirstRun);
    // "Go on" is the person's step to the app; the rail's entries leave the page as well.
    connect(firstRun_, &FirstRunPage::goOn, this, &Frame::leaveShared);
    connect(firstRun_, &FirstRunPage::openSettings, this, [this] { choose(settingsKey()); });
    connect(firstRun_, &FirstRunPage::openHelp, this, [this] { choose(helpKey()); });
    // The rail's badge is true from the start, not only once Activity was opened.
    QTimer::singleShot(0, activity_, &ActivityPage::read);
}

void Frame::setContent(QWidget *content)
{
    QWidget *old = stack_->widget(0);
    stack_->removeWidget(old);
    old->deleteLater();
    stack_->insertWidget(0, content);
    if (!sharedKey(selected_))
        stack_->setCurrentIndex(0);
}

void Frame::addEntry(const QString &key, const QString &title)
{
    auto *button = new RailButton(RailButton::Entry, title);
    entries_.insert(key, button);
    // Before the shared entries (Activity, Settings, Help), which are last in the order.
    order_.insert(order_.size() - 3, button);
    static_cast<QVBoxLayout *>(appEntries_->layout())->addWidget(button);
    connect(button, &QAbstractButton::clicked, this, [this, key] { choose(key); });
    connect(button, &RailButton::stepped, this, [this, button](int by) { step(button, by); });
    if (selected_.isEmpty())
        mark(key);
}

void Frame::setBadge(const QString &key, int count, bool warn)
{
    if (RailButton *button = entries_.value(key))
        button->setBadge(count, warn);
}

int Frame::badge(const QString &key) const
{
    RailButton *button = entries_.value(key);
    return button ? button->badge() : 0;
}

bool Frame::badgeWarns(const QString &key) const
{
    RailButton *button = entries_.value(key);
    return button && button->warns();
}

void Frame::mark(const QString &key)
{
    selected_ = key;
    for (auto at = entries_.begin(); at != entries_.end(); ++at)
        at.value()->setChecked(at.key() == key);
}

void Frame::select(const QString &key)
{
    if (!entries_.contains(key))
        return;
    followed_ = key;
    mark(key);
    stack_->setCurrentIndex(indexOf(key));
}

// Where a page is in the stack: the app's content first, then the frame's own.
int Frame::indexOf(const QString &key) const
{
    return key == activityKey() ? 1 : key == settingsKey() ? 2 : key == helpKey() ? 3 : 0;
}

void Frame::follow(const QString &key)
{
    if (!entries_.contains(key) || sharedKey(key))
        return;
    followed_ = key;
    if (!sharedKey(selected_))
        mark(key);
}

bool Frame::activityShown() const
{
    return selected_ == activityKey();
}

bool Frame::settingsShown() const
{
    return selected_ == settingsKey();
}

bool Frame::helpShown() const
{
    return selected_ == helpKey();
}

bool Frame::firstRunShown() const
{
    return selected_ == QLatin1String("firstrun");
}

void Frame::addHelpTarget(const QString &key)
{
    if (!helpTargets_.contains(key))
        helpTargets_ << key;
}

// The page before the app's own: nothing in the rail is marked, and the page asks
// the helper about the agent each time it is shown.
void Frame::showFirstRun()
{
    mark(QStringLiteral("firstrun"));
    stack_->setCurrentIndex(4);
    firstRun_->read();
    FirstRunPage::markSeen(app_);
}

// Back to the app's own page, the one it is on: from Help, from the first page.
void Frame::leaveShared()
{
    mark(followed_);
    stack_->setCurrentIndex(0);
}

// A link of Help: a rail entry, Activity and Settings are chosen as a click would;
// a page the app offered is shown by the app, once Help has been left.
void Frame::helpGo(const QString &target)
{
    if (target == activityKey() || target == settingsKey() || entries_.contains(target)) {
        choose(target);
    } else if (helpTargets_.contains(target)) {
        leaveShared();
        emit helpLink(target);
    }
}

// The person chose an entry: show what it stands for and say so.
void Frame::choose(const QString &key)
{
    // Away from the page of a run that works: it carries on, and Activity has it.
    if (!ownRun_.isEmpty() && !ownNoted_ && key != activityKey()) {
        ownNoted_ = true;
        flash(tr("The agent carries on with its run. You can find it in Activity."));
    }
    if (key == activityKey()) {
        mark(key);
        stack_->setCurrentIndex(1);
        activity_->read();
    } else if (key == settingsKey()) {
        mark(key);
        stack_->setCurrentIndex(2);
        settings_->read();
        return; // the frame's own page: the app has nothing to switch to
    } else if (key == helpKey()) {
        mark(key);
        // Drawn for the pages the app has now: no link leads to a page that is not there.
        help_->draw(entries_.keys() + helpTargets_);
        stack_->setCurrentIndex(indexOf(key));
        return; // the frame's own page as well
    } else {
        select(key);
    }
    emit chosen(key);
}

void Frame::step(RailButton *from, int by)
{
    const int at = order_.indexOf(from);
    if (at < 0)
        return;
    // The three apps are one stop each; Up and Down stay among the entries.
    QList<RailButton *> entries;
    for (RailButton *button : order_)
        if (button->parent() != appsRow_)
            entries << button;
    const int index = entries.indexOf(from);
    if (index < 0)
        return;
    const int next = qBound(0, index + by, int(entries.size()) - 1);
    entries[next]->setFocus(Qt::TabFocusReason);
}

// ---- The strip

void Frame::setNextSystem(const QStringList &titles)
{
    if (titles.isEmpty()) {
        strip_->hide();
        strip_->say(QString(), QString());
        return;
    }
    QStringList names;
    for (const QString &title : titles)
        names << nameOf(title);
    QString which;
    if (names.size() == 1)
        which = names.first();
    else if (names.size() == 2)
        which = tr("%1 and %2").arg(names[0], names[1]);
    else
        which = tr("%1, %2 and %3 more").arg(names[0], names[1]).arg(names.size() - 2);
    strip_->say(titles.size() == 1 ? tr("The next system holds 1 change.")
                                   : tr("The next system holds %1 changes.").arg(titles.size()),
                names.size() == 1 ? tr("%1 takes effect after a restart.").arg(which)
                                  : tr("%1 take effect after a restart.").arg(which));
    // Shown unless the app keeps it away; `setStripAllowed` decides that.
    strip_->setVisible(stripAllowed_);
}

void Frame::setStripAllowed(bool allowed)
{
    stripAllowed_ = allowed;
    strip_->setVisible(allowed && !strip_->sentence().isEmpty());
}

bool Frame::stripShown() const
{
    return !strip_->isHidden();
}

QString Frame::stripText() const
{
    return strip_->sentence();
}

// ---- The status line

void Frame::setStatus(const QString &words)
{
    statusWords_ = words;
    if (flashWords_.isEmpty()) {
        status_->setText(words);
        status_->setToolTip(words);
    }
}

QString Frame::statusText() const
{
    return flashWords_.isEmpty() ? statusWords_ : flashWords_;
}

void Frame::addStatusWidget(QWidget *widget)
{
    statusExtras_->addWidget(widget);
}

void Frame::flash(const QString &words)
{
    flashWords_ = words;
    const QString shown = words.isEmpty() ? statusWords_ : words;
    status_->setText(shown);
    status_->setToolTip(shown);
    if (words.isEmpty())
        flashTimer_->stop();
    else
        flashTimer_->start(8000);
}

void Frame::startApp(App app)
{
    if (app == app_)
        return;
    // A picture run does not start the person's real apps: one it was not told of is not started.
    if (pictureMode() && !qEnvironmentVariableIsSet(apps[app].variable)) {
        flash(tr("%1 is not started in a picture run: its program is not named in the environment (%2).")
                  .arg(tr(apps[app].name), QString::fromLatin1(apps[app].variable)));
        return;
    }
    const QString program = qEnvironmentVariable(apps[app].variable, QString::fromLatin1(apps[app].program));
    if (program.isEmpty() || !QProcess::startDetached(program, {}))
        flash(tr("%1 could not be started: %2 was not found or would not run.")
                  .arg(tr(apps[app].name), program.isEmpty() ? QString::fromLatin1(apps[app].program) : program));
}

QStringList Frame::entryTitles() const
{
    QStringList titles;
    for (RailButton *button : order_)
        if (button->parent() != appsRow_)
            titles << button->text();
    return titles;
}

void Frame::showAgent(const QString &words)
{
    agentName_ = words;
    agent_->setText(agentNote_.isEmpty() ? words : agentNote_);
}

void Frame::setAgentNote(const QString &words)
{
    agentNote_ = words;
    agent_->setText(words.isEmpty() ? agentName_ : words);
}

QString Frame::agentText() const
{
    return agent_->text();
}

void Frame::setAgent(const QString &program)
{
    agentProgram_ = program;
    readAgentStatus();
}

// `levitate-agent --status`, read without holding the window up.
void Frame::readAgentStatus()
{
    const QString name = QFileInfo(agentProgram_).fileName();
    if (name != QLatin1String("levitate-agent")) {
        // A stand-in, or an agent that needs no asking: its file name, as it always was.
        showAgent(tr("Agent: %1").arg(name));
        return;
    }
    if (asking_) {
        askAgain_ = true; // what it reports may have changed since the question went out
        return;
    }
    if (agentName_.isEmpty())
        showAgent(tr("Agent: asking…"));
    asking_ = new Run(agentProgram_, {"--status"}, this);
    connect(asking_, &Run::finished, this, [this, name](bool ok) {
        Run *run = asking_;
        asking_ = nullptr;
        run->deleteLater();
        if (askAgain_) {
            askAgain_ = false;
            readAgentStatus();
            return;
        }
        const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
        if (!ok || !answer.contains("agents")) {
            showAgent(tr("Agent: %1 could not be asked").arg(name));
            return;
        }
        QJsonObject chosen;
        for (const QJsonValue &agent : answer["agents"].toArray())
            if (agent["id"].toString() == answer["chosen"].toString())
                chosen = agent.toObject();
        const QString who = chosen["name"].toString(answer["chosen"].toString());
        if (!chosen["installed"].toBool()) {
            showAgent(tr("Agent: %1 is not installed").arg(who));
            return;
        }
        const QJsonValue signedIn = chosen["signed_in"];
        showAgent(signedIn.isBool() ? tr("Agent: %1, %2").arg(who, signedIn.toBool() ? tr("signed in") : tr("not signed in"))
                                    : tr("Agent: %1").arg(who));
    });
    asking_->start();
}

// ---- Activity

void Frame::setOwnRun(Run *run)
{
    if (!run || !run->isAgent())
        return;
    ownRuns_ << run;
    if (!run->id().isEmpty())
        setOwnRun(run->id());
    else
        connect(run, &Run::idKnown, this, [this, run](const QString &id) {
            // Not for a run that ended before the helper named it.
            if (run->exitCode() < 0)
                setOwnRun(id);
        });
}

void Frame::setOwnRun(const QString &id)
{
    if (id != ownRun_)
        ownNoted_ = false;
    ownRun_ = id;
    activity_->setOwnRun(id);
    if (id.isEmpty())
        activity_->read();
}

// A window that closes, or the person leaving for another page: the run does
// not end, and Activity is where it is found.
void Frame::letGoOfRuns()
{
    // The runs this window follows and the ones its pages started (Settings' "Try it"),
    // and what Activity attached to: all of them let go of, none stopped. Only a run
    // the helper has named (so that Activity has it) is worth saying.
    QList<Run *> all;
    for (const QPointer<Run> &run : std::as_const(ownRuns_))
        if (run)
            all << run;
    for (Run *run : findChildren<Run *>())
        if (!all.contains(run))
            all << run;
    int carried = 0;
    for (Run *run : all) {
        if (run->released() || !run->running())
            continue;
        const bool said = run->isAgent() && !run->id().isEmpty();
        if (run->isAgent() ? !run->id().isEmpty() : run->isAttach()) {
            run->release();
            carried += said;
        }
    }
    if (carried > 0)
        flash(tr("The agent's run carries on without this window. You can find it in Activity."));
}

void Frame::watchWindow()
{
    QWidget *top = window();
    if (!top || top == this || top == watched_)
        return;
    top->installEventFilter(this);
    watched_ = top;
}

bool Frame::event(QEvent *event)
{
    if (event->type() == QEvent::ParentChange)
        watchWindow();
    return QWidget::event(event);
}

void Frame::showEvent(QShowEvent *event)
{
    watchWindow();
    QWidget::showEvent(event);
    // The first time this person opens this app: the page before the app's own.
    // The app has been reading since it was made; nothing here waits for it.
    if (!firstRunChecked_) {
        firstRunChecked_ = true;
        if (FirstRunPage::wanted(app_))
            showFirstRun();
    }
}

bool Frame::eventFilter(QObject *watched, QEvent *event)
{
    if (watched == watched_ && event->type() == QEvent::Close) {
        // This filter sees the close event before the window's own handler, which may
        // refuse ("close anyway?" in Recipe). So the runs are let go of only once the
        // event has been dealt with and the window really is gone.
        QPointer<QWidget> top = watched_;
        QTimer::singleShot(0, this, [this, top] {
            if (!top || !top->isVisible())
                letGoOfRuns();
        });
    }
    return false;
}

void Frame::closeEvent(QCloseEvent *event)
{
    letGoOfRuns();
    QWidget::closeEvent(event);
}

void Frame::readAgain()
{
    activity_->read();
    if (settings_->started())
        settings_->read();
    if (firstRunShown())
        firstRun_->read();
    readAgentStatus();
}

#include "frame.moc"
