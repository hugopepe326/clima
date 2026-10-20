use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon, WeatherIcon};
use crate::format::{day_label, long_date, uv_category, weekday_long, wind_description};
use crate::models::{condition, Day, Sky};
use crate::state::AppCtx;

#[component]
pub fn DailyForecast() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <section id="dias" class="scroll-mt-24 lg:col-span-2">
            <div class="card h-full p-4 sm:p-5">
                <div class="mb-4 flex items-start justify-between gap-3">
                    <div>
                        <h2 class="section-title">"Pronóstico de 7 días"</h2>
                        <p class="mt-1 text-[10px] text-slate-500">
                            {move || {
                                ctx.weather
                                    .get()
                                    .map(|w| w.provider.model().to_string())
                                    .unwrap_or_else(|| "Consultando modelos…".to_string())
                            }}
                        </p>
                    </div>
                    <span class="rounded-full border border-white/[0.06] bg-white/[0.03] px-2 py-1 text-[8.5px] font-medium uppercase tracking-[0.12em] text-slate-500">
                        {move || ctx.units.get().label()}
                    </span>
                </div>

                {move || {
                    let Some(weather) = ctx.weather.get() else {
                        return ().into_any();
                    };
                    let units = ctx.units.get();
                    let days = weather.daily.clone();
                    if days.is_empty() {
                        return ().into_any();
                    }
                    let selected = ctx.selected_day.get().min(days.len() - 1);
                    let week_min = days.iter().map(|d| d.temp_min).fold(f64::INFINITY, f64::min);
                    let week_max = days.iter().map(|d| d.temp_max).fold(f64::NEG_INFINITY, f64::max);
                    let range = (week_max - week_min).max(1.0);

                    let rows = days
                        .iter()
                        .enumerate()
                        .map(|(index, day)| {
                            day_row(ctx, units, day, index, selected, week_min, range)
                        })
                        .collect_view();

                    let detail = days.get(selected).map(|day| {
                        detail_panel(units, day, selected)
                    });

                    view! {
                        <div class="space-y-1">{rows}</div>
                        {detail}
                    }
                    .into_any()
                }}
            </div>
        </section>
    }
}

fn day_row(
    ctx: AppCtx,
    units: crate::models::Units,
    day: &Day,
    index: usize,
    selected: usize,
    week_min: f64,
    range: f64,
) -> AnyView {
    let info = condition(day.weather_code);
    let left = ((day.temp_min - week_min) / range * 100.0).clamp(0.0, 92.0);
    let width = ((day.temp_max - day.temp_min) / range * 100.0).clamp(8.0, 100.0 - left);
    let is_selected = index == selected;
    let label = day_label(&day.date, index);
    let probability = day
        .precipitation_probability
        .map(|p| format!("{p:.0}%"))
        .unwrap_or_default();
    let has_probability = !probability.is_empty();
    let bar = bar_gradient(info.sky);

    view! {
        <button
            type="button"
            class=format!(
                "focus-ring group flex w-full items-center gap-2 rounded-xl px-2 py-2 text-left transition-all sm:gap-3 sm:px-2.5 {}",
                if is_selected {
                    "bg-white/[0.065] ring-1 ring-white/[0.06]"
                } else {
                    "hover:bg-white/[0.035]"
                }
            )
            on:click=move |_| ctx.selected_day.set(index)
            aria-pressed=if is_selected { "true" } else { "false" }
        >
            <span class=format!(
                "w-12 shrink-0 text-[11px] font-medium sm:w-16 {}",
                if is_selected { "text-white" } else { "text-slate-300" }
            )>
                {label}
            </span>
            <span class="shrink-0 transition-transform duration-300 group-hover:scale-110" title=info.label>
                <WeatherIcon code=day.weather_code is_day=true class="h-7 w-7 sm:h-8 sm:w-8" />
            </span>
            <span class="w-8 shrink-0 text-right text-[10.5px] tabular-nums text-slate-500 sm:w-10">
                {units.temp_str(day.temp_min)}
            </span>
            <span class="relative h-1.5 flex-1 overflow-hidden rounded-full bg-white/[0.055]">
                <span
                    class=format!("absolute inset-y-0 rounded-full bg-gradient-to-r {bar}")
                    style=format!("left:{left:.1}%; width:{width:.1}%")
                ></span>
            </span>
            <span class="w-8 shrink-0 text-right text-[11px] font-medium tabular-nums text-slate-100 sm:w-10">
                {units.temp_str(day.temp_max)}
            </span>
            <span class=format!(
                "hidden w-9 shrink-0 items-center justify-end gap-0.5 text-[9px] tabular-nums sm:flex {}",
                if has_probability { "text-ice-300/70" } else { "text-transparent" }
            )>
                <Show when=move || has_probability>
                    <Icon kind=UiIcon::Drop class="h-2.5 w-2.5" />
                </Show>
                {probability}
            </span>
        </button>
    }
    .into_any()
}

