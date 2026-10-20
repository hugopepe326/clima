use leptos::prelude::*;

#[component]
pub fn Brand(
    #[prop(optional)] compact: bool,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let class = if class.is_empty() {
        "flex items-center gap-2.5".to_string()
    } else {
        class
    };

    view! {
        <div class=class>
            <div class="relative grid h-10 w-10 shrink-0 place-items-center rounded-[14px] border border-white/10 bg-gradient-to-br from-sky-400/20 via-navy-700/40 to-navy-900 shadow-glow">
                <svg viewBox="0 0 48 48" class="h-7 w-7" aria-hidden="true">
                    <defs>
                        <linearGradient id="fwLogoGrad" x1="8" y1="6" x2="40" y2="40" gradientUnits="userSpaceOnUse">
                            <stop stop-color="#a5f3fc"></stop>
                            <stop offset="0.55" stop-color="#60a5fa"></stop>
                            <stop offset="1" stop-color="#2563eb"></stop>
                        </linearGradient>
                    </defs>
                    <circle cx="31" cy="16" r="7" fill="url(#fwLogoGrad)"></circle>
                    <path
                        d="M10 31.5a6.5 6.5 0 0 1 6.5-6.5c.7 0 1.4.1 2 .3A9.5 9.5 0 0 1 37 28.5a5.5 5.5 0 0 1-1 10.9H16.5a6.5 6.5 0 0 1-6.5-6.5Z"
                        fill="#e0f2fe"
                        opacity="0.96"
                    ></path>
                    <path
                        d="M8 23.5A4.5 4.5 0 0 1 12.5 19"
                        stroke="#93c5fd"
                        stroke-width="1.8"
                        stroke-linecap="round"
                        opacity="0.65"
                    ></path>
                </svg>
            </div>
            <div class="min-w-0">
                <div class="font-display text-[18px] leading-none tracking-[-0.025em]">
                    <span class="font-light text-white">Fede</span>
                    <span class="font-medium text-ice-200">Weather</span>
                </div>
                <Show when=move || !compact>
                    <p class="mt-1 text-[9px] font-medium uppercase tracking-[0.24em] text-slate-500">
                        "Clima real · en vivo"
                    </p>
                </Show>
            </div>
        </div>
    }
}
