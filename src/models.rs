use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Proveedores y unidades
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provider {
    OpenMeteo,
    MetNo,
}

impl Default for Provider {
    fn default() -> Self {
        Self::OpenMeteo
    }
}

impl Provider {
    pub const ALL: [Provider; 2] = [Provider::OpenMeteo, Provider::MetNo];

    pub fn id(self) -> &'static str {
        match self {
            Self::OpenMeteo => "open-meteo",
            Self::MetNo => "met-no",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::OpenMeteo => "Open-Meteo",
            Self::MetNo => "MET Norway",
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            Self::OpenMeteo => "Open-Meteo",
            Self::MetNo => "MET.no",
        }
    }

    pub fn model(self) -> &'static str {
        match self {
            Self::OpenMeteo => "Open-Meteo · best-match",
            Self::MetNo => "MET Norway · Locationforecast 2.0",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::OpenMeteo => "Modelo global de alta resolución. Pronóstico horario y diario.",
            Self::MetNo => "Modelo nórdico del Instituto Meteorológico de Noruega, cobertura mundial.",
        }
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "met-no" => Self::MetNo,
            _ => Self::OpenMeteo,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Units {
    Celsius,
    Fahrenheit,
}

impl Default for Units {
    fn default() -> Self {
        Self::Celsius
    }
}

impl Units {
    pub fn symbol(self) -> &'static str {
        match self {
            Self::Celsius => "C",
            Self::Fahrenheit => "F",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Celsius => "°C",
            Self::Fahrenheit => "°F",
        }
    }

    pub fn temp(self, celsius: f64) -> f64 {
        match self {
            Self::Celsius => celsius,
            Self::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        }
    }

    pub fn temp_str(self, celsius: f64) -> String {
        format!("{}°", self.temp(celsius).round() as i64)
    }

    pub fn temp_dec(self, celsius: f64, decimals: usize) -> String {
        format!("{:.*}°", decimals, self.temp(celsius))
    }

    pub fn delta_str(self, celsius_delta: f64) -> String {
        let value = match self {
            Self::Celsius => celsius_delta,
            Self::Fahrenheit => celsius_delta * 9.0 / 5.0,
        };
        format!("{:.0}°", value.abs())
    }

    pub fn from_id(id: &str) -> Self {
        match id {
            "f" => Self::Fahrenheit,
            _ => Self::Celsius,
        }
    }
}

// ---------------------------------------------------------------------------
// Ubicación
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    #[serde(default)]
    pub admin: Option<String>,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub country_code: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default = "default_timezone")]
    pub timezone: String,
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[serde(default)]
    pub population: Option<u64>,
    #[serde(default)]
    pub is_current: bool,
}

fn default_timezone() -> String {
    "UTC".to_string()
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: "Córdoba".to_string(),
            admin: Some("Provincia de Córdoba".to_string()),
            country: Some("Argentina".to_string()),
            country_code: Some("AR".to_string()),
            latitude: -31.4135,
            longitude: -64.1811,
            timezone: "America/Argentina/Cordoba".to_string(),
            accuracy: None,
            population: Some(2_106_734),
            is_current: false,
        }
    }
}

impl Location {
    pub fn coordinates(&self) -> String {
        format!("{:.3}°, {:.3}°", self.latitude, self.longitude)
    }

    pub fn place_line(&self) -> String {
        let mut parts: Vec<&str> = Vec::new();
        if let Some(admin) = self.admin.as_deref() {
            if !admin.is_empty() && admin != self.name {
                parts.push(admin);
            }
        }
        if let Some(country) = self.country.as_deref() {
            if !country.is_empty() {
                parts.push(country);
            }
        }
        if parts.is_empty() {
            self.coordinates()
        } else {
            parts.join(" · ")
        }
    }

    pub fn same_place(&self, other: &Location) -> bool {
        (self.latitude - other.latitude).abs() < 0.02
            && (self.longitude - other.longitude).abs() < 0.02
            && self.name.eq_ignore_ascii_case(&other.name)
    }
}

// ---------------------------------------------------------------------------
// Modelo meteorológico unificado
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Weather {
    pub current: Current,
    pub hourly: Vec<Hour>,
    pub daily: Vec<Day>,
    pub timezone: String,
    pub utc_offset_seconds: i32,
    pub fetched_at_ms: f64,
    pub provider: Provider,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Debug)]
