cdata = configuration_data()
cdata.set('PROJECT_ROOT_DIRECTORY', meson.project_source_root())
cdata.set('OUTPUT_DIRECTORY', join_paths(meson.project_build_root(), 'docs', 'ref'))
cdata.set('VERSION', meson.project_version())
cdata.set('PROJECT_NAME', meson.project_name())

doxyfile = configure_file(
    input: 'Doxyfile.in',
    output: 'Doxyfile',
    configuration: cdata,
    install: false,
)

doxygen = find_program('doxygen')
docs_ref_target = run_target('docs_ref', command: [doxygen, doxyfile])
