#[cfg(test)]
mod agc_integration {
    use std::path::Path;

    fn comanche_path() -> &'static Path {
        Path::new("assets/agc/Comanche055.bin")
    }

    fn luminary_path() -> &'static Path {
        Path::new("assets/agc/Luminary099.bin")
    }

    fn agc_binaries_exist() -> bool {
        comanche_path().exists() && luminary_path().exists()
    }

    fn init_comanche() -> Option<apollo_mission_simulator::virtual_agc::VirtualAgc> {
        if !agc_binaries_exist() {
            return None;
        }
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        match agc.init(comanche_path(), None) {
            Ok(()) => Some(agc),
            Err(_) => match agc.load_binfile(comanche_path()) {
                Ok(()) => Some(agc),
                Err(_) => None,
            },
        }
    }

    fn init_luminary() -> Option<apollo_mission_simulator::virtual_agc::VirtualAgc> {
        if !agc_binaries_exist() {
            return None;
        }
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        match agc.init(luminary_path(), None) {
            Ok(()) => Some(agc),
            Err(_) => match agc.load_binfile(luminary_path()) {
                Ok(()) => Some(agc),
                Err(_) => None,
            },
        }
    }

    fn send_key(agc: &mut apollo_mission_simulator::virtual_agc::VirtualAgc, key: apollo_mission_simulator::virtual_agc::DskyKey) {
        agc.write_io(apollo_mission_simulator::virtual_agc::AgcChannel::DskyKeyboard as i32, key as i32);
    }

    fn step_n(agc: &mut apollo_mission_simulator::virtual_agc::VirtualAgc, n: usize) {
        for _ in 0..n {
            agc.step();
        }
        agc.channel_routine();
    }

    fn process_keypress(agc: &mut apollo_mission_simulator::virtual_agc::VirtualAgc) {
        step_n(agc, 50000);
    }

    // ============================================================
    // AGC Initialization
    // ============================================================

    #[test]
    fn agc_allocates_state() {
        let agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        drop(agc);
    }

    #[test]
    fn agc_init_comanche055() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        assert!(init_comanche().is_some());
    }

    #[test]
    fn agc_init_luminary099() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        assert!(init_luminary().is_some());
    }

    #[test]
    fn agc_step_100k_cycles_no_crash() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        for _ in 0..100_000 { agc.step(); }
    }

    // ============================================================
    // DSKY Display Decode (Channel 013 — pure logic, no AGC ROM)
    // ============================================================

    #[test]
    fn dsky_decode_r1_positive() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_display = apollo_mission_simulator::virtual_agc::AgcChannel::DskyDisplay as i32;

        let comp0 = (0 << 10) | (0 << 9) | (1 << 5) | 2;
        let comp1 = (1 << 10) | (3 << 8) | (4 << 4) | 5;
        agc.write_io(ch_display, comp0);
        agc.write_io(ch_display, comp1);

        let dsky = agc.get_dsky_state();
        assert_eq!(dsky.r1_sign, '+');
        assert_eq!(dsky.r1, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn dsky_decode_r2_negative() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_display = apollo_mission_simulator::virtual_agc::AgcChannel::DskyDisplay as i32;

        let comp2 = (2 << 10) | (1 << 9) | (6 << 5) | 7;
        let comp3 = (3 << 10) | (8 << 8) | (9 << 4) | 0;
        agc.write_io(ch_display, comp2);
        agc.write_io(ch_display, comp3);

        let dsky = agc.get_dsky_state();
        assert_eq!(dsky.r2_sign, '-');
        assert_eq!(dsky.r2, [6, 7, 8, 9, 0]);
    }

    #[test]
    fn dsky_decode_r3_zero() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_display = apollo_mission_simulator::virtual_agc::AgcChannel::DskyDisplay as i32;

        let comp4 = (4 << 10) | (0 << 9) | (0 << 5) | 0;
        let comp5 = (5 << 10) | (0 << 8) | (0 << 4) | 0;
        agc.write_io(ch_display, comp4);
        agc.write_io(ch_display, comp5);

        let dsky = agc.get_dsky_state();
        assert_eq!(dsky.r3_sign, '+');
        assert_eq!(dsky.r3, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn dsky_decode_prog_verb_noun() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_display = apollo_mission_simulator::virtual_agc::AgcChannel::DskyDisplay as i32;

        agc.write_io(ch_display, (6 << 10) | (1 << 5) | 1);
        agc.write_io(ch_display, (7 << 10) | (1 << 5) | 6);
        agc.write_io(ch_display, (8 << 10) | (3 << 5) | 6);

        let dsky = agc.get_dsky_state();
        assert_eq!(dsky.prog, [1, 1]);
        assert_eq!(dsky.verb, [1, 6]);
        assert_eq!(dsky.noun, [3, 6]);
    }

    #[test]
    fn dsky_annunciator_all_bits() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_lights = apollo_mission_simulator::virtual_agc::AgcChannel::DskyLights as i32;

        agc.write_io(ch_lights, 0x3FF);
        assert_eq!(agc.get_dsky_state().lights, 0x3FF);

        agc.write_io(ch_lights, 0);
        assert_eq!(agc.get_dsky_state().lights, 0);
    }

    #[test]
    fn dsky_max_negative_register() {
        let mut agc = apollo_mission_simulator::virtual_agc::VirtualAgc::new();
        let ch_display = apollo_mission_simulator::virtual_agc::AgcChannel::DskyDisplay as i32;

        agc.write_io(ch_display, (0 << 10) | (1 << 9) | (9 << 5) | 9);
        agc.write_io(ch_display, (1 << 10) | (9 << 8) | (9 << 4) | 9);

        let dsky = agc.get_dsky_state();
        assert_eq!(dsky.r1_sign, '-');
        assert_eq!(dsky.r1, [9, 9, 9, 9, 9]);
    }

    // ============================================================
    // DskyKey and AgcChannel Enum Values
    // ============================================================

    #[test]
    fn dsky_key_octal_values() {
        use apollo_mission_simulator::virtual_agc::DskyKey;
        assert_eq!(DskyKey::Verb as i32, 0o21);
        assert_eq!(DskyKey::Noun as i32, 0o22);
        assert_eq!(DskyKey::Plus as i32, 0o25);
        assert_eq!(DskyKey::Minus as i32, 0o26);
        assert_eq!(DskyKey::Zero as i32, 0o20);
        assert_eq!(DskyKey::One as i32, 0o01);
        assert_eq!(DskyKey::Two as i32, 0o02);
        assert_eq!(DskyKey::Three as i32, 0o03);
        assert_eq!(DskyKey::Four as i32, 0o04);
        assert_eq!(DskyKey::Five as i32, 0o05);
        assert_eq!(DskyKey::Six as i32, 0o06);
        assert_eq!(DskyKey::Seven as i32, 0o07);
        assert_eq!(DskyKey::Eight as i32, 0o10);
        assert_eq!(DskyKey::Nine as i32, 0o11);
        assert_eq!(DskyKey::Clear as i32, 0o23);
        assert_eq!(DskyKey::Enter as i32, 0o24);
        assert_eq!(DskyKey::Reset as i32, 0o31);
        assert_eq!(DskyKey::Proceed as i32, 0o27);
    }

    #[test]
    fn agc_channel_octal_values() {
        use apollo_mission_simulator::virtual_agc::AgcChannel;
        assert_eq!(AgcChannel::DskyKeyboard as i32, 0o15);
        assert_eq!(AgcChannel::DskyDisplay as i32, 0o13);
        assert_eq!(AgcChannel::DskyLights as i32, 0o16);
        assert_eq!(AgcChannel::Scaler1 as i32, 0o04);
        assert_eq!(AgcChannel::Scaler2 as i32, 0o03);
        assert_eq!(AgcChannel::ImuMode as i32, 0o30);
        assert_eq!(AgcChannel::ImuGimbal as i32, 0o33);
        assert_eq!(AgcChannel::Thrust as i32, 0o55);
    }

    // ============================================================
    // DSKY Key Sequences (requires Comanche055.bin)
    // ============================================================

    #[test]
    fn v35_lamp_test() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        send_key(&mut agc, apollo_mission_simulator::virtual_agc::DskyKey::Verb);
        process_keypress(&mut agc);
        send_key(&mut agc, apollo_mission_simulator::virtual_agc::DskyKey::Three);
        process_keypress(&mut agc);
        send_key(&mut agc, apollo_mission_simulator::virtual_agc::DskyKey::Five);
        process_keypress(&mut agc);
        send_key(&mut agc, apollo_mission_simulator::virtual_agc::DskyKey::Enter);
        process_keypress(&mut agc);
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn v37_program_request_p00() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::Three, DskyKey::Seven, DskyKey::Enter,
                     DskyKey::Zero, DskyKey::Zero, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn v36_fresh_start() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::Three, DskyKey::Six, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn v06_n36_display_agc_clock() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::Zero, DskyKey::Six,
                     DskyKey::Noun, DskyKey::Three, DskyKey::Six, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn v16_n62_monitor_inertial() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::One, DskyKey::Six,
                     DskyKey::Noun, DskyKey::Six, DskyKey::Two, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        step_n(&mut agc, 200_000);
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn v21_n33_load_r1() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::Two, DskyKey::One,
                     DskyKey::Noun, DskyKey::Three, DskyKey::Three, DskyKey::Enter,
                     DskyKey::Plus, DskyKey::Zero, DskyKey::Zero,
                     DskyKey::Five, DskyKey::Zero, DskyKey::Zero, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        let _ = agc.get_dsky_state();
    }

    // ============================================================
    // AGC Channel I/O (requires Comanche055.bin)
    // ============================================================

    #[test]
    fn channel_5_zero_after_init() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        assert_eq!(agc.read_io(5), 0);
    }

    #[test]
    fn channel_14_zero_after_init() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_comanche().unwrap();
        assert_eq!(agc.read_io(14), 0);
    }

    // ============================================================
    // Luminary099 (LM AGC)
    // ============================================================

    #[test]
    fn luminary_100k_cycles_no_crash() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_luminary().unwrap();
        for _ in 0..100_000 { agc.step(); }
        let _ = agc.get_dsky_state();
    }

    #[test]
    fn luminary_v37_p63_descent() {
        if !agc_binaries_exist() { eprintln!("SKIPPED"); return; }
        let mut agc = init_luminary().unwrap();
        step_n(&mut agc, 100_000);
        for key in &[DskyKey::Verb, DskyKey::Three, DskyKey::Seven, DskyKey::Enter,
                     DskyKey::Six, DskyKey::Three, DskyKey::Enter] {
            send_key(&mut agc, *key);
            process_keypress(&mut agc);
        }
        let _ = agc.get_dsky_state();
    }

    use apollo_mission_simulator::virtual_agc::DskyKey;
}
