use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSet};
use crate::virtual_agc::{VirtualAgc, AgcChannel, DskyKey, key_to_channel_value};
use crate::panels::{PanelInteraction, DskyDisplay, DskyDigit, DskySign, DskyKeyType, DskyStatusLights};
use std::path::Path;
use chrono::Timelike;

pub struct AgcPlugin;

impl Plugin for AgcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<AgcState>()
            .add_systems(Startup, init_virtual_agc)
            .add_systems(Update, read_3d_dsky_keys.run_if(in_state(crate::game_state::AppState::InGame)).before(step_agc))
            .add_systems(Update, step_agc.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, read_dsky_channels.run_if(in_state(crate::game_state::AppState::InGame)).after(step_agc))
            .add_systems(Update, sync_agc_to_3d_dsky.run_if(in_state(crate::game_state::AppState::InGame)).after(read_dsky_channels))
            .add_systems(Update, apply_agc_outputs.run_if(in_state(crate::game_state::AppState::InGame)).after(step_agc))
            .add_systems(Update, manual_rcs_control.run_if(in_state(crate::game_state::AppState::InGame)))
            .add_systems(Update, dsky_ui.run_if(in_state(crate::game_state::AppState::InGame)).after(EguiSet::InitContexts))
            .add_systems(Update, debug_agc_dsky_sync.run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct AgcState {
    pub program: u8,
    pub verb: [u8; 2],
    pub noun: [u8; 2],
    pub r1: DisplayRegister,
    pub r2: DisplayRegister,
    pub r3: DisplayRegister,
    pub annunciators: Annunciators,
    pub is_verb_entry: bool,
    pub is_noun_entry: bool,
    pub current_entry: Vec<u8>,
    pub entry_mode: Option<EntryMode>,
    pub virtual_agc: Option<VirtualAgc>,
    pub key_buffer: Vec<DskyKey>,
    pub manual_mode: bool,
}

impl Default for AgcState {
    fn default() -> Self {
        Self {
            program: 0,
            verb: [0, 0],
            noun: [0, 0],
            r1: DisplayRegister::default(),
            r2: DisplayRegister::default(),
            r3: DisplayRegister::default(),
            annunciators: Annunciators::default(),
            is_verb_entry: false,
            is_noun_entry: false,
            current_entry: Vec::new(),
            entry_mode: None,
            virtual_agc: None,
            key_buffer: Vec::new(),
            manual_mode: false,
        }
    }
}

pub struct DisplayRegister {
    pub sign: char,
    pub digits: [u8; 5],
}

impl Default for DisplayRegister {
    fn default() -> Self {
        Self {
            sign: '+',
            digits: [0; 5],
        }
    }
}

pub struct Annunciators {
    pub uplink_acty: bool,
    pub no_att: bool,
    pub stby: bool,
    pub restart: bool,
    pub key_rel: bool,
    pub opr_err: bool,
    pub temp: bool,
    pub gimbal_lock: bool,
    pub tracker: bool,
    pub prog: bool,
}

impl Default for Annunciators {
    fn default() -> Self {
        Self {
            uplink_acty: false,
            no_att: false,
            stby: false,
            restart: false,
            key_rel: false,
            opr_err: false,
            temp: false,
            gimbal_lock: false,
            tracker: false,
            prog: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryMode {
    Verb,
    Noun,
    Data,
}

fn init_virtual_agc(mut state: ResMut<AgcState>) {
    let mut agc = VirtualAgc::new();
    
    let bin_path = Path::new("assets/agc/Comanche055.bin");
    if bin_path.exists() {
        match agc.init(bin_path, None) {
            Ok(()) => {
                info!("Initialized Comanche055 (Apollo 11 CM) AGC");
            }
            Err(e) => {
                warn!("Failed to initialize AGC engine: {}. Trying load_binfile fallback.", e);
                match agc.load_binfile(bin_path) {
                    Ok(()) => {
                        info!("Loaded Comanche055 via load_binfile fallback");
                    }
                    Err(e2) => {
                        warn!("Failed to load AGC binary: {}. Using fallback mode.", e2);
                    }
                }
            }
        }
    } else {
        warn!("AGC binary not found at {:?}. Using fallback mode.", bin_path);
    }
    
    state.virtual_agc = Some(agc);
}

fn step_agc(mut state: ResMut<AgcState>) {
    let keys: Vec<DskyKey> = state.key_buffer.drain(..).collect();
    
    if let Some(ref mut agc) = state.virtual_agc {
        agc.step();
        
        for key in keys {
            let value = key_to_channel_value(key);
            agc.write_io(AgcChannel::DskyKeyboard as i32, value);
        }
        
        agc.channel_routine();
    }
}

fn read_dsky_channels(mut state: ResMut<AgcState>) {
    if let Some(ref mut agc) = state.virtual_agc {
        let (prog, verb, noun, r1, r2, r3, lights) = {
            let dsky = agc.get_dsky_state();
            (
                dsky.prog,
                dsky.verb,
                dsky.noun,
                DisplayRegister { sign: dsky.r1_sign, digits: dsky.r1 },
                DisplayRegister { sign: dsky.r2_sign, digits: dsky.r2 },
                DisplayRegister { sign: dsky.r3_sign, digits: dsky.r3 },
                dsky.lights,
            )
        };

        state.program = prog[0] * 10 + prog[1];
        state.verb = verb;
        state.noun = noun;
        state.r1 = r1;
        state.r2 = r2;
        state.r3 = r3;

        state.annunciators.uplink_acty = (lights & 0o00001) != 0;
        state.annunciators.no_att = (lights & 0o00002) != 0;
        state.annunciators.stby = (lights & 0o00004) != 0;
        state.annunciators.key_rel = (lights & 0o00010) != 0;
        state.annunciators.opr_err = (lights & 0o00020) != 0;
        state.annunciators.restart = (lights & 0o00040) != 0;
        state.annunciators.tracker = (lights & 0o00100) != 0;
        state.annunciators.prog = (lights & 0o00200) != 0;
        state.annunciators.temp = (lights & 0o00400) != 0;
        state.annunciators.gimbal_lock = (lights & 0o01000) != 0;
    }
}

fn apply_agc_outputs(
    mut state: ResMut<AgcState>,
    mut csm_query: Query<&mut crate::systems::csm::CommandServiceModule>,
) {
    if let Some(ref mut agc) = state.virtual_agc {
        let ch5 = agc.read_io(5);
        let ch6 = agc.read_io(6);
        let ch14 = agc.read_io(14);

        if let Ok(mut csm) = csm_query.get_single_mut() {
            for quad_idx in 0..4 {
                if quad_idx < csm.rcs.quads.len() {
                    let base_bit = quad_idx * 4;
                    for thruster_idx in 0..4 {
                        let bit = base_bit + thruster_idx;
                        let is_firing = if bit < 14 {
                            (ch5 & (1 << bit)) != 0
                        } else {
                            (ch6 & (1 << (bit - 14))) != 0
                        };

                        if is_firing && csm.rcs.quads[quad_idx].enabled {
                            csm.rcs.quads[quad_idx].thrusters[thruster_idx].fired_count += 1;
                        }
                    }
                }
            }

            let sps_on = (ch14 & 0o00001) != 0;
            if sps_on && csm.sps.engine.status == crate::systems::SystemStatus::Nominal {
                csm.sps.thrust_newtons = csm.sps.engine.thrust_vac_newtons;
            } else {
                csm.sps.thrust_newtons = 0.0;
            }
        }
    }
}

fn manual_rcs_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<AgcState>,
    mut csm_query: Query<&mut crate::systems::csm::CommandServiceModule>,
    mut staging_events: EventWriter<crate::lvdc::ManualStagingEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyM) {
        state.manual_mode = !state.manual_mode;
        info!("Manual RCS mode: {}", if state.manual_mode { "ON" } else { "OFF" });
    }
    
    if keyboard.just_pressed(KeyCode::F1) {
        staging_events.send(crate::lvdc::ManualStagingEvent { stage: 1, action: crate::lvdc::StagingAction::CutoffAndSeparate });
        info!("STAGING: S-IC cutoff commanded");
    }
    if keyboard.just_pressed(KeyCode::F2) {
        staging_events.send(crate::lvdc::ManualStagingEvent { stage: 2, action: crate::lvdc::StagingAction::Ignite });
        info!("STAGING: S-II ignition commanded");
    }
    if keyboard.just_pressed(KeyCode::F3) {
        staging_events.send(crate::lvdc::ManualStagingEvent { stage: 2, action: crate::lvdc::StagingAction::CutoffAndSeparate });
        info!("STAGING: S-II cutoff commanded");
    }
    if keyboard.just_pressed(KeyCode::F4) {
        staging_events.send(crate::lvdc::ManualStagingEvent { stage: 3, action: crate::lvdc::StagingAction::Ignite });
        info!("STAGING: S-IVB ignition commanded");
    }
    if keyboard.just_pressed(KeyCode::F5) {
        staging_events.send(crate::lvdc::ManualStagingEvent { stage: 3, action: crate::lvdc::StagingAction::Cutoff });
        info!("STAGING: S-IVB cutoff commanded");
    }
    
    if !state.manual_mode {
        return;
    }
    
    if let Ok(mut csm) = csm_query.get_single_mut() {
        for quad_idx in 0..csm.rcs.quads.len() {
            if !csm.rcs.quads[quad_idx].enabled {
                continue;
            }
            for thruster_idx in 0..4 {
                csm.rcs.quads[quad_idx].thrusters[thruster_idx].status = crate::systems::SystemStatus::Nominal;
            }
        }
        
        if keyboard.pressed(KeyCode::KeyI) {
            if csm.rcs.quads.len() > 0 { csm.rcs.quads[0].thrusters[0].status = crate::systems::SystemStatus::Firing; }
            if csm.rcs.quads.len() > 1 { csm.rcs.quads[1].thrusters[0].status = crate::systems::SystemStatus::Firing; }
        }
        if keyboard.pressed(KeyCode::KeyK) {
            if csm.rcs.quads.len() > 0 { csm.rcs.quads[0].thrusters[1].status = crate::systems::SystemStatus::Firing; }
            if csm.rcs.quads.len() > 1 { csm.rcs.quads[1].thrusters[1].status = crate::systems::SystemStatus::Firing; }
        }
        if keyboard.pressed(KeyCode::KeyJ) {
            if csm.rcs.quads.len() > 2 { csm.rcs.quads[2].thrusters[2].status = crate::systems::SystemStatus::Firing; }
            if csm.rcs.quads.len() > 3 { csm.rcs.quads[3].thrusters[2].status = crate::systems::SystemStatus::Firing; }
        }
        if keyboard.pressed(KeyCode::KeyL) {
            if csm.rcs.quads.len() > 2 { csm.rcs.quads[2].thrusters[3].status = crate::systems::SystemStatus::Firing; }
            if csm.rcs.quads.len() > 3 { csm.rcs.quads[3].thrusters[3].status = crate::systems::SystemStatus::Firing; }
        }
        
        if keyboard.pressed(KeyCode::Space) {
            if csm.sps.engine.status == crate::systems::SystemStatus::Nominal {
                csm.sps.thrust_newtons = csm.sps.engine.thrust_vac_newtons * 0.1;
            }
        }
    }
}

fn read_3d_dsky_keys(
    mut interaction_events: EventReader<PanelInteraction>,
    mut state: ResMut<AgcState>,
) {
    for event in interaction_events.read() {
        if let PanelInteraction::KeyPressed(_, key_type) = event {
            if let Some(key) = dsky_key_type_to_virtual(*key_type) {
                state.key_buffer.push(key);
                info!("DSKY key forwarded to AGC: {:?}", key_type);
            }
        }
    }
}

fn dsky_key_type_to_virtual(key_type: DskyKeyType) -> Option<DskyKey> {
    match key_type {
        DskyKeyType::Verb => Some(DskyKey::Verb),
        DskyKeyType::Noun => Some(DskyKey::Noun),
        DskyKeyType::Plus => Some(DskyKey::Plus),
        DskyKeyType::Minus => Some(DskyKey::Minus),
        DskyKeyType::Number(0) => Some(DskyKey::Zero),
        DskyKeyType::Number(1) => Some(DskyKey::One),
        DskyKeyType::Number(2) => Some(DskyKey::Two),
        DskyKeyType::Number(3) => Some(DskyKey::Three),
        DskyKeyType::Number(4) => Some(DskyKey::Four),
        DskyKeyType::Number(5) => Some(DskyKey::Five),
        DskyKeyType::Number(6) => Some(DskyKey::Six),
        DskyKeyType::Number(7) => Some(DskyKey::Seven),
        DskyKeyType::Number(8) => Some(DskyKey::Eight),
        DskyKeyType::Number(9) => Some(DskyKey::Nine),
        DskyKeyType::Clear => Some(DskyKey::Clear),
        DskyKeyType::Enter => Some(DskyKey::Enter),
        DskyKeyType::KeyRel => Some(DskyKey::Proceed),
        DskyKeyType::Reset => Some(DskyKey::Reset),
        DskyKeyType::Pro => Some(DskyKey::Proceed),
        _ => None,
    }
}

fn sync_agc_to_3d_dsky(
    state: Res<AgcState>,
    mut dsky_query: Query<&mut DskyDisplay>,
) {
    for mut dsky in dsky_query.iter_mut() {
        dsky.prog = state.program;
        dsky.verb = state.verb[0] * 10 + state.verb[1];
        dsky.noun = state.noun[0] * 10 + state.noun[1];
        
        let (r1_digits, r1_sign) = display_register_to_dsky(&state.r1);
        dsky.r1 = r1_digits;
        dsky.r1_sign = r1_sign;
        
        let (r2_digits, r2_sign) = display_register_to_dsky(&state.r2);
        dsky.r2 = r2_digits;
        dsky.r2_sign = r2_sign;
        
        let (r3_digits, r3_sign) = display_register_to_dsky(&state.r3);
        dsky.r3 = r3_digits;
        dsky.r3_sign = r3_sign;
        
        dsky.lights.uplink_acty = state.annunciators.uplink_acty;
        dsky.lights.no_att = state.annunciators.no_att;
        dsky.lights.stby = state.annunciators.stby;
        dsky.lights.restart = state.annunciators.restart;
        dsky.lights.key_rel = state.annunciators.key_rel;
        dsky.lights.opr_err = state.annunciators.opr_err;
        dsky.lights.temp = state.annunciators.temp;
        dsky.lights.gimbal_lock = state.annunciators.gimbal_lock;
        dsky.lights.tracker = state.annunciators.tracker;
        dsky.lights.prog = state.annunciators.prog;
    }
}

fn display_register_to_dsky(reg: &DisplayRegister) -> ([DskyDigit; 5], DskySign) {
    let digits = reg.digits.map(|d| match d {
        0 => DskyDigit::Zero,
        1 => DskyDigit::One,
        2 => DskyDigit::Two,
        3 => DskyDigit::Three,
        4 => DskyDigit::Four,
        5 => DskyDigit::Five,
        6 => DskyDigit::Six,
        7 => DskyDigit::Seven,
        8 => DskyDigit::Eight,
        9 => DskyDigit::Nine,
        _ => DskyDigit::Blank,
    });
    let sign = match reg.sign {
        '+' => DskySign::Plus,
        '-' => DskySign::Minus,
        _ => DskySign::Blank,
    };
    (digits, sign)
}

fn decode_register(value: i32) -> DisplayRegister {
    let sign = if (value & 0o40000) != 0 { '-' } else { '+' };
    let mut digits = [0u8; 5];
    let mut v = value & 0o37777;
    
    for i in (0..5).rev() {
        digits[i] = (v % 10) as u8;
        v /= 10;
    }
    
    DisplayRegister { sign, digits }
}

fn debug_agc_dsky_sync(
    state: Res<AgcState>,
    dsky_query: Query<&DskyDisplay>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_seconds();
    if *timer >= 2.0 {
        *timer = 0.0;
        let agc_prog = state.program;
        let agc_verb = state.verb[0] * 10 + state.verb[1];
        let agc_noun = state.noun[0] * 10 + state.noun[1];
        info!("AGC display: PROG={:02o} VERB={:02o} NOUN={:02o}", agc_prog, agc_verb, agc_noun);
        
        for dsky in dsky_query.iter() {
            info!("3D DSKY sync: PROG={:02o} VERB={:02o} NOUN={:02o} COMP_ACTY={}", 
                dsky.prog, dsky.verb, dsky.noun, dsky.comp_acty);
        }
    }
}

fn dsky_ui(
    mut contexts: EguiContexts,
    mut state: ResMut<AgcState>,
    camera_mode: Res<crate::CameraMode>,
    mut panel_events: EventWriter<crate::panels::PanelInteraction>,
) {
    if *camera_mode != crate::CameraMode::Interior {
        return;
    }
    let ctx = contexts.ctx_mut();
    let mut frame = egui::Frame::window(&ctx.style());
    frame.fill = egui::Color32::from_rgb(28, 30, 26);
    frame.stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(55, 58, 50));
    frame.rounding = egui::Rounding::same(4.0);
    frame.inner_margin = egui::Margin::same(12.0);

    egui::Window::new("DSKY")
        .default_pos([10.0, 40.0])
        .default_size([360.0, 540.0])
        .frame(frame)
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            let bezel_color = egui::Color32::from_rgb(35, 38, 32);
            let display_bg = egui::Color32::from_rgb(10, 16, 12);
            let glow_on = egui::Color32::from_rgb(80, 255, 160);
            let glow_dim = egui::Color32::from_rgb(18, 35, 24);
            let label_color = egui::Color32::from_rgb(160, 165, 150);
            let font_mono = egui::FontId::monospace(22.0);
            let font_small = egui::FontId::monospace(14.0);
            let font_tiny = egui::FontId::monospace(11.0);

            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("APOLLO GUIDANCE COMPUTER")
                        .font(font_tiny.clone())
                        .color(label_color),
                );
            });

            ui.add_space(8.0);

            egui::Frame::none()
                .fill(display_bg)
                .rounding(egui::Rounding::same(3.0))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        dsky_field(ui, "PROG", &format!("{:02}", state.program), &font_mono, glow_on, glow_dim, label_color, &font_tiny);
                        dsky_field(ui, "VERB", &format!("{}{}", state.verb[0], state.verb[1]), &font_mono, glow_on, glow_dim, label_color, &font_tiny);
                        dsky_field(ui, "NOUN", &format!("{}{}", state.noun[0], state.noun[1]), &font_mono, glow_on, glow_dim, label_color, &font_tiny);
                    });

                    ui.add_space(10.0);

                    display_register_styled(ui, "R1", &state.r1, &font_mono, glow_on, glow_dim, label_color, &font_small);
                    display_register_styled(ui, "R2", &state.r2, &font_mono, glow_on, glow_dim, label_color, &font_small);
                    display_register_styled(ui, "R3", &state.r3, &font_mono, glow_on, glow_dim, label_color, &font_small);
                });

            ui.add_space(8.0);

            egui::Frame::none()
                .fill(bezel_color)
                .rounding(egui::Rounding::same(3.0))
                .inner_margin(egui::Margin::symmetric(6.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        annunciator_light_styled(ui, "UPTM", state.annunciators.uplink_acty, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "NO ATT", state.annunciators.no_att, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "STBY", state.annunciators.stby, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "RESTART", state.annunciators.restart, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "KEY REL", state.annunciators.key_rel, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "OPR ERR", state.annunciators.opr_err, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "TEMP", state.annunciators.temp, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "GIMBAL", state.annunciators.gimbal_lock, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "TRACKER", state.annunciators.tracker, glow_on, glow_dim, &font_tiny);
                        annunciator_light_styled(ui, "PROG", state.annunciators.prog, glow_on, glow_dim, &font_tiny);
                    });
                });

            ui.add_space(8.0);

            dsky_keyboard_styled(ui, &mut state, &mut panel_events);
        });
}

