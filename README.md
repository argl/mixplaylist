# mixplaylist

Reads an Ableton Live project file (`.als`) and prints a timestamped playlist of every clip in the arrangement, sorted by start position. This is a somewhat standardized playlist format, suitable for usage in Mixcloud or other contexts.

## Usage

```sh
cargo run -- <file.als>

# or build it and install it into one of your `PATH`s 
cargo build -r
cp target/release/mixplaylist /usr/local/bin
```

Output:

```
[00:00] Some clip name
[00:37] Some other clip name
[01:57] Third clip
...
```

## License

MIT
