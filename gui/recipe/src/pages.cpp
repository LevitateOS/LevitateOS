#include "pages.h"

#include "beforewords.h"
#include "client.h"
#include "parts.h"

#include <QCheckBox>
#include <QComboBox>
#include <QDialogButtonBox>
#include <QFontDatabase>
#include <QFormLayout>
#include <QGridLayout>
#include <QHeaderView>
#include <QJsonArray>
#include <QLineEdit>
#include <QLocale>
#include <QPlainTextEdit>
#include <QProgressBar>
#include <QPushButton>
#include <QScrollArea>
#include <QTreeWidget>
#include <QVBoxLayout>

#include <memory>

namespace {

QPushButton *primary(const QString &words)
{
    auto *button = new QPushButton(words);
    button->setDefault(true);
    return button;
}

// The top of a page about one recipe: its picture, name and one line, with
// the page's buttons on the right.
QHBoxLayout *recipeHeader(const Recipe &recipe, const QString &title, QLabel **titleOut = nullptr,
                          const QString &from = QString())
{
    auto *row = new QHBoxLayout;
    row->setSpacing(16);
    row->addWidget(new Tile(recipe.shown(), QString(), 60));
    auto *words = new QVBoxLayout;
    words->setSpacing(1);
    QLabel *name = heading(title);
    if (titleOut)
        *titleOut = name;
    words->addWidget(name);
    words->addWidget(text(recipe.description));
    QString line = recipe.category.isEmpty() ? QObject::tr("Version %1").arg(recipe.version)
                                             : QObject::tr("Version %1 · %2").arg(recipe.version, categoryTitle(recipe.category));
    if (!from.isEmpty())
        line += QObject::tr(" · from %1").arg(from);
    words->addWidget(dim(text(line)));
    row->addLayout(words);
    row->addStretch();
    return row;
}

QString fetchSentence(const Recipe &recipe)
{
    if (recipe.sources.isEmpty())
        return recipe.needs.network.isEmpty()
                   ? QObject::tr("Nothing is fetched from the network.")
                   : QObject::tr("Fetches over the network, from the hosts listed beside this and no others.");
    QStringList parts;
    for (const Source &source : recipe.sources)
        parts << QObject::tr("from %1, %2").arg(source.host(), source.pin());
    return QObject::tr("Gets the source %1.").arg(parts.join(QObject::tr("; and ")));
}

QString buildSentence(const Recipe &recipe)
{
    if (!recipe.hooks.contains("build"))
        return QObject::tr("The recipe has no build step.");
    QString sentence = QObject::tr("Runs the recipe's build step on this computer, inside the sandbox. It sees none of your files.");
    if (!recipe.buildDeps.isEmpty())
        sentence += QObject::tr(" Builds first: %1.").arg(recipe.buildDeps.join(QObject::tr(", ")));
    return sentence;
}

QString installSentence(const Recipe &recipe)
{
    QStringList parts;
    if (!recipe.menu.isEmpty())
        parts << QObject::tr("the menu entry %1").arg(recipe.menu.join(QObject::tr(", ")));
    if (recipe.programs.size() == 1)
        parts << QObject::tr("the program %1").arg(recipe.programs.first());
    else if (recipe.programs.size() > 1)
        parts << QObject::tr("the programs %1").arg(recipe.programs.join(QObject::tr(", ")));
    QString sentence = parts.isEmpty() ? QObject::tr("Puts its files in place.")
                                       : QObject::tr("Adds %1.").arg(parts.join(QObject::tr(" and ")));
    if (recipe.afterInstall == QLatin1String("restart"))
        sentence += QObject::tr(" The computer must be restarted afterwards.");
    else if (recipe.afterInstall == QLatin1String("log-in-again"))
        sentence += QObject::tr(" You must log in again afterwards.");
    return sentence;
}

// One numbered step: what the front end can say from enforced facts, then the
// author's own note, marked as such.
QHBoxLayout *stepRow(int number, const QString &title, const QString &sentence, const QString &note)
{
    auto *row = new QHBoxLayout;
    row->setSpacing(12);
    row->addWidget(new StepMark(number), 0, Qt::AlignTop);
    auto *words = new QVBoxLayout;
    words->setSpacing(1);
    words->addWidget(strong(title));
    words->addWidget(text(sentence, true));
    if (!note.isEmpty())
        words->addWidget(dim(text(QObject::tr("The author's note: %1").arg(note), true)));
    row->addLayout(words, 1);
    return row;
}

QTreeWidget *table(const QStringList &columns)
{
    auto *tree = new QTreeWidget;
    tree->setHeaderLabels(columns);
    tree->setRootIsDecorated(false);
    tree->setUniformRowHeights(true);
    tree->setAllColumnsShowFocus(true);
    tree->setIndentation(0);
    tree->header()->setMinimumSectionSize(96);
    tree->header()->setDefaultAlignment(Qt::AlignLeft | Qt::AlignVCenter);
    tree->header()->setStretchLastSection(false);
    tree->header()->setSectionResizeMode(0, QHeaderView::Stretch);
    for (int column = 1; column < columns.size(); ++column)
        tree->header()->setSectionResizeMode(column, QHeaderView::ResizeToContents);
    return tree;
}

void clear(QLayout *layout)
{
    while (QLayoutItem *item = layout->takeAt(0)) {
        if (QWidget *widget = item->widget())
            widget->deleteLater();
        if (QLayout *child = item->layout())
            clear(child);
        delete item;
    }
}

} // namespace

// ---------------------------------------------------------------- Browse

BrowsePage::BrowsePage(QWidget *parent)
    : Page(parent)
{
    top_ = new QWidget;
    auto *column = new QVBoxLayout(top_);
    column->setContentsMargins(28, 20, 28, 0);
    column->setSpacing(14);
    auto *words = new QVBoxLayout;
    words->setSpacing(1);
    title_ = heading(QString());
    subtitle_ = dim(text(QString()));
    words->addWidget(title_);
    words->addWidget(subtitle_);
    column->addLayout(words);
    auto *row = new QHBoxLayout;
    row->setSpacing(8);
    search_ = new QLineEdit;
    search_->setPlaceholderText(tr("Search by name or by what it does"));
    search_->setClearButtonEnabled(true);
    search_->setAccessibleName(tr("Search software"));
    row->addWidget(search_, 1);
    // Which source's recipes are shown: all of them, or one.
    from_ = new QComboBox;
    from_->setAccessibleName(tr("Recipes from"));
    row->addWidget(from_);
    category_ = new QComboBox;
    category_->setAccessibleName(tr("Kind of software"));
    row->addWidget(category_);
    column->addLayout(row);
    // Above the list, which `fresh` puts last; the header must not grow with the window.
    top_->setSizePolicy(QSizePolicy::Preferred, QSizePolicy::Maximum);
    static_cast<QVBoxLayout *>(layout())->insertWidget(0, top_);
}

