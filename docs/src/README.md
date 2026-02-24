# nd-cli

`nd-cli` is a terminal directory navigator that replaces repetitive
`ls`/`cd` loops with indexed selection.

It lists subdirectories in the current location, lets you move deeper by number,
go back with `b`, and quit with `q`.

```text
Path: /current/working/dir
Commands: [1..3] select, [b] back, [q] quit

┃ 1. src
┃ 2. docs
┃ 3. target
>
```

## How It Works

1. `nd-cli` reads directories from the current working directory (or `--start-dir`).
2. It shows only directories, sorted alphabetically.
3. You choose a directory index, `b` (back), or `q` (quit).
4. On quit, it prints the selected path.
5. The shell `nd` wrapper function changes your shell cwd to that printed path.

## Next Steps

- Start with [Getting Started](getting-started.md).
- See [Command Reference](command-reference.md) for all commands and flags.
