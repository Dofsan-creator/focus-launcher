//! Sons et musique d'ambiance synthétisés : aucun fichier audio à livrer ni à licencier.
//! Un seul contexte audio pour toute l'application (WebView2 en limite le nombre).
//!
//! La musique est paisible par construction : une gamme sans dissonance (ré majeur
//! pentatonique), des accords tenus qui se fondent l'un dans l'autre, quelques notes
//! de cloche espacées, et une réverbération longue. Rien ne se répète à l'identique.

use leptos::prelude::{set_interval_with_handle, set_timeout, IntervalHandle};
use std::cell::RefCell;
use std::time::Duration;
use web_sys::{AudioContext, AudioNode, BiquadFilterType, GainNode, OscillatorType};

/// Son de l'interface.
#[derive(Clone, Copy)]
pub enum Tone {
    /// Déplacement.
    Move,
    /// Validation.
    Confirm,
    /// Retour.
    Back,
}

/// Accords tenus, du grave à l'aigu : ré, sol, si mineur, la suspendu. Chaque note
/// appartient à la gamme des cloches, donc rien ne frotte.
const CHORDS: [[f32; 4]; 4] = [
    [146.83, 220.00, 329.63, 369.99],
    [98.00, 146.83, 246.94, 369.99],
    [123.47, 185.00, 220.00, 293.66],
    [110.00, 164.81, 246.94, 329.63],
];
/// Gamme des cloches : ré majeur pentatonique sur deux octaves.
const BELLS: [f32; 9] = [293.66, 329.63, 369.99, 440.00, 493.88, 587.33, 659.25, 739.99, 880.00];
/// Durée d'un accord, de sa montée et de sa descente, en secondes.
const CHORD_EVERY: f64 = 20.0;
const CHORD_RISE: f64 = 6.0;
const CHORD_FALL: f64 = 9.0;
/// Niveau général de la musique : un fond, jamais au premier plan.
const MUSIC_LEVEL: f32 = 0.1;

struct Ambient {
    master: GainNode,
    /// Entrée commune des voix, avant le filtre et la réverbération.
    bus: GainNode,
    next_chord: f64,
    next_bell: f64,
    chord: usize,
    last_bell: usize,
    timer: Option<IntervalHandle>,
}

thread_local! {
    static CONTEXT: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
    static AMBIENT: RefCell<Option<Ambient>> = const { RefCell::new(None) };
}

fn with_context<T>(work: impl FnOnce(&AudioContext) -> Option<T>) -> Option<T> {
    CONTEXT.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            *slot = AudioContext::new().ok();
        }
        slot.as_ref().and_then(work)
    })
}

/// Joue un son très court, dans la même gamme que la musique. Une absence d'audio
/// n'est pas une erreur.
pub fn play(tone: Tone) {
    let (frequency, length, level) = match tone {
        Tone::Move => (587.33, 0.07, 0.022),
        Tone::Confirm => (880.0, 0.14, 0.034),
        Tone::Back => (440.0, 0.11, 0.026),
    };
    let _ = with_context(|context| {
        let oscillator = context.create_oscillator().ok()?;
        let gain = context.create_gain().ok()?;
        oscillator.frequency().set_value(frequency);
        let now = context.current_time();
        // Montée de quelques millisecondes : pas de claquement à l'attaque.
        gain.gain().set_value_at_time(0.0001, now).ok()?;
        gain.gain().linear_ramp_to_value_at_time(level, now + 0.008).ok()?;
        gain.gain()
            .exponential_ramp_to_value_at_time(0.0001, now + length)
            .ok()?;
        oscillator.connect_with_audio_node(&gain).ok()?;
        gain.connect_with_audio_node(&context.destination()).ok()?;
        oscillator.start().ok()?;
        oscillator.stop_with_when(now + length + 0.02).ok()?;
        Some(())
    });
}

fn random() -> f64 {
    js_sys::Math::random()
}

