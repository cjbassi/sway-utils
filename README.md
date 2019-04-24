# sway-utils

Miscellaneous commands for use with Swaywm.

### Subcommands:

#### focused-program-cwd

Prints the cwd of the focused program.

Can be used to open a new terminal in the same directory as the currently focused one. Example keybind:

```
bindsym $mod+Shift+Return exec alacritty --working-directory $(sway-utils focused-window-cwd)
```

#### focused-program-kill

Kills the focused program.

Used to close the focused program, not just the focused window. Useful for closing Chrome and other multi window programs. Example keybind:

```
bindsym $mod+Shift+w exec sway-utils focused-program-kill
```

#### focused-program-pid

Prints the PID of the focused program.

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
