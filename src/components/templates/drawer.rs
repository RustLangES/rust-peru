use leptos::prelude::*;
use reactive_stores::Store;

use crate::{app, models::global_state::{AppState, AppStateStoreFields}};

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
                class="fixed top-0 left-0 h-screen w-64 bg-gray-800 text-white p-4 shadow-lg transform transition-transform duration-300"
            >
                olaaaa
            </div>
        </div>
    }
}
