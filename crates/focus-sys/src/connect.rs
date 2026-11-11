//! Wi-Fi, Bluetooth et radios par les API publiques de Windows (WinRT).
//! Les appels bloquent : l'application les fait tourner dans un fil à part,
//! jamais dans celui de la fenêtre.

use serde::Serialize;
use windows::core::HSTRING;
use windows::Devices::Bluetooth::{BluetoothConnectionStatus, BluetoothDevice, BluetoothLEDevice};
use windows::Devices::Enumeration::{
    DeviceInformation, DevicePairingResultStatus, DeviceUnpairingResultStatus,
};
use windows::Devices::Radios::{Radio, RadioKind, RadioState};
use windows::Devices::WiFi::{
    WiFiAdapter, WiFiAvailableNetwork, WiFiConnectionStatus, WiFiReconnectionKind,
};
use windows::Networking::Connectivity::{NetworkConnectivityLevel, NetworkInformation};
use windows::Security::Credentials::PasswordCredential;

/// Résultat d'un appel Windows ; l'erreur s'affiche avec `to_string`.
pub type WinResult<T> = windows::core::Result<T>;

/// Une radio de l'ordinateur.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioInfo {
    /// `wifi` ou `bluetooth`.
    pub kind: &'static str,
    /// Allumée.
    pub on: bool,
}

fn kind_name(kind: RadioKind) -> Option<&'static str> {
    if kind == RadioKind::WiFi {
        Some("wifi")
    } else if kind == RadioKind::Bluetooth {
        Some("bluetooth")
    } else {
        None
    }
}

/// Radios Wi-Fi et Bluetooth présentes.
pub fn radios() -> WinResult<Vec<RadioInfo>> {
    let _ = Radio::RequestAccessAsync()?.get()?;
    let mut found = Vec::new();
    for radio in Radio::GetRadiosAsync()?.get()? {
        if let Some(kind) = kind_name(radio.Kind()?) {
            found.push(RadioInfo {
                kind,
                on: radio.State()? == RadioState::On,
            });
        }
    }
    Ok(found)
}

/// Allume ou éteint une radio. Vrai si Windows a accepté.
pub fn set_radio(kind: &str, on: bool) -> WinResult<bool> {
    let _ = Radio::RequestAccessAsync()?.get()?;
    let state = if on { RadioState::On } else { RadioState::Off };
    let mut done = false;
    for radio in Radio::GetRadiosAsync()?.get()? {
        if kind_name(radio.Kind()?) == Some(kind) {
            let status = radio.SetStateAsync(state)?.get()?;
            done |= status == windows::Devices::Radios::RadioAccessStatus::Allowed;
        }
    }
    Ok(done)
}

/// État de la connexion à Internet, pour la barre du haut.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetStatus {
    /// `wifi`, `ethernet` ou `none`.
    pub kind: &'static str,
    /// Accès à Internet.
    pub internet: bool,
    /// Nom du réseau Wi-Fi, s'il y en a un.
    pub ssid: Option<String>,
}

/// Connexion actuelle.
pub fn net_status() -> NetStatus {
    let read = || -> WinResult<NetStatus> {
        let Ok(profile) = NetworkInformation::GetInternetConnectionProfile() else {
            return Ok(NetStatus {
                kind: "none",
                ..NetStatus::default()
            });
        };
        let internet =
            profile.GetNetworkConnectivityLevel()? == NetworkConnectivityLevel::InternetAccess;
        if profile.IsWlanConnectionProfile()? {
            let ssid = profile
                .WlanConnectionProfileDetails()?
                .GetConnectedSsid()?
                .to_string_lossy();
            Ok(NetStatus {
                kind: "wifi",
                internet,
                ssid: Some(ssid),
            })
        } else {
            Ok(NetStatus {
                kind: "ethernet",
                internet,
                ssid: None,
            })
        }
    };
    read().unwrap_or(NetStatus {
        kind: "none",
        ..NetStatus::default()
    })
}

/// Un réseau Wi-Fi visible.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiNetwork {
    /// Nom.
    pub ssid: String,
    /// Force du signal, 0 à 4.
    pub bars: u8,
    /// Protégé par un mot de passe.
    pub secure: bool,
    /// Réseau actuel.
    pub connected: bool,
}

fn adapter() -> WinResult<Option<WiFiAdapter>> {
    let _ = WiFiAdapter::RequestAccessAsync()?.get()?;
    let adapters = WiFiAdapter::FindAllAdaptersAsync()?.get()?;
    Ok(adapters.into_iter().next())
}

fn is_secure(network: &WiFiAvailableNetwork) -> WinResult<bool> {
    use windows::Networking::Connectivity::NetworkAuthenticationType as Auth;
    let auth = network.SecuritySettings()?.NetworkAuthenticationType()?;
    Ok(auth != Auth::Open80211 && auth != Auth::None)
}

/// Réseaux visibles, du plus fort au plus faible. `scan` relance une recherche.
pub fn wifi_networks(scan: bool) -> WinResult<Vec<WifiNetwork>> {
    let Some(adapter) = adapter()? else {
        return Ok(Vec::new());
    };
    if scan {
        adapter.ScanAsync()?.get()?;
    }
    let connected = net_status().ssid;
    let mut networks: Vec<WifiNetwork> = Vec::new();
    for network in adapter.NetworkReport()?.AvailableNetworks()? {
        let ssid = network.Ssid()?.to_string_lossy();
        if ssid.is_empty() {
            continue;
        }
        let item = WifiNetwork {
            connected: connected.as_deref() == Some(ssid.as_str()),
            bars: network.SignalBars()?,
            secure: is_secure(&network)?,
            ssid,
        };
        // Un même réseau apparaît une fois par borne : on garde la plus forte.
        match networks.iter_mut().find(|known| known.ssid == item.ssid) {
            Some(known) if known.bars < item.bars => *known = item,
            Some(_) => {}
            None => networks.push(item),
        }
    }
    networks.sort_by(|a, b| b.connected.cmp(&a.connected).then(b.bars.cmp(&a.bars)));
    Ok(networks)
}

