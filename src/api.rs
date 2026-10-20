use chrono::{NaiveDate, Timelike, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;

use crate::format::solar_times;
use crate::js;
use crate::models::{
    is_night_symbol, symbol_to_wmo, AirQuality, Current, Day, Hour, Location, Provider, Weather,
};

const OPEN_METEO_FORECAST: &str = "https://api.open-meteo.com/v1/forecast";
const OPEN_METEO_GEOCODING: &str = "https://geocoding-api.open-meteo.com/v1/search";
const OPEN_METEO_AIR: &str = "https://air-quality-api.open-meteo.com/v1/air-quality";
const MET_NO: &str = "https://api.met.no/weatherapi/locationforecast/2.0/complete";
const NOMINATIM: &str = "https://nominatim.openstreetmap.org/reverse";

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .build()
        .map_err(|e| format!("No se pudo inicializar el cliente HTTP: {e}"))
}

async fn get_json<T: DeserializeOwned>(url: &str, source: &str) -> Result<T, String> {
    let response = client()?
        .get(url)
        .timeout(Duration::from_secs(18))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| {
            let detail = if e.is_timeout() {
                "la solicitud tardó demasiado".to_string()
            } else {
                e.to_string()
            };
            format!(
                "No se pudo consultar {source}. Revisá tu conexión o el bloqueador de contenido ({detail})."
            )
        })?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        let snippet: String = body.chars().take(160).collect();
        return Err(format!(
            "{source} respondió HTTP {}. {}",
            status.as_u16(),
            snippet.trim()
        ));
    }

    response
        .json::<T>()
        .await
        .map_err(|e| format!("{source} devolvió una respuesta inesperada: {e}"))
}

// ---------------------------------------------------------------------------
// Open-Meteo
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct OmResponse {
    latitude: f64,
    longitude: f64,
    timezone: String,
    utc_offset_seconds: i32,
    current: OmCurrent,
    hourly: OmHourly,
    daily: OmDaily,
}

#[derive(Deserialize)]
struct OmCurrent {
    time: String,
    temperature_2m: Option<f64>,
    relative_humidity_2m: Option<f64>,
    apparent_temperature: Option<f64>,
    is_day: Option<u8>,
    precipitation: Option<f64>,
    weather_code: Option<u8>,
    cloud_cover: Option<f64>,
    pressure_msl: Option<f64>,
    wind_speed_10m: Option<f64>,
    wind_direction_10m: Option<f64>,
    wind_gusts_10m: Option<f64>,
}

#[derive(Deserialize)]
struct OmHourly {
    time: Vec<String>,
    temperature_2m: Vec<Option<f64>>,
    relative_humidity_2m: Vec<Option<f64>>,
    apparent_temperature: Vec<Option<f64>>,
    precipitation_probability: Vec<Option<f64>>,
    precipitation: Vec<Option<f64>>,
    weather_code: Vec<Option<u8>>,
    cloud_cover: Vec<Option<f64>>,
    pressure_msl: Vec<Option<f64>>,
    wind_speed_10m: Vec<Option<f64>>,
    wind_direction_10m: Vec<Option<f64>>,
    wind_gusts_10m: Vec<Option<f64>>,
    uv_index: Vec<Option<f64>>,
    is_day: Vec<Option<u8>>,
    visibility: Vec<Option<f64>>,
    dew_point_2m: Vec<Option<f64>>,
}

#[derive(Deserialize)]
struct OmDaily {
    time: Vec<String>,
    weather_code: Vec<Option<u8>>,
    temperature_2m_max: Vec<Option<f64>>,
    temperature_2m_min: Vec<Option<f64>>,
    apparent_temperature_max: Vec<Option<f64>>,
    apparent_temperature_min: Vec<Option<f64>>,
    sunrise: Vec<Option<String>>,
    sunset: Vec<Option<String>>,
    uv_index_max: Vec<Option<f64>>,
    precipitation_sum: Vec<Option<f64>>,
    precipitation_probability_max: Vec<Option<f64>>,
    wind_speed_10m_max: Vec<Option<f64>>,
    wind_gusts_10m_max: Vec<Option<f64>>,
}

fn at<T: Copy>(values: &[Option<T>], index: usize) -> Option<T> {
    values.get(index).copied().flatten()
}

fn hm_from_iso(value: &str) -> Option<String> {
    value.get(11..16).map(|s| s.to_string())
}

