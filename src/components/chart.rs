use leptos::prelude::*;

use crate::format::{short_time, smooth_path};
use crate::models::Units;
use crate::state::AppCtx;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Metric {
    Temperature,
    Precipitation,
    Wind,
}

impl Metric {
    fn label(self) -> &'static str {
        match self {
            Self::Temperature => "Temperatura",
            Self::Precipitation => "Lluvia",
            Self::Wind => "Viento",
        }
    }

    fn short(self) -> &'static str {
        match self {
            Self::Temperature => "Temp.",
            Self::Precipitation => "Lluvia",
            Self::Wind => "Viento",
        }
    }
}

#[component]
pub fn TrendChart() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let metric = RwSignal::new(Metric::Temperature);

    view! {
        <section class="card h-full p-4 sm:p-5 lg:col-span-3">
            <div class="flex flex-wrap items-start justify-between gap-3">
                <div>
                    <h2 class="section-title">"Tendencia · 24 horas"</h2>
                    <p class="mt-1 text-[10px] text-slate-500">
                        {move || format!("Datos horarios reales · {}", ctx.provider.get().label())}
                    </p>
                </div>
                <div class="inline-flex rounded-xl border border-white/[0.065] bg-black/15 p-0.5">
                    {[Metric::Temperature, Metric::Precipitation, Metric::Wind]
                        .into_iter()
                        .map(|option| {
                            let active = move || metric.get() == option;
                            view! {
                                <button
                                    type="button"
                                    class=move || format!(
                                        "focus-ring rounded-[9px] px-2.5 py-1.5 text-[9px] font-medium transition-all {}",
                                        if active() {
                                            "bg-white/[0.08] text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]"
                                        } else {
                                            "text-slate-500 hover:text-slate-300"
                                        }
                                    )
                                    aria-pressed=move || active().to_string()
                                    on:click=move |_| metric.set(option)
                                >
                                    {option.short()}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            {move || {
                let Some(weather) = ctx.weather.get() else {
                    return view! {
                        <div class="mt-4 flex h-56 items-center justify-center">
                            <div class="skeleton h-40 w-full rounded-2xl"></div>
                        </div>
                    }.into_any();
                };
                let units = ctx.units.get();
                let selected = metric.get();
                let hours: Vec<_> = weather.hourly.iter().take(24).cloned().collect();

                if hours.len() < 2 {
                    return view! {
                        <div class="mt-4 grid h-56 place-items-center text-[11px] text-slate-500">
                            "No hay suficientes datos horarios para graficar."
                        </div>
                    }.into_any();
                }

                let values: Vec<f64> = hours
                    .iter()
                    .map(|hour| match selected {
                        Metric::Temperature => units.temp(hour.temperature),
                        Metric::Precipitation => hour.precipitation,
                        Metric::Wind => hour.wind_speed.unwrap_or(0.0),
                    })
                    .collect();

                let (min_value, max_value) = match selected {
                    Metric::Temperature => {
                        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
                        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        (min - 1.0, max + 1.0)
                    }
                    Metric::Precipitation => {
                        let max = values.iter().copied().fold(0.0_f64, f64::max);
                        (0.0, (max + 0.2).max(0.6))
                    }
                    Metric::Wind => {
                        let max = values.iter().copied().fold(0.0_f64, f64::max);
                        (0.0, (max + 2.0).max(6.0))
                    }
                };
                let span = (max_value - min_value).max(0.01);

                let width = 720.0;
                let height = 300.0;
                let pad_left = 44.0;
                let pad_right = 18.0;
                let pad_top = 26.0;
                let pad_bottom = 38.0;
                let plot_w = width - pad_left - pad_right;
                let plot_h = height - pad_top - pad_bottom;

                let points: Vec<(f64, f64)> = values
                    .iter()
                    .enumerate()
                    .map(|(index, value)| {
                        let x = pad_left + plot_w * index as f64 / (values.len() - 1) as f64;
                        let y = pad_top + plot_h * (max_value - value) / span;
                        (x, y)
                    })
                    .collect();

                let line = smooth_path(&points);
                let area = format!(
                    "{} L {:.2} {:.2} L {:.2} {:.2} Z",
                    line,
                    points.last().unwrap().0,
                    pad_top + plot_h,
                    points.first().unwrap().0,
                    pad_top + plot_h
                );

                let (stroke, stop_top, stop_bottom, unit_label) = match selected {
                    Metric::Temperature => ("#7dd3fc", "#38bdf8", "#38bdf8", units.label()),
                    Metric::Precipitation => ("#67e8f9", "#22d3ee", "#22d3ee", "mm/h"),
                    Metric::Wind => ("#6ee7b7", "#34d399", "#34d399", "km/h"),
                };

                let grid: Vec<AnyView> = (0..=3)
                    .map(|index| {
                        let y = pad_top + plot_h * index as f64 / 3.0;
                        let value = max_value - span * index as f64 / 3.0;
                        let label = match selected {
                            Metric::Temperature => format!("{value:.0}°"),
                            Metric::Precipitation => format!("{value:.1}"),
                            Metric::Wind => format!("{value:.0}"),
                        };
                        view! {
                            <g>
                                <line
                                    x1=pad_left
                                    y1=y
                                    x2=width - pad_right
                                    y2=y
                                    stroke="rgba(255,255,255,0.055)"
                                    stroke-width="1"
                                    stroke-dasharray="3 5"
                                ></line>
                                <text
                                    x=pad_left - 8.0
                                    y=y + 3.5
                                    text-anchor="end"
                                    fill="#64748b"
                                    font-size="10"
                                    font-family="Inter, sans-serif"
                                >
                                    {label}
                                </text>
                            </g>
                        }
                        .into_any()
                    })
                    .collect();

                let x_labels: Vec<AnyView> = [0usize, 6, 12, 18, 23]
                    .into_iter()
                    .filter(|index| *index < hours.len())
                    .map(|index| {
                        let x = pad_left + plot_w * index as f64 / (hours.len() - 1) as f64;
                        let label = if index == 0 {
                            "Ahora".to_string()
                        } else {
                            short_time(&hours[index].time)
                        };
                        view! {
                            <text
                                x=x
                                y=height - 14.0
                                text-anchor="middle"
                                fill=if index == 0 { "#93c5fd" } else { "#64748b" }
                                font-size="10"
                                font-family="Inter, sans-serif"
                            >
                                {label}
                            </text>
                        }
                        .into_any()
                    })
                    .collect();

                let dots: Vec<AnyView> = points
                    .iter()
                    .enumerate()
                    .map(|(index, (x, y))| {
                        let value = values[index];
                        let label = if index == 0 {
                            "Ahora".to_string()
                        } else {
                            short_time(&hours[index].time)
                        };
                        let value_label = match selected {
                            Metric::Temperature => units.temp_dec(reverse_temp(units, value), 1),
                            Metric::Precipitation => format!("{value:.1} mm"),
                            Metric::Wind => format!("{value:.0} km/h"),
                        };
                        view! {
                            <g class="group">
                                <circle cx=*x cy=*y r="9" fill="transparent">
                                    <title>{format!("{label} · {value_label}")}</title>
                                </circle>
                                <circle
                                    cx=*x
                                    cy=*y
                                    r=if index == 0 { 4.5 } else { 2.4 }
                                    fill=if index == 0 { "#f8fafc" } else { stroke }
                                    stroke=if index == 0 { stroke } else { "none" }
                                    stroke-width=if index == 0 { 2.0 } else { 0.0 }
                                ></circle>
                            </g>
                        }
                        .into_any()
                    })
                    .collect();

                let summary = match selected {
                    Metric::Temperature => {
                        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
                        format!(
                            "Máx {} · Mín {}",
                            units.temp_str(reverse_temp(units, max)),
                            units.temp_str(reverse_temp(units, min))
                        )
                    }
                    Metric::Precipitation => {
                        let total: f64 = values.iter().sum();
                        format!("Acumulado {total:.1} mm")
                    }
                    Metric::Wind => {
                        let max = values.iter().copied().fold(0.0_f64, f64::max);
                        format!("Máx {max:.0} km/h")
                    }
                };

                view! {
                    <div class="mt-3">
                        <div class="mb-1 flex items-center justify-between px-1">
                            <span class="text-[9px] font-medium uppercase tracking-[0.14em] text-slate-500">
                                {selected.label()}
                            </span>
                            <span class="num text-[10px] text-slate-400">{summary}</span>
                        </div>
                        <svg
                            viewBox=format!("0 0 {width} {height}")
                            class="w-full"
                            role="img"
                            aria-label=format!("Gráfico de {} de las próximas 24 horas", selected.label())
                        >
                            <defs>
                                <linearGradient id="fwChartFill" x1="0" y1="0" x2="0" y2="1">
                                    <stop offset="0%" stop-color=stop_top stop-opacity="0.28"></stop>
                                    <stop offset="100%" stop-color=stop_bottom stop-opacity="0.0"></stop>
                                </linearGradient>
                            </defs>

                            {grid}
                            <path d=area fill="url(#fwChartFill)"></path>
                            <path
                                d=line
                                fill="none"
                                stroke=stroke
                                stroke-width="2.4"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            ></path>
                            {dots}
                            {x_labels}
                            <text
                                x=width - pad_right
                                y=height - 14.0
                                text-anchor="end"
                                fill="#475569"
                                font-size="9"
                                font-family="Inter, sans-serif"
                            >
                                {unit_label}
                            </text>
                        </svg>
                    </div>
                }
                .into_any()
            }}
        </section>
    }
}

fn reverse_temp(units: Units, value: f64) -> f64 {
    match units {
        Units::Celsius => value,
        Units::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
    }
}
