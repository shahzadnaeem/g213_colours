# Logitech G213 Keyboard USB Backlight Util

## Introduction

A Rust version of [G213Colors](https://github.com/SebiTimeWaster/G213Colors)

## Notes

Definitely a Work-in-progress!

Currently all you can do is set a single colour for the whole keyboard

You can use any valid [X11 colour name](https://en.wikipedia.org/wiki/X11_color_names) - eg aliceblue etc

```sh
$ cargo build ## (add -r for release and change target/debug to target/release below)

# Needs to be run as root for access to the USB device

$ sudo target/debug/g213_colours         # Default, an ok white on my keyboard
$ sudo target/debug/g213_colours ffd0c0  # An ok white on my keyboard
$ sudo target/debug/g213_colours 10d010  # Green
$ sudo target/debug/g213_colours aliceblue # Any valid X11 colour

```

## Todo

- Add support for various options like the original
- Running as root is not so good 🤕
