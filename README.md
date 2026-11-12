# Bevy Starter

This repo is a minimal starter for Bevy `0.20`

## Inspiration

- [`bevy_space`](https://github.com/perlindgren/bevy-space)
- [`bevy_new_2d`](https://github.com/TheBevyFlock/bevy_new_2d)
- [`sobevy`](https://codeberg.org/doomy/sobevy)
- [`Mischief in miniature`](https://github.com/alice-i-cecile/mischief-in-miniature)

## Building

You can build your game

```
cargo run
```

If you want the extra dev features then you can toggle them:

```
cargo run --features dev
```

Depending on if you are building a 2D or 3D game you can set your Bevy features
accordingly in `Cargo.toml` to reduce compile times. For 2D games you can use:

```toml
[dependencies.bevy]
version = "0.20"
default-features = false
features = ["2d"]
```

See
[Cargo Feature Collections](https://bevy.org/news/bevy-0-18/#cargo-feature-collections)
for more information.

## Features

- Cargo configured according to Bevy guide with build optimizations
- [Avian](https://github.com/Jondolf/avian) physics
- Assets menaging with bevy_asset_loader and iyes_progress.
- Data-Driven framework with bevy_common_assets
- Controller with bevy_tnua
- Navmesh gen and pathing with vleue_navigator
- Generic set of starting plugins with your games logic inside `GamePlugin`
- `TLDR.md` for quick reference and passing to LLMs
- `.agents` folder with bevy skills for LLMs

## Missing

- Deployment
