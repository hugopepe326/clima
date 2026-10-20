use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon};
use crate::components::switches::UnitsSwitch;
use crate::models::Provider;
use crate::state::AppCtx;

#[component]
pub fn Settings() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <section id="ajustes" class="scroll-mt-24 pb-2">
            <div class="card overflow-hidden">
                <div class="border-b border-white/[0.055] px-4 py-4 sm:px-5">
                    <div class="flex items-center justify-between gap-3">
                        <div>
                            <h2 class="section-title">"Ajustes y fuentes"</h2>
                            <p class="mt-1 text-[10px] text-slate-500">
                                "Controlá el proveedor, las unidades y tu ubicación"
                            </p>
                        </div>
                        <span class="grid h-9 w-9 place-items-center rounded-xl border border-white/[0.07] bg-white/[0.03] text-slate-500">
                            <Icon kind=UiIcon::Settings class="h-4 w-4" />
                        </span>
                    </div>
                </div>

                <div class="grid gap-4 p-4 sm:p-5 lg:grid-cols-2">
                    <div>
                        <p class="text-[9px] font-semibold uppercase tracking-[0.18em] text-slate-500">
                            "Proveedor meteorológico"
                        </p>
                        <div class="mt-2.5 grid gap-2 sm:grid-cols-2">
                            {Provider::ALL.into_iter().map(|provider| {
                                let active = move || ctx.provider.get() == provider;
                                view! {
                                    <button
                                        type="button"
                                        class=move || format!(
                                            "focus-ring relative rounded-2xl border p-3 text-left transition-all {}",
                                            if active() {
                                                "border-ice-400/25 bg-ice-400/[0.07]"
                                            } else {
                                                "border-white/[0.06] bg-white/[0.02] hover:border-white/[0.12] hover:bg-white/[0.04]"
                                            }
                                        )
                                        aria-pressed=move || active().to_string()
                                        on:click=move |_| ctx.provider.set(provider)
                                    >
                                        <div class="flex items-center justify-between">
                                            <span class="text-[12px] font-medium text-slate-100">{provider.label()}</span>
                                            <Show when=move || active() fallback=|| view! { <span class="h-4 w-4 rounded-full border border-white/10"></span> }>
                                                <span class="grid h-4 w-4 place-items-center rounded-full bg-ice-300 text-navy-950">
                                                    <Icon kind=UiIcon::Check class="h-2.5 w-2.5" />
                                                </span>
                                            </Show>
                                        </div>
                                        <p class="mt-1.5 text-[9px] leading-relaxed text-slate-500">
                                            {provider.description()}
                                        </p>
                                        <p class="mt-2 text-[8px] font-medium uppercase tracking-[0.1em] text-slate-600">
                                            {provider.model()}
                                        </p>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                        <p class="mt-2 flex items-center gap-1.5 text-[8.5px] text-slate-600">
                            <Icon kind=UiIcon::Check class="h-2.5 w-2.5 text-emerald-400" />
                            "Sin registro, sin API key y sin datos simulados."
                        </p>
                    </div>

                    <div class="grid gap-3 sm:grid-cols-2">
                        <div class="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-3">
                            <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-slate-500">"Unidades"</p>
                            <div class="mt-2"><UnitsSwitch /></div>
                            <p class="mt-2 text-[9px] leading-relaxed text-slate-600">
                                "Se guarda en tu dispositivo."
                            </p>
                        </div>
                        <div class="rounded-2xl border border-white/[0.06] bg-white/[0.02] p-3">
                            <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-slate-500">"Ubicación"</p>
                            <button
                                type="button"
                                class="focus-ring mt-2 flex w-full items-center justify-center gap-1.5 rounded-xl border border-ice-400/15 bg-ice-400/[0.07] px-2 py-2 text-[9.5px] font-medium text-ice-200 transition-colors hover:bg-ice-400/[0.11]"
                                on:click=move |_| ctx.use_my_location()
                                disabled=move || ctx.geo_loading.get()
                            >
                                <Icon kind=UiIcon::Locate class="h-3 w-3" />
                                {move || if ctx.geo_loading.get() { "Localizando…" } else { "Usar GPS" }}
                            </button>
                            <p class="mt-2 text-[9px] leading-relaxed text-slate-600">
                                "Requiere permiso del navegador y HTTPS."
                            </p>
                        </div>
                    </div>
                </div>

                <div class="border-t border-white/[0.055] px-4 py-4 sm:px-5">
                    <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                        <SourceLink
                            name="Open-Meteo"
                            detail="Pronóstico y geocodificación"
                            href="https://open-meteo.com/"
                        />
                        <SourceLink
                            name="MET Norway"
                            detail="Locationforecast 2.0"
                            href="https://www.met.no/en"
                        />
                        <SourceLink
                            name="OpenStreetMap"
                            detail="Mapas y geocodificación inversa"
                            href="https://www.openstreetmap.org/copyright"
                        />
                        <SourceLink
                            name="Open-Meteo CAMS"
                            detail="Calidad del aire europea"
                            href="https://open-meteo.com/en/docs/air-quality-api"
                        />
                    </div>
                </div>

                <div class="border-t border-white/[0.055] bg-black/10 px-4 py-3 sm:px-5">
                    <p class="text-[8.5px] leading-relaxed text-slate-600">
                        "FedeWeather no es un servicio meteorológico oficial. Los datos pertenecen a sus fuentes y se muestran con atribución. Open-Meteo y MET Norway publican bajo CC BY 4.0; OpenStreetMap bajo ODbL."
                    </p>
                    <p class="mt-1.5 flex flex-wrap items-center gap-x-2 gap-y-1 text-[8px] text-slate-700">
                        <span>"Rust + WebAssembly"</span>
                        <span>"·"</span>
                        <span>"Leptos"</span>
                        <span>"·"</span>
                        <span>"Trunk"</span>
                        <span>"·"</span>
                        <span>"Tailwind CSS"</span>
                        <span>"·"</span>
                        <span>"reqwest + web-sys"</span>
                        <span>"·"</span>
                        <span>"Leaflet"</span>
                    </p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SourceLink(
    #[prop(into)] name: String,
    #[prop(into)] detail: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            rel="noopener noreferrer"
            class="group flex items-center justify-between gap-2 rounded-xl border border-transparent px-1 py-1 transition-colors hover:border-white/[0.06] hover:bg-white/[0.02]"
        >
            <span>
                <span class="block text-[10px] font-medium text-slate-300 group-hover:text-white">{name}</span>
                <span class="mt-0.5 block text-[8.5px] text-slate-600">{detail}</span>
            </span>
            <Icon kind=UiIcon::External class="h-3 w-3 text-slate-700 transition-colors group-hover:text-ice-300" />
        </a>
    }
}
