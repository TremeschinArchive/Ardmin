👆【☰】Table of Contents

<div align="center">
    <h1>Ardmin</h1>
</div>

Ardour Session Minimizer, a small CLI tool to simplify (in size) an Ardour Session Folder by deleting unused sources (WAV, MIDI), old plugin states and _somewhat_ non important files.

# ● Installation

## ▸ Precompiled binary
Grab the latest binary release [here](https://github.com/BrokenSource/Ardmin/releases/latest)

## ▸ Running from the Source Code
Head over to the [Protostar Monorepo](https://github.com/Tremeschin/Protostar) for instructions on bootstrapping and running the Projects.

# ● Usage

```
$ ardmin -h
Ardmin, an Ardour Session Minimizer.

(c) 2023 Tremeschin, MIT License.

Usage: Ardmin [OPTIONS] --path <PATH>

Options:
  -p, --path <PATH>        (Global      ) Path to a Folder of Sessions
  -a, --all                (Global      ) Apply all optimizations
  -u, --unused             (Optimization) Remove unused Source files (MIDI, WAV)
  -s, --states             (Optimization) Remove old plugin states (5% chance of breaking session??)
  -b, --backup             (Optimization) Remove backup (.bak) of sessions
      --history            (Optimization) Remove history (.history) of sessions
  -r, --residuals          (Optimization) Remove analysis, dead, peaks folders
  -h, --help               Print help
  -V, --version            Print version
```

Apply all optimizations possible:
- `./ardmin -p ~/AudioLabs/ArdourSessions -a`



<br>

# ● Licenses / Licensing

## ⏵ Code
**Currently all code is PROPRIETARY, no copying allowed, only reading if I gave you personally access or the repo itself**

**The files and root license says AGPLv3-only because this project will be open source in the future.**

