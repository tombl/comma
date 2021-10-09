# ,

> Comma runs software without installing it.

A `pacman` port of [shopify/comma](https://github.com/shopify/comma)

Literally just a tiny wrapper for `pacman` and `fzf` that finds the right package for your command, and installs it temporarily while your command runs.

## Installation

```sh
git clone https://github.com/tombl/comma
cd comma
cargo build --release
cp target/release/comma ~/.local/bin/,
```

## Usage

```sh
, cowsay neato
```

Run `sudo pacman -Fy` on occasion to update the binary database.