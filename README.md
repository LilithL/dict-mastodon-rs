# dict-mastodon-rs

## Installation

You have to compile the compile the project and compile it with `cargo build --release`.
Check releases to get the source code.

## Usage

You can have a detailed list of possible options by typing `masto_bot --help`, `masto_bot help` or simply `masto_bot`.

## Dictionaries

You can either write full sentences by starting the line with `**` like : `**I'm feelings good today` or simply type a word per line to which masto_bot will append the configured word.

## Features

Here are the current feature. This is going to change with time ofc:

- CLI interface with different commands
- Reading config files and parsing them
- Reading the dictionary
- Selecting a random sentence or word in the dictionary and adding a custom word
- Use the wordnik API to fetch extra adjectives
- Register to a mastodon instance and saving the tokens into the config file
- Login on a registered account
- Post a generated sentence
- Post indefinitelly with a custom delay
