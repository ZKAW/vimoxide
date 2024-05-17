# Vimoxide
A tool to quickly open files with Vim / Nvim based on frequency of access.

## Setup
```bash
git clone https://github.com/ZKAW/vimoxide
```
```
cargo install cargo-make
```

## Install
The following will build the project and add it to your /usr/bin directory.
```bash
cargo make install
```

(optional)
Add the following to your shell configuration file for quick access.
```bash
alias v='vimoxide'
```

Note that you can switch between `vim` and `nvim` at any time
by changing the `executor` field in the `~/.config/vimoxide/conf.json` file.

## Usage
```bash
vimoxide or v <file>
```

## Example
```bash
v some/sub/folder/file.txt
v file
```
