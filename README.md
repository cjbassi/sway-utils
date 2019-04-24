# sway-utils

Miscellaneous commands for use with Swaywm.

#### Subcommands:

##### focused-window-pwd

Prints the cwd of the focused window.

Can be used to open a new terminal in the same directory as the currently focused one. Example keybind:

```
bindsym $mod+Shift+Return exec alacritty --working-directory $(sway-utils focused-window-pwd)
```

## Installation

### Prebuilt binaries:

Note: (currently only a binary for Linux-x86_64 is available)

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/sway-utils
```

### From source:

```bash
cargo install --git https://github.com/cjbassi/sway-utils
```

## TODO

- subcommand to kill an application like Chrome
