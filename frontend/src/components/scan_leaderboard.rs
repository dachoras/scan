//! Leaderboard display component for Scan.

use crate::api::ApiService;
use crate::components::scan_logic::Sector;
use crate::i18n::LocaleContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub sector: Sector,
    pub reload_trigger: usize,
}

#[function_component(ScanLeaderboard)]
pub fn scan_leaderboard(props: &Props) -> Html {
    let sector = props.sector;
    let reload_trigger = props.reload_trigger;
    let entries = use_state(Vec::new);
    let locale = use_context::<LocaleContext>().expect("locale context");

    {
        let entries = entries.clone();
        use_effect_with((sector, reload_trigger), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let category = sector.name();
                if let Ok(list) = ApiService::get_leaderboard(category).await {
                    entries.set(list);
                } else {
                    entries.set(Vec::new());
                }
            });
        });
    }

    html! {
        <div class="leaderboard-panel glassmorphic">
            <h3>{ format!("{} {}", sector.name(), locale.t("leaderboard")) }</h3>
            <div class="leaderboard-list">
                { if entries.is_empty() {
                    html! { <div class="leaderboard-empty">{ locale.t("no_scores") }</div> }
                } else {
                    html! {
                        <ul class="leaderboard-ol">
                            { for entries.iter().take(5).enumerate().map(|(idx, entry)| {
                                html! {
                                    <li key={idx} class="leaderboard-item">
                                        <span class="leader-name">{ format!("{}. {}", idx + 1, entry.name) }</span>
                                        <span class="leader-score">{ format!("{:.1}s", entry.score as f64 / 10.0) }</span>
                                    </li>
                                }
                            }) }
                        </ul>
                    }
                } }
            </div>
        </div>
    }
}
