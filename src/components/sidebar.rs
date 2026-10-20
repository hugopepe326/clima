use leptos::prelude::*;

use crate::components::brand::Brand;
use crate::components::icons::{Icon, UiIcon};
use crate::components::search::SearchBox;
use crate::components::switches::{LiveBadge, ProviderSwitch, UnitsSwitch};
use crate::format::{flag_emoji, updated_label};
use crate::models::{condition, Location};
use crate::state::AppCtx;
use crate::storage;

#[component]
pub fn Sidebar() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    let remove_favorite = move |location: Location| {
        let mut favorites = ctx.favorites.get_untracked();
        favorites.retain(|f| !f.same_place(&location));
        ctx.favorites.set(favorites.clone());
        storage::save_favorites(&favorites);
    };

    view! {
        <aside class="fixed inset-y-0 left-0 z-40 hidden w-[292px] flex-col border-r border-white/[0.055] bg-navy-950/75 backdrop-blur-2xl lg:flex xl:w-[312px]">
            <div class="flex items-center justify-between px-5 pb-5 pt-6">
                <Brand />
                <LiveBadge />
            </div>

            <div class="px-4">
                <SearchBox />
                <button
                    type="button"
                    class="focus-ring mt-2 flex w-full items-center justify-center gap-2 rounded-2xl border border-ice-400/15 bg-ice-400/[0.07] px-3 py-2.5 text-[11px] font-medium text-ice-200 transition-all hover:border-ice-400/30 hover:bg-ice-400/[0.11] disabled:opacity-50"
                    on:click=move |_| ctx.use_my_location()
                    disabled=move || ctx.geo_loading.get()
                >
                    <span class=move || if ctx.geo_loading.get() { "animate-spin" } else { "" }>
                        <Icon kind=UiIcon::Locate class="h-4 w-4" />
                    </span>
                    {move || if ctx.geo_loading.get() { "Localizando…" } else { "Usar mi ubicación" }}
                </button>
            </div>

            <div class="mt-5 flex-1 overflow-y-auto px-4 pb-4">
                <NowCard />

                <div class="mt-6">
                    <div class="mb-2 flex items-center justify-between px-1">
                        <span class="section-title">"Favoritos"</span>
                        <span class="text-[9px] tabular-nums text-slate-600">
                            {move || format!("{}", ctx.favorites.get().len())}
                        </span>
                    </div>

                    {move || {
                        let favorites = ctx.favorites.get();
                        if favorites.is_empty() {
                            view! {
                                <div class="rounded-2xl border border-dashed border-white/[0.08] px-3.5 py-4">
                                    <p class="text-[11px] leading-relaxed text-slate-500">
                                        "Guardá ubicaciones con la estrella para acceder al instante."
                                    </p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-1">
                                    {favorites.into_iter().map(|location| {
                                        let item = location.clone();
                                        let removal = location.clone();
                                        let active = {
                                            let location = location.clone();
                                            move || ctx.location.get().same_place(&location)
                                        };
                                        let flag = flag_emoji(location.country_code.as_deref());
                                        view! {
                                            <div class=move || format!(
                                                "group flex items-center gap-2 rounded-xl border px-2.5 py-2 transition-all {}",
                                                if active() {
                                                    "border-ice-400/15 bg-white/[0.055]"
                                                } else {
                                                    "border-transparent hover:bg-white/[0.035]"
                                                }
                                            )>
                                                <button
                                                    type="button"
                                                    class="flex min-w-0 flex-1 items-center gap-2.5 text-left"
                                                    on:click=move |_| ctx.select_location(item.clone())
                                                >
                                                    <span class="grid h-7 w-7 shrink-0 place-items-center rounded-lg bg-white/[0.04] text-sm">
                                                        {flag}
                                                    </span>
                                                    <span class="min-w-0">
                                                        <span class="block truncate text-[12px] font-medium text-slate-200">
                                                            {location.name.clone()}
                                                        </span>
                                                        <span class="block truncate text-[9.5px] text-slate-500">
                                                            {location.place_line()}
                                                        </span>
                                                    </span>
                                                </button>
                                                <button
                                                    type="button"
                                                    class="focus-ring grid h-6 w-6 shrink-0 place-items-center rounded-md text-slate-600 opacity-0 transition-all hover:bg-white/5 hover:text-rose-300 group-hover:opacity-100"
                                                    aria-label="Quitar de favoritos"
                                                    on:click=move |_| remove_favorite(removal.clone())
                                                >
                                                    <Icon kind=UiIcon::Close class="h-3 w-3" />
                                                </button>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}

                    <div class="mb-2 mt-6 flex items-center justify-between px-1">
                        <span class="section-title">"Recientes"</span>
                    </div>

                    {move || {
                        let current = ctx.location.get();
                        let favorites = ctx.favorites.get();
                        let recents: Vec<Location> = ctx
                            .recents
                            .get()
                            .into_iter()
                            .filter(|r| !r.same_place(&current) && !favorites.iter().any(|f| f.same_place(r)))
                            .take(4)
                            .collect();

                        if recents.is_empty() {
                            view! {
                                <p class="px-1 text-[10px] leading-relaxed text-slate-600">
                                    "Las búsquedas y ubicaciones GPS aparecerán acá."
                                </p>
                            }.into_any()
                        } else {
                            view! {
                                <div class="space-y-0.5">
                                    {recents.into_iter().map(|location| {
                                        let item = location.clone();
                                        let flag = flag_emoji(location.country_code.as_deref());
                                        view! {
                                            <button
                                                type="button"
                                                class="focus-ring flex w-full items-center gap-2.5 rounded-lg px-2.5 py-1.5 text-left transition-colors hover:bg-white/[0.035]"
                                                on:click=move |_| ctx.select_location(item.clone())
                                            >
                                                <span class="grid h-6 w-6 place-items-center rounded-md bg-white/[0.03] text-xs">
                                                    {flag}
                                                </span>
                                                <span class="min-w-0 flex-1">
                                                    <span class="block truncate text-[11px] text-slate-300">{location.name.clone()}</span>
                                                </span>
                                                <Icon kind=UiIcon::ChevronRight class="h-3 w-3 text-slate-700" />
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                <div class="mt-6 rounded-2xl border border-white/[0.06] bg-white/[0.025] p-3">
                    <div class="mb-2 flex items-center justify-between">
                        <span class="section-title">"Fuente de datos"</span>
                        <Icon kind=UiIcon::Layers class="h-3.5 w-3.5 text-slate-600" />
                    </div>
                    <ProviderSwitch />
                    <p class="mt-2 text-[9.5px] leading-relaxed text-slate-500">
                        {move || ctx.provider.get().description()}
                    </p>
                </div>
            </div>

            <div class="border-t border-white/[0.055] p-4">
                <div class="flex items-center justify-between">
                    <span class="text-[9px] font-semibold uppercase tracking-[0.18em] text-slate-600">
                        "Unidades"
                    </span>
                    <UnitsSwitch />
                </div>

                <div class="mt-3 flex items-center justify-between rounded-xl bg-white/[0.025] px-2.5 py-2">
                    <div class="flex items-center gap-2">
                        <span class="relative flex h-2 w-2">
                            <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-emerald-400 opacity-30"></span>
                            <span class="relative inline-flex h-2 w-2 rounded-full bg-emerald-400"></span>
                        </span>
                        <span class="text-[9.5px] text-slate-400">"Datos reales sin claves"</span>
                    </div>
                    <button
                        type="button"
                        class="focus-ring grid h-7 w-7 place-items-center rounded-lg text-slate-500 transition-colors hover:bg-white/[0.06] hover:text-white"
                        aria-label="Actualizar datos"
                        title="Actualizar datos"
                        on:click=move |_| ctx.refresh_now()
                    >
                        <span class=move || if ctx.loading.get() { "block animate-spin" } else { "block" }>
                            <Icon kind=UiIcon::Refresh class="h-3.5 w-3.5" />
                        </span>
                    </button>
                </div>
                <p class="mt-2.5 px-1 text-[8.5px] leading-relaxed text-slate-600">
                    "Open-Meteo · MET Norway · OpenStreetMap"
                </p>
            </div>
        </aside>
    }
}

#[component]
fn NowCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <div class="relative overflow-hidden rounded-2xl border border-white/[0.07] bg-gradient-to-br from-white/[0.055] to-white/[0.02] p-3.5">
            <div class="pointer-events-none absolute -right-8 -top-10 h-24 w-24 rounded-full bg-ice-400/10 blur-2xl"></div>
            <div class="relative flex items-start justify-between">
                <div class="min-w-0">
                    <p class="text-[8.5px] font-semibold uppercase tracking-[0.2em] text-ice-300/70">
                        {move || if ctx.location.get().is_current { "Mi ubicación" } else { "Ubicación actual" }}
                    </p>
                    <p class="mt-1 truncate font-display text-[15px] font-medium text-white">
                        {move || ctx.location.get().name}
                    </p>
                    <p class="truncate text-[9px] text-slate-500">
                        {move || ctx.location.get().place_line()}
                    </p>
                </div>
                <button
                    type="button"
                    class=move || format!(
                        "focus-ring grid h-7 w-7 shrink-0 place-items-center rounded-full transition-colors {}",
                        if ctx.is_favorite() {
                            "bg-amber-300/10 text-amber-300"
                        } else {
                            "text-slate-600 hover:bg-white/5 hover:text-amber-200"
                        }
                    )
                    aria-label="Guardar en favoritos"
                    on:click=move |_| ctx.toggle_favorite()
                >
                    <Show
                        when=move || ctx.is_favorite()
                        fallback=|| view! { <Icon kind=UiIcon::Star class="h-3.5 w-3.5" /> }
                    >
                        <Icon kind=UiIcon::StarFilled class="h-3.5 w-3.5" />
                    </Show>
                </button>
            </div>
            <div class="relative mt-3 flex items-end justify-between gap-2">
                <div>
                    <div class="flex items-start">
                        <span class="num text-4xl font-extralight leading-none tracking-[-0.05em] text-white">
                            {move || ctx.weather.get().map(|w| ctx.units.get().temp_str(w.current.temperature)).unwrap_or_else(|| "--°".to_string())}
                        </span>
                    </div>
                    <p class="mt-1 line-clamp-1 text-[10px] text-slate-400">
                        {move || ctx.weather.get().map(|w| condition(w.current.weather_code).label.to_string()).unwrap_or_else(|| "Consultando…".to_string())}
                    </p>
                </div>
                <div class="text-right">
                    <p class="text-[9px] text-slate-600">
                        {move || ctx.weather.get().map(|w| updated_label(w.fetched_at_ms, ctx.now_ms.get())).unwrap_or_default()}
                    </p>
                    <p class="mt-0.5 text-[8.5px] text-slate-600">
                        {move || ctx.weather.get().map(|w| w.provider.short().to_string()).unwrap_or_default()}
                    </p>
                </div>
            </div>
        </div>
    }
}