void BrowsePage::showLoading(const QString &title, const QString &words)
{
    top_->hide();
    listed_.clear();
    Page::showLoading(title, words);
}

void BrowsePage::show(const Catalog &catalog, const QString &category, const QString &search, const QString &origin)
{
    QVBoxLayout *column = fresh();
    // The header above has the page's top margin already: only the usual gap is left.
    column->setContentsMargins(28, 14, 28, 16);
    top_->show();
    listed_.clear();
    title_->setText(!search.isEmpty() ? tr("Search: %1").arg(search)
                    : category.isEmpty() ? tr("All software") : categoryTitle(category));
    const Origin *from = origin.isEmpty() ? nullptr : catalog.origin(origin);
    QString subtitle = tr("Each one is installed from a recipe you can read.");
    if (!search.isEmpty() && !category.isEmpty())
        subtitle = tr("Only in %1. %2").arg(categoryTitle(category), subtitle);
    if (from)
        subtitle = tr("Only what %1 has. %2").arg(from->shown(), subtitle);
    subtitle_->setText(subtitle);
    // More than one source: say on every line which one it is from.
    int sourcesWithRecipes = 0;
    for (const Origin &o : catalog.origins)
        sourcesWithRecipes += catalog.countFrom(o.name) > 0;

    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::StyledPanel);
    auto *inner = new QWidget;
    inner->setBackgroundRole(QPalette::Base);
    inner->setAutoFillBackground(true);
    auto *grid = new QGridLayout(inner);
    grid->setContentsMargins(14, 2, 14, 2);
    grid->setHorizontalSpacing(14);
    grid->setVerticalSpacing(0);
    grid->setColumnStretch(1, 1);
    grid->setColumnMinimumWidth(2, 150);
    grid->setColumnMinimumWidth(3, 70);
    grid->setColumnMinimumWidth(4, 100);

    int row = 0;
    for (const Recipe &recipe : catalog.recipes) {
        if (!recipe.listed() || (!category.isEmpty() && recipe.category != category))
            continue;
        if (!origin.isEmpty() && recipe.origin != origin)
            continue;
        if (!search.isEmpty() && !recipe.name.contains(search, Qt::CaseInsensitive)
            && !recipe.title.contains(search, Qt::CaseInsensitive)
            && !recipe.description.contains(search, Qt::CaseInsensitive))
            continue;
        listed_ << recipe.name;
        if (row)
            grid->addWidget(rule(), row++, 0, 1, 5);
        grid->setRowMinimumHeight(row, 58);
        const QString name = recipe.name;

        grid->addWidget(new Tile(recipe.shown(), QString(), 36), row, 0);
        auto *words = new QVBoxLayout;
        words->setSpacing(0);
        auto *link = new ClickLabel(recipe.shown());
        QFont bold = link->font();
        bold.setBold(true);
        link->setFont(bold);
        connect(link, &ClickLabel::clicked, this, [this, name] { emit openRecipe(name); });
        words->addStretch();
        words->addWidget(link, 0, Qt::AlignLeft);
        words->addWidget(dim(text(recipe.description)));
        words->addStretch();
        grid->addLayout(words, row, 1);
        const Origin *its = catalog.origin(recipe.origin);
        grid->addWidget(dim(text(sourcesWithRecipes > 1 && its
                                     ? tr("%1\nfrom %2").arg(categoryTitle(recipe.category), its->shown())
                                     : categoryTitle(recipe.category))),
                        row, 2);
        grid->addWidget(dim(text(recipe.version)), row, 3);

        // Software can be in several places; the button is for what is next to do.
        const QList<const InstalledItem *> held = catalog.installedEverywhere(name);
        const UpdateItem *newer = catalog.update(name);
        bool onlyNext = !held.isEmpty();
        for (const InstalledItem *item : held)
            onlyNext = onlyNext && item->place == Place::Next;
        if (newer) {
            const Place where = newer->place;
            auto *button = new QPushButton(tr("Update"));
            connect(button, &QPushButton::clicked, this, [this, name, where] { emit update(name, where); });
            grid->addWidget(button, row, 4, Qt::AlignRight);
        } else if (held.isEmpty() && !catalog.waitsForRestart(name)) {
            auto *button = new QPushButton(tr("Install"));
            connect(button, &QPushButton::clicked, this, [this, name] { emit install(name); });
            grid->addWidget(button, row, 4, Qt::AlignRight);
        } else {
            grid->addWidget(dim(text(onlyNext || held.isEmpty() ? tr("After a restart") : tr("Installed"))), row, 4, Qt::AlignRight);
        }
        ++row;
    }
    if (!row)
        grid->addWidget(dim(text(search.isEmpty() ? tr("Nothing here.") : tr("No recipe matches “%1”.").arg(search))),
                        row++, 0, 1, 5, Qt::AlignCenter);
    grid->setRowStretch(row, 1);
    scroll->setWidget(inner);
    column->addWidget(scroll, 1);

    // What no recipe exists for yet can be asked for: an agent writes one.
    auto *foot = new QHBoxLayout;
    foot->addWidget(dim(text(row && !search.isEmpty() ? tr("Not what you were looking for? An agent can write a recipe for it.")
                             : !search.isEmpty() ? tr("An agent can write a recipe for it. You read it before anything is installed.")
                                                 : tr("Missing something? An agent can write a recipe for it."),
                             true)),
                    1);
    auto *ask = new QPushButton(search.isEmpty() ? tr("Ask for software…") : tr("Ask for “%1”…").arg(search));
    connect(ask, &QPushButton::clicked, this, [this, search] { emit askFor(search); });
    foot->addWidget(ask);
    column->addLayout(foot);
}

void BrowsePage::showProblem(const QString &problem)
{
    QVBoxLayout *column = fresh();
    top_->hide();
    listed_.clear();
    column->addWidget(heading(tr("The recipes could not be read")));
    column->addWidget(text(problem, true));
    auto *again = new QPushButton(tr("Try again"));
    connect(again, &QPushButton::clicked, this, &BrowsePage::retry);
    column->addWidget(again, 0, Qt::AlignLeft);
    column->addStretch();
}

