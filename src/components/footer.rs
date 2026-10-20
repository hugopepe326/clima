use leptos::prelude::*;

use crate::components::brand::Brand;
use crate::state::{scroll_to_top, AppCtx};

#[component]
pub fn Footer() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <footer class="mt-8 border-t border-white/[0.05] px-4 pb-8 pt-7 sm:px-6 lg:px-8">
            <div class="mx-auto flex max-w-[1560px] flex-col gap-5">
                <div class="flex flex-col gap-5 sm:flex-row sm:items-start sm:justify-between">
                    <div>
                        <Brand compact=true />
                        <p class="mt-2 max-w-sm text-[10px] leading-relaxed text-slate-500">
                            "Clima real, en el momento. Sin datos simulados, sin claves de API y sin vueltas."
                        </p>
                    </div>

                    <div class="grid grid-cols-2 gap-x-8 gap-y-2 text-[9px] sm:grid-cols-3">
                        <a
                            href="https://open-meteo.com/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-slate-500 transition-colors hover:text-ice-300"
                        >
                            "Open-Meteo ↗"
                        </a>
                        <a
                            href="https://www.met.no/en"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-slate-500 transition-colors hover:text-ice-300"
                        >
                            "MET Norway ↗"
                        </a>
                        <a
                            href="https://www.openstreetmap.org/copyright"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-slate-500 transition-colors hover:text-ice-300"
                        >
                            "OpenStreetMap ↗"
                        </a>
                        <a
                            href="https://open-meteo.com/en/docs/air-quality-api"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-slate-500 transition-colors hover:text-ice-300"
                        >
                            "CAMS AQ ↗"
                        </a>
                        <a
                            href="https://leafletjs.com/"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-slate-500 transition-colors hover:text-ice-300"
                        >
                            "Leaflet ↗"
                        </a>
                        <button
                            type="button"
                            class="text-left text-slate-500 transition-colors hover:text-ice-300"
                            on:click=move |_| scroll_to_top()
                        >
                            "Volver arriba ↑"
                        </button>
                    </div>
                </div>

                <div class="flex flex-col gap-2 border-t border-white/[0.04] pt-4 text-[8.5px] text-slate-600 sm:flex-row sm:items-center sm:justify-between">
                    <p>
                        "© "
                        {move || js_sys::Date::new_0().get_full_year()}
                        " FedeWeather · Hecho con Rust, WebAssembly y mucho café."
                    </p>
                    <p class="flex flex-wrap items-center gap-x-2 gap-y-1">
                        <span class="inline-flex items-center gap-1">
                            <span class="h-1.5 w-1.5 rounded-full bg-emerald-400"></span>
                            "Datos en vivo"
                        </span>
                        <span class="text-slate-700">"·"</span>
                        <span>
                            {move || ctx
                                .weather
                                .get()
                                .map(|w| w.provider.model().to_string())
                                .unwrap_or_else(|| "Fuentes sin API key".to_string())}
                        </span>
                        <span class="text-slate-700">"·"</span>
                        <span>"Sin cookies de rastreo"</span>
                    </p>
                </div>
            </div>
        </footer>
    }
}
