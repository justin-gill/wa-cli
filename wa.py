#!/usr/bin/env python3

import requests
import os
import sys
from io import BytesIO
from PIL import Image
from sixel import converter
import configparser

API_URL = "https://api.wolframalpha.com/v1/result"
API_FULL_URL = "https://api.wolframalpha.com/v1/simple"
HOME_DIR = os.path.expanduser("~")
CONFIG_DIR = os.getenv('XDG_CONFIG_HOME', os.path.join(HOME_DIR, '.config'))
CONFIG_FILE = os.path.join(CONFIG_DIR, "wa-cli", "config.ini")

def read_config():
    '''Read configuration from config.ini.'''
    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)
    if 'DEFAULT' in config:
        return config['DEFAULT'].get('APP_ID')
    return None

def configure():
    '''
    Configure the Wolfram Alpha App ID and save it to config.ini.
    '''
    if not os.path.exists(CONFIG_FILE):
        os.makedirs(os.path.dirname(CONFIG_FILE), exist_ok=True)
        with open(CONFIG_FILE, 'w') as file:
            file.write('[DEFAULT]\nAPP_ID = <YOUR_APP_ID_HERE>\n')

    config = configparser.ConfigParser()
    config.read(CONFIG_FILE)

    current_key = config['DEFAULT'].get('APP_ID', None)
    if current_key and current_key != '<YOUR_APP_ID_HERE>':
        print(f"Current App ID: {current_key}")
        overwrite = input("Would you like to overwrite it? (y/N): ").strip().lower()
        if overwrite == 'n':
            print("Exiting without changes.")
            sys.exit(0)

    new_key = input("Enter your new Wolfram Alpha App ID: ").strip()
    if not new_key:
        print("No key entered. Exiting without changes.")
        sys.exit(1)

    config['DEFAULT'] = {'APP_ID': new_key}
    with open(CONFIG_FILE, 'w') as file:
        config.write(file)

    print("API Key has been updated.")
    sys.exit(0)

def make_request(query, full_answer=False):
    '''
    Make a request to the Wolfram Alpha API and print the result.
    '''
    app_id = read_config()
    if not app_id:
        print("API Key is not configured. Please run 'wa configure' to set it up.")
        sys.exit(1)

    if not full_answer:
        response = requests.get(API_URL, params={'appid': app_id, 'i': query})
        print(response.text)
    else:
        response = requests.get(API_FULL_URL, params={'units': 'metric', 'appid': app_id, 'i': query})
        img = Image.open(BytesIO(response.content))
        with BytesIO() as output:
            img.save(output, format="PNG")
            img.seek(0)
            c = converter.SixelConverter(output)
            c.write(sys.stdout)

def main():
    '''
    Entry point for the script. Handles system arguments.
    '''
    if len(sys.argv) < 2:
        print("usage: wa [configure] <query>")
        return

    if sys.argv[1] == "configure" and len(sys.argv) == 2:
        configure()
        return

    if not os.path.exists(CONFIG_FILE):
        print("Please run 'wa configure' to configure your Wolfram Alpha App ID.")
        return

    query = ' '.join(sys.argv[1:])
    if query.startswith("--full"):
        make_request(' '.join(sys.argv[2:]), full_answer=True)
    else:
        make_request(query)

if __name__ == '__main__':
    main()
