# Saturn V Exterior Visual Pass — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the exterior 3D Saturn V historically accurate and visually convincing for Apollo 11 SA-506.

**Architecture:** Keep the existing procedural builder in `src/spacecraft/mod.rs`; replace/extend materials and geometry inline. Tune exterior rendering in `src/main.rs`.

**Tech Stack:** Rust, Bevy 0.14, PBR materials.

---

### Task 1: Fix material palette

**Files:**
- Modify: `src/spacecraft/mod.rs:153-209`

- [ ] **Step 1: Replace material definitions**

Replace the first block of material definitions in `spawn_saturn_v` with a historically accurate palette:

```rust
let insulation_white = materials.add(StandardMaterial {
    base_color: Color::srgb(0.94, 0.94, 0.92),
    metallic: 0.0,
    perceptual_roughness: 0.85,
    ..default()
});
let thermal_black = materials.add(StandardMaterial {
    base_color: Color::srgb(0.08, 0.08, 0.08),
    metallic: 0.0,
    perceptual_roughness: 0.75,
    ..default()
});
let foil_silver = materials.add(StandardMaterial {
    base_color: Color::srgb(0.72, 0.72, 0.74),
    metallic: 1.0,
    perceptual_roughness: 0.25,
    ..default()
});
let engine_bronze = materials.add(StandardMaterial {
    base_color: Color::srgb(0.55, 0.30, 0.15),
    metallic: 1.0,
    perceptual_roughness: 0.35,
    ..default()
});
let heat_shield_mat = materials.add(StandardMaterial {
    base_color: Color::srgb(0.25, 0.18, 0.10),
    metallic: 0.0,
    perceptual_roughness: 0.95,
    ..default()
});
let gold_foil = materials.add(StandardMaterial {
    base_color: Color::srgb(0.75, 0.65, 0.25),
    metallic: 0.8,
    perceptual_roughness: 0.25,
    ..default()
});
let glow_ring = materials.add(StandardMaterial {
    base_color: Color::srgb(0.9, 0.3, 0.1),
    emissive: LinearRgba::new(0.8, 0.2, 0.05, 0.3),
    ..default()
});
```

Remove the old `white`, `black`, `silver`, `aluminum`, `rust`, `heat_shield`, `copper`, `copper_dark` variables. Subsequent tasks will rename references.

- [ ] **Step 2: Verify cargo check**

Run: `cargo check`
Expected: PASS (it will fail until downstream references are updated, which is fine for now).

---

### Task 2: Apply correct materials to all stages

**Files:**
- Modify: `src/spacecraft/mod.rs:237-1067`

- [ ] **Step 1: S-IC materials**

Use `thermal_black` for thrust structure, aft skirt, intertank, forward skirt.
Use `insulation_white` for LOX and RP1 tank sections.
Use `foil_silver` for junction rings, stabilizing fins, engine gimbals, injector, feed lines.
Use `engine_bronze` for F-1 nozzle and throat.
Use `glow_ring` for the engine glow.

- [ ] **Step 2: Remove visible internal tanks**

Delete the `ic_lox_tank` and `ic_rp1_tank` spawns (around lines 615-628). The exterior tanks are the body sections; the internal tank meshes overlap and look wrong.

- [ ] **Step 3: S-II materials**

Change S-II engine skirt and body from `aluminum` to `insulation_white`.
Change J-2 nozzles from `black` to `engine_bronze`.
Use `thermal_black` for interstage2.
Delete `s2_lox_tank` and `s2_lh2_tank` spawns (around lines 709-722).

- [ ] **Step 4: S-IVB materials**

Change S-IVB aft skirt and body from `silver`/`aluminum` to `insulation_white`.
Change J-2 nozzle from `black` to `engine_bronze`.
Delete `ivb_lox_tank` and `ivb_lh2_tank` spawns (around lines 773-786).

- [ ] **Step 5: IU materials**

Keep IU body `foil_silver`. Use `foil_silver` for bumps and LVDC boxes.

- [ ] **Step 6: SLA, SM, CM, LES materials**

Change SLA material from light gray to `thermal_black`.
Change SM body to `insulation_white`.
Change SPS nozzle to `engine_bronze`.
Change CM body from `silver` to `insulation_white`.
Change CM heat shield material to `heat_shield_mat`.
Change BPC to `insulation_white`.
Keep LES lower white, upper black (use `insulation_white` and `thermal_black`).

- [ ] **Step 7: Update flag/USA materials**

For the flag background and stripes, remove `emissive` and `unlit`. Use normal PBR materials.
For the blue field, remove emissive.
For USA letters, use `thermal_black`.

- [ ] **Step 8: cargo check**

Run: `cargo check`
Expected: PASS

