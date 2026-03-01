# Luca

![Luca](./images/luca.png)

A smart calculator app.

## Build

Install the dependencies.

```bash
sudo apt install libgtk-4-dev
sudo apt install libgranite-7-dev

flatpak install -y appcenter io.elementary.Sdk
flatpak install runtime/org.freedesktop.Sdk.Extension.rust-stable/x86_64/23.08 --user -y
```

Build the package.

```bash
flatpak-builder --user flatpak_app pro.lasne.luca.json --force-clean --install
```

When you add a dependency to `Cargo.toml`, you need to update flatpak's manifest.
Use the script from [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools/tree/master/cargo)

```bash
./flatpak-builder-tools/cargo/flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```