// ---------------------------------------------------------------- A recipe

void RecipePage::show(const Catalog &catalog, const QString &name)
{
    QVBoxLayout *column = fresh();
    const Recipe *found = catalog.recipe(name);
    if (!found) {
        column->addWidget(heading(tr("No recipe named %1").arg(name)));
        column->addStretch();
        return;
    }
    const Recipe recipe = *found;
    const QList<const InstalledItem *> held = catalog.installedEverywhere(name);
    const QList<Place> free = catalog.installPlaces(name);
    const bool several = held.size() > 1;
    // "for me", "for this computer": which place a button or a line is about.
    auto where = [&catalog](Place place) {
        if (!catalog.twoPlaces)
            return QString();
        return place == Place::Me ? tr("for me") : place == Place::Computer ? tr("for this computer") : tr("in the next system");
    };

    auto *backLink = new ClickLabel(tr("‹ All software"));
    dim(backLink);
    connect(backLink, &ClickLabel::clicked, this, &RecipePage::back);
    column->addWidget(backLink, 0, Qt::AlignLeft);

    const Origin *its = catalog.origin(recipe.origin);
    QHBoxLayout *header = recipeHeader(recipe, recipe.shown(), nullptr, its ? its->shown() : QString());
    auto *actions = new QVBoxLayout;
    actions->setSpacing(6);
    auto *buttons = new QHBoxLayout;
    // What it does for each place that holds it; with two, the button says which.
    auto named = [&](const QString &words, Place place) {
        return several || (catalog.twoPlaces && place == Place::Next) ? tr("%1 %2").arg(words, where(place)) : words;
    };
    for (const InstalledItem *item : held) {
        const Place place = item->place;
        if (place != Place::Next && !item->menu.isEmpty() && item == held.first()) {
            auto *open = new QPushButton(tr("Open"));
            connect(open, &QPushButton::clicked, this, [this, name, place] { emit this->open(name, place); });
            buttons->addWidget(open);
        }
        auto *remove = new QPushButton(tr("%1…").arg(named(tr("Remove"), place)));
        connect(remove, &QPushButton::clicked, this, [this, name, place] { emit this->remove(name, place); });
        buttons->addWidget(remove);
    }
    for (const InstalledItem *item : held) {
        const UpdateItem *update = catalog.update(name, item->place);
        if (!update || item->place == Place::Next)
            continue;
        const Place place = item->place;
        auto *go = primary(named(tr("Update to %1").arg(update->to), place));
        connect(go, &QPushButton::clicked, this, [this, name, place] { emit this->update(name, place); });
        buttons->addWidget(go);
    }
    if (!free.isEmpty() && !catalog.waitsForRestart(name)) {
        // Installed in one place and free in the other: the page offers that other place.
        auto *go = held.isEmpty() ? primary(tr("Install")) : new QPushButton(tr("Install %1").arg(where(free.first())));
        connect(go, &QPushButton::clicked, this, [this, name] { emit install(name); });
        buttons->addWidget(go);
    }
    QStringList said;
    for (const InstalledItem *item : held) {
        const QString at = where(item->place);
        said << (at.isEmpty() ? tr("Version %1 is installed").arg(item->version)
                              : tr("Version %1 is installed %2").arg(item->version, at));
    }
    bool anyUpdate = false;
    for (const InstalledItem *item : held)
        anyUpdate = anyUpdate || catalog.update(name, item->place);
    // Without a version to put beside the buttons: what the next page is for.
    QString line = !said.isEmpty() && !anyUpdate ? said.join(tr("; ")) : tr("You see what will happen before anything starts");
    if (catalog.waitsForRestart(name) && !catalog.installedItem(name, Place::Next))
        line = tr("The next system holds a change for it. It takes effect after a restart.");
    // Several buttons, each saying where, do not fit beside the name: they take a row of their own.
    if (buttons->count() > 3 || held.size() > 1) {
        column->addLayout(header);
        auto *row = new QHBoxLayout;
        row->addWidget(dim(text(line, true)), 1);
        row->addLayout(buttons);
        column->addLayout(row);
    } else {
        actions->addLayout(buttons);
        actions->addWidget(dim(text(line)), 0, Qt::AlignRight);
        header->addLayout(actions);
        column->addLayout(header);
    }

    auto *pair = new QHBoxLayout;
    pair->setSpacing(14);

    auto *does = new Panel;
    does->column()->addWidget(strong(tr("What installing does")));
    does->column()->addLayout(stepRow(1, tr("Fetch"), fetchSentence(recipe), recipe.notes.value("acquire")));
    does->column()->addLayout(stepRow(2, tr("Build"), buildSentence(recipe), recipe.notes.value("build")));
    does->column()->addLayout(stepRow(3, tr("Install"), installSentence(recipe), recipe.notes.value("install")));
    does->column()->addStretch();
    pair->addWidget(does, 1);

    auto *uses = new Panel;
    auto *usesTitle = new QVBoxLayout;
    usesTitle->setSpacing(0);
    usesTitle->addWidget(strong(tr("What the recipe may use")));
    usesTitle->addWidget(dim(text(tr("The sandbox refuses anything else."))));
    uses->column()->addLayout(usesTitle);
    auto *form = new QFormLayout;
    form->setHorizontalSpacing(16);
    form->setVerticalSpacing(7);
    auto fact = [form](const QString &label, const QString &value) {
        form->addRow(dim(text(label)), text(value, true));
    };
    fact(tr("Network"), recipe.needs.network.isEmpty() ? tr("none")
                                                        : tr("%1. Nothing else.").arg(endpointsText(recipe.needs.network)));
    if (recipe.needs.networkMaxBytes)
        fact(tr("Download"), tr("up to %1").arg(bytesText(recipe.needs.networkMaxBytes)));
    if (recipe.needs.memoryMib)
        fact(tr("Memory"), tr("up to %1").arg(bytesText(recipe.needs.memoryMib * 1048576)));
    fact(tr("Disk"), recipe.needs.storageMaxBytes
                         ? tr("up to %1 while it builds").arg(bytesText(recipe.needs.storageMaxBytes))
                         : tr("the standard limit"));
    if (recipe.needs.timeoutSecs)
        fact(tr("Time"), counted(recipe.needs.timeoutSecs / 60, tr("up to %1 minute"), tr("up to %1 minutes")));
    fact(tr("Your files"), tr("none"));
    uses->column()->addLayout(form);
    uses->column()->addStretch();
    pair->addWidget(uses, 1);
    column->addLayout(pair);

    auto *about = new Panel;
    auto *aboutTitle = new QVBoxLayout;
    aboutTitle->setSpacing(0);
    aboutTitle->addWidget(strong(tr("About %1").arg(recipe.shown())));
    aboutTitle->addWidget(dim(text(tr("In the words of the recipe's author."))));
    about->column()->addLayout(aboutTitle);
    if (!recipe.details.isEmpty())
        about->column()->addWidget(text(recipe.details, true));
    if (recipe.details.isEmpty() && recipe.homepage.isEmpty() && recipe.license.isEmpty())
        about->column()->addWidget(dim(text(tr("This recipe says nothing about itself yet."))));
    auto *facts = new QHBoxLayout;
    facts->setSpacing(24);
    if (!recipe.homepage.isEmpty()) {
        auto *home = new QLabel(QStringLiteral("<a href=\"%1\">%2</a>")
                                    .arg(recipe.homepage.toHtmlEscaped(), QUrl(recipe.homepage).host().toHtmlEscaped()));
        home->setOpenExternalLinks(true);
        facts->addWidget(home);
    }
    if (!recipe.license.isEmpty())
        facts->addWidget(dim(text(recipe.license == QLatin1String("proprietary")
                                      ? tr("Licence: proprietary, not free software")
                                      : tr("Licence: %1").arg(recipe.license))));
    if (!recipe.keeps.isEmpty())
        facts->addWidget(dim(text(tr("Keeps its settings in %1").arg(recipe.keeps.join(tr(", "))))));
    facts->addStretch();
    about->column()->addLayout(facts);
    column->addWidget(about);
    column->addStretch();
}

