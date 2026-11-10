// The base of every page: it is told what to show and rebuilds itself. While
// what it shows is still being read, it says so.
#pragma once

#include <QWidget>

class QVBoxLayout;

class Page : public QWidget {
public:
    // Something is being read: a title, a line of words, a moving bar.
    void showLoading(const QString &title, const QString &words);
protected:
    explicit Page(QWidget *parent = nullptr);
    // Drops what the page showed and returns an empty column to fill.
    QVBoxLayout *fresh();
private:
    QVBoxLayout *outer_;
    QWidget *body_ = nullptr;
};
