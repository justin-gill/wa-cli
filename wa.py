#!/usr/bin/env python3

import requests
import os
import sys

API_URL = "https://api.wolframalpha.com/v1/result"
HOME_DIR = os.path.expanduser("~")

def configure():
    '''
    Configure the Wolfram Alpha App ID, save to ~/.wa/credentials
    '''
    wa_dir = os.path.join(HOME_DIR, ".wa")
    credentials_file = os.path.join(wa_dir, "credentials")
    os.makedirs(wa_dir, exist_ok=True)

    current_key = None
    if os.path.exists(credentials_file):
        with open(credentials_file, "r") as file:
            current_key = file.read().strip().split(' ')[-1]
        print(f"Current App ID: {current_key}")
        overwrite = input("Would you like to overwrite it? (y/N): ").strip().lower()
        if overwrite == 'n':
            print("Exiting without changes.")
            sys.exit(0)

    new_key = input("Enter your new Wolfram Alpha App ID: ").strip()

    if not new_key:
        print("No key entered. Exiting without changes.")
        sys.exit(1)

    with open(credentials_file, "w") as file:
        print(f"Writing to {credentials_file}")
        file.write(f'APP_ID = {new_key}')

    print("API Key has been updated.")
    sys.exit(0)

def make_request(query):
    '''
    Make a request to the Wolfram Alpha API and print the result.
    '''
    app_id = open(os.path.join(HOME_DIR, '.wa/credentials')).read().strip().split(' ')[-1]
    response = requests.get(API_URL, params={'appid': app_id, 'i': query})
    print(response.text)

def main():
    '''
    Entry point for the script. Handles system arguments.
    '''
    if len(sys.argv) < 2:
        print("usage: wa [configure] <query>")
        return

    if sys.argv[1] == "configure" and len(sys.argv) == 2:
        configure()

    if not os.path.exists(os.path.join(HOME_DIR, '.wa/credentials')):
        print("Please run 'wa configure' to configure your Wolfram Alpha App ID.")
        return

    query = ' '.join(sys.argv[1:])
    make_request(query)

if __name__ == '__main__':
    main()
