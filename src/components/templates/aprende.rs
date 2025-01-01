use leptos::prelude::*;
use reactive_stores::Store;

use crate::{app, components::atoms::section_separator::Separator, models::global_state::{AppState, AppStateStoreFields}};

#[island]
pub fn AprendeSection() -> impl IntoView {
    let app_state = expect_context::<Store<AppState>>();


    view! {
        // Two columns
        <section class="grid md:grid-cols-2">
            // one column with information
            <div class="flex flex-col justify-center items-center p-4 md:py-12">
                <h3 class="text-3xl font-bold text-center">"¡Aprende Rust con nosotros!"</h3>
                <p class="text-center">En Rust Perú, te ofrecemos una serie de recursos para que puedas aprender Rust de manera sencilla y divertida.</p>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-gray-800 text-white p-4 rounded-lg">
                        <h4 class="text-2xl font-bold">Cursos</h4>
                        <p>En Rust Perú, te ofrecemos una serie de cursos para que puedas aprender Rust de manera sencilla y divertida.</p>
                    </div>
                    // <div class="bg-gray-800 text-white p-4 rounded-lg">
                    //     <h4 class="text-2xl font-bold">Talleres</h4>
                    //     <p>En Rust Perú, te ofrecemos una serie de talleres para que puedas aprender Rust de manera sencilla y divertida.</p>
                    // </div>
                    // <div class="bg-gray-800 text-white p-4 rounded-lg">
                    //     <h4 class="text-2xl font-bold">Charlas</h4>
                    //     <p>En Rust Perú, te ofrecemos una serie de charlas para que puedas aprender Rust de manera sencilla y divertida.</p>
                    // </div>
                    <div class="bg-gray-800 text-white p-4 rounded-lg">
                        <h4 class="text-2xl font-bold">Comunidad</h4>
                        <p>En Rust Perú, te ofrecemos una serie de recursos para que puedas aprender Rust de manera sencilla y divertida.</p>
                    </div>
                </div>
            </div>
            // one column with images
            <div>
                <div class="relative columns-1 sm:columns-3 gap-8">
                    <div class="relative aspect-w-16 aspect-h-9">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/RustMX2018.png" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="relative aspect-w-1 aspect-h-1 mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/Peru2024.png" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="relative aspect-w-1 aspect-h-1 mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/MX2024v2.png" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-1 aspect-h-1 mt-8 sm:mt-0">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/MX2024.jpg" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-16 aspect-h-9 mt-8">
                      <img class="w-full object-cover rounded-lg" src="https://images.unsplash.com/photo-1611605645802-c21be743c321?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=2940&amp;q=80" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-1 aspect-h-1 mt-8">
                      <img class="w-full object-cover rounded-lg" src="https://images.unsplash.com/photo-1498603993951-8a027a8a8f84?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=2936&amp;q=80" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-1 aspect-h-1 mt-8 sm:mt-0">
                      <img class="w-full object-cover rounded-lg" src="https://images.unsplash.com/photo-1526400473556-aac12354f3db?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=2940&amp;q=80" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-1 aspect-h-1 mt-8">
                      <img class="w-full object-cover rounded-lg" src="https://images.unsplash.com/photo-1617369120004-4fc70312c5e6?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=1587&amp;q=80" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-16 aspect-h-9 mt-8">
                      <img class="w-full object-cover rounded-lg" src="https://images.unsplash.com/photo-1518892096458-a169843d7f7f?ixlib=rb-1.2.1&amp;ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&amp;auto=format&amp;fit=crop&amp;w=2940&amp;q=80" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                  </div>
            </div>
        </section>
    }
}