fn dsky_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &str,
    font: &egui::FontId,
    glow_on: egui::Color32,
    glow_dim: egui::Color32,
    label_color: egui::Color32,
    label_font: &egui::FontId,
) {
    ui.vertical(|ui| {
        ui.label(egui::RichText::new(label).font(label_font.clone()).color(label_color));
        ui.label(egui::RichText::new(value).font(font.clone()).color(glow_on).background_color(glow_dim));
    });
    ui.add_space(12.0);
}

fn display_register_styled(
    ui: &mut egui::Ui,
    label: &str,
    reg: &DisplayRegister,
    font: &egui::FontId,
    glow_on: egui::Color32,
    glow_dim: egui::Color32,
    label_color: egui::Color32,
    label_font: &egui::FontId,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(format!("{}:", label)).font(label_font.clone()).color(label_color));
        ui.label(egui::RichText::new(format!("{}", reg.sign)).font(font.clone()).color(glow_on));
        for d in &reg.digits {
            let txt = if *d == 0 { "0".to_string() } else { format!("{}", d) };
            ui.label(egui::RichText::new(txt).font(font.clone()).color(glow_on).background_color(glow_dim));
        }
    });
}

fn annunciator_light_styled(
    ui: &mut egui::Ui,
    label: &str,
    lit: bool,
    glow_on: egui::Color32,
    glow_dim: egui::Color32,
    font: &egui::FontId,
) {
    let (fg, bg) = if lit {
        (glow_on, egui::Color32::from_rgb(20, 50, 30))
    } else {
        (egui::Color32::from_rgb(45, 50, 42), glow_dim)
    };
    ui.add(egui::Label::new(
        egui::RichText::new(label).font(font.clone()).color(fg).background_color(bg)
    ).selectable(false));
    ui.add_space(4.0);
}