fn om_url(location: &Location) -> String {
    format!(
        "{OPEN_METEO_FORECAST}?latitude={:.5}&longitude={:.5}\
         &current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,weather_code,cloud_cover,pressure_msl,wind_speed_10m,wind_direction_10m,wind_gusts_10m\
         &hourly=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,cloud_cover,pressure_msl,wind_speed_10m,wind_direction_10m,wind_gusts_10m,uv_index,is_day,visibility,dew_point_2m\
         &daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,precipitation_sum,precipitation_probability_max,wind_speed_10m_max,wind_gusts_10m_max\
         &timezone=auto&forecast_days=7&wind_speed_unit=kmh",
        location.latitude, location.longitude
    )
}

async fn fetch_open_meteo(location: &Location) -> Result<Weather, String> {
    let data: OmResponse = get_json(&om_url(location), "Open-Meteo").await?;

    let current_time = data.current.time.clone();
    let current_date = current_time
        .get(0..10)
        .unwrap_or_default()
        .to_string();

    let current_hour_idx = data
        .hourly
        .time
        .iter()
        .position(|t| t == &current_time)
        .or_else(|| data.hourly.time.iter().position(|t| t.as_str() >= current_time.as_str()))
        .unwrap_or(0);

    let start = data
        .hourly
        .time
        .iter()
        .position(|t| t.as_str() >= current_time.as_str())
        .unwrap_or(0);

    let hourly: Vec<Hour> = (start..data.hourly.time.len())
        .filter_map(|i| {
            let temperature = at(&data.hourly.temperature_2m, i)?;
            Some(Hour {
                time: data.hourly.time[i].clone(),
                date: data.hourly.time[i].get(0..10).unwrap_or_default().to_string(),
                temperature,
                apparent: at(&data.hourly.apparent_temperature, i),
                humidity: at(&data.hourly.relative_humidity_2m, i),
                precipitation_probability: at(&data.hourly.precipitation_probability, i),
                precipitation: at(&data.hourly.precipitation, i).unwrap_or(0.0),
                weather_code: at(&data.hourly.weather_code, i).unwrap_or(3),
                is_day: at(&data.hourly.is_day, i).unwrap_or(1) != 0,
                wind_speed: at(&data.hourly.wind_speed_10m, i),
                wind_gust: at(&data.hourly.wind_gusts_10m, i),
                wind_direction: at(&data.hourly.wind_direction_10m, i),
                pressure: at(&data.hourly.pressure_msl, i),
                cloud_cover: at(&data.hourly.cloud_cover, i),
                visibility_km: at(&data.hourly.visibility, i).map(|m| m / 1000.0),
                uv_index: at(&data.hourly.uv_index, i),
                dew_point: at(&data.hourly.dew_point_2m, i),
            })
        })
        .take(48)
        .collect();

    let temperature = data
        .current
        .temperature_2m
        .ok_or_else(|| "Open-Meteo no devolvió la temperatura actual".to_string())?;

    let current = Current {
        time: current_time,
        date: current_date.clone(),
        temperature,
        apparent: data.current.apparent_temperature.or_else(|| {
            at(&data.hourly.apparent_temperature, current_hour_idx)
        }),
        humidity: data
            .current
            .relative_humidity_2m
            .or_else(|| at(&data.hourly.relative_humidity_2m, current_hour_idx)),
        precipitation: data.current.precipitation.unwrap_or(0.0),
        precipitation_probability: at(&data.hourly.precipitation_probability, current_hour_idx),
        weather_code: data.current.weather_code.unwrap_or(3),
        is_day: data.current.is_day.unwrap_or(1) != 0,
        wind_speed: data.current.wind_speed_10m,
        wind_gust: data.current.wind_gusts_10m,
        wind_direction: data.current.wind_direction_10m,
        pressure: data.current.pressure_msl,
        cloud_cover: data.current.cloud_cover,
        visibility_km: at(&data.hourly.visibility, current_hour_idx).map(|m| m / 1000.0),
        uv_index: at(&data.hourly.uv_index, current_hour_idx),
        dew_point: at(&data.hourly.dew_point_2m, current_hour_idx),
    };

    let mut daily: Vec<Day> = Vec::new();
    for i in 0..data.daily.time.len() {
        let (Some(max), Some(min)) = (
            at(&data.daily.temperature_2m_max, i),
            at(&data.daily.temperature_2m_min, i),
        ) else {
            continue;
        };
        let date = data.daily.time[i].clone();
        let fallback = date
            .parse::<NaiveDate>()
            .ok()
            .map(|d| solar_times(d, data.latitude, data.longitude, data.utc_offset_seconds))
            .unwrap_or((None, None));
        daily.push(Day {
            date,
            weather_code: at(&data.daily.weather_code, i).unwrap_or(3),
            temp_max: max,
            temp_min: min,
            apparent_max: at(&data.daily.apparent_temperature_max, i),
            apparent_min: at(&data.daily.apparent_temperature_min, i),
            sunrise: data
                .daily
                .sunrise
                .get(i)
                .and_then(|v| v.as_deref())
                .and_then(hm_from_iso)
                .or(fallback.0),
            sunset: data
                .daily
                .sunset
                .get(i)
                .and_then(|v| v.as_deref())
                .and_then(hm_from_iso)
                .or(fallback.1),
            uv_max: at(&data.daily.uv_index_max, i),
            precipitation_sum: at(&data.daily.precipitation_sum, i),
            precipitation_probability: at(&data.daily.precipitation_probability_max, i),
            wind_max: at(&data.daily.wind_speed_10m_max, i),
            wind_gust_max: at(&data.daily.wind_gusts_10m_max, i),
        });
    }

    if daily.is_empty() {
        return Err("Open-Meteo no devolvió pronóstico diario".to_string());
    }

    Ok(Weather {
        current,
        hourly,
        daily,
        timezone: data.timezone,
        utc_offset_seconds: data.utc_offset_seconds,
        fetched_at_ms: js_sys::Date::now(),
        provider: Provider::OpenMeteo,
        latitude: data.latitude,
        longitude: data.longitude,
    })
}

