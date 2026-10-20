use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon};
use crate::components::switches::{ProviderSwitch, UnitsSwitch};
use crate::models::Provider;
use crate::state::AppCtx;

#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <div class="space-y-4 sm:space-y-5 animate-fade-up">
            <div class="skeleton h-[420px] rounded-[2rem] sm:h-[460px]"></div>

            <div>
                <div class="mb-3 px-1">
                    <div class="skeleton h-2.5 w-40 rounded-full"></div>
                    <div class="skeleton mt-2 h-2 w-64 rounded-full"></div>
                </div>
                <div class="card flex gap-2 overflow-hidden p-2">
                    {(0..10).map(|_| view! {
                        <div class="skeleton h-[126px] min-w-[76px] flex-1 rounded-2xl"></div>
                    }).collect_view()}
                </div>
            </div>

            <div class="grid gap-3 lg:grid-cols-5">
                <div class="skeleton h-[390px] rounded-3xl lg:col-span-2"></div>
                <div class="skeleton h-[390px] rounded-3xl lg:col-span-3"></div>
            </div>

            <div class="grid grid-cols-2 gap-3 lg:grid-cols-4">
                {(0..8).map(|_| view! {
                    <div class="skeleton h-[150px] rounded-3xl"></div>
                }).collect_view()}
            </div>

            <div class="skeleton h-[420px] rounded-3xl"></div>
        </div>
    }
}

#[component]
pub fn LoadingOrError() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <Show
            when=move || ctx.loading.get()
            fallback=move || view! { <ErrorState /> }
        >
            <PageSkeleton />
        </Show>
    }
}

#[component]
pub fn ErrorState() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let message = ctx
        .error
        .get()
        .unwrap_or_else(|| "No se pudo contactar a la fuente de datos.".to_string());

    view! {
        <div class="animate-fade-up rounded-[1.75rem] border border-white/[0.08] bg-gradient-to-br from-[#13253d] via-[#0d1c32] to-[#081122] p-6 shadow-glass sm:p-10">
            <div class="mx-auto max-w-2xl text-center">
                <span class="mx-auto grid h-14 w-14 place-items-center rounded-2xl border border-rose-400/20 bg-rose-400/[0.08] text-rose-300">
                    <Icon kind=UiIcon::Alert class="h-6 w-6" />
                </span>
                <h2 class="mt-4 font-display text-2xl font-light tracking-[-0.02em] text-white">
                    "No pudimos obtener datos reales"
                </h2>
                <p class="mt-2 text-[12px] leading-relaxed text-slate-400">
                    "FedeWeather no inventa información: si la fuente falla, te lo dice. "
                    {message}
                </p>
                <div class="mt-4 flex items-center justify-center gap-2">
                    <span class="text-[10px] text-slate-500">"Ubicación:"</span>
                    <span class="text-[11px] text-slate-300">
                        {move || ctx.location.get().name}
                        <span class="mx-1 text-slate-600">"·"</span>
                        {move || ctx.location.get().place_line()}
                    </span>
                </div>

                <div class="mt-6 flex flex-wrap items-center justify-center gap-2">
                    <button
                        type="button"
                        class="focus-ring inline-flex items-center gap-2 rounded-xl bg-ice-400 px-4 py-2.5 text-[11px] font-semibold text-navy-950 transition-all hover:bg-ice-300"
                        on:click=move |_| {
                            if ctx.loading.get_untracked() {
                                return;
                            }
                            ctx.refresh_now();
                        }
                    >
                        <span class=move || if ctx.loading.get() { "animate-spin" } else { "" }>
                            <Icon kind=UiIcon::Refresh class="h-3.5 w-3.5" />
                        </span>
                        "Reintentar ahora"
                    </button>
                    <button
                        type="button"
                        class="focus-ring inline-flex items-center gap-2 rounded-xl border border-white/[0.1] bg-white/[0.04] px-4 py-2.5 text-[11px] font-medium text-slate-200 transition-colors hover:bg-white/[0.07]"
                        on:click=move |_| {
                            ctx.provider.update(|provider| {
                                *provider = match provider {
                                    Provider::OpenMeteo => Provider::MetNo,
                                    Provider::MetNo => Provider::OpenMeteo,
                                };
                            });
                        }
                    >
                        <Icon kind=UiIcon::Layers class="h-3.5 w-3.5" />
                        "Probar la otra fuente"
                    </button>
                </div>

                <div class="mt-6 flex flex-wrap items-center justify-center gap-3">
                    <ProviderSwitch compact=true />
                    <UnitsSwitch />
                </div>

                <div class="mt-6 rounded-2xl border border-white/[0.06] bg-black/15 p-3 text-left">
                    <p class="text-[9px] font-semibold uppercase tracking-[0.16em] text-slate-500">
                        "Qué podés revisar"
                    </p>
                    <ul class="mt-2 space-y-1.5 text-[10px] leading-relaxed text-slate-500">
                        <li class="flex gap-2"><span class="text-slate-700">"01"</span>"Conexión a internet y bloqueadores de contenido (CORS)."</li>
                        <li class="flex gap-2"><span class="text-slate-700">"02"</span>"Que el sitio se sirva por HTTPS o localhost para usar el GPS."</li>
                        <li class="flex gap-2"><span class="text-slate-700">"03"</span>"Cambiar de proveedor: Open-Meteo o MET Norway responden distinto en cada región."</li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn StaleBanner() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <Show when=move || ctx.error.get().is_some() && ctx.weather.get().is_some()>
            {move || {
                ctx.error.get().map(|message| view! {
                    <div class="flex items-start gap-3 rounded-2xl border border-amber-400/15 bg-amber-400/[0.055] px-3.5 py-3">
                        <span class="mt-0.5 shrink-0 text-amber-300">
                            <Icon kind=UiIcon::Alert class="h-4 w-4" />
                        </span>
                        <div class="min-w-0 flex-1">
                            <p class="text-[10px] font-medium text-amber-200">"No se pudo actualizar. Se muestran los últimos datos reales recibidos."</p>
                            <p class="mt-0.5 text-[9px] leading-relaxed text-amber-100/50">{message}</p>
                        </div>
                        <button
                            type="button"
                            class="focus-ring shrink-0 rounded-lg bg-amber-400/10 px-2.5 py-1.5 text-[9px] font-medium text-amber-200 hover:bg-amber-400/15"
                            on:click=move |_| {
                                ctx.error.set(None);
                                ctx.refresh_now();
                            }
                        >
                            "Reintentar"
                        </button>
                        <button
                            type="button"
                            class="grid h-6 w-6 shrink-0 place-items-center rounded-md text-amber-200/60 hover:bg-white/5 hover:text-amber-100"
                            aria-label="Ocultar aviso"
                            on:click=move |_| ctx.error.set(None)
                        >
                            <Icon kind=UiIcon::Close class="h-3 w-3" />
                        </button>
                    </div>
                })
            }}
        </Show>
    }
}
