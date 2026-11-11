# levitate-agent: stands between the desktop apps and a real coding agent.
# No widgets; QtCore only, so it also runs where no display is.
cmake_minimum_required(VERSION 3.21)
project(levitate-agent VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Core)

qt_add_executable(levitate-agent
    src/main.cpp
    src/settings.cpp
    src/record.cpp
    src/session.cpp
    src/client.cpp
    src/link.cpp
    src/sigwatch.cpp
    src/notify.cpp
    src/child.cpp
    src/conversation.cpp
    src/codex.cpp
    src/claude.cpp
    src/passthrough.cpp
    src/status.cpp
    src/account.cpp
)
target_compile_options(levitate-agent PRIVATE -Wall -Wextra)
target_link_libraries(levitate-agent PRIVATE Qt6::Core)

include(GNUInstallDirs)
install(TARGETS levitate-agent RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
