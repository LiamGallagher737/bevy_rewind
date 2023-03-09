<div align="center">

# Bevy XXX 

*Description*

<!-- Find and replace all `YourGithubUser` with your GitHub username -->
<!-- Find and replace all `bevy_XXX` with your crate name, make sure to use underscores and not spaces -->

<!-- Replace `released%20version` with `main` if you plan to track the main branch aas much as you can -->
[<img alt="bevy tracking" src="https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue?style=for-the-badge" height="24">](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/YourGithubUser/bevy_XXX/rust.yml?branch=main&style=for-the-badge" height="24">](https://github.com/YourGithubUser/bevy_XXX/actions)
[<img alt="github" src="https://img.shields.io/badge/github-bevy_XXX-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24">](https://github.com/YourGithubUser/bevy_XXX)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bevy_XXX.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24">](https://crates.io/crates/bevy_XXX)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bevy_XXX-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24">](https://docs.rs/bevy_XXX)

</div>



# [Demo](examples/demo.rs)

*Description of my amazing demo*

```console
cargo run --example demo 
```

<div align="center">
    <img src="examples/images/demo.png" alt="Screenshot of Demo Example" width="600" />
</div>



# Setup

Import the prelude

```rust
use bevy_XXX::prelude::*;
```

Add the plugin to your app
```rust
.add_plugin(MyAmazingPlugin)
```

Config Options

```rust
MyAmazingPlugin {
    option_1: f32, // How fast you want your app to run
    option_2: bool, // Should your app crash randomly
}
```



# Usage

***Explain some basic usage of your crate, components, system sets, etc.***



# Bevy Tracking

|Bevy|bevy_XXX|
|---|---|
|0.10|0.1.0|



<!-- REMOVE THIS IF YOU USE A DIFFERENT LICENSE -->
# License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
