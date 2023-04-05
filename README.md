# Logitech G213 Keyboard USB Backlight Utility

## Introduction

A Rust version of [G213Colors](https://github.com/SebiTimeWaster/G213Colors)

## Notes

Currently all you can do is set a single colour for the whole keyboard

You can use any valid [X11 colour name](https://en.wikipedia.org/wiki/X11_color_names) - eg aliceblue etc

```sh
$ cargo build ## (add -r for release and change target/debug to target/release below)

# Needs to be run as root for access to the USB device

$ sudo target/debug/g213_cols              # Default, an ok white on my keyboard
$ sudo target/debug/g213_cols ffd0c0       # An ok white on my keyboard
$ sudo target/debug/g213_cols 10d010       # A green
$ sudo target/debug/g213_cols aliceblue    # Any valid X11 colour
$ sudo target/debug/g213_cols alice blue   # Any valid X11 two (or three) word colour
$ sudo target/debug/g213_cols "alice blue" # Multi word colour as a single argument
$ sudo target/debug/g213_cols alice_blue   # Underscores are allowed, become spaces

```

## Todo

- Add support for various options like the original
- Running as root is not so good 🤕
- Make an installable binary and publish to crates.io
