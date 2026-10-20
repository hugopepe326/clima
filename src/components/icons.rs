use leptos::prelude::*;

use crate::models::{condition, Sky};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiIcon {
    Search,
    Locate,
    Star,
    StarFilled,
    Refresh,
    Wind,
    Humidity,
    Gauge,
    Eye,
    Sun,
    Sunrise,
    Sunset,
    Moon,
    Umbrella,
    Drop,
    Map,
    Settings,
    Home,
    Clock,
    Calendar,
    Close,
    ChevronRight,
    External,
    Thermometer,
    Check,
    Alert,
    Layers,
    Cloud,
    Sparkle,
    Waves,
}

#[component]
pub fn Icon(#[prop(into)] kind: UiIcon, #[prop(into, optional)] class: String) -> impl IntoView {
    let class = if class.is_empty() {
        "h-5 w-5".to_string()
    } else {
        class
    };

    let inner: AnyView = match kind {
        UiIcon::Search => view! {
            <circle cx="11" cy="11" r="7"></circle>
            <path d="m20 20-3.4-3.4"></path>
        }
        .into_any(),
        UiIcon::Locate => view! {
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M12 2v3M12 19v3M2 12h3M19 12h3"></path>
            <path d="M12 5a7 7 0 1 0 0 14 7 7 0 0 0 0-14Z"></path>
        }
        .into_any(),
        UiIcon::Star => view! {
            <path d="m12 3 2.7 5.6 6.1.9-4.4 4.3 1 6.1-5.4-2.9-5.4 2.9 1-6.1L3.2 9.5l6.1-.9L12 3Z"></path>
        }
        .into_any(),
        UiIcon::StarFilled => view! {
            <path
                d="m12 3 2.7 5.6 6.1.9-4.4 4.3 1 6.1-5.4-2.9-5.4 2.9 1-6.1L3.2 9.5l6.1-.9L12 3Z"
                fill="currentColor"
                stroke="none"
            ></path>
        }
        .into_any(),
        UiIcon::Refresh => view! {
            <path d="M20 11a8.1 8.1 0 0 0-14.8-4.5L3 9"></path>
            <path d="M3 4v5h5"></path>
            <path d="M4 13a8.1 8.1 0 0 0 14.8 4.5L21 15"></path>
            <path d="M21 20v-5h-5"></path>
        }
        .into_any(),
        UiIcon::Wind => view! {
            <path d="M3 8h11a3 3 0 1 0-3-3"></path>
            <path d="M3 12h15a3 3 0 1 1-3 3"></path>
            <path d="M3 16h7"></path>
        }
        .into_any(),
        UiIcon::Humidity => view! {
            <path d="M12 3s6 6.2 6 11a6 6 0 0 1-12 0c0-4.8 6-11 6-11Z"></path>
            <path d="M9.5 14.5a2.8 2.8 0 0 0 2.7 2.7"></path>
        }
        .into_any(),
        UiIcon::Gauge => view! {
            <path d="M4 18a9 9 0 1 1 16 0"></path>
            <path d="m12 14 4-4"></path>
            <circle cx="12" cy="18" r="1.4" fill="currentColor" stroke="none"></circle>
        }
        .into_any(),
        UiIcon::Eye => view! {
            <path d="M2.5 12S6 5.5 12 5.5 21.5 12 21.5 12 18 18.5 12 18.5 2.5 12 2.5 12Z"></path>
            <circle cx="12" cy="12" r="3"></circle>
        }
        .into_any(),
        UiIcon::Sun => view! {
            <circle cx="12" cy="12" r="4"></circle>
            <path d="M12 2v2M12 20v2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M2 12h2M20 12h2M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"></path>
        }
        .into_any(),
        UiIcon::Sunrise => view! {
            <path d="M12 3v6M8 7l4-4 4 4"></path>
            <path d="M3 18h18M5 14a7 7 0 0 1 14 0"></path>
        }
        .into_any(),
        UiIcon::Sunset => view! {
            <path d="M12 9V3M8 5l4 4 4-4"></path>
            <path d="M3 18h18M5 14a7 7 0 0 1 14 0"></path>
        }
        .into_any(),
        UiIcon::Moon => view! {
            <path d="M20 15.2A8.4 8.4 0 0 1 8.8 4 8.5 8.5 0 1 0 20 15.2Z"></path>
        }
        .into_any(),
        UiIcon::Umbrella => view! {
            <path d="M12 3a9 9 0 0 1 9 9H3a9 9 0 0 1 9-9Z"></path>
            <path d="M12 12v6a2 2 0 0 0 4 0"></path>
        }
        .into_any(),
        UiIcon::Drop => view! {
            <path d="M12 3.5s5.5 5.7 5.5 9.5a5.5 5.5 0 0 1-11 0C6.5 9.2 12 3.5 12 3.5Z"></path>
        }
        .into_any(),
        UiIcon::Map => view! {
            <path d="m3 6 6-3 6 3 6-3v15l-6 3-6-3-6 3V6Z"></path>
            <path d="M9 3v15M15 6v15"></path>
        }
        .into_any(),
        UiIcon::Settings => view! {
            <circle cx="12" cy="12" r="3"></circle>
            <path d="M19.4 15a1.7 1.7 0 0 0 .3 1.9l.1.1-2.7 2.7-.1-.1a1.7 1.7 0 0 0-1.9-.3 1.7 1.7 0 0 0-1 1.5V21h-3.8v-.2a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.9.3l-.1.1L4.5 17l.1-.1a1.7 1.7 0 0 0 .3-1.9 1.7 1.7 0 0 0-1.5-1H3v-3.8h.4a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.9L4.5 7 7.2 4.3l.1.1a1.7 1.7 0 0 0 1.9.3 1.7 1.7 0 0 0 1-1.5V3h3.8v.2a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.9-.3l.1-.1L19.5 7l-.1.1a1.7 1.7 0 0 0-.3 1.9 1.7 1.7 0 0 0 1.5 1h.4v3.8h-.4a1.7 1.7 0 0 0-1.2 1.2Z"></path>
        }
        .into_any(),
        UiIcon::Home => view! {
            <path d="m3 10 9-7 9 7v10a1 1 0 0 1-1 1h-5v-7H9v7H4a1 1 0 0 1-1-1V10Z"></path>
        }
        .into_any(),
        UiIcon::Clock => view! {
            <circle cx="12" cy="12" r="9"></circle>
            <path d="M12 7v5l3.5 2"></path>
        }
        .into_any(),
        UiIcon::Calendar => view! {
            <rect x="3" y="5" width="18" height="16" rx="2"></rect>
            <path d="M8 3v4M16 3v4M3 10h18"></path>
        }
        .into_any(),
        UiIcon::Close => view! {
            <path d="m6 6 12 12M18 6 6 18"></path>
        }
        .into_any(),
        UiIcon::ChevronRight => view! {
            <path d="m9 6 6 6-6 6"></path>
        }
        .into_any(),
        UiIcon::External => view! {
            <path d="M14 3h7v7M21 3l-9 9"></path>
            <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
        }
        .into_any(),
        UiIcon::Thermometer => view! {
            <path d="M14 14.8V5a2 2 0 1 0-4 0v9.8a4 4 0 1 0 4 0Z"></path>
        }
        .into_any(),
        UiIcon::Check => view! {
            <path d="m5 12 4 4 10-10"></path>
        }
        .into_any(),
        UiIcon::Alert => view! {
            <path d="M12 3 2.5 20h19L12 3Z"></path>
            <path d="M12 9v5M12 17.5v.1"></path>
        }
        .into_any(),
        UiIcon::Layers => view! {
            <path d="m12 3 9 5-9 5-9-5 9-5Z"></path>
            <path d="m3 13 9 5 9-5M3 18l9 5 9-5"></path>
        }
        .into_any(),
        UiIcon::Cloud => view! {
            <path d="M17.5 19a4.5 4.5 0 0 0 .4-9A6.5 6.5 0 0 0 5.6 9.3 4.4 4.4 0 0 0 6.5 19h11Z"></path>
        }
        .into_any(),
        UiIcon::Sparkle => view! {
            <path d="M12 3l1.8 5.2L19 10l-5.2 1.8L12 17l-1.8-5.2L5 10l5.2-1.8L12 3Z"></path>
            <path d="M19 16l.8 2.2L22 19l-2.2.8L19 22l-.8-2.2L16 19l2.2-.8L19 16Z"></path>
        }
        .into_any(),
        UiIcon::Waves => view! {
            <path d="M3 8c2 0 2 2 4 2s2-2 4-2 2 2 4 2 2-2 4-2"></path>
            <path d="M3 14c2 0 2 2 4 2s2-2 4-2 2 2 4 2 2-2 4-2"></path>
            <path d="M3 20c2 0 2 2 4 2s2-2 4-2 2 2 4 2 2-2 4-2"></path>
        }
        .into_any(),
    };

    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="1.7"
            stroke-linecap="round"
            stroke-linejoin="round"
            aria-hidden="true"
        >
            {inner}
        </svg>
    }
}