// ---------------------------------------------------------------------------
// MET Norway (MET.no)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct MetResponse {
    properties: MetProperties,
}

#[derive(Deserialize)]
struct MetProperties {
    timeseries: Vec<MetSeries>,
}

#[derive(Deserialize)]
struct MetSeries {
    time: String,
    data: MetData,
}

#[derive(Deserialize, Default)]
struct MetData {
    #[serde(default)]
    instant: MetInstant,
    #[serde(default)]
    next_1_hours: Option<MetNext>,
    #[serde(default)]
    next_6_hours: Option<MetNext>,
}

#[derive(Deserialize, Default)]
struct MetInstant {
    #[serde(default)]
    details: MetDetails,
}

#[derive(Deserialize, Default)]
struct MetDetails {
    air_temperature: Option<f64>,
    relative_humidity: Option<f64>,
    wind_speed: Option<f64>,
    wind_from_direction: Option<f64>,
    air_pressure_at_sea_level: Option<f64>,
    cloud_area_fraction: Option<f64>,
    dew_point_temperature: Option<f64>,
    ultraviolet_index_clear_sky: Option<f64>,
    apparent_air_temperature: Option<f64>,
}

#[derive(Deserialize, Default)]
struct MetNext {
    #[serde(default)]
    summary: MetSummary,
    #[serde(default)]
    details: MetNextDetails,
}

#[derive(Deserialize, Default)]
struct MetSummary {
    symbol_code: Option<String>,
}

#[derive(Deserialize, Default)]
struct MetNextDetails {
    precipitation_amount: Option<f64>,
    air_temperature_max: Option<f64>,
    air_temperature_min: Option<f64>,
}

struct MetPoint {
    local_iso: String,
    date: String,
    hour: u32,
    temperature: f64,
    apparent: Option<f64>,
    humidity: Option<f64>,
    wind_kmh: Option<f64>,
    wind_direction: Option<f64>,
    pressure: Option<f64>,
    cloud: Option<f64>,
    dew_point: Option<f64>,
    uv: Option<f64>,
    code: u8,
    is_day: bool,
    precipitation: f64,
    precipitation_6h: Option<f64>,
    temp_max_6h: Option<f64>,
    temp_min_6h: Option<f64>,
    offset: i32,
}

fn met_url(location: &Location) -> String {
    format!(
        "{MET_NO}?lat={:.4}&lon={:.4}",
        location.latitude, location.longitude
    )
}