// ---------------------------------------------------------------- A run

void RunPage::begin(const Catalog &catalog, const QString &name, Run *run, Place place)
{
    catalog_ = catalog;
    name_ = name;
    run_ = run;
    marks_.clear();
    agentNote_.clear();
    stepsTotal_ = stepsStarted_ = 0;
    const Recipe *found = catalog_.recipe(name);
    const Recipe recipe = found ? *found : Recipe{};
    const InstalledItem *installed = catalog_.installedItem(name, place);
    updatingFrom_ = installed ? installed->version : QString();

    column_ = fresh();
    QHBoxLayout *header = recipeHeader(
        recipe, updatingFrom_.isEmpty() ? tr("Installing %1").arg(recipe.shown()) : tr("Updating %1").arg(recipe.shown()),
        &title_);
    auto *actions = new QVBoxLayout;
    actions->setSpacing(6);
    stop_ = new QPushButton(tr("Stop"));
    connect(stop_, &QPushButton::clicked, this, [this, run] {
        stop_->setEnabled(false);
        run->stop();
    });
    actions->addWidget(stop_, 0, Qt::AlignRight);
    stopNote_ = dim(text(tr("Stopping keeps what is already fetched and built")));
    actions->addWidget(stopNote_);
    header->addLayout(actions);
    column_->addLayout(header);

    auto *planPanel = new Panel;
    plan_ = new QGridLayout;
    plan_->setHorizontalSpacing(10);
    plan_->setVerticalSpacing(10);
    plan_->setColumnStretch(7, 1);
    planPanel->column()->addLayout(plan_);
    bar_ = new QProgressBar;
    bar_->setRange(0, 0);
    bar_->setTextVisible(false);
    bar_->setFixedHeight(8);
    planPanel->column()->addWidget(bar_);
    state_ = dim(text(tr("Starting")));
    planPanel->column()->addWidget(state_);
    column_->addWidget(planPanel);

    outputTitle_ = strong(tr("Output"));
    column_->addWidget(outputTitle_);
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
    emit status(tr("Starting %1").arg(recipe.shown()));
}

void RunPage::addPlanRow(const QString &recipe, const QStringList &hooks)
{
    const int row = plan_->rowCount();
    const Recipe *found = catalog_.recipe(recipe);
    QLabel *name = recipe == name_ ? strong(found ? found->shown() : recipe) : dim(text(recipe));
    name->setMinimumWidth(110);
    plan_->addWidget(name, row, 0);
    int column = 1;
    for (int index = 0; index < hooks.size(); ++index) {
        auto *stepMark = new StepMark(index + 1);
        marks_.insert(recipe + ':' + hooks[index], stepMark);
        plan_->addWidget(stepMark, row, column++);
        QLabel *label = text(hookTitle(hooks[index]));
        label->setMinimumWidth(70);
        plan_->addWidget(label, row, column++);
    }
    stepsTotal_ += hooks.size();
}

StepMark *RunPage::mark(const QString &recipe, const QString &hook)
{
    // A `recipe` that sends no plan still gets its rows, as its steps appear.
    if (!marks_.contains(recipe + ':' + hook) && !marks_.contains(recipe + QStringLiteral(":acquire"))
        && !marks_.contains(recipe + QStringLiteral(":build")) && !marks_.contains(recipe + QStringLiteral(":install")))
        addPlanRow(recipe, {"acquire", "build", "install"});
    return marks_.value(recipe + ':' + hook);
}

