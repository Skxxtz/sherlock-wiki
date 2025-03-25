# Sherlock-wiki
Sherlock wiki is a simple script to allow you to make a Wikipedia API request from sherlock directly. 

## Usage
1. Download the script and build it using `cargo build --release`.
2. Place the resulting file `sherlock-wiki/target/release/sherlock-wiki` into your desired location (for example `~/.config/sherlock/scripts/sherlock-wiki`).
3. Now refer to it in your `fallback.json` file like so:
```
    {
        "name": "Wikipedia Search",
        "alias": "wiki",
        "type": "bulk_text",
        "on_return": "next",
        "async": true, 
        "args": {"icon": "wikipedia", "exec": "/home/basti/.config/sherlock/scripts/sherlock-wiki", "exec-args": "'{keyword}'"},
        "priority": 0 
    }
```
> NOTE: to use an icon that's not included in your standard icon theme you must set a custom icon path within your `config.toml` file. See [Example Config]("https://github.com/Skxxtz/sherlock/blob/main/docs/examples/config.toml") ``
