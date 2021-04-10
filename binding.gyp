{
  "targets": [
    {
      "target_name": "skyra-ai",
      "sources": [
        "src/games/TicTacToe.cc",
        "src/games/ConnectFour.cc",
        "src/main.cc"
      ],
      "include_dirs": [
        "<!@(node -p \"require('node-addon-api').include\")",
        "<(module_root_dir)/include"
      ],
      "dependencies": [
        "<!(node -p \"require('node-addon-api').gyp\")"
      ],
      "defines": [
        "NAPI_DISABLE_CPP_EXCEPTIONS"
      ],
      "xcode_settings": {
        "CLANG_CXX_LIBRARY": "libc++",
        "MACOSX_DEPLOYMENT_TARGET": "10.7",
        "GCC_ENABLE_CPP_EXCEPTIONS": "NO"
      },
      "msvs_settings": {
        "VCCLCompilerTool": {
          "ExceptionHandling": 0,
          "WarningLevel": 4,
          "EnablePREfast": "true",
        }
      },
      "cflags": [ "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ],
      "cflags_cc": [ "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ],
    },
    {
      "target_name": "skyra-ai-tests",
      "type": "executable",
      "sources": [
        "tests/Connect4.cc",
        "src/games/TicTacToe.cc",
        "src/games/ConnectFour.cc",
      ],
      "include_dirs": [
        "<(module_root_dir)/include",
        "googletest/googletest/include"
      ],
      "libraries": [
        "-lgtest -lgtest_main", "-L../googletest/build/lib"
      ],
      "cflags": [ "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ],
      "cflags_cc": [ "-std=c++17", "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ]
    }
  ]
}
