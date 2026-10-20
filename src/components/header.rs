use leptos::prelude::*;

use crate::components::brand::Brand;
use crate::components::icons::{Icon, UiIcon};
use crate::components::search::SearchBox;
use crate::components::switches::{LiveBadge, ProviderSwitch, UnitsSwitch};
use crate::format::long_date;
use crate::state::AppCtx;

#[component]
pub fn Header() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <header class="sticky top-0 z-30 border-b border-white/[0.05] bg-navy-950/70 backdrop-blur-2xl">
            <div class="mx-auto flex h-[68px] max-w-[1560px] items-center gap-2 px-4 sm:px-6 lg:px-8">
                <div class="lg:hidden">
                    <Brand compact=true />
                </div>

                <div class="hidden min-w-0 lg:block">
                    <p class="text-[8.5px] font-semibold uppercase tracking-[0.22em] text-slate-600">
                        "Panel meteorológico"
                    </p>
                    <div class="mt-0.5 flex items-baseline gap-2.5">
                        <span class="max-w-[260px] truncate font-display text-[15px] font-medium tracking-[-0.01em] text-white">
                            {move || ctx.location.get().name}
                        </span>
                        <span class="h-1 w-1 rounded-full bg-slate-700"></span>
                        <span class="truncate text-[10px] text-slate-500">
                            {move || ctx
                                .weather
                                .get()
                                .map(|w| long_date(&w.current.date))
                                .unwrap_or_else(|| "Conectando con las fuentes…".to_string())}
                        </span>
                    </div>
                </div>

                <div class="ml-auto flex items-center gap-1.5 sm:gap-2">
                    <div class="hidden sm:block">
                        <LiveBadge />
                    </div>
                    <div class="hidden md:block">
                        <ProviderSwitch compact=true />
                    </div>
                    <div class="hidden sm:block">
                        <UnitsSwitch />
                    </div>

                    <button
                        type="button"
                        class="focus-ring grid h-9 w-9 place-items-center rounded-xl border border-white/[0.07] bg-white/[0.035] text-slate-400 transition-all hover:border-white/[0.14] hover:bg-white/[0.07] hover:text-white"
                        aria-label="Actualizar datos meteorológicos"
                        title="Actualizar datos"
                        on:click=move |_| ctx.refresh_now()
                    >
                        <span class=move || if ctx.loading.get() { "block animate-spin" } else { "block" }>
                            <Icon kind=UiIcon::Refresh class="h-4 w-4" />
                        </span>
                    </button>

                    <button
                        type="button"
                        class="focus-ring grid h-9 w-9 place-items-center rounded-xl border border-white/[0.07] bg-white/[0.035] text-slate-400 transition-all hover:border-white/[0.14] hover:bg-white/[0.07] hover:text-white lg:hidden"
                        aria-label="Buscar ciudad"
                        title="Buscar ciudad"
                        on:click=move |_| ctx.mobile_search.set(true)
                    >
                        <Icon kind=UiIcon::Search class="h-4 w-4" />
                    </button>

                    <button
                        type="button"
                        class="focus-ring grid h-9 w-9 place-items-center rounded-xl border border-ice-400/15 bg-ice-400/[0.07] text-ice-200 transition-all hover:border-ice-400/30 hover:bg-ice-400/[0.12] lg:hidden"
                        aria-label="Usar mi ubicación"
                        title="Usar mi ubicación"
                        on:click=move |_| ctx.use_my_location()
                        disabled=move || ctx.geo_loading.get()
                    >
                        <span class=move || if ctx.geo_loading.get() { "block animate-spin" } else { "block" }>
                            <Icon kind=UiIcon::Locate class="h-4 w-4" />
                        </span>
                    </button>
                </div>
            </div>
        </header>

        <Show when=move || ctx.mobile_search.get()>
            <div class="fixed inset-0 z-[80] lg:hidden">
                <div
                    class="absolute inset-0 bg-navy-950/80 backdrop-blur-md"
                    on:click=move |_| ctx.mobile_search.set(false)
                ></div>
                <div class="safe-top relative mx-auto mt-3 w-[calc(100%-1.5rem)] max-w-xl animate-fade-up rounded-3xl border border-white/[0.09] bg-[#08142b] p-4 shadow-glass">
                    <div class="mb-3 flex items-center justify-between">
                        <div>
                            <p class="text-[9px] font-semibold uppercase tracking-[0.22em] text-ice-300/70">
                                "Buscar lugar"
                            </p>
                            <p class="mt-0.5 text-[10px] text-slate-500">"Resultados reales de Open-Meteo"</p>
                        </div>
                        <button
                            type="button"
                            class="focus-ring grid h-8 w-8 place-items-center rounded-full bg-white/[0.05] text-slate-400 hover:text-white"
                            aria-label="Cerrar buscador"
                            on:click=move |_| ctx.mobile_search.set(false)
                        >
                            <Icon kind=UiIcon::Close class="h-4 w-4" />
                        </button>
                    </div>
                    <SearchBox mobile=true autofocus=true />
                    <button
                        type="button"
                        class="focus-ring mt-2 flex w-full items-center justify-center gap-2 rounded-2xl border border-ice-400/15 bg-ice-400/[0.07] px-3 py-2.5 text-[11px] font-medium text-ice-200 transition-all hover:bg-ice-400/[0.11]"
                        on:click=move |_| ctx.use_my_location()
                    >
                        <Icon kind=UiIcon::Locate class="h-4 w-4" />
                        "Usar mi ubicación GPS"
                    </button>
                </div>
            </div>
        </Show>
    }
}
