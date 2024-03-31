# gpt-cli

gpt-cli is a command line interface for `Chat GPT` api.
It allows you to interact with `Chat GPT` from the command line.

## Build / Install

To build and install the project, run the following command:

```bash
cargo install --path .
```

## Usage

```bash
Usage: gpt-cli [OPTIONS] <COMMAND>

Commands:
  d      Default chat gpt
  t      Translation pre-promt (default language: en)
  c      Correction pre-promt (default language: en)
  usage  Display the total usage of tokens
  help   Print this message or the help of the given subcommand(s)

Options:
  -a, --append
  -h, --help     Print help
  -V, --version  Print version
```
