# drg-info

Display matrix core unlock progression information from a Deep Rock Galactic save file, using [localcc/deeprockgalactic-saveeditor](https://github.com/localcc/deeprockgalactic-saveeditor).

# Usage

```
drg-info 1.0

djanatyn <djanatyn@gmail.com>

Command line interface to localcc/deeprockgalactiic-saveeditor features

USAGE:
    drg-info <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    missing    Output missing overclocks and cosmetics from save file as JSON
    parse      Parse save file and output as JSON 
```


``` sh
; drg-info missing "$DRG_SAVE_FILE" \
    | jq '{ missing_overclocks: .overclocks | length, missing_cosmetics: .cosmetics | length}'
```
```json
{
  "missing_overclocks": 34,
  "missing_cosmetics": 41
}

```

```sh
; drg-info missing "$DRG_SAVE_FILE" \
    | jq '.overclocks | first'
``` 
```json
{
  "class": "Scout",
  "weapon": "DRAK-25 Plasma Carbine",
  "name": "Aggressive Venting",
  "cost": {
    "credits": 8000,
    "bismor": 130,
    "croppa": 95,
    "enor": 80,
    "jadiz": 0,
    "magnite": 0,
    "umanite": 0
  },
  "state": "Unacquired"
}
```

# Building

```bash
; git clone https://github.com/djanatyn/drg-info
; cd drg-info && cargo run -- help
```


# Related
* https://github.com/localcc/deeprockgalactic-saveeditor
* https://github.com/rob0rt/drg-save-parser
* https://github.com/spicyboys/drg-completionist
* https://drg-completionist.com/
