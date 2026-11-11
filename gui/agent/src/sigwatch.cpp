#include "sigwatch.h"

#include <QSocketNotifier>

#include <csignal>
#include <fcntl.h>
#include <unistd.h>

static int pipeEnds[2] = {-1, -1};

static void onSignalNumber(int number)
{
    const char byte = char(number);
    [[maybe_unused]] const ssize_t written = ::write(pipeEnds[1], &byte, 1);
}

SigWatch::SigWatch(std::initializer_list<int> numbers, QObject *parent)
    : QObject(parent)
{
    if (::pipe2(pipeEnds, O_CLOEXEC | O_NONBLOCK) != 0)
        return;
    struct sigaction action {};
    action.sa_handler = onSignalNumber;
    sigemptyset(&action.sa_mask);
    for (int number : numbers)
        sigaction(number, &action, nullptr);
    notifier_ = new QSocketNotifier(pipeEnds[0], QSocketNotifier::Read, this);
    connect(notifier_, &QSocketNotifier::activated, this, [this] {
        char byte;
        while (::read(pipeEnds[0], &byte, 1) > 0)
            emit raised(byte);
    });
}
