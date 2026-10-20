use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::components::icons::{Icon, UiIcon};
use crate::js;
use crate::models::condition;
use crate::state::AppCtx;

#[component]
pub fn MapSection() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let map_ready = RwSignal::new(false);
    let map_failed = RwSignal::new(false);

    Effect::new(move |_| {
        let Some(weather) = ctx.weather.get() else {
            return;
        };
        let units = ctx.units.get();
        let dark = ctx.map_dark.get();
        let location = ctx.location.get();
        let temp = format!("{:.0}", units.temp(weather.current.temperature));
        let units_label = units.label().to_string();
        let description = condition(weather.current.weather_code).label.to_string();

        spawn_local(async move {
            for attempt in 0..12 {
                let temp = temp.clone();
                let units_label = units_label.clone();
                let description = description.clone();
                let name = location.name.clone();

                let success = js::sync_map(
                    location.latitude,
                    location.longitude,
                    &name,
                    &temp,
                    &description,
                    &units_label,
                    10,
                );

                if success {
                    js::set_map_layer(if dark { "dark" } else { "osm" });
                    map_ready.set(true);
                    map_failed.set(false);
                    return;
                }

                if attempt == 11 {
                    map_failed.set(true);
                }
                gloo_timers::future::TimeoutFuture::new(280).await;
            }
        });
    });

    view! {
        <section id="mapa" class="scroll-mt-24">
            <div class="card overflow-hidden p-3 sm:p-4">
                <div class="flex flex-wrap items-center justify-between gap-2 px-1 pb-3">
                    <div>
                        <h2 class="section-title">"Mapa en vivo"</h2>
                        <p class="mt-1 text-[10px] text-slate-500">
                            "Arrastrá, hacé zoom y explorá la zona de "
                            <span class="text-slate-400">
                                {move || ctx.location.get().name}
                            </span>
                        </p>
                    </div>
                    <div class="flex items-center gap-1.5">
                        <div class="inline-flex rounded-xl border border-white/[0.07] bg-black/20 p-0.5">
                            <button
                                type="button"
                                class=move || format!(
                                    "focus-ring rounded-[9px] px-2.5 py-1.5 text-[9px] font-medium transition-all {}",
                                    if ctx.map_dark.get() { "bg-white/[0.08] text-white" } else { "text-slate-500 hover:text-slate-300" }
                                )
                                aria-pressed=move || ctx.map_dark.get().to_string()
                                on:click=move |_| {
                                    ctx.map_dark.set(true);
                                    js::set_map_layer("dark");
                                }
                            >
                                "Oscuro"
                            </button>
                            <button
                                type="button"
                                class=move || format!(
                                    "focus-ring rounded-[9px] px-2.5 py-1.5 text-[9px] font-medium transition-all {}",
                                    if !ctx.map_dark.get() { "bg-white/[0.08] text-white" } else { "text-slate-500 hover:text-slate-300" }
                                )
                                aria-pressed=move || (!ctx.map_dark.get()).to_string()
                                on:click=move |_| {
                                    ctx.map_dark.set(false);
                                    js::set_map_layer("osm");
                                }
                            >
                                "Calles"
                            </button>
                        </div>
                        <button
                            type="button"
                            class="focus-ring grid h-8 w-8 place-items-center rounded-xl border border-white/[0.07] bg-white/[0.035] text-slate-400 transition-colors hover:bg-white/[0.07] hover:text-white"
                            aria-label="Centrar mapa en la ubicación"
                            title="Centrar mapa"
                            on:click=move |_| {
                                let location = ctx.location.get_untracked();
                                js::recenter_map(location.latitude, location.longitude);
                            }
                        >
                            <Icon kind=UiIcon::Locate class="h-3.5 w-3.5" />
                        </button>
                    </div>
                </div>

                <div class="relative overflow-hidden rounded-2xl border border-white/[0.06] bg-navy-900">
                    <div id="weather-map" class="z-0 h-[330px] w-full sm:h-[420px]" aria-label="Mapa interactivo del clima"></div>

                    <div class="pointer-events-none absolute bottom-4 left-3 z-[500] flex items-center gap-2 rounded-full border border-white/10 bg-navy-950/75 px-3 py-1.5 text-[9px] text-slate-400 backdrop-blur-md">
                        <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-ice-300"></span>
                        <span class="num">
                            {move || ctx.location.get().coordinates()}
                        </span>
                    </div>

                    <Show when=move || !map_ready.get() && !map_failed.get()>
                        <div class="pointer-events-none absolute inset-0 z-[450] grid place-items-center bg-navy-900/40 backdrop-blur-[2px]">
                            <div class="flex items-center gap-2 rounded-full bg-navy-950/80 px-3 py-2 text-[10px] text-slate-300">
                                <span class="h-3 w-3 animate-spin rounded-full border border-ice-400/30 border-t-ice-300"></span>
                                "Cargando mapa interactivo…"
                            </div>
                        </div>
                    </Show>

                    <Show when=move || map_failed.get()>
                        <div class="pointer-events-none absolute inset-0 z-[450] grid place-items-center bg-navy-900/70 px-6 text-center">
                            <div>
                                <p class="text-[12px] text-slate-300">"No se pudo cargar Leaflet"</p>
                                <p class="mt-1 text-[10px] leading-relaxed text-slate-500">
                                    "Revisá tu conexión o el bloqueador de contenido y recargá la página."
                                </p>
                            </div>
                        </div>
                    </Show>
                </div>

                <div class="mt-2 flex flex-wrap items-center justify-between gap-1 px-1 text-[8px] text-slate-600">
                    <span>
                        {move || if ctx.map_dark.get() {
                            "Leaflet · Esri · OpenStreetMap"
                        } else {
                            "Leaflet · OpenStreetMap"
                        }}
                    </span>
                    <span>"Zoom, paneo y capas 100 % interactivos"</span>
                </div>
            </div>
        </section>
    }
}
