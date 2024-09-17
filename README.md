# wa-cli

wa-cli is a command line tool for querying the Wolfram Alpha Short Answer and Simple API directly from your terminal.

## Features

- **Text and Graphical Response:** Instant answers from the Wolfram Alpha API, including both text-based and graphical (sixel) outputs.
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
    cd wa-cli
    ```

3. Run the installation script:

    ```bash
    ./install
    ```
This will copy the script to `XDG_BIN_HOME` and create a config file in `XDG_CONFIG_HOME`

## Usage

### Configuration

You must first have a Wolfram Alpha App ID to use this tool.
To get an App ID, visit the [Wolfram Alpha Developer Portal](https://developer.wolframalpha.com/portal/myapps/index.html) and create an account. You will have 2,000 free queries per month.

To configure your Wolfram Alpha App ID, run:

```bash
wa configure
```

Follow the prompts to enter your App ID.
The App ID will be stored in a config.ini in your `XDG_CONFIG_HOME/wa-cli`

### Querying Wolfram Alpha

To query Wolfram Alpha, simply run:

```bash
wa <query>
```

You can also request a full answer using the --full flag
Note: Currently, you must use a terminal that supports sixel (https://www.arewesixelyet.com/)

```bash
wa --full <query>
```

#### Example
```
wa What is the capital of France?
```

## Dependencies
* python3
* Pillow
* sixel
Note: For distros that don't support global pip installs (Arch, Debian, etc.), these packages will need to be installed with your package manager.

## Future Improvements
* Add Support Multiple Configuration Profiles
* Terminal Graphics Protocol
* Lack of API usage tracking.
* Switch to another language because python for global scripts with dependencies is a bad idea :/

