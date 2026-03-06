project(
    'wonderland',
    'c',
    default_options: ['warning_level=3', 'c_std=c23'],
    meson_version: '>= 1.3.0',
    version: '0.1.0',
    license: 'MIT',
)

unity = subproject('unity')
unity_dep = unity.get_variable('unity_dep')
unity_gen_runner = unity.get_variable('gen_test_runner')

subdir('libs')

regex = '^((?!(subprojects|tests)).)*$'
run_target(
    'tidy',
    command: [
        'run-clang-tidy',
        '-fix', 'files',
        regex,
        '-format',
        '-p=' + meson.project_build_root(),
    ],
)
