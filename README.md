# shaka

`shaka` generates shell code for aliases and functions from config files.

## Configuration

`shaka` loads configuration files in the following order:

### Global

This is user level configuration.

- ~/.config/shaka.yaml
- ~/.config/shaka.json
- ~/.shaka.yaml
- ~/.shaka.json

### Project

This is project-level configuration (based on the current directory). It has higher priority than global configuration.

- ./.shaka.yaml
- ./.shaka.json

## Supported Shells

- bash
- fish
- pwsh (PowerShell)
- zsh

## Example

- `~/.config/shaka.yaml` example:

  ```yaml
  dc: docker compose
  ```

- `bash` example:

  ```sh
  eval "$(shaka bash)"
  ```

- `fish` example:

  ```sh
  shaka fish | source
  ```

- `pwsh` (PowerShell) example:

  ```sh
  Invoke-Expression (& shaka pwsh | Out-String)
  ```

- `zsh` example:
  ```sh
  eval "$(shaka zsh)"
  ```

## PowerShell Alias Conflict

By default, `shaka pwsh` omits built-in aliases to avoid conflicts. If you want to include them, use:

```sh
Invoke-Expression (& shaka pwsh-conflict | Out-String)
```

## PowerShell Environment Variable Expansion

In `pwsh` output mode, `shaka` expands environment variables in command values before rendering functions.

- Supported forms: `$NAME` and `$env:NAME`
- Example input:

  ```yaml
  ocd: $HOME/scoop/apps/opencode-desktop/current/OpenCode
  ```

  can render to:

  ```sh
  function ocd { C:\Users\Sayad/scoop/apps/opencode-desktop/current/OpenCode @args }
  ```

- If a variable is missing, the token is left unchanged
- This expansion is only applied for `pwsh`; `bash`, `fish`, and `zsh` outputs are unchanged

## Output

`shaka` outputs shell code that defines aliases and functions based on the configuration files. Evaluate the output in your shell to set them up.

- For example, the previous configuration example will create an alias `dc` that runs `docker compose`.
  - `bash`, `fish`, and `zsh`

    ```sh
    alias dc='docker compose'
    ```

  - `pwsh` (PowerShell)

    ```sh
    Remove-Alias -Name dc -Force -ErrorAction SilentlyContinue
    function dc { docker compose @args }
    ```

  - `pwsh` (PowerShell) with built-in aliases included

    ```sh
    function dc { docker compose @args }
    ```

`Remove-Alias` is used in `pwsh` mode instead of `Remove-Item` for alias cleanup.

## Cargo

Build and run locally:

```sh
cargo run -- pwsh
```

Install from the current source checkout:

```sh
cargo install --path .
```

If `shaka` is published on crates.io, install it as a package:

```sh
cargo install shaka
```
