<h1 align="center">SHAKA</h1>

<p align="center"><strong>One config for every shell shortcut.</strong></p>

<p align="center">
  Generate aliases and functions for <code>bash</code>, <code>zsh</code>, <code>fish</code>, and PowerShell from a single YAML or JSONC config.
</p>

## Why you should choose shaka?

Managing shell shortcuts becomes increasingly difficult as environments, shells, and projects multiply.

What begins as a few aliases in a single shell profile often evolves into parallel definitions across `.zshrc`, `.bashrc`, `config.fish`, and PowerShell profiles. Once project-specific commands are added, the setup becomes fragmented, repetitive, and harder to maintain consistently.

This typically leads to a few recurring problems:

- the same shortcut must be maintained in different shell syntaxes
- personal and project-level commands become distributed across multiple files
- changes made in one shell environment do not automatically carry over to others
- onboarding a new machine requires rebuilding the same setup manually
- PowerShell introduces different behavior and compatibility concerns from POSIX-style shells

`shaka` addresses this by allowing you to define shortcuts once in a single YAML or JSONC file and generate the appropriate shell-specific output when needed.

Key benefits:

- one source of truth for command shortcuts
- shell-specific output from the same config
- project-level overrides on top of global defaults
- less manual profile editing
- PowerShell-specific handling for alias conflicts and env var expansion

## Quick Start

The example below uses `zsh`, but the same workflow applies to the other supported shells.

1. Create a global config file at `~/.config/shaka.yaml`:

```yaml
dc: docker compose
gs: git status
```

2. Ask `shaka` to generate shell code for your shell:

```sh
shaka zsh
```

Example output:

```sh
alias dc='docker compose'
alias gs='git status'
```

3. Evaluate that output in your shell so the aliases become available in the current session:

```sh
eval "$(shaka zsh)"
```

4. Use the alias:

```sh
gs
```

Example behavior:

```sh
$ gs
git status
On branch main
nothing to commit, working tree clean
```

To make this automatic every time you open a shell, add the same `eval "$(shaka zsh)"` line to your shell profile.

## Installation

Install from the current source checkout:

```sh
cargo install --path .
```

If `shaka` is published on crates.io, install it as a package:

```sh
cargo install shaka
```

Build and run locally during development:

```sh
cargo run -- zsh
```

## Usage

`shaka` prints shell code to standard output:

```sh
shaka <bash|fish|pwsh|pwsh-conflict|zsh>
```

Typical usage:

- `bash`

  ```sh
  eval "$(shaka bash)"
  ```

- `zsh`

  ```sh
  eval "$(shaka zsh)"
  ```

- `fish`

  ```sh
  shaka fish | source
  ```

- `pwsh`

  ```sh
  Invoke-Expression (& shaka pwsh | Out-String)
  ```

If the shell argument is missing or unsupported, `shaka` exits with an error and prints the expected usage string.

## Supported Shells

- `bash`
- `fish`
- `pwsh`
- `zsh`

## Configuration

`shaka` loads configuration files in the following order. Later files override earlier files.

### Global

User-level configuration:

- `~/.config/shaka.yaml`
- `~/.config/shaka.json`
- `~/.shaka.yaml`
- `~/.shaka.json`

### Project

Project-level configuration from the current directory. These files have higher priority than global configuration:

- `./.shaka.yaml`
- `./.shaka.json`

## Configuration Format

`shaka` accepts either YAML or JSONC.

Map form:

```yaml
dc: docker compose
gs: git status
```

Pair-list form:

```yaml
- [dc, docker compose]
- [gs, git status]
```

JSONC form:

```jsonc
{
  // comments are allowed
  "dc": "docker compose",
  "gs": "git status"
}
```

## Precedence

Global config is useful for personal defaults:

```yaml
# ~/.config/shaka.yaml
dc: docker compose
ls: eza
```

Project config can override or extend it:

```yaml
# ./.shaka.yaml
dc: docker compose -f dev.yml
test: cargo test
```

The merged result will behave as if you defined:

```yaml
ls: eza
dc: docker compose -f dev.yml
test: cargo test
```

## Output

`shaka` outputs shell code that you evaluate in your shell profile or startup script.

For the config:

```yaml
dc: docker compose
```

`bash`, `zsh`, and `fish` render aliases:

```sh
alias dc='docker compose'
```

PowerShell renders functions:

```sh
Remove-Alias -Name dc -Force -ErrorAction SilentlyContinue
function dc { docker compose @args }
```

## PowerShell Modes

By default, `shaka pwsh` removes an existing alias with the same name before defining the function. This avoids conflicts with built-in aliases.

If you want to keep built-in aliases and only emit functions, use:

```sh
Invoke-Expression (& shaka pwsh-conflict | Out-String)
```

That renders:

```sh
function dc { docker compose @args }
```

`shaka` uses `Remove-Alias` for cleanup in PowerShell mode.

## PowerShell Environment Variable Expansion

In `pwsh` output mode, `shaka` expands environment variables in command values before rendering functions.

- Supported forms: `$NAME` and `$env:NAME`
- Missing variables are left unchanged
- Expansion is only applied for `pwsh`; `bash`, `fish`, and `zsh` outputs are unchanged

Example input:

```yaml
ocd: $HOME/scoop/apps/opencode-desktop/current/OpenCode
```

Example output:

```sh
function ocd { C:\Users\Sayad/scoop/apps/opencode-desktop/current/OpenCode @args }
```

## Contributing

Issues and pull requests are welcome.

For local verification:

```sh
cargo test
```

## License

MIT
