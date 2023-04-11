# Logitech G213 Keyboard USB Backlight Utility

## Introduction

A Rust version of [G213Colors](https://github.com/SebiTimeWaster/G213Colors)

See Commands section below for supported commands.

The last successful command is saved to `~/.g213-cols.json`. This will be used if `g213-cols` is called subsequentlt called with no arguments. This allows the state of the keyboard to be quickly restored.

## Installing

As `g213-cols` needs to be run as root to access the G213 keyboard via USB, the following steps need to be followed to install it correctly.

You will need to ensure you have set up `sudo` access for some of the following commands.

- Step 1 - Installing - standard cargo step
  - `cargo install --path .` will install `g213-cols` in `~/.cargo/bin`
- Step 2 - Creating a `setuid root` version
  - NOTE: `g213-cols` must be copied to and run from a different location. Otherwise further `cargo install` steps may fail.
  - Copy the above `g213-cols` binary to a location in your `PATH` - eg `~/bin`
    - `sudo cp ~/.cargo/bin/g213-cols ~/bin`
    - `sudo strip ~/bin/g213-cols`
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

- Use saved command
  - g213-cols                        -- Runs the previosly successful command
                                          from ~/.g213-cols.json
```

## Todo

- Make an installable binary and publish to crates.io
