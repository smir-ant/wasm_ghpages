use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
// use crate::pages::not_found::NotFound;

/// Маргрутизатор, который формирует домашнюю страницу (можно и другие пути, например 404 через добавление Route в Routes)
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();  // контекст, который управляет различными аспектами веб-страницы, такими как стили, заголовки, мета-теги и т.д.
    // “контекст” обычно относится к набору данных или состоянию, которое доступно всему приложению или определенной его части

    view! {
        <Html lang="en" dir="ltr" />  // ltr (слева-направо направление текста)
        <Title text="Wasm(Leptos) via Github Pages"/>  // название страницы
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        // <Stylesheet id="tailwind" href="/public/tailwind.css"/>

        <Router>
            <Routes>
                <Route path="/wasm_ghpages/*" view=Home />
            </Routes>
        </Router>
    }
}
