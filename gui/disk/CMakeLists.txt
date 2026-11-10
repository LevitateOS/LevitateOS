cmake_minimum_required(VERSION 3.21)
project(disk-gui VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets Svg)

qt_add_executable(disk-gui
    src/main.cpp
    src/scan.cpp
    src/mess.cpp
    src/map.cpp
    src/pages.cpp
    src/systems.cpp
    src/window.cpp
    src/shots.cpp
    ../kit/parts.cpp
    ../kit/look.cpp
    ../kit/run.cpp
    ../kit/words.cpp
    ../kit/page.cpp
    ../kit/refresh.cpp
    ../kit/work.cpp
)
target_include_directories(disk-gui PRIVATE ../kit)
target_compile_options(disk-gui PRIVATE -Wall -Wextra)
target_link_libraries(disk-gui PRIVATE Qt6::Widgets Qt6::Svg)

include(GNUInstallDirs)
install(TARGETS disk-gui RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
install(FILES data/org.levitateos.disk.desktop DESTINATION ${CMAKE_INSTALL_DATADIR}/applications)
