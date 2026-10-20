use chrono::{Datelike, NaiveDate};
use std::f64::consts::PI;

use crate::models::Sky;

// ---------------------------------------------------------------------------
// Fechas y horas (las horas llegan ya en la zona horaria de la ciudad)
// ---------------------------------------------------------------------------

const WEEKDAYS_SHORT: [&str; 7] = ["lun", "mar", "mié", "jue", "vie", "sáb", "dom"];
const WEEKDAYS_LONG: [&str; 7] = [
    "lunes",
    "martes",
    "miércoles",
    "jueves",
    "viernes",
    "sábado",
    "domingo",
];
const MONTHS: [&str; 12] = [
    "enero",
    "febrero",
    "marzo",
    "abril",
    "mayo",
    "junio",
    "julio",
    "agosto",
    "septiembre",
    "octubre",
    "noviembre",
    "diciembre",
];

pub fn parse_date(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
}

pub fn short_time(local_iso: &str) -> String {
    let time = local_iso.get(11..16).unwrap_or("--:--");
    time.to_string()
}

pub fn day_label(date: &str, index: usize) -> String {
    if index == 0 {
        return "Hoy".to_string();
    }
    if index == 1 {
        return "Mañana".to_string();
    }
    match parse_date(date) {
        Some(d) => format!(
            "{} {}",
            WEEKDAYS_SHORT[d.weekday().num_days_from_monday() as usize],
            d.day()
        ),
        None => date.to_string(),
    }
}

pub fn weekday_short(date: &str) -> String {
    match parse_date(date) {
        Some(d) => WEEKDAYS_SHORT[d.weekday().num_days_from_monday() as usize].to_string(),
        None => "—".to_string(),
    }
}

pub fn weekday_long(date: &str) -> String {
    match parse_date(date) {
        Some(d) => WEEKDAYS_LONG[d.weekday().num_days_from_monday() as usize].to_string(),
        None => "—".to_string(),
    }
}

pub fn long_date(date: &str) -> String {
    match parse_date(date) {
        Some(d) => format!(
            "{}, {} de {}",
            WEEKDAYS_LONG[d.weekday().num_days_from_monday() as usize],
            d.day(),
            MONTHS[(d.month0() as usize).min(11)]
        ),
        None => String::new(),
    }
}

pub fn updated_label(fetched_at_ms: f64, now_ms: f64) -> String {
    let seconds = ((now_ms - fetched_at_ms) / 1000.0).max(0.0);
    if seconds < 45.0 {
        "ahora mismo".to_string()
    } else if seconds < 3600.0 {
        format!("hace {} min", (seconds / 60.0).round() as i64)
    } else if seconds < 86_400.0 {
        format!("hace {} h", (seconds / 3600.0).round() as i64)
    } else {
        format!("hace {} d", (seconds / 86_400.0).round() as i64)
    }
}

// ---------------------------------------------------------------------------
// Viento, UV, presión, calidad del aire
// ---------------------------------------------------------------------------

const COMPASS: [&str; 8] = ["N", "NE", "E", "SE", "S", "SO", "O", "NO"];

pub fn wind_direction_label(degrees: f64) -> &'static str {
    let normalized = (degrees % 360.0 + 360.0) % 360.0;
    let index = ((normalized + 22.5) / 45.0).floor() as usize % 8;
    COMPASS[index]
}

pub fn wind_description(speed_kmh: f64) -> &'static str {
    match speed_kmh {
        s if s < 1.0 => "En calma",
        s if s < 12.0 => "Brisa suave",
        s if s < 20.0 => "Viento moderado",
        s if s < 29.0 => "Viento fresco",
        s if s < 39.0 => "Viento fuerte",
        s if s < 62.0 => "Viento muy fuerte",
        _ => "Tempestad",
    }
}

pub fn uv_category(value: f64) -> (&'static str, &'static str, &'static str) {
    // (nombre, color de texto, color de barra)
    if value < 3.0 {
        ("Bajo", "text-emerald-300", "from-emerald-400 to-emerald-300")
    } else if value < 6.0 {
        ("Moderado", "text-yellow-300", "from-yellow-400 to-amber-300")
    } else if value < 8.0 {
        ("Alto", "text-orange-300", "from-orange-400 to-amber-400")
    } else if value < 11.0 {
        ("Muy alto", "text-rose-300", "from-rose-500 to-orange-400")
    } else {
        ("Extremo", "text-fuchsia-300", "from-fuchsia-500 to-rose-400")
    }
}

pub fn aqi_category(value: f64) -> (&'static str, &'static str, &'static str) {
    // Índice Europeo de Calidad del Aire (EAQI)
    if value <= 20.0 {
        ("Buena", "text-emerald-300", "from-emerald-400 to-teal-300")
    } else if value <= 40.0 {
        ("Aceptable", "text-lime-300", "from-lime-400 to-emerald-300")
    } else if value <= 60.0 {
        ("Moderada", "text-yellow-300", "from-yellow-400 to-amber-300")
    } else if value <= 80.0 {
        ("Mala", "text-orange-300", "from-orange-500 to-amber-400")
    } else if value <= 100.0 {
        ("Muy mala", "text-rose-300", "from-rose-500 to-orange-400")
    } else {
        ("Extrema", "text-fuchsia-300", "from-fuchsia-500 to-rose-500")
    }
}

