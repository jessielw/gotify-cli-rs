# Gotify-CLI

A CLI to send notifications to Gotify supporting proper markdown built in rust

## Usage

```
Light CLI tool to send notifications to your Gotify server

Usage: gotify-cli.exe [OPTIONS] --server <SERVER> --app-key <APP_KEY> --title <TITLE> --message <MESSAGE>

Options:
  -s, --server <SERVER>      Gotify server URL
  -a, --app-key <APP_KEY>    Gotify app token
  -t, --title <TITLE>        Title of the message
  -m, --message <MESSAGE>    Message to send (supports markdown)
  -p, --priority <PRIORITY>  Priority level [default: 5]
  -h, --help                 Print help
  -V, --version              Print version
```
