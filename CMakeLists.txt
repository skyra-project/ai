cmake_minimum_required(VERSION 3.13 FATAL_ERROR)
cmake_policy(SET CMP0042 NEW)
set(CMAKE_CXX_STANDARD 11)

project (skyra-ai
  VERSION 1.1.0
  DESCRIPTION "Advanced Gaming AI for Skyra"
  HOMEPAGE_URL "https://github.com/skyra-project/ai"
  LANGUAGES CXX)

include_directories(${CMAKE_JS_INC})
add_definitions(-DNAPI_VERSION=3)
add_definitions(-DNAPI_DISABLE_CPP_EXCEPTIONS)

set(SOURCE_FILES
  src/main.cc
  src/games/TicTacToe.cc
  src/games/ConnectFour.cc)

add_library(${PROJECT_NAME} SHARED ${SOURCE_FILES} ${CMAKE_JS_SRC})
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")
target_link_libraries(${PROJECT_NAME} ${CMAKE_JS_LIB})

# Include N-API wrappers
execute_process(COMMAND node -p "require('node-addon-api').include"
  WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
  OUTPUT_VARIABLE NODE_ADDON_API_DIR
  )

string(REPLACE "\n" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})
string(REPLACE "\"" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})

target_include_directories(${PROJECT_NAME}
  PUBLIC
  ${CMAKE_SOURCE_DIR}/include
  ${NODE_ADDON_API_DIR}
  ${CMAKE_JS_INC}
  )
