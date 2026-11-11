// One run, in the process that hosts it. The request arrives once on standard
// input (from the helper the window started); after that, windows come and go
// over a Unix socket. Every event is written to the record, and sent to
// whatever windows are attached at that moment. The run does not end when the
// last window leaves.
#pragma once

#include "holding.h"
#include "record.h"
#include "settings.h"

#include <QElapsedTimer>
#include <QJsonObject>
#include <QList>
#include <QObject>
#include <QStringList>

struct Request {
    QString prompt, app, title;
    QByteArray line; // as it came, for an agent that reads the apps' own protocol
    // Folders the agent may write in, beyond what the settings give it: made
    // absolute and checked to lie in the person's home folder.
    QStringList mayWrite;
    // Places the person approved for setting aside, with the holding folder
    // the helper moves them into (the agent only decides); `given` is false
    // when the request has none.
    SetAsidePlan setAside;
};

class Adapter;
class Link;
class Listener;
class QSocketNotifier;
class SigWatch;

class Session : public QObject {
    Q_OBJECT
public:
    Session(const Settings &settings, const QString &id, qint64 starter, QObject *parent = nullptr);
    void setAdapter(Adapter *adapter) { adapter_ = adapter; }
    // Starts reading the request and watching for the signals that stop us.
    void run();
    const Settings &settings() const { return settings_; }
    // An event to the attached windows and to the record.
    void send(const QJsonObject &event);
    // A line from an agent that already speaks the protocol: it goes to the
    // windows as it is, and into the record as what it says.
    void forward(const QByteArray &line);
    // What such an agent wrote on its standard output, which belongs to the
    // window that started the run.
    void forwardOutput(const QByteArray &bytes);
    // Ends the run: writes the last line of the record, tells the windows,
    // shows a notice if none is watching, and exits. `reason` is for the
    // notice of a run that was stopped.
    void finish(const QString &state, int exitCode, const QString &reason = QString());
    bool finished() const { return finished_; }

private:
    struct Client {
        Link *link;
        bool attach; // reads events as they are, with their time; a relay reads what the app always read
    };

    void onInput();
    void begin(const QByteArray &line);
    void onAccepted(int fd);
    void onClientLine(Link *link, const QByteArray &line);
    void onClientClosed(Link *link);
    void replay(const Client &client);
    void answer(const QByteArray &line);
    void stopRequested(const QString &reason);
    void note(const QJsonObject &event);
    // What the windows are told of an event: `app` is the line the apps always
    // read, `stamped` the event as the record holds it.
    void broadcast(const QByteArray &app, const QJsonObject &stamped);
    void control(const QJsonObject &message);
    bool nobodyWatching() const;
    void tell(const QString &text);
    QString noticeTitle() const;

    const Settings &settings_;
    Record record_;
    const QString id_;
    const qint64 starter_;
    Adapter *adapter_ = nullptr;
    QSocketNotifier *input_ = nullptr;
    SigWatch *signals_ = nullptr;
    Listener *listener_ = nullptr;
    QList<Client> clients_;
    QList<Link *> greeting_; // connected, not yet said what they are
    QByteArray pending_;
    Request request_;
    QElapsedTimer age_;
    // The one question that waits, if any: whoever answers first is heard.
    bool asking_ = false, asked_ = false;
    QString question_;
    QString summary_, failure_, state_;
    bool started_ = false, finished_ = false, everAttached_ = false;
    int exitCode_ = 0;
};
