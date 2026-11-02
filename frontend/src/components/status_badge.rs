use crate::models::ServerStatus;
use leptos::prelude::*;

#[component]
pub fn StatusBadge(status: ServerStatus) -> impl IntoView {
    let (class, label) = match status {
        ServerStatus::Online => ("badge online", "online"),
        ServerStatus::Offline => ("badge offline", "offline"),
        ServerStatus::Degraded => ("badge degraded", "degraded"),
        ServerStatus::Unknown => ("badge unknown", "unknown"),
    };
    view! { <span class=class>{label}</span> }
}
