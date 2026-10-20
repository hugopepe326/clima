use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon, WeatherIcon};
use crate::components::switches::UnitsSwitch;
use crate::format::{
    hero_glow, hero_gradient, humidity_description, updated_label, uv_category, wind_description,
    wind_direction_label,
};
use crate::models::{condition, Provider, Weather};
use crate::state::AppCtx;

#[component]
pub fn Hero() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <section id="inicio" class="scroll-mt-24">
            {move || {
                ctx.weather.get().map(|w| hero_view(ctx, &w))
            }}
        </section>
    }
}

fn hero_view(ctx: AppCtx, weather: &Weather) -> AnyView {
    let units = ctx.units.get();
    let location = ctx.location.get();
    let current = weather.current.clone();
    let daily = weather.daily.clone();
    let provider = weather.provider;
    let info = condition(current.weather_code);
    let gradient = hero_gradient(info.sky, current.is_day);
    let glow = hero_glow(info.sky, current.is_day);
    let today = daily.first().cloned();
    let favorite = ctx.is_favorite();
    let units_label = units.label().to_string();
    let unit_symbol = units.symbol().to_string();

    let temp = units.temp_str(current.temperature);
    let feels = current
        .apparent
        .map(|v| units.temp_str(v))
        .unwrap_or_else(|| "—".to_string());
    let high = today.as_ref().map(|d| units.temp_str(d.temp_max)).unwrap_or_else(|| "—".to_string());
    let low = today.as_ref().map(|d| units.temp_str(d.temp_min)).unwrap_or_else(|| "—".to_string());

    let wind_value = current
        .wind_speed
        .map(|v| format!("{v:.0} km/h"))
        .unwrap_or_else(|| "—".to_string());
    let wind_sub = current
        .wind_speed
        .map(|v| {
            let dir = current
                .wind_direction
                .map(wind_direction_label)
                .unwrap_or("—");
            format!("{} · {}", dir, wind_description(v))
        })
        .unwrap_or_else(|| "Sin datos de viento".to_string());

    let humidity_value = current
        .humidity
        .map(|v| format!("{v:.0}%"))
        .unwrap_or_else(|| "—".to_string());
    let humidity_sub = current
        .humidity
        .map(|v| humidity_description(v).to_string())
        .unwrap_or_else(|| "Sin datos".to_string());

    let precip_value = current
        .precipitation_probability
        .map(|v| format!("{v:.0}%"))
        .unwrap_or_else(|| {
            if current.precipitation > 0.0 {
                format!("{:.1} mm", current.precipitation)
            } else {
                "0%".to_string()
            }
        });
    let precip_sub = if current.precipitation > 0.0 {
        format!("{:.1} mm ahora", current.precipitation)
    } else {
        "Sin precipitación ahora".to_string()
    };

    let uv_value = current
        .uv_index
        .map(|v| format!("{v:.1}"))
        .unwrap_or_else(|| "—".to_string());
    let uv_sub = current
        .uv_index
        .map(|v| uv_category(v).0.to_string())
        .unwrap_or_else(|| "Sin datos".to_string());

    let updated = updated_label(weather.fetched_at_ms, ctx.now_ms.get());
    let date = crate::format::long_date(&current.date);
    let accuracy = location.accuracy.map(|a| format!("±{a:.0} m")).unwrap_or_default();
    let provider_hint = match provider {
        Provider::OpenMeteo => "Cambiar a MET Norway",
        Provider::MetNo => "Cambiar a Open-Meteo",
    };
    let place_line = location.place_line();

    view! {
        <div class=format!("atmosphere relative overflow-hidden rounded-[1.75rem] border border-white/[0.08] bg-gradient-to-br {gradient} p-5 shadow-glass sm:rounded-[2rem] sm:p-7 lg:p-9")>
            <div class=format!("pointer-events-none absolute -right-24 -top-28 h-80 w-80 rounded-full blur-3xl {glow}")></div>
            <div class="pointer-events-none absolute -bottom-32 -left-20 h-64 w-64 rounded-full bg-white/[0.04] blur-3xl"></div>
            <div class="pointer-events-none absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-white/20 to-transparent"></div>

            <div class="relative z-10">
                <div class="flex items-start justify-between gap-3">
                    <div class="min-w-0">
                        <p class="flex items-center gap-1.5 text-[9px] font-semibold uppercase tracking-[0.22em] text-white/55">
                            <span class="h-1 w-1 rounded-full bg-ice-300"></span>
                            {if location.is_current { "Mi ubicación · GPS" } else { "Ubicación seleccionada" }}
                            {(!accuracy.is_empty()).then(|| view! {
                                <span class="rounded-full bg-black/20 px-1.5 py-0.5 text-[8px] normal-case tracking-normal text-white/45">
                                    {accuracy}
                                </span>
                            })}
                        </p>
                        <div class="mt-1.5 flex items-center gap-2">
                            <h2 class="truncate font-display text-2xl font-light tracking-[-0.03em] text-white sm:text-3xl">
                                {location.name.clone()}
                            </h2>
                            <button
                                type="button"
                                class=format!(
                                    "focus-ring grid h-7 w-7 shrink-0 place-items-center rounded-full transition-colors {}",
                                    if favorite { "bg-amber-300/15 text-amber-300" } else { "bg-white/[0.05] text-white/45 hover:text-amber-200" }
                                )
                                aria-label="Alternar favorito"
                                title="Guardar en favoritos"
                                on:click=move |_| ctx.toggle_favorite()
                            >
                                <Show when=move || ctx.is_favorite() fallback=|| view! { <Icon kind=UiIcon::Star class="h-3.5 w-3.5" /> }>
                                    <Icon kind=UiIcon::StarFilled class="h-3.5 w-3.5" />
                                </Show>
                            </button>
                        </div>
                        <p class="mt-1 truncate text-[11px] text-white/50 sm:text-xs">
                            {place_line}
                            <span class="mx-1.5 text-white/20">"·"</span>
                            <span class="text-white/40">{date}</span>
                        </p>
                    </div>

                    <div class="flex shrink-0 flex-col items-end gap-2">
                        <button
                            type="button"
                            class="focus-ring inline-flex items-center gap-1.5 rounded-full border border-white/15 bg-black/20 px-2.5 py-1.5 text-[9px] font-medium text-white/75 backdrop-blur-md transition-all hover:border-white/25 hover:bg-black/30 hover:text-white"
                            title=provider_hint
                            aria-label=provider_hint
                            on:click=move |_| {
                                ctx.provider.update(|value| {
                                    *value = match value {
                                        Provider::OpenMeteo => Provider::MetNo,
                                        Provider::MetNo => Provider::OpenMeteo,
                                    }
                                });
                            }
                        >
                            <span class="h-1.5 w-1.5 rounded-full bg-ice-300"></span>
                            {provider.short()}
                            <Icon kind=UiIcon::Refresh class="h-2.5 w-2.5 opacity-60" />
                        </button>
                        <p class="text-[9px] text-white/40">
                            <span class="hidden sm:inline">"Actualizado "</span>
                            {updated}
                        </p>
                        <div class="sm:hidden">
                            <UnitsSwitch />
                        </div>
                    </div>
                </div>

                <div class="mt-6 grid items-center gap-2 sm:mt-8 lg:grid-cols-[1fr_auto]">
                    <div class="min-w-0">
                        <div class="flex items-start">
                            <span class="num font-display text-[5.5rem] font-extralight leading-[0.82] tracking-[-0.07em] text-white drop-shadow-[0_10px_30px_rgba(0,0,0,0.25)] sm:text-[7.5rem] lg:text-[8.5rem]">
                                {temp}
                            </span>
                            <span class="ml-1 mt-2 text-lg font-light text-white/45 sm:mt-3 sm:text-xl">
                                {unit_symbol}
                            </span>
                        </div>
                        <div class="mt-4 flex flex-wrap items-center gap-x-3 gap-y-1">
                            <p class="text-xl font-light tracking-[-0.01em] text-white sm:text-2xl">
                                {info.label}
                            </p>
                            <span class="hidden h-4 w-px bg-white/20 sm:block"></span>
                            <p class="text-[11px] text-white/55 sm:text-xs">
                                "Sensación de "
                                <span class="font-medium text-white/85">{feels}</span>
                            </p>
                        </div>
                        <p class="mt-1.5 text-[11px] text-white/45">
                            "Máx "
                            <span class="text-white/75">{high}</span>
                            <span class="mx-2 text-white/20">"·"</span>
                            "Mín "
                            <span class="text-white/75">{low}</span>
                            <span class="mx-2 hidden text-white/20 sm:inline">"·"</span>
                            <span class="hidden text-white/35 sm:inline">{units_label.clone()}</span>
                        </p>
                    </div>

                    <div class="relative mx-auto flex h-36 w-36 items-center justify-center sm:h-48 sm:w-48 lg:mx-4 lg:h-56 lg:w-56">
                        <div class=format!("absolute inset-4 rounded-full blur-3xl {glow}")></div>
                        <WeatherIcon
                            code=current.weather_code
                            is_day=current.is_day
                            class="relative h-32 w-32 sm:h-44 sm:w-44 lg:h-52 lg:w-52"
                            animated=true
                        />
                    </div>
                </div>

                <div class="mt-6 grid grid-cols-2 gap-2 sm:mt-8 sm:grid-cols-4">
                    <StatChip label="Viento" value=wind_value sub=wind_sub icon=UiIcon::Wind />
                    <StatChip label="Humedad" value=humidity_value sub=humidity_sub icon=UiIcon::Humidity />
                    <StatChip label="Precipitación" value=precip_value sub=precip_sub icon=UiIcon::Umbrella />
                    <StatChip label="Índice UV" value=uv_value sub=uv_sub icon=UiIcon::Sun />
                </div>
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn StatChip(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into)] sub: String,
    icon: UiIcon,
) -> impl IntoView {
    view! {
        <div class="rounded-2xl border border-white/[0.08] bg-black/15 px-3 py-2.5 backdrop-blur-md transition-colors hover:bg-black/25 sm:px-3.5 sm:py-3">
            <div class="flex items-center gap-1.5 text-white/45">
                <Icon kind=icon class="h-3 w-3" />
                <span class="text-[8.5px] font-medium uppercase tracking-[0.14em]">{label}</span>
            </div>
            <p class="mt-1.5 num text-[15px] font-light text-white sm:text-base">{value}</p>
            <p class="mt-0.5 line-clamp-1 text-[9px] text-white/40">{sub}</p>
        </div>
    }
}
