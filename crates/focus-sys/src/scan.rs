//! Instantané en lecture seule des processus et des services.

use crate::wide::from_wide;
use focus_core::model::{ProcessInfo, ProcessRole, ServiceInfo, ServiceState, Snapshot};
use focus_core::whitelist::normalize;
use std::mem::size_of;
use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::{CloseHandle, FILETIME, HANDLE, MAX_PATH};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
use windows::Win32::System::Services::{
    CloseServiceHandle, EnumServicesStatusExW, OpenSCManagerW, ENUM_SERVICE_STATUS_PROCESSW,
    SC_ENUM_PROCESS_INFO, SC_MANAGER_ENUMERATE_SERVICE, SERVICE_RUNNING, SERVICE_STATE_ALL,
    SERVICE_STOPPED, SERVICE_WIN32,
};
use windows::Win32::System::Threading::{
    GetProcessTimes, OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
    PROCESS_QUERY_LIMITED_INFORMATION,
};

/// Heure de création d'un processus, qui le distingue d'un pid réutilisé.
pub fn process_start_time(pid: u32) -> Option<u64> {
    // SAFETY: le handle est vérifié puis fermé ; droits en lecture limitée.
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let time = start_time(handle);
        let _ = CloseHandle(handle);
        time
    }
}

unsafe fn start_time(handle: HANDLE) -> Option<u64> {
    let (mut created, mut exited, mut kernel, mut user) = (
        FILETIME::default(),
        FILETIME::default(),
        FILETIME::default(),
        FILETIME::default(),
    );
    // SAFETY: les pointeurs désignent des variables locales valides.
    unsafe { GetProcessTimes(handle, &mut created, &mut exited, &mut kernel, &mut user).ok()? };
    Some((u64::from(created.dwHighDateTime) << 32) | u64::from(created.dwLowDateTime))
}

fn process_details(pid: u32) -> (u64, Option<String>, u64) {
    // SAFETY: le handle est vérifié puis fermé ; tous les tampons sont locaux.
    unsafe {
        let Ok(handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
            return (0, None, 0);
        };
        let time = start_time(handle).unwrap_or(0);
        let mut buffer = [0_u16; MAX_PATH as usize * 4];
        let mut size = buffer.len() as u32;
        let path = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(buffer.as_mut_ptr()),
            &mut size,
        )
        .ok()
        .map(|()| String::from_utf16_lossy(&buffer[..size as usize]));
        let mut memory = PROCESS_MEMORY_COUNTERS::default();
        let bytes = GetProcessMemoryInfo(
            handle,
            &mut memory,
            size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
        .map_or(0, |()| memory.WorkingSetSize as u64);
        let _ = CloseHandle(handle);
        (time, path, bytes)
    }
}

/// Session Windows d'un processus, sans l'ouvrir.
pub fn session_of(pid: u32) -> Option<u32> {
    let mut session = 0_u32;
    // SAFETY: variable locale ; aucun handle n'est ouvert.
    unsafe { windows::Win32::System::RemoteDesktop::ProcessIdToSessionId(pid, &mut session) }
        .ok()
        .map(|()| session)
}

/// Dossier de Windows en minuscules, avec sa barre finale.
fn windows_dir() -> String {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_owned());
    root.trim_end_matches('\\').to_lowercase() + "\\"
}

/// Place d'un processus dans la session : seuls les programmes du compte comptent.
/// Une application a une fenêtre visible ; le reste, hors dossier de Windows, est de
/// l'arrière-plan. Tout ce qui est illisible ou d'une autre session reste « système ».
fn role_of(pid: u32, path: Option<&str>, own_session: Option<u32>, windowed: &[u32], windows_dir: &str) -> ProcessRole {
    let Some(path) = path else {
        return ProcessRole::System;
    };
    if pid == std::process::id() || own_session.is_none() || session_of(pid) != own_session {
        ProcessRole::System
    } else if windowed.contains(&pid) {
        ProcessRole::App
    } else if path.to_lowercase().starts_with(windows_dir) {
        ProcessRole::System
    } else {
        ProcessRole::Background
    }
}

