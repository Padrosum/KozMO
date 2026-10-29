use crate::plugins::GameRuntimeState;
use bevy::{
    audio::{AddAudioSource, AudioPlayer, Decodable, PlaybackSettings, Source, Volume},
    prelude::*,
    reflect::TypePath,
};
use std::{
    f32::consts::TAU,
    num::{NonZeroU16, NonZeroU32},
    sync::Arc,
    time::Duration,
};

pub struct CosmicAudioPlugin;

impl Plugin for CosmicAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_source::<CosmicSynthAsset>()
            .add_systems(Startup, setup_procedural_synth_bank)
            .add_systems(Update, play_pending_cosmic_sfx);
    }
}

/// Procedural synthesizer sound effect categories for KozMO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CosmicSfxKind {
    /// Subtle crystal tap for UI buttons and tab switches.
    UiClick,
    /// Warm harmonic chord when selecting a universe on the cosmic map.
    UniverseSelect,
    /// Ascending pentatonic chime when spreading prophecy or enlightening mortals.
    ProphecyChime,
    /// Deep resonance sweep when harvesting souls or extracting cosmic essence.
    SoulHarvest,
    /// Radiant major-9th celestial chord for miracles, life seeding, and power unlocks.
    MiracleChord,
    /// Dramatic sub-bass & dissonant strike for smiting rivals, holy wars, and sealing.
    DivineStrike,
    /// Dimensional harmonic sweep for gateways, scouting, and ambient resonance.
    GatewayPulse,
    /// Ethereal temple bell for random cosmic events, dilemmas, and tutorial milestones.
    CosmicEventAlert,
    /// Grand ascending fanfare when achieving one of the three Victory paths.
    VictoryFanfare,
}

impl CosmicSfxKind {
    pub const ALL: [Self; 9] = [
        Self::UiClick,
        Self::UniverseSelect,
        Self::ProphecyChime,
        Self::SoulHarvest,
        Self::MiracleChord,
        Self::DivineStrike,
        Self::GatewayPulse,
        Self::CosmicEventAlert,
        Self::VictoryFanfare,
    ];
}

/// In-memory procedural PCM waveform asset decoded directly by `bevy_audio`.
#[derive(Asset, TypePath, Clone)]
pub struct CosmicSynthAsset {
    pub samples: Arc<[f32]>,
    pub sample_rate: u32,
}

pub struct CosmicSynthDecoder {
    samples: Arc<[f32]>,
    cursor: usize,
    sample_rate: u32,
}

impl Iterator for CosmicSynthDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.samples.len() {
            let val = self.samples[self.cursor];
            self.cursor += 1;
            Some(val)
        } else {
            None
        }
    }
}

impl Source for CosmicSynthDecoder {
    fn current_span_len(&self) -> Option<usize> {
        Some(self.samples.len().saturating_sub(self.cursor))
    }

    fn channels(&self) -> NonZeroU16 {
        NonZeroU16::MIN
    }

    fn sample_rate(&self) -> NonZeroU32 {
        NonZeroU32::new(self.sample_rate).unwrap_or(NonZeroU32::MIN)
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f64(
            self.samples.len() as f64 / self.sample_rate as f64,
        ))
    }
}

impl Decodable for CosmicSynthAsset {
    type Decoder = CosmicSynthDecoder;

    fn decoder(&self) -> Self::Decoder {
        CosmicSynthDecoder {
            samples: Arc::clone(&self.samples),
            cursor: 0,
            sample_rate: self.sample_rate,
        }
    }
}

#[derive(Resource, Default)]
pub struct CosmicSynthBank {
    pub handles: Vec<(CosmicSfxKind, Handle<CosmicSynthAsset>)>,
}

impl CosmicSynthBank {
    pub fn get(&self, kind: CosmicSfxKind) -> Option<Handle<CosmicSynthAsset>> {
        self.handles
            .iter()
            .find(|(k, _)| *k == kind)
            .map(|(_, h)| h.clone())
    }
}

const SAMPLE_RATE: u32 = 44_100;

