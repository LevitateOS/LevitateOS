#include "settings.h"

#include "parts.h"
#include "run.h"

#include <QAbstractButton>
#include <QButtonGroup>
#include <QComboBox>
#include <QEvent>
#include <QFileDialog>
#include <QFileInfo>
#include <QFontDatabase>
#include <QGridLayout>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonDocument>
#include <QPushButton>
#include <QRadioButton>
#include <QScrollArea>
#include <QScrollBar>
#include <QSettings>
#include <QStyle>
#include <QVBoxLayout>

namespace {

const char *hello = "Say hello in one sentence. Do not run any command and do not change any file.";

void tint(QLabel *label, Mood mood)
{
    QPalette p = label->palette();
    p.setColor(QPalette::WindowText, moodColour(p, mood));
    label->setPalette(p);
}

QLabel *mood(const QString &words, Mood mood, bool wrap = true)
{
    QLabel *label = text(words, wrap);
    tint(label, mood);
    return label;
}

// Rows sit on the window's own colour, so that a radio button (drawn on the
// view colour) can be told from what is behind it.
Panel *rowsBox()
{
    auto *panel = new Panel;
    panel->setBackgroundRole(QPalette::Window);
    panel->column()->setSpacing(0);
    panel->column()->setContentsMargins(0, 0, 0, 0);
    return panel;
}

// Whether a widget of the page is on it now: the old body is hidden by the
// page while it waits to be deleted, and a new widget is not yet "shown" until
// the event loop has run once, which is no reason to leave it out.
bool onPage(const QWidget *widget, const QWidget *page)
{
    for (const QWidget *w = widget; w && w != page; w = w->parentWidget())
        if (w->isHidden() && w->testAttribute(Qt::WA_WState_ExplicitShowHide))
            return false;
    return true;
}

// A section: its title and a column to fill.
QVBoxLayout *section(QVBoxLayout *page, const QString &title)
{
    auto *box = new QVBoxLayout;
    box->setSpacing(6);
    auto *head = strong(title);
    QFont font = head->font();
    font.setPointSizeF(font.pointSizeF() * 1.1);
    head->setFont(font);
    box->addWidget(head);
    page->addLayout(box);
    return box;
}

QString agentWords(const QJsonObject &agent)
{
    return agent["name"].toString(agent["id"].toString());
}

// What is true of an agent, and in which mood.
QString truth(const QJsonObject &agent, Mood *how)
{
    *how = Mood::Plain;
    if (!agent["installed"].toBool()) {
        *how = Mood::Warn;
        return QObject::tr("Not installed");
    }
    const QJsonValue signedIn = agent["signed_in"];
    if (!signedIn.isBool())
        return QObject::tr("Installed");
    *how = signedIn.toBool() ? Mood::Ok : Mood::Warn;
    return signedIn.toBool() ? QObject::tr("Installed, and signed in") : QObject::tr("Installed, not signed in");
}

} // namespace

