stdx_inc = [include_directories('include')]
stdx_src = []

subdir('src')

stdx_static = static_library('stdx', stdx_src, include_directories: stdx_inc)

if get_option('enable-tests')
    subdir('test')
endif
