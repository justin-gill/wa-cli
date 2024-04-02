# wa-cli

wa-cli is a command-line interface tool that allows you to query the Wolfram Alpha Short Answer API directly from your terminal.

## Features

- **Configure API Key:** Easily configure your Wolfram Alpha App ID to authenticate API requests.
- **Query Wolfram Alpha:** Send queries directly to the Wolfram Alpha API and receive detailed responses.
- **Easy Installation:** Simple installation process to set up the CLI tool on your system.

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

3. Run the installation script with sudo permissions:

    ```bash
    sudo ./install
    ```
This will copy the script to `/usr/local/bin`

## Usage

### Configuration

You must first have a Wolfram Alpha App ID to use this tool.
To get an App ID, visit the [Wolfram Alpha Developer Portal](https://developer.wolframalpha.com/portal/myapps/index.html) and create an account. You will have 2,000 free queries per month.

To configure your Wolfram Alpha App ID, run:

```bash
wa configure
```

Follow the prompts to enter your App ID.
The App ID will be stored in the `~/.wa/credentials` file.

### Querying Wolfram Alpha

To query Wolfram Alpha, simply run:

```bash
wa query <query>
```

#### Example
```
wa What is the capital of France?
```

## Dependencies
* python3

## Issues/Improvements
* Currently supports only short answer queries.
* Limited to a fixed bin path and credentials file path, making it compatible only with Linux systems.
* Lack of API usage tracking.

