cmake_minimum_required(VERSION 3.16)

project(snake-ai-v2 VERSION 1.0.1)

set(CMAKE_CXX_STANDARD 17)

include(external/vcpkg/scripts/buildsystems/vcpkg.cmake)

find_package(GLEW CONFIG REQUIRED)
find_package(glfw3 CONFIG REQUIRED)
find_package(imgui CONFIG REQUIRED)

file(GLOB_RECURSE var_src src/*.cpp external/imgui/*.cpp)

add_executable(snake-ai-v2 ${var_src}) 

# add_compile_definitions(PROJECT_ROOT_DIR="${CMAKE_SOURCE_DIR}")

target_link_libraries(snake-ai-v2 PRIVATE glfw GLEW::GLEW imgui::imgui)