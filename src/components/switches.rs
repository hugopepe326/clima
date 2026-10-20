use leptos::prelude::*;

use crate::models::{Provider, Units};
use crate::state::AppCtx;

#[component]
pub fn ProviderSwitch(#[prop(optional)] compact: bool) -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <div class=move || {
            if compact {
                "inline-flex rounded-xl border border-white/[0.07] bg-white/[0.03] p-0.5"
            } else {
                "grid grid-cols-2 gap-1 rounded-2xl border border-white/[0.07] bg-black/20 p-1"
            }
        }>
            {Provider::ALL.into_iter().map(|provider| {
                let active = move || ctx.provider.get() == provider;
                view! {
                    <button
                        type="button"
                        class=move || format!(
                            "focus-ring relative rounded-[10px] font-medium transition-all duration-300 {} {}",
                            if compact { "px-2.5 py-1.5 text-[10px]" } else { "px-3 py-2 text-[11px]" },
                            if active() {
                                "bg-ice-400/15 text-ice-100 shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]"
                            } else {
                                "text-slate-500 hover:bg-white/[0.04] hover:text-slate-300"
                            }
                        )
                        aria-pressed=move || active().to_string()
                        title=provider.description()
                        on:click=move |_| {
                            if ctx.provider.get_untracked() != provider {
                                ctx.provider.set(provider);
                            }
                        }
                    >
                        <span class="flex items-center justify-center gap-1.5">
                            <span class=move || format!(
                                "h-1.5 w-1.5 rounded-full transition-colors {}",
                                if active() { "bg-ice-300" } else { "bg-slate-700" }
                            )></span>
                            {provider.short()}
                        </span>
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn UnitsSwitch() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        <div class="inline-flex rounded-xl border border-white/[0.07] bg-white/[0.03] p-0.5">
            {[Units::Celsius, Units::Fahrenheit].into_iter().map(|unit| {
                let active = move || ctx.units.get() == unit;
                view! {
                    <button
                        type="button"
                        class=move || format!(
                            "focus-ring rounded-[10px] px-2.5 py-1.5 text-[10px] font-medium transition-all duration-300 {}",
                            if active() {
                                "bg-white/[0.09] text-white shadow-[inset_0_1px_0_rgba(255,255,255,0.08)]"
                            } else {
                                "text-slate-500 hover:text-slate-300"
                            }
                        )
                        aria-pressed=move || active().to_string()
                        aria-label=format!("Mostrar temperatura en {}", unit.label())
                        on:click=move |_| ctx.units.set(unit)
                    >
                        {unit.label()}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}

#[component]
pub fn LiveBadge() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();
    let loading = ctx.loading;

    view! {
        <span class="inline-flex items-center gap-1.5 rounded-full border border-emerald-400/15 bg-emerald-400/[0.07] px-2.5 py-1 text-[9px] font-medium uppercase tracking-[0.16em] text-emerald-300">
            <span class=move || format!(
                "h-1.5 w-1.5 rounded-full bg-emerald-300 {}",
                if loading.get() { "animate-pulse opacity-60" } else { "shadow-[0_0_8px_rgba(110,231,183,0.8)]" }
            )></span>
            {move || if loading.get() { "Sincronizando" } else { "En vivo" }}
        </span>
    }
}
