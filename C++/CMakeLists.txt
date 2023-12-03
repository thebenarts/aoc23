cmake_minimum_required(VERSION 3.17)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(C++)

#======================= INCLUSION OF Our Code ======================#
set(SOURCE_DIR "${CMAKE_SOURCE_DIR}/src")

file(GLOB_RECURSE SOURCE_FILES
    "${SOURCE_DIR}/*.cpp"
)

# add the data to the target
add_executable(C++ "${SOURCE_FILES}")

set_property(DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR} PROPERTY VS_STARTUP_PROJECT C++)
