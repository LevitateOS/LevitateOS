# All three desktop apps in one build, for the image: configure, build and
# install this folder and a system tree gets the three programs, their menu
# entries and their polkit actions. Each app's own folder still builds alone.
cmake_minimum_required(VERSION 3.21)
project(levitate-desktop-apps VERSION 0.1 LANGUAGES CXX)

add_subdirectory(recipe)
add_subdirectory(recab)
add_subdirectory(disk)
add_subdirectory(agent)