pub fn pressure_status(hpa: f64) -> (&'static str, u8) {
    if hpa < 1000.0 {
        ("Baja", 30)
    } else if hpa <= 1020.0 {
        ("Normal", 55)
    } else {
        ("Alta", 80)
    }
}

pub fn pressure_trend(current: f64, in_six_hours: Option<f64>) -> &'static str {
    match in_six_hours {
        Some(next) if next - current > 0.8 => "Subiendo",
        Some(next) if current - next > 0.8 => "Bajando",
        Some(_) => "Estable",
        None => "Sin tendencia",
    }
}

pub fn humidity_description(humidity: f64) -> &'static str {
    match humidity {
        h if h < 30.0 => "Aire seco",
        h if h < 50.0 => "Confortable",
        h if h < 70.0 => "Algo húmedo",
        h if h < 85.0 => "Húmedo",
        _ => "Muy húmedo",
    }
}

pub fn visibility_description(km: f64) -> &'static str {
    match km {
        v if v >= 30.0 => "Visibilidad excelente",
        v if v >= 15.0 => "Buena visibilidad",
        v if v >= 8.0 => "Visibilidad moderada",
        v if v >= 3.0 => "Neblina",
        _ => "Visibilidad reducida",
    }
}

// ---------------------------------------------------------------------------
// Solemne geometría solar, sin APIs extra
// ---------------------------------------------------------------------------

/// Devuelve (amanecer, atardecer) en HH:MM local. `offset_seconds` es el huso
/// horario aplicable a esa fecha (incluye horario de verano).
pub fn solar_times(
    date: NaiveDate,
    latitude: f64,
    longitude: f64,
    offset_seconds: i32,
) -> (Option<String>, Option<String>) {
    let n = date.ordinal() as f64;
    let gamma = 2.0 * PI / 365.0 * (n - 1.0);

    let eqtime = 229.18
        * (0.000075 + 0.001868 * gamma.cos()
            - 0.032077 * gamma.sin()
            - 0.014615 * (2.0 * gamma).cos()
            - 0.040849 * (2.0 * gamma).sin());

    let decl = 0.006918 - 0.399912 * gamma.cos() + 0.070257 * gamma.sin()
        - 0.006758 * (2.0 * gamma).cos()
        + 0.000907 * (2.0 * gamma).sin()
        - 0.002697 * (3.0 * gamma).cos()
        + 0.00148 * (3.0 * gamma).sin();

    let lat_rad = latitude.to_radians();
    let zenith = 90.833_f64.to_radians();
    let cos_ha = (zenith.cos() / (lat_rad.cos() * decl.cos()))
        - lat_rad.tan() * decl.tan();

    if !(-1.0..=1.0).contains(&cos_ha) {
        return (None, None);
    }

    let ha_deg = cos_ha.acos().to_degrees();
    let offset_min = offset_seconds as f64 / 60.0;

    let sunrise_utc = 720.0 - 4.0 * (longitude + ha_deg) - eqtime;
    let sunset_utc = 720.0 - 4.0 * (longitude - ha_deg) - eqtime;

    (
        Some(minutes_to_local(sunrise_utc, offset_min)),
        Some(minutes_to_local(sunset_utc, offset_min)),
    )
}

fn minutes_to_local(minutes_utc: f64, offset_min: f64) -> String {
    let mut total = (minutes_utc + offset_min).round();
    total = total.rem_euclid(1440.0);
    let hours = (total / 60.0).floor() as u32;
    let mins = (total % 60.0) as u32;
    format!("{hours:02}:{mins:02}")
}

/// Fracción del día transcurrida y posición sobre el arco solar (viewBox 130x60).
pub fn sun_arc_position(
    sunrise: &str,
    sunset: &str,
    current: &str,
) -> (f64, Option<(f64, f64)>) {
    let to_minutes = |s: &str| -> Option<f64> {
        let mut parts = s.split(':');
        let h: f64 = parts.next()?.parse().ok()?;
        let m: f64 = parts.next()?.parse().ok()?;
        Some(h * 60.0 + m)
    };

    let (Some(rise), Some(set), Some(now)) = (
        to_minutes(sunrise),
        to_minutes(sunset),
        to_minutes(current),
    ) else {
        return (0.0, None);
    };

    if set <= rise {
        return (0.0, None);
    }

    let fraction = ((now - rise) / (set - rise)).clamp(0.0, 1.0);
    let x = 10.0 + 110.0 * fraction;
    let y = 52.0 - 4.0 * 58.0 * fraction * (1.0 - fraction);
    (fraction, Some((x, y)))
}

// ---------------------------------------------------------------------------
// Luna
// ---------------------------------------------------------------------------

