use crate::cli::Shell;

pub fn init_script(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash | Shell::Zsh => {
            r#"
nd() {
  if [ "$1" = "init" ]; then
    command nd "$@"
    return
  fi
  local target
  target="$(command nd "$@")" || return
  [ -n "$target" ] && cd "$target"
}
"#
        }

        Shell::Fish => {
            r#"
function nd
  if test "$argv[1]" = "init"
    command nd $argv
    return
  end
  set target (command nd $argv)
  or return
  test -n "$target"; and cd "$target"
end
"#
        }

        Shell::PowerShell => {
            r#"
function nd
  param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $Args
  )

  if ($Args.Length -gt 0 -and $Args[0] -eq "init") {
    & nd @Args
    return
  }

  $target = & nd @Args
  if ($LASTEXITCODE -ne 0) {
    return
  }

  if ($target -and -not [string]::IsNullOrWhiteSpace($target)) {
    Set-Location -LiteralPath $target
  }
end
"#
        }
    }
}
