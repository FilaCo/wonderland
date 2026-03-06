wecs_inc = [include_directories('include'), stdx_inc]
wecs_src = []

subdir('src')

wecs_static = static_library('wecs', wecs_src, include_directories: wecs_inc, link_with: stdx_static)

if get_option('enable-tests')
    subdir('tests')
endif