fn setup_procedural_synth_bank(
    mut commands: Commands,
    mut synth_assets: ResMut<Assets<CosmicSynthAsset>>,
) {
    let mut bank = CosmicSynthBank::default();
    for kind in CosmicSfxKind::ALL {
        let asset = synthesize_sfx(kind);
        let handle = synth_assets.add(asset);
        bank.handles.push((kind, handle));
    }
    commands.insert_resource(bank);
}

fn play_pending_cosmic_sfx(
    mut commands: Commands,
    mut runtime: ResMut<GameRuntimeState>,
    bank: Option<Res<CosmicSynthBank>>,
) {
    if !runtime.audio_enabled {
        runtime.pending_sfx.clear();
        return;
    }
    let Some(bank) = bank else {
        runtime.pending_sfx.clear();
        return;
    };

    // Deduplicate identical SFX triggered on the same frame so audio never clips
    let mut played_this_frame: Vec<CosmicSfxKind> = Vec::new();
    let vol = runtime.audio_volume.clamp(0.0, 1.0);

    for kind in runtime.pending_sfx.drain(..) {
        if played_this_frame.contains(&kind) {
            continue;
        }
        played_this_frame.push(kind);
        if let Some(handle) = bank.get(kind) {
            commands.spawn((
                AudioPlayer(handle),
                PlaybackSettings::DESPAWN.with_volume(Volume::Linear(vol)),
            ));
        }
    }
}

/// Synthesizes a multi-harmonic cosmic waveform for each `CosmicSfxKind`.
pub fn synthesize_sfx(kind: CosmicSfxKind) -> CosmicSynthAsset {
    let samples: Vec<f32> = match kind {
        CosmicSfxKind::UiClick => {
            // Crisp obsidian/amber chime (55 ms)
            synth_notes(
                &[(528.0, 0.0, 0.055, 0.22), (792.0, 0.008, 0.045, 0.14)],
                0.06,
            )
        }
        CosmicSfxKind::UniverseSelect => {
            // Warm celestial fifth + octave chord (180 ms)
            synth_notes(
                &[
                    (432.0, 0.0, 0.17, 0.24),
                    (648.0, 0.015, 0.16, 0.18),
                    (864.0, 0.03, 0.14, 0.12),
                ],
                0.19,
            )
        }
        CosmicSfxKind::ProphecyChime => {
            // Ascending golden pentatonic arpeggio (360 ms)
            synth_notes(
                &[
                    (440.0, 0.00, 0.16, 0.22),
                    (554.37, 0.06, 0.16, 0.20),
                    (659.25, 0.12, 0.18, 0.20),
                    (880.0, 0.18, 0.17, 0.22),
                ],
                0.36,
            )
        }
        CosmicSfxKind::SoulHarvest => {
            // Mystical frequency glide downward with harmonic shimmer (320 ms)
            synth_glide(396.0, 220.0, 0.32, 0.28, true)
        }
        CosmicSfxKind::MiracleChord => {
            // Radiant major-9th choir chord (460 ms)
            synth_notes(
                &[
                    (523.25, 0.00, 0.42, 0.20),
                    (659.25, 0.03, 0.40, 0.18),
                    (783.99, 0.06, 0.38, 0.17),
                    (987.77, 0.09, 0.35, 0.14),
                    (1174.66, 0.12, 0.32, 0.12),
                ],
                0.46,
            )
        }
        CosmicSfxKind::DivineStrike => {
            // Dramatic cosmic thunder drop + dissonant overtone (360 ms)
            let mut base = synth_glide(185.0, 65.0, 0.36, 0.34, false);
            let overtone = synth_notes(
                &[(277.18, 0.0, 0.24, 0.18), (196.0, 0.02, 0.28, 0.20)],
                0.36,
            );
            for (b, o) in base.iter_mut().zip(overtone.iter()) {
                *b = (*b + *o).clamp(-0.95, 0.95);
            }
            base
        }
        CosmicSfxKind::GatewayPulse => {
            // Dimensional upward sweep (280 ms)
            synth_glide(293.66, 587.33, 0.28, 0.26, true)
        }
        CosmicSfxKind::CosmicEventAlert => {
            // Two-tone astral temple bell (340 ms)
            synth_notes(
                &[(587.33, 0.00, 0.18, 0.24), (880.00, 0.11, 0.22, 0.25)],
                0.34,
            )
        }
        CosmicSfxKind::VictoryFanfare => {
            // Grand 5-note ascending cosmic ascension fanfare (750 ms)
            synth_notes(
                &[
                    (523.25, 0.00, 0.22, 0.22),
                    (659.25, 0.11, 0.22, 0.22),
                    (783.99, 0.22, 0.24, 0.22),
                    (1046.50, 0.34, 0.38, 0.24),
                    (1318.51, 0.44, 0.30, 0.20),
                ],
                0.75,
            )
        }
    };

    CosmicSynthAsset {
        samples: Arc::from(samples),
        sample_rate: SAMPLE_RATE,
    }
}