async fn fetch_met_no(location: &Location) -> Result<Weather, String> {
    let response: MetResponse = get_json(&met_url(location), "MET Norway").await?;
    if response.properties.timeseries.is_empty() {
        return Err("MET Norway no devolvió series de tiempo para esta ubicación".to_string());
    }

    let now_local = js::local_now_iso(&location.timezone);
    let mut points: Vec<MetPoint> = Vec::with_capacity(response.properties.timeseries.len());

    for item in &response.properties.timeseries {
        let parsed = match chrono::DateTime::parse_from_rfc3339(&item.time) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => continue,
        };
        let epoch_ms = parsed.timestamp_millis() as f64;
        let offset = js::tz_offset_seconds(&location.timezone, epoch_ms) as i32;
        let local = parsed + chrono::Duration::seconds(offset as i64);
        let local_iso = local.format("%Y-%m-%dT%H:%M").to_string();
        let date = local.format("%Y-%m-%d").to_string();
        let hour = local.hour();

        let details = &item.data.instant.details;
        let Some(temperature) = details.air_temperature else {
            continue;
        };

        let symbol = item
            .data
            .next_1_hours
            .as_ref()
            .and_then(|n| n.summary.symbol_code.clone())
            .or_else(|| {
                item.data
                    .next_6_hours
                    .as_ref()
                    .and_then(|n| n.summary.symbol_code.clone())
            })
            .unwrap_or_default();

        let (sunrise, sunset) = date
            .parse::<NaiveDate>()
            .map(|d| solar_times(d, location.latitude, location.longitude, offset))
            .unwrap_or((None, None));
        let is_day = match is_night_symbol(&symbol) {
            Some(night) => !night,
            None => {
                let now_hm = format!("{hour:02}:{:02}", local.minute());
                match (sunrise.as_deref(), sunset.as_deref()) {
                    (Some(rise), Some(set)) => now_hm.as_str() >= rise && now_hm.as_str() <= set,
                    _ => (6..19).contains(&hour),
                }
            }
        };

        let precipitation = item
            .data
            .next_1_hours
            .as_ref()
            .and_then(|n| n.details.precipitation_amount)
            .unwrap_or(0.0);

        points.push(MetPoint {
            local_iso,
            date,
            hour,
            temperature,
            apparent: details.apparent_air_temperature,
            humidity: details.relative_humidity,
            wind_kmh: details.wind_speed.map(|v| v * 3.6),
            wind_direction: details.wind_from_direction,
            pressure: details.air_pressure_at_sea_level,
            cloud: details.cloud_area_fraction,
            dew_point: details.dew_point_temperature,
            uv: details.ultraviolet_index_clear_sky,
            code: if symbol.is_empty() {
                3
            } else {
                symbol_to_wmo(&symbol)
            },
            is_day,
            precipitation,
            precipitation_6h: item
                .data
                .next_6_hours
                .as_ref()
                .and_then(|n| n.details.precipitation_amount),
            temp_max_6h: item
                .data
                .next_6_hours
                .as_ref()
                .and_then(|n| n.details.air_temperature_max),
            temp_min_6h: item
                .data
                .next_6_hours
                .as_ref()
                .and_then(|n| n.details.air_temperature_min),
            offset,
        });
    }

    if points.is_empty() {
        return Err("MET Norway no devolvió datos utilizables".to_string());
    }

    points.sort_by(|a, b| a.local_iso.cmp(&b.local_iso));

    let current_idx = points
        .iter()
        .position(|p| p.local_iso.as_str() >= now_local.as_str())
        .unwrap_or(0);
    let current_point = &points[current_idx];

    let current = Current {
        time: current_point.local_iso.clone(),
        date: current_point.date.clone(),
        temperature: current_point.temperature,
        apparent: current_point.apparent,
        humidity: current_point.humidity,
        precipitation: current_point.precipitation,
        precipitation_probability: None,
        weather_code: current_point.code,
        is_day: current_point.is_day,
        wind_speed: current_point.wind_kmh,
        wind_gust: None,
        wind_direction: current_point.wind_direction,
        pressure: current_point.pressure,
        cloud_cover: current_point.cloud,
        visibility_km: None,
        uv_index: current_point.uv,
        dew_point: current_point.dew_point,
    };

    let hourly: Vec<Hour> = points
        .iter()
        .filter(|p| p.local_iso.as_str() >= now_local.as_str())
        .take(48)
        .map(|p| Hour {
            time: p.local_iso.clone(),
            date: p.date.clone(),
            temperature: p.temperature,
            apparent: p.apparent,
            humidity: p.humidity,
            precipitation_probability: None,
            precipitation: p.precipitation,
            weather_code: p.code,
            is_day: p.is_day,
            wind_speed: p.wind_kmh,
            wind_gust: None,
            wind_direction: p.wind_direction,
            pressure: p.pressure,
            cloud_cover: p.cloud,
            visibility_km: None,
            uv_index: p.uv,
            dew_point: p.dew_point,
        })
        .collect();

    let mut grouped: BTreeMap<String, Vec<&MetPoint>> = BTreeMap::new();
    for point in &points {
        grouped.entry(point.date.clone()).or_default().push(point);
    }

    let today = now_local.get(0..10).unwrap_or_default().to_string();
    let mut daily: Vec<Day> = Vec::new();

    for (date, group) in grouped.iter().filter(|(date, _)| date.as_str() >= today.as_str()) {
        if group.is_empty() {
            continue;
        }

        let mut max_candidates: Vec<f64> = group.iter().map(|p| p.temperature).collect();
        let mut min_candidates: Vec<f64> = max_candidates.clone();
        for p in group {
            if let Some(v) = p.temp_max_6h {
                max_candidates.push(v);
            }
            if let Some(v) = p.temp_min_6h {
                min_candidates.push(v);
            }
        }

        let temp_max = max_candidates
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let temp_min = min_candidates
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);

        let daytime = group
            .iter()
            .find(|p| (12..=15).contains(&p.hour))
            .or_else(|| group.get(group.len() / 2))
            .unwrap_or(&group[0]);

        let apparent_max = group.iter().filter_map(|p| p.apparent).reduce(f64::max);
        let apparent_min = group.iter().filter_map(|p| p.apparent).reduce(f64::min);
        let uv_max = group.iter().filter_map(|p| p.uv).reduce(f64::max);
        let wind_max = group.iter().filter_map(|p| p.wind_kmh).reduce(f64::max);

        let mut buckets: BTreeMap<u32, f64> = BTreeMap::new();
        for p in group {
            if let Some(amount) = p.precipitation_6h {
                buckets.entry(p.hour / 6).or_insert(amount);
            }
        }
        let precipitation_sum = if buckets.is_empty() {
            Some(group.iter().map(|p| p.precipitation).sum())
        } else {
            Some(buckets.values().sum())
        };

        let offset = daytime.offset;
        let (sunrise, sunset) = date
            .parse::<NaiveDate>()
            .map(|d| solar_times(d, location.latitude, location.longitude, offset))
            .unwrap_or((None, None));

        daily.push(Day {
            date: date.clone(),
            weather_code: daytime.code,
            temp_max,
            temp_min,
            apparent_max,
            apparent_min,
            sunrise,
            sunset,
            uv_max,
            precipitation_sum,
            precipitation_probability: None,
            wind_max,
            wind_gust_max: None,
        });

        if daily.len() >= 7 {
            break;
        }
    }

    if daily.is_empty() {
        return Err("MET Norway no devolvió pronóstico diario utilizable".to_string());
    }

    Ok(Weather {
        current,
        hourly,
        daily,
        timezone: location.timezone.clone(),
        utc_offset_seconds: points[0].offset,
        fetched_at_ms: js_sys::Date::now(),
        provider: Provider::MetNo,
        latitude: location.latitude,
        longitude: location.longitude,
    })
}

