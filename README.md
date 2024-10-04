# Sclip

A little tool to speak out the clipboard content, depends on `xclip` and `espeak`, thus only works on Linux with X11.

## Why not just `xclip -o | espeak`?

This tool interrupts last speaking process when new content is copied.

## Usage

```text
$ sclip --help

Usage: sclip [OPTIONS]

Options:
  -t, --time <TIME>            Time interval for checking clipboard content in milliseconds [default: 300]
  -s, --selection <SELECTION>  The selection of `xclip` to read from [default: clipboard] [possible values: primary, secondary, clipboard, buffer-cut]
  -i, --initial                If true, when the clipboard is not empty, read it out immediately
  -h, --help                   Print help
```

## Development

You'll need to have `xclip` and `espeak` installed.

```bash
git clone https://github.com/Patchethium/sclip.git

cd sclip

cargo run

cargo build --release
```

## License

WTFPL