/// Résultat d'une connexion, traduit par l'interface.
pub fn wifi_connect(ssid: &str, password: Option<&str>) -> WinResult<&'static str> {
    let Some(adapter) = adapter()? else {
        return Ok("noAdapter");
    };
    let network = adapter
        .NetworkReport()?
        .AvailableNetworks()?
        .into_iter()
        .filter(|network| network.Ssid().is_ok_and(|name| name.to_string_lossy() == ssid))
        .max_by_key(|network| network.SignalBars().unwrap_or(0));
    let Some(network) = network else {
        return Ok("notFound");
    };
    let result = match password.filter(|text| !text.is_empty()) {
        Some(password) => {
            let credential = PasswordCredential::new()?;
            credential.SetPassword(&HSTRING::from(password))?;
            adapter
                .ConnectWithPasswordCredentialAsync(
                    &network,
                    WiFiReconnectionKind::Automatic,
                    &credential,
                )?
                .get()?
        }
        None => adapter
            .ConnectAsync(&network, WiFiReconnectionKind::Automatic)?
            .get()?,
    };
    let status = result.ConnectionStatus()?;
    Ok(if status == WiFiConnectionStatus::Success {
        "success"
    } else if status == WiFiConnectionStatus::InvalidCredential {
        "badPassword"
    } else if status == WiFiConnectionStatus::Timeout {
        "timeout"
    } else {
        "failed"
    })
}

/// Se déconnecte du réseau Wi-Fi actuel.
pub fn wifi_disconnect() -> WinResult<()> {
    if let Some(adapter) = adapter()? {
        adapter.Disconnect()?;
    }
    Ok(())
}

/// Un appareil Bluetooth.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BtDevice {
    /// Identifiant Windows, renvoyé tel quel pour appairer ou oublier.
    pub id: String,
    /// Nom.
    pub name: String,
    /// Appairé.
    pub paired: bool,
    /// Connecté.
    pub connected: bool,
}

fn classic_connected(id: &HSTRING) -> bool {
    BluetoothDevice::FromIdAsync(id)
        .and_then(|op| op.get())
        .and_then(|device| device.ConnectionStatus())
        .is_ok_and(|status| status == BluetoothConnectionStatus::Connected)
}

fn le_connected(id: &HSTRING) -> bool {
    BluetoothLEDevice::FromIdAsync(id)
        .and_then(|op| op.get())
        .and_then(|device| device.ConnectionStatus())
        .is_ok_and(|status| status == BluetoothConnectionStatus::Connected)
}

fn find(selector: &HSTRING, paired: bool, le: bool, out: &mut Vec<BtDevice>) -> WinResult<()> {
    for info in DeviceInformation::FindAllAsyncAqsFilter(selector)?.get()? {
        let id = info.Id()?;
        let name = info.Name()?.to_string_lossy();
        if name.is_empty() || out.iter().any(|known| known.name == name) {
            continue;
        }
        let connected = paired && if le { le_connected(&id) } else { classic_connected(&id) };
        out.push(BtDevice {
            id: id.to_string_lossy(),
            name,
            paired,
            connected,
        });
    }
    Ok(())
}

/// Appareils appairés, classiques et basse consommation.
pub fn bt_paired() -> WinResult<Vec<BtDevice>> {
    let mut devices = Vec::new();
    find(&BluetoothDevice::GetDeviceSelectorFromPairingState(true)?, true, false, &mut devices)?;
    find(&BluetoothLEDevice::GetDeviceSelectorFromPairingState(true)?, true, true, &mut devices)?;
    devices.sort_by(|a, b| b.connected.cmp(&a.connected).then(a.name.cmp(&b.name)));
    Ok(devices)
}

/// Appareils proches pas encore appairés. La recherche dure quelques secondes.
pub fn bt_nearby() -> WinResult<Vec<BtDevice>> {
    let mut devices = Vec::new();
    find(&BluetoothDevice::GetDeviceSelectorFromPairingState(false)?, false, false, &mut devices)?;
    find(&BluetoothLEDevice::GetDeviceSelectorFromPairingState(false)?, false, true, &mut devices)?;
    Ok(devices)
}

/// Appaire un appareil. Windows peut afficher sa propre demande de confirmation.
pub fn bt_pair(id: &str) -> WinResult<&'static str> {
    let info = DeviceInformation::CreateFromIdAsync(&HSTRING::from(id))?.get()?;
    let status = info.Pairing()?.PairAsync()?.get()?.Status()?;
    Ok(
        if status == DevicePairingResultStatus::Paired
            || status == DevicePairingResultStatus::AlreadyPaired
        {
            "success"
        } else {
            "failed"
        },
    )
}

/// Oublie un appareil appairé.
pub fn bt_unpair(id: &str) -> WinResult<&'static str> {
    let info = DeviceInformation::CreateFromIdAsync(&HSTRING::from(id))?.get()?;
    let status = info.Pairing()?.UnpairAsync()?.get()?.Status()?;
    Ok(
        if status == DeviceUnpairingResultStatus::Unpaired
            || status == DeviceUnpairingResultStatus::AlreadyUnpaired
        {
            "success"
        } else {
            "failed"
        },
    )
}
