use std::ffi::{c_char, c_int, c_void, CString};
use std::path::Path;

#[repr(C)]
pub struct AgcState {
    _opaque: [u8; 0],
}

extern "C" {
    fn agc_state_alloc() -> *mut c_void;
    fn agc_state_free(state: *mut c_void);
}

#[link(name = "yaAGC", kind = "static")]
extern "C" {
    pub fn agc_engine_init(
        state: *mut c_void,
        rom_image: *const c_char,
        core_dump: *const c_char,
        all_or_erasable: c_int,
    ) -> c_int;

    pub fn agc_engine(state: *mut c_void) -> c_int;

    pub fn agc_load_binfile(state: *mut c_void, rom_image: *const c_char) -> c_int;

    pub fn ReadIO(state: *mut c_void, address: c_int) -> c_int;

    pub fn WriteIO(state: *mut c_void, address: c_int, value: c_int);

    pub fn CpuWriteIO(state: *mut c_void, address: c_int, value: c_int);

    pub fn MakeCoreDump(state: *mut c_void, filename: *const c_char);

    pub fn OverflowCorrected(value: c_int) -> i16;

    pub fn SignExtend(word: i16) -> c_int;

    pub fn AddSP16(addend1: c_int, addend2: c_int) -> c_int;

    pub fn UnprogrammedIncrement(state: *mut c_void, counter: c_int, inc_type: c_int);

    pub fn ChannelOutput(state: *mut c_void, channel: c_int, value: c_int);

    pub fn ChannelInput(state: *mut c_void) -> c_int;

    pub fn ChannelRoutine(state: *mut c_void);
}

/// Tracks DSKY display state by intercepting writes to channel 013.
/// The AGC writes to channel 013 with a multiplexed format where each
/// write updates one component (PROG, VERB, NOUN, R1, R2, R3).
#[derive(Debug, Clone, Default)]
pub struct DskyTracker {
    pub prog: [u8; 2],
    pub verb: [u8; 2],
    pub noun: [u8; 2],
    pub r1: [u8; 5],
    pub r2: [u8; 5],
    pub r3: [u8; 5],
    pub r1_sign: char,
    pub r2_sign: char,
    pub r3_sign: char,
    pub lights: u16,
}

pub struct VirtualAgc {
    state: *mut c_void,
    dsky: DskyTracker,
}

impl Drop for VirtualAgc {
    fn drop(&mut self) {
        if !self.state.is_null() {
            unsafe { agc_state_free(self.state) };
            self.state = std::ptr::null_mut();
        }
    }
}

impl VirtualAgc {
    pub fn new() -> Self {
        let ptr = unsafe { agc_state_alloc() };
        assert!(!ptr.is_null(), "Failed to allocate AGC state");

        Self { state: ptr, dsky: DskyTracker::default() }
    }

    pub fn init(&mut self, rom_path: &Path, core_dump: Option<&Path>) -> Result<(), String> {
        let rom_cstring = CString::new(rom_path.to_str().ok_or("Invalid ROM path")?)
            .map_err(|e| format!("CString conversion failed: {}", e))?;

        let core_cstring = core_dump
            .and_then(|p| CString::new(p.to_str()?).ok());

        let core_ptr = core_cstring
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(std::ptr::null());

        let result = unsafe {
            agc_engine_init(
                self.state,
                rom_cstring.as_ptr(),
                core_ptr,
                0,
            )
        };

        if result == 0 {
            Ok(())
        } else {
            Err(format!("agc_engine_init failed with code: {}", result))
        }
    }

    pub fn load_binfile(&mut self, path: &Path) -> Result<(), String> {
        let cstring = CString::new(path.to_str().ok_or("Invalid path")?)
            .map_err(|e| format!("CString conversion failed: {}", e))?;

        let result = unsafe { agc_load_binfile(self.state, cstring.as_ptr()) };

        if result == 0 {
            Ok(())
        } else {
            Err(format!("agc_load_binfile failed with code: {}", result))
        }
    }

    pub fn step(&mut self) -> i32 {
        unsafe { agc_engine(self.state) }
    }

    pub fn read_io(&mut self, address: i32) -> i32 {
        unsafe { ReadIO(self.state, address) }
    }

