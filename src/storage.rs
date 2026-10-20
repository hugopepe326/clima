use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::Storage;

use crate::models::{Location, Provider, Units};

const KEY_LOCATION: &str = "fedeweather.v1.location";
const KEY_PROVIDER: &str = "fedeweather.v1.provider";
const KEY_UNITS: &str = "fedeweather.v1.units";
const KEY_FAVORITES: &str = "fedeweather.v1.favorites";
const KEY_RECENTS: &str = "fedeweather.v1.recents";

fn storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn load_json<T: DeserializeOwned>(key: &str) -> Option<T> {
    let raw = storage()?.get_item(key).ok().flatten()?;
    serde_json::from_str(&raw).ok()
}

fn save_json<T: Serialize>(key: &str, value: &T) -> Result<(), JsValue> {
    let store = storage().ok_or_else(|| JsValue::from_str("localStorage no disponible"))?;
    let raw = serde_json::to_string(value)
        .map_err(|_| JsValue::from_str("No se pudo serializar el estado"))?;
    store.set_item(key, &raw)
}

pub fn load_location() -> Option<Location> {
    load_json(KEY_LOCATION)
}

pub fn load_provider() -> Provider {
    storage()
        .and_then(|s| s.get_item(KEY_PROVIDER).ok().flatten())
        .map(|id| Provider::from_id(&id))
        .unwrap_or_default()
}

pub fn load_units() -> Units {
    storage()
        .and_then(|s| s.get_item(KEY_UNITS).ok().flatten())
        .map(|id| Units::from_id(&id))
        .unwrap_or_default()
}

pub fn load_favorites() -> Vec<Location> {
    load_json(KEY_FAVORITES).unwrap_or_default()
}

pub fn load_recents() -> Vec<Location> {
    load_json(KEY_RECENTS).unwrap_or_default()
}

pub fn save_location(location: &Location) {
    let _ = save_json(KEY_LOCATION, location);
}

pub fn save_provider(provider: Provider) {
    if let Some(store) = storage() {
        let _ = store.set_item(KEY_PROVIDER, provider.id());
    }
}

pub fn save_units(units: Units) {
    if let Some(store) = storage() {
        let _ = store.set_item(KEY_UNITS, match units {
            Units::Celsius => "c",
            Units::Fahrenheit => "f",
        });
    }
}

pub fn save_favorites(favorites: &[Location]) {
    let _ = save_json(KEY_FAVORITES, &favorites);
}

pub fn save_recents(recents: &[Location]) {
    let _ = save_json(KEY_RECENTS, &recents);
}
