use leptos::*;

#[component]
pub fn TripIncludes() -> impl IntoView {
    view! { 
        <section class="bg-zinc-800 text-white py-12">
            <div class="text-center mb-10">
                <h2 class="text-3xl lg:text-4xl font-semibold">"Every trip includes"</h2>
            </div>

            <div class="flex flex-col lg:flex-row justify-around items-center space-y-8 lg:space-y-0 lg:space-x-6">
                <div class="flex flex-col items-center">
                    <img src="/public/icons//icons.sprite.svg" alt="Free charging across Barcelona" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Intelligent pricing"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="path/to/icon2.png" alt="Free charging across Barcelona" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Free charging across Barcelona"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="path/to/icon3.png" alt="Free parking" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Free parking"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="path/to/icon4.png" alt="Safety, 24/7" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Safety, 24/7"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="path/to/icon5.png" alt="Premium cars only" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Premium cars only"</p>
                </div>
            </div>
        </section>
    }
}
