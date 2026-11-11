// The talk between a run's process and the windows (clients) attached to it:
// lines of JSON over a Unix socket. Raw sockets, not QLocalSocket, because
// that is in QtNetwork and this program is QtCore only.
#pragma once

#include <QByteArray>
#include <QObject>
#include <QString>

class QSocketNotifier;

// One end of a connected stream, cut into lines.
class Link : public QObject {
    Q_OBJECT
public:
    explicit Link(int fd, QObject *parent = nullptr);
    ~Link() override;
    // One line; the line break is added here.
    void send(const QByteArray &line);
    // Closes once everything queued has gone out.
    void closeWhenSent();
    void close();
    bool isOpen() const { return fd_ >= 0; }

signals:
    void line(const QByteArray &line);
    // Once, when the other end went away or close() was called.
    void closed();

private:
    void readable();
    void writable();

    int fd_;
    QByteArray in_, out_;
    QSocketNotifier *reader_ = nullptr, *writer_ = nullptr;
    bool closing_ = false;
};

// Waits for connections on a listening socket.
class Listener : public QObject {
    Q_OBJECT
public:
    Listener(int fd, QObject *parent = nullptr);
    ~Listener() override;

signals:
    void accepted(int fd);

private:
    int fd_;
    QSocketNotifier *notifier_;
};

namespace sockets {
// Listens at FOLDER/NAME (the folder is made, for the person alone). Returns
// the descriptor, or -1 with the reason in `problem`.
int listenAt(const QString &folder, const QString &name, QString *problem);
// Connects to FOLDER/NAME; -1 when nothing is listening there.
int connectTo(const QString &folder, const QString &name);
bool exists(const QString &folder, const QString &name);
void remove(const QString &folder, const QString &name);
} // namespace sockets
