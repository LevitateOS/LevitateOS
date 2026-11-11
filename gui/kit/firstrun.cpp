#include "firstrun.h"

#include "help.h"
#include "look.h"
#include "parts.h"
#include "run.h"

#include <QAbstractButton>
#include <QFileInfo>
#include <QHBoxLayout>
#include <QJsonArray>
#include <QJsonDocument>
#include <QPushButton>
#include <QScrollArea>
#include <QSettings>
#include <QVBoxLayout>

namespace {

QString keyOf(Frame::App app)
{
    return app == Frame::Recipe ? QStringLiteral("recipe") : app == Frame::System ? QStringLiteral("system")
                                                                                 : QStringLiteral("disk");
}

// The same test as Settings and Activity use for the helper.
QString helper()
{
    return qEnvironmentVariable("LEVITATE_ACTIVITY_AGENT", "levitate-agent");
}

bool onPage(const QWidget *widget, const QWidget *page)
{
    for (const QWidget *w = widget; w && w != page; w = w->parentWidget())
        if (w->isHidden() && w->testAttribute(Qt::WA_WState_ExplicitShowHide))
            return false;
    return true;
}

} // namespace

bool FirstRunPage::wanted(Frame::App app)
{
    if (pictureMode())
        return false;
    QSettings settings(QStringLiteral("LevitateOS"), QStringLiteral("firstrun"));
    return !settings.value(keyOf(app), false).toBool();
}

void FirstRunPage::markSeen(Frame::App app)
{
    if (pictureMode())
        return;
    QSettings settings(QStringLiteral("LevitateOS"), QStringLiteral("firstrun"));
    settings.setValue(keyOf(app), true);
    settings.sync();
}

FirstRunPage::FirstRunPage(Frame::App app, QWidget *parent)
    : Page(parent)
    , app_(app)
{
    draw();
}

void FirstRunPage::read()
{
    if (asking_)
        return;
    agent_ = Asking;
    draw();
    asking_ = new Run(helper(), {"--status"}, this);
    connect(asking_, &Run::finished, this, [this](bool ok) {
        Run *run = asking_;
        asking_ = nullptr;
        run->deleteLater();
        const QJsonObject answer = QJsonDocument::fromJson(run->answer()).object();
        if (!ok || !answer.contains("agents")) {
            agent_ = CouldNotAsk;
            problem_ = run->problem();
            draw();
            return;
        }
        QJsonObject chosen;
        for (const QJsonValue &agent : answer["agents"].toArray())
            if (agent["id"].toString() == answer["chosen"].toString())
                chosen = agent.toObject();
        agentName_ = chosen["name"].toString(answer["chosen"].toString());
        const QJsonValue signedIn = chosen["signed_in"];
        agent_ = !chosen["installed"].toBool() ? NotInstalled
                 : !signedIn.isBool()           ? SignInUnknown
                 : signedIn.toBool()            ? Ready
                                                : NotSignedIn;
        draw();
    });
    asking_->start();
}