fn dsky_keyboard_styled(
    ui: &mut egui::Ui,
    state: &mut AgcState,
    panel_events: &mut EventWriter<crate::panels::PanelInteraction>,
) {
    let button_size = egui::vec2(62.0, 36.0);
    let key_font = egui::FontId::monospace(13.0);

    let mut emit_click = |key_type: crate::panels::DskyKeyType| {
        panel_events.send(crate::panels::PanelInteraction::KeyPressed(
            Entity::PLACEHOLDER,
            key_type,
        ));
    };

    let key_btn = |ui: &mut egui::Ui, label: &str, size: egui::Vec2| -> egui::Response {
        let btn = egui::Button::new(egui::RichText::new(label).font(key_font.clone()).color(egui::Color32::from_rgb(200, 200, 190)))
            .fill(egui::Color32::from_rgb(45, 48, 40))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(65, 68, 58)))
            .rounding(egui::Rounding::same(2.0));
        ui.add_sized(size, btn)
    };

    ui.horizontal(|ui| {
        if key_btn(ui, "VERB", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Verb);
            state.key_buffer.push(DskyKey::Verb);
            state.is_verb_entry = true;
            state.is_noun_entry = false;
            state.current_entry.clear();
            state.entry_mode = Some(EntryMode::Verb);
        }
        if key_btn(ui, "NOUN", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Noun);
            state.key_buffer.push(DskyKey::Noun);
            state.is_verb_entry = false;
            state.is_noun_entry = true;
            state.current_entry.clear();
            state.entry_mode = Some(EntryMode::Noun);
        }
        if key_btn(ui, "+", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Plus);
            state.key_buffer.push(DskyKey::Plus);
            handle_key(state, b'+');
        }
        if key_btn(ui, "-", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Minus);
            state.key_buffer.push(DskyKey::Minus);
            handle_key(state, b'-');
        }
    });

    for row in 0..3 {
        ui.horizontal(|ui| {
            for col in 0..3 {
                let num = row * 3 + col + 1;
                if key_btn(ui, &format!("{}", num), button_size).clicked() {
                    emit_click(crate::panels::DskyKeyType::Number(num as u8));
                    let key = match num {
                        1 => DskyKey::One,
                        2 => DskyKey::Two,
                        3 => DskyKey::Three,
                        4 => DskyKey::Four,
                        5 => DskyKey::Five,
                        6 => DskyKey::Six,
                        7 => DskyKey::Seven,
                        8 => DskyKey::Eight,
                        9 => DskyKey::Nine,
                        _ => DskyKey::Zero,
                    };
                    state.key_buffer.push(key);
                    handle_key(state, b'0' + num as u8);
                }
            }
        });
    }

    ui.horizontal(|ui| {
        if key_btn(ui, "0", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Number(0));
            state.key_buffer.push(DskyKey::Zero);
            handle_key(state, b'0');
        }
        if key_btn(ui, "CLR", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Clear);
            state.key_buffer.push(DskyKey::Clear);
            state.current_entry.clear();
            if let Some(EntryMode::Verb) = state.entry_mode {
                state.verb = [0, 0];
            } else if let Some(EntryMode::Noun) = state.entry_mode {
                state.noun = [0, 0];
            }
        }
        if key_btn(ui, "ENTR", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Enter);
            state.key_buffer.push(DskyKey::Enter);
            process_entry(state);
        }
    });

    ui.horizontal(|ui| {
        if key_btn(ui, "RSET", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Reset);
            state.key_buffer.push(DskyKey::Reset);
            reset_dsky(state);
        }
        if key_btn(ui, "PRO", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::Pro);
            state.key_buffer.push(DskyKey::Proceed);
            state.annunciators.key_rel = false;
        }
        if key_btn(ui, "KEY REL", button_size).clicked() {
            emit_click(crate::panels::DskyKeyType::KeyRel);
            state.annunciators.key_rel = false;
            state.current_entry.clear();
            state.is_verb_entry = false;
            state.is_noun_entry = false;
            state.entry_mode = None;
        }
    });
}

