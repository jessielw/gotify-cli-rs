# Gotify-CLI

A CLI to send notifications to Gotify supporting proper markdown built in rust

## Usage

```
Light CLI tool to send notifications to your Gotify server

Usage: gotify-cli-rs [OPTIONS] --server <SERVER> --app-key <APP_KEY> --title <TITLE> --message <MESSAGE>

Options:
  -s, --server <SERVER>      Gotify server URL
  -a, --app-key <APP_KEY>    Gotify app token
  -t, --title <TITLE>        Title of the message
  -m, --message <MESSAGE>    Message to send (supports markdown)
  -p, --priority <PRIORITY>  Priority level [default: 5]
  -h, --help                 Print help
  -V, --version              Print version
```

## Example Usage

```
gotify-cli-rs --server "https://gotify.randomserver.com" --app-key "APPLICATION_KEY" --title "Some Notification" --message "**Alert!!**\nYou have an alert!" --priority 8
```

## Markdown Information

Supports correctly sending markdown to the Gotify server. You should use `\n` for line breaks and gotify-cli-rs will automatically take care of spacing/proper line breaks. See [docs](https://github.github.com/gfm/) for specific markdown version supported by Gotify.
