use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::geolocate;
use crate::js;
use crate::models::{AirQuality, Location, Provider, Units, Weather};
use crate::storage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: u32,
    pub message: String,
    pub kind: ToastKind,
}

#[derive(Clone, Copy)]
pub struct AppCtx {
    pub location: RwSignal<Location>,
    pub provider: RwSignal<Provider>,
    pub units: RwSignal<Units>,
    pub weather: RwSignal<Option<Weather>>,
    pub air: RwSignal<Option<AirQuality>>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub geo_loading: RwSignal<bool>,
    pub toast: RwSignal<Option<Toast>>,
    pub favorites: RwSignal<Vec<Location>>,
    pub recents: RwSignal<Vec<Location>>,
    pub selected_day: RwSignal<usize>,
    pub now_ms: RwSignal<f64>,
    pub refresh: RwSignal<u32>,
    pub mobile_search: RwSignal<bool>,
    pub map_dark: RwSignal<bool>,
}

impl AppCtx {
    pub fn new() -> Self {
        let location = storage::load_location().unwrap_or_default();
        let favorites = storage::load_favorites();
        let recents = storage::load_recents();

        Self {
            location: RwSignal::new(location),
            provider: RwSignal::new(storage::load_provider()),
            units: RwSignal::new(storage::load_units()),
            weather: RwSignal::new(None),
            air: RwSignal::new(None),
            loading: RwSignal::new(true),
            error: RwSignal::new(None),
            geo_loading: RwSignal::new(false),
            toast: RwSignal::new(None),
            favorites: RwSignal::new(favorites),
            recents: RwSignal::new(recents),
            selected_day: RwSignal::new(0),
            now_ms: RwSignal::new(js_sys::Date::now()),
            refresh: RwSignal::new(0),
            mobile_search: RwSignal::new(false),
            map_dark: RwSignal::new(true),
        }
    }

    pub fn notify(&self, message: impl Into<String>, kind: ToastKind) {
        let id = js_sys::Date::now() as u32;
        self.toast.set(Some(Toast {
            id,
            message: message.into(),
            kind,
        }));

        let toast = self.toast;
        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(4_600).await;
            if toast.get_untracked().map(|t| t.id) == Some(id) {
                toast.set(None);
            }
        });
    }

    pub fn refresh_now(&self) {
        self.refresh.update(|n| *n = n.wrapping_add(1));
    }

    pub fn select_location(&self, location: Location) {
        if self.location.get_untracked().same_place(&location) {
            self.mobile_search.set(false);
            return;
        }
        self.location.set(location);
        self.selected_day.set(0);
        self.mobile_search.set(false);
        self.error.set(None);
        scroll_to_top();
    }

    pub fn use_my_location(&self) {
        if self.geo_loading.get_untracked() {
            return;
        }
        self.geo_loading.set(true);
        let ctx = *self;

        spawn_local(async move {
            let result = async {
                let (latitude, longitude, accuracy) = geolocate::request_position().await?;
                match api::reverse_geocode(latitude, longitude).await {
                    Ok(mut location) => {
                        location.accuracy = Some(accuracy);
                        location.is_current = true;
                        Ok::<Location, String>(location)
                    }
                    Err(_) => Ok::<Location, String>(Location {
                        name: "Mi ubicación".to_string(),
                        admin: None,
                        country: None,
                        country_code: None,
                        latitude,
                        longitude,
                        timezone: js::browser_timezone(),
                        accuracy: Some(accuracy),
                        population: None,
                        is_current: true,
                    }),
                }
            }
            .await;

            match result {
                Ok(mut location) => {
                    let accuracy = location.accuracy.unwrap_or(0.0);
                    location.is_current = true;
                    ctx.location.set(location);
                    ctx.selected_day.set(0);
                    ctx.error.set(None);
                    ctx.mobile_search.set(false);
                    scroll_to_top();
                    if accuracy > 2_000.0 {
                        ctx.notify(
                            format!(
                                "Ubicación aproximada (±{:.0} km). El clima corresponde a esa zona.",
                                accuracy / 1000.0
                            ),
                            ToastKind::Info,
                        );
                    } else {
                        ctx.notify("Ubicación actualizada con GPS", ToastKind::Success);
                    }
                }
                Err(message) => ctx.notify(message, ToastKind::Error),
            }
            ctx.geo_loading.set(false);
        });
    }

    pub fn toggle_favorite(&self) {
        let current = self.location.get_untracked();
        let mut favorites = self.favorites.get_untracked();
        let existing = favorites.iter().position(|f| f.same_place(&current));

        if let Some(index) = existing {
            favorites.remove(index);
            self.notify("Ubicación quitada de favoritos", ToastKind::Info);
        } else {
            let mut saved = current.clone();
            saved.is_current = false;
            favorites.insert(0, saved);
            favorites.truncate(8);
            self.notify(
                format!("{} guardada en favoritos", current.name),
                ToastKind::Success,
            );
        }

        self.favorites.set(favorites.clone());
        storage::save_favorites(&favorites);
    }

    pub fn is_favorite(&self) -> bool {
        let current = self.location.get();
        self.favorites
            .get()
            .iter()
            .any(|f| f.same_place(&current))
    }

    pub fn push_recent(&self, location: &Location) {
        let mut recents = self.recents.get_untracked();
        recents.retain(|r| !r.same_place(location));
        recents.insert(0, location.clone());
        recents.truncate(6);
        self.recents.set(recents.clone());
        storage::save_recents(&recents);
    }
}

pub fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

pub fn scroll_to_id(id: &str) {
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        if let Some(element) = document.get_element_by_id(id) {
            element.scroll_into_view_with_bool(true);
        }
    }
}
