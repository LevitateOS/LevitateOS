// Small widgets the pages are made of. Colours come from the desktop's
// palette only, so the app looks like Plasma on LevitateOS and like LXQt on
// AcornOS without a line of theme code.
#pragma once

#include <QFrame>
#include <QLabel>
#include <QPalette>
#include <QWidget>

class QVBoxLayout;

QLabel *text(const QString &words, bool wrap = false);
QLabel *dim(QLabel *label);
QLabel *strong(const QString &words);
QLabel *heading(const QString &words);
QWidget *rule();

// A framed box on the view colour: a list, a group of facts.
class Panel : public QFrame {
public:
    explicit Panel(QWidget *parent = nullptr);
    QVBoxLayout *column();
};

// A panel whose outline says what it holds: the thing in use, something
// still open, something ready, something that went wrong.
class Card : public Panel {
public:
    enum Tone { Plain, On, Open, Ready, Wrong };
    explicit Card(Tone tone, QWidget *parent = nullptr);
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    Tone tone_;
};

// A small drawn picture for a card. Drawn, not typed or loaded, so it needs
// no icon theme and no font.
class Glyph : public QWidget {
public:
    enum Kind { Lock, Plus, Before, Check, Cross };
    Glyph(Kind kind, bool filled, QWidget *parent = nullptr);
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    Kind kind_;
    bool filled_;
};

// A panel outlined in the colour for "this went wrong".
class Verdict : public Panel {
public:
    using Panel::Panel;
protected:
    void paintEvent(QPaintEvent *event) override;
};

// A label that acts as a link: click, or Tab to it and press Return.
class ClickLabel : public QLabel {
    Q_OBJECT
public:
    explicit ClickLabel(const QString &words, QWidget *parent = nullptr);
    // Looks like a link: underlined, in the palette's link colour, and a
    // focus frame when Tab reaches it (the underline then stays).
    void setLinkLook();
signals:
    void clicked();
protected:
    void mouseReleaseEvent(QMouseEvent *event) override;
    void keyPressEvent(QKeyEvent *event) override;
    void focusInEvent(QFocusEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
    void paintEvent(QPaintEvent *event) override;
private:
    bool link_ = false;
};

// A ClickLabel with the link look: a way to another page.
ClickLabel *linkLabel(const QString &words);

// A program's picture: its icon when the recipe has one, else its first letter.
class Tile : public QWidget {
public:
    Tile(const QString &name, const QString &iconPath, int size, QWidget *parent = nullptr);
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    QString letter_, iconPath_;
};

// What an agent is handed when something needs fixing or changing: the issue,
// the facts, and the expected result. The person may rewrite the last part;
// it is the only part that is theirs to say.
class QPlainTextEdit;
class PromptPanel : public Panel {
public:
    PromptPanel(const QString &issue, const QStringList &facts, const QString &expected, QWidget *parent = nullptr);
    // The three parts as the text an agent reads.
    QString prompt() const;
private:
    QString issue_;
    QStringList facts_;
    QPlainTextEdit *expected_;
};

// The round mark in front of a step: its number, a tick, or a cross.
class StepMark : public QWidget {
public:
    enum State { Pending, Running, Done, Skipped, Failed };
    explicit StepMark(int number, QWidget *parent = nullptr);
    void setState(State state);
    State state() const { return state_; }
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    int number_;
    State state_ = Pending;
};

// What a short piece of state says: "this needs you", "this went well", "this
// went wrong". The colours follow the palette's lightness, so the words stay
// readable on the dark Levitate look and on the light Acorn one.
enum class Mood { Plain, Warn, Ok, Bad };
QColor moodColour(const QPalette &palette, Mood mood);

// A label that never makes its line wider than the room it is given: what
// does not fit ends in an ellipsis, and the whole text is its tooltip.
class ElidedLabel : public QLabel {
public:
    explicit ElidedLabel(const QString &words, QWidget *parent = nullptr);
    void setMood(Mood mood);
    QSize minimumSizeHint() const override { return {0, QLabel::minimumSizeHint().height()}; }
protected:
    void paintEvent(QPaintEvent *event) override;
};

// A small outlined count or state ("1 running"), in the colour of its mood.
class Chip : public QLabel {
public:
    explicit Chip(const QString &words, Mood mood = Mood::Plain, QWidget *parent = nullptr);
    QSize sizeHint() const override;
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    Mood mood_;
};
