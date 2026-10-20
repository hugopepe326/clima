use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon, WeatherIcon};
use crate::format::{short_time, weekday_short};
use crate::models::condition;
use crate::state::AppCtx;

#[component]
pub fn Hourly() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <section id="horas" class="scroll-mt-24">
            <div class="mb-3 flex items-end justify-between gap-3 px-1">
                <div>
                    <h2 class="section-title">"Próximas 24 horas"</h2>
                    <p class="mt-1 text-[10px] text-slate-500">
                        {move || {
                            let Some(weather) = ctx.weather.get() else {
                                return "Consultando…".to_string();
                            };
                            let rain = weather.hourly.iter().skip(1).find(|hour| {
                                hour.precipitation_probability.unwrap_or(0.0) >= 40.0
                                    || hour.precipitation > 0.1
                            });
                            match rain {
                                Some(hour) => format!(
                                    "{} probable a las {}",
                                    condition(hour.weather_code).label.to_lowercase(),
                                    short_time(&hour.time)
                                ),
                                None => "Sin precipitaciones previstas en las próximas 24 h".to_string(),
                            }
                        }}
                    </p>
                </div>
                <span class="hidden items-center gap-1 text-[9px] text-slate-600 sm:flex">
                    <Icon kind=UiIcon::Clock class="h-3 w-3" />
                    {move || ctx.units.get().label()}
                </span>
            </div>

            <div class="card overflow-hidden p-1.5 sm:p-2">
                <div class="snap-row no-scrollbar flex gap-0.5 overflow-x-auto pb-0.5 sm:gap-1">
                    {move || {
                        let Some(weather) = ctx.weather.get() else {
                            return ().into_any();
                        };
                        let units = ctx.units.get();
                        weather
                            .hourly
                            .iter()
                            .take(24)
                            .enumerate()
                            .map(|(index, hour)| {
                                let is_now = index == 0;
                                let info = condition(hour.weather_code);
                                let time_label = if is_now {
                                    "Ahora".to_string()
                                } else {
                                    short_time(&hour.time)
                                };
                                let day_marker = if index > 0 && hour.time.ends_with("T00:00") {
                                    Some(weekday_short(&hour.date))
                                } else {
                                    None
                                };
                                let precipitation = hour
                                    .precipitation_probability
                                    .map(|p| format!("{p:.0}%"))
                                    .or_else(|| {
                                        (hour.precipitation > 0.0)
                                            .then(|| format!("{:.1}mm", hour.precipitation))
                                    })
                                    .unwrap_or_default();
                                let has_precip = !precipitation.is_empty();
                                let wind = hour
                                    .wind_speed
                                    .map(|v| format!("{v:.0}"))
                                    .unwrap_or_else(|| "—".to_string());

                                view! {
                                    <div class=format!(
                                        "group flex min-w-[68px] shrink-0 flex-col items-center rounded-2xl px-2 pb-3 pt-2.5 transition-all sm:min-w-[76px] {}",
                                        if is_now {
                                            "bg-white/[0.075] ring-1 ring-ice-400/15"
                                        } else {
                                            "hover:bg-white/[0.04]"
                                        }
                                    )>
                                        <span class=format!(
                                            "text-[10px] font-medium {}",
                                            if is_now { "text-ice-200" } else { "text-slate-500" }
                                        )>
                                            {time_label}
                                        </span>
                                        <span class="mt-0.5 h-2.5 text-[7.5px] uppercase leading-none tracking-wider text-slate-600">
                                            {day_marker.unwrap_or_default()}
                                        </span>
                                        <div class="my-1.5 transition-transform duration-300 group-hover:scale-110" title=info.label>
                                            <WeatherIcon
                                                code=hour.weather_code
                                                is_day=hour.is_day
                                                class="h-9 w-9 sm:h-10 sm:w-10"
                                            />
                                        </div>
                                        <span class="num text-[14px] font-medium leading-none text-slate-100">
                                            {units.temp_str(hour.temperature)}
                                        </span>
                                        <span class=format!(
                                            "mt-1.5 flex h-3.5 items-center gap-0.5 text-[9px] leading-none {}",
                                            if has_precip { "text-ice-300/80" } else { "text-transparent" }
                                        )>
                                            <Show when=move || has_precip>
                                                <Icon kind=UiIcon::Drop class="h-2.5 w-2.5" />
                                            </Show>
                                            {precipitation}
                                        </span>
                                        <span class="mt-1 flex items-center gap-0.5 text-[8.5px] leading-none text-slate-500">
                                            <Icon kind=UiIcon::Wind class="h-2.5 w-2.5 text-slate-600" />
                                            {wind}
                                            <span class="text-[6.5px] text-slate-600">"km/h"</span>
                                        </span>
                                    </div>
                                }
                            })
                            .collect_view()
                            .into_any()
                    }}
                </div>
            </div>
        </section>
    }
}
