use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = r##"
const __fwTzCache = new Map();

export function tz_offset_seconds(timeZone, epochMs) {
  try {
    const key = timeZone + "@" + Math.floor(epochMs / 3600000);
    if (__fwTzCache.has(key)) return __fwTzCache.get(key);
    const dtf = new Intl.DateTimeFormat("en-US", {
      timeZone,
      hour12: false,
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit"
    });
    const parts = {};
    for (const p of dtf.formatToParts(new Date(epochMs))) parts[p.type] = p.value;
    let hour = parseInt(parts.hour, 10);
    if (hour === 24) hour = 0;
    const asUTC = Date.UTC(
      parseInt(parts.year, 10),
      parseInt(parts.month, 10) - 1,
      parseInt(parts.day, 10),
      hour,
      parseInt(parts.minute, 10),
      parseInt(parts.second, 10)
    );
    const offset = Math.round((asUTC - epochMs) / 1000);
    __fwTzCache.set(key, offset);
    return offset;
  } catch (_) {
    return 0;
  }
}

export function local_now_iso(timeZone) {
  try {
    const dtf = new Intl.DateTimeFormat("en-CA", {
      timeZone,
      hour12: false,
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit"
    });
    const parts = {};
    for (const p of dtf.formatToParts(new Date())) parts[p.type] = p.value;
    let hour = parts.hour === "24" ? "00" : parts.hour;
    return `${parts.year}-${parts.month}-${parts.day}T${hour}:${parts.minute}`;
  } catch (_) {
    const d = new Date();
    const pad = (n) => String(n).padStart(2, "0");
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
  }
}

export function browser_timezone() {
  try {
    return Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
  } catch (_) {
    return "UTC";
  }
}

const __FW_DARK_LABELS = "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Dark_Gray_Reference/MapServer/tile/{z}/{y}/{x}";

const __FW_TILES = {
  dark: {
    url: "https://server.arcgisonline.com/ArcGIS/rest/services/Canvas/World_Dark_Gray_Base/MapServer/tile/{z}/{y}/{x}",
    attribution: 'Tiles &copy; <a href="https://www.esri.com/" target="_blank" rel="noopener">Esri</a> &mdash; Esri, DeLorme, NAVTEQ &middot; <a href="https://www.openstreetmap.org/copyright" target="_blank" rel="noopener">OpenStreetMap</a>',
    subdomains: "",
    maxZoom: 16
  },
  osm: {
    url: "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright" target="_blank" rel="noopener">OpenStreetMap</a>',
    subdomains: "abc",
    maxZoom: 19
  }
};

function ensureLayers(map) {
  if (map.__fwDark) return;
  map.__fwDark = L.tileLayer(__FW_TILES.dark.url, __FW_TILES.dark).addTo(map);
  map.__fwDarkLabels = L.tileLayer(__FW_DARK_LABELS, { maxZoom: 16, opacity: 0.85 }).addTo(map);
  map.__fwOsm = L.tileLayer(__FW_TILES.osm.url, __FW_TILES.osm);
  map.__fwLayer = "dark";
}

function markerIcon() {
  return L.divIcon({
    className: "fw-marker",
    html: '<div class="fw-marker-ping"><div class="fw-marker-dot"></div></div>',
    iconSize: [22, 22],
    iconAnchor: [11, 11],
    popupAnchor: [0, -12]
  });
}

function buildPopup(name, temp, desc, units) {
  const root = document.createElement("div");
  root.className = "fw-popup";
  const title = document.createElement("div");
  title.style.fontSize = "12px";
  title.style.letterSpacing = "0.04em";
  title.style.color = "#93c5fd";
  title.style.textTransform = "uppercase";
  title.textContent = name;
  const row = document.createElement("div");
  row.style.display = "flex";
  row.style.alignItems = "baseline";
  row.style.gap = "8px";
  const t = document.createElement("span");
  t.className = "fw-temp";
  t.textContent = temp;
  const u = document.createElement("span");
  u.style.color = "#94a3b8";
  u.style.fontSize = "12px";
  u.textContent = units;
  row.appendChild(t);
  row.appendChild(u);
  const d = document.createElement("div");
  d.style.color = "#cbd5e1";
  d.style.fontSize = "12px";
  d.textContent = desc;
  root.appendChild(title);
  root.appendChild(row);
  root.appendChild(d);
  return root;
}

export function sync_map(lat, lon, name, temp, desc, units, zoom) {
  if (typeof L === "undefined") return false;
  const host = document.getElementById("weather-map");
  if (!host) return false;

  let map = window.__fwMap;
  if (!map) {
    map = L.map(host, {
      zoomControl: true,
      attributionControl: true,
      scrollWheelZoom: true,
      worldCopyJump: true,
      minZoom: 2,
      maxZoom: 16
    }).setView([lat, lon], zoom || 10);
    ensureLayers(map);
    const marker = L.marker([lat, lon], { icon: markerIcon() }).addTo(map);
    map.__fwMarker = marker;
    map.__fwLastPlace = "";
    window.__fwMap = map;
    setTimeout(() => map.invalidateSize(), 250);
    setTimeout(() => map.invalidateSize(), 900);
  }

  ensureLayers(map);
  const marker = map.__fwMarker;
  marker.setLatLng([lat, lon]);
  const popup = buildPopup(name, temp, desc, units);
  const existing = marker.getPopup();
  if (existing) {
    existing.setContent(popup);
  } else {
    marker.bindPopup(popup, { closeButton: true, autoPan: true, maxWidth: 240 });
  }
  const placeKey = name + "@" + lat.toFixed(3) + "," + lon.toFixed(3);
  if (map.__fwLastPlace !== placeKey) {
    map.__fwLastPlace = placeKey;
    map.setView([lat, lon], zoom || Math.max(map.getZoom(), 9), { animate: true });
    marker.openPopup();
    setTimeout(() => map.invalidateSize(), 250);
  } else {
    map.panTo([lat, lon], { animate: true });
  }
  return true;
}

export function set_map_layer(kind) {
  const map = window.__fwMap;
  if (!map || !map.__fwDark) return;
  const target = kind === "osm" ? map.__fwOsm : map.__fwDark;
  const other = kind === "osm" ? map.__fwDark : map.__fwOsm;
  const labels = map.__fwDarkLabels;
  if (map.hasLayer(other)) map.removeLayer(other);
  if (!map.hasLayer(target)) target.addTo(map);
  if (kind === "osm") {
    if (labels && map.hasLayer(labels)) map.removeLayer(labels);
  } else if (labels && !map.hasLayer(labels)) {
    labels.addTo(map);
  }
  map.__fwLayer = kind === "osm" ? "osm" : "dark";
}

export function recenter_map(lat, lon) {
  const map = window.__fwMap;
  if (!map) return;
  map.setView([lat, lon], 11, { animate: true });
  if (map.__fwMarker) map.__fwMarker.openPopup();
  setTimeout(() => map.invalidateSize(), 200);
}

export function invalidate_map() {
  if (window.__fwMap) window.__fwMap.invalidateSize();
}
"##)]
extern "C" {
    pub fn tz_offset_seconds(timezone: &str, epoch_ms: f64) -> f64;
    pub fn local_now_iso(timezone: &str) -> String;
    pub fn browser_timezone() -> String;
    pub fn sync_map(
        lat: f64,
        lon: f64,
        name: &str,
        temp: &str,
        desc: &str,
        units: &str,
        zoom: u32,
    ) -> bool;
    pub fn set_map_layer(kind: &str);
    pub fn recenter_map(lat: f64, lon: f64);
    pub fn invalidate_map();
}