/// Synthesizes polyphonic notes `(freq_hz, start_sec, duration_sec, amplitude)` with warm harmonics.
fn synth_notes(notes: &[(f32, f32, f32, f32)], total_secs: f32) -> Vec<f32> {
    let total_samples = (total_secs * SAMPLE_RATE as f32) as usize;
    let mut out = vec![0.0f32; total_samples];

    for &(freq, start_s, dur_s, amp) in notes {
        let start_idx = (start_s * SAMPLE_RATE as f32) as usize;
        let len_samples = (dur_s * SAMPLE_RATE as f32) as usize;
        let attack = (0.012 * SAMPLE_RATE as f32).max(1.0);

        for i in 0..len_samples {
            let idx = start_idx + i;
            if idx >= total_samples {
                break;
            }
            let t = i as f32 / SAMPLE_RATE as f32;
            let progress = (i as f32 / len_samples.max(1) as f32).clamp(0.0, 1.0);
            let env_in = (i as f32 / attack).clamp(0.0, 1.0);
            let env_out = (1.0 - progress).powf(1.65);
            let env = env_in * env_out;

            // Subtle vibrato + fundamental + 2nd & 3rd soft harmonics
            let vib = (TAU * 5.5 * t).sin() * 0.003;
            let phase = TAU * freq * (1.0 + vib) * t;
            let wave = phase.sin() * 0.72 + (phase * 2.0).sin() * 0.20 + (phase * 3.0).sin() * 0.08;
            out[idx] = (out[idx] + wave * amp * env).clamp(-0.95, 0.95);
        }
    }

    out
}

/// Synthesizes a smooth frequency glide from `f_start` to `f_end` with optional shimmer harmonic.
fn synth_glide(f_start: f32, f_end: f32, duration_secs: f32, amp: f32, shimmer: bool) -> Vec<f32> {
    let total_samples = (duration_secs * SAMPLE_RATE as f32) as usize;
    let mut out = vec![0.0f32; total_samples];
    let attack = (0.015 * SAMPLE_RATE as f32).max(1.0);
    let mut phase = 0.0f32;

    for (i, sample) in out.iter_mut().enumerate() {
        let progress = i as f32 / total_samples.max(1) as f32;
        let freq = f_start + (f_end - f_start) * progress;
        phase += TAU * freq / SAMPLE_RATE as f32;

        let env_in = (i as f32 / attack).clamp(0.0, 1.0);
        let env_out = (1.0 - progress).powf(1.4);
        let env = env_in * env_out;

        let mut wave = phase.sin() * 0.78 + (phase * 1.5).sin() * 0.22;
        if shimmer {
            wave += (phase * 2.0).sin() * 0.15;
        }
        *sample = (wave * amp * env).clamp(-0.95, 0.95);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_procedural_cosmic_sfx_waveforms_are_valid_and_clamped() {
        for kind in CosmicSfxKind::ALL {
            let asset = synthesize_sfx(kind);
            assert_eq!(asset.sample_rate, SAMPLE_RATE);
            assert!(
                asset.samples.len() >= 2000,
                "Expected non-trivial waveform for {:?}",
                kind
            );
            for &s in asset.samples.iter() {
                assert!(
                    s.is_finite() && (-1.0..=1.0).contains(&s),
                    "Invalid sample {s} in {:?}",
                    kind
                );
            }
            let mut dec = asset.decoder();
            assert_eq!(dec.channels().get(), 1);
            assert_eq!(dec.sample_rate().get(), SAMPLE_RATE);
            assert!(dec.total_duration().is_some());
            assert_eq!(dec.next(), Some(asset.samples[0]));
        }
    }
}
