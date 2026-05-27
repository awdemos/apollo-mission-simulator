use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod config;

pub struct PanelsPlugin;

impl Plugin for PanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PanelInteraction>()
            .add_systems(Update, (
                handle_panel_clicks,
                animate_switch_positions,
                update_panel_lights,
                handle_dsky_input,
                update_event_timer,
                debug_panel_count,
            ));
    }
}

fn debug_panel_count(
    switches: Query<&PanelSwitch>,
    breakers: Query<&CircuitBreaker>,
    lights: Query<&PanelLight>,
    mut has_logged: Local<bool>,
) {
    let switch_count = switches.iter().count();
    let breaker_count = breakers.iter().count();
    let light_count = lights.iter().count();
    let total = switch_count + breaker_count + light_count;
    
    if total > 0 && !*has_logged {
        info!("Panels active: {} switches, {} breakers, {} lights", switch_count, breaker_count, light_count);
        *has_logged = true;
    }
}

#[derive(Event, Debug, Clone)]
pub enum PanelInteraction {
    SwitchToggled(Entity, SwitchId, SwitchState),
    BreakerToggled(Entity, BreakerId, bool),
    ButtonPressed(Entity, ButtonId),
    KeyPressed(Entity, DskyKeyType),
    RotaryChanged(Entity, RotaryId, u8),
}

#[derive(Component, Debug, Clone)]
pub struct PanelSwitch {
    pub id: SwitchId,
    pub state: SwitchState,
    pub label: String,
    pub target_system: SystemTarget,
    pub position: Vec3,
    pub rotation_axis: Vec3,
    pub hit_radius: f32,
}

