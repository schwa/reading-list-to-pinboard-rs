# Reading List to Pinboard

This project uploads your Safari Reading List to <http://raindrop.io> and/or <https://pinboard.in>. It also supports exporting bookmarks to various formats.

## Installation

You'll need rust installed first, obviously

```sh
git clone http://github.com/schwa/reading-list-to-pinboard-rs
cd reading-list-to-pinboard-rs
cargo install --path .
```

Or just:

```sh
cargo install --git http://github.com/schwa/reading-list-to-pinboard-rs
```

## Usage

Make sure you either have `PINBOARD_TOKEN` or `RAINDROP_TOKEN` set in your environment. Alternatively you can have a `.env` file in your home directory with the following contents:

```env
PINBOARD_TOKEN="<username>:<token>"
RAINDROP_TOKEN="<token>"
```

```sh
reading-list-to-pinboard
```

The command will read your Safari Reading List and then prompt you to upload it to Pinboard.in, Raindrop.io, and/or export it to a markdown file (current at `$HOME/Notes/Daily Notes/<date>.md`).

This command *will not* delete your Safari Reading List items (Apple doesn't not expose a way to do this safely) so you will have to do that manually in the browser.

## Limitations

The workflow is very brittle and is designed for *my* use case. This is not a robust tool that other folks should use out of the box - but instead should serve as a (perhaps bad) starting point for their own projects.

See limitation above about deleting Safari Reading List items.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
