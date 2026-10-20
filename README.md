# FedeWeather

Clima real, en tiempo real. SPA construida en **Rust + WebAssembly (Leptos 0.8)** con mapas Leaflet, búsqueda de ciudades, geolocalización y cambio de proveedor meteorológico. **No contiene datos simulados ni de relleno**: si una fuente falla, la interfaz muestra un error explícito o un estado de carga, nunca un valor inventado.

![stack](https://img.shields.io/badge/Rust-1.85+-000?logo=rust) ![wasm](https://img.shields.io/badge/WebAssembly-Leptos%200.8-654ff0) ![tailwind](https://img.shields.io/badge/Tailwind-3.4-38bdf8?logo=tailwindcss)

## Funcionalidad

- **Datos en vivo** de dos proveedores sin API key:
  - [Open-Meteo](https://open-meteo.com/) — modelo global `best_match` (horario + diario).
  - [MET Norway](https://api.met.no/) — `locationforecast/2.0/complete` (modelo nórdico).
- **Condiciones actuales completas**: temperatura, sensación térmica, viento y ráfagas, humedad, punto de rocío, presión, visibilidad, nubosidad, UV y precipitación.
- **Pronóstico reactivo** de 24 horas y 7 días con gráfico SVG interactivo (temperatura / lluvia / viento).
- **Calidad del aire** real vía CAMS / Open-Meteo Air Quality (EAQI, PM2.5, PM10, O₃).
- **Sol y luna** calculados localmente (orto, ocaso, duración del día, fase lunar e iluminación).
- **Mapa Leaflet** con capas *Oscuro* (Esri Dark Gray + etiquetas) y *Calles* (OpenStreetMap), marcador con popup y recentrado.
- **Búsqueda de ciudades** con geocodificación Open-Meteo, navegación por teclado y resultados en vivo.
- **Geolocalización** de alta precisión con geocodificación inversa (Nominatim/OSM) y fallback a la zona horaria del navegador.
- **Favoritos y recientes**, proveedor, unidades y capa del mapa persistidos en `localStorage`.
- **Auto-refresh** cada 10 minutos y reloj interno para “actualizado hace…”.
- Diseño mobile-first, navegación inferior en móvil, iconografía SVG animada y soporte de `prefers-reduced-motion`.

## Stack

| Capa | Tecnología |
| --- | --- |
| UI | Leptos 0.8 (CSR), fine-grained reactivity |
| Estilos | Tailwind CSS 3.4 (paleta navy/ice, tipografías Inter + Roboto Flex) |
| HTTP | `reqwest` 0.13 sobre `fetch` del navegador (WASM automático por target) |
| Mapas | Leaflet 1.9 + OpenStreetMap + Esri Dark Gray |
| Build | Trunk 0.22 + wasm-bindgen + wasm-opt |
| Estado | Signals de Leptos + `localStorage` |

## Requisitos

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
# Tailwind y wasm-opt los descarga Trunk automáticamente.
```

Rust ≥ 1.85. No se necesita Node.js ni claves de API.

## Desarrollo

```bash
trunk serve --port 8080
```

Abrí <http://127.0.0.1:8080>. La ubicación inicial es **Córdoba, Argentina** (datos reales); luego se recuerda la última ubicación elegida.

## Build de producción

```bash
trunk build --release
# o
./build.sh            # genera dist/
```

`dist/` es 100 % estático: HTML, CSS, JS, WASM y assets. Podés servirlo con cualquier CDN.

Para publicar en un subdirectorio (por ejemplo GitHub Pages en `/FedeWeather/`):

```bash
trunk build --release --public-url /FedeWeather/
# o bien:
TRUNK_BUILD_PUBLIC_URL=/FedeWeather/ ./build.sh
```

## Deploy

### Vercel

El repo incluye `vercel.json`. Importá el proyecto en Vercel y usá la configuración detectada:

- **Build command:** `bash scripts/vercel-build.sh`
- **Output directory:** `dist`

El script instala Rust, el target `wasm32-unknown-unknown` y Trunk si no están disponibles. No requiere variables de entorno.

### GitHub Pages

El workflow `.github/workflows/deploy.yml` compila y publica en cada push a `main`. Configurá **Settings → Pages → Source: GitHub Actions**. El `public_url` se ajusta automáticamente al nombre del repositorio.

## Estructura

```
.
├── Cargo.toml
├── Trunk.toml
├── index.html                 # shell, fuentes, Leaflet CDN, assets Trunk
├── tailwind.config.js
├── style/tailwind.css         # tema, componentes, overrides de Leaflet
├── public/                    # favicon + manifest PWA
└── src/
    ├── main.rs                # mount Leptos
    ├── app.rs                 # raíz, fetch, refresh, persistencia
    ├── state.rs               # AppCtx (signals), toasts, favoritos, GPS
    ├── api.rs                 # Open-Meteo, MET Norway, geocoding, AQI
    ├── models.rs              # modelos unificados + mapeo WMO/símbolos
    ├── format.rs              # fechas, sol/luna, categorías, paths SVG
    ├── geolocate.rs           # permisos y precisión GPS
    ├── js.rs                  # interop Leaflet y zonas horarias
    ├── storage.rs             # localStorage versionado
    └── components/            # hero, hourly, daily, chart, metrics, map…
```

## Decisiones técnicas

- **`reqwest` en WASM:** en 0.13 no existe la feature `wasm`; el backend `fetch` se selecciona según el target. Por eso el crate se declara con `default-features = false, features = ["json", "query"]`.
- **Modelo unificado:** ambos proveedores se normalizan a `Weather`/`Hour`/`Day`, de modo que la UI no conoce diferencias de formato. MET Norway entrega símbolos `symbol_code` que se mapean a códigos WMO.
- **Sin backend ni claves:** todas las llamadas van directo del navegador a APIs públicas con CORS habilitado.
- **Estados honestos:** skeletons durante la carga, banner *stale* si falla un refresco y mensajes de error accionables. No se muestran valores cacheados como si fueran actuales.
- **Accesibilidad:** foco visible, `aria-label`/`aria-pressed`, contraste alto y respeto por movimiento reducido.

## Limitaciones conocidas

- **MET Norway** pide un `User-Agent` identificable en sus términos. El navegador prohíbe sobrescribirlo, por lo que se envía el UA del navegador; para uso intensivo se recomienda un proxy propio que agregue el header.
- **Geocodificación inversa** usa Nominatim (política de uso: 1 req/s). Solo se llama tras una acción explícita del usuario.
- **Calidad del aire** siempre proviene de Open-Meteo CAMS, aunque el proveedor meteorológico sea MET Norway (se indica en la tarjeta).
- La geolocalización requiere HTTPS (o `localhost`) y permiso del usuario.
- Las capas Esri/OSM dependen de sus servidores públicos y de sus políticas de atribución.

## Atribuciones

- Open-Meteo (licencia CC BY 4.0) y Open-Meteo Air Quality (CAMS).
- MET Norway / Norwegian Meteorological Institute.
- OpenStreetMap contributors (ODbL) y Esri Dark Gray Canvas.
- Leaflet (BSD-2-Clause).

Hecho con foco en datos reales, rendimiento y detalle visual.
