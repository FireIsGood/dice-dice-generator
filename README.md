# Dice Dice Generator

A Dice Dice generator written in Rust.

Dice Dice is a game I made up to simulate a miniature round of gambling. It is
a series of numbered answers that are covered by spoilers. Users start with a
certain number of points and may then click as many answers as they want.
Answers may be positive or negative towards score and the player counts the
score on their own.

It's just a random text game I made up on the spot since I was running out of
ideas for Minesweeper.

```markdown
Config name: all good
## Dice Dice (Day 1984)

*Test Edition*

Unspoiler any number of the following prompts, when you want to stop, record your number. You may then check the rest. (please don't post the answers)

You start with 100 points. Calculate on your points with what is written.
:one: ||`add 5                   `||
:two: ||`subtract 10             `||
:three: ||`spin in a circle        `||
```

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
# Name of the config
name = "all good"

# Optional minimum length to pad to
min_length = 24

# Optional rules to display
rules = """
## Dice Dice (Day 1984)

*Test Edition*

Unspoiler any number of the following prompts, when you want to stop, record your number. You may then check the rest. (please don't post the answers)

You start with 100 points. Calculate on your points with what is written."""

# Vector of tables of answers
answers = [
    # Answers have an optional number
    { operation = "add", number = 5 },
    { operation = "subtract", number = 10 },
    { operation = "spin in a circle" },
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
