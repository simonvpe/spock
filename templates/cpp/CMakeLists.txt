cmake_minimum_required(VERSION 3.8)
project({{ name }})

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS \"-Wall -Wextra\")
set(CMAKE_CXX_FLAGS_DEBUG \"-g\")
set(CMAKE_CXX_FLAGS_RELEASE \"-O3\")

include_directories(${CMAKE_SOURCE_DIR}/include)

add_subdirectory(src)
{% if testing != \"\" %}
add_subdirectory(test)
{% endif %}
