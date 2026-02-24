# Command Reference

## Summary

```text
nd [--show-hidden] [--start-dir <PATH>] [--no-color]
nd init [bash|zsh|fish|powershell]
nd --help
nd --version
```

## Commands

### `nd`

Starts the interactive navigator.

Behavior:

- Lists only directories in the current location.
- Sorts directory names alphabetically (case-insensitive).
- Accepts `1..N` to enter a directory.
- Accepts `b` to move to parent.
- Accepts `q` to quit and print the selected path.

### `nd init [shell]`

Prints shell integration code for `bash`, `zsh`, `fish`, or `powershell`.

The generated wrapper function runs `nd-cli`, captures its printed path,
then changes the shell working directory.

## Flags

### `--show-hidden`

Includes directories whose names start with `.`.

### `--start-dir <PATH>`

Starts browsing from a specific directory instead of your current shell cwd.

### `--no-color`

Disables ANSI color output.

## Environment

### `NO_COLOR`

If set, color output is disabled even without `--no-color`.