void FirstRunPage::draw()
{
    const HelpText t = helpText(app_);
    QVBoxLayout *column = fresh();

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    auto *body = new QWidget;
    auto *page = new QVBoxLayout(body);
    page->setContentsMargins(0, 0, 12, 0);
    page->setSpacing(12);
    scroll->setWidget(body);
    column->addWidget(scroll, 1);

    auto *wide = new QWidget;
    wide->setMaximumWidth(720);
    auto *lines = new QVBoxLayout(wide);
    lines->setContentsMargins(0, 0, 0, 0);
    lines->setSpacing(12);
    auto *at = new QHBoxLayout;
    at->addWidget(wide, 1);
    at->addStretch(1);
    page->addLayout(at);
    page->addStretch();

    lines->addWidget(heading(tr("Welcome to %1").arg(t.name)));
    lines->addWidget(text(t.purpose, true));
    lines->addWidget(text(t.never, true));

    // ---- Worth knowing first
    {
        QLabel *head = strong(tr("Worth knowing first"));
        QFont font = head->font();
        font.setPointSizeF(font.pointSizeF() * 1.2);
        head->setFont(font);
        lines->addWidget(head);
        auto *panel = new Panel;
        QVBoxLayout *inside = panel->column();
        inside->setSpacing(4);
        bool first = true;
        for (const HelpIdea &idea : t.ideas) {
            if (!idea.forFirstRun)
                continue;
            if (!first)
                inside->addWidget(rule());
            first = false;
            inside->addWidget(strong(idea.title));
            inside->addWidget(text(idea.first, true));
        }
        lines->addWidget(panel);
    }

    // ---- Is an agent ready
    {
        QLabel *head = strong(tr("The agent"));
        QFont font = head->font();
        font.setPointSizeF(font.pointSizeF() * 1.2);
        head->setFont(font);
        lines->addWidget(head);
        const bool ready = agent_ == Ready;
        auto *card = new Card(ready ? Card::Ready : agent_ == Asking ? Card::Plain : Card::Open);
        QVBoxLayout *inside = card->column();
        inside->setSpacing(4);
        QString said, more;
        switch (agent_) {
        case Asking:
            said = tr("Asking the agent…");
            break;
        case Ready:
            said = tr("An agent is ready: %1 is installed and signed in.").arg(agentName_);
            break;
        case NotSignedIn:
            said = tr("No agent is ready yet: %1 is installed, but not signed in.").arg(agentName_);
            break;
        case NotInstalled:
            said = tr("No agent is ready yet: %1 is not installed.").arg(agentName_);
            break;
        case SignInUnknown:
            said = tr("%1 is installed. Whether it is signed in could not be asked.").arg(agentName_);
            break;
        case CouldNotAsk:
            said = tr("The agent could not be asked.");
            more = problem_;
            break;
        }
        inside->addWidget(strong(said));
        if (!more.isEmpty())
            inside->addWidget(dim(text(more, true)));
        if (agent_ == NotSignedIn || agent_ == NotInstalled || agent_ == CouldNotAsk) {
            inside->addWidget(text(tr("You can use %1 now. The agent is needed only when you hand it something.").arg(t.name),
                                   true));
            ClickLabel *go = linkLabel(agent_ == NotSignedIn ? tr("Sign in in Settings") : tr("Choose an agent in Settings"));
            connect(go, &ClickLabel::clicked, this, &FirstRunPage::openSettings);
            inside->addWidget(go, 0, Qt::AlignLeft);
        }
        lines->addWidget(card);
    }

    // ---- On: outside the scrolled part, so the one button is always in view.
    column->addWidget(rule());
    {
        auto *row = new QHBoxLayout;
        row->setSpacing(14);
        auto *on = new QPushButton(tr("Go on to %1").arg(t.name));
        on->setDefault(true);
        connect(on, &QPushButton::clicked, this, &FirstRunPage::goOn);
        row->addWidget(on);
        ClickLabel *help = linkLabel(tr("Read Help first"));
        connect(help, &ClickLabel::clicked, this, &FirstRunPage::openHelp);
        row->addWidget(help);
        row->addStretch();
        row->addWidget(dim(text(tr("This page is shown once. Help has “Show the first page again”."))));
        column->addLayout(row);
    }
}

QString FirstRunPage::pageText() const
{
    QStringList all;
    for (QLabel *label : findChildren<QLabel *>())
        if (onPage(label, this))
            all << label->text();
    for (QAbstractButton *button : findChildren<QAbstractButton *>())
        if (onPage(button, this))
            all << button->text();
    return all.join(QLatin1Char('\n'));
}

bool FirstRunPage::press(const QString &words)
{
    for (ClickLabel *label : findChildren<ClickLabel *>()) {
        if (label->text() == words && onPage(label, this)) {
            emit label->clicked();
            return true;
        }
    }
    for (QAbstractButton *button : findChildren<QAbstractButton *>()) {
        if (button->text() == words && button->isEnabled() && onPage(button, this)) {
            button->click();
            return true;
        }
    }
    return false;
}
