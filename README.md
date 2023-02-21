# sxhkd-rofi-cheatsheet

A blazingly fast dynamic sxhkd cheatsheet generator for rofi written in Rust.

## Installation

Just move the binary in releases to your path and make it executable, or build it yourself.

## Usage

```bash
sxhkd-rofi-cheatsheet
```

It will read your sxhkdrc file and generate a cheatsheet in rofi for you. When a command is selected, it will execute it. It is needed that sxhkd has following format:

```bash
# 
# SECTION HEADER
# 

# description
keybind
    command

# description
keybind
    command
#...
```

There could be multiple headers.
