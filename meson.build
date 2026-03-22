project(
    'Wonderland',
    'cpp',
    default_options: ['warning_level=3', 'cpp_std=c++26'],
    meson_version: '>= 1.3.0',
    version: '0.1.0',
    license: 'MIT',
)

sdl3 = subproject('sdl3')
sdl3_dep = sdl3.get_variable('sdl3_dep')

threads_dep = dependency('threads')

wonderland_inc = [include_directories('include')]
wonderland_src = []
wonderland_deps = [sdl3_dep, threads_dep]

subdir('src')

wonderland_static = static_library(
    meson.project_name(),
    wonderland_src,
    include_directories: wonderland_inc,
    dependencies: wonderland_deps,
)

wonderland_editor = executable(
    meson.project_name(),
    files('src/main.cpp'),
    include_directories: wonderland_inc,
    link_with: wonderland_static,
)

if get_option('enable-tests')
    subdir('tests')
endif

if get_option('enable-benchmarks')
    subdir('benchmarks')
endif

subdir('docs')

regex = '^((?!(benchmarks|docs|subprojects|tests)).)*$'
run_target(
    'lint',
    command: [
        'run-clang-tidy',
        'files',
        regex,
        '-p=' + meson.project_build_root(),
    ],
)
