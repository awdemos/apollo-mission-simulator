use bevy::prelude::*;
use std::collections::VecDeque;

pub struct CommunicationsPlugin;

impl Plugin for CommunicationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommunicationsBus>()
            .init_resource::<GroundControlState>()
            .add_systems(Startup, setup_communications)
            .add_systems(Update, (
                update_telemetry_downlink,
                process_uplink_commands,
                update_signal_strength,
                simulate_communications_hardware,
                update_pll_tracking,
                update_usb_multiplexer,
            ).run_if(in_state(crate::game_state::AppState::InGame)));
    }
}

// =============================================================================
// DATA RATES
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRate {
    /// Emergency rate - historically a Morse tone on 768 kHz subcarrier, not digital data
    Emergency160bps,
    /// Low rate telemetry - 400 bps for AGC words, 1600 bps general (CuriousMarc Part 38)
    LowRate1_6kbps,
    /// High rate PCM telemetry - 51.2 kbps (CuriousMarc Part 35)
    HighRate51_2kbps,
    /// Voice-only mode - no telemetry data
    VoiceOnly,
}

impl DataRate {
    pub fn bits_per_second(&self) -> f64 {
        match self {
            DataRate::Emergency160bps => 160.0,
            DataRate::LowRate1_6kbps => 1_600.0,
            DataRate::HighRate51_2kbps => 51_200.0,
            DataRate::VoiceOnly => 0.0,
        }
    }

    pub fn words_per_second(&self) -> f64 {
        match self {
            DataRate::Emergency160bps => 16.0,
            DataRate::LowRate1_6kbps => 100.0,
            DataRate::HighRate51_2kbps => 3_200.0,
            DataRate::VoiceOnly => 0.0,
        }
    }
}

// =============================================================================
// MODULATION TYPES
// =============================================================================

/// Apollo Unified S-Band used PM (Phase Modulation) for the carrier,
/// with FM on voice subcarrier and PSK on data subcarrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModulationType {
    /// Phase Modulation - used for the main 2.2875 GHz carrier
    PhaseModulation,
    /// Frequency Modulation - used on 1.25 MHz voice subcarrier
    FrequencyModulation,
    /// Phase Shift Keying - used on 1.024 MHz data subcarrier
    PhaseShiftKeying,
    /// On-Off Keying - used for emergency Morse tone on 768 kHz
    OnOffKeying,
}

// =============================================================================
// TELEMETRY WORD & FRAME
// =============================================================================

/// Historical Apollo PCM telemetry word: 8-bit data + 2-bit parity
#[derive(Debug, Clone, Copy)]
pub struct TelemetryWord {
    pub data: u8,
    pub parity: u8,
    pub address: u8,
}

impl TelemetryWord {
    pub fn new(data: u8, address: u8) -> Self {
        let parity = Self::calculate_parity(data);
        Self { data, parity, address }
    }

    fn calculate_parity(data: u8) -> u8 {
        let ones_count = (0..8).filter(|i| (data >> i) & 1 == 1).count();
        if ones_count % 2 == 0 { 0b01 } else { 0b10 }
    }

    pub fn to_bits(&self) -> u16 {
        ((self.address as u16) << 10) | ((self.data as u16) << 2) | (self.parity as u16)
    }
}

/// Apollo PCM telemetry frame: 128 words (CuriousMarc Part 36)
/// Sync pattern is 26 bits (CuriousMarc Part 36: "26 bits for sync pattern")
#[derive(Debug, Clone)]
pub struct TelemetryFrame {
    /// 26-bit sync pattern - IRIG optimal pattern for 26 bits
    /// Pattern: 11111010011010110011000000
    /// Designed so it never reappears in frame data (CuriousMarc Part 36)
    pub sync_pattern: u32,
    pub words: Vec<TelemetryWord>,
    pub frame_number: u32,
    pub timestamp: f64,
}

/// Apollo PCM telemetry uses 128 words per minor frame (CuriousMarc Part 36)
pub const WORDS_PER_FRAME: usize = 128;

/// 26-bit optimal sync pattern (IRIG 106 standard)
/// Last two bits are zeros (CuriousMarc Part 36: "last two bits of the sync pattern both zero")
pub const SYNC_PATTERN_26BIT: u32 = 0b11111010011010110011000000;