pub async fn fetch_weather(location: &Location, provider: Provider) -> Result<Weather, String> {
    match provider {
        Provider::OpenMeteo => fetch_open_meteo(location).await,
        Provider::MetNo => fetch_met_no(location).await,
    }
}

// ---------------------------------------------------------------------------
// Calidad del aire (Open-Meteo CAMS, sin clave)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct AirResponse {
    current: AirCurrent,
}

#[derive(Deserialize)]
struct AirCurrent {
    #[serde(default)]
    time: String,
    european_aqi: Option<f64>,
    us_aqi: Option<f64>,
    pm10: Option<f64>,
    pm2_5: Option<f64>,
    ozone: Option<f64>,
    nitrogen_dioxide: Option<f64>,
    sulphur_dioxide: Option<f64>,
    carbon_monoxide: Option<f64>,
}

pub async fn fetch_air_quality(location: &Location) -> Result<AirQuality, String> {
    let url = format!(
        "{OPEN_METEO_AIR}?latitude={:.4}&longitude={:.4}\
         &current=european_aqi,us_aqi,pm10,pm2_5,ozone,nitrogen_dioxide,sulphur_dioxide,carbon_monoxide\
         &timezone=auto",
        location.latitude, location.longitude
    );
    let data: AirResponse = get_json(&url, "Open-Meteo Air Quality").await?;
    let aqi = AirQuality {
        european_aqi: data.current.european_aqi,
        us_aqi: data.current.us_aqi,
        pm2_5: data.current.pm2_5,
        pm10: data.current.pm10,
        ozone: data.current.ozone,
        nitrogen_dioxide: data.current.nitrogen_dioxide,
        sulphur_dioxide: data.current.sulphur_dioxide,
        carbon_monoxide: data.current.carbon_monoxide,
        time: data.current.time,
    };
    if aqi.is_empty() {
        Err("La API de calidad del aire no devolvió valores".to_string())
    } else {
        Ok(aqi)
    }
}

