# mixplaylist

Reads an Ableton Live project file (`.als`) and prints a timestamped playlist of every clip in the arrangement, sorted by start position.

## Usage

```sh
cargo run -- <file.als>
```

Output:

```
[00:00] Flea - Golden Wingship
[00:37] clown core - google your own death (live)
[01:57] Slikback - EO
...
```

## How it works

Ableton `.als` files are gzip-compressed XML. The tool decompresses the file, reads the project BPM from `<Tempo>`, then collects every `<AudioClip>` and `<MidiClip>` from the arrangement along with their `Time` position (in beats) and `Name`. Beat positions are converted to `MM:SS` using the project tempo.

## License

MIT