void RunPage::onEvent(const QJsonObject &event)
{
    const QString kind = event["event"].toString();
    const QString recipe = event["recipe"].toString();
    const Recipe *found = catalog_.recipe(recipe);
    const QString shown = found ? found->shown() : recipe;

    if (kind == QLatin1String("plan")) {
        for (const QJsonValue &value : event["recipes"].toArray()) {
            QStringList hooks;
            for (const QJsonValue &hook : value.toObject()["steps"].toArray())
                hooks << hook.toString();
            addPlanRow(value.toObject()["name"].toString(), hooks);
        }
        // One recipe's own plan, once its checks have run: the steps it does
        // not name are not needed this time.
        if (!recipe.isEmpty() && event["steps"].isArray()) {
            QStringList needed;
            for (const QJsonValue &hook : event["steps"].toArray())
                needed << hook.toString();
            for (const QString &hook : {QStringLiteral("acquire"), QStringLiteral("build"), QStringLiteral("install")}) {
                StepMark *stepMark = needed.contains(hook) ? nullptr : mark(recipe, hook);
                if (stepMark)
                    stepMark->setState(StepMark::Skipped);
            }
        }
    } else if (kind == QLatin1String("step") || kind == QLatin1String("recipe-hook")) {
        const QString hook = event["hook"].toString();
        const QString outcome = event["status"].toString();
        if (hook != QLatin1String("acquire") && hook != QLatin1String("build") && hook != QLatin1String("install"))
            return;
        StepMark *stepMark = mark(recipe, hook);
        if (!stepMark)
            return;
        if (outcome.contains("run")) {
            stepMark->setState(StepMark::Running);
            ++stepsStarted_;
            bar_->setRange(0, 0);
            const QString doing = hook == QLatin1String("acquire") ? tr("Fetching %1").arg(shown)
                                  : hook == QLatin1String("build") ? tr("Building %1").arg(shown)
                                                                    : tr("Installing %1").arg(shown);
            state_->setText(doing);
            emit status(tr("%1, step %2 of %3").arg(doing).arg(stepsStarted_).arg(qMax(stepsTotal_, stepsStarted_)));
        } else if (outcome.contains("skip") || outcome == QLatin1String("satisfied")) {
            stepMark->setState(StepMark::Skipped);
        } else if (outcome.contains("fail")) {
            stepMark->setState(StepMark::Failed);
        } else if (outcome == QLatin1String("done") || outcome == QLatin1String("success")) {
            stepMark->setState(StepMark::Done);
        }
    } else if (kind == QLatin1String("fetch")) {
        const qint64 bytes = event["bytes"].toInteger(), total = event["total"].toInteger();
        if (total > 0) {
            bar_->setRange(0, int(total / 1024));
            bar_->setValue(int(bytes / 1024));
            state_->setText(tr("Fetching %1: %2 of %3").arg(shown, bytesText(bytes), bytesText(total)));
        } else {
            state_->setText(tr("Fetching %1: %2 so far").arg(shown, bytesText(bytes)));
        }
    } else if (kind == QLatin1String("progress")) {
        bar_->setRange(0, event["total"].toInt());
        bar_->setValue(event["done"].toInt());
        state_->setText(tr("Building %1: %2 of %3").arg(shown).arg(event["done"].toInt()).arg(event["total"].toInt()));
    } else if (kind == QLatin1String("say")) {
        state_->setText(event["text"].toString());
    } else if (kind == QLatin1String("agent")) {
        // The agent changed the recipe during this run: say what, where it can be seen.
        agentNote_ = event["summary"].toString();
        state_->setText(tr("The agent changed the recipe: %1").arg(agentNote_));
        output_->appendPlainText(tr("agent: %1").arg(agentNote_));
    }
}

void RunPage::onFinished(bool ok)
{
    emit status(QString());
    if (ok) {
        if (!agentNote_.isEmpty())
            emit note(tr("The agent changed the recipe: %1").arg(agentNote_));
        emit succeeded(name_);
        return;
    }
    const QJsonObject failure = run_->failure();
    const bool stoppedByUser = !stop_->isEnabled() && failure.isEmpty();
    const Recipe *found = catalog_.recipe(name_);
    const QString shown = found ? found->shown() : name_;
    const QString version = found ? found->version : QString();
    const QString name = name_;
    bar_->hide();
    stop_->hide();
    stopNote_->hide();
    state_->setText(stoppedByUser ? tr("Stopped") : tr("Stopped at this step"));
    title_->setText(shown);

    // What happened, and what it left.
    QString what;
    if (stoppedByUser)
        what = tr("You stopped it");
    else if (updatingFrom_.isEmpty())
        what = tr("Installing %1 was stopped").arg(shown);
    else
        what = tr("The update to %1 was stopped").arg(version);
    const QString unchanged = updatingFrom_.isEmpty()
                                  ? tr("Nothing on this computer was changed.")
                                  : tr("%1 %2 is still installed and works as before.").arg(shown, updatingFrom_);
    auto *verdict = new Verdict;
    verdict->setFrameShape(QFrame::NoFrame);
    verdict->column()->addWidget(strong(what));
    verdict->column()->addWidget(
        text(stoppedByUser ? tr("What was already fetched and built is kept.") : unchanged, true));
    column_->insertWidget(1, verdict);

    auto *buttons = new QHBoxLayout;
    auto *again = new QPushButton(tr("Try again"));
    auto *leave = new QPushButton(tr("Back"));
    connect(again, &QPushButton::clicked, this, [this, name] { emit retry(name); });
    connect(leave, &QPushButton::clicked, this, &RunPage::back);
    if (stoppedByUser) {
        buttons->addWidget(again);
        buttons->addWidget(leave);
        buttons->addStretch();
        verdict->column()->addLayout(buttons);
        emit failed(name_);
        return;
    }

    // Recipe's premise is that an agent does the fixing. The page puts together
    // what the agent is handed: the issue, the facts, and the expected result.
    const QJsonObject refused = failure["refused"].toObject();
    const QString hook = failure["hook"].toString();
    const QString doing = updatingFrom_.isEmpty() ? tr("Installing %1 %2").arg(shown, version)
                                                   : tr("Updating %1 from %2 to %3").arg(shown, updatingFrom_, version);
    QString issue;
    QStringList facts;
    if (!refused.isEmpty()) {
        const QString during = hook == QLatin1String("acquire") ? tr("fetching")
                               : hook == QLatin1String("build") ? tr("building") : tr("installing");
        QStringList declared;
        for (const QJsonValue &value : refused["declared"].toArray())
            declared << value.toString();
        issue = tr("%1 stopped while %2. The recipe tried to connect to %3, which is not on its list, so the "
                   "sandbox refused the connection.")
                    .arg(doing, during, endpointText(refused["endpoint"].toString()));
        facts << tr("The recipe allows: %1").arg(endpointsText(declared));
    } else {
        issue = hook.isEmpty() ? tr("%1 stopped.").arg(doing)
                               : tr("%1 stopped in the %2 step.").arg(doing, hookTitle(hook).toLower());
        if (!failure["reason"].toString().isEmpty())
            facts << tr("Recipe's own message: %1").arg(failure["reason"].toString());
    }
    if (found && !found->path.isEmpty())
        facts << tr("The recipe: %1").arg(found->path);
    const int lines = output_->toPlainText().isEmpty() ? 0 : output_->blockCount();
    if (lines)
        facts << tr("The output of the run: %1").arg(counted(lines, tr("%1 line"), tr("%1 lines")));
    facts << unchanged;

    auto *prompt = new PromptPanel(
        issue, facts,
        tr("%1 %2 installs and its own check passes. What the recipe may use stays as it is; if that has to "
           "change, say what and why, and stop.")
            .arg(shown, version));

    auto *fixNow = primary(tr("Let the agent fix it"));
    connect(fixNow, &QPushButton::clicked, this, [this, name, prompt] { emit fix(name, prompt->prompt()); });
    auto *showOutput = new QPushButton(tr("Show the output"));
    showOutput->setCheckable(true);
    connect(showOutput, &QPushButton::toggled, this, [this](bool on) {
        outputTitle_->setVisible(on);
        output_->setVisible(on);
    });
    buttons->addWidget(showOutput);
    buttons->addWidget(again);
    buttons->addWidget(leave);
    buttons->addStretch();
    buttons->addWidget(fixNow);
    prompt->column()->addLayout(buttons);
    column_->insertWidget(2, prompt);

    // The output stays one click away; the prompt already counts it among the facts.
    outputTitle_->hide();
    output_->hide();
    column_->addStretch();
    emit failed(name_);
}

