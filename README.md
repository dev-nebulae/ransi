# ransi - Rust ANSI

## Table of Contents

- [About](#about)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [License](#license)
- [Contributing](#contributing)
- [Notes](#notes)

## About

`ransi` is a simple Rust library to help you make TUIs without going through
the pain of ANSI escape codes.

`ransi` provides a simple and easy-to-use API that allows you
to format, color, and style text on the terminal.

## Features

- Works in almost all terminals
- Supports every ANSI escape code <sup><a href="#supports-every-ansi-escape-code">1</a></sup>
- True color support
- 256 color support
- 16 color support
- No dependencies
- Lightweight
- Easy to use

## Installation

Run this command to install the latest version of `ransi`:

```bash
cargo add ransi
```

Or add it to your `Cargo.toml` file:

```toml
[dependencies]
ransi = "0.1"
```

## Usage

### Functions

---

| Function                                          | Description                          |
|---------------------------------------------------|--------------------------------------|
| `change_fg(color: &str or (r: u8, g: u8, b: u8))` | Change the foreground color          |
| `change_bg(color: &str or (r: u8, g: u8, b: u8))` | Change the background color          |
| `bold(string: &str)`                              | Make the string be bold if printed   |
| `italic(string: &str)`                            | Make the string be italic if printed |
| `blink(string: &str)`                             | Make the string blink if printed     |
| `mov_cur(x: u16, y: u16)`                         | Move the cursor to a position        |
| `

---

### Traits

- `ransi::ColorIn` - Color Input. Used by the `change_col` function to support
or a tuple of three `u8`s or the color name.

## Notes

##### Supports every ANSI escape code

This isn't *completely true*, as it misses the 21m escape code,
which almost always leads to undefined behavior in different terminals.

Originally, 21m was used to dim text, but no terminal supports it,
and the ones that do, and sometimes they double-underline the text
and make it bright.

It is pretty confusing, so we don't support it.
