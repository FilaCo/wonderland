wonderland_editor_name = meson.project_name() + 'Editor'
wonderland_editor_inc = [wonderland_inc]
wonderland_editor_src = []

subdir('src')

wonderland_editor = executable(
    wonderland_editor_name,
    wonderland_editor_src,
    include_directories: wonderland_editor_inc,
    link_with: wonderland_static,
)