// ---------------------------------------------------------------------------
// Iconos meteorológicos animados
// ---------------------------------------------------------------------------

#[component]
pub fn WeatherIcon(
    code: u8,
    #[prop(optional)] is_day: bool,
    #[prop(into, optional)] class: String,
    #[prop(optional)] animated: bool,
) -> impl IntoView {
    let info = condition(code);
    let class = if class.is_empty() {
        "h-12 w-12".to_string()
    } else {
        class
    };
    let motion = if animated { " animate-bob" } else { "" };
    let svg_class = format!("{class}{motion}");
    let label = info.label;

    let inner: AnyView = match (info.sky, is_day) {
        (Sky::Clear, true) | (Sky::MostlyClear, true) => sun(60.0, 50.0, 1.0).into_any(),
        (Sky::Clear, false) | (Sky::MostlyClear, false) => moon(60.0, 50.0, 1.0).into_any(),
        (Sky::PartlyCloudy, true) => view! {
            <g transform="translate(-20 -16) scale(0.62)">{sun(60.0, 50.0, 1.0)}</g>
            <g transform="translate(4 20) scale(0.9)">{cloud()}</g>
        }
        .into_any(),
        (Sky::PartlyCloudy, false) => view! {
            <g transform="translate(-20 -16) scale(0.62)">{moon(60.0, 50.0, 1.0)}</g>
            <g transform="translate(4 20) scale(0.9)">{cloud()}</g>
        }
        .into_any(),
        (Sky::Cloudy, _) => view! {
            <g transform="translate(-12 -4) scale(0.75)" opacity="0.55">{cloud()}</g>
            <g transform="translate(6 14) scale(0.92)">{cloud()}</g>
        }
        .into_any(),
        (Sky::Fog, _) => view! {
            <g transform="translate(0 -10) scale(0.92)">{cloud()}</g>
            <g class="animate-drift" style="animation-delay: 0s">
                <path d="M22 96h76M30 106h52M26 116h64" stroke="#cbd5e1" stroke-width="3.5" stroke-linecap="round" opacity="0.45"></path>
            </g>
        }
        .into_any(),
        (Sky::Drizzle, _) => view! {
            <g transform="translate(0 -6) scale(0.9)">{cloud()}</g>
            {drops(2, 1.2)}
        }
        .into_any(),
        (Sky::Rain, _) => view! {
            <g transform="translate(0 -4) scale(0.9)">{cloud()}</g>
            {drops(3, 1.0)}
        }
        .into_any(),
        (Sky::HeavRain, _) => view! {
            <g transform="translate(0 -4) scale(0.9)">{cloud()}</g>
            {drops(5, 1.5)}
        }
        .into_any(),
        (Sky::Showers, _) => view! {
            <g transform="translate(-24 -20) scale(0.58)">{sun(60.0, 50.0, 1.0)}</g>
            <g transform="translate(6 8) scale(0.82)">{cloud()}</g>
            {drops(3, 1.0)}
        }
        .into_any(),
        (Sky::Snow, _) => view! {
            <g transform="translate(0 -4) scale(0.9)">{cloud()}</g>
            {flakes(3)}
        }
        .into_any(),
        (Sky::Sleet, _) => view! {
            <g transform="translate(0 -4) scale(0.9)">{cloud()}</g>
            <line x1="40" y1="94" x2="36" y2="106" stroke="#93c5fd" stroke-width="3" stroke-linecap="round" class="animate-rain"></line>
            <line x1="60" y1="94" x2="56" y2="106" stroke="#93c5fd" stroke-width="3" stroke-linecap="round" class="animate-rain" style="animation-delay:.5s"></line>
            {flakes(1)}
        }
        .into_any(),
        (Sky::Thunder, _) => view! {
            <g transform="translate(0 -4) scale(0.9)">{cloud()}</g>
            <path d="M63 88 51 106h8l-4 14 16-20h-8l6-12Z" fill="#fde68a" class="animate-flash"></path>
        }
        .into_any(),
    };

    view! {
        <svg
            class=svg_class
            viewBox="0 0 120 120"
            fill="none"
            role="img"
            aria-label=label
        >
            <defs>
                <linearGradient id="fwSun" x1="40" y1="28" x2="82" y2="76" gradientUnits="userSpaceOnUse">
                    <stop stop-color="#fde68a"></stop>
                    <stop offset="0.55" stop-color="#fbbf24"></stop>
                    <stop offset="1" stop-color="#f59e0b"></stop>
                </linearGradient>
                <linearGradient id="fwMoon" x1="34" y1="24" x2="78" y2="78" gradientUnits="userSpaceOnUse">
                    <stop stop-color="#f8fafc"></stop>
                    <stop offset="1" stop-color="#c7d2fe"></stop>
                </linearGradient>
                <linearGradient id="fwCloud" x1="28" y1="42" x2="94" y2="100" gradientUnits="userSpaceOnUse">
                    <stop stop-color="#f1f5f9" stop-opacity="0.98"></stop>
                    <stop offset="1" stop-color="#bfdbfe" stop-opacity="0.82"></stop>
                </linearGradient>
                <linearGradient id="fwCloudBack" x1="28" y1="42" x2="94" y2="100" gradientUnits="userSpaceOnUse">
                    <stop stop-color="#94a3b8" stop-opacity="0.65"></stop>
                    <stop offset="1" stop-color="#64748b" stop-opacity="0.5"></stop>
                </linearGradient>
            </defs>
            {inner}
        </svg>
    }
}

