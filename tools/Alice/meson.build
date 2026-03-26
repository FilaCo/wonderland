antlr4_rt_lib = antlr4_rt.dependency('antlr4_static')

alice_gen_files = custom_target(
    'antlr4-gen',
    input: files('Alice.g4'),
    output: [
        'AliceLexer.cpp',
        'AliceLexer.h',
        'AliceParser.cpp',
        'AliceParser.h',
        'AliceListener.cpp',
        'AliceListener.h',
        'AliceBaseListener.cpp',
        'AliceBaseListener.h',
        'AliceVisitor.cpp',
        'AliceVisitor.h',
        'AliceBaseVisitor.cpp',
        'AliceBaseVisitor.h',
    ],
    command: [
        find_program('antlr4'),
        '-Dlanguage=Cpp',
        '-visitor',
        '-o', '@OUTDIR@',
        '-Xexact-output-dir', '@INPUT@',
    ],
)

alice_gen_inc = [include_directories('.')]

alice_gen_lib = static_library(
    'AliceGen',
    alice_gen_files,
    dependencies: [antlr4_rt_lib],
    include_directories: alice_gen_inc,
)
