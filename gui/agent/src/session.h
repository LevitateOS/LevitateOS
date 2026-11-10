// One run, seen from the app's side: the request and answers arrive on
// standard input, events leave on standard error, and every event is also
// written to the record.
#pragma once

#include "record.h"
#include "settings.h"

#include <QJsonObject>
#include <QObject>

struct Request {
    QString prompt, app, title;
    QByteArray line; // as it came, for an agent that reads the apps' own protocol
};

class Adapter;
class QSocketNotifier;

class Session : public QObject {
    Q_OBJECT
public:
    Session(const Settings &settings, QObject *parent = nullptr);
    void setAdapter(Adapter *adapter) { adapter_ = adapter; }
    // Starts reading the app's input and watching for the signals that stop us.
    void run();
    const Settings &settings() const { return settings_; }
    // An event to the app and to the record.
    void send(const QJsonObject &event);
    // A line from an agent that already speaks the protocol: it goes to the
    // app as it is, and into the record as what it says.
    void forward(const QByteArray &line);
    // Ends the run: writes the last line of the record and exits.
    void finish(const QString &state, int exitCode);
    bool finished() const { return finished_; }

private:
    void onInput();
    void onSignal();
    void handle(const QByteArray &line);
    void toApp(const QByteArray &bytes);

    const Settings &settings_;
    Record record_;
    Adapter *adapter_ = nullptr;
    QSocketNotifier *input_ = nullptr, *signals_ = nullptr;
    QByteArray pending_;
    bool started_ = false, finished_ = false;
};
