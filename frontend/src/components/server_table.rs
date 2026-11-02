use crate::models::Server;
use leptos::prelude::*;

#[component]
pub fn ServerTable(
    servers: ReadSignal<Vec<Server>>,
    set_servers: WriteSignal<Vec<Server>>,
) -> impl IntoView {
    let on_delete = move |id: uuid::Uuid| {
        let set_servers = set_servers;
        wasm_bindgen_futures::spawn_local(async move {
            if crate::api::delete_server(id).await.is_ok() {
                set_servers.update(|v| v.retain(|s| s.id != id));
            }
        });
    };
    let on_check = move |id: uuid::Uuid| {
        let set_servers = set_servers;
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(updated) = crate::api::check_server(id).await {
                set_servers.update(|v| {
                    if let Some(s) = v.iter_mut().find(|s| s.id == id) {
                        *s = updated;
                    }
                });
            }
        });
    };

    view! {
        <table>
            <thead>
                <tr>
                    <th>Name</th><th>IP</th><th>Port</th><th>Status</th><th>Latency</th><th>Actions</th>
                </tr>
            </thead>
            <tbody>
                <For
                    each=move || servers.get()
                    key=|s| s.id
                    children=move |s| {
                        let id = s.id;
                        view! {
                            <tr>
                                <td>{s.name.clone()}</td>
                                <td>{s.ip.clone()}</td>
                                <td>{s.port}</td>
                                <td><crate::components::status_badge::StatusBadge status=s.status /></td>
                                <td>{s.latency_ms.map(|x| format!("{x} ms")).unwrap_or_default()}</td>
                                <td class="row">
                                    <button on:click=move |_| on_check(id)>Check</button>
                                    <button class="danger" on:click=move |_| on_delete(id)>Delete</button>
                                </td>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    }
}
