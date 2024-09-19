# wa-cli

wa-cli is a command line tool for querying the Wolfram Alpha Short Answer and Simple API directly from your terminal.

## Features

- **Text and Graphical Response:** Instant answers from the Wolfram Alpha API, including both text-based and graphical outputs.
- **API Key Management:** Easily configure your Wolfram Alpha App ID directly from the command line.
- **Cross-Platform Configuration:** Adheres to XDG standards for configuration file locations

## Installation

To install wa-cli, follow these steps:

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/justin-gill/wa-cli.git
    ```

2. Navigate to the project directory:

    ```bash
    cargo install --path .
    ```
Make sure $CARGO_HOME/bin is in your PATH

## Usage

### Configuration

You must first have a Wolfram Alpha App ID to use this tool.
To get an App ID, visit the [Wolfram Alpha Developer Portal](https://developer.wolframalpha.com/portal/myapps/index.html) and create an account. You will have 2,000 free queries per month.

To configure your Wolfram Alpha App ID, run:

```bash
wa-cli configure
```

Follow the prompts to enter your App ID.
The App ID will be stored in a config.toml in your `XDG_CONFIG_HOME/wa-cli`

### Querying Wolfram Alpha

To query Wolfram Alpha, simply run:

```bash
wa-cli query "<query>"
```

You can also request a full answer using the --full flag
Note: Will not work within tmux, refer to this [issue](https://github.com/atanunq/viuer/issues/29)

```bash
wa-cli --full query "<query>"
```

## Dependencies
* [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Future Improvements
* More configuration options