// ---------------------------------------------------------------- Installed

namespace {

// Where a row is, in the words of its column.
QString whereCell(const InstalledItem &item)
{
    if (item.place == Place::Computer && item.leavesNext)
        return QObject::tr("%1, goes in the next system").arg(placeTitle(item.place));
    return placeTitle(item.place);
}

// A sentence about what the place means for this software, said only when
// there are two places to tell apart.
QString whereSentence(const InstalledItem &item)
{
    switch (item.place) {
    case Place::Me:
        return QObject::tr("It is in your own store: only you have it.");
    case Place::Computer:
        if (!item.inStore) // recab's list of a locked computer: the running system
            return item.leavesNext ? QObject::tr("It is in the running system, for everyone. The next system no longer holds it.")
                                   : QObject::tr("It is in the running system, for everyone.");
        return QObject::tr("It is on this computer, for everyone.");
    case Place::Next:
        break;
    }
    return QObject::tr("Only the next system holds it. It works after a restart.");
}

} // namespace

void InstalledPage::show(const Catalog &catalog, const QString &selectedName, int selectedPlace)
{
    QVBoxLayout *column = fresh();
    // What Recipe says the stores take; summed from the list for a store it
    // could not measure.
    qint64 size = catalog.activeBytes;
    if (!size)
        for (const InstalledItem &item : catalog.installed)
            size += item.sizeBytes;
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Installed")));
    QString summary;
    if (catalog.installed.isEmpty()) {
        summary = tr("Nothing is installed from recipes yet.");
    } else if (!catalog.twoPlaces) {
        summary = tr("%1, %2.").arg(counted(catalog.installed.size(), tr("%1 program"), tr("%1 programs")), bytesText(size));
    } else {
        // Sizes only where a store measured them: the running system of a locked computer has none.
        int me = 0, computer = 0, next = 0;
        for (const InstalledItem &item : catalog.installed)
            (item.place == Place::Me ? me : item.place == Place::Computer ? computer : next)++;
        summary = tr("%1: %2 for me, %3 for this computer.")
                      .arg(counted(catalog.installed.size() - next, tr("%1 program"), tr("%1 programs")),
                           QString::number(me), QString::number(computer));
        if (next)
            summary += tr(" %1 more only in the next system.").arg(next);
    }
    if (catalog.earlierGenerations)
        summary += tr(" Earlier versions kept beside them, to go back to, take %1 more.").arg(bytesText(catalog.earlierBytes));
    top->addWidget(dim(text(summary, true)));
    if (!catalog.computerProblem.isEmpty())
        top->addWidget(dim(text(tr("This computer's own software could not be read: %1").arg(catalog.computerProblem), true)));
    header->addLayout(top, 1);
    for (Place place : {Place::Me, Place::Computer}) {
        const Store &store = catalog.stores[int(place)];
        if (!store.has || !store.earlierGenerations)
            continue;
        auto *clearThem = new QPushButton(catalog.twoPlaces ? tr("Clear earlier versions, %1…").arg(place == Place::Me ? tr("for me") : tr("for this computer"))
                                                            : tr("Clear earlier versions…"));
        clearThem->setToolTip(tr("Deletes every earlier version Recipe keeps there. Nothing that is installed changes."));
        connect(clearThem, &QPushButton::clicked, this, [this, place] { emit clearEarlier(place); });
        header->addWidget(clearThem, 0, Qt::AlignBottom);
    }
    column->addLayout(header);
    if (catalog.installed.isEmpty()) {
        column->addStretch();
        return;
    }

    QTreeWidget *tree = table({tr("Name"), tr("Where"), tr("Version"), tr("Newer"), tr("Installed"), tr("Size"),
                               tr("Earlier")});
    auto shared = std::make_shared<Catalog>(catalog);
    bool anyDate = false, anyKept = false, anySize = false;
    for (const InstalledItem &item : catalog.installed) {
        const Recipe *recipe = catalog.recipe(item.name);
        const UpdateItem *update = catalog.update(item.name, item.place);
        auto *line = new QTreeWidgetItem(tree, {recipe ? recipe->shown() : item.name, whereCell(item), item.version,
                                                update ? (update->fromCollection ? tr("recipe has %1").arg(update->to)
                                                                                 : tr("%1 is ready").arg(update->to))
                                                       : QString(),
                                                QLocale().toString(item.installedOn, "d MMM yyyy"),
                                                item.sizeBytes ? bytesText(item.sizeBytes) : QString(),
                                                item.kept.isEmpty() ? (item.inStore ? tr("none") : QString()) : item.kept});
        anyDate = anyDate || item.installedOn.isValid();
        anyKept = anyKept || !item.kept.isEmpty();
        anySize = anySize || item.sizeBytes;
        line->setData(0, Qt::UserRole, item.name);
        line->setData(1, Qt::UserRole, int(item.place));
        line->setSizeHint(0, QSize(0, 32));
        if (item.name == selectedName && (selectedPlace < 0 || int(item.place) == selectedPlace) && !tree->currentItem())
            tree->setCurrentItem(line);
    }
    if (!tree->currentItem())
        tree->setCurrentItem(tree->topLevelItem(0));
    // A receipt written before Recipe recorded it has no date; recab's list has none of these.
    tree->setColumnHidden(1, !catalog.twoPlaces);
    tree->setColumnHidden(4, !anyDate);
    tree->setColumnHidden(5, !anySize);
    tree->setColumnHidden(6, !anyKept);
    tree->setMaximumHeight(tree->header()->sizeHint().height() + 6
                           + tree->sizeHintForRow(0) * int(qMax<qsizetype>(catalog.installed.size(), 4)));
    column->addWidget(tree);

    auto *chosen = new Panel;
    auto *row = new QHBoxLayout;
    chosen->column()->addLayout(row);
    column->addWidget(chosen);
    column->addStretch();

    auto fill = [this, row, shared](QTreeWidgetItem *line) {
        clear(row);
        if (!line)
            return;
        const QString name = line->data(0, Qt::UserRole).toString();
        const Place place = Place(line->data(1, Qt::UserRole).toInt());
        const InstalledItem *item = shared->installedItem(name, place);
        const Recipe *recipe = shared->recipe(name);
        const UpdateItem *update = shared->update(name, place);
        if (!item)
            return;
        auto *words = new QVBoxLayout;
        words->setSpacing(1);
        words->addWidget(strong(tr("%1 %2").arg(recipe ? recipe->shown() : name, item->version)));
        QString about = recipe ? recipe->description + tr(".") : QString();
        if (shared->twoPlaces)
            about += (about.isEmpty() ? QString() : tr(" ")) + whereSentence(*item);
        if (item->menu.isEmpty() && !item->programs.isEmpty())
            about += tr(" Run it in a terminal as %1.").arg(item->programs.join(tr(", ")));
        if (!item->kept.isEmpty())
            about += tr(" Version %1 is kept, so you can go back to it.").arg(item->kept);
        words->addWidget(dim(text(about, true)));
        row->addLayout(words, 1);
        // A command-line tool has nothing to open; it is used from a terminal.
        if (!item->menu.isEmpty()) {
            auto *open = new QPushButton(tr("Open"));
            connect(open, &QPushButton::clicked, this, [this, name, place] { emit this->open(name, place); });
            row->addWidget(open);
        }
        if (!item->kept.isEmpty() && item->inStore) {
            auto *older = new QPushButton(tr("Go back to %1").arg(item->kept));
            connect(older, &QPushButton::clicked, this, [this, name, place] { emit goBack(name, place); });
            row->addWidget(older);
        }
        auto *remove = new QPushButton(tr("Remove…"));
        connect(remove, &QPushButton::clicked, this, [this, name, place] { emit this->remove(name, place); });
        row->addWidget(remove);
        if (update && place != Place::Next) {
            auto *newer = primary(tr("Update to %1").arg(update->to));
            connect(newer, &QPushButton::clicked, this, [this, name, place] { emit this->update(name, place); });
            row->addWidget(newer);
        }
        emit selected(name, place);
    };
    connect(tree, &QTreeWidget::currentItemChanged, this, [fill](QTreeWidgetItem *line) { fill(line); });
    fill(tree->currentItem());
}

