cmake_minimum_required(VERSION 3.21)
project(recab-gui VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets Svg)

qt_add_executable(recab-gui
    src/main.cpp
    src/model.cpp
    src/client.cpp
    src/pages.cpp
    src/beforewords.cpp
    src/window.cpp
    src/shots.cpp
    ../kit/parts.cpp
    ../kit/look.cpp
    ../kit/run.cpp
    ../kit/words.cpp
    ../kit/page.cpp
    ../kit/before.cpp
    ../kit/refresh.cpp
    ../kit/work.cpp
    ../kit/frame.cpp
    ../kit/activity.cpp
    ../kit/settings.cpp
    ../kit/help.cpp
    ../kit/firstrun.cpp
    ../kit/helpstory.cpp
    ../kit/nextsystem.cpp
    ../kit/livestory.cpp
)
target_include_directories(recab-gui PRIVATE ../kit)
target_compile_options(recab-gui PRIVATE -Wall -Wextra)
target_link_libraries(recab-gui PRIVATE Qt6::Widgets Qt6::Svg)

include(GNUInstallDirs)
install(TARGETS recab-gui RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
install(FILES data/org.levitateos.recab.desktop DESTINATION ${CMAKE_INSTALL_DATADIR}/applications)
install(FILES data/org.levitateos.recab.policy DESTINATION ${CMAKE_INSTALL_DATADIR}/polkit-1/actions)
