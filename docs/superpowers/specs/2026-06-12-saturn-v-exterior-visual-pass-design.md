# Saturn V Exterior Visual Pass — Design

## Goal
Make the exterior 3D view of the Saturn V historically accurate and visually convincing for Apollo 11 (SA-506), fixing shape, materials, surface detail, markings, and rendering quality.

## Current State
- `src/spacecraft/mod.rs` procedurally builds the full Saturn V from Bevy primitives.
- No external rocket textures; all materials are `StandardMaterial` created in code.
- Dimensions in `src/config/mod.rs` are historically based.

## Problems Found
1. **Wrong colors / materials**
   - S-II body is silver aluminum; should be white insulation.
   - S-IVB body is silver aluminum; should be white insulation.
   - SLA is light gray/white; should be black.
   - Command Module is silver; should be white.
   - Several materials use `emissive` (silver, aluminum, flag) making surfaces self-illuminate.
2. **Visible internal tanks**
   - LOX/RP1/LH2 tank meshes are rendered outside the stage body and overlap exterior surfaces.
3. **Crude geometry**
   - F-1 and J-2 engine bells are single cones.
   - Stabilizing fins are simple cuboids.
   - USA letters and flag are flat black/white cuboids.
   - No panel lines, ribbing, or stage separation rings.
4. **Rendering**
   - Exterior camera exposure and sun/ambient light not tuned for the new PBR materials.

## Design
### Materials
Create a dedicated material palette in `spawn_saturn_v`:
- `insulation_white`: matte white, roughness 0.85 (S-II, S-IVB, CM, BPC).
- `thermal_black`: matte black, roughness 0.75 (S-IC skirts/intertank, SLA, engine bells).
- `foil_silver`: metallic foil, low emissive/no emissive (IU, interstage rings, tank skirts).
- `engine_copper`: metallic copper/bronze for nozzle interiors.
- `heat_shield`: dark rough brown/black for CM heat shield.

Remove all non-light emissive values from exterior materials.

### Geometry
- S-IC: keep white LOX/RP1 tanks, black aft skirt, black intertank, black forward skirt.
- S-II: change body to white insulation; add 8 vertical stringers / 2 horizontal rib rings.
- S-IVB: change body to white insulation; add 8 vertical stringers.
- IU: keep silver, add small equipment boxes and ribbing.
- SLA: change to black; add 4 vertical panel seams.
- SM: keep white, add RCS quad details and panel seam bands.
- CM: change to white insulation; add crew access hatch outline.
- LES: keep white lower/black upper; improve canard shape.
- Engine bells: stack two cones for F-1 (nozzle + extension) and J-2; add gimbal ring detail.
- Fins: replace cuboid fins with tapered wedge shapes.

### Markings
- Improve USA letters with slightly better proportions and place on S-IC white section.
- Improve flag geometry: 13 stripes, blue canton, simple star field.
- Add small black "United States" style stripe band near S-II forward skirt (optional if too noisy).

### Rendering
- In `src/main.rs` exterior camera mode: raise far plane if needed, tune exposure to ~13 stops, ensure directional sun casts shadows.
- Add subtle ambient light for exterior view so black sections are visible.

## Scope
- Exterior Saturn V only (`src/spacecraft/mod.rs` and `src/main.rs` camera/lighting).
- No new external textures; procedural geometry/materials only.
- Preserve existing dimensions and structural integrity components.

## Verification
- `cargo check` passes.
- `cargo clippy` passes.
- Run game, switch to exterior camera, confirm rocket reads as Apollo 11 Saturn V from a distance.
