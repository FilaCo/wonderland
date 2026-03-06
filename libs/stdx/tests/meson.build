cases = []

subdir('collections')

foreach case : cases
    test(
        case['name'],
        executable(
            case['name'],
            case['sources'],
            dependencies: unity_dep,
            include_directories: stdx_inc,
            link_with: stdx_static,
        ),
    )
endforeach