---

### Task 3: Improve engine bell geometry

**Files:**
- Modify: `src/spacecraft/mod.rs:245-332`

- [ ] **Step 1: F-1 nozzle stack**

Replace the single F-1 cone with a two-part bell:

```rust
let f1_nozzle_upper_radius = 1.1 * SATURN_V_SCALE;
let f1_nozzle_lower_radius = 1.83 * SATURN_V_SCALE;
let f1_nozzle_height = 0.56 * SATURN_V_SCALE;
let f1_upper_height = f1_nozzle_height * 0.45;
let f1_lower_height = f1_nozzle_height * 0.55;
let f1_upper_mesh = meshes.add(Cone { radius: f1_nozzle_upper_radius, height: f1_upper_height });
let f1_lower_mesh = meshes.add(Cone { radius: f1_nozzle_lower_radius, height: f1_lower_height });
```

Spawn the upper cone with `engine_bronze` and the lower cone with `thermal_black` (nozzle extension). Position them so they meet.

- [ ] **Step 2: J-2 nozzle stack**

Do the same for J-2 engines in S-II and S-IVB:

```rust
let j2_nozzle_upper_radius = 0.65 * SATURN_V_SCALE;
let j2_nozzle_lower_radius = 1.0 * SATURN_V_SCALE;
let j2_nozzle_height = 0.34 * SATURN_V_SCALE;
let j2_upper_height = j2_nozzle_height * 0.5;
let j2_lower_height = j2_nozzle_height * 0.5;
```

- [ ] **Step 3: SPS nozzle improvement**

Change SPS nozzle material to `engine_bronze`.

- [ ] **Step 4: cargo check**

Run: `cargo check`
Expected: PASS

---

### Task 4: Add surface detail

**Files:**
- Modify: `src/spacecraft/mod.rs`

- [ ] **Step 1: S-IC stabilizing fins**

Replace the simple cuboid fins with tapered wedges. Use `thermal_black` for the fin body and add a small silver leading edge.

- [ ] **Step 2: S-II stringers**

Add 8 thin vertical strips around the S-II body using `thermal_black` or darker insulation. Position them evenly around the circumference, slightly outside the body radius.

- [ ] **Step 3: S-IVB stringers**

Add 8 similar vertical strips to the S-IVB body.

- [ ] **Step 4: IU ribbing**

Add 2 horizontal thin rings around the IU body using `foil_silver`.

- [ ] **Step 5: SM panel seams**

Add 4 vertical panel seam strips around the SM body using a slightly darker white material (e.g. `Color::srgb(0.88, 0.88, 0.86)` with same roughness).

- [ ] **Step 6: cargo check**

Run: `cargo check`
Expected: PASS

---

### Task 5: Improve markings

**Files:**
- Modify: `src/spacecraft/mod.rs:456-595`

- [ ] **Step 1: Improve USA letters**

Keep the blocky style but make letters taller and slightly thicker. Use `thermal_black`.

- [ ] **Step 2: Improve flag**

Remove `unlit` and `emissive` from flag materials. Add white star dots in the blue canton using tiny white cubes. Keep 7 red stripes and 6 white stripes.

- [ ] **Step 3: cargo check**

Run: `cargo check`
Expected: PASS

---

### Task 6: Tune exterior camera and lighting

**Files:**
- Modify: `src/main.rs:185-278`

- [ ] **Step 1: Improve exterior exposure and ambient**

In `update_orbit_camera`, change:

```rust
ambient.brightness = 4.5;
exp.ev100 = 9.0;
```

This helps the black sections read clearly against the sky.

- [ ] **Step 2: Increase exterior far plane**

In the same function, ensure `p.far = 20000.0` so the full stack is visible from far away.

- [ ] **Step 3: cargo check**

Run: `cargo check`
Expected: PASS

---

### Task 7: Final verification

**Files:**
- All modified files

- [ ] **Step 1: cargo clippy**

Run: `cargo clippy -- -D warnings`
Expected: PASS

- [ ] **Step 2: cargo test**

Run: `cargo test`
Expected: PASS

- [ ] **Step 3: Build release**

Run: `cargo build --release`
Expected: PASS

- [ ] **Step 4: Manual visual check**

Run: `cargo run --release`
Switch to exterior camera (default). Verify:
- S-IC has black bottom and black intertank band
- S-II and S-IVB are white
- SLA is black
- CM is white with dark heat shield
- Engine bells have bronze/black two-tone
- USA and flag are visible on S-IC

---

## Self-review

- Spec coverage: all A-E issues addressed (shape, materials, detail, lighting, historical colors).
- No placeholders.
- Type consistency: all materials are `Handle<StandardMaterial>`.
