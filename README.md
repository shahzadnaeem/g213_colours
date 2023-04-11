# Logitech G213 Keyboard USB Backlight Utility

## Introduction

A Rust version of [G213Colors](https://github.com/SebiTimeWaster/G213Colors)

## Installing

This utility needs to be run as root to access the G213 keyboard via USB.

You will need to ensure you have `sudo` access for some of the following commands.

- Step 1 - Installing
  - `cargo install --path .` will install `g213-cols` in `~/.cargo/bin`
- Step 2 - Creating a `setuid root` version
  - NOTE: We need to copy the utility to a different location. Otherwise further `cargo install` steps will fail.
  - Copy the above `g213-cols` binary to a location in your `PATH` - eg `~/bin`
    - `sudo cp ~/.cargo/bin/g213-cols ~/bin`
    - `sudo chown root.root ~/bin/g213-cols`
    - `sudo chmod u+s ~/bin/g213-cols` - now this command will run as `root`

## Commands

See [X11 colour names](https://en.wikipedia.org/wiki/X11_color_names) for all supported colour names - eg alice blue, lawn green etc

```text
- Set whole keyboard colour
  - g213-cols colour                 -- sets the default 'white'
  - g213-cols colour ffff00          -- sets the specified hex colour, yellow in this case
  - g213-cols colour lawn green      -- sets the named X11 colour
  - g213-cols colour "lawn green"    -- sets the named X11 colour - as a single argument
  - g213-cols colour lawn_green      -- sets the named X11 colour - underscores become spaces
- Set the colour of a specific keyboard region - 1 to 5
  - g213-cols region 2 [colour]      -- sets the region to the [colour] as defined above
- Set 'breathe' mode
  - g213-cols breathe 1000 [colour]  -- sets the breathe time (in ms) for the [colour]
- Set 'cycle' mode
  - g213-cols cycle 1000             -- sets the cycle time (in ms) all colours
```

### Building

```sh
$ cargo build ## (add -r for release and change target/debug to target/release below)

# Needs to be run as root for access to the USB device

$ sudo target/debug/g213_cols ...  # Run any command as described above

```

## Todo

- Make an installable binary and publish to crates.io
