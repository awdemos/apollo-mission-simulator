pub const CM_HEIGHT: f32 = 3.48;
pub const CM_BASE_DIAMETER: f32 = 3.91;
pub const CM_BASE_RADIUS: f32 = CM_BASE_DIAMETER / 2.0;
pub const CM_TOP_RADIUS: f32 = 0.6;
pub const CM_CONICAL_HALF_ANGLE_DEG: f32 = 33.0;

pub const PV_HEIGHT: f32 = 3.23;
pub const FLOOR_Y: f32 = 0.15;
pub const CEILING_Y: f32 = FLOOR_Y + PV_HEIGHT * 0.85;

pub const COUCH_WIDTH: f32 = 0.584;
pub const COUCH_DEPTH: f32 = 0.65;
pub const COUCH_SPACING: f32 = 0.64;
pub const COUCH_Y: f32 = FLOOR_Y + 0.45;
pub const COUCH_Z: f32 = -0.35;

pub const CONSOLE_WIDTH: f32 = 2.13;
pub const CONSOLE_HEIGHT: f32 = 0.91;
pub const CONSOLE_DEPTH: f32 = 0.61;
pub const CONSOLE_Z: f32 = -0.95;
pub const CONSOLE_Y: f32 = FLOOR_Y + 0.85;
pub const WING_WIDTH: f32 = 0.91;
pub const WING_DEPTH: f32 = 0.61;
pub const WING_ANGLE_DEG: f32 = 30.0;

pub const HATCH_WIDTH: f32 = 0.74;
pub const HATCH_HEIGHT: f32 = 0.86;
pub const HATCH_Y: f32 = FLOOR_Y + 1.15;

pub const TUNNEL_RADIUS: f32 = 0.405;
pub const TUNNEL_LENGTH: f32 = 0.7;

pub const RENDEZVOUS_WINDOW_WIDTH: f32 = 0.20;
pub const RENDEZVOUS_WINDOW_HEIGHT: f32 = 0.23;
pub const SIDE_WINDOW_SIZE: f32 = 0.23;
pub const RENDEZVOUS_WINDOW_Y: f32 = FLOOR_Y + 1.35;
pub const SIDE_WINDOW_Y: f32 = FLOOR_Y + 1.1;
pub const RENDEZVOUS_WINDOW_ANGLE_DEG: f32 = 25.0;

pub const DSKY_WIDTH: f32 = 0.48;
pub const DSKY_HEIGHT: f32 = 0.42;
pub const DSKY_DEPTH: f32 = 0.14;

pub const FDAI_SIZE: f32 = 0.28;
pub const FDAI_SPHERE_RADIUS: f32 = 0.11;

pub const TIMER_WIDTH: f32 = 0.35;
pub const TIMER_HEIGHT: f32 = 0.14;

pub const RHC_RADIUS: f32 = 0.035;
pub const RHC_LENGTH: f32 = 0.12;
pub const THC_RADIUS: f32 = 0.03;
pub const THC_LENGTH: f32 = 0.1;

pub const SWITCH_RADIUS: f32 = 0.02;
pub const SWITCH_HEIGHT: f32 = 0.05;
pub const BREAKER_RADIUS: f32 = 0.015;
pub const BREAKER_HEIGHT: f32 = 0.04;

pub const INTERIOR_LIGHT_INTENSITY: f32 = 200_000.0;
pub const OVERHEAD_LIGHT_INTENSITY: f32 = 150_000.0;
pub const INTERIOR_LIGHT_RANGE: f32 = 10.0;
pub const OVERHEAD_LIGHT_RANGE: f32 = 12.0;

pub const CAMERA_MOVE_SPEED: f32 = 3.0;
pub const CAMERA_LOOK_SPEED: f32 = 0.008;
pub const CAMERA_PITCH_CLAMP_DEG: f32 = 85.0;

// Saturn V visual scale factor
// Real Saturn V is 110.6m tall
// Scale 0.1 makes it ~11 units tall - very visible against Earth (radius 10)
pub const SATURN_V_SCALE: f32 = 0.1;

