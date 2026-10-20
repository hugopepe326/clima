use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon};
use crate::format::{
    aqi_category, humidity_description, moon_phase, pressure_status, pressure_trend,
    short_time, sun_arc_position, uv_category, visibility_description, wind_description,
    wind_direction_label,
};
use crate::models::AirQuality;
use crate::state::AppCtx;

#[component]
pub fn Metrics() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <section id="detalles" class="scroll-mt-24">
            <div class="mb-3 flex items-end justify-between gap-3 px-1">
                <div>
                    <h2 class="section-title">"Condiciones actuales"</h2>
                    <p class="mt-1 text-[10px] text-slate-500">"Todos los valores son observaciones y pronóstico en vivo"</p>
                </div>
                <span class="hidden text-[9px] text-slate-600 sm:block">
                    {move || ctx.weather.get().map(|w| w.provider.short().to_string()).unwrap_or_default()}
                </span>
            </div>

            <div class="grid grid-cols-2 gap-2.5 sm:gap-3 lg:grid-cols-4">
                <WindCard />
                <HumidityCard />
                <FeelsCard />
                <UvCard />
                <SunCard />
                <MoonCard />
                <PressureCard />
                <PrecipCard />
                <VisibilityCard />
                {move || ctx.air.get().map(|_| view! { <AqiCard /> })}
            </div>
        </section>
    }
}

#[component]
fn CardShell(
    #[prop(into)] title: String,
    icon: UiIcon,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let class = format!("card card-hover p-4 sm:p-5 {class}");
    view! {
        <div class=class>
            <div class="flex items-center justify-between gap-2">
                <h3 class="text-[8.5px] font-semibold uppercase tracking-[0.18em] text-slate-500">
                    {title}
                </h3>
                <Icon kind=icon class="h-3.5 w-3.5 text-slate-600" />
            </div>
            {children()}
        </div>
    }
}

fn empty() -> AnyView {
    view! { <span class="hidden"></span> }.into_any()
}

