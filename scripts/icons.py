import os
import shutil
import sys

import requests


def get_url(name, size):
    return f"https://fonts.gstatic.com/s/i/short-term/release/materialsymbolsoutlined/{name}/default/{size}px.svg"



def get_icons(name):
    # Remove the ./name directory if it exists
    if os.path.exists(name):
        shutil.rmtree(name)

    # Create the ./name directory
    os.makedirs(name)

    # Download files with different sizes into the ./name directory
    sizes = [20, 24, 40, 48]

    for size in sizes:
        url = get_url(name, size)
        destination = f"{name}/{size}px.svg"

        try:
            response = requests.get(url)
            response.raise_for_status()  # Check for errors in the HTTP response

            with open(destination, 'wb') as file:
                file.write(response.content)

            print(f'Download from {url} to {destination} successful.')
        except requests.exceptions.RequestException as e:
            print(f'Error: {e}')


if __name__ == "__main__":
    # Check if at least one argument is provided
    if len(sys.argv) < 2:
        print("Usage: python icons.py name1 name2 ...")
        sys.exit(1)

    # Process each command line argument as a directory name
    for arg in sys.argv[1:]:
        get_icons(arg)