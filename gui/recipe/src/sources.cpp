#include "sources.h"

#include "parts.h"

#include <QDesktopServices>
#include <QDir>
#include <QFileInfo>
#include <QFontDatabase>
#include <QHBoxLayout>
#include <QInputDialog>
#include <QPlainTextEdit>
#include <QPushButton>
#include <QScrollArea>
#include <QUrl>

// ---------------------------------------------------------------- The sources

void SourcesPage::show(const Catalog &catalog)
{
    QVBoxLayout *column = fresh();
    auto *header = new QHBoxLayout;
    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("Sources")));
    top->addWidget(dim(text(tr("Where recipes come from: repositories whose list is fetched, and folders on this "
                               "computer. Nothing is installed from a source until you ask."),
                            true)));
    header->addLayout(top, 1);
    auto *all = new QPushButton(tr("Fetch all again"));
    connect(all, &QPushButton::clicked, this, [this] { emit refresh(QString()); });
    header->addWidget(all, 0, Qt::AlignBottom);
    auto *more = new QPushButton(tr("Add a source…"));
    connect(more, &QPushButton::clicked, this, [this] {
        bool ok = false;
        const QString where = QInputDialog::getText(
            this, tr("Add a source"),
            tr("The address of a repository of recipes, or a folder on this computer:"), QLineEdit::Normal, QString(), &ok);
        if (ok && !where.trimmed().isEmpty())
            emit add(where.trimmed());
    });
    header->addWidget(more, 0, Qt::AlignBottom);
    column->addLayout(header);

    auto *inner = new QWidget;
    auto *list = new QVBoxLayout(inner);
    list->setContentsMargins(0, 0, 0, 0);
    list->setSpacing(10);
    for (const Origin &origin : catalog.origins) {
        const QString name = origin.name;
        auto *panel = new Panel;
        auto *row = new QHBoxLayout;
        auto *words = new QVBoxLayout;
        words->setSpacing(2);
        QString title = origin.shown();
        title[0] = title[0].toUpper();
        words->addWidget(strong(title == origin.name ? title : tr("%1 (%2)").arg(title, origin.name)));
        // A folder's path is a path: shown as one.
        QLabel *where = text(origin.where(), true);
        if (!origin.remote())
            where->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
        dim(where);
        words->addWidget(where);
        const int recipes = catalog.countFrom(origin.name);
        QLabel *state = text(tr("%1. %2.").arg(counted(recipes, tr("%1 recipe"), tr("%1 recipes")), origin.state()), true);
        if (origin.problem.isEmpty())
            dim(state);
        words->addWidget(state);
        row->addLayout(words, 1);
        if (recipes) {
            auto *look = new QPushButton(tr("Show its recipes"));
            connect(look, &QPushButton::clicked, this, [this, name] { emit browse(name); });
            row->addWidget(look, 0, Qt::AlignTop);
        }
        if (!origin.remote() && !origin.path.isEmpty()) {
            auto *folder = new QPushButton(tr("Open the folder"));
            const QString path = origin.path;
            connect(folder, &QPushButton::clicked, this, [path] { QDesktopServices::openUrl(QUrl::fromLocalFile(path)); });
            row->addWidget(folder, 0, Qt::AlignTop);
        }
        if (origin.remote()) {
            auto *again = new QPushButton(tr("Fetch again"));
            connect(again, &QPushButton::clicked, this, [this, name] { emit refresh(name); });
            row->addWidget(again, 0, Qt::AlignTop);
        }
        // The two folders Recipe always reads are not sources that can be removed.
        if (name != QLatin1String("local") && name != QLatin1String("image")) {
            auto *drop = new QPushButton(tr("Remove…"));
            connect(drop, &QPushButton::clicked, this, [this, name] { emit remove(name); });
            row->addWidget(drop, 0, Qt::AlignTop);
        }
        panel->column()->addLayout(row);
        list->addWidget(panel);
    }
    if (catalog.origins.isEmpty())
        list->addWidget(dim(text(tr("recipe named no source."))));
    // Recipes `recipe list` could not read: named with its own reason, and not offered.
    if (!catalog.unreadableRecipes.isEmpty()) {
        auto *problems = new Panel;
        auto *title = new QVBoxLayout;
        title->setSpacing(0);
        title->addWidget(strong(counted(catalog.unreadableRecipes.size(), tr("%1 recipe could not be read"),
                                        tr("%1 recipes could not be read"))));
        title->addWidget(dim(text(tr("They are not offered until they can be."))));
        problems->column()->addLayout(title);
        for (const RecipeProblem &problem : catalog.unreadableRecipes) {
            auto *row = new QHBoxLayout;
            row->setSpacing(14);
            auto *file = new QLabel(QFileInfo(problem.path).fileName());
            file->setFont(QFontDatabase::systemFont(QFontDatabase::FixedFont));
            file->setMinimumWidth(130);
            row->addWidget(file, 0, Qt::AlignTop);
            row->addWidget(text(problem.problem, true), 1);
            auto *read = new QPushButton(tr("Read the file"));
            const QString path = problem.path;
            connect(read, &QPushButton::clicked, this, [path] { QDesktopServices::openUrl(QUrl::fromLocalFile(path)); });
            row->addWidget(read, 0, Qt::AlignTop);
            problems->column()->addLayout(row);
        }
        list->addWidget(problems);
    }
    list->addStretch();
    auto *scroll = new QScrollArea;
    scroll->setWidgetResizable(true);
    scroll->setFrameShape(QFrame::NoFrame);
    scroll->setWidget(inner);
    column->addWidget(scroll, 1);
}

