cmake_minimum_required(VERSION 3.16)

project(playground)
set(APP_NAME ${PROJECT_NAME})

# TODO: Add a helper function to our CMake module which automatically
# handles some of this boilerplate for a "typical" Qt application
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Qml QuickControls2 QmlImportScanner QuickTest Test REQUIRED)

set(
    CPP_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/src/myobject.cpp
)

# Generate the C++ code
file(STRINGS "target/cxx-qt-gen/cpp_sources.txt" GEN_SOURCES)
# list(TRANSFORM GEN_SOURCES PREPEND "target/")

# Define our executable with our QML resource files
add_executable(${APP_NAME} "${CPP_SOURCES}" "${GEN_SOURCES}" "${RESOURCES}")

TARGET_COMPILE_DEFINITIONS(${APP_NAME} PRIVATE QT_QML_DEBUG>)

# Include generated sources
target_include_directories(${APP_NAME} PUBLIC "target")
# Our cxx_qt and cxx headers are in this folder and need to be included
target_include_directories(${APP_NAME} PUBLIC "target/cxx-qt-gen/statics")
target_include_directories(${APP_NAME} PUBLIC "target/cxx-qt-gen/include")
target_include_directories(${APP_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/src")

# Link to generated rust library
set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
find_package(Threads REQUIRED)
target_link_directories(${APP_NAME} PRIVATE "target/debug")
target_link_libraries(${APP_NAME} PRIVATE rust Threads::Threads ${CMAKE_DL_LIBS})


target_link_libraries(${APP_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::QuickControls2
    Qt${QT_VERSION_MAJOR}::Test
)

