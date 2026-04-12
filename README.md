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

  ```bash
  eval "$(shaka bash)"
  ```

- `fish` example:

  ```fish
  shaka fish | source
  ```

- `pwsh` (PowerShell) example:

  ```pwsh
  Invoke-Expression (&shaka pwsh | Out-String)
  ```

- `zsh` example:
  ```zsh
  eval "$(shaka zsh)"
  ```

## PowerShell Alias Conflict

By default, `shaka pwsh` omits built-in aliases to avoid conflicts. If you want to include them, use:

```pwsh
Invoke-Expression (&shaka pwsh-conflict | Out-String)
```

## Output

`shaka` outputs shell code that defines aliases and functions based on the configuration files. Evaluate the output in your shell to set them up.

- For example, the previous configuration example will create an alias `dc` that runs `docker compose`.
  - `bash`, `fish`, and `zsh`

    ```bash
    alias dc='docker compose'
    ```

  - `pwsh` (PowerShell)

    ```pwsh
    Remove-Item Alias:dc -ErrorAction Ignore
    function dc { docker compose @args }
    ```

  - `pwsh` (PowerShell) with built-in aliases included

    ```pwsh
    function dc { docker compose @args }
    ```