// ---------------------------------------------------------------- Asking for a recipe

QString ownRecipesFolder(const Catalog &catalog)
{
    const Origin *own = catalog.origin(QStringLiteral("local"));
    if (own && !own->path.isEmpty())
        return own->path;
    const QString data = qEnvironmentVariable("XDG_DATA_HOME", QDir::homePath() + QStringLiteral("/.local/share"));
    return data + QStringLiteral("/recipe/recipes");
}

void WishPage::show(const Catalog &catalog, const QString &wish)
{
    QVBoxLayout *column = fresh();
    auto *backLink = new ClickLabel(tr("‹ All software"));
    dim(backLink);
    connect(backLink, &ClickLabel::clicked, this, &WishPage::back);
    column->addWidget(backLink, 0, Qt::AlignLeft);

    auto *top = new QVBoxLayout;
    top->setSpacing(1);
    top->addWidget(heading(tr("What software do you want?")));
    top->addWidget(dim(text(tr("Say it in your own words: a name, or what it should do. An agent finds it and writes "
                               "the recipe. Nothing is installed: the recipe appears here for you to read first."),
                            true)));
    column->addLayout(top);

    auto *words = new QPlainTextEdit(wish);
    words->setAccessibleName(tr("The software you want"));
    words->setPlaceholderText(tr("For example: a file manager for the terminal."));
    words->setFixedHeight(74);
    column->addWidget(words);

    // Where a person's own recipes live: the folder Recipe reads first.
    const Origin *own = catalog.origin(QStringLiteral("local"));
    // The folder is what the agent is allowed to write in, and the page says so.
    top->addWidget(dim(text(tr("The agent may write in your own recipe folder only (%1). Anything else it wants done, it asks "
                               "you first, unless you allowed it more in Settings.")
                                .arg(ownRecipesFolder(catalog)),
                            true)));
    QStringList facts;
    facts << (own ? tr("Recipes written for this person go into %1, as NAME.rhai").arg(own->path)
                  : tr("Recipes written for this person go into Recipe's own folder for them, as NAME.rhai"));
    facts << tr("`recipe info FILE` reads a recipe and says what is wrong with it, in seconds, without running it");
    facts << tr("A recipe names what it downloads with a checksum, and says in ABOUT what the software is");
    QStringList have;
    for (const Recipe &recipe : catalog.recipes)
        have << recipe.name;
    facts << tr("Recipes that exist already: %1").arg(have.join(tr(", ")));
    auto *prompt = new PromptPanel(
        tr("The person asks for the software written above, and no recipe for it exists."), facts,
        tr("One recipe for it in that folder that `recipe info` accepts, for a released version, with checksums. "
           "Nothing is installed and nothing else is changed."));
    column->addWidget(prompt);
    column->addStretch();

    auto *buttons = new QHBoxLayout;
    auto *cancel = new QPushButton(tr("Cancel"));
    auto *hand = new QPushButton(tr("Hand it to the agent"));
    hand->setDefault(true);
    connect(cancel, &QPushButton::clicked, this, &WishPage::back);
    connect(hand, &QPushButton::clicked, this, [this, words, prompt] {
        const QString wanted = words->toPlainText().trimmed();
        if (!wanted.isEmpty())
            emit handOver(tr("The request:\n%1\n\n%2").arg(wanted, prompt->prompt()));
    });
    buttons->addStretch();
    buttons->addWidget(cancel);
    buttons->addWidget(hand);
    column->addLayout(buttons);
}
