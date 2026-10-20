use leptos::prelude::*;

use crate::components::icons::{Icon, UiIcon};
use crate::state::{scroll_to_id, AppCtx};

struct NavItem {
    id: &'static str,
    label: &'static str,
    icon: UiIcon,
}

const NAV_ITEMS: [NavItem; 5] = [
    NavItem { id: "inicio", label: "Inicio", icon: UiIcon::Home },
    NavItem { id: "horas", label: "Horas", icon: UiIcon::Clock },
    NavItem { id: "dias", label: "7 días", icon: UiIcon::Calendar },
    NavItem { id: "mapa", label: "Mapa", icon: UiIcon::Map },
    NavItem { id: "ajustes", label: "Ajustes", icon: UiIcon::Settings },
];

#[component]
pub fn MobileNav() -> impl IntoView {
    let active = RwSignal::new("inicio");
    let ctx = expect_context::<AppCtx>();

    view! {
        <nav
            class="safe-bottom fixed inset-x-0 bottom-0 z-50 border-t border-white/[0.07] bg-navy-950/95 backdrop-blur-2xl lg:hidden"
            aria-label="Navegación principal"
        >
            <div class="mx-auto grid max-w-lg grid-cols-5 px-2 pt-1.5">
                {NAV_ITEMS.into_iter().map(|item| {
                    let is_active = move || active.get() == item.id;
                    view! {
                        <button
                            type="button"
                            class=move || format!(
                                "group relative flex flex-col items-center gap-1 rounded-xl px-1 pb-1 pt-1.5 transition-colors {}",
                                if is_active() { "text-ice-200" } else { "text-slate-500 hover:text-slate-300" }
                            )
                            aria-current=move || if is_active() { "page" } else { "false" }
                            on:click=move |_| {
                                active.set(item.id);
                                if item.id == "inicio" {
                                    crate::state::scroll_to_top();
                                } else {
                                    scroll_to_id(item.id);
                                }
                                ctx.mobile_search.set(false);
                            }
                        >
                            <span class=move || format!(
                                "absolute -top-1.5 h-0.5 rounded-full bg-ice-300 transition-all duration-300 {}",
                                if is_active() { "w-5 opacity-100" } else { "w-0 opacity-0" }
                            )></span>
                            <span class=move || format!(
                                "grid h-7 w-8 place-items-center rounded-lg transition-colors {}",
                                if is_active() { "bg-ice-400/10" } else { "group-hover:bg-white/[0.04]" }
                            )>
                                <Icon kind=item.icon class="h-[17px] w-[17px]" />
                            </span>
                            <span class="text-[8.5px] font-medium tracking-wide">{item.label}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </nav>
    }
}