/// Liste les processus. Un accès refusé n'est pas une erreur.
pub fn processes() -> Vec<ProcessInfo> {
    let mut list = Vec::new();
    let own_session = session_of(std::process::id());
    let windowed = crate::console::app_window_pids();
    let windows_dir = windows_dir();
    // SAFETY: le snapshot est fermé ; `entry` a sa taille renseignée.
    unsafe {
        let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
            return list;
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut more = Process32FirstW(snapshot, &mut entry).is_ok();
        while more {
            let pid = entry.th32ProcessID;
            let (start_time, path, memory) = process_details(pid);
            list.push(ProcessInfo {
                pid,
                start_time,
                name: normalize(&from_wide(&entry.szExeFile)),
                role: role_of(pid, path.as_deref(), own_session, &windowed, &windows_dir),
                path,
                memory,
            });
            more = Process32NextW(snapshot, &mut entry).is_ok();
        }
        let _ = CloseHandle(snapshot);
    }
    list
}

/// Liste les services Win32 par nom interne.
pub fn services() -> Vec<ServiceInfo> {
    let mut list = Vec::new();
    // SAFETY: le tampon est aligné sur `u64`, assez grand pour les structures
    // renvoyées, et les chaînes pointées vivent dans ce même tampon.
    unsafe {
        let Ok(manager) =
            OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), SC_MANAGER_ENUMERATE_SERVICE)
        else {
            return list;
        };
        let (mut needed, mut count, mut resume) = (0_u32, 0_u32, 0_u32);
        let _ = EnumServicesStatusExW(
            manager,
            SC_ENUM_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            None,
            &mut needed,
            &mut count,
            Some(&mut resume),
            PCWSTR::null(),
        );
        let mut buffer = vec![0_u64; (needed as usize).div_ceil(8) + 1];
        let bytes =
            std::slice::from_raw_parts_mut(buffer.as_mut_ptr().cast::<u8>(), buffer.len() * 8);
        resume = 0;
        if EnumServicesStatusExW(
            manager,
            SC_ENUM_PROCESS_INFO,
            SERVICE_WIN32,
            SERVICE_STATE_ALL,
            Some(bytes),
            &mut needed,
            &mut count,
            Some(&mut resume),
            PCWSTR::null(),
        )
        .is_ok()
        {
            let items = std::slice::from_raw_parts(
                buffer.as_ptr().cast::<ENUM_SERVICE_STATUS_PROCESSW>(),
                count as usize,
            );
            for item in items {
                let state = item.ServiceStatusProcess.dwCurrentState;
                list.push(ServiceInfo {
                    name: item.lpServiceName.to_string().unwrap_or_default(),
                    display_name: item.lpDisplayName.to_string().unwrap_or_default(),
                    state: if state == SERVICE_RUNNING {
                        ServiceState::Running
                    } else if state == SERVICE_STOPPED {
                        ServiceState::Stopped
                    } else {
                        ServiceState::Other
                    },
                });
            }
        }
        let _ = CloseServiceHandle(manager);
    }
    list
}

/// Prend un instantané complet.
pub fn snapshot() -> Snapshot {
    Snapshot {
        processes: processes(),
        services: services(),
    }
}

/// Noms des processus, sans ouvrir aucun d'eux : c'est ce qui permet de suivre un
/// jeu protégé par un anticheat sans jamais y toucher (SPEC D7).
pub fn process_names() -> Vec<String> {
    let mut names = Vec::new();
    // SAFETY: le snapshot est fermé ; `entry` a sa taille renseignée.
    unsafe {
        let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
            return names;
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut more = Process32FirstW(snapshot, &mut entry).is_ok();
        while more {
            names.push(normalize(&from_wide(&entry.szExeFile)));
            more = Process32NextW(snapshot, &mut entry).is_ok();
        }
        let _ = CloseHandle(snapshot);
    }
    names
}

