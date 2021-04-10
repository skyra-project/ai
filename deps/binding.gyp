{
    "targets": [
        {
            "target_name": "googletest",
            "type": "static_library",
            "include_dirs": [
                "googletest/googletest/include/",
                "googletest/googletest"
            ],
            "direct_dependent_settings": {
                "include_dirs": [
                     "<!@(node -p \"require('node-addon-api').include\")",
                    "googletest/googletest/include",
                ]
            },
            "msvs_settings": {
                "VCCLCompilerTool": {
                "ExceptionHandling": 0,
                "WarningLevel": 4,
                "EnablePREfast": "true"
        	}
            },
            "xcode_settings": {
                "OTHER_CFLAGS": [
                    "-fPIC",
                    "-DPIC",
                    "-O3",
                    "-Wno-implicit-function-declaration"
                ]
            },
            "cflags": [ "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ],
      		"cflags_cc": [ "-Werror", "-Wall", "-Wextra", "-Wpedantic", "-Wunused-parameter", "-fno-exceptions" ],
            "sources": [
                "googletest/googletest/src/gtest-all.cc",
                "googletest/googletest/src/gtest_main.cc"
            ]
        }
    ]
}