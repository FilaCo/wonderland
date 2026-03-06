cases = [
    {
        'name': 'test_registry',
        'sources': [
            files('test_registry.c'),
            unity_gen_runner.process('test_registry.c'),
        ],
    },
]

foreach case : cases
    test(
        case['name'],
        executable(
            case['name'],
            case['sources'],
            dependencies: unity_dep,
            include_directories: wecs_inc,
            link_with: wecs_static,
        ),
    )
endforeach