// ---------------------------------------------------------------------------
// Geocodificación y búsqueda
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct GeoResponse {
    #[serde(default)]
    results: Vec<GeoItem>,
}

#[derive(Deserialize)]
struct GeoItem {
    name: String,
    latitude: f64,
    longitude: f64,
    #[serde(default)]
    country: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
    #[serde(default)]
    admin1: Option<String>,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    population: Option<u64>,
}

pub async fn search_locations(query: &str) -> Result<Vec<Location>, String> {
    let trimmed = query.trim();
    if trimmed.chars().count() < 2 {
        return Ok(Vec::new());
    }

    let response: GeoResponse = client()?
        .get(OPEN_METEO_GEOCODING)
        .query(&[
            ("name", trimmed),
            ("count", "8"),
            ("language", "es"),
            ("format", "json"),
        ])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("No se pudo buscar ciudades: {e}"))?
        .error_for_status()
        .map_err(|e| format!("El buscador respondió con un error: {e}"))?
        .json::<GeoResponse>()
        .await
        .map_err(|e| format!("Respuesta inválida del buscador: {e}"))?;

    Ok(response
        .results
        .into_iter()
        .map(|item| Location {
            name: item.name,
            admin: item.admin1,
            country: item.country,
            country_code: item.country_code,
            latitude: item.latitude,
            longitude: item.longitude,
            timezone: item.timezone.unwrap_or_else(|| "UTC".to_string()),
            accuracy: None,
            population: item.population,
            is_current: false,
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Geocodificación inversa (Nominatim / OpenStreetMap)
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct NominatimResponse {
    #[serde(default)]
    display_name: Option<String>,
    #[serde(default)]
    address: Option<NominatimAddress>,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Deserialize)]
struct NominatimAddress {
    #[serde(default)]
    city: Option<String>,
    #[serde(default)]
    town: Option<String>,
    #[serde(default)]
    village: Option<String>,
    #[serde(default)]
    municipality: Option<String>,
    #[serde(default)]
    county: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    country: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
}

pub async fn reverse_geocode(latitude: f64, longitude: f64) -> Result<Location, String> {
    let lat = format!("{latitude:.6}");
    let lon = format!("{longitude:.6}");
    let response: NominatimResponse = client()?
        .get(NOMINATIM)
        .query(&[
            ("format", "jsonv2"),
            ("lat", lat.as_str()),
            ("lon", lon.as_str()),
            ("zoom", "14"),
            ("addressdetails", "1"),
            ("accept-language", "es"),
        ])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|_| "No se pudo obtener el nombre del lugar".to_string())?
        .error_for_status()
        .map_err(|_| "El servicio de nombres respondió con un error".to_string())?
        .json::<NominatimResponse>()
        .await
        .map_err(|_| "Respuesta inválida del servicio de nombres".to_string())?;

    if let Some(error) = response.error {
        return Err(format!("No se pudo identificar el lugar: {error}"));
    }

    let address = response.address.unwrap_or(NominatimAddress {
        city: None,
        town: None,
        village: None,
        municipality: None,
        county: None,
        state: None,
        country: None,
        country_code: None,
    });

    let name = address
        .city
        .clone()
        .or(address.town.clone())
        .or(address.village.clone())
        .or(address.municipality.clone())
        .or(address.county.clone())
        .or_else(|| {
            response
                .display_name
                .as_deref()
                .and_then(|d| d.split(',').next().map(str::trim).map(String::from))
        })
        .ok_or_else(|| "El lugar no tiene un nombre reconocible".to_string())?;

    Ok(Location {
        name,
        admin: address.state,
        country: address.country,
        country_code: address.country_code,
        latitude,
        longitude,
        timezone: js::browser_timezone(),
        accuracy: None,
        population: None,
        is_current: true,
    })
}
