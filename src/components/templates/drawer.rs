use leptos::prelude::*;
use reactive_stores::Store;

use crate::models::global_state::{AppState, AppStateStoreFields};

#[island]
pub fn Drawer() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();
    let toggle_menu = move |_| app_state.is_open().set(!app_state.is_open().get());
    let close_drawer = move |_| app_state.is_open().set(false);

    let overlay = move || if app_state.is_open().get() {
        Some(view! {
            <div
                class="fixed inset-0 z-40 bg-black bg-opacity-50"
                on:click=close_drawer
            />
        })
    } else {
        None
    };

    view! {
        <div class="">
            <button
                on:click=toggle_menu
                class="fixed top-4 left-4 z-50 p-2 bg-blue-500 text-white rounded shadow hover:bg-blue-600"
            >
            <span data-hk="0-0-0-10" class="w-6 h-1 bg-black dark:bg-white block my-4 relative after:absolute after:block after:bg-black dark:after:bg-white after:w-6 after:h-1 after:bottom-2 before:absolute before:block before:bg-black dark:before:bg-white before:w-6 before:h-1 before:-bottom-2"></span>
            </button>
            {overlay}
            <div
                class=("translate-x-0", move || app_state.is_open().get())
                class=("-translate-x-full", move || !app_state.is_open().get()) 
                class="fixed z-50 top-0 left-0 h-screen w-64 bg-gray-800 text-white p-4 shadow-lg transform transition-transform duration-300 flex justify-between flex-col"
            >
                <div class="h-full">
                    <div class="p-5">
                        <img src="/logo.webp" width="640" height="640" class="size-24 rounded-full mx-auto" />
                        <h2 class="font-bold mt-4 font-akira text-xl">Rust Perú</h2>
                        <nav class="text-left mt-4 px-2">
                            <ul class="space-y-2 list-disc">
                                <li class="list-image-learning">
                                    <a href="https://rustlang-es.org/aprende" class="align-super text-white hover:text-blue-500">"Aprende"</a>
                                </li>
                                <li class="list-image-book">
                                    <a href="https://book.rustlang-es.org/" class="block text-white hover:text-blue-500">"El libro"</a>
                                </li>
                                <li class="list-image-communities">
                                    <a href="https://rustlang-es.org/comunidades" class="block text-white hover:text-blue-500">"Otras Comunidades"</a>
                                </li>
                            </ul>
                        </nav>
                    </div>
                    <div class="mt-4">
                        <p>"Únete a nuestra comunidad de Telegram"</p>
                        <a href="https://t.me/rustperu" class="bg-neon-500 text-white p-2 rounded-lg">
                            <img src="https://img.shields.io/endpoint?color=neon&style=flat&url=https://tg.sumanjay.workers.dev/rustperu" class="mx-auto" />
                        </a>
                    </div>
                </div>
                <div>
                    <p>"Hecho por la comunidad de Rust Perú con apoyo de "<a href="https://rustlang-es.org/" class="text-blue-500">"RustLang en Español"</a></p>
                </div>
            </div>
        </div>
    }
}
