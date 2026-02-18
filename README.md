# `nd-cli`

`nd-cli` is a terminal directory navigator.
It displays subdirectories, lets you move using numeric choices, and prints the final selected path when you quit.

## Installation

```bash
cargo install nd-cli
```

### Enable shell integration (required to change directories from your current shell).

The project crate is `nd-cli`, and the binary/shell command is `nd`:

```bash
# zsh
echo 'eval "$(nd init zsh)"' >> ~/.zshrc
source ~/.zshrc

# bash
echo 'eval "$(nd init bash)"' >> ~/.bashrc
source ~/.bashrc

# fish
echo 'nd init fish | source' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish

# powershell
Add-Content -Path $PROFILE -Value 'Invoke-Expression (& nd init powershell)'
. $PROFILE
```

Use `nd` to navigate and update your current shell directory:

```bash
nd
```

Without shell integration, you can still run the binary directly:

```bash
nd
```

After shell integration, `nd` is a shell function that runs `nd` and then `cd`s to the selected directory.

For local development:

```bash
cargo run -- --show-hidden
```

## Usage

```bash
nd
```

Commands inside the TUI:

- `1..N` select a directory
- `b` go to parent directory
- `q` quit and print selected path

Flags:

- `--show-hidden` include directories that start with `.`
- `--start-dir <PATH>` start browsing from a specific directory
- `--no-color` disable ANSI color output
