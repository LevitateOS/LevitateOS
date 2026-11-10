cmake_minimum_required(VERSION 3.21)
project(recipe-gui VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)

find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets Svg)

qt_add_executable(recipe-gui
    src/main.cpp
    src/model.cpp
    src/client.cpp
    ../kit/parts.cpp
    ../kit/look.cpp
    ../kit/run.cpp
    ../kit/words.cpp
    ../kit/page.cpp
    ../kit/refresh.cpp
    ../kit/work.cpp
    src/pages.cpp
    src/sources.cpp
    src/window.cpp
    src/shots.cpp
)
target_include_directories(recipe-gui PRIVATE ../kit)
target_compile_options(recipe-gui PRIVATE -Wall -Wextra)
target_link_libraries(recipe-gui PRIVATE Qt6::Widgets Qt6::Svg)

include(GNUInstallDirs)
install(TARGETS recipe-gui RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
install(FILES data/org.levitateos.recipe.desktop DESTINATION ${CMAKE_INSTALL_DATADIR}/applications)
install(FILES data/org.levitateos.recipe.policy DESTINATION ${CMAKE_INSTALL_DATADIR}/polkit-1/actions)
