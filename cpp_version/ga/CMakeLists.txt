cmake_minimum_required(VERSION 3.0.0)

project(ga)

if(MSVC)
	add_definitions(/std:c++17 /permissive)
else()
	add_definitions(-std=c++17)
endif()

set(M_SRC main.cpp)

add_executable(ga ${M_SRC})