impl TelemetryFrame {
    pub fn new(frame_number: u32, met: f64) -> Self {
        Self {
            sync_pattern: SYNC_PATTERN_26BIT,
            words: Vec::with_capacity(WORDS_PER_FRAME),
            frame_number,
            timestamp: met,
        }
    }

    pub fn add_word(&mut self, word: TelemetryWord) {
        if self.words.len() < WORDS_PER_FRAME {
            self.words.push(word);
        }
    }

    pub fn to_bitstream(&self) -> Vec<bool> {
        // 26-bit sync + 128 words × 12 bits each = 26 + 1536 = 1562 bits
        let mut bits = Vec::with_capacity(1562);
        
        // 26-bit sync pattern (MSB first)
        for i in (0..26).rev() {
            bits.push((self.sync_pattern >> i) & 1 == 1);
        }
        
        // 128 words × 12 bits (10 address/data + 2 parity)
        for word in &self.words {
            let word_bits = word.to_bits();
            for i in (0..12).rev() {
                bits.push((word_bits >> i) & 1 == 1);
            }
        }
        
        bits
    }
}

// =============================================================================
// COMMAND WORD & UP DATA LINK
// =============================================================================

/// Up Data Link (UDL) command word format
/// Apollo UDL used PSK modulation with sub-bit encoding (CuriousMarc Parts 32-33)
#[derive(Debug, Clone, Copy)]
pub enum CommandWord {
    Basic { address: u8, data: u16 },
    Extended { address: u8, data1: u16, data2: u16 },
}

impl CommandWord {
    pub fn to_bits(&self) -> u64 {
        match self {
            CommandWord::Basic { address, data } => {
                let parity = Self::calculate_parity_18(*address, *data);
                ((*address as u64) << 13) | ((*data as u64) << 1) | (parity as u64)
            }
            CommandWord::Extended { address, data1, data2 } => {
                let parity = Self::calculate_parity_35(*address, *data1, *data2);
                ((*address as u64) << 30) | ((*data1 as u64) << 18) | 
                ((*data2 as u64) << 6) | (parity as u64)
            }
        }
    }

    fn calculate_parity_18(address: u8, data: u16) -> u8 {
        let address_ones = (0..5).filter(|i| (address >> i) & 1 == 1).count();
        let data_ones = (0..12).filter(|i| (data >> i) & 1 == 1).count();
        ((address_ones + data_ones) % 2) as u8
    }

    fn calculate_parity_35(address: u8, data1: u16, data2: u16) -> u8 {
        let address_ones = (0..5).filter(|i| (address >> i) & 1 == 1).count();
        let data1_ones = (0..12).filter(|i| (data1 >> i) & 1 == 1).count();
        let data2_ones = (0..12).filter(|i| (data2 >> i) & 1 == 1).count();
        ((address_ones + data1_ones + data2_ones) % 2) as u8
    }
}