SettingsPage::SettingsPage(const QString &app, QWidget *parent)
    : Page(parent)
    , app_(app)
    , program_(qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT", "levitate-agent"))
{
}

bool SettingsPage::busy() const
{
    return statusRun_ || settingsRun_ || op_ || trying_;
}

void SettingsPage::read()
{
    started_ = true;
    if (statusRun_ || settingsRun_) {
        again_ = true;
        return;
    }
    if (!loaded_ && problem_.isEmpty())
        showLoading(tr("Settings"), tr("Asking %1 for what it keeps.").arg(QFileInfo(program_).fileName()));
    statusOk_ = settingsOk_ = false;
    status_ = {};
    statusRun_ = new Run(program_, {"--status"}, this);
    settingsRun_ = new Run(program_, {"--settings"}, this);
    // Both answers are needed before the page can be drawn.
    auto done = [this](Run *&slot, QJsonObject &into, bool &ok, const char *key, const QString &flag) {
        return [this, &slot, &into, &ok, key, flag](bool finished) {
            Run *run = slot;
            slot = nullptr;
            run->deleteLater();
            const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
            ok = finished && answer.contains(key);
            if (ok)
                into = answer;
            else
                problem_ = tr("`%1 %2` failed: %3")
                               .arg(QFileInfo(program_).fileName(), flag,
                                    run->problem().isEmpty() ? tr("it did not answer") : run->problem());
            if (!statusRun_ && !settingsRun_)
                readDone();
        };
    };
    connect(statusRun_, &Run::finished, this, done(statusRun_, status_, statusOk_, "agents", "--status"));
    connect(settingsRun_, &Run::finished, this, done(settingsRun_, settings_, settingsOk_, "run_commands", "--settings"));
    statusRun_->start();
    settingsRun_->start();
}

void SettingsPage::readDone()
{
    if (statusOk_ && settingsOk_) {
        problem_.clear();
        loaded_ = true;
    } else {
        loaded_ = false;
    }
    render();
    if (again_) {
        again_ = false;
        read();
    }
}

// Every change goes to the helper; the page then reads again and shows what
// the helper reports, never what was clicked. A refusal is shown in place.
void SettingsPage::set(const QString &assignment, const QString &where)
{
    if (op_)
        return;
    notice_.clear();
    op_ = new Run(program_, {"--set", assignment}, this);
    connect(op_, &Run::finished, this, [this, where](bool ok) {
        Run *run = op_;
        op_ = nullptr;
        run->deleteLater();
        notice_ = ok ? QString() : run->problem();
        noticeWhere_ = where;
        emit changed();
        read();
    });
    op_->start();
    render();
}

void SettingsPage::chooseProgram(const QString &path)
{
    if (!path.isEmpty())
        set(QStringLiteral("agent=%1").arg(path), QStringLiteral("other"));
}

void SettingsPage::account(bool signIn)
{
    if (op_)
        return;
    notice_.clear();
    confirmingOut_ = false;
    accountOf_ = QStringLiteral("codex");
    accountVerb_ = signIn ? QStringLiteral("--sign-in") : QStringLiteral("--sign-out");
    accountLines_.clear();
    stopped_ = false;
    op_ = new Run(program_, {accountVerb_}, this);
    // What Codex prints while it waits (an address to open) is shown as it comes.
    connect(op_, &Run::output, this, [this](const QString &line) {
        accountLines_ << line.trimmed();
        if (accountLabel_)
            accountLabel_->setText(accountLines_.join(QLatin1Char('\n')));
    });
    connect(op_, &Run::finished, this, [this](bool ok) {
        Run *run = op_;
        op_ = nullptr;
        run->deleteLater();
        notice_ = ok ? QString()
                  : stopped_ ? tr("Signing in was stopped before it finished.")
                  : accountLines_.isEmpty() ? run->problem()
                                            : accountLines_.last();
        noticeWhere_ = accountOf_;
        accountOf_.clear();
        accountLines_.clear();
        emit changed();
        read();
    });
    op_->start();
    render();
}

void SettingsPage::tryIt()
{
    if (trying_)
        return;
    answer_.clear();
    notice_.clear();
    answerFailed_ = false;
    trying_ = new Run(program_, {}, this);
    trying_->setPrompt(QString::fromLatin1(hello), app_, tr("Say hello"));
    auto *said = new QString;
    connect(trying_, &Run::event, this, [said](const QJsonObject &event) {
        const QString kind = event["event"].toString();
        if (kind == QLatin1String("agent") && !event["summary"].toString().isEmpty())
            *said = event["summary"].toString();
        else if (kind == QLatin1String("say") && !event["text"].toString().isEmpty() && said->isEmpty())
            *said = event["text"].toString();
    });
    connect(trying_, &Run::finished, this, [this, said](bool ok) {
        Run *run = trying_;
        trying_ = nullptr;
        run->deleteLater();
        if (ok && !said->isEmpty()) {
            answer_ = said->trimmed();
        } else {
            answerFailed_ = true;
            answer_ = !run->problem().isEmpty() ? run->problem() : tr("It ended without an answer.");
        }
        delete said;
        emit ran();
        render();
    });
    trying_->start();
    render();
}

void SettingsPage::pickCommands(const QString &words)
{
    set(QStringLiteral("run_commands=%1").arg(words == tr("Allowed") ? "allowed" : "ask"), QStringLiteral("commands"));
}

// ---- Drawing

QWidget *SettingsPage::agentRow(const QJsonObject &agent, QButtonGroup *group)
{
    const QString id = agent["id"].toString();
    auto *row = new QWidget;
    auto *line = new QHBoxLayout(row);
    line->setContentsMargins(12, 8, 12, 8);
    line->setSpacing(12);

    auto *column = new QVBoxLayout;
    column->setSpacing(1);
    auto *radio = new QRadioButton(agentWords(agent));
    QFont bold = radio->font();
    bold.setBold(true);
    radio->setFont(bold);
    group->addButton(radio);
    radio->setChecked(settings_["agent"].toString() == id);
    radio->setEnabled(!op_ && agent["installed"].toBool());
    connect(radio, &QRadioButton::clicked, this, [this, id] { set(QStringLiteral("agent=%1").arg(id), id); });
    column->addWidget(radio);

    // Under the name, in line with it.
    auto *under = new QVBoxLayout;
    under->setSpacing(2);
    under->setContentsMargins(style()->pixelMetric(QStyle::PM_ExclusiveIndicatorWidth) + style()->pixelMetric(QStyle::PM_RadioButtonLabelSpacing), 0, 0, 0);
    column->addLayout(under);

    const bool working = op_ && accountOf_ == id;
    auto *button = new QPushButton;
    button->setEnabled(!op_);
    if (working) {
        const bool in = accountVerb_ == QLatin1String("--sign-in");
        under->addWidget(mood(in ? tr("Signing in. Finish in the browser; what %1 printed is below.").arg(agentWords(agent))
                                 : tr("Signing out…"),
                              Mood::Plain));
        if (in) {
            accountLabel_ = mood(accountLines_.join(QLatin1Char('\n')), Mood::Plain);
            accountLabel_->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
            under->addWidget(accountLabel_);
        }
        button->setText(in ? tr("Stop") : tr("Signing out…"));
        // Stopping ends the helper, which ends Codex's login with it.
        button->setEnabled(in);
        connect(button, &QPushButton::clicked, this, [this] {
            if (op_) {
                stopped_ = true;
                op_->stop();
            }
        });
    } else {
        Mood how;
        const QString words = truth(agent, &how);
        const bool signedIn = agent["signed_in"].isBool() && agent["signed_in"].toBool();
        if (confirmingOut_ && id == QLatin1String("codex") && signedIn) {
            under->addWidget(mood(tr("Sign out of %1? The apps cannot hand anything to it until you sign in again.")
                                      .arg(agentWords(agent)),
                                  Mood::Warn));
            button->setText(tr("Sign out"));
            connect(button, &QPushButton::clicked, this, [this] { account(false); });
            auto *keep = new QPushButton(tr("Keep it"));
            connect(keep, &QPushButton::clicked, this, [this] {
                confirmingOut_ = false;
                render();
            });
            line->addLayout(column, 1);
            line->addWidget(keep, 0, Qt::AlignVCenter);
            line->addWidget(button, 0, Qt::AlignVCenter);
            return row;
        }
        under->addWidget(mood(words, how, false));
        if (noticeWhere_ == id && !notice_.isEmpty())
            under->addWidget(mood(notice_, Mood::Bad));
        if (!agent["installed"].toBool()) {
            delete button;
            button = nullptr;
        } else if (signedIn) {
            button->setText(tr("Sign out…"));
            connect(button, &QPushButton::clicked, this, [this] {
                confirmingOut_ = true;
                notice_.clear();
                render();
            });
        } else {
            button->setText(tr("Sign in…"));
            connect(button, &QPushButton::clicked, this, [this] { account(true); });
        }
    }
    line->addLayout(column, 1);
    if (button)
        line->addWidget(button, 0, Qt::AlignVCenter);
    return row;
}

QWidget *SettingsPage::otherRow(QButtonGroup *group)
{
    const QString chosen = settings_["agent"].toString();
    QStringList named;
    for (const QJsonValue &agent : status_["agents"].toArray())
        if (!agent["id"].toString().contains(QLatin1Char('/')))
            named << agent["id"].toString();
    const bool isOther = !named.contains(chosen);

    auto *row = new QWidget;
    auto *line = new QHBoxLayout(row);
    line->setContentsMargins(12, 8, 12, 8);
    line->setSpacing(12);
    auto *column = new QVBoxLayout;
    column->setSpacing(1);
    auto *radio = new QRadioButton(tr("Another program"));
    QFont bold = radio->font();
    bold.setBold(true);
    radio->setFont(bold);
    group->addButton(radio);
    radio->setChecked(isOther);
    radio->setEnabled(!op_);
    auto *under = new QVBoxLayout;
    under->setSpacing(2);
    under->setContentsMargins(style()->pixelMetric(QStyle::PM_ExclusiveIndicatorWidth) + style()->pixelMetric(QStyle::PM_RadioButtonLabelSpacing), 0, 0, 0);
    under->addWidget(dim(text(tr("Any program that takes a request and reports back the same way"), true)));
    if (isOther) {
        auto *path = new ElidedLabel(chosen);
        path->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
        under->addWidget(path);
        bool found = false;
        for (const QJsonValue &agent : status_["agents"].toArray())
            if (agent["id"].toString() == chosen)
                found = agent["installed"].toBool();
        under->addWidget(mood(found ? tr("Found, and it can be run") : tr("Cannot be run from here"),
                              found ? Mood::Ok : Mood::Warn, false));
    }
    if (noticeWhere_ == QLatin1String("other") && !notice_.isEmpty())
        under->addWidget(mood(notice_, Mood::Bad));
    column->addWidget(radio);
    column->addLayout(under);
    auto *choose = new QPushButton(tr("Choose…"));
    choose->setEnabled(!op_);
    auto dialog = [this] {
        const QString path = QFileDialog::getOpenFileName(this, tr("Choose the program"));
        if (path.isEmpty())
            render(); // the radio button goes back to what the helper reports
        else
            chooseProgram(path);
    };
    connect(choose, &QPushButton::clicked, this, dialog);
    // Choosing the row with nothing chosen yet asks for the program, as the button does.
    connect(radio, &QRadioButton::clicked, this, [this, isOther, dialog] {
        if (!isOther)
            dialog();
        else
            render();
    });
    line->addLayout(column, 1);
    line->addWidget(choose, 0, Qt::AlignVCenter);
    return row;
}

void SettingsPage::render()
{
    if (auto *old = findChild<QScrollArea *>())
        scroll_ = old->verticalScrollBar()->value();
    accountLabel_ = nullptr;
    QVBoxLayout *column = fresh();

    if (!problem_.isEmpty()) {
        column->addWidget(heading(tr("Settings could not be read")));
        column->addWidget(text(problem_, true));
        auto *buttons = new QHBoxLayout;
        auto *again = new QPushButton(tr("Try again"));
        again->setDefault(true);
        connect(again, &QPushButton::clicked, this, &SettingsPage::read);
        buttons->addWidget(again);
        buttons->addStretch();
        column->addLayout(buttons);
        column->addStretch();
        return;
    }

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    auto *body = new QWidget;
    auto *page = new QVBoxLayout(body);
    page->setContentsMargins(0, 0, 8, 0);
    page->setSpacing(18);
    scroll->setWidget(body);
    column->addWidget(scroll, 1);

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Settings")));
    top->addWidget(dim(text(tr("One set for Recipe, System and Disk. What is set here holds in all three."), true)));
    page->addLayout(top);

    // ---- The agent
    {
        QVBoxLayout *inside = section(page, tr("The agent that does the work"));
        auto *panel = rowsBox();
        // One choice among the rows; the helper's answer decides which is checked.
        auto *group = new QButtonGroup(panel);
        bool first = true;
        for (const QJsonValue &value : status_["agents"].toArray()) {
            const QJsonObject agent = value.toObject();
            // A program chosen by its path is the last row's business.
            if (agent["id"].toString().contains(QLatin1Char('/')))
                continue;
            if (!first)
                panel->column()->addWidget(rule());
            first = false;
            panel->column()->addWidget(agentRow(agent, group));
        }
        if (!first)
            panel->column()->addWidget(rule());
        panel->column()->addWidget(otherRow(group));
        inside->addWidget(panel);

        auto *tryRow = new QHBoxLayout;
        auto *button = new QPushButton(trying_ ? tr("Asking…") : tr("Try it"));
        button->setEnabled(!trying_);
        connect(button, &QPushButton::clicked, this, &SettingsPage::tryIt);
        tryRow->addWidget(button);
        tryRow->addWidget(dim(text(tr("Asks the chosen agent to say hello."))));
        tryRow->addStretch();
        inside->addLayout(tryRow);
        if (!answer_.isEmpty())
            inside->addWidget(answerFailed_ ? mood(answer_, Mood::Bad) : text(tr("It answered: %1").arg(answer_), true));
    }

    // ---- What it may do
    {
        QVBoxLayout *inside = section(page, tr("What the agent may do without asking"));
        auto *panel = rowsBox();
        auto row = [&](const QString &what, QWidget *control, QWidget *under = nullptr) {
            auto *widget = new QWidget;
            auto *grid = new QGridLayout(widget);
            grid->setContentsMargins(12, 8, 12, 8);
            grid->setHorizontalSpacing(16);
            grid->setColumnStretch(0, 1);
            grid->addWidget(text(what, true), 0, 0, Qt::AlignVCenter);
            grid->addWidget(control, 0, 1, Qt::AlignVCenter | Qt::AlignRight);
            if (under)
                grid->addWidget(under, 1, 0, 1, 2);
            return widget;
        };
        panel->column()->addWidget(row(tr("Read files and look things up"), mood(tr("Always"), Mood::Ok, false)));
        panel->column()->addWidget(rule());

        auto *choice = new QComboBox;
        choice->setAccessibleName(tr("Run commands as you, in your own folders"));
        choice->addItems({tr("Ask each time"), tr("Allowed")});
        choice->setCurrentIndex(settings_["run_commands"].toString() == QLatin1String("allowed") ? 1 : 0);
        choice->setEnabled(!op_);
        connect(choice, &QComboBox::activated, this, [this, choice](int index) { pickCommands(choice->itemText(index)); });
        auto *under = new QVBoxLayout;
        auto *underWidget = new QWidget;
        underWidget->setLayout(under);
        under->setContentsMargins(0, 0, 0, 0);
        under->setSpacing(2);
        under->addWidget(dim(text(tr("With Allowed, the agent's own sandbox lets it write in your home folder. A command "
                                     "it wants run outside its sandbox is still asked each time."),
                                  true)));
        const QString agent = settings_["agent"].toString();
        if (agent.contains(QLatin1Char('/')))
            under->addWidget(dim(text(tr("Only Codex reads this setting. The program chosen above does not."), true)));
        if (noticeWhere_ == QLatin1String("commands") && !notice_.isEmpty())
            under->addWidget(mood(notice_, Mood::Bad));
        panel->column()->addWidget(row(tr("Run commands as you, in your own folders"), choice, underWidget));
        panel->column()->addWidget(rule());
        panel->column()->addWidget(row(tr("Run a command as administrator"),
                                       mood(tr("Always asks. It cannot be switched off."), Mood::Warn, false)));
        inside->addWidget(panel);
    }

    // ---- The windows
    {
        QVBoxLayout *inside = section(page, tr("The windows"));
        inside->addWidget(text(tr("Each window opens at the size it had. Language follows the desktop."), true));
    }

    page->addStretch();
    page->addWidget(dim(text(tr("Settings are kept for you, not for everyone on this computer."), true)));
    scroll->verticalScrollBar()->setValue(scroll_);
    // The scroll range exists only after layout.
    QMetaObject::invokeMethod(scroll, [scroll, this] { scroll->verticalScrollBar()->setValue(scroll_); }, Qt::QueuedConnection);
}

// ---- For a check

QString SettingsPage::pageText() const
{
    QStringList all;
    for (QLabel *label : findChildren<QLabel *>())
        if (onPage(label, this))
            all << label->text();
    for (QAbstractButton *button : findChildren<QAbstractButton *>())
        if (onPage(button, this))
            all << button->text();
    for (QComboBox *combo : findChildren<QComboBox *>())
        if (onPage(combo, this))
            all << combo->currentText();
    return all.join(QLatin1Char('\n'));
}

QString SettingsPage::checkedAgent() const
{
    for (QRadioButton *radio : findChildren<QRadioButton *>())
        if (onPage(radio, this) && radio->isChecked())
            return radio->text() == tr("Another program") ? QStringLiteral("other") : radio->text().toLower();
    return QString();
}

bool SettingsPage::press(const QString &words)
{
    for (QAbstractButton *button : findChildren<QAbstractButton *>()) {
        if (button->text() == words && onPage(button, this) && button->isEnabled()) {
            button->click();
            return true;
        }
    }
    return false;
}

// ---- The size a window had

namespace {

class SizeKeeper : public QObject {
public:
    SizeKeeper(QWidget *window, const QString &key)
        : QObject(window)
        , window_(window)
        , key_(key)
    {
        window->installEventFilter(this);
    }

protected:
    bool eventFilter(QObject *watched, QEvent *event) override
    {
        if (watched == window_ && event->type() == QEvent::Close) {
            // A maximised window keeps the size it had before, not the screen's.
            const QSize size = window_->isMaximized() || window_->isFullScreen() ? window_->normalGeometry().size()
                                                                                  : window_->size();
            QSettings settings(QStringLiteral("LevitateOS"), QStringLiteral("windows"));
            if (size.isValid() && !size.isEmpty())
                settings.setValue(key_, size);
            settings.sync();
        }
        return false;
    }

private:
    QWidget *window_;
    QString key_;
};

} // namespace

void rememberWindowSize(QWidget *window, const QString &app, QSize first, bool remember)
{
    QSize size = first;
    if (remember) {
        QSettings settings(QStringLiteral("LevitateOS"), QStringLiteral("windows"));
        const QSize kept = settings.value(app).toSize();
        // A size nobody could use (a hand-edited file) is the first size.
        if (kept.width() >= 400 && kept.height() >= 300)
            size = kept;
        new SizeKeeper(window, app);
    }
    window->resize(size);
}
