# Apollo Mission Simulator

> **WARNING: Very early, buggy experience awaits.**

A 3D Apollo mission simulator built with Rust and the Bevy game engine. Features
a historically accurate Command Module interior, interactive control panels, a
real yaAGC-powered Apollo Guidance Computer, and realistic orbital mechanics.

## Features

- **Historically Accurate CM Interior**: Conical frustum hull with accurate
  dimensions from NASA Operations Handbook (3.91m base, 3.48m height)
- **Interactive Panels**: Clickable switches, circuit breakers, DSKY keys, FDAI
  display, and event timer — all wired to spacecraft subsystems
- **Real AGC Integration**: Powered by the actual yaAGC emulator running
  Comanche055 (Apollo 11 CM software). The 3D DSKY is a real terminal for the
  AGC — press V37E and the AGC actually receives it
- **Realistic Lunar Orbit**: Elliptical, inclined orbit with accurate Keplerian
  mechanics, initialized to the Apollo 11 launch epoch (1969-07-16 13:32 UTC)
- **Fault System**: Cascading failures with repair procedures, ground control
  assistance, and difficulty progression
- **Historically Accurate Communications**: Unified S-Band radio with correct
  subcarriers (1.024 MHz data, 1.25 MHz voice), modulation, and telemetry frame
  format verified against CuriousMarc Apollo Comms restoration data

## Controls

- **WASD** — Move inside the capsule
- **Q/E/Space/Shift** — Vertical movement
- **RMB Hold** — Look around (interior mode)
- **LMB** — Click switches, breakers, DSKY keys
- **Escape** — Unlock cursor
- **Tab** — Switch camera mode (Interior / Exterior / Free)

## Building

Requires Rust 1.79+ and the yaAGC library built from
[virtualagc](https://github.com/rburkey2005/virtualagc).

```bash
cargo run --release
```

## License

This project is licensed under the GNU General Public License v2.0 or later.
See [LICENSE](LICENSE) and [NOTICE.md](NOTICE.md) for details.

The GPL is required because this project statically links with yaAGC, which is
GPL-licensed. The original Apollo Guidance Computer software (Comanche055,
Luminary099, etc.) is in the public domain as a work of the US Government.

## Attribution

- **yaAGC**: Ronald S. Burkey and the Virtual AGC Project
  (https://www.ibiblio.org/apollo/)
- **Bevy Engine**: Bevy Contributors (https://bevyengine.org/)
- **Historical Data**: NASA Operations Handbook, CuriousMarc Apollo Comms series
