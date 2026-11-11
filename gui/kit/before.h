// A page that says what will happen before anything starts: a question as the
// title, "Nothing has started...", then the sections the app fills, then
// Cancel and one named button. It knows nothing about recipes or disks; the
// app puts its own words in. When the choice changes the page draws itself
// again from the content it was given and says so (`choiceChanged`), so an app
// whose sentences depend on the choice can give it new content at once.
#pragma once

#include "page.h"

#include <QList>
#include <QPointer>
#include <QScrollArea>
#include <QPair>
#include <QStringList>

class BeforePage : public Page {
    Q_OBJECT
public:
    // One place the thing can go: a name and two lines under it.
    struct Choice {
        QString key, name, first, second;
    };

    struct Content {
        // A question: "Install Krita?"
        QString title;
        // The places. Two or more: the person chooses, `chosen` is picked.
        // One: it is shown as a statement, nothing to choose. None: no section.
        QString whereTitle;
        QList<Choice> choices;
        QString chosen;
        // Plain sentences, one per line.
        QString happensTitle;
        QStringList happens;
        // Two columns, then a line under them.
        QString tableTitle;
        QList<QPair<QString, QString>> table;
        QString tableNote;
        // The foot: a line, and one link after it (the link is shown only
        // when `linkWords` is not empty).
        QString foot, linkWords;
        // The one button that goes on (Cancel is always there).
        QString go;
    };

    explicit BeforePage(QWidget *parent = nullptr);
    void show(const Content &content);
    // The key picked now (empty when there is no choice section).
    QString chosen() const { return content_.chosen; }

    // For a check: all the words on the page, and the names of the choices.
    QString pageText() const;
    QStringList choiceNames() const;
    // Picks a choice as a click would.
    void choose(const QString &key);

signals:
    void cancelled();
    // The one button, with what is chosen (empty without a choice).
    void confirmed(const QString &choice);
    void choiceChanged(const QString &choice);
    void linkClicked();

private:
    Content content_;
    // Weak: the page's body is deleted after it is replaced.
    QPointer<QScrollArea> scroll_;
};
