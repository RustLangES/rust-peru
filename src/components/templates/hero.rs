use leptos::prelude::*;
use reactive_stores::Store;

use crate::{app, components::atoms::{section_separator::Separator}, models::global_state::{AppState, AppStateStoreFields}};


#[island]
pub fn Hero() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
        
    let telegram_icon = include_str!("../../../public/assets/images/telegram_icon.svg");

    view! {
        // <div class="h-screen flex flex-col justify-center items-center text-center text-white bg-cover bg-center bg-machu-picchu bg-blend-hard-light bg-black/60">
        <div class="h-screen flex flex-col justify-center items-center text-center text-white bg-cover bg-center bg-machu-picchu bg-blend-hard-light bg-black/60 justify-between">
            <div class="h-full flex flex-col justify-center items-center">
                <img src="/logo.png" width="640" height="640" class="w-64 h-64 rounded-full" />
                <h1 class="text-6xl font-bold mt-4 bg-black bg-opacity-70 p-4 rounded-md font-akira">Rust Perú</h1>
                <div class="flex flex-col gap-4 mt-4 justify-center items-center">
                    <p class="text-2xl">"Únete a nuestra comunidad en:"</p>
                    <a href="https://t.me/rustperu" class="bg-black max-w-sm bg-opacity-70 text-white p-2 rounded-lg flex flex-row items-center justify-center font-bold gap-2 text-xl">
                        <span class="text-white fill-white color-white text-2xl" inner_html={telegram_icon}></span>
                        <span>"Telegram"</span>
                    </a>
                </div>
            </div>
            <div class="w-full h-16 text-2xl flex text-orange-oxided-50">
                <Separator />
            </div>
        </div>
    }
}