fn sun(cx: f64, cy: f64, scale: f64) -> AnyView {
    let transform = format!("translate({cx} {cy}) scale({scale}) translate(-60 -50)");
    view! {
        <g transform=transform>
            <circle cx="60" cy="50" r="34" fill="#fbbf24" opacity="0.12"></circle>
            <g class="animate-spin-slow" style="transform-origin: 60px 50px">
                <g stroke="#fcd34d" stroke-width="3.2" stroke-linecap="round" opacity="0.85">
                    <line x1="60" y1="10" x2="60" y2="19"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(45 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(90 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(135 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(180 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(225 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(270 60 50)"></line>
                    <line x1="60" y1="10" x2="60" y2="19" transform="rotate(315 60 50)"></line>
                </g>
            </g>
            <circle cx="60" cy="50" r="19" fill="url(#fwSun)"></circle>
            <circle cx="54" cy="44" r="6" fill="#fffbeb" opacity="0.45"></circle>
        </g>
    }
    .into_any()
}

fn moon(cx: f64, cy: f64, scale: f64) -> AnyView {
    let transform = format!("translate({cx} {cy}) scale({scale}) translate(-60 -50)");
    view! {
        <g transform=transform>
            <circle cx="60" cy="50" r="34" fill="#c7d2fe" opacity="0.1"></circle>
            <path d="M78 24a28 28 0 1 1-36 46A33 33 0 0 0 78 24Z" fill="url(#fwMoon)"></path>
            <circle cx="88" cy="22" r="1.8" fill="#e0e7ff" class="animate-pulse-soft"></circle>
            <circle cx="96" cy="38" r="1.2" fill="#e0e7ff" class="animate-pulse-soft" style="animation-delay:.7s"></circle>
            <circle cx="84" cy="44" r="1" fill="#e0e7ff" class="animate-pulse-soft" style="animation-delay:1.2s"></circle>
        </g>
    }
    .into_any()
}

fn cloud() -> AnyView {
    view! {
        <g>
            <ellipse cx="48" cy="74" rx="21" ry="18" fill="url(#fwCloud)"></ellipse>
            <ellipse cx="68" cy="63" rx="25" ry="22" fill="url(#fwCloud)"></ellipse>
            <ellipse cx="88" cy="74" rx="19" ry="16" fill="url(#fwCloud)"></ellipse>
            <rect x="29" y="70" width="73" height="23" rx="11.5" fill="url(#fwCloud)"></rect>
            <ellipse cx="56" cy="58" rx="14" ry="9" fill="#ffffff" opacity="0.22"></ellipse>
        </g>
    }
    .into_any()
}

fn drops(count: usize, scatter: f64) -> AnyView {
    let xs = [42.0, 60.0, 78.0, 51.0, 69.0];
    let mut items = Vec::new();
    for i in 0..count {
        let x = xs[i % xs.len()] + (i as f64 % 2.0) * scatter;
        let delay = i as f64 * 0.18;
        items.push(view! {
            <line
                x1=x
                y1="94"
                x2=x - 3.0
                y2="107"
                stroke="#93c5fd"
                stroke-width="3.2"
                stroke-linecap="round"
                class="animate-rain"
                style=format!("animation-delay:{delay:.2}s")
            ></line>
        });
    }
    view! { <g>{items.into_iter().collect_view()}</g> }.into_any()
}

fn flakes(count: usize) -> AnyView {
    let positions = [(40.0, 9.0), (62.0, 6.0), (82.0, 8.0)];
    let mut items = Vec::new();
    for i in 0..count {
        let (x, y) = positions[i % positions.len()];
        let delay = i as f64 * 0.6;
        items.push(view! {
            <g class="animate-snow" style=format!("animation-delay:{delay:.2}s") transform=format!("translate({x} {y})")>
                <path d="M0 -6V6M-5.2 -3 5.2 3M-5.2 3 5.2 -3" stroke="#e0f2fe" stroke-width="2" stroke-linecap="round"></path>
            </g>
        });
    }
    view! { <g>{items.into_iter().collect_view()}</g> }.into_any()
}
