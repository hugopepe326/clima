use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api;
use crate::components::daily::DailyForecast;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::hero::Hero;
use crate::components::hourly::Hourly;
use crate::components::map::MapSection;
use crate::components::metrics::Metrics;
use crate::components::mobile_nav::MobileNav;
use crate::components::settings::Settings;
use crate::components::sidebar::Sidebar;
use crate::components::skeleton::{LoadingOrError, StaleBanner};
use crate::components::toast::ToastHost;
use crate::components::chart::TrendChart;
use crate::state::AppCtx;
use crate::storage;

#[component]
pub fn App() -> impl IntoView {
    let ctx = AppCtx::new();
    provide_context(ctx);

    let sequence = RwSignal::new(0u32);

    // Carga real de datos: se dispara al montar y cada vez que cambia
    // la ubicación, el proveedor o se solicita una actualización manual.
    Effect::new(move |_| {
        let location = ctx.location.get();
        let provider = ctx.provider.get();
        let _ = ctx.refresh.get();

        let my_sequence = sequence.get_untracked().wrapping_add(1);
        sequence.set(my_sequence);
        ctx.loading.set(true);
        ctx.error.set(None);

        spawn_local(async move {
            let (weather_result, air_result) = futures_util::future::join(
                api::fetch_weather(&location, provider),
                api::fetch_air_quality(&location),
            )
            .await;

            if sequence.get_untracked() != my_sequence {
                return;
            }

            match weather_result {
                Ok(weather) => {
                    ctx.weather.set(Some(weather));
                    ctx.error.set(None);
                }
                Err(message) => {
                    ctx.error.set(Some(message));
                }
            }

            ctx.air.set(air_result.ok());
            ctx.loading.set(false);
        });
    });

    // Persistencia ligera, sin backend.
    Effect::new(move |_| storage::save_location(&ctx.location.get()));
    Effect::new(move |_| storage::save_provider(ctx.provider.get()));
    Effect::new(move |_| storage::save_units(ctx.units.get()));

    Effect::new(move |_| {
        let location = ctx.location.get();
        ctx.push_recent(&location);

        if let Some(document) = web_sys::window().and_then(|window| window.document()) {
            document.set_title(&format!("{} · FedeWeather", location.name));
        }
    });

    // Reloj interno para "actualizado hace…" y refresco automático cada 10 min.
    spawn_local(async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(30_000).await;
            ctx.now_ms.set(js_sys::Date::now());
        }
    });

    spawn_local(async move {
        loop {
            gloo_timers::future::TimeoutFuture::new(600_000).await;
            if !ctx.loading.get_untracked() {
                ctx.refresh.update(|value| *value = value.wrapping_add(1));
            }
        }
    });

    view! {
        <div class="min-h-dvh">
            <Sidebar />

            <div class="flex min-h-dvh flex-col lg:pl-[292px] xl:pl-[312px]">
                <Header />

                <main class="mx-auto w-full max-w-[1560px] flex-1 px-3 pb-28 pt-3 sm:px-5 sm:pt-4 lg:px-8 lg:pb-10">
                    <div class="space-y-3 sm:space-y-4">
                        <StaleBanner />
                        <Show
                            when=move || ctx.weather.get().is_some()
                            fallback=move || view! { <LoadingOrError /> }
                        >
                            <Content />
                        </Show>
                    </div>
                </main>

                <Footer />
            </div>

            <MobileNav />
            <ToastHost />
        </div>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="animate-fade-up space-y-4 sm:space-y-5">
            <Hero />
            <Hourly />

            <div class="grid items-stretch gap-3 lg:grid-cols-5">
                <DailyForecast />
                <TrendChart />
            </div>

            <Metrics />
            <MapSection />
            <Settings />
        </div>
    }
}