#[component]
fn WindCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Viento" icon=UiIcon::Wind>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let current = weather.current.clone();
                let speed = current.wind_speed;
                let direction = current.wind_direction.unwrap_or(0.0);
                let compass = wind_direction_label(direction);
                let gust = current.wind_gust.map(|v| format!("{v:.0} km/h")).unwrap_or_else(|| "—".to_string());
                let rotation = direction;
                let speed_label = speed.map(|v| format!("{v:.0}")).unwrap_or_else(|| "—".to_string());
                let description = speed.map(wind_description).unwrap_or("Sin datos de viento");
                view! {
                    <div class="mt-3 flex items-center gap-2">
                        <div class="min-w-0 flex-1">
                            <div class="flex items-start">
                                <span class="num text-3xl font-extralight leading-none text-white">
                                    {speed_label}
                                </span>
                                <span class="ml-1 mt-1 text-[9px] text-slate-500">"km/h"</span>
                            </div>
                            <p class="mt-2 text-[10px] text-slate-400">
                                {description}
                                <span class="mx-1 text-slate-600">"·"</span>
                                <span class="text-slate-500">{compass}</span>
                            </p>
                            <p class="mt-1 text-[9px] text-slate-600">"Ráfagas " {gust}</p>
                        </div>
                        <div class="relative h-[74px] w-[74px] shrink-0">
                            <svg viewBox="0 0 90 90" class="h-full w-full">
                                <circle cx="45" cy="45" r="40" fill="rgba(255,255,255,0.02)" stroke="rgba(255,255,255,0.07)" stroke-width="1"></circle>
                                <circle cx="45" cy="45" r="31" fill="none" stroke="rgba(255,255,255,0.045)" stroke-width="1" stroke-dasharray="1 5"></circle>
                                <g style=format!("transform: rotate({rotation:.0}deg); transform-origin: 45px 45px;")>
                                    <path d="M45 13 50 33 45 30 40 33Z" fill="#93c5fd"></path>
                                    <path d="M45 77 40 57 45 60 50 57Z" fill="rgba(148,163,184,0.35)"></path>
                                </g>
                                <circle cx="45" cy="45" r="5" fill="#0d1d3a" stroke="#93c5fd" stroke-width="1.5"></circle>
                                <text x="45" y="9" text-anchor="middle" fill="#64748b" font-size="7" font-family="Inter, sans-serif">"N"</text>
                                <text x="84" y="47" text-anchor="middle" fill="#64748b" font-size="7" font-family="Inter, sans-serif">"E"</text>
                                <text x="45" y="88" text-anchor="middle" fill="#64748b" font-size="7" font-family="Inter, sans-serif">"S"</text>
                                <text x="7" y="47" text-anchor="middle" fill="#64748b" font-size="7" font-family="Inter, sans-serif">"O"</text>
                            </svg>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn HumidityCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Humedad" icon=UiIcon::Humidity>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let current = weather.current;
                let humidity = current.humidity;
                let dew = current.dew_point.map(|v| ctx.units.get().temp_str(v)).unwrap_or_else(|| "—".to_string());
                let humidity_label = humidity.map(|v| format!("{v:.0}")).unwrap_or_else(|| "—".to_string());
                let humidity_text = humidity.map(humidity_description).unwrap_or("Sin datos de humedad");
                view! {
                    <div class="mt-3">
                        <div class="flex items-start">
                            <span class="num text-3xl font-extralight leading-none text-white">
                                {humidity_label}
                            </span>
                            <span class="ml-0.5 mt-0.5 text-sm font-light text-slate-500">"%"</span>
                        </div>
                        <p class="mt-2 text-[10px] text-slate-400">{humidity_text}</p>
                        <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-white/[0.055]">
                            <div
                                class="h-full rounded-full bg-gradient-to-r from-ice-500 via-ice-300 to-cyan-200"
                                style=format!("width:{:.1}%", humidity.unwrap_or(0.0).clamp(0.0, 100.0))
                            ></div>
                        </div>
                        <p class="mt-2 text-[9px] text-slate-600">
                            "Punto de rocío " <span class="text-slate-400">{dew}</span>
                        </p>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn FeelsCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Sensación" icon=UiIcon::Thermometer>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let units = ctx.units.get();
                let current = weather.current;
                let apparent = current.apparent.unwrap_or(current.temperature);
                let diff = apparent - current.temperature;
                let note = if diff.abs() < 0.7 {
                    "Similar a la temperatura real".to_string()
                } else if diff > 0.0 {
                    format!("Se siente {} más cálido por la humedad y el viento", units.delta_str(diff))
                } else {
                    format!("Se siente {} más frío por el viento", units.delta_str(diff))
                };
                view! {
                    <div class="mt-3">
                        <span class="num text-3xl font-extralight leading-none text-white">
                            {units.temp_str(apparent)}
                        </span>
                        <p class="mt-2 text-[10px] leading-relaxed text-slate-400">{note}</p>
                        <div class="mt-3 flex items-center gap-2 rounded-xl bg-white/[0.025] px-2.5 py-2">
                            <Icon kind=UiIcon::Cloud class="h-3.5 w-3.5 text-slate-600" />
                            <span class="text-[9px] text-slate-500">
                                "Nubosidad "
                                <span class="text-slate-300">
                                    {current.cloud_cover.map(|v| format!("{v:.0}%")).unwrap_or_else(|| "—".to_string())}
                                </span>
                            </span>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn UvCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Índice UV" icon=UiIcon::Sun>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let current = weather.current;
                let value = current.uv_index;
                let (name, text_class, bar_class) = match value {
                    Some(value) => uv_category(value),
                    None => ("Sin datos", "text-slate-500", "from-slate-600 to-slate-500"),
                };
                let uv_label = value.map(|v| format!("{v:.1}")).unwrap_or_else(|| "—".to_string());
                let bar_width = value.map(|v| (v / 11.0 * 100.0).clamp(0.0, 100.0)).unwrap_or(0.0);
                let max_today = weather.daily.first().and_then(|d| d.uv_max);
                view! {
                    <div class="mt-3">
                        <div class="flex items-end justify-between">
                            <span class="num text-3xl font-extralight leading-none text-white">
                                {uv_label}
                            </span>
                            <span class=format!("text-[10px] font-medium {text_class}")>{name}</span>
                        </div>
                        <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-white/[0.055]">
                            <div
                                class=format!("h-full rounded-full bg-gradient-to-r {bar_class}")
                                style=format!("width:{bar_width:.1}%")
                            ></div>
                        </div>
                        <p class="mt-2 text-[9px] text-slate-600">
                            "Máximo de hoy "
                            <span class="text-slate-400">
                                {max_today.map(|v| format!("{v:.1}")).unwrap_or_else(|| "—".to_string())}
                            </span>
                        </p>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn SunCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Sol" icon=UiIcon::Sunrise class="col-span-2">
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let today = weather.daily.first().cloned();
                let sunrise = today.as_ref().and_then(|d| d.sunrise.clone()).unwrap_or_else(|| "—".to_string());
                let sunset = today.as_ref().and_then(|d| d.sunset.clone()).unwrap_or_else(|| "—".to_string());
                let now = short_time(&weather.current.time);
                let (fraction, _) = sun_arc_position(&sunrise, &sunset, &now);
                let x = 25.0 + 210.0 * fraction;
                let y = 66.0 - 50.0 * (std::f64::consts::PI * fraction).sin();
                let daylight = daylight_duration(&sunrise, &sunset);
                view! {
                    <div class="mt-2">
                        <div class="flex items-start justify-between">
                            <div>
                                <p class="text-[9px] uppercase tracking-[0.16em] text-slate-600">"Hora local"</p>
                                <p class="mt-0.5 num text-xl font-light text-white">{now}</p>
                            </div>
                            <div class="text-right">
                                <p class="text-[9px] uppercase tracking-[0.16em] text-slate-600">"Duración del día"</p>
                                <p class="mt-0.5 text-[11px] text-slate-300">{daylight}</p>
                            </div>
                        </div>
                        <svg viewBox="0 0 260 92" class="mt-2 w-full" role="img" aria-label="Arco solar del día">
                            <defs>
                                <linearGradient id="fwSunArc" x1="25" y1="66" x2="235" y2="66" gradientUnits="userSpaceOnUse">
                                    <stop stop-color="#fb923c" stop-opacity="0.7"></stop>
                                    <stop offset="0.5" stop-color="#fbbf24"></stop>
                                    <stop offset="1" stop-color="#f472b6" stop-opacity="0.7"></stop>
                                </linearGradient>
                                <linearGradient id="fwSunArcFill" x1="0" y1="10" x2="0" y2="66" gradientUnits="userSpaceOnUse">
                                    <stop stop-color="#fbbf24" stop-opacity="0.14"></stop>
                                    <stop offset="1" stop-color="#fbbf24" stop-opacity="0"></stop>
                                </linearGradient>
                            </defs>
                            <path d="M25 66 A105 50 0 0 1 235 66 Z" fill="url(#fwSunArcFill)"></path>
                            <path d="M25 66 A105 50 0 0 1 235 66" fill="none" stroke="url(#fwSunArc)" stroke-width="1.5" stroke-dasharray="3 4"></path>
                            <line x1="14" y1="66" x2="246" y2="66" stroke="rgba(255,255,255,0.08)" stroke-width="1"></line>
                            <circle cx=x cy=y r="7" fill="#fbbf24" opacity="0.18"></circle>
                            <circle cx=x cy=y r="3.5" fill="#fde68a"></circle>
                            <text x="25" y="86" text-anchor="middle" fill="#94a3b8" font-size="9" font-family="Inter, sans-serif">
                                {sunrise.clone()}
                            </text>
                            <text x="235" y="86" text-anchor="middle" fill="#94a3b8" font-size="9" font-family="Inter, sans-serif">
                                {sunset.clone()}
                            </text>
                            <text x="25" y="76" text-anchor="middle" fill="#64748b" font-size="6.5" font-family="Inter, sans-serif">
                                "AMANECER"
                            </text>
                            <text x="235" y="76" text-anchor="middle" fill="#64748b" font-size="6.5" font-family="Inter, sans-serif">
                                "ATARDECER"
                            </text>
                        </svg>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn MoonCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Luna" icon=UiIcon::Moon>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let date = weather.current.date.clone();
                let Some(info) = date.parse::<chrono::NaiveDate>().ok().map(moon_phase) else {
                    return empty();
                };
                let offset = if info.waxing {
                    48.0 * info.illumination
                } else {
                    -48.0 * info.illumination
                };
                view! {
                    <div class="mt-2 flex items-center gap-3">
                        <div class="relative h-16 w-16 shrink-0">
                            <svg viewBox="0 0 100 100" class="h-full w-full">
                                <circle cx="50" cy="50" r="34" fill="#c7d2fe" opacity="0.08"></circle>
                                <circle cx="50" cy="50" r="25" fill="#1e293b"></circle>
                                <circle cx="50" cy="50" r="25" fill="#e2e8f0"></circle>
                                <circle cx=50.0 + offset cy="50" r="25" fill="#0d1d3a"></circle>
                                <circle cx="78" cy="24" r="1.5" fill="#e0e7ff" class="animate-pulse-soft"></circle>
                                <circle cx="86" cy="40" r="1" fill="#e0e7ff" class="animate-pulse-soft" style="animation-delay:.8s"></circle>
                            </svg>
                        </div>
                        <div class="min-w-0">
                            <p class="text-[11px] font-medium leading-tight text-slate-200">{info.name}</p>
                            <p class="mt-1 text-[9px] text-slate-500">
                                {format!("{:.0}% iluminada", info.illumination * 100.0)}
                            </p>
                            <p class="mt-1 text-[9px] text-slate-600">
                                {if info.waxing { "Fase creciente" } else { "Fase menguante" }}
                            </p>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn PressureCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Presión" icon=UiIcon::Gauge>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let pressure = weather.current.pressure;
                let pressure_label = pressure.map(|v| format!("{v:.0}")).unwrap_or_else(|| "—".to_string());
                let status = pressure.map(|v| pressure_status(v).0).unwrap_or("Sin datos");
                let next = weather.hourly.get(6).and_then(|h| h.pressure);
                let trend = match pressure {
                    Some(value) => pressure_trend(value, next),
                    None => "—",
                };
                let pct = pressure
                    .map(|v| ((v - 980.0) / 60.0).clamp(0.0, 1.0))
                    .unwrap_or(0.0);
                let dash = 226.2 * pct;
                view! {
                    <div class="mt-2 flex items-center gap-3">
                        <div class="relative h-[72px] w-[72px] shrink-0">
                            <svg viewBox="0 0 90 90" class="h-full w-full">
                                <circle cx="45" cy="45" r="36" fill="none" stroke="rgba(255,255,255,0.06)" stroke-width="5"></circle>
                                <circle
                                    cx="45"
                                    cy="45"
                                    r="36"
                                    fill="none"
                                    stroke="#60a5fa"
                                    stroke-width="5"
                                    stroke-linecap="round"
                                    stroke-dasharray=format!("{dash:.1} 300")
                                    transform="rotate(-90 45 45)"
                                ></circle>
                            </svg>
                            <div class="absolute inset-0 grid place-items-center">
                                <span class="num text-[17px] font-light text-white">{pressure_label}</span>
                            </div>
                        </div>
                        <div class="min-w-0">
                            <p class="text-[10px] text-slate-300">{status}</p>
                            <p class="mt-1 text-[9px] text-slate-500">"hPa · " {trend}</p>
                            <p class="mt-1 text-[8px] text-slate-600">"Nivel del mar"</p>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn PrecipCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Precipitación" icon=UiIcon::Umbrella>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let current = weather.current;
                let today = weather.daily.first().cloned();
                let total = today
                    .as_ref()
                    .and_then(|d| d.precipitation_sum)
                    .map(|v| format!("{v:.1} mm"))
                    .unwrap_or_else(|| "—".to_string());
                let probability = current
                    .precipitation_probability
                    .map(|v| format!("{v:.0}%"))
                    .unwrap_or_else(|| "—".to_string());
                view! {
                    <div class="mt-3">
                        <span class="num text-3xl font-extralight leading-none text-white">
                            {format!("{:.1}", current.precipitation)}
                            <span class="ml-1 text-[9px] font-normal text-slate-500">"mm"</span>
                        </span>
                        <p class="mt-2 text-[10px] text-slate-400">
                            {if current.precipitation > 0.05 { "Precipitando ahora" } else { "Sin lluvia en este momento" }}
                        </p>
                        <div class="mt-3 space-y-1.5 rounded-xl bg-white/[0.025] px-2.5 py-2">
                            <div class="flex justify-between text-[9px]">
                                <span class="text-slate-600">"Probabilidad"</span>
                                <span class="text-slate-300">{probability}</span>
                            </div>
                            <div class="flex justify-between text-[9px]">
                                <span class="text-slate-600">"Acumulado hoy"</span>
                                <span class="text-slate-300">{total}</span>
                            </div>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn VisibilityCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Visibilidad" icon=UiIcon::Eye>
            {move || {
                let Some(weather) = ctx.weather.get() else { return empty(); };
                let visibility = weather.current.visibility_km;
                view! {
                    <div class="mt-3">
                        <span class="num text-3xl font-extralight leading-none text-white">
                            {visibility.map(|v| format!("{v:.0}")).unwrap_or_else(|| "—".to_string())}
                            <span class="ml-1 text-[9px] font-normal text-slate-500">"km"</span>
                        </span>
                        <p class="mt-2 text-[10px] leading-relaxed text-slate-400">
                            {visibility.map(visibility_description).unwrap_or("Esta fuente no reporta visibilidad")}
                        </p>
                        <div class="mt-3 flex items-center gap-2 rounded-xl bg-white/[0.025] px-2.5 py-2">
                            <Icon kind=UiIcon::Waves class="h-3.5 w-3.5 text-slate-600" />
                            <span class="text-[9px] text-slate-500">
                                "Nubosidad "
                                <span class="text-slate-300">
                                    {weather.current.cloud_cover.map(|v| format!("{v:.0}%")).unwrap_or_else(|| "—".to_string())}
                                </span>
                            </span>
                        </div>
                    </div>
                }.into_any()
            }}
        </CardShell>
    }
}

#[component]
fn AqiCard() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <CardShell title="Calidad del aire" icon=UiIcon::Sparkle class="col-span-2">
            {move || {
                let Some(aqi) = ctx.air.get() else { return empty(); };
                aqi_content(&aqi)
            }}
        </CardShell>
    }
}

fn aqi_content(aqi: &AirQuality) -> AnyView {
    let value = aqi.european_aqi.or(aqi.us_aqi).unwrap_or(0.0);
    let (name, text_class, bar_class) = aqi_category(value);
    let pm25 = aqi.pm2_5.map(|v| format!("{v:.1}")).unwrap_or_else(|| "—".to_string());
    let pm10 = aqi.pm10.map(|v| format!("{v:.1}")).unwrap_or_else(|| "—".to_string());
    let ozone = aqi.ozone.map(|v| format!("{v:.0}")).unwrap_or_else(|| "—".to_string());
    let index_label = if aqi.european_aqi.is_some() { "EAQI" } else { "US AQI" };

    view! {
        <div class="mt-3">
            <div class="flex items-end justify-between gap-3">
                <div class="flex items-start">
                    <span class="num text-3xl font-extralight leading-none text-white">{format!("{value:.0}")}</span>
                    <span class="ml-1.5 mt-1 text-[8px] uppercase tracking-wider text-slate-600">{index_label}</span>
                </div>
                <span class=format!("text-[10px] font-medium {text_class}")>{name}</span>
            </div>
            <div class="mt-3 h-1.5 overflow-hidden rounded-full bg-white/[0.055]">
                <div
                    class=format!("h-full rounded-full bg-gradient-to-r {bar_class}")
                    style=format!("width:{:.1}%", (value / 100.0 * 100.0).clamp(0.0, 100.0))
                ></div>
            </div>
            <div class="mt-3 grid grid-cols-3 gap-2">
                <Pollutant label="PM2.5" value=pm25 unit="µg/m³" />
                <Pollutant label="PM10" value=pm10 unit="µg/m³" />
                <Pollutant label="O₃" value=ozone unit="µg/m³" />
            </div>
            <p class="mt-2 text-[8px] text-slate-600">
                "CAMS · Open-Meteo Air Quality"
            </p>
        </div>
    }
    .into_any()
}

#[component]
fn Pollutant(#[prop(into)] label: String, #[prop(into)] value: String, #[prop(into)] unit: String) -> impl IntoView {
    view! {
        <div class="rounded-xl bg-white/[0.025] px-2 py-1.5">
            <p class="text-[7.5px] uppercase tracking-[0.12em] text-slate-600">{label}</p>
            <p class="mt-0.5 text-[11px] text-slate-300">
                {value}
                <span class="ml-0.5 text-[7px] text-slate-600">{unit}</span>
            </p>
        </div>
    }
}

fn daylight_duration(sunrise: &str, sunset: &str) -> String {
    let parse = |value: &str| -> Option<u32> {
        let mut parts = value.split(':');
        let hours: u32 = parts.next()?.parse().ok()?;
        let minutes: u32 = parts.next()?.parse().ok()?;
        Some(hours * 60 + minutes)
    };
    match (parse(sunrise), parse(sunset)) {
        (Some(rise), Some(set)) if set >= rise => {
            let minutes = set - rise;
            format!("{} h {} min", minutes / 60, minutes % 60)
        }
        _ => "—".to_string(),
    }
}
