//! Volume général et volume par application, à travers l'audio de Windows.

use serde::Serialize;
use windows::core::Interface;
use windows::Win32::Media::Audio::{
    eConsole, eRender, IAudioSessionControl2, IAudioSessionManager2, IMMDeviceEnumerator,
    ISimpleAudioVolume, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};

/// Une session audio.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSession {
    /// Processus propriétaire.
    pub pid: u32,
    /// Volume, de 0 à 1.
    pub volume: f32,
}

fn manager() -> windows::core::Result<IAudioSessionManager2> {
    // SAFETY: COM est initialisé sur ce fil ; les interfaces gèrent leur durée de vie.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        device.Activate(CLSCTX_ALL, None)
    }
}

fn each_session(mut visit: impl FnMut(u32, &ISimpleAudioVolume)) -> windows::core::Result<()> {
    // SAFETY: chaque interface renvoyée est valide tant que la variable existe.
    unsafe {
        let sessions = manager()?.GetSessionEnumerator()?;
        for index in 0..sessions.GetCount()? {
            let control = sessions.GetSession(index)?;
            let details: IAudioSessionControl2 = control.cast()?;
            let pid = details.GetProcessId().unwrap_or(0);
            let volume: ISimpleAudioVolume = control.cast()?;
            visit(pid, &volume);
        }
    }
    Ok(())
}

/// Liste les sessions audio. L'audio étant facultatif, une erreur donne une liste vide.
pub fn sessions() -> Vec<AudioSession> {
    let mut list = Vec::new();
    let result = each_session(|pid, volume| {
        // SAFETY: interface valide pendant le rappel.
        if let Ok(level) = unsafe { volume.GetMasterVolume() } {
            list.push(AudioSession { pid, volume: level });
        }
    });
    if let Err(err) = result {
        tracing::warn!("audio sessions unavailable: {err}");
    }
    list
}

/// Règle le volume d'une application.
pub fn set_volume(pid: u32, level: f32) -> windows::core::Result<()> {
    let level = level.clamp(0.0, 1.0);
    each_session(|session_pid, volume| {
        if session_pid == pid {
            // SAFETY: interface valide pendant le rappel.
            let _ = unsafe { volume.SetMasterVolume(level, std::ptr::null()) };
        }
    })
}

fn endpoint() -> windows::core::Result<windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume> {
    // SAFETY: COM est initialisé sur ce fil ; les interfaces gèrent leur durée de vie.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?.Activate(CLSCTX_ALL, None)
    }
}

/// Volume général de la sortie son, de 0 à 1.
pub fn master_volume() -> windows::core::Result<f32> {
    // SAFETY: interface valide le temps de l'appel.
    unsafe { endpoint()?.GetMasterVolumeLevelScalar() }
}

/// Règle le volume général, de 0 à 1. Le son est rétabli s'il était coupé.
pub fn set_master_volume(level: f32) -> windows::core::Result<()> {
    let volume = endpoint()?;
    // SAFETY: interface valide le temps des appels ; aucun contexte d'événement.
    unsafe {
        volume.SetMasterVolumeLevelScalar(level.clamp(0.0, 1.0), std::ptr::null())?;
        volume.SetMute(false, std::ptr::null())
    }
}