#[derive(Component, Debug, Clone)]
pub struct DskyDisplay {
    pub prog: u8,
    pub verb: u8,
    pub noun: u8,
    pub r1: [DskyDigit; 5],
    pub r2: [DskyDigit; 5],
    pub r3: [DskyDigit; 5],
    pub r1_sign: DskySign,
    pub r2_sign: DskySign,
    pub r3_sign: DskySign,
    pub comp_acty: bool,
    pub lights: DskyStatusLights,
    pub keyboard_buffer: Vec<DskyKeyType>,
    pub entering_verb: bool,
    pub entering_noun: bool,
    pub enter_mode: DskyEnterMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DskyDigit {
    Blank,
    Zero, One, Two, Three, Four,
    Five, Six, Seven, Eight, Nine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DskySign {
    Blank,
    Plus,
    Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DskyEnterMode {
    Idle,
    EnteringVerb,
    EnteringNoun,
    EnteringData,
}

#[derive(Component, Debug, Clone)]
pub struct DskyStatusLights {
    pub uplink_acty: bool,
    pub temp: bool,
    pub no_att: bool,
    pub gimbal_lock: bool,
    pub stby: bool,
    pub prog: bool,
    pub key_rel: bool,
    pub restart: bool,
    pub opr_err: bool,
    pub tracker: bool,
}

impl Default for DskyDisplay {
    fn default() -> Self {
        Self {
            prog: 0,
            verb: 0,
            noun: 0,
            r1: [DskyDigit::Blank; 5],
            r2: [DskyDigit::Blank; 5],
            r3: [DskyDigit::Blank; 5],
            r1_sign: DskySign::Blank,
            r2_sign: DskySign::Blank,
            r3_sign: DskySign::Blank,
            comp_acty: false,
            lights: DskyStatusLights::default(),
            keyboard_buffer: Vec::new(),
            entering_verb: false,
            entering_noun: false,
            enter_mode: DskyEnterMode::Idle,
        }
    }
}

impl Default for DskyStatusLights {
    fn default() -> Self {
        Self {
            uplink_acty: false,
            temp: false,
            no_att: false,
            gimbal_lock: false,
            stby: false,
            prog: false,
            key_rel: false,
            restart: false,
            opr_err: false,
            tracker: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchId {
    RcsQuadA,
    RcsQuadB,
    RcsQuadC,
    RcsQuadD,
    ScsMode,
    TvCEnable,
    EngineArm,
    AbortMode,
    SpsEnable,
    FuelCell1,
    FuelCell2,
    FuelCell3,
    MainBusA,
    MainBusB,
    BatteryA,
    BatteryB,
    BatteryC,
    Inverter1,
    Inverter2,
    Inverter3,
    CmRcsHeaters,
    SmRcsHeaters,
    O2Fan1,
    O2Fan2,
    H2Fan1,
    H2Fan2,
    CryoPumps,
    CabinFan,
    ScePower,
    UllageThrust,
    SpsThrustOn,
    SpsThrustOff,
    RhcPower,
    ThcPower,
    ImuCage,
    ImuAlign,
    GncMode,
    EntryMonitor,
    DsePower,
    SBandPower,
    VhfPower,
    HighGainAntenna,
    TapeRecorder,
    EventTimerStart,
    EventTimerStop,
    EventTimerReset,
    InteriorLights,
    Floodlights,
    UvLights,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchState {
    Off,
    On,
    Auto,
    Momentary,
}

#[derive(Component, Debug, Clone)]
pub struct CircuitBreaker {
    pub id: BreakerId,
    pub closed: bool,
    pub label: String,
    pub target_system: SystemTarget,
    pub amp_rating: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BreakerId {
    FuelCell1MainBusA,
    FuelCell2MainBusA,
    FuelCell3MainBusB,
    BatteryRelayBus,
    BatteryAEntry,
    BatteryBEntry,
    BatteryCEntry,
    Inverter1,
    Inverter2,
    Inverter3,
    RcsQuadAHeater,
    RcsQuadBHeater,
    RcsQuadCHeater,
    RcsQuadDHeater,
    RcsQuadAProp,
    RcsQuadBProp,
    RcsQuadCProp,
    RcsQuadDProp,
    SpsPropellant,
    SpsHelium,
    SpsPilotValve,
    O2Tank1,
    O2Tank2,
    H2Tank1,
    H2Tank2,
    CryoFan1,
    CryoFan2,
    CabinFan1,
    CabinFan2,
    SceA,
    SceB,
    GncPlatform,
    ImuOper,
    ImuCage,
    CmcPower,
    IoaPower,
    IobPower,
    Dse,
    SBandTransmitter,
    SBandReceiver,
    SBandPowerAmp,
    VhfTransmitterA,
    VhfTransmitterB,
    VhfReceiver,
    HighGainAntenna,
    TapeRecorder,
    EventTimer,
    InteriorLightsFlood,
    InteriorLightsPanel,
    UvLight,
    Custom(String),
}

#[derive(Component, Debug, Clone)]
pub struct PushButton {
    pub id: ButtonId,
    pub label: String,
    pub target_system: SystemTarget,
    pub pressed: bool,
    pub momentary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ButtonId {
    Abort,
    AbortStage,
    DskyProceed,
    DskyKeyRel,
    DskyReset,
    DskyEnter,
    DskyClear,
    MasterAlarm,
    CautionWarningReset,
    SpsThrustOn,
    SpsThrustOff,
    JetPrimary,
    JetSecondary,
    TvGimbalDrivePitch1,
    TvGimbalDrivePitch2,
    TvGimbalDriveYaw1,
    TvGimbalDriveYaw2,
    Custom(String),
}

#[derive(Component, Debug, Clone)]
pub struct RotarySwitch {
    pub id: RotaryId,
    pub position: u8,
    pub num_positions: u8,
    pub labels: Vec<String>,
    pub target_system: SystemTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RotaryId {
    ScsMode,
    GncMode,
    EntryMode,
    RcsPropellant,
    SpsThrustProg,
    FDAISelect,
    FDAIScale,
    EventTimerMode,
    Custom(String),
}

#[derive(Component, Debug, Clone)]
pub struct PanelLight {
    pub label: String,
    pub lit: bool,
    pub color: LightColor,
    pub blink: bool,
    pub blink_timer: f32,
    pub target_system: SystemTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightColor {
    White,
    Green,
    Amber,
    Red,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTarget {
    Electrical,
    Rcs,
    Sps,
    LifeSupport,
    Guidance,
    Communications,
    Propulsion,
    None,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DskyKey {
    pub key: DskyKeyType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DskyKeyType {
    Verb,
    Noun,
    Plus,
    Minus,
    Number(u8),
    Clear,
    Enter,
    KeyRel,
    Reset,
    Pro,
}

#[derive(Component)]
pub struct InteractivePanel;

#[derive(Component, Debug, Clone)]
pub struct FdaiDisplay {
    pub roll_degrees: f32,
    pub pitch_degrees: f32,
    pub yaw_degrees: f32,
    pub roll_error: f32,
    pub pitch_error: f32,
    pub yaw_error: f32,
    pub roll_rate: f32,
    pub pitch_rate: f32,
    pub yaw_rate: f32,
    pub error_scale: FdaiErrorScale,
    pub rate_scale: FdaiRateScale,
    pub source: FdaiSource,
    pub gimbal_lock_warning: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdaiErrorScale {
    Narrow,
    Wide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdaiRateScale {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FdaiSource {
    Imu,
    Gdc,
    AttSet,
}

impl Default for FdaiDisplay {
    fn default() -> Self {
        Self {
            roll_degrees: 0.0,
            pitch_degrees: 0.0,
            yaw_degrees: 0.0,
            roll_error: 0.0,
            pitch_error: 0.0,
            yaw_error: 0.0,
            roll_rate: 0.0,
            pitch_rate: 0.0,
            yaw_rate: 0.0,
            error_scale: FdaiErrorScale::Narrow,
            rate_scale: FdaiRateScale::Low,
            source: FdaiSource::Imu,
            gimbal_lock_warning: false,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct EventTimer {
    pub minutes: u8,
    pub seconds: u8,
    pub counting: bool,
    pub count_up: bool,
    pub auto_start_at_liftoff: bool,
}

impl Default for EventTimer {
    fn default() -> Self {
        Self {
            minutes: 0,
            seconds: 0,
            counting: false,
            count_up: true,
            auto_start_at_liftoff: true,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct RotationalHandController {
    pub pitch_deflection: f32,
    pub yaw_deflection: f32,
    pub roll_deflection: f32,
    pub breakout_switches_active: [bool; 6],
    pub direct_switches_active: [bool; 6],
    pub powered: bool,
    pub mode: RhcMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RhcMode {
    Normal,
    Direct,
    Off,
}

#[derive(Component, Debug, Clone)]
pub struct TranslationalHandController {
    pub x_deflection: f32,
    pub y_deflection: f32,
    pub z_deflection: f32,
    pub powered: bool,
    pub locked: bool,
}

fn handle_panel_clicks(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut switch_query: Query<(Entity, &mut PanelSwitch, &GlobalTransform)>,
    mut breaker_query: Query<(Entity, &mut CircuitBreaker, &GlobalTransform)>,
    mut button_query: Query<(Entity, &mut PushButton, &GlobalTransform)>,
    mut key_query: Query<(Entity, &DskyKey, &GlobalTransform)>,
    mut rotary_query: Query<(Entity, &mut RotarySwitch, &GlobalTransform)>,
    mut interaction_events: EventWriter<PanelInteraction>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let Some(cursor_pos) = window.cursor_position() else { return };

    let (camera, camera_transform) = camera_query.single();
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return };

    let max_distance = 5.0;

    let ray_hit = |transform: &GlobalTransform, hit_radius: f32| -> Option<f32> {
        let panel_pos = transform.translation();
        let to_panel = panel_pos - ray.origin;
        let projected = to_panel.dot(*ray.direction);
        if projected < 0.0 || projected > max_distance {
            return None;
        }
        let closest_point = ray.origin + *ray.direction * projected;
        let distance = (closest_point - panel_pos).length();
        if distance < hit_radius {
            Some(projected)
        } else {
            None
        }
    };

    let mut closest_hit: Option<(f32, Entity, PanelInteraction)> = None;

    for (entity, switch, transform) in switch_query.iter_mut() {
        if let Some(dist) = ray_hit(transform, switch.hit_radius) {
            let new_state = match switch.state {
                SwitchState::Off => SwitchState::On,
                SwitchState::On => SwitchState::Off,
                SwitchState::Auto => SwitchState::On,
                SwitchState::Momentary => SwitchState::Momentary,
            };
            let interaction = PanelInteraction::SwitchToggled(entity, switch.id.clone(), new_state);
            if closest_hit.is_none() || dist < closest_hit.as_ref().unwrap().0 {
                closest_hit = Some((dist, entity, interaction));
            }
        }
    }

    for (entity, breaker, transform) in breaker_query.iter_mut() {
        if let Some(dist) = ray_hit(transform, 0.03) {
            let new_state = !breaker.closed;
            let interaction = PanelInteraction::BreakerToggled(entity, breaker.id.clone(), new_state);
            if closest_hit.is_none() || dist < closest_hit.as_ref().unwrap().0 {
                closest_hit = Some((dist, entity, interaction));
            }
        }
    }

    for (entity, button, transform) in button_query.iter_mut() {
        if let Some(dist) = ray_hit(transform, 0.04) {
            let interaction = PanelInteraction::ButtonPressed(entity, button.id.clone());
            if closest_hit.is_none() || dist < closest_hit.as_ref().unwrap().0 {
                closest_hit = Some((dist, entity, interaction));
            }
        }
    }

    for (entity, key, transform) in key_query.iter_mut() {
        if let Some(dist) = ray_hit(transform, 0.025) {
            let interaction = PanelInteraction::KeyPressed(entity, key.key);
            if closest_hit.is_none() || dist < closest_hit.as_ref().unwrap().0 {
                closest_hit = Some((dist, entity, interaction));
            }
        }
    }

    for (entity, rotary, transform) in rotary_query.iter_mut() {
        if let Some(dist) = ray_hit(transform, 0.05) {
            let new_pos = (rotary.position + 1) % rotary.num_positions;
            let interaction = PanelInteraction::RotaryChanged(entity, rotary.id.clone(), new_pos);
            if closest_hit.is_none() || dist < closest_hit.as_ref().unwrap().0 {
                closest_hit = Some((dist, entity, interaction));
            }
        }
    }

    if let Some((_dist, entity, interaction)) = closest_hit {
        match &interaction {
            PanelInteraction::SwitchToggled(_, _, new_state) => {
                if let Ok((_, mut switch, _)) = switch_query.get_mut(entity) {
                    switch.state = *new_state;
                }
            }
            PanelInteraction::BreakerToggled(_, _, new_state) => {
                if let Ok((_, mut breaker, _)) = breaker_query.get_mut(entity) {
                    breaker.closed = *new_state;
                }
            }
            PanelInteraction::ButtonPressed(_, _) => {
                if let Ok((_, mut button, _)) = button_query.get_mut(entity) {
                    button.pressed = true;
                }
            }
            PanelInteraction::RotaryChanged(_, _, new_pos) => {
                if let Ok((_, mut rotary, _)) = rotary_query.get_mut(entity) {
                    rotary.position = *new_pos;
                }
            }
            _ => {}
        }
        interaction_events.send(interaction);
    }
}

fn animate_switch_positions(
    mut switch_query: Query<(&PanelSwitch, &mut Transform), Without<CircuitBreaker>>,
    mut breaker_query: Query<(&CircuitBreaker, &mut Transform), Without<PanelSwitch>>,
    mut button_query: Query<(&PushButton, &mut Transform), (Without<PanelSwitch>, Without<CircuitBreaker>)>,
    mut rotary_query: Query<(&RotarySwitch, &mut Transform), (Without<PanelSwitch>, Without<CircuitBreaker>, Without<PushButton>)>,
) {
    for (switch, mut transform) in switch_query.iter_mut() {
        let target_rotation = match switch.state {
            SwitchState::Off => -0.4,
            SwitchState::On => 0.4,
            SwitchState::Auto => 0.0,
            SwitchState::Momentary => 0.0,
        };
        let axis = switch.rotation_axis.normalize_or_zero();
        if axis != Vec3::ZERO {
            let current = transform.rotation;
            let target = Quat::from_axis_angle(axis, target_rotation);
            transform.rotation = current.slerp(target, 0.3);
        }
    }

    for (breaker, mut transform) in breaker_query.iter_mut() {
        let target_y = if breaker.closed { 0.0 } else { 0.03 };
        transform.translation.y = transform.translation.y.lerp(target_y, 0.3);
    }

    for (button, mut transform) in button_query.iter_mut() {
        let target_z = if button.pressed { -0.01 } else { 0.0 };
        transform.translation.z = transform.translation.z.lerp(target_z, 0.5);
    }

    for (rotary, mut transform) in rotary_query.iter_mut() {
        let angle = (rotary.position as f32 / rotary.num_positions.max(1) as f32) * std::f32::consts::TAU;
        let target = Quat::from_rotation_y(angle);
        transform.rotation = transform.rotation.slerp(target, 0.3);
    }
}

fn update_panel_lights(
    mut light_query: Query<(&mut PanelLight, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (mut light, material_handle) in light_query.iter_mut() {
        let Some(material) = materials.get_mut(material_handle) else { continue };
        
        let base_color = match light.color {
            LightColor::White => Color::srgb(0.9, 0.9, 0.9),
            LightColor::Green => Color::srgb(0.2, 0.8, 0.2),
            LightColor::Amber => Color::srgb(0.9, 0.6, 0.1),
            LightColor::Red => Color::srgb(0.9, 0.1, 0.1),
            LightColor::Blue => Color::srgb(0.2, 0.4, 0.9),
        };

        if light.lit {
            if light.blink {
                light.blink_timer += time.delta_seconds();
                let blink_on = (light.blink_timer * 2.0).sin() > 0.0;
                material.base_color = if blink_on { base_color } else { Color::srgb(0.1, 0.1, 0.1) };
                material.emissive = if blink_on {
                    match light.color {
                        LightColor::White => LinearRgba::rgb(0.4, 0.4, 0.4),
                        LightColor::Green => LinearRgba::rgb(0.1, 0.4, 0.1),
                        LightColor::Amber => LinearRgba::rgb(0.4, 0.25, 0.05),
                        LightColor::Red => LinearRgba::rgb(0.4, 0.05, 0.05),
                        LightColor::Blue => LinearRgba::rgb(0.05, 0.15, 0.4),
                    }
                } else {
                    LinearRgba::BLACK
                };
            } else {
                material.base_color = base_color;
                material.emissive = match light.color {
                    LightColor::White => LinearRgba::rgb(0.4, 0.4, 0.4),
                    LightColor::Green => LinearRgba::rgb(0.1, 0.4, 0.1),
                    LightColor::Amber => LinearRgba::rgb(0.4, 0.25, 0.05),
                    LightColor::Red => LinearRgba::rgb(0.4, 0.05, 0.05),
                    LightColor::Blue => LinearRgba::rgb(0.05, 0.15, 0.4),
                };
            }
        } else {
            material.base_color = Color::srgb(0.08, 0.08, 0.08);
            material.emissive = LinearRgba::BLACK;
        }
    }
}

pub fn spawn_historical_panels(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    console_y: f32,
    console_z: f32,
    console_width: f32,
    console_height: f32,
    console_depth: f32,
) {
    let switch_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.7, 0.72),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        ..default()
    });

    let breaker_in_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.62),
        metallic: 0.7,
        perceptual_roughness: 0.4,
        ..default()
    });

    let panel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.35, 0.4),
        metallic: 0.3,
        perceptual_roughness: 0.6,
        ..default()
    });

    let light_off_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.08, 0.08, 0.08),
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });

    spawn_left_panel_switches(parent, meshes, &switch_material, &panel_material, &light_off_material, console_y, console_z, console_width, console_height, console_depth);
    spawn_right_panel_switches(parent, meshes, &switch_material, &panel_material, &light_off_material, console_y, console_z, console_width, console_height, console_depth);
    spawn_overhead_breakers(parent, meshes, &breaker_in_material, &panel_material, console_y, console_z, console_width);
    spawn_main_panel_indicators(parent, meshes, &light_off_material, &panel_material, console_y, console_z, console_width, console_height, console_depth);
}

fn spawn_left_panel_switches(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    switch_material: &Handle<StandardMaterial>,
    panel_material: &Handle<StandardMaterial>,
    _light_material: &Handle<StandardMaterial>,
    console_y: f32,
    console_z: f32,
    console_width: f32,
    console_height: f32,
    _console_depth: f32,
) {
    let switch_mesh = meshes.add(Cylinder::new(0.02, 0.05));

    let wing_width = 0.91f32;
    let panel_backing = meshes.add(Cuboid::new(wing_width * 0.8, console_height * 0.7, 0.04));
    let wing_x = -(console_width * 0.5 + wing_width * 0.3);
    let wing_z = console_z + 0.25;
    let wing_y = console_y;

    parent.spawn(PbrBundle {
        mesh: panel_backing,
        material: panel_material.clone(),
        transform: Transform::from_xyz(wing_x, wing_y, wing_z)
            .with_rotation(Quat::from_rotation_y(0.25)),
        ..default()
    });

    let panel_rotation = Quat::from_rotation_y(0.25);
    for (i, def) in config::LEFT_PANEL_SWITCHES.iter().enumerate() {
        let row = i / 2;
        let col = i % 2;
        let local_x = -0.25 + col as f32 * 0.15;
        let local_y = 0.25 - row as f32 * 0.12;
        let local_pos = Vec3::new(local_x, local_y, 0.02);
        let world_pos = panel_rotation * local_pos + Vec3::new(wing_x, wing_y, wing_z);

        parent.spawn((
            PbrBundle {
                mesh: switch_mesh.clone(),
                material: switch_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(panel_rotation * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            PanelSwitch {
                id: def.id.clone(),
                state: def.state,
                label: def.label.to_string(),
                target_system: def.target,
                position: world_pos,
                rotation_axis: panel_rotation * Vec3::Z,
                hit_radius: 0.04,
            },
            InteractivePanel,
        ));
    }
}

fn spawn_right_panel_switches(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    switch_material: &Handle<StandardMaterial>,
    panel_material: &Handle<StandardMaterial>,
    light_material: &Handle<StandardMaterial>,
    console_y: f32,
    console_z: f32,
    console_width: f32,
    _console_height: f32,
    _console_depth: f32,
) {
    let switch_mesh = meshes.add(Cylinder::new(0.02, 0.05));
    let light_mesh = meshes.add(Cylinder::new(0.012, 0.008));

    let wing_width = 0.91f32;
    let panel_backing = meshes.add(Cuboid::new(wing_width * 0.8, 0.6, 0.04));
    let wing_x = console_width * 0.5 + wing_width * 0.3;
    let wing_z = console_z + 0.25;
    let wing_y = console_y;

    parent.spawn(PbrBundle {
        mesh: panel_backing,
        material: panel_material.clone(),
        transform: Transform::from_xyz(wing_x, wing_y, wing_z)
            .with_rotation(Quat::from_rotation_y(-0.25)),
        ..default()
    });

    let panel_rotation = Quat::from_rotation_y(-0.25);
    for (i, def) in config::RIGHT_PANEL_SWITCHES.iter().enumerate() {
        let row = i / 2;
        let col = i % 2;
        let local_x = -0.25 + col as f32 * 0.15;
        let local_y = 0.25 - row as f32 * 0.12;
        let local_pos = Vec3::new(local_x, local_y, 0.02);
        let world_pos = panel_rotation * local_pos + Vec3::new(wing_x, wing_y, wing_z);

        parent.spawn((
            PbrBundle {
                mesh: switch_mesh.clone(),
                material: switch_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(panel_rotation * Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..default()
            },
            PanelSwitch {
                id: def.id.clone(),
                state: def.state,
                label: def.label.to_string(),
                target_system: def.target,
                position: world_pos,
                rotation_axis: panel_rotation * Vec3::Z,
                hit_radius: 0.04,
            },
            InteractivePanel,
        ));

        if matches!(def.id, SwitchId::FuelCell1 | SwitchId::FuelCell2 | SwitchId::FuelCell3) {
            let light_pos = panel_rotation * Vec3::new(local_x + 0.02, local_y + 0.03, 0.02) + Vec3::new(wing_x, wing_y, wing_z);
            parent.spawn((
                PbrBundle {
                    mesh: light_mesh.clone(),
                    material: light_material.clone(),
                    transform: Transform::from_translation(light_pos)
                        .with_rotation(panel_rotation),
                    ..default()
                },
                PanelLight {
                    label: format!("{} STATUS", def.label),
                    lit: true,
                    color: LightColor::Green,
                    blink: false,
                    blink_timer: 0.0,
                    target_system: def.target,
                },
            ));
        }
    }
}

fn spawn_overhead_breakers(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    breaker_material: &Handle<StandardMaterial>,
    panel_material: &Handle<StandardMaterial>,
    console_y: f32,
    console_z: f32,
    console_width: f32,
) {
    let breaker_mesh = meshes.add(Cylinder::new(0.008, 0.025));

    let panel_backing = meshes.add(Cuboid::new(console_width * 0.9, 0.35, 0.04));
    let panel_y = console_y + 0.7;
    let panel_z = console_z + 0.1;

    parent.spawn(PbrBundle {
        mesh: panel_backing,
        material: panel_material.clone(),
        transform: Transform::from_xyz(0.0, panel_y, panel_z)
            .with_rotation(Quat::from_rotation_x(-0.05)),
        ..default()
    });

    let panel_rotation = Quat::from_rotation_x(-0.05);
    for (i, def) in config::OVERHEAD_BREAKERS.iter().enumerate() {
        let row = i / 6;
        let col = i % 6;
        let local_x = -0.5 + col as f32 * 0.18;
        let local_z = 0.02 + row as f32 * 0.1;
        let local_pos = Vec3::new(local_x, 0.0, local_z);
        let world_pos = panel_rotation * local_pos + Vec3::new(0.0, panel_y, panel_z);

        parent.spawn((
            PbrBundle {
                mesh: breaker_mesh.clone(),
                material: breaker_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(panel_rotation * Quat::from_rotation_x(std::f32::consts::PI)),
                ..default()
            },
            CircuitBreaker {
                id: def.id.clone(),
                closed: true,
                label: def.label.to_string(),
                target_system: def.target,
                amp_rating: def.amps,
            },
            InteractivePanel,
        ));
    }
}

fn spawn_main_panel_indicators(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    light_material: &Handle<StandardMaterial>,
    _panel_material: &Handle<StandardMaterial>,
    console_y: f32,
    console_z: f32,
    _console_width: f32,
    _console_height: f32,
    console_depth: f32,
) {
    let light_mesh = meshes.add(Cylinder::new(0.015, 0.006));
    let panel_rotation = Quat::from_rotation_x(-0.12);

    for (i, def) in config::MAIN_INDICATORS.iter().enumerate() {
        let col = i % 4;
        let row = i / 4;
        let local_x = -0.4 + col as f32 * 0.25;
        let local_y = 0.3 - row as f32 * 0.08;
        let local_z = console_depth * 0.5 + 0.02;
        let local_pos = Vec3::new(local_x, local_y, local_z);
        let world_pos = panel_rotation * local_pos + Vec3::new(0.0, console_y, console_z);

        parent.spawn((
            PbrBundle {
                mesh: light_mesh.clone(),
                material: light_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(panel_rotation),
                ..default()
            },
            PanelLight {
                label: def.label.to_string(),
                lit: false,
                color: def.color,
                blink: def.blink,
                blink_timer: 0.0,
                target_system: def.target,
            },
        ));
    }
}



fn handle_dsky_input(
    mut interaction_events: EventReader<PanelInteraction>,
    mut dsky_query: Query<&mut DskyDisplay>,
) {
    for event in interaction_events.read() {
        if let PanelInteraction::KeyPressed(_, _key_type) = event {
            for mut dsky in dsky_query.iter_mut() {
                dsky.comp_acty = true;
            }
        }
    }
}



fn update_event_timer(
    mut timer_query: Query<&mut EventTimer>,
    time: Res<Time>,
) {
    for mut timer in timer_query.iter_mut() {
        if !timer.counting {
            continue;
        }
        
        if time.delta_seconds() >= 1.0 {
            if timer.count_up {
                timer.seconds += 1;
                if timer.seconds >= 60 {
                    timer.seconds = 0;
                    timer.minutes += 1;
                    if timer.minutes >= 60 {
                        timer.minutes = 0;
                    }
                }
            } else {
                if timer.seconds > 0 {
                    timer.seconds -= 1;
                } else if timer.minutes > 0 {
                    timer.seconds = 59;
                    timer.minutes -= 1;
                } else {
                    timer.counting = false;
                }
            }
        }
    }
}
