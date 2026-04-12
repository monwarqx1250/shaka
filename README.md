# shaka

## Configuration

`shaka` looks for configuration files in the following order:

### Global

This is user level configuration.

- ~/.config/shaka.yaml
- ~/.config/shaka.json
- ~/.shaka.yaml
- ~/.shaka.json

### Project

This is project level configuration (based on the current directory). It will override global configuration.

- ./.shaka.yaml
- ./.shaka.json

## Supported Shells

- bash
- fish
- pwsh
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

- `pwsh` (powershell or pwsh) example:

  ```pwsh
  Invoke-Expression (&shaka pwsh | Out-String)
  ```

- `zsh` example:
  ```zsh
  eval "$(shaka zsh)"
  ```

## Powershell Alias Conflict

By default `shaka pwsh` omits builtin aliases to avoid conflicts. If you want to include them, use:

```pwsh
Invoke-Expression (&shaka pwsh-conflict | Out-String)
```
