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

### anyhow
- **Copyright**: David Tolnay and contributors
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Source**: https://github.com/dtolnay/anyhow
- **Usage**: Error handling and propagation

### thiserror
- **Copyright**: David Tolnay and contributors
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Source**: https://github.com/dtolnay/thiserror
- **Usage**: Derive macro for custom error types

### tracing
- **Copyright**: Tokio Contributors
- **License**: MIT
- **Source**: https://github.com/tokio-rs/tracing
- **Usage**: Structured logging and diagnostics

### rand
- **Copyright**: The Rust Rand Project Developers
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Source**: https://github.com/rust-random/rand
- **Usage**: Random number generation for fault simulation

### cc
- **Copyright**: Alex Crichton and contributors
- **License**: MIT OR Apache-2.0 (dual-licensed)
- **Source**: https://github.com/rust-lang/cc-rs
- **Usage**: Build dependency for compiling C FFI code

## Apollo Guidance Computer Software

The following AGC binaries are included as assets and are believed to be in the
public domain as works of the United States Government:

- **Comanche055** (Apollo 11 Command Module AGC)
- **Luminary099** (Apollo 11 Lunar Module AGC)
- **Source**: NASA / MIT Instrumentation Laboratory
- **License**: Public Domain (US Government work)
- **Digital Archive**: https://www.ibiblio.org/apollo/

## Textures and Visual Assets

- **Earth texture** (`assets/textures/earth.jpg`): NASA Blue Marble Next Generation
  - **Source**: NASA Earth Observatory / Visible Earth
  - **URL**: https://eoimages.gsfc.nasa.gov/images/imagerecords/74000/74092/
  - **License**: Public Domain (NASA)
  - **Credit**: NASA Earth Observatory image by Reto Stöckli

- **Moon texture** (`assets/textures/moon.jpg`): NASA LROC WAC Global Morphologic Map
  - **Source**: NASA Lunar Reconnaissance Orbiter Camera (LROC)
  - **URL**: https://www.nasa.gov/missions/nasa-goddard-creates-cgi-moon-kit/
  - **License**: Public Domain (NASA)
  - **Credit**: NASA/GSFC/SVS/Ernie Wright

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
