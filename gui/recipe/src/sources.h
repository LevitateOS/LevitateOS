// Two pages about where recipes come from: the list of sources, and asking
// an agent to write a recipe that no source has.
#pragma once

#include "model.h"
#include "page.h"

class PromptPanel;

// The sources: what each is, when its list was fetched, and what can be done.
class SourcesPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog);
signals:
    // Fetch one source's list again, or all when the name is empty.
    void refresh(const QString &name);
    void add(const QString &addressOrFolder);
    void remove(const QString &name);
    void browse(const QString &name);
};

// The folder an agent writes a person's own recipes into: the "local"
// source's folder, else Recipe's own place for them under the data folder.
QString ownRecipesFolder(const Catalog &catalog);

// Say what software is wanted; an agent writes the recipe.
class WishPage : public Page {
    Q_OBJECT
public:
    using Page::Page;
    void show(const Catalog &catalog, const QString &wish);
signals:
    void back();
    void handOver(const QString &prompt);
};
