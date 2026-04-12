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
