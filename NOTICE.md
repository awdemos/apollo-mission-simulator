# Third-Party Attribution and Licensing

This project incorporates or links against the following third-party software
and assets. All trademarks and copyrights belong to their respective owners.

## Software

### yaAGC - Virtual Apollo Guidance Computer
- **Copyright**: 2003-2024 Ronald S. Burkey and contributors
- **License**: GNU General Public License v2.0 or later
- **Source**: https://www.ibiblio.org/apollo/
- **Repository**: https://github.com/rburkey2005/virtualagc
- **Usage**: Statically linked (`libyaAGC.a`) for AGC emulation and DSKY interface

### Bevy Game Engine
- **Copyright**: Bevy Contributors
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Source**: https://bevyengine.org/
- **Usage**: Core game engine and ECS framework

### bevy_egui
- **Copyright**: Jakub Hlusička and contributors
- **License**: MIT
- **Source**: https://github.com/mvlabat/bevy_egui
- **Usage**: Immediate-mode GUI for debug panels and DSKY overlay

### nalgebra
- **Copyright**: Sébastien Crozet and contributors
- **License**: BSD-3-Clause
- **Source**: https://nalgebra.org/
- **Usage**: Linear algebra for orbital mechanics

### chrono
- **Copyright**: chrono contributors
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Usage**: Date/time handling for mission epochs

### Rust Standard Library and Crates
- anyhow, thiserror, tracing, rand, cc
- Various open-source licenses (MIT, Apache-2.0)

## Apollo Guidance Computer Software

The following AGC binaries are included as assets and are believed to be in the
public domain as works of the United States Government:

- **Comanche055** (Apollo 11 Command Module AGC)
- **Luminary099** (Apollo 11 Lunar Module AGC)
- **Source**: NASA / MIT Instrumentation Laboratory
- **License**: Public Domain (US Government work)
- **Digital Archive**: https://www.ibiblio.org/apollo/

## Textures and Visual Assets

- **Earth texture** (`assets/textures/earth.jpg`): 8192x4096 equirectangular map
  - License status: **REQUIRES VERIFICATION** - believed to be NASA Visible Earth
    (public domain) or similar freely-licensed source, but origin not confirmed.
  - Action needed: Replace with confirmed public-domain texture if provenance
    cannot be established.

- **Moon texture** (`assets/textures/moon.jpg`): 8192x4096 equirectangular map
  - License status: **REQUIRES VERIFICATION** - believed to be NASA LRO WAC
    (public domain) or similar freely-licensed source, but origin not confirmed.
  - Action needed: Replace with confirmed public-domain texture if provenance
    cannot be established.

## Historical Documentation

Lunar orbital elements, spacecraft dimensions, and panel layouts are derived from
declassified NASA technical documentation including:
- Apollo Operations Handbook (SM2A-03-Block II)
- NASA Technical Reports
- CuriousMarc Apollo Comms restoration video series

These facts and measurements are not subject to copyright.

## Disclaimer

This is a fan-made educational simulator. It is not affiliated with, endorsed by,
or sponsored by NASA, the United States Government, or any Apollo program
contractor. All Apollo-related trademarks are property of their respective owners.