// Saturn V actual dimensions (meters) - scaled by SATURN_V_SCALE for game
// Historical Saturn V was 110.6m tall total
// Stage proportions: S-IC ~38%, S-II ~22%, S-IVB ~16% of total
// Upper stack (IU+SLA+SM+CM+LES) ~24% of total
pub const S_IC_HEIGHT_M: f32 = 42.1;
pub const S_IC_DIAMETER_M: f32 = 10.1;
pub const S_II_HEIGHT_M: f32 = 24.8;
pub const S_II_DIAMETER_M: f32 = 10.1;
pub const S_IVb_HEIGHT_M: f32 = 17.8;
pub const S_IVb_DIAMETER_M: f32 = 6.6;
pub const IU_HEIGHT_M: f32 = 0.9;
pub const IU_DIAMETER_M: f32 = 6.6;
pub const SLA_HEIGHT_M: f32 = 6.4;
pub const SLA_BASE_DIAMETER_M: f32 = 6.6;   // Bottom (attached to IU)
pub const SLA_TOP_DIAMETER_M: f32 = 3.9;    // Top (attached to SM)
pub const SM_HEIGHT_M: f32 = 6.5;
pub const SM_DIAMETER_M: f32 = 3.9;
pub const CM_HEIGHT_M: f32 = 3.0;
pub const CM_BASE_DIAMETER_M: f32 = 3.9;
pub const CM_TOP_DIAMETER_M: f32 = 1.2;     // Forward hatch diameter
pub const LES_HEIGHT_M: f32 = 9.1;
pub const LES_DIAMETER_M: f32 = 0.66;

// Scaled dimensions
pub const S_IC_HEIGHT: f32 = S_IC_HEIGHT_M * SATURN_V_SCALE;
pub const S_IC_RADIUS: f32 = (S_IC_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const S_II_HEIGHT: f32 = S_II_HEIGHT_M * SATURN_V_SCALE;
pub const S_II_RADIUS: f32 = (S_II_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const S_IVb_HEIGHT: f32 = S_IVb_HEIGHT_M * SATURN_V_SCALE;
pub const S_IVb_RADIUS: f32 = (S_IVb_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const IU_HEIGHT: f32 = IU_HEIGHT_M * SATURN_V_SCALE;
pub const IU_RADIUS: f32 = (IU_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const SLA_HEIGHT: f32 = SLA_HEIGHT_M * SATURN_V_SCALE;
pub const SLA_BASE_RADIUS: f32 = (SLA_BASE_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const SLA_TOP_RADIUS: f32 = (SLA_TOP_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const SM_HEIGHT: f32 = SM_HEIGHT_M * SATURN_V_SCALE;
pub const SM_RADIUS: f32 = (SM_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const CSM_HEIGHT: f32 = CM_HEIGHT_M * SATURN_V_SCALE;
pub const CSM_BASE_RADIUS: f32 = (CM_BASE_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const CSM_TOP_RADIUS: f32 = (CM_TOP_DIAMETER_M * SATURN_V_SCALE) / 2.0;
pub const LES_HEIGHT: f32 = LES_HEIGHT_M * SATURN_V_SCALE;
pub const LES_RADIUS: f32 = (LES_DIAMETER_M * SATURN_V_SCALE) / 2.0;

pub const INTERSTAGE_1_HEIGHT: f32 = 0.3 * SATURN_V_SCALE;
pub const INTERSTAGE_2_HEIGHT: f32 = 0.25 * SATURN_V_SCALE;
pub const BPC_HEIGHT: f32 = 2.8 * SATURN_V_SCALE;

pub const MLP_HEIGHT: f32 = 0.76;

pub const SATURN_V_TOTAL_HEIGHT: f32 = S_IC_HEIGHT
    + INTERSTAGE_1_HEIGHT
    + S_II_HEIGHT
    + INTERSTAGE_2_HEIGHT
    + S_IVb_HEIGHT
    + IU_HEIGHT
    + SLA_HEIGHT
    + SM_HEIGHT
    + CSM_HEIGHT
    + BPC_HEIGHT
    + LES_HEIGHT;

pub const CM_CENTER_OFFSET: f32 = -SATURN_V_TOTAL_HEIGHT * 0.5
    + S_IC_HEIGHT + INTERSTAGE_1_HEIGHT
    + S_II_HEIGHT + INTERSTAGE_2_HEIGHT
    + S_IVb_HEIGHT + IU_HEIGHT + SLA_HEIGHT + SM_HEIGHT
    + CSM_HEIGHT * 0.5;