// ---------------------------------------------------------------- Updates

void UpdatesPage::show(const Catalog &catalog, const QString &selectedName, int selectedPlace)
{
    QVBoxLayout *column = fresh();
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Updates")));
    top->addWidget(dim(text(catalog.updates.isEmpty()
                                ? tr("Everything installed is at the newest version of its recipe.")
                                : counted(catalog.updates.size(), tr("%1 recipe has a newer version."), tr("%1 recipes have a newer version.")))));
    header->addLayout(top);
    header->addStretch();
    auto *check = new QPushButton(tr("Check for updates"));
    check->setToolTip(tr("Fetches every source's list again"));
    connect(check, &QPushButton::clicked, this, &UpdatesPage::check);
    header->addWidget(check, 0, Qt::AlignBottom);
    if (catalog.updates.size() > 1) {
        auto *all = primary(tr("Update all"));
        connect(all, &QPushButton::clicked, this, &UpdatesPage::updateAll);
        header->addWidget(all, 0, Qt::AlignBottom);
    }
    column->addLayout(header);
    if (!catalog.updateProblems.isEmpty())
        column->addWidget(dim(text(tr("Not looked up: %1.").arg(catalog.updateProblems.join(tr("; "))), true)));
    auto *system = dim(text(tr("The system itself is updated in System Updates.")));
    if (catalog.updates.isEmpty()) {
        column->addStretch();
        column->addWidget(system);
        return;
    }

    QTreeWidget *tree = table({tr("Name"), tr("Where"), tr("Version"), tr("What the newer recipe asks for")});
    auto shared = std::make_shared<Catalog>(catalog);
    for (const UpdateItem &item : catalog.updates) {
        const Recipe *recipe = catalog.recipe(item.name);
        auto *line = new QTreeWidgetItem(tree, {recipe ? recipe->shown() : item.name, placeTitle(item.place),
                                                item.sameVersion ? tr("%1, another recipe").arg(item.from)
                                                                 : tr("%1 → %2").arg(item.from, item.to),
                                                !item.changesKnown ? tr("Not compared")
                                                : item.asksMore() ? tr("More than before") : tr("Nothing new")});
        line->setData(0, Qt::UserRole, item.name);
        line->setData(1, Qt::UserRole, int(item.place));
        line->setSizeHint(0, QSize(0, 32));
        if (item.name == selectedName && (selectedPlace < 0 || int(item.place) == selectedPlace) && !tree->currentItem())
            tree->setCurrentItem(line);
    }
    if (!tree->currentItem())
        tree->setCurrentItem(tree->topLevelItem(0));
    tree->setColumnHidden(1, !catalog.twoPlaces);
    tree->setMaximumHeight(tree->header()->sizeHint().height() + 6
                           + tree->sizeHintForRow(0) * int(qMax<qsizetype>(catalog.updates.size(), 3)));
    column->addWidget(tree);

    auto *detail = new Panel;
    auto *inside = new QVBoxLayout;
    inside->setSpacing(10);
    detail->column()->addLayout(inside);
    column->addWidget(detail);
    column->addStretch();
    column->addWidget(system);

    auto fill = [this, inside, shared](QTreeWidgetItem *line) {
        clear(inside);
        if (!line)
            return;
        const QString name = line->data(0, Qt::UserRole).toString();
        const Place place = Place(line->data(1, Qt::UserRole).toInt());
        const UpdateItem *item = shared->update(name, place);
        const Recipe *recipe = shared->recipe(name);
        if (!item)
            return;
        const QString shown = recipe ? recipe->shown() : name;
        auto *title = new QVBoxLayout;
        title->setSpacing(0);
        auto *form = new QFormLayout;
        form->setHorizontalSpacing(16);
        form->setVerticalSpacing(7);
        form->addRow(dim(text(tr("Version"))),
                     text(item->sameVersion ? tr("%1 still; its source publishes another recipe for it").arg(item->from)
                                            : tr("%1 → %2").arg(item->from, item->to)));
        if (!item->changesKnown) {
            title->addWidget(strong(tr("What the newer recipe of %1 may use").arg(shown)));
            title->addWidget(dim(text(item->fromCollection
                                          ? tr("What version %1 was allowed to use is not known here, so the two cannot be "
                                               "compared.").arg(item->from)
                                          : tr("Version %1 was installed before Recipe recorded what a recipe may use, so the "
                                               "two cannot be compared. From the next update on they can.")
                                                .arg(item->from), true)));
            inside->addLayout(title);
            if (recipe) {
                form->addRow(dim(text(tr("Network"))),
                             text(recipe->needs.network.isEmpty() ? tr("none") : endpointsText(recipe->needs.network), true));
                if (recipe->needs.networkMaxBytes)
                    form->addRow(dim(text(tr("Download"))), text(tr("up to %1").arg(bytesText(recipe->needs.networkMaxBytes))));
                if (recipe->needs.memoryMib)
                    form->addRow(dim(text(tr("Memory"))), text(tr("up to %1").arg(bytesText(recipe->needs.memoryMib * 1048576))));
            }
            inside->addLayout(form);
            auto *onlyButton = new QHBoxLayout;
            auto *goNow = new QPushButton(tr("Update %1").arg(shown));
            connect(goNow, &QPushButton::clicked, this, [this, name, place] { emit update(name, place); });
            onlyButton->addWidget(goNow);
            onlyButton->addStretch();
            inside->addLayout(onlyButton);
            emit selected(name, place);
            return;
        }
        title->addWidget(strong(tr("What changed in %1's recipe").arg(shown)));
        title->addWidget(dim(text(tr("Compared with the recipe version %1 was installed from.").arg(item->from))));
        inside->addLayout(title);
        // Only what differs is named; a list says what is new, what is gone and what stays.
        for (const Change &change : item->changes) {
            if (!change.isList()) {
                const QString how = change.kind == QLatin1String("raised")    ? tr("more than before")
                                    : change.kind == QLatin1String("lowered") ? tr("less than before")
                                                                              : tr("changed");
                form->addRow(dim(text(change.title())),
                             strong(tr("%1 → %2  (%3)").arg(Change::amount(change.before), Change::amount(change.after), how)));
                continue;
            }
            const bool network = change.field == QLatin1String("network");
            auto shownList = [network](const QStringList &names) { return network ? endpointsText(names) : names.join(QObject::tr(", ")); };
            auto *lines = new QVBoxLayout;
            lines->setSpacing(1);
            if (!change.added().isEmpty())
                lines->addWidget(strong(tr("New: %1").arg(shownList(change.added()))));
            if (!change.removed().isEmpty())
                lines->addWidget(text(tr("No longer: %1").arg(shownList(change.removed())), true));
            if (!change.stays().isEmpty())
                lines->addWidget(dim(text(tr("As before: %1").arg(shownList(change.stays())), true)));
            form->addRow(dim(text(change.title())), lines);
        }
        form->addRow(dim(text(item->changes.isEmpty() ? tr("What it may use") : tr("Everything else"))),
                     dim(text(item->changes.isEmpty() ? tr("the same as before: it asks for nothing new") : tr("as before"))));
        inside->addLayout(form);
        auto *buttons = new QHBoxLayout;
        auto *go = new QPushButton(tr("Update %1").arg(shown));
        connect(go, &QPushButton::clicked, this, [this, name, place] { emit update(name, place); });
        buttons->addWidget(go);
        buttons->addStretch();
        inside->addLayout(buttons);
        emit selected(name, place);
    };
    connect(tree, &QTreeWidget::currentItemChanged, this, [fill](QTreeWidgetItem *line) { fill(line); });
    fill(tree->currentItem());
}

