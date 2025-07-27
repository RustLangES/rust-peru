use leptos::prelude::*;

#[island]
pub fn AprendeSection() -> impl IntoView {

    view! {
        <section class="grid md:grid-cols-2 gap-4 p-5">
            <div class="flex flex-col gap-5 justify-center items-center p-4 md:py-12 mx-5">
                <h3 class="text-3xl font-bold text-center">"¡Aprende Rust con nosotros!"</h3>
                <p class="text-center">"En Rust Perú, fomentamos la participación, la integración, la colaboración y la diversidad en la comunidad. Nuestro objetivo es promover la adopción de Rust en el Perú y en la región, a través de la educación, la formación y la colaboración."</p>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div class="bg-orange-oxided-200 text-black p-4 rounded-lg font-inter">
                        <h4 class="text-2xl font-bold mb-2">Comunidad</h4>
                        <p>"La comunidad de Rust Perú es muy activa y siempre está dispuesta a ayudarte. Si tienes alguna duda, pregunta o sugerencia, no dudes en contactarnos, siempre estamos a un mensaje de distancia."</p>
                    </div>
                    <div class="bg-orange-oxided-200 text-black p-4 rounded-lg font-inter">
                      <h4 class="text-2xl font-bold mb-2">Charlas</h4>
                      <p>"En Rust Perú, solemos realizar charlas y eventos para que puedas aprender Rust de manera sencilla y divertida. A veces en lugares públicos, a veces en línea y en otras ocasiones en empresas o coworkings que nos detinan un espacio."</p>
                    </div>
                </div>
                <div class="bg-orange-oxided-100 bg-blend-darken bg-call-to-action bg-cover text-black p-4 rounded-lg font-inter max-w-lg flex flex-col items-center justify-center gap-2">
                  <h4 class="text-2xl font-bold mb-2">"Call to Action"</h4>
                  <p>"Si quieres dar un paso en tu carrera profesional, volverte divulgador, o simplemente quieres compartir algo, ¡No dudes!"</p>
                  <p>"Siempre estamos buscando personas que quieran compartir sus conocimientos, experiencias y proyectos con la comunidad."</p>
                  <p>"¡Podría ser una gran oportunidad!"</p>
                </div>
            </div>
            <div class="p-3">
                <div class="relative flex flex-row sm:block columns-1 sm:columns-3 gap-1 md:gap-8">
                    <div class="relative aspect-w-16 aspect-h-9">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/RustMX2018.webp" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg  h-fit"></div>
                    </div>
                    <div class="relative aspect-w-1 aspect-h-1 md:mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/Peru2024.webp" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg h-fit"></div>
                    </div>
                    <div class="relative aspect-w-1 aspect-h-1 md:mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/MX2024v2.avif" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg  h-fit"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-1 aspect-h-1 mt-8 sm:mt-0">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/MX2024.avif" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-16 aspect-h-9 mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/nerdearla2024.avif" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-16 aspect-h-9 mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/sergio2024.avif" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                    <div class="hidden sm:block relative aspect-w-16 aspect-h-9 mt-8">
                      <img class="w-full object-cover rounded-lg" src="./assets/images/sticker.avif" />
                      <div class="absolute inset-0 ring-1 ring-inset ring-black/10 rounded-lg"></div>
                    </div>
                  </div>
            </div>
        </section>
    }
}