pub struct Current {
    pub time: String,
    pub date: String,
    pub temperature: f64,
    pub apparent: Option<f64>,
    pub humidity: Option<f64>,
    pub precipitation: f64,
    pub precipitation_probability: Option<f64>,
    pub weather_code: u8,
    pub is_day: bool,
    pub wind_speed: Option<f64>,
    pub wind_gust: Option<f64>,
    pub wind_direction: Option<f64>,
    pub pressure: Option<f64>,
    pub cloud_cover: Option<f64>,
    pub visibility_km: Option<f64>,
    pub uv_index: Option<f64>,
    pub dew_point: Option<f64>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Hour {
    pub time: String,
    pub date: String,
    pub temperature: f64,
    pub apparent: Option<f64>,
    pub humidity: Option<f64>,
    pub precipitation_probability: Option<f64>,
    pub precipitation: f64,
    pub weather_code: u8,
    pub is_day: bool,
    pub wind_speed: Option<f64>,
    pub wind_gust: Option<f64>,
    pub wind_direction: Option<f64>,
    pub pressure: Option<f64>,
    pub cloud_cover: Option<f64>,
    pub visibility_km: Option<f64>,
    pub uv_index: Option<f64>,
    pub dew_point: Option<f64>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Day {
    pub date: String,
    pub weather_code: u8,
    pub temp_max: f64,
    pub temp_min: f64,
    pub apparent_max: Option<f64>,
    pub apparent_min: Option<f64>,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub uv_max: Option<f64>,
    pub precipitation_sum: Option<f64>,
    pub precipitation_probability: Option<f64>,
    pub wind_max: Option<f64>,
    pub wind_gust_max: Option<f64>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct AirQuality {
    pub european_aqi: Option<f64>,
    pub us_aqi: Option<f64>,
    pub pm2_5: Option<f64>,
    pub pm10: Option<f64>,
    pub ozone: Option<f64>,
    pub nitrogen_dioxide: Option<f64>,
    pub sulphur_dioxide: Option<f64>,
    pub carbon_monoxide: Option<f64>,
    pub time: String,
}

impl AirQuality {
    pub fn is_empty(&self) -> bool {
        self.european_aqi.is_none()
            && self.us_aqi.is_none()
            && self.pm2_5.is_none()
            && self.pm10.is_none()
    }
}

// ---------------------------------------------------------------------------
// Condiciones WMO
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sky {
    Clear,
    MostlyClear,
    PartlyCloudy,
    Cloudy,
    Fog,
    Drizzle,
    Rain,
    HeavRain,
    Showers,
    Snow,
    Sleet,
    Thunder,
}

#[derive(Clone, Copy, Debug)]
pub struct Condition {
    pub label: &'static str,
    pub sky: Sky,
}

pub fn condition(code: u8) -> Condition {
    let (label, sky) = match code {
        0 => ("Despejado", Sky::Clear),
        1 => ("Mayormente despejado", Sky::MostlyClear),
        2 => ("Parcialmente nublado", Sky::PartlyCloudy),
        3 => ("Nublado", Sky::Cloudy),
        45 | 48 => ("Niebla", Sky::Fog),
        51 => ("Llovizna ligera", Sky::Drizzle),
        53 => ("Llovizna moderada", Sky::Drizzle),
        55 => ("Llovizna intensa", Sky::Drizzle),
        56 | 57 => ("Llovizna helada", Sky::Sleet),
        61 => ("Lluvia ligera", Sky::Rain),
        63 => ("Lluvia moderada", Sky::Rain),
        65 => ("Lluvia intensa", Sky::HeavRain),
        66 | 67 => ("Lluvia helada", Sky::Sleet),
        71 => ("Nevada ligera", Sky::Snow),
        73 => ("Nevada moderada", Sky::Snow),
        75 => ("Nevada intensa", Sky::Snow),
        77 => ("Granos de nieve", Sky::Snow),
        80 => ("Chubascos ligeros", Sky::Showers),
        81 => ("Chubascos moderados", Sky::Showers),
        82 => ("Chubascos intensos", Sky::HeavRain),
        85 => ("Chubascos de nieve", Sky::Snow),
        86 => ("Chubascos de nieve intensos", Sky::Snow),
        95 => ("Tormenta", Sky::Thunder),
        96 => ("Tormenta con granizo", Sky::Thunder),
        99 => ("Tormenta fuerte con granizo", Sky::Thunder),
        _ => ("Condición desconocida", Sky::Cloudy),
    };
    Condition { label, sky }
}

/// Convierte un código de símbolo de MET Norway a un código WMO aproximado.
pub fn symbol_to_wmo(symbol: &str) -> u8 {
    let s = symbol
        .trim_end_matches("_day")
        .trim_end_matches("_night")
        .trim_end_matches("_polartwilight")
        .to_ascii_lowercase();

    match s.as_str() {
        "clearsky" => 0,
        "fair" => 1,
        "partlycloudy" => 2,
        "cloudy" => 3,
        "fog" => 45,
        "lightrain" => 61,
        "rain" => 63,
        "heavyrain" => 65,
        "lightrainshowers" => 80,
        "rainshowers" => 80,
        "heavyrainshowers" => 82,
        "lightrainshowersandthunder" | "rainshowersandthunder" | "heavyrainshowersandthunder" => 95,
        "lightsleet" => 66,
        "sleet" => 67,
        "heavysleet" => 67,
        "lightsnow" => 71,
        "snow" => 73,
        "heavysnow" => 75,
        "lightsnowshowers" => 85,
        "snowshowers" => 85,
        "heavysnowshowers" => 86,
        "lightsleetshowers" | "sleetshowers" | "heavysleetshowers" => 67,
        "lightrainandthunder" | "rainandthunder" | "heavyrainandthunder" => 95,
        "lightsnowandthunder" | "snowandthunder" | "heavysnowandthunder" => 95,
        "sleetandthunder" | "heavysleetandthunder" | "lightsleetandthunder" => 95,
        "lightssleetshowersandthunder"
        | "sleetshowersandthunder"
        | "heavysleetshowersandthunder"
        | "lightssnowshowersandthunder"
        | "snowshowersandthunder"
        | "heavysnowshowersandthunder" => 95,
        _ => 3,
    }
}

pub fn is_night_symbol(symbol: &str) -> Option<bool> {
    if symbol.ends_with("_night") {
        Some(true)
    } else if symbol.ends_with("_day") {
        Some(false)
    } else {
        None
    }
}
