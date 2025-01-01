use leptos::prelude::*;
use reactive_stores::Store;

use crate::{app, components::atoms::section_separator::Separator, models::global_state::{AppState, AppStateStoreFields}};

#[island]
pub fn Hero() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();


    view! {
        // <div class="h-screen flex flex-col justify-center items-center text-center text-white bg-cover bg-center bg-machu-picchu bg-blend-hard-light bg-black/60">
        <div class="h-screen flex flex-col justify-center items-center text-center text-white bg-cover bg-center bg-machu-picchu bg-blend-hard-light bg-black/60 justify-between">
            <div class="h-full flex flex-col justify-center items-center">
                <img src="/logo.png" width="640" height="640" class="w-64 h-64 rounded-full" />
                <h1
                    class="text-6xl font-bold mt-4 bg-black bg-opacity-70 p-4 rounded-md font-akira"
                >Rust Perú</h1>
            </div>
            <div class="w-full h-16 text-2xl flex text-orange-oxided-50">
                <Separator />
            </div>
        </div>
    }
}
