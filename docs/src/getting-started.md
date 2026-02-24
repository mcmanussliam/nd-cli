# Getting Started

## Install

```bash
cargo install nd-cli
```

Check the install:

```bash
nd-cli --version
```

## Configure Your Shell

Add the `nd` shell wrapper so your shell can `cd` after `nd-cli` exits.

### zsh

```bash
echo 'eval "$(nd-cli init zsh)"' >> ~/.zshrc
source ~/.zshrc
```

### bash

```bash
echo 'eval "$(nd-cli init bash)"' >> ~/.bashrc
source ~/.bashrc
```

### fish

```bash
echo 'nd-cli init fish | source' >> ~/.config/fish/config.fish
source ~/.config/fish/config.fish
```

### powershell

```powershell
Add-Content -Path $PROFILE -Value 'Invoke-Expression (& nd-cli init powershell)'
. $PROFILE
```

## Run Your First Session

```bash
nd
```

Select a directory by number, use `b` to go up one level, and `q` to quit.

## Quick Tips

- Use `nd --show-hidden` to include dot-directories.
- Use `nd --start-dir <PATH>` to begin in a specific location.
- Use `nd --no-color` (or set `NO_COLOR`) to disable color output.