fn handle_key(state: &mut AgcState, key: u8) {
    if state.current_entry.len() < 2 {
        state.current_entry.push(key);

        if let Some(EntryMode::Verb) = state.entry_mode {
            state.verb = [0, 0];
            for (i, &val) in state.current_entry.iter().enumerate() {
                if i < 2 && val >= b'0' && val <= b'9' {
                    state.verb[i] = val - b'0';
                }
            }
        } else if let Some(EntryMode::Noun) = state.entry_mode {
            state.noun = [0, 0];
            for (i, &val) in state.current_entry.iter().enumerate() {
                if i < 2 && val >= b'0' && val <= b'9' {
                    state.noun[i] = val - b'0';
                }
            }
        }
    }
}

fn process_entry(state: &mut AgcState) {
    if state.current_entry.len() == 2 {
        let verb = state.verb[0] * 10 + state.verb[1];
        let noun = state.noun[0] * 10 + state.noun[1];

        match (verb, noun) {
            (35, _) => test_lights(state),
            (36, _) => fresh_start(state),
            (37, _) => {
                state.is_verb_entry = false;
                state.is_noun_entry = true;
                state.entry_mode = Some(EntryMode::Noun);
                state.current_entry.clear();
            }
            (16, 36) => display_time(state),
            _ => {}
        }
    }
}

