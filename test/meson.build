gtest = subproject('gtest')
gtest_dep = gtest.get_variable('gtest_dep')
gtest_main_dep = gtest.get_variable('gtest_main_dep')

test_wonderland_deps = [gtest_dep, gtest_main_dep]
test_wonderland_cases = []

subdir('ECS')

foreach case : test_wonderland_cases
    test(
        case['name'],
        executable(
            case['name'],
            case['sources'],
            dependencies: test_wonderland_deps,
            include_directories: wonderland_inc,
            link_with: wonderland_static,
        ),
    )
endforeach