fn detail_panel(units: crate::models::Units, day: &Day, selected: usize) -> AnyView {
    let info = condition(day.weather_code);
    let uv = day.uv_max.map(|v| format!("{v:.1}")).unwrap_or_else(|| "—".to_string());
    let uv_label = day.uv_max.map(|v| uv_category(v).0.to_string()).unwrap_or_else(|| "Sin datos".to_string());
    let precip = day
        .precipitation_sum
        .map(|v| format!("{v:.1} mm"))
        .unwrap_or_else(|| "—".to_string());
    let wind = day
        .wind_max
        .map(|v| format!("{v:.0} km/h"))
        .unwrap_or_else(|| "—".to_string());
    let wind_label = day
        .wind_max
        .map(|v| wind_description(v).to_string())
        .unwrap_or_else(|| "Sin datos".to_string());
    let apparent = day
        .apparent_max
        .map(|v| units.temp_str(v))
        .unwrap_or_else(|| "—".to_string());
    let title = if selected == 0 {
        "Hoy".to_string()
    } else {
        format!(
            "{} {}",
            capitalize(&weekday_long(&day.date)),
            day.date.get(8..10).unwrap_or("")
        )
    };
    let date_full = long_date(&day.date);

    view! {
        <div class="mt-3 animate-fade-up rounded-2xl border border-white/[0.055] bg-black/15 p-3.5">
            <div class="flex items-center justify-between gap-2">
                <div class="min-w-0">
                    <p class="text-[8.5px] font-semibold uppercase tracking-[0.18em] text-slate-500">
                        "Detalle · " {title}
                    </p>
                    <p class="mt-1 truncate text-[12px] font-light text-slate-200">
                        {info.label}
                        <span class="mx-1.5 text-slate-600">"·"</span>
                        <span class="text-slate-500">{date_full}</span>
                    </p>
                </div>
                <div class="flex items-center gap-1.5 rounded-full bg-white/[0.04] px-2.5 py-1">
                    <WeatherIcon code=day.weather_code is_day=true class="h-5 w-5" />
                    <span class="text-[10px] text-slate-300">
                        {units.temp_str(day.temp_min)}
                        <span class="text-slate-600">" / "</span>
                        {units.temp_str(day.temp_max)}
                    </span>
                </div>
            </div>

            <div class="mt-3 grid grid-cols-2 gap-2 sm:grid-cols-3 lg:grid-cols-6">
                <DetailItem icon=UiIcon::Sunrise label="Amanecer" value=day.sunrise.clone().unwrap_or_else(|| "—".to_string()) />
                <DetailItem icon=UiIcon::Sunset label="Atardecer" value=day.sunset.clone().unwrap_or_else(|| "—".to_string()) />
                <DetailItem icon=UiIcon::Sun label="UV máximo" value=format!("{uv} · {uv_label}") />
                <DetailItem icon=UiIcon::Umbrella label="Precipitación" value=precip />
                <DetailItem icon=UiIcon::Wind label="Viento máx." value=format!("{wind} · {wind_label}") />
                <DetailItem icon=UiIcon::Thermometer label="Sensación máx." value=apparent />
            </div>
        </div>
    }
    .into_any()
}

#[component]
fn DetailItem(#[prop(into)] label: String, #[prop(into)] value: String, icon: UiIcon) -> impl IntoView {
    view! {
        <div class="rounded-xl bg-white/[0.025] px-2.5 py-2">
            <div class="flex items-center gap-1 text-slate-600">
                <Icon kind=icon class="h-2.5 w-2.5" />
                <span class="text-[7.5px] font-medium uppercase tracking-[0.12em]">{label}</span>
            </div>
            <p class="mt-1 truncate text-[10px] text-slate-300" title=value.clone()>{value.clone()}</p>
        </div>
    }
}

fn bar_gradient(sky: Sky) -> &'static str {
    match sky {
        Sky::Clear | Sky::MostlyClear => "from-amber-300 via-orange-300 to-amber-200",
        Sky::PartlyCloudy => "from-sky-400 via-cyan-300 to-amber-200",
        Sky::Cloudy | Sky::Fog => "from-slate-400 via-sky-300 to-cyan-200",
        Sky::Rain | Sky::HeavRain | Sky::Showers | Sky::Drizzle => {
            "from-sky-500 via-ice-400 to-cyan-300"
        }
        Sky::Thunder => "from-violet-400 via-fuchsia-300 to-ice-300",
        Sky::Snow | Sky::Sleet => "from-cyan-200 via-sky-200 to-white",
    }
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}
