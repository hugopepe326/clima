use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::components::icons::{Icon, UiIcon};
use crate::format::format_population;
use crate::models::Location;
use crate::state::AppCtx;

#[derive(Clone, Debug, PartialEq)]
enum SearchStatus {
    Idle,
    Loading,
    Ready,
    Empty,
    Error(String),
}

#[component]
pub fn SearchBox(
    #[prop(optional)] autofocus: bool,
    #[prop(optional)] mobile: bool,
) -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let query = RwSignal::new(String::new());
    let results = RwSignal::new(Vec::<Location>::new());
    let status = RwSignal::new(SearchStatus::Idle);
    let open = RwSignal::new(false);
    let generation = RwSignal::new(0u32);
    let active = RwSignal::new(0usize);
    let input_ref = NodeRef::<leptos::html::Input>::new();

    if autofocus {
        request_animation_frame(move || {
            if let Some(input) = input_ref.get_untracked() {
                let _ = input.focus();
            }
        });
    }

    Effect::new(move |_| {
        let raw = query.get();
        let trimmed = raw.trim().to_string();

        if trimmed.chars().count() < 2 {
            results.set(Vec::new());
            status.set(SearchStatus::Idle);
            if trimmed.is_empty() {
                open.set(false);
            }
            return;
        }

        let my_generation = generation.get_untracked().wrapping_add(1);
        generation.set(my_generation);
        status.set(SearchStatus::Loading);
        open.set(true);

        spawn_local(async move {
            gloo_timers::future::TimeoutFuture::new(320).await;
            if generation.get_untracked() != my_generation {
                return;
            }
            match api::search_locations(&trimmed).await {
                Ok(list) => {
                    if generation.get_untracked() != my_generation {
                        return;
                    }
                    if list.is_empty() {
                        results.set(Vec::new());
                        status.set(SearchStatus::Empty);
                    } else {
                        active.set(0);
                        results.set(list);
                        status.set(SearchStatus::Ready);
                    }
                    open.set(true);
                }
                Err(message) => {
                    if generation.get_untracked() == my_generation {
                        results.set(Vec::new());
                        status.set(SearchStatus::Error(message));
                        open.set(true);
                    }
                }
            }
        });
    });

    let choose = move |location: Location| {
        ctx.select_location(location);
        query.set(String::new());
        results.set(Vec::new());
        status.set(SearchStatus::Idle);
        open.set(false);
        active.set(0);
    };

    let on_keydown = move |event: web_sys::KeyboardEvent| {
        let count = results.get_untracked().len();
        match event.key().as_str() {
            "ArrowDown" if count > 0 => {
                event.prevent_default();
                active.update(|index| *index = (*index + 1) % count);
            }
            "ArrowUp" if count > 0 => {
                event.prevent_default();
                active.update(|index| *index = if *index == 0 { count - 1 } else { *index - 1 });
            }
            "Enter" if count > 0 => {
                event.prevent_default();
                let index = active.get_untracked().min(count - 1);
                if let Some(location) = results.get_untracked().get(index).cloned() {
                    choose(location);
                }
            }
            "Escape" => {
                open.set(false);
                if let Some(input) = input_ref.get_untracked() {
                    let _ = input.blur();
                }
            }
            _ => {}
        }
    };

    let clear = move |_| {
        query.set(String::new());
        results.set(Vec::new());
        status.set(SearchStatus::Idle);
        open.set(false);
        if let Some(input) = input_ref.get_untracked() {
            let _ = input.focus();
        }
    };

    let panel_class = if mobile {
        "relative z-50 mt-2 overflow-hidden rounded-2xl border border-white/[0.09] bg-[#0a1730]/95 shadow-glass backdrop-blur-2xl animate-fade-up"
    } else {
        "absolute left-0 right-0 top-[calc(100%+8px)] z-50 overflow-hidden rounded-2xl border border-white/[0.09] bg-[#0a1730]/95 shadow-glass backdrop-blur-2xl animate-fade-up"
    };

    view! {
        <div class=move || {
            if mobile {
                "relative w-full".to_string()
            } else {
                "relative w-full".to_string()
            }
        }>
            <div class="group relative">
                <span class="pointer-events-none absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-500 transition-colors group-focus-within:text-ice-300">
                    <Icon kind=UiIcon::Search class="h-4 w-4" />
                </span>
                <input
                    node_ref=input_ref
                    type="text"
                    class="focus-ring h-11 w-full rounded-2xl border border-white/[0.08] bg-white/[0.045] pl-10 pr-10 text-[13px] text-slate-100 shadow-inner shadow-black/20 outline-none transition-all placeholder:text-slate-500 hover:border-white/[0.14] focus:border-ice-400/40 focus:bg-white/[0.065]"
                    placeholder="Buscar ciudad o región…"
                    autocomplete="off"
                    spellcheck="false"
                    aria-label="Buscar ciudad"
                    aria-autocomplete="list"
                    aria-expanded=move || open.get().to_string()
                    prop:value=move || query.get()
                    on:input=move |event| {
                        query.set(event_target_value(&event));
                        open.set(true);
                    }
                    on:focus=move |_| {
                        if results.get_untracked().len() > 0 {
                            open.set(true);
                        }
                    }
                    on:keydown=on_keydown
                />
                <Show when=move || !query.get().trim().is_empty()>
                    <button
                        type="button"
                        class="focus-ring absolute right-2 top-1/2 grid h-7 w-7 -translate-y-1/2 place-items-center rounded-full text-slate-500 transition-colors hover:bg-white/5 hover:text-slate-200"
                        aria-label="Limpiar búsqueda"
                        on:click=clear
                    >
                        <Icon kind=UiIcon::Close class="h-3.5 w-3.5" />
                    </button>
                </Show>
            </div>

            <Show when=move || open.get()>
                <div
                    class="fixed inset-0 z-40 bg-navy-950/20"
                    on:click=move |_| open.set(false)
                ></div>
                <div class=panel_class>
                    <div class="flex items-center justify-between border-b border-white/[0.06] px-3.5 py-2.5">
                        <span class="text-[9px] font-semibold uppercase tracking-[0.2em] text-slate-500">
                            "Resultados"
                        </span>
                        <span class="text-[9px] text-slate-600">"Open-Meteo Geocoding"</span>
                    </div>

                    {move || match status.get() {
                        SearchStatus::Loading => view! {
                            <div class="space-y-1 p-2">
                                {(0..3).map(|_| view! {
                                    <div class="flex items-center gap-3 rounded-xl p-2.5">
                                        <div class="skeleton h-8 w-8 rounded-lg"></div>
                                        <div class="flex-1 space-y-1.5">
                                            <div class="skeleton h-2.5 w-28 rounded-full"></div>
                                            <div class="skeleton h-2 w-40 rounded-full"></div>
                                        </div>
                                    </div>
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        SearchStatus::Empty => view! {
                            <div class="px-4 py-6 text-center">
                                <p class="text-sm font-light text-slate-300">"Sin coincidencias"</p>
                                <p class="mt-1 text-[11px] text-slate-500">"Probá con otro nombre o agregá el país."</p>
                            </div>
                        }.into_any(),
                        SearchStatus::Error(message) => view! {
                            <div class="px-4 py-5">
                                <p class="text-sm font-light text-rose-300">"No se pudo buscar"</p>
                                <p class="mt-1 text-[11px] leading-relaxed text-slate-500">{message}</p>
                            </div>
                        }.into_any(),
                        SearchStatus::Ready => {
                            let items = results.get();
                            view! {
                                <div class="max-h-[320px] overflow-y-auto p-1.5">
                                    {items.into_iter().enumerate().map(|(index, location)| {
                                        let item = location.clone();
                                        let location_for_class = location.clone();
                                        let is_active = move || active.get() == index;
                                        let place_line = location.place_line();
                                        let pop = location.population.map(format_population).unwrap_or_default();
                                        let country_code = location
                                            .country_code
                                            .as_deref()
                                            .map(|code| code.to_uppercase())
                                            .unwrap_or_else(|| "··".to_string());
                                        view! {
                                            <button
                                                type="button"
                                                class=move || format!(
                                                    "group flex w-full items-center gap-3 rounded-xl px-2.5 py-2.5 text-left transition-colors {}",
                                                    if is_active() { "bg-white/[0.07]" } else { "hover:bg-white/[0.045]" }
                                                )
                                                on:click=move |_| choose(item.clone())
                                                on:mouseenter=move |_| active.set(index)
                                            >
                                                <span class="grid h-9 w-9 shrink-0 place-items-center rounded-lg border border-white/[0.06] bg-white/[0.035] text-[10px] font-medium tracking-[0.08em] text-slate-400">
                                                    {country_code}
                                                </span>
                                                <span class="min-w-0 flex-1">
                                                    <span class="block truncate text-[13px] font-medium text-slate-100">
                                                        {location_for_class.name.clone()}
                                                    </span>
                                                    <span class="mt-0.5 block truncate text-[10.5px] text-slate-500">
                                                        {place_line}
                                                    </span>
                                                </span>
                                                <span class="hidden shrink-0 text-right text-[9px] leading-tight text-slate-600 sm:block">
                                                    {pop}
                                                </span>
                                                <Icon kind=UiIcon::ChevronRight class="h-3.5 w-3.5 shrink-0 text-slate-600 transition-transform group-hover:translate-x-0.5 group-hover:text-ice-300" />
                                            </button>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                        SearchStatus::Idle => view! {
                            <div class="px-4 py-5 text-center text-[11px] leading-relaxed text-slate-500">
                                "Escribí al menos 2 letras. Resultados reales de Open-Meteo."
                            </div>
                        }.into_any(),
                    }}
                </div>
            </Show>

            <Show when=move || ctx.geo_loading.get()>
                <div class="mt-2 flex items-center gap-2 px-1 text-[10px] text-ice-300">
                    <span class="h-1.5 w-1.5 animate-pulse rounded-full bg-ice-300"></span>
                    "Obteniendo tu ubicación real…"
                </div>
            </Show>
        </div>
    }
}
