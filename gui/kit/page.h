// The base of every page: it is told what to show and rebuilds itself.
#pragma once

#include <QWidget>

class QVBoxLayout;

class Page : public QWidget {
protected:
    explicit Page(QWidget *parent = nullptr);
    // Drops what the page showed and returns an empty column to fill.
    QVBoxLayout *fresh();
private:
    QVBoxLayout *outer_;
    QWidget *body_ = nullptr;
};
