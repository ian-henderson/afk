# afk.rs

A command-line program to make your mouse wander. ;)

## Installation

### Cargo

    cargo install afk

### Homebrew

Coming soon!

## Usage

    afk [FLAGS] [OPTIONS]

## Flags

    -d, --debug Activate debug mode
    -h, --help Prints help information
    -V, --version Prints version information
    -v, --verbose Verbose mode (-v, -vv, -vvv, etc.)

## Options

    --max-delay <max-delay>       Max delay time in seconds [default: 30]
    --max-distance <max-distance> Max distance in pixels for mouse to move. Don't make this too big. The mouse
                                  won't move at all if it doesn't have the room! [default: 100]
    --min-delay <min-delay>       Min delay time in seconds [default: 5]
    --min-distance <min-distance> Min distance in pixels for mouse to move [default: 1]

## Example

    afk -v --max-delay 15 --max-distance 200
