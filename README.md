<div align="center">
  <img alt="rocco" width="250" src="assets/rocco.png" />
<h1>cargo docco</h1>
<p>cargo docco generates literate programming style 
documentation pages.</p>

[![Build Status](https://github.com/creativcoder/cargo-docco/workflows/build/badge.svg)](https://github.com/creativcoder/cargo-docco/actions)
[![crates.io](https://img.shields.io/crates/v/cargo-docco.svg)](https://crates.io/crates/cargo-docco)

</div>

## In action

![docco in action](assets/docco_cast.gif)

## Installation

```sh
$ cargo install cargo-docco
```

(Please check [`cargo`'s documentation](http://doc.crates.io/) to learn how `cargo install` works and how to set up your system so it finds binaries installed by `cargo`.)

#### Usage

```plain
> cargo docco -h
cargo-docco 0.1.0
Literate-style documentation generator from source code

USAGE:
    cargo docco [OPTIONS] -i <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input>         input source file
    -o <output>        optional path to the generated output html file
```

## Contribution

All kinds of contributions are welcome.

Questions can be asked in [issues](https://github.com/creativcoder/cargo-docco/issues).

To help us help you get pull requests merged quickly and smoothly, open an issue before submitted large changes. Please keep the contents of pull requests and commits short. Commit messages should include the intent of the commit.

## Support

<a href="https://www.buymeacoffee.com/creativcoder" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>

[![ko-fi](https://www.ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/P5P71YZ0L)

## License

Dual licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.