// ---------------------------------------------------------------- Remove

RemoveDialog::RemoveDialog(const Recipe &recipe, const InstalledItem &item, QWidget *parent)
    : QDialog(parent)
{
    setWindowTitle(tr("Remove %1").arg(recipe.shown()));
    auto *column = new QVBoxLayout(this);
    column->setContentsMargins(22, 20, 22, 18);
    column->setSpacing(12);
    QLabel *question = strong(tr("Remove %1 %2?").arg(recipe.shown(), item.version));
    QFont larger = question->font();
    larger.setPointSizeF(larger.pointSizeF() * 1.2);
    question->setFont(larger);
    column->addWidget(question);
    QString words;
    if (!item.inStore) {
        // On a locked computer the change goes into the next system.
        words = item.place == Place::Next
                    ? tr("It is taken out of the next system, which is the only one that holds it.")
                    : tr("It is taken out of the next system. The running system keeps it until the restart.");
    } else {
        words = recipe.menu.isEmpty()
                    ? tr("It is no longer installed.")
                    : tr("%1 leaves the applications menu and is no longer installed.").arg(recipe.shown());
        // Recipe keeps what it removes among the earlier versions.
        words += tr(" Recipe keeps its files among the earlier versions until those are cleared, on the Installed page.");
    }
    words += recipe.keeps.isEmpty()
                 ? tr(" What it wrote in your home folder stays.")
                 : tr(" What it wrote in your home folder stays: %1.").arg(recipe.keeps.join(tr(", ")));
    QLabel *explained = text(words, true);
    explained->setMinimumWidth(420);
    column->addWidget(explained);
    column->addStretch();
    auto *buttons = new QDialogButtonBox(QDialogButtonBox::Cancel);
    buttons->addButton(tr("Remove"), QDialogButtonBox::AcceptRole);
    buttons->button(QDialogButtonBox::Cancel)->setDefault(true);
    connect(buttons, &QDialogButtonBox::accepted, this, &QDialog::accept);
    connect(buttons, &QDialogButtonBox::rejected, this, &QDialog::reject);
    column->addWidget(buttons);
}

