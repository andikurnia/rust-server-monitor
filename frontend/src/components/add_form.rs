use crate::models::CreateServerRequest;
use leptos::prelude::*;

#[component]
pub fn AddForm(set_servers: WriteSignal<Vec<crate::models::Server>>) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (ip, set_ip) = signal(String::new());
    let (port, set_port) = signal("22".to_string());
    let (tags, set_tags) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let name = name.get();
        let ip = ip.get();
        let port: u16 = port.get().parse().unwrap_or(22);
        let tags: Vec<String> = tags
            .get()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let set_servers = set_servers;
        let set_error = set_error;
        wasm_bindgen_futures::spawn_local(async move {
            match crate::api::create_server(CreateServerRequest { name, ip, port, tags }).await {
                Ok(server) => {
                    set_servers.update(|v| v.push(server));
                    set_error.set(None);
                }
                Err(e) => set_error.set(Some(e)),
            }
        });
    };

    view! {
        <form on:submit=on_submit class="card row">
            <input placeholder="Name" prop:value=name on:input=move |ev| set_name.set(event_target_value(&ev)) />
            <input placeholder="External IP" prop:value=ip on:input=move |ev| set_ip.set(event_target_value(&ev)) />
            <input placeholder="Port" prop:value=port on:input=move |ev| set_port.set(event_target_value(&ev)) />
            <input placeholder="Tags (comma)" prop:value=tags on:input=move |ev| set_tags.set(event_target_value(&ev)) />
            <button type="submit">Add server</button>
            {move || error.get().map(|e| view! { <span class="muted">{e}</span> })}
        </form>
    }
}
