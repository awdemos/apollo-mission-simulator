# Apollo Mission Simulator — Fault Scenarios Reference

> 80 diagnostic scenarios (F01–F80) for normal and dynamic error challenge playthroughs.
> Each fault is a diagnostic puzzle: **Observe → Isolate → Decide → Act → Verify**.
> Houston (LLM NPC) guides players via radio in authentic ground control style.

---

## Table of Contents

1. [Electrical Power & Distribution (F01–F10)](#electrical-power--distribution-f01f10)
2. [Guidance, Navigation & Control / AGC (F11–F20)](#guidance-navigation--control--agc-f11f20)
3. [Propulsion — SPS, RCS, APS (F21–F30)](#propulsion--sps-rcs-aps-f21f30)
4. [Environmental Control & Life Support (F31–F40)](#environmental-control--life-support-f31f40)
5. [Communications & Instrumentation (F41–F50)](#communications--instrumentation-f41f50)
6. [Structural & Mechanical (F51–F60)](#structural--mechanical-f51f60)
7. [Cryogenics & Propellant Systems (F61–F70)](#cryogenics--propellant-systems-f61f70)
8. [Crew Interface & Procedural Edge Cases (F71–F80)](#crew-interface--procedural-edge-cases-f71f80)
9. [Implementation Notes](#implementation-notes-for-game-design)

---

## Electrical Power & Distribution (F01–F10)

### F01. Main Bus A Undervolt — Transient

- **Trigger:** Fuel cell 1 oxygen purge check valve sticks open during cryo stir, causing transient demand spike.
- **Symptoms:** Main Bus A drops to 24.5V; DC indicator flickers; cryo fan load shedding activates; AGC receives power caution light.
- **Resolution:** Identify offending fuel cell via DC voltmeter panel; isolate Fuel Cell 1 from Bus A using FC disconnect switches; reconfigure Bus A to draw from FC 2 and 3; manually balance load by shedding non-critical cabin fans. Use multimeter to verify bus stability before reconnecting.
- **Tools:** Circuit breaker puller, multimeter, load checklist.
- **Severity:** Moderate. Time pressure: 8 minutes before AGC restart required.

### F02. Fuel Cell 3 Failure — Condenser Exhaust High

- **Trigger:** Fuel cell 3 condenser exhaust temperature exceeds nominal (205°F) due to contaminant buildup.
- **Symptoms:** FC 3 voltage output drops to 26V under load; H2 purge line shows frost; O2 flowmeter reads low; master caution illuminates.
- **Resolution:** Perform manual FC 3 purge sequence using panel switches; if unsuccessful, isolate FC 3, shed cryo fans and cabin equipment to maintain Bus B above 27V; calculate remaining mission power budget using amp-hour log.
- **Tools:** FC purge switch, slide rule for power budget, voltmeter.
- **Severity:** High. Apollo 13-style cascade risk if not contained.

### F03. AC Bus 1 Phase Loss

- **Trigger:** Inverter 1 loses one phase due to loose cannon plug in aft equipment bay.
- **Symptoms:** AC1 bus frequency unstable (380–405 Hz); glycol pumps slow; cabin fans surge; AGC DSKY display dims intermittently.
- **Resolution:** Switch AC1 load to Inverter 2 via AC bus switch; access aft equipment bay to torque cannon plug P3 on Inverter 1; restore dual-bus redundancy after verifying phase lock with synchroscope.
- **Tools:** Torque wrench (cannon plug), synchroscope, flashlight.
- **Severity:** Moderate.

### F04. Battery A Thermal Runaway

- **Trigger:** Battery A internal short develops; temperature climbs past 90°F.
- **Symptoms:** Battery A case distension visible via hatch window; battery charger shows reverse current; smoke odor in CM; H2 concentration rise in battery compartment.
- **Resolution:** Isolate Battery A from all buses immediately; open battery compartment vent valve to vacuum; switch entry batteries to manual monitoring; if rupture imminent, jettison CM RCS propellant to reduce ignition risk nearby. Reconfigure power for entry using Batteries B/C only.
- **Tools:** Bus isolation switches, vent valve handle, thermal monitor.
- **Severity:** Critical. Catastrophic if not isolated within 4 minutes.

### F05. Fuel Cell O2/H2 Imbalance — Crossover Leak

- **Trigger:** O2/H2 reactant crossover valve develops internal leak; fuel cell stack begins consuming O2 at 2:1 ratio instead of balanced.
- **Symptoms:** O2 tank pressure drops faster than H2; fuel cell voltage sags under peak load; water production increases; condenser exit temp drops.
- **Resolution:** Monitor reactant pressures on meter panel; calculate consumption rates manually; if imbalance exceeds 10 psi differential, manually equalize using reactant valve switches; prepare to isolate offending fuel cell and rely on remaining two.
- **Tools:** Pressure gauge, manual log, reactant valve switches.
- **Severity:** Moderate. Apollo 13 precursor condition.

### F06. Pyro Bus A Short

- **Trigger:** Moisture ingress in pyrotechnic bus wiring causes resistive short.
- **Symptoms:** Pyro bus A voltage drops to 18V; RCS propellant isolation valves fail to fire on test; SM jettison capability degraded.
- **Resolution:** Trace pyro bus A wiring using schematic; identify shorted leg via isolation switches; switch all pyro functions to Bus B; if both buses compromised, manually arm individual pyro circuits and fire using direct battery taps (emergency procedure).
- **Tools:** Schematic panel, multimeter, wire cutters (emergency isolation).
- **Severity:** High. Blocks SM separation if unresolved.

### F07. DC-to-DC Converter Failure — AGC Supply

- **Trigger:** +14V logic supply converter fails; AGC receives dirty power.
- **Symptoms:** DSKY segments flicker; AGC restarts spontaneously; program alarms 1103, 1104; computer enters standby during critical burn.
- **Resolution:** Monitor AGC power on DC voltmeter; switch AGC to backup +14V converter via AGC circuit breakers; verify DSKY brightness stabilization; re-enter program data lost during restart using backup pad.
- **Tools:** Circuit breaker panel, DSKY keyboard, backup data pad.
- **Severity:** Critical during powered flight.

### F08. Entry Battery Cell Reversal

- **Trigger:** One cell in Entry Battery B reverses polarity under high load during re-entry.
- **Symptoms:** Battery B voltage collapses to 20V; sequential system failures: RCS propellant transfer pumps, CM RCS pressurization; CM/SM separation pyros underpowered.
- **Resolution:** Immediately shed non-critical loads (cabin lights, comm secondary); isolate Battery B; redistribute entry loads to Batteries A and C; manually calculate revised entry trajectory with reduced RCS availability using charts.
- **Tools:** Load shed checklist, entry charts, manual RCS firing switches.
- **Severity:** Critical.

### F09. Cryo Fan Motor Seizure — Bus Overload

- **Trigger:** Cryo O2 fan 2 motor bearing seizes; locked rotor current overloads Bus B.
- **Symptoms:** Master alarm; Bus B voltage sags to 25V; cryo O2 pressure stratification begins; FC 2 and 3 O2 flow drops.
- **Resolution:** Pull Cryo Fan 2 circuit breaker immediately to save Bus B; verify O2 tank pressure equalization via manual pressure readings; if stratification exceeds 20 psi delta, perform manual tank stir using direct battery connection to Fan 1 (alternate wiring).
- **Tools:** Circuit breaker puller, jumper cables (stowed), pressure gauge.
- **Severity:** High. Apollo 13 analog.

### F10. Ground Power Umbilical Disconnect Failure

- **Trigger:** T-0 launch commit and CM umbilical fails to separate; ground power still coupled as vehicle ascends.
- **Symptoms:** Internal power buses show ground loop; DC voltmeter reads 32V (ground overvoltage); RCS isolation valves fail to arm; launch escape tower logic confused.
- **Resolution:** Detect umbilical via hatch window indicator; manually pull CM umbilical disconnect handle in cabin; if still engaged, switch all buses to internal fuel cells and physically cut ground sense wire using emergency wire cutters to prevent backfeed.
- **Tools:** Umbilical disconnect handle, wire cutters, voltmeter.
- **Severity:** Critical. Must resolve before Max-Q.

---

## Guidance, Navigation & Control / AGC (F11–F20)

### F11. AGC 1201/1202 Executive Overload

- **Trigger:** Rendezvous radar left in SLEW during descent; computer overloaded with spurious angle data.
- **Symptoms:** DSKY flashes 1201 or 1202; V16 N68 display freezes; landing program continues but throttle commands lag.
- **Resolution:** Recognize alarm code on DSKY; verify radar mode switch not in LGC (Lunar Guidance Computer) slew; switch radar to LGC AUTO or OFF; monitor DSKY for alarm cessation; if alarm persists, proceed with manual throttle override and abort options.
- **Tools:** DSKY, Mode Switch Panel, LM Throttle.
- **Severity:** Critical during descent. Apollo 11 historical.

### F12. IMU Gimbal Lock — Platform Tumble

- **Trigger:** Excessive spacecraft rotation rates align middle gimbal with outer gimbal (±85°), causing loss of one degree of freedom.
- **Symptoms:** FDAI attitude display goes wild; AGC issues 402 alarm; platform alignment lost; stars no longer track in optics.
- **Resolution:** Immediately cease rotation; recognize gimbal lock by FDAI behavior; use BMAGs (Body-Mounted Attitude Gyros) for coarse rate damping; perform IMU coarse align using known star sightings (AOT or sextant); realign platform using P52 program.
- **Tools:** FDAI, AOT, DSKY (P52), coarse alignment checklist.
- **Severity:** High. Loss of navigation until realigned.

### F13. AGS (Abort Guidance System) Drift — LM

- **Trigger:** AGS gyro assembly experiences uncommanded drift of 0.5°/hour.
- **Symptoms:** AGS attitude diverges from AGC by >2°; cross-pointer display shows conflicting data; abort trajectory calculations erroneous.
- **Resolution:** Compare AGS and AGC attitude displays; if AGS diverging, switch guidance to PGNS (Primary); attempt AGS gyro torquing via DEDA (Data Entry and Display Assembly); if drift exceeds correction capability, compute abort manually using AGC and visual landmark tracking.
- **Tools:** DEDA, DSKY, cross-pointer display, landmark charts.
- **Severity:** Moderate. Abort capability degraded.

### F14. DSKY Segment Failure — Display Corruption

- **Trigger:** DSKY electroluminescent segment driver fails; critical digits unreadable.
- **Symptoms:** Program alarm codes display with missing segments (e.g., 1202 shows as "1 2 2"); velocity readouts ambiguous; crew cannot verify burn parameters.
- **Resolution:** Identify failed segment pattern; cross-reference with printed backup display tables; switch to alternate DSKY if LM configuration; if CM-only, use V16 Nxx requests to force display on working register areas; verbally confirm readouts with ground if comm available.
- **Tools:** DSKY, backup display tables, comm panel.
- **Severity:** High during critical burns.

### F15. Optics Coupling Failure — Sextant Trunnion Jam

- **Trigger:** Sextant trunnion bearing lubricant cold-soaks in shadow; manual drive jams at 45° elevation.
- **Symptoms:** Optics mode switch to SXT shows no response; star sightings impossible; P52 alignment fails; platform alignment degrades over hours.
- **Resolution:** Attempt optics drive via manual hand crank (stowed in equipment bay); if mechanical jam persists, use AOT (Alignment Optical Telescope) for coarse alignment; perform star sightings using AOT and manual star chart interpolation; update AGC with degraded alignment and monitor drift.
- **Tools:** Sextant hand crank, AOT, star charts, DSKY.
- **Severity:** Moderate. Cumulative navigation error risk.

### F16. RCS Thruster Valve Stuck Open — CM

- **Trigger:** RCS thruster 16 (pitch down) primary valve fails open; continuous thrust.
- **Symptoms:** Uncommanded pitch rotation; FDAI shows continuous rate; fuel/oxidizer quantities deplete in quad D; spacecraft enters spin.
- **Resolution:** Recognize stuck thruster by fuel flow and attitude rate; manually isolate quad D using RCS propellant isolation switches; use opposing quads (A/B/C) to null rates; if propellant depletion threatens, prepare for emergency SM separation and CM RCS takeover.
- **Tools:** RCS isolation switches, FDAI, manual translation controller.
- **Severity:** Critical.

### F17. AGC Memory Bit Flip — Cosmic Ray SEU

- **Trigger:** Single-event upset flips bit in AGC erasable memory; corrupts landing radar slope calculation.
- **Symptoms:** Landing radar altitude readings jump erratically; AGC commands throttle oscillations; DSKY shows nonsensical altitude rates.
- **Resolution:** Detect anomaly via cross-check with LR tapemeter and visual lunar surface; if AGC data suspect, switch to manual throttle control; attempt AGC restart (V36E) if time permits; if restart fails, use LR tapemeter and piloting charts for manual landing.
- **Tools:** DSKY (V36E), LR tapemeter, throttle, landing charts.
- **Severity:** Critical during descent.

### F18. Gimbal Motor Failure — SPS Engine

- **Trigger:** SPS pitch gimbal actuator motor fails; engine thrust vector fixed off-center.
- **Symptoms:** SPS burn induces uncommanded pitch moment; spacecraft rotates despite RCS compensation; FDAI shows steady rate during burn.
- **Resolution:** Detect via FDAI rate during SPS thrusting; manually trim using RCS at high propellant cost; if gimbal stuck >2° off null, abort burn, perform MCC (Mid-Course Correction) using RCS only, and recalculate trajectory using charts.
- **Tools:** FDAI, RCS hand controller, burn charts, DSKY.
- **Severity:** High. Major trajectory perturbation.

### F19. LM PGNS/AGS Mode Confusion — Landing

- **Trigger:** Mode select switch between PGNS and AGS develops intermittent contact; system alternates guidance sources mid-descent.
- **Symptoms:** Throttle commands erratic; altitude rate jumps between PGNS and AGS values; landing radar data rejected intermittently.
- **Resolution:** Monitor cross-pointer and DSKY for source switching; firmly select PGNS or AGS mode and tape switch in position; if PGNS failed, manually compute landing trajectory using AGS and DEDA; if both failed, abort using visual piloting and abort stage.
- **Tools:** Mode switch, DEDA, DSKY, throttle.
- **Severity:** Critical.

### F20. Abort Stage Command Failure — LM

- **Trigger:** Abort stage relay fails to close; ascent stage cannot separate from descent stage.
- **Symptoms:** Abort stage button pressed; no separation; DSKY shows program 71 but stage remains attached; descent propellant depleting.
- **Resolution:** Verify abort stage circuit breakers closed; attempt manual stage separation using explosive bolt bypass switches; if bolts fire but structural latch remains, use manual override handles in LM cabin to physically release interstage latches; fire ascent engine with descent attached (emergency) if necessary.
- **Tools:** Abort stage button, bypass switches, manual release handles.
- **Severity:** Catastrophic if unresolved.

---

## Propulsion — SPS, RCS, APS (F21–F30)

### F21. SPS Helium Pressure Drop

- **Trigger:** SPS helium pressurization system leak; pressure decays from 3800 psi.
- **Symptoms:** SPS propellant tank pressure drops; engine cannot achieve full thrust; burn duration extends.
- **Resolution:** Monitor helium pressure gauge; if leak rate <50 psi/min, calculate maximum available SPS burn time; use SPS only for critical burns (LOI, TEI); substitute RCS for mid-course corrections; if pressure <2000 psi, consider SPS unusable and plan free-return trajectory.
- **Tools:** Pressure gauge, slide rule, trajectory charts.
- **Severity:** High.

### F22. RCS Quad Depletion — Asymmetric

- **Trigger:** RCS quad A oxidizer tank depletes due to stuck thruster or leak.
- **Symptoms:** Quad A fuel/ox imbalance; CM translation in +X direction impaired; attitude control requires excessive opposing quad usage.
- **Resolution:** Identify depleted quad via quantity gauges; isolate quad A using propellant isolation switches; recenter CM RCS remaining propellant using crossfeed valves (if equipped); compensate for asymmetric thrust by biasing trim gimbals; calculate remaining RCS budget for entry.
- **Tools:** Quantity gauges, isolation switches, crossfeed valves, calculator.
- **Severity:** Moderate to High.

### F23. SPS Engine Rough Combustion — Injector Face Damage

- **Trigger:** SPS injector face develops hot spot; combustion instability detected.
- **Symptoms:** SPS chamber pressure oscillates ±15%; spacecraft vibration; PUGS (Propellant Utilization and Gauging System) shows off-nominal mixture ratio.
- **Resolution:** Monitor chamber pressure gauge; if oscillation >10%, manually terminate SPS burn; switch to RCS for trajectory correction; inspect SPS nozzle extension via optics for hot streaks; if nozzle breach suspected, do not restart SPS.
- **Tools:** Chamber pressure gauge, optics, abort switches.
- **Severity:** High. Engine destruction risk.

### F24. LM Descent Engine Particle Impact — Fuel Enrichment

- **Trigger:** Descent engine fuel injector clogged by particle; oxidizer-rich combustion erodes chamber.
- **Symptoms:** Descent engine thrust drops 10%; fuel pressure low; chamber pressure high; LM descent rate increases unexpectedly.
- **Resolution:** Cross-check fuel/ox pressures; if fuel pressure low, manually throttle down to 50% to reduce erosion; if thrust continues decaying, abort descent using ascent engine (P71); if abort unavailable, perform hover-slam landing at higher than planned descent rate.
- **Tools:** Throttle, pressure gauges, abort stage button.
- **Severity:** Critical during descent.

### F25. APS (Ascent Propulsion System) Helium Regulator Failure

- **Trigger:** APS helium regulator fails open; overpressurizes fuel/ox tanks.
- **Symptoms:** APS tank pressures rise above 280 psi; tank relief valves open; propellant temperatures drop due to Joule-Thomson cooling; engine start transient violent.
- **Resolution:** Monitor APS tank pressures; if >270 psi, manually vent helium using override valve; if regulator stuck open, isolate helium bottle and accept reduced APS ullage pressure; calculate APS burn duration with degraded pressurization.
- **Tools:** Pressure gauges, helium vent valve, calculator.
- **Severity:** High. Tank rupture risk.

### F26. RCS Thruster Nozzle Burnthrough

- **Trigger:** RCS thruster 5 nozzle develops crack; hot gas impinges on CM heat shield.
- **Symptoms:** Thruster 5 chamber pressure drops; CM aft compartment temperature rises; heat shield ablation smell in cabin.
- **Resolution:** Immediately isolate quad containing thruster 5; inspect heat shield integrity via aft equipment bay window; if heat shield damage confirmed, evaluate entry risk; if damage localized, plan ballistic entry with reduced G-load; if damage extensive, consider skip entry or ocean landing away from recovery forces.
- **Tools:** Isolation switches, temperature probe, visual inspection.
- **Severity:** Catastrophic if heat shield compromised.

### F27. SPS Propellant Utilization Valve Stuck — Fuel Rich

- **Trigger:** PUGS mixture ratio valve sticks in fuel-rich position; SPS specific impulse drops.
- **Symptoms:** SPS fuel depletes faster than oxidizer; total impulse reduced; burn duration for TEI insufficient by 8%.
- **Resolution:** Monitor fuel/ox quantity gauges during burn; if fuel depleting >5% ahead of ox, manually command PUGS to oxidizer-rich to balance; if valve stuck, compute actual ΔV achieved vs. required; use RCS to make up deficit if within budget; otherwise, plan alternate Earth entry corridor.
- **Tools:** PUGS switches, quantity gauges, ΔV charts.
- **Severity:** High.

### F28. LM DPS Propellant Stratification

- **Trigger:** Long coast in lunar orbit causes DPS propellants to stratify; ullage bubble migrates.
- **Symptoms:** DPS engine start results in cavitation; thrust oscillation; propellant feed lines gas-bound.
- **Resolution:** Prior to DPS start, perform ullage burn using RCS (4x +X thrusters, 7.5 seconds); monitor DPS propellant tank pressures for equalization; if cavitation persists, throttle DPS to 10% and hold until propellants settle; then proceed to full throttle.
- **Tools:** RCS controller, DPS throttle, pressure gauges.
- **Severity:** Moderate. Can cause hard start.

### F29. CM RCS Hypergolic Ignition Failure

- **Trigger:** CM RCS thruster fails to ignite due to fuel injector blockage; raw propellant accumulates.
- **Symptoms:** No thrust despite valve open; fuel/ox quantities decrease with no attitude change; aft compartment fuel odor; ignition attempt produces "hard start" bang.
- **Resolution:** Close thruster valve immediately; if raw propellant accumulation suspected, vent aft compartment using cabin relief valve; allow compartment to purge for 2 minutes; attempt reignition once; if second failure, permanently isolate thruster and rely on remaining 15.
- **Tools:** Thruster isolation switches, cabin relief valve.
- **Severity:** High. Explosion risk if propellant pools.

### F30. SPS Gimbal Actuator Hydraulic Lock

- **Trigger:** SPS gimbal actuator hydraulic fluid cold-soaks; actuator jams at null position.
- **Symptoms:** SPS burn produces no attitude perturbation initially; then sudden release as hydraulic fluid warms; unpredictable thrust vector.
- **Resolution:** Prior to SPS burn, cycle gimbal actuators through ±2° using manual trim switches to warm hydraulic fluid; if actuator still stuck, abort SPS burn; use RCS for all attitude control during burn; manually steer using RCS hand controller with programmed offsets.
- **Tools:** SPS trim switches, RCS controller, hydraulic temp gauge.
- **Severity:** High.

---

## Environmental Control & Life Support (F31–F40)

### F31. Cabin O2 Partial Pressure Drop

- **Trigger:** O2 tank 1 supply valve malfunction; flow restricted to cabin.
- **Symptoms:** Cabin pressure drops to 4.8 psia; O2 partial pressure gauge shows 2.8 psi (below 3.0 minimum); crew hypoxia symptoms; CO2 scrubber works harder.
- **Resolution:** Verify O2 tank 1 pressure; switch cabin O2 supply to tank 2; manually adjust O2 flow rate using cabin regulator to 0.5 lb/hr; monitor crew via biometrics; if both tanks compromised, use emergency O2 masks from CM supplies.
- **Tools:** O2 regulator, tank switches, biometrics.
- **Severity:** High. Crew incapacitation risk.

### F32. CO2 Scrubber Canister Saturation — Primary

- **Trigger:** Primary CO2 canister LiOH exhausted; CO2 levels rise.
- **Symptoms:** CO2 partial pressure exceeds 7.6 mmHg; crew headaches; condensation on windows; master caution.
- **Resolution:** Monitor CO2 partial pressure gauge; replace primary canister with spare from stowage; if no spares, construct emergency canister using CM hose, tape, and spare LiOH bags (Apollo 13 procedure); verify flow with tissue paper test.
- **Tools:** Canister wrench, tape, LiOH bags, hose.
- **Severity:** High. Apollo 13 historical analog.

### F33. Glycol Pump Failure — Secondary Loop

- **Trigger:** Secondary glycol pump motor burns out; cooling loop stagnant.
- **Symptoms:** ECS temp rises in equipment bays; electronics overtemp caution; cabin humidity climbs; water separator efficiency drops.
- **Resolution:** Switch to primary glycol pump; if primary also failed, manually bypass secondary loop and use cabin fans to convectively cool equipment; monitor electronics temps; if IMU or AGC approaches 120°F, power down non-critical systems.
- **Tools:** Pump switch, bypass valves, temp gauges.
- **Severity:** Moderate.

### F34. Water Separator Failure — Humidity Cascade

- **Trigger:** Water separator motor fails; humidity removal stops.
- **Symptoms:** Cabin relative humidity >80%; condensation on all cold surfaces; electronics short risk; suit circuit humidity rises.
- **Resolution:** Attempt manual restart of water separator; if failed, increase cabin temperature to reduce relative humidity (counterintuitive but effective); use portable dehumidifier canisters if stowed; monitor suit circuit for water ingestion; if suit flooded, switch to direct O2 mode.
- **Tools:** Temp controller, portable canisters, suit switches.
- **Severity:** Moderate.

### F35. Suit Circuit O2 Flow Obstruction

- **Trigger:** Suit circuit hose kinked or filter clogged; astronaut receives reduced O2 flow.
- **Symptoms:** Suit pressure drops; astronaut reports breathing difficulty; flowmeter shows <4 lb/hr.
- **Resolution:** Trace suit circuit hoses; clear kink or replace filter cartridge; if flow still low, switch astronaut to direct O2 bypass; if both suits affected, use emergency O2 masks and abort mission.
- **Tools:** Hose, filter cartridges, suit disconnects.
- **Severity:** High. Crew survival threat.

### F36. Waste Management System Vacuum Failure

- **Trigger:** Waste stowage vent valve stuck closed; vacuum loss.
- **Symptoms:** Odor in cabin; waste management system pressure equalizes with cabin; fecal bag sealing fails.
- **Resolution:** Attempt manual valve actuation; if stuck, use portable urine collection bags and seal fecal bags with extra adhesive; increase cabin O2 flow to dilute odors; post-mission biohazard protocol.
- **Tools:** Manual valve handle, adhesive, collection bags.
- **Severity:** Low (hygiene/morale). Not mission-critical but immersive.

### F37. Cabin Pressure Integrity Leak — Micrometeoroid

- **Trigger:** Micrometeoroid punctures CM hull; slow leak.
- **Symptoms:** Cabin pressure drops 0.1 psi/hr; hissing sound; pressure suit inflates slightly.
- **Resolution:** Locate leak using stethoscope and soapy water (if accessible); apply emergency hull patch kit (rubber plug + epoxy); if leak in inaccessible area, increase O2 makeup flow to maintain pressure; calculate leak rate vs. O2 reserves; if leak rate exceeds makeup capacity, prepare for emergency deorbit.
- **Tools:** Stethoscope, patch kit, epoxy, O2 regulator.
- **Severity:** High. Cumulative resource drain.

### F38. Heat Shield Ablation Anomaly — Localized Overheating

- **Trigger:** Heat shield density variation causes localized hotspot during entry.
- **Symptoms:** CM aft compartment temp rises abnormally; heat shield telemetry shows 50°F above nominal; potential breach.
- **Resolution:** Monitor aft compartment temps; if >200°F, prepare for structural breach; orient CM to distribute heat load using RCS (if any remaining); if breach imminent, brace for hard landing; deploy parachutes at max dynamic pressure limit.
- **Tools:** Temp sensors, RCS controller, parachute switches.
- **Severity:** Catastrophic.

### F39. Potable Water Tank Contamination

- **Trigger:** Bacterial growth in potable water tank; water turbid.
- **Symptoms:** Water from dispenser cloudy; odor; crew nausea if consumed.
- **Resolution:** Isolate potable water tank; switch to contingency water packs; if no packs, use urine processing system (if equipped) or condensate from ECS; treat with iodine tablets.
- **Tools:** Isolation valve, contingency packs, iodine.
- **Severity:** Low to Moderate.

### F40. Suit Temperature Control Valve Stuck — Cold

- **Trigger:** Suit temp control valve stuck in full cold; astronaut hypothermia.
- **Symptoms:** Suit outlet temp 35°F; astronaut shivering; dexterity impaired; biometrics show dropping core temp.
- **Resolution:** Manually bypass temp control valve using suit circuit bypass; use body heat and exercise to maintain temp; if valve cannot be bypassed, abort EVA or mission segment; warm astronaut with blankets and direct cabin heat.
- **Tools:** Bypass valve, blankets, exercise protocol.
- **Severity:** Moderate.

---

## Communications & Instrumentation (F41–F50)

### F41. S-Band Power Amplifier Failure

- **Trigger:** S-band PA tube fails; downlink signal drops 20 dB.
- **Symptoms:** Ground reports weak signal; telemetry dropouts; voice comm garbled; TV transmission impossible.
- **Resolution:** Switch S-band PA to secondary amplifier; if both failed, switch to VHF-AM for voice (range-limited); use high-gain antenna manual positioning to maximize margin; if all comm lost, rely on backup CSM recovery beacon on entry.
- **Tools:** PA switch, antenna cranks, VHF panel.
- **Severity:** High. Loss of ground support.

### F42. VHF Ranging Failure — LM/CSM

- **Trigger:** VHF ranging transponder fails; LM cannot determine range/rate to CSM.
- **Symptoms:** LM cross-pointer display shows no VHF data; rendezvous radar primary; AGS rendezvous program lacks backup ranging.
- **Resolution:** Switch VHF ranging to alternate channel; if failed, use rendezvous radar as primary; if radar also failed, compute range/rate manually using sextant angle measurements and DSKY orbital mechanics programs.
- **Tools:** VHF panel, sextant, DSKY.
- **Severity:** Moderate during rendezvous.

### F43. Telemetry Encoder Failure — Data Loss

- **Trigger:** PCM telemetry encoder fails; all spacecraft data to ground lost.
- **Symptoms:** Ground reports "no telemetry"; crew unaware of ground visibility; biomedical data lost.
- **Resolution:** Switch telemetry to backup encoder; if failed, manually record critical parameters (cabin pressure, O2, CO2, temps) on voice downlink every 15 minutes; use minimal voice to preserve power and ground link.
- **Tools:** Encoder switch, voice comm, logbook.
- **Severity:** Moderate. Ground blind but crew can operate.

### F44. High Gain Antenna Gimbal Jam

- **Trigger:** HGA pitch gimbal jams at 45° elevation; cannot track Earth.
- **Symptoms:** Signal strength drops as Earth moves out of antenna pattern; ground commands lost during lunar orbit.
- **Resolution:** Attempt HGA drive reset; if jammed, switch to omni-directional antennas (lower gain, higher power); manually position HGA using cranks; if still stuck, use omni antennas and accept reduced data rate; plan comm windows when Earth in omni coverage.
- **Tools:** HGA cranks, antenna switch, comm timeline.
- **Severity:** Moderate.

### F45. CSM/LM Intercom Failure

- **Trigger:** Audio center fails; CSM and LM cannot communicate internally.
- **Symptoms:** Push-to-talk produces no response; crew isolated in separate vehicles.
- **Resolution:** Switch intercom to backup audio bus; if failed, use VHF simplex (both vehicles transmit/receive on same freq); if VHF unavailable, use hardwired umbilical comm if docked; if undocked, use scheduled light signals or Morse via RCS thrusters (emergency).
- **Tools:** Audio panel, VHF radio, umbilical.
- **Severity:** Moderate. Coordination impossible without workaround.

### F46. Biomedical Sensor Failure — ECG Artifact

- **Trigger:** Biomedical harness electrode detaches; ECG shows flatline artifact.
- **Symptoms:** Ground panic; "crewman down" call; actual crew fine but telemetry misleading.
- **Resolution:** Crew verifies health via voice; ground confirms via voice; astronaut reattaches electrode or switches to backup biomed harness; if harness damaged, manually monitor pulse and respiration, report via voice every 30 minutes.
- **Tools:** Biomed harness, voice comm, manual stethoscope.
- **Severity:** Low. False alarm but mission distraction.

### F47. Updata Link Corruption — Adversarial Command Injection

- **Trigger:** Ground uplink corrupted by solar interference or spoofing; AGC receives incorrect state vector.
- **Symptoms:** DSKY shows updated state vector but ground track diverges; AGC navigation conflicts with ground radar.
- **Resolution:** Verify uplink data via voice readback before incorporation; if AGC already loaded bad data, manually re-enter correct state vector using DSKY and pad data; if corruption persistent, reject all uplinks and navigate autonomously using P21 (Ground Track Determination).
- **Tools:** DSKY, voice comm, state vector pad.
- **Severity:** High. Navigation corruption.

### F48. Event Timer Malfunction — Countdown Freeze

- **Trigger:** Event timer crystal oscillator drifts; countdown freezes or runs at half speed.
- **Symptoms:** Burn timing inaccurate; manual procedures mis-timed; DSKY MET disagrees with event timer.
- **Resolution:** Cross-check timer against AGC MET (Mission Elapsed Time); if timer failed, use AGC as master clock; if both disagree, use wristwatch and ground-reported GET (Ground Elapsed Time); manually compute burn ignition time.
- **Tools:** Event timer, DSKY, wristwatch, voice comm.
- **Severity:** Moderate.

### F49. TV Camera Overheating — Lunar Surface

- **Trigger:** Lunar surface TV camera thermal design inadequate for extended ops; overheats.
- **Symptoms:** Picture rolls, loses sync, then goes black; public broadcast interrupted; EVA documentation lost.
- **Resolution:** Power cycle camera; if overheating, shade with PLSS (Portable Life Support System) thermal blanket; alternate between two cameras; if both failed, use 16mm DAC (Data Acquisition Camera) for documentation; prioritize EVA safety over TV.
- **Tools:** Power switch, thermal blanket, DAC.
- **Severity:** Low. Operations continue.

### F50. Radio Direction Finder Failure — Recovery

- **Trigger:** CSM recovery beacon fails; ground stations cannot locate splashdown point.
- **Symptoms:** No beacon on 243 MHz; recovery aircraft search pattern widens.
- **Resolution:** Activate backup beacon (406 MHz if equipped); use VHF voice to report position to recovery forces; deploy dye marker and flashing light upon splashdown; if all beacons failed, rely on ground radar tracking of entry trajectory.
- **Tools:** Beacon switch, dye marker, VHF radio.
- **Severity:** Moderate. Post-splashdown recovery delay.

---

## Structural & Mechanical (F51–F60)

### F51. CM/SM Separation Bolt Misfire

- **Trigger:** One of four CM/SM separation bolts fails to fire; CM hangs on SM.
- **Symptoms:** CM/SM separation sequence initiated but jolt incomplete; CM attitude perturbed; RCS firing ineffective against SM mass.
- **Resolution:** Verify separation via window visual; if SM still attached, fire remaining bolts individually using manual pyro switches; if structural latch remains engaged, use RCS to wrench CM free with rotational impulse; if still attached, prepare for entry with SM (non-standard, high CG).
- **Tools:** Pyro switches, RCS controller, visual inspection.
- **Severity:** Critical.

### F52. LM Landing Gear Strut Failure

- **Trigger:** One LM landing gear strut collapses on touchdown; LM tilts.
- **Symptoms:** LM attitude 15° off vertical; propellant slosh; ascent stage engine gimbal hits mechanical stop.
- **Resolution:** Assess tilt angle via window and shadow; if <15°, proceed with surface ops cautiously; if >15°, abort surface stay immediately; fire ascent engine at tilt (engine gimbal compensates within limits); if ascent impossible, use RCS to push LM upright before staging.
- **Tools:** Window observation, RCS, ascent stage switch.
- **Severity:** High.

### F53. Hatch Seal Leak — CM/LM Tunnel

- **Trigger:** Tunnel hatch O-ring damaged during LM extraction; slow leak at 0.2 psi/min.
- **Symptoms:** Tunnel pressure drops; LM cabin O2 feeds CM leak; master caution.
- **Resolution:** Inspect hatch seal visually; if damaged, equalize tunnel and CM pressure to minimize differential; use spare O-rings from toolkit if accessible; if leak persists, isolate LM from CM using internal hatches and accept LM as uninhabitable until repair.
- **Tools:** Hatch wrench, spare O-rings, flashlight.
- **Severity:** Moderate.

### F54. Parachute Riser Tangle — Drogue

- **Trigger:** Drogue parachute risers tangle due to premature mortar firing; drogue fails to inflate.
- **Symptoms:** No drogue deployment at 24,000 ft; CM in free fall; barometric altimeter shows rapid descent.
- **Resolution:** Monitor descent rate; if drogue fails, manually fire drogue mortar backup; if backup fails, deploy main parachutes at lower altitude (10,000 ft) using barostat override; if mains also tangle, prepare for impact attenuation using couch struts and brace position.
- **Tools:** Drogue switch, barostat override, couch struts.
- **Severity:** Catastrophic.

### F55. Docking Probe Latch Failure

- **Trigger:** LM docking probe capture latches fail to engage; CSM and LM cannot hard-dock.
- **Symptoms:** Probe extends, makes contact, but no capture; LM drifts away; 12 latches fail to close.
- **Resolution:** Retract probe and re-extend; if latches still fail, use docking probe manual override to force retraction; attempt docking at lower relative velocity; if hard-dock impossible, use LM drogue and CM tunnel interface for soft-dock; transfer crew via EVA if necessary.
- **Tools:** Probe switch, manual override, tunnel hatch.
- **Severity:** High. Blocks crew transfer.

### F56. Heat Shield Separation Plane Gap

- **Trigger:** Heat shield retention bolts loosen; gap opens between heat shield and CM structure.
- **Symptoms:** Aft compartment pressure equalizes with exterior during entry; hot gas ingress risk.
- **Resolution:** Visual inspection via equipment bay window; if gap <0.25 inch, acceptable; if >0.25 inch, attempt to torque retention bolts using EVA wrench through access panel; if inaccessible, orient CM to minimize heat load on gap region using RCS.
- **Tools:** Torque wrench, access panel, window.
- **Severity:** High.

### F57. LM Window Cracking — Thermal Stress

- **Trigger:** LM forward window develops crack due to thermal cycling.
- **Symptoms:** Visible crack propagation; cabin pressure leak; structural integrity compromised.
- **Resolution:** Monitor crack length; if <2 inches, cover with thermal tape to reduce stress concentration; if >2 inches or pressure dropping, abort LM ops; transfer to CM immediately; jettison LM early if necessary.
- **Tools:** Thermal tape, pressure gauge, hatch.
- **Severity:** High. Cabin breach risk.

### F58. Antenna Deployment Motor Failure

- **Trigger:** S-band omnidirectional antenna deployment motor fails; antenna remains stowed.
- **Symptoms:** No omni coverage; comm blackouts during attitude changes.
- **Resolution:** Attempt manual antenna deployment using backup motor; if failed, use EVA to manually deploy antenna (if pre-EVA planned); if EVA impossible, rely on HGA and accept comm limitations.
- **Tools:** Motor switch, EVA tools.
- **Severity:** Moderate.

### F59. Couch Strut Shock Absorber Failure

- **Trigger:** Impact attenuation strut fails to compress; hard landing forces transmitted to crew.
- **Symptoms:** Crew experiences 25G+ impact; potential injury; couch structure cracks.
- **Resolution:** Pre-landing: verify strut pressure gauge; if low, manually pump struts using hand pump (if equipped); if pump failed, assume hard landing and assume brace position; post-landing, assess crew mobility before egress.
- **Tools:** Pressure gauge, hand pump, brace protocol.
- **Severity:** Moderate to High.

### F60. SM Sector 4 Panel Blowoff

- **Trigger:** SM sector 4 panel (EPS sector) explosively departs; wiring harness exposed.
- **Symptoms:** SM exterior visible via window; panel missing; EPS wiring arc-flashes; fuel cell 4 (if present) exposed to vacuum.
- **Resolution:** Assess damage via window and optics; if EPS wiring intact, secure loose cables using tie-wraps; if fuel cell exposed, isolate and monitor; if SPS nozzle visible and damaged, abort SPS usage; plan early CM separation.
- **Tools:** Optics, tie-wraps, isolation switches.
- **Severity:** High. Apollo 13 analog.

---

## Cryogenics & Propellant Systems (F61–F70)

### F61. O2 Tank 2 Fan Wiring Short — Cryo Stir

- **Trigger:** O2 Tank 2 fan wiring insulation damaged; spark ignites Teflon in pure O2.
- **Symptoms:** O2 Tank 2 pressure rises rapidly; tank dome temperature spikes; master alarm; seconds later, tank rupture.
- **Resolution:** **Pre-rupture:** Detect pressure rise >100 psi above nominal; immediately close O2 Tank 2 isolation valve; stop cryo stir on Tank 2; transfer load to Tank 1. **Post-rupture:** Isolate SM sector; assess CM integrity; if CM intact, abort lunar mission; use LM as lifeboat (Apollo 13 full procedure).
- **Tools:** Isolation valve, LM hatch, power down checklist.
- **Severity:** Catastrophic. Apollo 13 exact historical.

### F62. H2 Tank Pressure Sensor Failure — False High

- **Trigger:** H2 tank pressure sensor stuck; reads 300 psi regardless of actual.
- **Symptoms:** Ground and crew believe H2 overpressure; automatic venting initiated; actual H2 may be low or high.
- **Resolution:** Cross-check H2 tank temperature and fuel cell H2 flow rate; if flow rate inconsistent with pressure reading, sensor failed; switch to backup pressure sensor; if no backup, calculate H2 mass from temperature and known tank volume; manually control H2 tank heaters.
- **Tools:** Temp gauge, flowmeter, heater switches, calculator.
- **Severity:** Moderate.

### F63. Cryo Tank Heater Stuck On

- **Trigger:** Cryo tank heater thermostat fails closed; continuous heating.
- **Symptoms:** O2/H2 tank pressure climbs above 900 psi; relief valve opens; venting detected.
- **Resolution:** Monitor tank pressure; if >880 psi, manually open heater circuit breaker; allow tank to cool passively; if pressure continues rising, open tank vent valve briefly to relieve pressure; calculate boil-off loss and adjust mission duration.
- **Tools:** Circuit breaker, vent valve, pressure gauge.
- **Severity:** High. Tank rupture risk.

### F64. SPS Propellant Line Freeze

- **Trigger:** SPS propellant lines exposed to cold soak; oxidizer viscosity increases.
- **Symptoms:** SPS burn ignition delayed; chamber pressure builds slowly; thrust asymmetric.
- **Resolution:** Prior to SPS burn, perform propellant line conditioning by opening tank pressurization and recirculation valves for 30 seconds; monitor line temperatures; if still cold, delay burn and use RCS attitude control to orient SM toward Sun for thermal soak.
- **Tools:** Pressurization valves, temp gauge, RCS controller.
- **Severity:** Moderate.

### F65. RCS Propellant Isolation Valve Slow Leak

- **Trigger:** RCS isolation valve seat erodes; helium bleed into propellant tank.
- **Symptoms:** RCS propellant tank pressure rises slowly; thruster performance degrades; ullage compression.
- **Resolution:** Monitor RCS tank pressure vs. temperature; if pressure rise exceeds thermal expectation, isolate affected quad; if helium contamination confirmed, vent affected quad propellants to space using thruster firing; rely on remaining quads.
- **Tools:** Isolation switches, pressure gauge, thruster controller.
- **Severity:** Moderate.

### F66. LM Descent Propellant Tank Bulge

- **Trigger:** Descent propellant tank overpressurizes; tank wall bulges visibly.
- **Symptoms:** DPS tank pressure >290 psi; tank wall visible deformation; potential rupture.
- **Resolution:** Immediately vent DPS tank pressure using manual vent valves; if vent stuck, fire DPS at 10% throttle to consume propellant and reduce pressure; if tank ruptures, abort descent immediately; use ascent stage for emergency separation.
- **Tools:** Vent valves, DPS throttle, abort stage button.
- **Severity:** Critical.

### F67. Fuel Cell Reactant Valve Stuck Open

- **Trigger:** Fuel cell O2 reactant valve fails open; O2 consumption uncontrolled.
- **Symptoms:** O2 tank pressure drops rapidly; fuel cell voltage high; water production excessive; condenser overflow.
- **Resolution:** Monitor O2 flow rate; if >2x nominal, manually close reactant valve using override; if override fails, isolate fuel cell; calculate remaining O2 with fuel cell offline; prioritize O2 for crew breathing over fuel cells if necessary (batteries can sustain entry).
- **Tools:** Override switch, isolation switch, flowmeter.
- **Severity:** High.

### F68. SPS Helium Check Valve Reverse Flow

- **Trigger:** SPS helium check valve fails; propellant backflows into helium system.
- **Symptoms:** Helium regulator contaminated; SPS ignition rough or fails; helium pressure drops then recovers anomalously.
- **Resolution:** Detect via helium pressure anomaly; if backflow suspected, do not attempt SPS ignition; use RCS for all propulsive maneuvers; if SPS already contaminated, engine is unusable; plan free-return or alternate trajectory.
- **Tools:** Pressure gauge, RCS controller, trajectory charts.
- **Severity:** High. SPS loss.

### F69. Cryo Tank Quantity Gauge Failure

- **Trigger:** Cryo tank capacitance probe fails; quantity reading frozen.
- **Symptoms:** Quantity gauge shows constant value despite known consumption; fuel cell reactant flow continues.
- **Resolution:** Cross-check quantity with pressure and temperature using PVT (Pressure-Volume-Temperature) calculations; manually log consumption rates; if O2 quantity truly unknown, assume worst-case and abort mission early to preserve margins.
- **Tools:** Pressure gauge, temp gauge, slide rule, logbook.
- **Severity:** Moderate.

### F70. LM APS Propellant Interstage Leak

- **Trigger:** Ascent/descent interstage seal fails; APS propellant leaks into descent stage.
- **Symptoms:** APS tank pressure drops; descent stage odor; potential fire hazard.
- **Resolution:** If leak detected pre-separation, abort LM ascent; if post-landing, do not initiate ascent; crew remains on surface or transfers to CM via EVA; if ascent must occur, accept degraded APS performance and reduced orbit.
- **Tools:** Pressure gauge, odor detection, hatch.
- **Severity:** Critical.

---

## Crew Interface & Procedural Edge Cases (F71–F80)

### F71. Abort Handle Shear Pin — Accidental Pull

- **Trigger:** Crew snags LM abort handle; shear pin prevents activation but handle bent.
- **Symptoms:** Abort handle misaligned; abort stage circuit shows continuity; potential accidental abort.
- **Resolution:** Inspect handle and shear pin; if pin sheared, abort stage is armed; secure handle with tape; if handle stuck in abort position, disconnect abort stage electrical connector to prevent inadvertent firing; proceed with manual staging only.
- **Tools:** Tape, connector wrench, flashlight.
- **Severity:** High. Apollo 14 historical analog.

### F72. DSKY Keyboard Switch Bounce

- **Trigger:** DSKY PRO key switch bounces; AGC receives multiple enters.
- **Symptoms:** Program accepts unintended data; NOUN 68 shows wrong values; burn parameters corrupted.
- **Resolution:** Verify all displayed values before PROceed; if corruption detected, reload correct data via VERB 21 NOUN 01; if keyboard electrically failed, use DEDA (LM) or ground uplink for data entry.
- **Tools:** DSKY, data pads, voice comm.
- **Severity:** Moderate.

### F73. Checklist Page Missing — Critical Procedure

- **Trigger:** Checklist page torn/loose; crew lacks abort procedure reference.
- **Symptoms:** Crew cannot recall exact circuit breaker sequence for abort; time lost.
- **Resolution:** Use onboard data files (if intact); query ground via voice for readback; if comm lost, rely on memorized training and schematics posted on cabin walls; reconstruct procedure logically from system interdependencies.
- **Tools:** Data files, voice comm, wall schematics.
- **Severity:** Moderate. Training-dependent.

### F74. Penlight Battery Failure — Dark Cabin

- **Trigger:** All penlights fail; cabin dark during power-down.
- **Symptoms:** Cannot read gauges or circuit breakers; procedures impossible in darkness.
- **Resolution:** Use cabin emergency floodlight (if battery backup); use glow-in-the-dark tape on critical switches; use Sun illumination through window if in daylight; feel for switches by shape and position memorization.
- **Tools:** Emergency light, tactile memory.
- **Severity:** Low to Moderate.

### F75. Loose Object Shorts Panel

- **Trigger:** Metal tool or clip falls behind panel; shorts DC bus.
- **Symptoms:** Bus trips; smoke; localized burn smell; system failures downstream.
- **Resolution:** Identify tripped breaker; do not reset until object removed; use magnetic retrieval tool or gloves to extract object; inspect wiring for damage; if wiring damaged, isolate circuit and use backup systems.
- **Tools:** Magnetic retrieval tool, gloves, flashlight.
- **Severity:** Moderate.

### F76. Caution/Warning System Failure — Master Alarm Stuck

- **Trigger:** Master alarm relay sticks closed; continuous alarm tone.
- **Symptoms:** Crew cannot distinguish new faults; alarm fatigue; actual new faults masked.
- **Resolution:** Identify stuck relay in caution/warning electronics; pull master alarm circuit breaker to silence; monitor individual caution lights visually; if C/W system completely failed, manually monitor all critical gauges every 2 minutes.
- **Tools:** Circuit breaker, visual scan pattern.
- **Severity:** Moderate. Masking risk.

### F77. Window Fogging — Anti-Fog Failure

- **Trigger:** Window anti-fog coating degrades; condensation blocks vision.
- **Symptoms:** Cannot see Earth, Moon, or horizon for navigation; landing impossible.
- **Resolution:** Wipe with clean cloth and anti-fog solution; if solution depleted, use saliva (historical technique); if still fogged, use optics (sextant/AOT) for navigation; for landing, use radar altimeter and DSKY alone.
- **Tools:** Cloth, solution, optics, radar.
- **Severity:** Moderate during landing.

### F78. Flight Plan Ink Smear — Time Critical

- **Trigger:** Water condensation smears flight plan; GET (Ground Elapsed Time) marks illegible.
- **Symptoms:** Missed maneuver times; crew confused about schedule.
- **Resolution:** Use AGC MET as master clock; reconstruct timeline from memory and DSKY programs; if ground contact available, request current GET and next maneuver time; write new timeline on blank paper.
- **Tools:** DSKY, voice comm, blank paper.
- **Severity:** Low to Moderate.

### F79. Urine Dump Nozzle Freeze

- **Trigger:** Urine dump nozzle freezes in space; backup pressure builds.
- **Symptoms:** Urine collection bag full; cannot empty; hygiene compromise.
- **Resolution:** Heat nozzle using cabin air or small heater; if still frozen, store bags and use backup collection method; if pressure builds dangerously, vent to space via alternate valve.
- **Tools:** Heater, alternate valve, collection bags.
- **Severity:** Low. Immersive detail.

### F80. AGC Program Alarm Cascade — Multiple Simultaneous

- **Trigger:** Multiple system faults trigger concurrent AGC alarms (1201 + 1210 + 1301).
- **Symptoms:** DSKY flashing multiple codes; AGC overloaded; computer enters standby; all automation lost.
- **Resolution:** Prioritize alarms by mission phase (descent = landing radar first, entry = G&C first); acknowledge alarms via PRO key; if AGC enters standby, perform full restart (V36E); reload critical programs from backup pad; if restart fails, fly manually using backup instruments and ground voice commands.
- **Tools:** DSKY, backup pads, voice comm, manual instruments.
- **Severity:** Critical. Total automation loss.

---

## Implementation Notes for Game Design

### Diagnostic Loop Structure

Each scenario should present **Symptoms First** (flashing lights, gauge readings, sounds, vibrations), forcing the player to:

1. **Observe** — Read instruments, listen to sounds, note DSKY codes.
2. **Isolate** — Use schematics to trace which system is root vs. symptom.
3. **Decide** — Consult checklists or training memory; wrong decision escalates fault.
4. **Act** — Flip switches, pull breakers, use tools, reconfigure software.
5. **Verify** — Confirm resolution via instrument feedback; partial fixes create lingering degradation.

### Tool Inventory for Player

| Tool | Use |
|------|-----|
| Multimeter (DC/AC volts, ohms) | Electrical diagnostics |
| Circuit breaker puller | Safe breaker manipulation |
| Torque wrench (cannon plugs, bolts) | Mechanical fastening |
| Sextant / AOT | Optical navigation |
| Slide rule / mission calculator | Orbital mechanics, power budgets |
| Schematic panels (in-cabin reference) | System tracing |
| DSKY cheat sheet / alarm code lookup | AGC operation |
| Emergency patch kit (hull, hoses) | Leak repair |
| Wire cutters / jumper cables | Electrical repair |
| Flashlight / penlight | Dark cabin ops |
| Magnetic retrieval tool | Foreign object removal |
| Thermal tape | Crack/plug temporary repair |
| Spare O-rings, LiOH canisters, hoses | Consumable replacement |

### Failure Cascade Model

Design scenarios so that unresolved faults can trigger cascading failures. Example chain:

> **F01** (Bus Undervolt) → **F07** (AGC Restart) → **F12** (IMU Gimbal Lock) if platform alignment is lost.

This creates emergent multi-system crises requiring true systems thinking rather than rote checklist following.

### Educational Scaffolding

Each scenario completion unlocks an **Engineering Note** explaining the real Apollo physics, electronics, or orbital mechanics involved — turning each failure into a micro-lesson on spacecraft engineering.

### Historical Analog Reference

| Fault | Historical Mission |
|-------|--------------------|
| F02 | Apollo 13 (fuel cell cascade) |
| F09 | Apollo 13 (cryo fan) |
| F11 | Apollo 11 (1201/1202 alarm) |
| F32 | Apollo 13 (LiOH canister adapter) |
| F61 | Apollo 13 (O2 tank explosion) |
| F60 | Apollo 13 (SM panel blowoff) |
| F71 | Apollo 14 (abort handle) |

### Severity Distribution

| Severity | Count | Faults |
|----------|-------|--------|
| Low | 4 | F36, F46, F49, F79 |
| Low–Moderate | 4 | F39, F74, F78, F36 |
| Moderate | 26 | F01, F03, F05, F13, F15, F22, F28, F33, F34, F40, F42, F43, F44, F45, F48, F50, F53, F58, F62, F64, F65, F69, F72, F73, F75, F76, F77 |
| High | 28 | F02, F06, F09, F18, F21, F23, F25, F27, F30, F31, F35, F37, F41, F47, F51, F52, F55, F56, F57, F59, F60, F63, F67, F68, F71 |
| Critical | 12 | F04, F07, F08, F16, F17, F19, F24, F29, F51, F66, F70, F80 |
| Catastrophic | 6 | F10, F20, F26, F38, F54, F61 |

### Category Distribution

| Category | Range | Count |
|----------|-------|-------|
| Electrical Power & Distribution | F01–F10 | 10 |
| Guidance, Navigation & Control / AGC | F11–F20 | 10 |
| Propulsion — SPS, RCS, APS | F21–F30 | 10 |
| Environmental Control & Life Support | F31–F40 | 10 |
| Communications & Instrumentation | F41–F50 | 10 |
| Structural & Mechanical | F51–F60 | 10 |
| Cryogenics & Propellant Systems | F61–F70 | 10 |
| Crew Interface & Procedural Edge Cases | F71–F80 | 10 |
