cmake_minimum_required(VERSION 3.10...3.15)

# Setting these is bad practice but it's a very specific target
set(CMAKE_C_COMPILER "i686-w64-mingw32-gcc")

message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")

#TODO figure out linking and passing the compiler test to get C++ to work
#set(CMAKE_CXX_COMPILER "i686-w64-mingw32-gcc")

#TODO add C++ to the languages
project(Mech3fixup
    VERSION 0.0.2
    DESCRIPTION "Fixes to make mech3 more stable"
    LANGUAGES C)

set(C_WARN_FLAGS "-Wall -Wextra -Wfloat-conversion -Wshadow")
set(CXX_WARN_FLAGS "${C_WARN_FLAGS} -Weffc++")

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} ${C_WARN_FLAGS}")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${CXX_WARN_FLAGS}")
set(CMAKE_SHARED_LINKER_FLAGS "-static-libgcc ${CMAKE_SHARED_LINKER_FLAGS}")

set(DLL_SRC
    src/mech3fixup.c
    src/exports.c
)

add_library(mech3fix SHARED ${DLL_SRC})
add_compile_definitions(PROJECT_VERSION="${CMAKE_PROJECT_VERSION}")
# Setup the shared library naming in the windows style
set(CMAKE_SHARED_LIBRARY_PREFIX "")
set(CMAKE_SHARED_LIBRARY_SUFFIX ".dll")

install(TARGETS mech3fix)

#get_cmake_property(_variableNames VARIABLES)
#list (SORT _variableNames)
#foreach (_variableName ${_variableNames})
#    message(STATUS "${_variableName}=${${_variableName}}")
#endforeach()
