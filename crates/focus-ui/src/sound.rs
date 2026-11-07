//! Sons de navigation synthétisés : aucun fichier audio à livrer ni à licencier.

use web_sys::AudioContext;

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

/// Joue un son très court. Une absence d'audio n'est pas une erreur.
pub fn play(tone: Tone) {
    let (frequency, length) = match tone {
        Tone::Move => (520.0, 0.04),
        Tone::Confirm => (760.0, 0.09),
        Tone::Back => (340.0, 0.07),
    };
    let _ = try_play(frequency, length);
}

fn try_play(frequency: f32, length: f64) -> Option<()> {
    let context = AudioContext::new().ok()?;
    let oscillator = context.create_oscillator().ok()?;
    let gain = context.create_gain().ok()?;
    oscillator.frequency().set_value(frequency);
    let now = context.current_time();
    gain.gain().set_value_at_time(0.05, now).ok()?;
    gain.gain().exponential_ramp_to_value_at_time(0.0001, now + length).ok()?;
    oscillator.connect_with_audio_node(&gain).ok()?;
    gain.connect_with_audio_node(&context.destination()).ok()?;
    oscillator.start().ok()?;
    oscillator.stop_with_when(now + length).ok()?;
    Some(())
}
