use crate::components::{add_form::AddForm, server_table::ServerTable};
use crate::models::ServerEvent;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

#[component]
pub fn Dashboard() -> impl IntoView {
    let (servers, set_servers) = signal(Vec::<crate::models::Server>::new());

    Effect::new(move |_| {
        let set_servers = set_servers;
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(list) = crate::api::fetch_servers().await {
                set_servers.set(list);
            }
        });

        let ws = web_sys::WebSocket::new("ws://127.0.0.1:8080/ws");
        if let Ok(ws) = ws {
            let set_servers = set_servers;
            let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Some(text) = e.data().as_string() {
                    if let Ok(event) = serde_json::from_str::<ServerEvent>(&text) {
                        match event {
                            ServerEvent::Snapshot { servers } => set_servers.set(servers),
                            ServerEvent::ServerAdded { server } => {
                                set_servers.update(|v| {
                                    if !v.iter().any(|s| s.id == server.id) {
                                        v.push(server);
                                    }
                                });
                            }
                            ServerEvent::ServerUpdated { server } => {
                                set_servers.update(|v| {
                                    if let Some(s) = v.iter_mut().find(|s| s.id == server.id) {
                                        *s = server;
                                    }
                                });
                            }
                            ServerEvent::ServerRemoved { id } => {
                                set_servers.update(|v| v.retain(|s| s.id != id));
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();
        }
    });

    view! {
        <div class="wrap">
            <h1>"Rust Server Monitor"</h1>
            <AddForm set_servers=set_servers />
            <div class="card">
                <ServerTable servers=servers set_servers=set_servers />
            </div>
        </div>
    }
}
