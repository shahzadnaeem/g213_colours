# Logitech G213 Keyboard USB Backlight Util

## Introduction

A Rust version of [G213Colors](https://github.com/SebiTimeWaster/G213Colors)

## Notes

Definitely a Work-in-progress!

Currently all you can do is set a single colour for the whole keyboard

```sh
$ cargo build ## (add -r for release and change debug to release below)

# Needs to be run as root for access to the USB device

$ sudo target/debug/usb-play ffd0c0  # An ok white on my keyboard
$ sudo target/debug/usb-play 10d010  # Green

```

## Todo

- Add support for various options like the original
- Running as root is not so good 🤕
