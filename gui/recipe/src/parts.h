// Small widgets the pages are made of. Colours come from the desktop's
// palette only, so the app looks like Plasma on LevitateOS and like LXQt on
// AcornOS without a line of theme code.
#pragma once

#include <QFrame>
#include <QLabel>
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
signals:
    void clicked();
protected:
    void mouseReleaseEvent(QMouseEvent *event) override;
    void keyPressEvent(QKeyEvent *event) override;
    void focusInEvent(QFocusEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
};

// A program's picture: its icon when the recipe has one, else its first letter.
class Tile : public QWidget {
public:
    Tile(const QString &name, const QString &iconPath, int size, QWidget *parent = nullptr);
protected:
    void paintEvent(QPaintEvent *event) override;
private:
    QString letter_, iconPath_;
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