fn test_lights(state: &mut AgcState) {
    state.annunciators = Annunciators {
        uplink_acty: true,
        no_att: true,
        stby: true,
        restart: true,
        key_rel: true,
        opr_err: true,
        temp: true,
        gimbal_lock: true,
        tracker: true,
        prog: true,
    };
    state.program = 88;
    state.verb = [8, 8];
    state.noun = [8, 8];
    state.r1 = DisplayRegister { sign: '+', digits: [8, 8, 8, 8, 8] };
    state.r2 = DisplayRegister { sign: '+', digits: [8, 8, 8, 8, 8] };
    state.r3 = DisplayRegister { sign: '+', digits: [8, 8, 8, 8, 8] };
}

fn fresh_start(state: &mut AgcState) {
    reset_dsky(state);
    state.program = 1;
}

fn reset_dsky(state: &mut AgcState) {
    state.program = 0;
    state.verb = [0, 0];
    state.noun = [0, 0];
    state.r1 = DisplayRegister::default();
    state.r2 = DisplayRegister::default();
    state.r3 = DisplayRegister::default();
    state.annunciators = Annunciators::default();
    state.current_entry.clear();
    state.is_verb_entry = false;
    state.is_noun_entry = false;
    state.entry_mode = None;
}

fn display_time(state: &mut AgcState) {
    let now = chrono::Utc::now();
    let hours = now.time().hour() as u8;
    let minutes = now.time().minute() as u8;
    let seconds = now.time().second() as u8;

    state.r1.sign = '+';
    state.r1.digits = [
        hours / 10,
        hours % 10,
        minutes / 10,
        minutes % 10,
        0,
    ];
    state.r2.sign = '+';
    state.r2.digits = [
        seconds / 10,
        seconds % 10,
        0,
        0,
        0,
    ];
}
