## Leetcode local development assistant

 A program that given the link or slug to a leetcode problem,
 creates a local file where you can develop and test your solution before post it back to leetcode.

 ## ScreenShots

 ### `cargo leet`
 ![ScreenShot](assets/help_scr_shot_top.png)

 ### `cargo leet generate --help`
 ![ScreenShot](assets/help_scr_shot_generate.png)

## Installation

NB: If cargo-leet is already installed you do the install it will just replace it even it it was previously installed from a different source. For example if you install it from a clone then run the command to install from git it will replace the existing version that is installed (they will not both be installed).

### From GitHub

```sh
cargo install --git https://github.com/rust-practice/cargo-leet.git --branch main
```

### From Clone

After cloning the repo run

```sh
cargo install --path .
```

or using alias from `.cargo/config.toml`

```sh
cargo i
```

## Uninstallation

```sh
cargo uninstall cargo-leet
```

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