/// Réverbération : un bruit qui s'éteint en quatre secondes, différent à chaque oreille.
fn reverb(context: &AudioContext) -> Option<web_sys::ConvolverNode> {
    let rate = context.sample_rate();
    let length = (rate * 4.0) as usize;
    let buffer = context.create_buffer(2, length as u32, rate).ok()?;
    for channel in 0..2 {
        let samples: Vec<f32> = (0..length)
            .map(|index| {
                let fade = 1.0 - index as f64 / length as f64;
                ((random() * 2.0 - 1.0) * fade * fade * fade) as f32
            })
            .collect();
        buffer.copy_to_channel(&samples, channel).ok()?;
    }
    let convolver = context.create_convolver().ok()?;
    convolver.set_buffer(Some(&buffer));
    Some(convolver)
}

/// Une voix : un oscillateur et son enveloppe, branchés sur `bus`. Elle monte en
/// `rise`, tient jusqu'à `hold`, s'éteint en `fall`, puis se libère toute seule.
#[allow(clippy::too_many_arguments)]
fn voice(
    context: &AudioContext,
    bus: &AudioNode,
    shape: OscillatorType,
    frequency: f32,
    level: f32,
    start: f64,
    rise: f64,
    hold: f64,
    fall: f64,
) -> Option<()> {
    let oscillator = context.create_oscillator().ok()?;
    oscillator.set_type(shape);
    oscillator.frequency().set_value(frequency);
    let gain = context.create_gain().ok()?;
    gain.gain().set_value_at_time(0.0001, start).ok()?;
    gain.gain().linear_ramp_to_value_at_time(level, start + rise).ok()?;
    gain.gain().set_value_at_time(level, start + rise + hold).ok()?;
    gain.gain()
        .exponential_ramp_to_value_at_time(0.0001, start + rise + hold + fall)
        .ok()?;
    oscillator.connect_with_audio_node(&gain).ok()?;
    gain.connect_with_audio_node(bus).ok()?;
    oscillator.start_with_when(start).ok()?;
    oscillator.stop_with_when(start + rise + hold + fall + 0.1).ok()?;
    Some(())
}

/// Accord tenu : chaque note est doublée d'une voix à peine désaccordée, pour la chaleur.
fn chord(context: &AudioContext, bus: &AudioNode, notes: [f32; 4], start: f64) {
    let hold = CHORD_EVERY - CHORD_RISE;
    for (index, frequency) in notes.into_iter().enumerate() {
        // Les notes aiguës sont plus discrètes que la basse.
        let level = 0.2 - index as f32 * 0.03;
        let _ = voice(context, bus, OscillatorType::Sine, frequency, level, start, CHORD_RISE, hold, CHORD_FALL);
        let _ = voice(context, bus, OscillatorType::Triangle, frequency * 1.003, level * 0.3, start, CHORD_RISE, hold, CHORD_FALL);
    }
}

/// Note de cloche : attaque douce, longue extinction, un harmonique léger.
fn bell(context: &AudioContext, bus: &AudioNode, frequency: f32, level: f32, start: f64) {
    let _ = voice(context, bus, OscillatorType::Sine, frequency, level, start, 0.04, 0.0, 5.5);
    let _ = voice(context, bus, OscillatorType::Sine, frequency * 2.0, level * 0.18, start, 0.02, 0.0, 2.2);
}