pub struct MoonInfo {
    pub illumination: f64,
    pub name: &'static str,
    pub waxing: bool,
}

pub fn moon_phase(date: NaiveDate) -> MoonInfo {
    let reference = NaiveDate::from_ymd_opt(2000, 1, 6).unwrap_or(date);
    let days = (date - reference).num_days() as f64 - 0.26;
    let phase = (days / 29.530_588_853).rem_euclid(1.0);
    let illumination = (1.0 - (2.0 * PI * phase).cos()) / 2.0;

    let name = if phase < 0.03 || phase > 0.97 {
        "Luna nueva"
    } else if phase < 0.22 {
        "Luna creciente"
    } else if phase < 0.28 {
        "Cuarto creciente"
    } else if phase < 0.47 {
        "Gibosa creciente"
    } else if phase < 0.53 {
        "Luna llena"
    } else if phase < 0.72 {
        "Gibosa menguante"
    } else if phase < 0.78 {
        "Cuarto menguante"
    } else {
        "Luna menguante"
    };

    MoonInfo {
        illumination,
        name,
        waxing: phase < 0.5,
    }
}

// ---------------------------------------------------------------------------
// Estética
// ---------------------------------------------------------------------------

pub fn hero_gradient(sky: Sky, is_day: bool) -> &'static str {
    match (sky, is_day) {
        (Sky::Clear, true) => "from-[#17506f] via-[#12405f] to-[#0a1b34]",
        (Sky::Clear, false) => "from-[#1c2a5e] via-[#121f44] to-[#070d1f]",
        (Sky::MostlyClear, true) => "from-[#1a5372] via-[#153f5f] to-[#0a1b34]",
        (Sky::MostlyClear, false) => "from-[#1c2a58] via-[#121e40] to-[#070d1f]",
        (Sky::PartlyCloudy, true) => "from-[#23526b] via-[#1a3d58] to-[#0b1c32]",
        (Sky::PartlyCloudy, false) => "from-[#202b54] via-[#151f3d] to-[#080d1e]",
        (Sky::Cloudy, _) => "from-[#2a3d54] via-[#1b2b43] to-[#0b1730]",
        (Sky::Fog, _) => "from-[#3a4656] via-[#263243] to-[#0e1725]",
        (Sky::Drizzle, _) | (Sky::Rain, _) => "from-[#243d57] via-[#172b45] to-[#08121f]",
        (Sky::HeavRain, _) | (Sky::Showers, _) => "from-[#20364e] via-[#15263d] to-[#070f1c]",
        (Sky::Thunder, _) => "from-[#302750] via-[#1d1a3b] to-[#0a0b1d]",
        (Sky::Snow, _) | (Sky::Sleet, _) => "from-[#2d4a61] via-[#1b3348] to-[#0a1a2a]",
    }
}

pub fn hero_glow(sky: Sky, is_day: bool) -> &'static str {
    match (sky, is_day) {
        (Sky::Clear | Sky::MostlyClear, true) => "bg-amber-300/25",
        (Sky::Clear | Sky::MostlyClear, false) => "bg-indigo-400/20",
        (Sky::Thunder, _) => "bg-violet-400/20",
        (Sky::Snow | Sky::Sleet, _) => "bg-cyan-200/20",
        (Sky::Rain | Sky::HeavRain | Sky::Showers | Sky::Drizzle, _) => "bg-sky-400/15",
        (Sky::Fog, _) => "bg-slate-300/15",
        _ => "bg-sky-400/15",
    }
}

pub fn format_population(pop: u64) -> String {
    if pop >= 1_000_000 {
        format!("{:.1} M hab.", pop as f64 / 1_000_000.0)
    } else if pop >= 1_000 {
        format!("{:.0} mil hab.", pop as f64 / 1_000.0)
    } else {
        format!("{pop} hab.")
    }
}

pub fn flag_emoji(country_code: Option<&str>) -> String {
    let Some(code) = country_code else {
        return "🌍".to_string();
    };
    let chars: Vec<char> = code
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .take(2)
        .flat_map(|c| {
            std::char::from_u32(0x1F1E6 + (c.to_ascii_uppercase() as u32 - 'A' as u32))
        })
        .collect();
    if chars.len() == 2 {
        chars.into_iter().collect()
    } else {
        "🌍".to_string()
    }
}

/// Curva suave (Catmull-Rom -> Bézier cúbica) para los gráficos SVG.
pub fn smooth_path(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut path = format!("M {:.2} {:.2}", points[0].0, points[0].1);
    for i in 0..points.len().saturating_sub(1) {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = points.get(i + 2).copied().unwrap_or(p2);
        let c1 = (
            p1.0 + (p2.0 - p0.0) / 6.0,
            p1.1 + (p2.1 - p0.1) / 6.0,
        );
        let c2 = (
            p2.0 - (p3.0 - p1.0) / 6.0,
            p2.1 - (p3.1 - p1.1) / 6.0,
        );
        path.push_str(&format!(
            " C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
            c1.0, c1.1, c2.0, c2.1, p2.0, p2.1
        ));
    }
    path
}

