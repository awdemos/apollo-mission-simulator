use crate::panels::{SwitchId, SwitchState, SystemTarget, BreakerId, LightColor};

pub struct SwitchDef {
    pub id: SwitchId,
    pub label: &'static str,
    pub state: SwitchState,
    pub target: SystemTarget,
}

pub struct BreakerDef {
    pub id: BreakerId,
    pub label: &'static str,
    pub amps: f32,
    pub target: SystemTarget,
}

pub struct IndicatorDef {
    pub label: &'static str,
    pub color: LightColor,
    pub target: SystemTarget,
    pub blink: bool,
}

pub const LEFT_PANEL_SWITCHES: &[SwitchDef] = &[
    SwitchDef { id: SwitchId::RcsQuadA, label: "RCS QUAD A", state: SwitchState::On, target: SystemTarget::Rcs },
    SwitchDef { id: SwitchId::RcsQuadB, label: "RCS QUAD B", state: SwitchState::On, target: SystemTarget::Rcs },
    SwitchDef { id: SwitchId::RcsQuadC, label: "RCS QUAD C", state: SwitchState::On, target: SystemTarget::Rcs },
    SwitchDef { id: SwitchId::RcsQuadD, label: "RCS QUAD D", state: SwitchState::On, target: SystemTarget::Rcs },
    SwitchDef { id: SwitchId::ScsMode, label: "SCS MODE", state: SwitchState::Auto, target: SystemTarget::Guidance },
    SwitchDef { id: SwitchId::TvCEnable, label: "TVC ENABLE", state: SwitchState::Off, target: SystemTarget::Propulsion },
    SwitchDef { id: SwitchId::EngineArm, label: "ENGINE ARM", state: SwitchState::Off, target: SystemTarget::Propulsion },
    SwitchDef { id: SwitchId::SpsEnable, label: "SPS ENABLE", state: SwitchState::Off, target: SystemTarget::Sps },
    SwitchDef { id: SwitchId::ImuCage, label: "IMU CAGE", state: SwitchState::Off, target: SystemTarget::Guidance },
    SwitchDef { id: SwitchId::ImuAlign, label: "IMU ALIGN", state: SwitchState::Off, target: SystemTarget::Guidance },
    SwitchDef { id: SwitchId::GncMode, label: "GNC MODE", state: SwitchState::Auto, target: SystemTarget::Guidance },
    SwitchDef { id: SwitchId::RhcPower, label: "RHC PWR", state: SwitchState::On, target: SystemTarget::Rcs },
];

pub const RIGHT_PANEL_SWITCHES: &[SwitchDef] = &[
    SwitchDef { id: SwitchId::FuelCell1, label: "FUEL CELL 1", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::FuelCell2, label: "FUEL CELL 2", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::FuelCell3, label: "FUEL CELL 3", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::MainBusA, label: "MAIN BUS A", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::MainBusB, label: "MAIN BUS B", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::ScePower, label: "SCE PWR", state: SwitchState::On, target: SystemTarget::Electrical },
    SwitchDef { id: SwitchId::SBandPower, label: "S-BAND", state: SwitchState::On, target: SystemTarget::Communications },
    SwitchDef { id: SwitchId::VhfPower, label: "VHF", state: SwitchState::Off, target: SystemTarget::Communications },
    SwitchDef { id: SwitchId::O2Fan1, label: "O2 FAN 1", state: SwitchState::Auto, target: SystemTarget::LifeSupport },
    SwitchDef { id: SwitchId::O2Fan2, label: "O2 FAN 2", state: SwitchState::Auto, target: SystemTarget::LifeSupport },
    SwitchDef { id: SwitchId::CryoPumps, label: "CRYO PUMPS", state: SwitchState::Auto, target: SystemTarget::LifeSupport },
    SwitchDef { id: SwitchId::CabinFan, label: "CABIN FAN", state: SwitchState::On, target: SystemTarget::LifeSupport },
];

pub const OVERHEAD_BREAKERS: &[BreakerDef] = &[
    BreakerDef { id: BreakerId::FuelCell1MainBusA, label: "FC1 MAIN A", amps: 20.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::FuelCell2MainBusA, label: "FC2 MAIN A", amps: 20.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::FuelCell3MainBusB, label: "FC3 MAIN B", amps: 20.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::BatteryRelayBus, label: "BAT RLY BUS", amps: 30.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::Inverter1, label: "INV 1", amps: 30.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::Inverter2, label: "INV 2", amps: 30.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::RcsQuadAProp, label: "RCS A PROP", amps: 5.0, target: SystemTarget::Rcs },
    BreakerDef { id: BreakerId::RcsQuadBProp, label: "RCS B PROP", amps: 5.0, target: SystemTarget::Rcs },
    BreakerDef { id: BreakerId::RcsQuadCProp, label: "RCS C PROP", amps: 5.0, target: SystemTarget::Rcs },
    BreakerDef { id: BreakerId::RcsQuadDProp, label: "RCS D PROP", amps: 5.0, target: SystemTarget::Rcs },
    BreakerDef { id: BreakerId::SpsPropellant, label: "SPS PROP", amps: 5.0, target: SystemTarget::Sps },
    BreakerDef { id: BreakerId::SpsHelium, label: "SPS HE", amps: 5.0, target: SystemTarget::Sps },
    BreakerDef { id: BreakerId::O2Tank1, label: "O2 TK 1", amps: 5.0, target: SystemTarget::LifeSupport },
    BreakerDef { id: BreakerId::O2Tank2, label: "O2 TK 2", amps: 5.0, target: SystemTarget::LifeSupport },
    BreakerDef { id: BreakerId::CabinFan1, label: "CAB FAN 1", amps: 5.0, target: SystemTarget::LifeSupport },
    BreakerDef { id: BreakerId::SceA, label: "SCE A", amps: 5.0, target: SystemTarget::Electrical },
    BreakerDef { id: BreakerId::CmcPower, label: "CMC", amps: 5.0, target: SystemTarget::Guidance },
    BreakerDef { id: BreakerId::SBandTransmitter, label: "S-BND XMTR", amps: 5.0, target: SystemTarget::Communications },
];

pub const MAIN_INDICATORS: &[IndicatorDef] = &[
    IndicatorDef { label: "NO ATT", color: LightColor::Amber, target: SystemTarget::Guidance, blink: false },
    IndicatorDef { label: "ISS", color: LightColor::Green, target: SystemTarget::Guidance, blink: false },
    IndicatorDef { label: "IMU", color: LightColor::Green, target: SystemTarget::Guidance, blink: false },
    IndicatorDef { label: "SPS", color: LightColor::Amber, target: SystemTarget::Sps, blink: false },
    IndicatorDef { label: "RCS", color: LightColor::Green, target: SystemTarget::Rcs, blink: false },
    IndicatorDef { label: "MASTER ALARM", color: LightColor::Red, target: SystemTarget::None, blink: true },
    IndicatorDef { label: "CAUTION", color: LightColor::Amber, target: SystemTarget::None, blink: true },
    IndicatorDef { label: "WARNING", color: LightColor::Red, target: SystemTarget::None, blink: false },
];