/// Programme ce qui doit sonner dans les prochaines secondes. Appelé chaque seconde.
fn tick() {
    AMBIENT.with(|cell| {
        let mut slot = cell.borrow_mut();
        let Some(ambient) = slot.as_mut() else { return };
        let _ = with_context(|context| {
            let now = context.current_time();
            let bus: &AudioNode = &ambient.bus;
            // Après une longue pause (fenêtre cachée pendant un jeu), on repart de maintenant.
            if ambient.next_chord < now - CHORD_EVERY {
                ambient.next_chord = now + 0.2;
                ambient.next_bell = now + 4.0;
            }
            while ambient.next_chord < now + 2.0 {
                chord(context, bus, CHORDS[ambient.chord % CHORDS.len()], ambient.next_chord.max(now));
                ambient.chord += 1;
                ambient.next_chord += CHORD_EVERY;
            }
            while ambient.next_bell < now + 1.5 {
                let start = ambient.next_bell.max(now);
                // Jamais deux fois la même note, jamais un grand saut.
                let step = 1 + (random() * 3.0) as usize;
                let up = random() < 0.5;
                let index = if up { ambient.last_bell + step } else { ambient.last_bell + BELLS.len() - step } % BELLS.len();
                ambient.last_bell = index;
                bell(context, bus, BELLS[index], 0.11, start);
                // Parfois une seconde note, plus haute et plus légère, lui répond.
                if random() < 0.3 {
                    bell(context, bus, BELLS[(index + 2) % BELLS.len()], 0.06, start + 0.42);
                }
                ambient.next_bell = start + 3.0 + random() * 5.0;
            }
            Some(())
        });
    });
}

/// Monte la chaîne : voix, filtre doux, puis son direct et réverbération.
fn start_ambient(context: &AudioContext) -> Option<Ambient> {
    let now = context.current_time();
    let master = context.create_gain().ok()?;
    master.gain().set_value_at_time(0.0001, now).ok()?;
    master.gain().linear_ramp_to_value_at_time(MUSIC_LEVEL, now + 5.0).ok()?;
    master.connect_with_audio_node(&context.destination()).ok()?;
    let bus = context.create_gain().ok()?;
    let soften = context.create_biquad_filter().ok()?;
    soften.set_type(BiquadFilterType::Lowpass);
    soften.frequency().set_value(1900.0);
    bus.connect_with_audio_node(&soften).ok()?;
    let dry = context.create_gain().ok()?;
    dry.gain().set_value(0.65);
    soften.connect_with_audio_node(&dry).ok()?;
    dry.connect_with_audio_node(&master).ok()?;
    if let Some(space) = reverb(context) {
        let wet = context.create_gain().ok()?;
        wet.gain().set_value(0.5);
        soften.connect_with_audio_node(&space).ok()?;
        space.connect_with_audio_node(&wet).ok()?;
        wet.connect_with_audio_node(&master).ok()?;
    }
    Some(Ambient {
        master,
        bus,
        next_chord: now + 0.1,
        next_bell: now + 5.0,
        chord: 0,
        last_bell: 3,
        timer: None,
    })
}

/// Allume ou coupe la musique d'ambiance, en fondu.
pub fn set_music(on: bool) {
    let running = AMBIENT.with(|cell| cell.borrow().is_some());
    match (on, running) {
        (true, false) => {
            let Some(mut ambient) = with_context(start_ambient) else { return };
            ambient.timer = set_interval_with_handle(tick, Duration::from_secs(1)).ok();
            AMBIENT.with(|cell| *cell.borrow_mut() = Some(ambient));
            tick();
        }
        (false, true) => {
            let Some(old) = AMBIENT.with(|cell| cell.borrow_mut().take()) else { return };
            if let Some(timer) = old.timer {
                timer.clear();
            }
            let _ = with_context(|context| {
                let now = context.current_time();
                old.master.gain().cancel_scheduled_values(now).ok()?;
                old.master.gain().set_value_at_time(old.master.gain().value(), now).ok()?;
                old.master.gain().linear_ramp_to_value_at_time(0.0, now + 1.2).ok()?;
                Some(())
            });
            // Les voix déjà programmées s'arrêtent seules ; la sortie est débranchée
            // une fois le fondu fini.
            set_timeout(
                move || {
                    let _ = old.master.disconnect();
                },
                Duration::from_millis(1500),
            );
        }
        _ => {}
    }
}

/// Met tout l'audio en pause quand la fenêtre est cachée (pendant un jeu).
pub fn set_suspended(hidden: bool) {
    CONTEXT.with(|cell| {
        if let Some(context) = cell.borrow().as_ref() {
            let _ = if hidden { context.suspend() } else { context.resume() };
        }
    });
}