/// Up Data Link encoder with PSK sub-bit encoding
/// Historical: 4 kHz clock divided to 2 kHz and 1 kHz for PSK (CuriousMarc Part 33)
#[derive(Component, Debug, Clone)]
pub struct UpDataLink {
    pub clock_frequency_hz: f64,
    pub sub_bit_rate_hz: f64,
    pub phase_shift_degrees: f32,
    pub command_queue: VecDeque<CommandWord>,
    pub psk_encoder_state: PskEncoderState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PskEncoderState {
    Idle,
    Encoding,
    Transmitting,
}

impl Default for UpDataLink {
    fn default() -> Self {
        Self {
            clock_frequency_hz: 4_000.0,
            sub_bit_rate_hz: 2_000.0,
            phase_shift_degrees: 180.0,
            command_queue: VecDeque::new(),
            psk_encoder_state: PskEncoderState::Idle,
        }
    }
}

// =============================================================================
// EMERGENCY KEY TONE
// =============================================================================

/// Emergency key sends Morse code tone on 768 kHz subcarrier
/// Used during Apollo 13 when main comm failed (CuriousMarc Part 31)
#[derive(Component, Debug, Clone)]
pub struct EmergencyKeyTone {
    pub frequency_hz: f64,
    pub tone_active: bool,
    pub morse_buffer: VecDeque<char>,
    pub current_symbol: Option<MorseSymbol>,
    pub symbol_timer: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseSymbol {
    Dot,
    Dash,
    LetterSpace,
    WordSpace,
}

impl Default for EmergencyKeyTone {
    fn default() -> Self {
        Self {
            frequency_hz: 768_000.0,
            tone_active: false,
            morse_buffer: VecDeque::new(),
            current_symbol: None,
            symbol_timer: 0.0,
        }
    }
}

// =============================================================================
// USB MULTIPLEXER
// =============================================================================

/// Apollo Unified S-Band multiplexing model
/// Carrier (2.2875 GHz) is PM-modulated with multiple subcarriers:
/// - Voice: FM on 1.25 MHz subcarrier
/// - Data: PSK on 1.024 MHz subcarrier  
/// - Emergency: OOK tone on 768 kHz subcarrier (CuriousMarc Part 39)
#[derive(Component, Debug, Clone)]
pub struct UsbMultiplexer {
    pub carrier_frequency_mhz: f64,
    pub voice_subcarrier_khz: f64,
    pub data_subcarrier_khz: f64,
    pub emergency_subcarrier_khz: f64,
    pub voice_modulation: ModulationType,
    pub data_modulation: ModulationType,
    pub emergency_modulation: ModulationType,
    pub voice_enabled: bool,
    pub data_enabled: bool,
    pub emergency_enabled: bool,
    pub voice_fm_deviation_khz: f64,
    pub data_psk_phase_shift_degrees: f32,
}

impl Default for UsbMultiplexer {
    fn default() -> Self {
        Self {
            carrier_frequency_mhz: 2287.5,
            voice_subcarrier_khz: 1250.0,
            data_subcarrier_khz: 1024.0,
            emergency_subcarrier_khz: 768.0,
            voice_modulation: ModulationType::FrequencyModulation,
            data_modulation: ModulationType::PhaseShiftKeying,
            emergency_modulation: ModulationType::OnOffKeying,
            voice_enabled: true,
            data_enabled: true,
            emergency_enabled: false,
            voice_fm_deviation_khz: 8.0,
            data_psk_phase_shift_degrees: 180.0,
        }
    }
}

// =============================================================================
// SIGNAL CONDITIONING EQUIPMENT (SCE)
// =============================================================================

/// Signal Conditioning Equipment processes analog sensor inputs before PCM
/// "SCE to AUX" switched to auxiliary power during Apollo 12 (CuriousMarc Part 36)
#[derive(Component, Debug, Clone)]
pub struct SignalConditioningEquipment {
    pub primary_power: bool,
    pub auxiliary_power: bool,
    pub analog_inputs: Vec<AnalogInput>,
    pub signal_conditioner_status: SceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SceStatus {
    Primary,
    Auxiliary,
    Failed,
    Off,
}

#[derive(Debug, Clone)]
pub struct AnalogInput {
    pub channel_number: u16,
    pub voltage: f32,
    pub signal_type: SignalType,
    pub conditioned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalType {
    Voltage,
    Current,
    Temperature,
    Pressure,
    Acceleration,
}

impl Default for SignalConditioningEquipment {
    fn default() -> Self {
        Self {
            primary_power: true,
            auxiliary_power: false,
            analog_inputs: Vec::new(),
            signal_conditioner_status: SceStatus::Primary,
        }
    }
}

// =============================================================================
// PLL CIRCUIT
// =============================================================================

/// Phase-Locked Loop for carrier tracking
/// Apollo receivers tracked 2.2875 GHz carrier with Doppler shifts up to ±100 kHz (CuriousMarc Part 30)
#[derive(Component, Debug, Clone)]
pub struct PllCircuit {
    pub center_frequency_mhz: f64,
    pub loop_bandwidth_hz: f64,
    pub locked: bool,
    pub frequency_offset_hz: f64,
    pub doppler_shift_hz: f64,
    pub phase_error_degrees: f32,
    pub lock_indicator: bool,
    pub signal_to_noise_db: f32,
}

impl Default for PllCircuit {
    fn default() -> Self {
        Self {
            center_frequency_mhz: 2287.5,
            loop_bandwidth_hz: 100.0,
            locked: true,
            frequency_offset_hz: 0.0,
            doppler_shift_hz: 0.0,
            phase_error_degrees: 0.0,
            lock_indicator: true,
            signal_to_noise_db: 15.0,
        }
    }
}

// =============================================================================
// COMMUNICATIONS BUS
// =============================================================================

#[derive(Resource)]
pub struct CommunicationsBus {
    pub downlink_rate: DataRate,
    pub uplink_rate: DataRate,
    pub signal_strength: f32,
    pub snr_db: f32,
    pub frequency_mhz: f64,
    pub data_subcarrier_khz: f64,
    pub voice_subcarrier_khz: f64,
    pub emergency_subcarrier_khz: f64,
    pub transmitter_power_watts: f32,
    pub bit_error_rate: f64,
    pub telemetry_buffer: VecDeque<TelemetryFrame>,
    pub command_buffer: VecDeque<CommandWord>,
    pub voice_buffer: VecDeque<f32>,
    pub total_bits_transmitted: u64,
    pub total_bits_received: u64,
    pub mode: CommMode,
    pub ground_station: String,
    pub status: CommStatus,
    pub carrier_modulation: ModulationType,
    pub s_band_transmitter_on: bool,
    pub s_band_receiver_on: bool,
    pub s_band_power_amp_on: bool,
    pub vhf_transmitter_a_on: bool,
    pub vhf_transmitter_b_on: bool,
    pub vhf_receiver_on: bool,
    pub high_gain_antenna_on: bool,
}

impl Default for CommunicationsBus {
    fn default() -> Self {
        Self {
            downlink_rate: DataRate::LowRate1_6kbps,
            uplink_rate: DataRate::LowRate1_6kbps,
            signal_strength: 1.0,
            snr_db: 15.0,
            frequency_mhz: 2287.5,
            data_subcarrier_khz: 1024.0,
            voice_subcarrier_khz: 1250.0,
            emergency_subcarrier_khz: 768.0,
            transmitter_power_watts: 20.0,
            bit_error_rate: 1e-6,
            telemetry_buffer: VecDeque::new(),
            command_buffer: VecDeque::new(),
            voice_buffer: VecDeque::new(),
            total_bits_transmitted: 0,
            total_bits_received: 0,
            mode: CommMode::TelemetryAndVoice,
            ground_station: "Madrid".to_string(),
            status: CommStatus::Nominal,
            carrier_modulation: ModulationType::PhaseModulation,
            s_band_transmitter_on: true,
            s_band_receiver_on: true,
            s_band_power_amp_on: true,
            vhf_transmitter_a_on: true,
            vhf_transmitter_b_on: false,
            vhf_receiver_on: true,
            high_gain_antenna_on: true,
        }
    }
}

// =============================================================================
// COMM MODE & STATUS
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommMode {
    TelemetryAndVoice,
    TelemetryOnly,
    VoiceOnly,
    Emergency,
    Ranging,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommStatus {
    Nominal,
    Degraded,
    Critical,
    Lost,
}

// =============================================================================
// GROUND CONTROL STATE
// =============================================================================

#[derive(Resource)]
pub struct GroundControlState {
    pub station: GroundStation,
    pub uplink_status: UplinkStatus,
    pub last_command: Option<CommandWord>,
    pub command_queue: VecDeque<CommandWord>,
    pub received_telemetry: VecDeque<TelemetryFrame>,
    pub capcom: String,
    pub flight_director: String,
    pub active_loop: VoiceLoop,
}

impl Default for GroundControlState {
    fn default() -> Self {
        Self {
            station: GroundStation::Houston,
            uplink_status: UplinkStatus::Standby,
            last_command: None,
            command_queue: VecDeque::new(),
            received_telemetry: VecDeque::new(),
            capcom: "CAPCOM".to_string(),
            flight_director: "FLIGHT".to_string(),
            active_loop: VoiceLoop::Flight,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundStation {
    Houston,
    Madrid,
    Canberra,
    Goldstone,
    Hawaii,
    Guaymas,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UplinkStatus {
    Standby,
    Transmitting,
    CommandAccepted,
    CommandRejected,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceLoop {
    Flight,
    Capcom,
    Afd,
    Fao,
    Retro,
    Fido,
    Guidance,
    Surgeon,
    Eecom,
    Gnc,
    Telmu,
    Control,
    Inco,
}

// =============================================================================
// USB EQUIPMENT
// =============================================================================

#[derive(Component)]
pub struct UsbEquipment {
    pub power_amplifier_temp: f32,
    pub exciter_temp: f32,
    pub receiver_agc: f32,
    pub phase_lock: bool,
    pub subcarrier_lock: bool,
    pub bit_sync_lock: bool,
    pub traveling_wave_tube_temp: f32,
    pub transmitter_output_watts: f32,
}

impl Default for UsbEquipment {
    fn default() -> Self {
        Self {
            power_amplifier_temp: 45.0,
            exciter_temp: 35.0,
            receiver_agc: 3.5,
            phase_lock: true,
            subcarrier_lock: true,
            bit_sync_lock: true,
            traveling_wave_tube_temp: 120.0,
            transmitter_output_watts: 20.0,
        }
    }
}

// =============================================================================
// SYSTEMS
// =============================================================================

fn setup_communications(mut commands: Commands) {
    commands.spawn((
        UsbEquipment::default(),
        UsbMultiplexer::default(),
        SignalConditioningEquipment::default(),
        PllCircuit::default(),
        UpDataLink::default(),
        EmergencyKeyTone::default(),
    ));
}

fn update_telemetry_downlink(
    mut comms: ResMut<CommunicationsBus>,
    time: Res<Time>,
) {
    if comms.mode == CommMode::Off {
        return;
    }

    let delta_time = time.delta_seconds_f64();
    let bits_to_transmit = (comms.downlink_rate.bits_per_second() * delta_time) as u64;
    comms.total_bits_transmitted += bits_to_transmit;

    let errors = (bits_to_transmit as f64 * comms.bit_error_rate) as u64;
    if errors > 0 {
        comms.status = CommStatus::Degraded;
    }
}

fn process_uplink_commands(
    mut comms: ResMut<CommunicationsBus>,
    mut ground: ResMut<GroundControlState>,
    time: Res<Time>,
) {
    if comms.mode == CommMode::Off || comms.signal_strength < 0.1 {
        return;
    }

    let delta_time = time.delta_seconds_f64();
    let bits_to_receive = (comms.uplink_rate.bits_per_second() * delta_time) as u64;
    comms.total_bits_received += bits_to_receive;

    while let Some(cmd) = ground.command_queue.pop_front() {
        comms.command_buffer.push_back(cmd);
        ground.last_command = Some(cmd);
        ground.uplink_status = UplinkStatus::CommandAccepted;
    }
}

fn update_signal_strength(mut comms: ResMut<CommunicationsBus>) {
    let base_strength = 1.0;
    let noise = (comms.bit_error_rate * 1e6) as f32;
    comms.signal_strength = (base_strength - noise).clamp(0.0, 1.0);
    comms.snr_db = 15.0 + 10.0 * comms.signal_strength.log10();
}

fn simulate_communications_hardware(
    mut usb_query: Query<&mut UsbEquipment>,
    comms: Res<CommunicationsBus>,
) {
    for mut equipment in usb_query.iter_mut() {
        equipment.power_amplifier_temp = 45.0 + (1.0 - comms.signal_strength) * 15.0;
        equipment.exciter_temp = 35.0 + (1.0 - comms.signal_strength) * 10.0;
        equipment.traveling_wave_tube_temp = 120.0 + (1.0 - comms.signal_strength) * 30.0;
        equipment.transmitter_output_watts = 20.0 * comms.signal_strength;
        
        if comms.signal_strength < 0.2 {
            equipment.phase_lock = false;
            equipment.subcarrier_lock = false;
            equipment.bit_sync_lock = false;
        } else if comms.signal_strength < 0.4 {
            equipment.phase_lock = true;
            equipment.subcarrier_lock = false;
            equipment.bit_sync_lock = false;
        } else if comms.signal_strength < 0.6 {
            equipment.phase_lock = true;
            equipment.subcarrier_lock = true;
            equipment.bit_sync_lock = false;
        } else {
            equipment.phase_lock = true;
            equipment.subcarrier_lock = true;
            equipment.bit_sync_lock = true;
        }
    }
}

fn update_pll_tracking(
    mut pll_query: Query<&mut PllCircuit>,
    comms: Res<CommunicationsBus>,
    time: Res<Time>,
) {
    for mut pll in pll_query.iter_mut() {
        // Simulate Doppler shift based on spacecraft velocity (simplified)
        // In reality this would come from orbital mechanics
        let doppler_variation = (time.elapsed_seconds() as f64 * 0.01).sin() * 50_000.0; // ±50 kHz
        pll.doppler_shift_hz = doppler_variation;
        pll.frequency_offset_hz = pll.doppler_shift_hz;
        
        // PLL lock depends on signal strength and SNR
        let lock_threshold = 0.3;
        let snr_lock_threshold = 8.0; // dB
        
        pll.locked = comms.signal_strength > lock_threshold && comms.snr_db > snr_lock_threshold;
        pll.lock_indicator = pll.locked;
        
        if pll.locked {
            pll.phase_error_degrees = (1.0 - comms.signal_strength) * 45.0;
            pll.signal_to_noise_db = comms.snr_db;
        } else {
            pll.phase_error_degrees = 180.0;
            pll.signal_to_noise_db = 0.0;
        }
    }
}

fn update_usb_multiplexer(
    mut mux_query: Query<&mut UsbMultiplexer>,
    comms: Res<CommunicationsBus>,
) {
    for mut mux in mux_query.iter_mut() {
        // Enable/disable subcarriers based on comm mode
        match comms.mode {
            CommMode::TelemetryAndVoice => {
                mux.voice_enabled = true;
                mux.data_enabled = true;
                mux.emergency_enabled = false;
            }
            CommMode::TelemetryOnly => {
                mux.voice_enabled = false;
                mux.data_enabled = true;
                mux.emergency_enabled = false;
            }
            CommMode::VoiceOnly => {
                mux.voice_enabled = true;
                mux.data_enabled = false;
                mux.emergency_enabled = false;
            }
            CommMode::Emergency => {
                mux.voice_enabled = false;
                mux.data_enabled = false;
                mux.emergency_enabled = true;
            }
            CommMode::Ranging => {
                mux.voice_enabled = false;
                mux.data_enabled = false;
                mux.emergency_enabled = false;
            }
            CommMode::Off => {
                mux.voice_enabled = false;
                mux.data_enabled = false;
                mux.emergency_enabled = false;
            }
        }
    }
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

pub fn format_met(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:03}:{:02}:{:02}", hours, minutes, secs)
}

pub fn ground_station_name(station: GroundStation) -> &'static str {
    match station {
        GroundStation::Houston => "Manned Spacecraft Center (Houston)",
        GroundStation::Madrid => "Madrid Tracking Station",
        GroundStation::Canberra => "Canberra Tracking Station",
        GroundStation::Goldstone => "Goldstone Tracking Station",
        GroundStation::Hawaii => "Hawaii Tracking Station",
        GroundStation::Guaymas => "Guaymas Tracking Station",
    }
}

pub fn voice_loop_name(loop_id: VoiceLoop) -> &'static str {
    match loop_id {
        VoiceLoop::Flight => "FLIGHT",
        VoiceLoop::Capcom => "CAPCOM",
        VoiceLoop::Afd => "AFD",
        VoiceLoop::Fao => "FAO",
        VoiceLoop::Retro => "RETRO",
        VoiceLoop::Fido => "FIDO",
        VoiceLoop::Guidance => "GUIDANCE",
        VoiceLoop::Surgeon => "SURGEON",
        VoiceLoop::Eecom => "EECOM",
        VoiceLoop::Gnc => "GNC",
        VoiceLoop::Telmu => "TELMU",
        VoiceLoop::Control => "CONTROL",
        VoiceLoop::Inco => "INCO",
    }
}
