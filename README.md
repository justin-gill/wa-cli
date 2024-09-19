# wa-cli

wa-cli is a command line tool for querying the Wolfram Alpha Short Answer and Simple API directly from your terminal.

## Features

- **Text and Graphical Response:** Instant answers from the Wolfram Alpha API, including both text-based and graphical outputs.
- **API Key Management:** Easily configure your Wolfram Alpha App ID directly from the command line.
- **Cross-Platform Configuration:** Adheres to XDG standards for configuration file locations

# Usage
```bash
wa-cli query "<query>"
```

You can also request a image answer using the --simple flag
Note: Will not work within tmux, refer to this [issue](https://github.com/atanunq/viuer/issues/29)

## Example
```bash
wa-cli query "What is the capital of France?"
> Paris, Île-de-France, France
```

## Installation

### From Source
```bash
git clone https://github.com/justin-gill/wa-cli.git
# Build and install
cd wa-cli
cargo install --path .
```

## Configuration

You must first have a Wolfram Alpha App ID to use this tool.
To get an App ID, visit the [Wolfram Alpha Developer Portal](https://developer.wolframalpha.com/portal/myapps/index.html) and create an account. You will have 2,000 free queries per month.

To configure your Wolfram Alpha App ID, run:

```bash
wa-cli configure
```

Follow the prompt to enter your App ID.
The App ID will be stored in a config.toml in your `XDG_CONFIG_HOME/wa-cli`, or `~/.config/wa-cli` by default.

## Dependencies
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
* libsixel (for image output on some terminals)


## Recommended
I would personally recommend using functions in your shell to make querying easier. Here is an example for bash:
```bash
function was() {
    ~/.cargo/bin/wa-cli -s query "$*"
}

function wa() {
    ~/.cargo/bin/wa-cli query "$*"
}
```

