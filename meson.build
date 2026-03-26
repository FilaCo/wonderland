project(
    'Wonderland',
    'cpp',
    default_options: ['warning_level=3', 'cpp_std=c++23'],
    meson_version: '>= 1.6.0',
    version: '0.1.0',
    license: 'MIT',
)

sdl3 = subproject('sdl3')
sdl3_dep = sdl3.get_variable('sdl3_dep')

threads_dep = dependency('threads')

cmake = import('cmake')
antlr4_rt_opts = cmake.subproject_options()
antlr4_rt_opts.add_cmake_defines({'ANTLR_BUILD_CPP_TESTS': false, 'WITH_DEMO': false, 'ANTLR_BUILD_SHARED': false})
antlr4_rt = cmake.subproject('antlr4-runtime', options:antlr4_rt_opts)

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

subdir('tools')

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
