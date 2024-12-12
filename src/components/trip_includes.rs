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
                    <img src="/public/icons/price.svg" alt="Free charging across Mumbai" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Intelligent pricing"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="/public/icons/free.svg" alt="Free charging across Mumbai" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Free charging across Mumbai"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="/public/icons/parking.svg" alt="Free parking" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Free parking"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="/public/icons/safety.svg" alt="Safety, 24/7" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Safety, 24/7"</p>
                </div>
                <div class="flex flex-col items-center">
                    <img src="/public/icons/cars.svg" alt="Premium cars only" class="w-12 h-12 mb-4"/>
                    <p class="text-center text-lg">"Premium cars only"</p>
                </div>
            </div>
        </section>
    }
}
