# Dice Dice Generator

A Dice Dice generator written in Rust.

Dice Dice is a game I made up to simulate a miniature round of gambling. It is
a series of numbered answers that are covered by spoilers. Users start with a
certain number of points and may then click as many answers as they want.
Answers may be positive or negative towards score and the player counts the
score on their own.

It's just a random text game I made up on the spot since I was running out of
ideas for Minesweeper.

## Installation

Download the [latest release binary](https://github.com/FireIsGood/minesweeper-generator/releases) or build from the source.

To build from the source, clone the repo and run the cargo command:

```bash
cargo build --release
```

The binary will be created in `./target/release`.

## Usage

The binary requires a configuration [TOML](https://toml.io/en/) file in the
following format:

```toml
name       = "Name of your configuration"
min_length = 24 # Minimum length to pad to

answers = [
    { operation = "Operation with a number", number = 4 }, # Number is optional
    { operation = "Operation with no number" },
]
```

Once you have a configuration, run the binary and supply the location of your
config file.

```bash
dice-dice-generator ./path/to/config.toml
```

## Limitations

Discord only allows for 99 emotes to be rendered in a message, so the numbers
in the message might get cut off at a certain length.

## Contributing

This project is a generator for a game that I made up and the configuration
file is harder to write than just doing the whole thing by hand. If you really
want to contribute regardless of that, feel free to make issues or pull
requests.

## License

[MIT](https://choosealicense.com/licenses/mit/)