/// Pids et noms des processus, sans ouvrir aucun processus (SPEC D7).
pub fn process_ids() -> Vec<(u32, String)> {
    let mut list = Vec::new();
    // SAFETY: le snapshot est fermé ; `entry` a sa taille renseignée.
    unsafe {
        let Ok(snapshot) = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) else {
            return list;
        };
        let mut entry = PROCESSENTRY32W {
            dwSize: size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };
        let mut more = Process32FirstW(snapshot, &mut entry).is_ok();
        while more {
            list.push((entry.th32ProcessID, normalize(&from_wide(&entry.szExeFile))));
            more = Process32NextW(snapshot, &mut entry).is_ok();
        }
        let _ = CloseHandle(snapshot);
    }
    list
}

/// L'un de ces exécutables tourne-t-il ?
pub fn any_running(names: &[String]) -> bool {
    !names.is_empty() && process_names().iter().any(|name| names.contains(name))
}

/// Vrai si un processus de ce nom tourne.
pub fn is_running(name: &str) -> bool {
    let key = normalize(name);
    processes().iter().any(|process| process.name == key)
}

/// Processus dont l'exécutable est dans `dir` : le vrai jeu, derrière un lanceur.
pub fn processes_in_dir(dir: &str) -> Vec<ProcessInfo> {
    let root = dir.replace('/', "\\").trim_end_matches('\\').to_lowercase() + "\\";
    processes()
        .into_iter()
        .filter(|process| {
            process
                .path
                .as_ref()
                .is_some_and(|path| path.to_lowercase().starts_with(&root))
        })
        .collect()
}

/// Nom lisible d'un programme (« Google Chrome », « Bloc-notes »), lu dans la
/// description de son exécutable. Lecture du fichier seulement : aucun processus
/// n'est ouvert. `None` si l'exécutable n'en a pas.
pub fn display_name(path: &str) -> Option<String> {
    use std::ffi::c_void;
    use windows::Win32::Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW};
    let file = crate::wide::wide(path);
    // SAFETY: tampons locaux de la taille annoncée par Windows ; les pointeurs rendus
    // par `VerQueryValueW` désignent l'intérieur de `data`, vivant pendant la lecture.
    unsafe {
        let size = GetFileVersionInfoSizeW(PCWSTR(file.as_ptr()), None);
        if size == 0 {
            return None;
        }
        let mut data = vec![0_u8; size as usize];
        GetFileVersionInfoW(PCWSTR(file.as_ptr()), 0, size, data.as_mut_ptr().cast()).ok()?;
        let query = |key: &str| -> Option<(*const c_void, u32)> {
            let key = crate::wide::wide(key);
            let mut pointer: *mut c_void = std::ptr::null_mut();
            let mut length = 0_u32;
            VerQueryValueW(data.as_ptr().cast(), PCWSTR(key.as_ptr()), &mut pointer, &mut length)
                .as_bool()
                .then_some((pointer.cast_const(), length))
                .filter(|(pointer, length)| !pointer.is_null() && *length > 0)
        };
        let (translation, length) = query(r"\VarFileInfo\Translation")?;
        if length < 4 {
            return None;
        }
        let language = *translation.cast::<u16>();
        let codepage = *translation.cast::<u16>().add(1);
        let (text, length) = query(&format!(r"\StringFileInfo\{language:04x}{codepage:04x}\FileDescription"))?;
        let chars = std::slice::from_raw_parts(text.cast::<u16>(), length as usize);
        let name = String::from_utf16_lossy(chars).trim_end_matches('\0').trim().to_owned();
        (!name.is_empty()).then_some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::display_name;

    #[test]
    fn windows_program_should_have_a_readable_name() {
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".to_owned());
        let name = display_name(&format!(r"{root}\System32\notepad.exe"));
        assert!(name.is_some_and(|name| !name.eq_ignore_ascii_case("notepad")));
    }

    #[test]
    fn missing_file_should_have_no_name() {
        assert_eq!(display_name(r"Z:\nowhere\none.exe"), None);
    }
}