    pub fn write_io(&mut self, address: i32, value: i32) {
        if address == AgcChannel::DskyDisplay as i32 {
            self.decode_dsky_write(value);
        }
        if address == AgcChannel::DskyLights as i32 {
            self.dsky.lights = value as u16;
        }
        unsafe { WriteIO(self.state, address, value) }
    }

    fn decode_dsky_write(&mut self, value: i32) {
        let component = ((value >> 10) & 0x1F) as u8;
        let data = value & 0x3FF;

        match component {
            0 => {
                self.dsky.r1_sign = if ((data >> 9) & 1) == 1 { '-' } else { '+' };
                self.dsky.r1[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.r1[1] = (data & 0xF) as u8;
            }
            1 => {
                self.dsky.r1[2] = ((data >> 8) & 0xF) as u8;
                self.dsky.r1[3] = ((data >> 4) & 0xF) as u8;
                self.dsky.r1[4] = (data & 0xF) as u8;
            }
            2 => {
                self.dsky.r2_sign = if ((data >> 9) & 1) == 1 { '-' } else { '+' };
                self.dsky.r2[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.r2[1] = (data & 0xF) as u8;
            }
            3 => {
                self.dsky.r2[2] = ((data >> 8) & 0xF) as u8;
                self.dsky.r2[3] = ((data >> 4) & 0xF) as u8;
                self.dsky.r2[4] = (data & 0xF) as u8;
            }
            4 => {
                self.dsky.r3_sign = if ((data >> 9) & 1) == 1 { '-' } else { '+' };
                self.dsky.r3[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.r3[1] = (data & 0xF) as u8;
            }
            5 => {
                self.dsky.r3[2] = ((data >> 8) & 0xF) as u8;
                self.dsky.r3[3] = ((data >> 4) & 0xF) as u8;
                self.dsky.r3[4] = (data & 0xF) as u8;
            }
            6 => {
                self.dsky.prog[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.prog[1] = (data & 0xF) as u8;
            }
            7 => {
                self.dsky.verb[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.verb[1] = (data & 0xF) as u8;
            }
            8 => {
                self.dsky.noun[0] = ((data >> 5) & 0xF) as u8;
                self.dsky.noun[1] = (data & 0xF) as u8;
            }
            _ => {}
        }
    }

    pub fn get_dsky_state(&self) -> &DskyTracker {
        &self.dsky
    }

    pub fn cpu_write_io(&mut self, address: i32, value: i32) {
        unsafe { CpuWriteIO(self.state, address, value) }
    }

    pub fn channel_output(&mut self, channel: i32, value: i32) {
        unsafe { ChannelOutput(self.state, channel, value) }
    }

    pub fn channel_input(&mut self) -> i32 {
        unsafe { ChannelInput(self.state) }
    }

    pub fn channel_routine(&mut self) {
        unsafe { ChannelRoutine(self.state) }
    }

    pub fn make_core_dump(&mut self, filename: &str) {
        let cstring = CString::new(filename).unwrap();
        unsafe { MakeCoreDump(self.state, cstring.as_ptr()) }
    }
}

impl Default for VirtualAgc {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for VirtualAgc {}
unsafe impl Sync for VirtualAgc {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgcChannel {
    DskyKeyboard = 0o15,
    DskyDisplay = 0o13,
    DskyLights = 0o16,
    Scaler1 = 0o04,
    Scaler2 = 0o03,
    ImuMode = 0o30,
    ImuGimbal = 0o33,
    Thrust = 0o55,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DskyKey {
    Verb = 0o21,
    Noun = 0o22,
    Plus = 0o25,
    Minus = 0o26,
    Zero = 0o20,
    One = 0o01,
    Two = 0o02,
    Three = 0o03,
    Four = 0o04,
    Five = 0o05,
    Six = 0o06,
    Seven = 0o07,
    Eight = 0o10,
    Nine = 0o11,
    Clear = 0o23,
    Enter = 0o24,
    Reset = 0o31,
    Proceed = 0o27,
}

pub fn key_to_channel_value(key: DskyKey) -> i32 {
    key as i32
}
