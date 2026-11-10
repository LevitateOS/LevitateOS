#include "child.h"

#include <csignal>
#include <unistd.h>

void Child::startInGroup()
{
    setChildProcessModifier([] { setsid(); });
    connect(this, &QProcess::finished, this, [this] { group_ = 0; });
    start();
    if (state() != QProcess::NotRunning)
        group_ = processId();
}

void Child::endGroup()
{
    if (group_ > 0)
        ::kill(-group_, SIGTERM);
    if (state() != QProcess::NotRunning)
        waitForFinished(1500);
    // Whatever the leader left behind in the group goes too.
    if (group_ > 0)
        ::kill(-group_, SIGKILL);
    if (state() != QProcess::NotRunning) {
        QProcess::kill();
        waitForFinished(500);
    }
    group_ = 0;
}

Words splitCommand(const QString &line)
{
    Words words;
    static const QString syntax = QStringLiteral("|;&<>`\n");
    for (const QChar c : line) {
        if (syntax.contains(c)) {
            words.problem = QStringLiteral("it uses shell syntax (%1)").arg(c == QLatin1Char('\n') ? QStringLiteral("a line break") : QString(c));
            return words;
        }
    }
    if (line.contains(QLatin1String("$("))) {
        words.problem = QStringLiteral("it uses shell syntax ($( ))");
        return words;
    }

    QString current;
    bool inWord = false;
    QChar quote;
    for (qsizetype i = 0; i < line.size(); ++i) {
        const QChar c = line[i];
        if (quote.isNull() && c.isSpace()) {
            if (inWord)
                words.list << current;
            current.clear();
            inWord = false;
        } else if (quote.isNull() && (c == QLatin1Char('\'') || c == QLatin1Char('"'))) {
            quote = c;
            inWord = true;
        } else if (!quote.isNull() && c == quote) {
            quote = QChar();
        } else if (c == QLatin1Char('\\') && quote != QLatin1Char('\'') && i + 1 < line.size()) {
            current += line[++i];
            inWord = true;
        } else {
            current += c;
            inWord = true;
        }
    }
    if (!quote.isNull()) {
        words.problem = QStringLiteral("a quote is not closed");
        return words;
    }
    if (inWord)
        words.list << current;
    if (words.list.isEmpty()) {
        words.problem = QStringLiteral("it is empty");
        return words;
    }
    words.ok = true;
    return words;
}
