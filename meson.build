project(
    'Wonderland',
    'cpp',
    default_options: ['warning_level=3', 'cpp_std=c++23'],
    meson_version: '>= 1.3.0',
    version: '0.1.0',
    license: 'MIT',
)

wonderland_inc = [include_directories('include')]
wondlerland_src = []

subdir('src')

wonderland_static = static_library(meson.project_name(), wondlerland_src, include_directories: wonderland_inc)

wonderland_exec = executable(
    meson.project_name(),
    files('src/main.cpp'),
    include_directories: wonderland_inc,
    link_with: wonderland_static,
)

if get_option('enable-tests')
    subdir('test')
endif

if get_option('enable-benchmarks')
    subdir('benchmark')
endif

subdir('docs')

regex = '^((?!(benchmark|docs|subprojects|test)).)*$'
run_target(
    'lint',
    command: [
        'run-clang-tidy',
        'files',
        regex,
        '-p=' + meson.project_build_root(),
    ],
)
