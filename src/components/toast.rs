use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon};
use crate::state::{AppCtx, ToastKind};

#[component]
pub fn ToastHost() -> impl IntoView {
    let ctx = expect_context::<AppCtx>();

    view! {
        {move || {
            ctx.toast.get().map(|toast| {
                let (border, bg, text, icon) = match toast.kind {
                    ToastKind::Success => (
                        "border-emerald-400/20",
                        "bg-[#0b1f2a]/95",
                        "text-emerald-300",
                        UiIcon::Check,
                    ),
                    ToastKind::Error => (
                        "border-rose-400/20",
                        "bg-[#251221]/95",
                        "text-rose-300",
                        UiIcon::Alert,
                    ),
                    ToastKind::Info => (
                        "border-ice-400/20",
                        "bg-[#0a1a2c]/95",
                        "text-ice-300",
                        UiIcon::Sparkle,
                    ),
                };
                view! {
                    <div class="pointer-events-none fixed inset-x-0 bottom-24 z-[90] flex justify-center px-4 lg:bottom-6 lg:justify-end lg:px-6">
                        <div class=format!(
                            "pointer-events-auto flex w-full max-w-sm animate-fade-up items-start gap-3 rounded-2xl border px-3.5 py-3 shadow-glass backdrop-blur-xl {border} {bg}"
                        )>
                            <span class=format!("mt-0.5 shrink-0 {text}")>
                                <Icon kind=icon class="h-4 w-4" />
                            </span>
                            <p class="flex-1 text-[11px] leading-relaxed text-slate-200">{toast.message}</p>
                            <button
                                type="button"
                                class="grid h-6 w-6 shrink-0 place-items-center rounded-md text-slate-500 hover:bg-white/5 hover:text-white"
                                aria-label="Cerrar aviso"
                                on:click=move |_| ctx.toast.set(None)
                            >
                                <Icon kind=UiIcon::Close class="h-3 w-3" />
                            </button>
                        </div>
                    </div>
                }
            })
        }}
    }
}
