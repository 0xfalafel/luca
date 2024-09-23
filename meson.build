# Tutorial for reference
# https://belmoussaoui.com/blog/8-how-to-flatpak-a-rust-application/

project(
    'luca',
    'rust',
    version: '0.1.0'
)

subdir('data')
subdir('src')

# install the files under `data`
datadir = get_option('prefix') / get_option('datadir')

application_id = 'io.github.0xfalafel.luca'
# Read more <https://mesonbuild.com/Reference-manual.html#install_data>

# Desktop file
install_data(
    'luca.desktop',
    install_dir: datadir / 'applications'
)

# Metainfo
install_data(
    'luca.metainfo.xml',
    install_dir: datadir / 'metainfo'
)

