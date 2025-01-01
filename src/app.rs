use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes}, static_routes::StaticRoute, SsrMode, StaticSegment
};
use reactive_stores::Store;

use crate::{components::templates::{aprende::AprendeSection, drawer::Drawer, hero::Hero}, models::global_state::AppState};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="es">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="description" content="Rust Perú"/>
                <link rel="icon" href="/assets/images/icon.png" type="image/png"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="bg-orange-oxided-50 h-screen flex flex-col">
                <App/>
            </body>
        </html>
    }
}

#[island]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-peru.css"/>

        // sets the document title
        <Title text="Rust Perú"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage ssr=SsrMode::Static(StaticRoute::new())/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[island]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let app_state = Store::new(AppState::default());
    provide_context(app_state);

    view! {
        <div>
            <Drawer />
            <main>
                <Hero></Hero>
                <AprendeSection />
              </main>
        </div>
    }
}
