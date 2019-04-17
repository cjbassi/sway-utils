# sway-utils

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

- subcommand to pwd of focused terminal
- subcommand to kill an application like Chrome
