<div align="center">

[![Rust Check](https://github.com/Walker-00/reh/actions/workflows/rust.yml/badge.svg)](https://github.com/Walker-00/rview/actions/workflows/rust.yml)
[![Clippy Analyze](https://github.com/Walker-00/reh/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Walker-00/rview/actions/workflows/rust-clippy.yml)

</div>

# rview
Reh mean... I don't Know, I just give it.
<br>
If `Reh` is a little weird you can call it as `Rich`.
<br>
This `Reh` is the a part of the project `Project SS`.

## [Features]

- [x] Cross-Platform Supported
- [x] Optimized with Cargo Flags
- [x] Support Videos and Animated Images
- [x] Videos and Animated Images From File Path
- [x] Error Handling
- [ ] Videos and Animated Images From Url
- [ ] Better Error Handling
- [ ] Better Optimization
- [ ] Using Concurrency or Parallelism

## [Usage]

```
Usage: reh [OPTIONS] --file <FILE> --loop <LOOPIT> [MODE]

Arguments:
  [MODE]  Choose Mode to set wallpaper [default: fit] [possible values: fit, crop, span, center, tile, stretch]

Options:
  -f, --file <FILE>    Video or animated image's file path
  -s, --sleep <SLEEP>  sleep time per one frame as millisecond to set frame wallpaper [default: 0]
  -l, --loop <LOOPIT>  loop the wallpaper video, forever?
  -h, --help           Print help (see more with '--help')
  -V, --version        Print version
```

## [Installation]

### [Linux]

```sh
wget https://github.com/Walker-00/reh/releases/download/v0.1.0/reh
chmod +x reh
sudo mv reh /bin # or ln $PWD/reh /bin/reh
```

### [Mac]

Follow The [Compile From Source](https://github.com/Walker-00/reh#compile-from-source) Instructions, Cuz I don't know how to cross-compile for mac.

### [Windows]

Follow The [Compile From Source](https://github.com/Walker-00/reh#compile-from-source) Instructions, Cuz I don't know how to cross-compile for Windows.

### [Compile From Source]

To Compile From Source You Need To Install:

- [Rustup](https://rustlang.org)
- FFMPEG libraries ex. libavutil, libavformat etc...

```sh
git clone https://github.com/Walker-00/reh
cd reh
cargo install --path .
```

## [Voices From Burma]
Currently, Burma is under the Military rule and as a burmese, living under the dictatorship is extremely chellenging.
<br>
But we stayed strong and are fighting against Military Junta's bullets and violence.
<br>
We demand Justice and Democracy.
<br>
And we kindly want to ask for the International support and attention.
