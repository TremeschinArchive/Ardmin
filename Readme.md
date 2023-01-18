👆【☰】Table of Contents

<div align="center">
    <h1>Ardmin</h1>
</div>

Ardour Session Minimizer, a blazing fast small CLI tool to simplify (in size) music sessions folders by deleting unused sources (WAV, MIDI), old plugin states and _somewhat_ non important files.

<sub><i><b>Note:</b> Backup, backup, backup, backup before using this tool.</i></sub>

# ● Installation

### ▸ Precompiled binary
Grab the latest binary release [here](https://github.com/BrokenSource/Ardmin/releases/latest)

### ▸ Running from the Source Code
Head over to the [Protostar Monorepo](https://github.com/Tremeschin/Protostar) for instructions on bootstrapping and running the Projects.

# ● Usage

Tested Ardour versions: `7.2-1`

```
$ ardmin -h
Ardmin, an Ardour Session Minimizer.

(c) 2023 Tremeschin, MIT License.

Usage: Ardmin [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>        (Global      ) Path to a Folder of Ardour Sessions
  -a, --all                (Global      ) Apply all optimizations
  -u, --unused             (Optimization) Remove unused Source files (MIDI, WAV)
  -s, --states             (Optimization) Remove old plugin states (5% chance of breaking per-plugin??)
  -b, --backup             (Optimization) Remove backup (.bak) of sessions
      --history            (Optimization) Remove history (.history) of sessions
  -r, --residuals          (Optimization) Remove analysis, dead, peaks folders
  -h, --help               Print help
  -V, --version            Print version
```

Apply all optimizations possible:
- `./ardmin -p ~/Path/To/Ardour/Sessions -a`

Apply only states and unused optimizations:
- `./ardmin -p ~/Path/To/Ardour/Sessions -s -u`

# ● Warnings

<i><b>Note:</b> Plugin states optimization behaviour seems a bit random, at times it might change a setting or two on the current presets or break entirely. Only happened with Vital and TAL Filter 2 for me, rarely.</i>

# ● License

AGPLv3-only License.
