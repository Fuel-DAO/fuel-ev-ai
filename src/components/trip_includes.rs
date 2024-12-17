use leptos::*;

#[component]
pub fn TripIncludes() -> impl IntoView {
    view! {
        <section id="features" class="flex flex-col items-center justify-center bg-zinc-800 text-white gap-16 lg:px-16 px-8 lg:py-24 py-16">
            <div class="font-semibold text-center lg:text-5xl text-3xl">
                "Every trip includes"
            </div>

            <div class="flex flex-col lg:flex-row items-center justify-around w-full gap-16 lg:gap-4">
                <div class="flex flex-col gap-2 items-center">
                    <img src="/public/icons/price.svg" alt="Intelligent pricing" class="w-14 h-14"/>
                    <div class="text-center">
                        "Intelligent" <br/> "pricing"
                    </div>
                </div>

                <div class="flex flex-col gap-2 items-center">
                    <img src="/public/icons/free.svg" alt="Free charging across Mumbai" class="w-14 h-14"/>
                    <div class="text-center">
                        "Free charging across" <br/> "Mumbai"
                    </div>
                </div>

                <div class="flex flex-col gap-2 items-center">
                    <img src="/public/icons/parking.svg" alt="Free parking" class="w-14 h-14"/>
                    <div class="text-center">
                        "Free parking"
                    </div>
                </div>

                <div class="flex flex-col gap-2 items-center">
                    <img src="/public/icons/safety.svg" alt="Safety, 24/7" class="w-14 h-14"/>
                    <div class="text-center">
                        "Safety, 24/7"
                    </div>
                </div>

                <div class="flex flex-col gap-2 items-center">
                    <img src="/public/icons/cars.svg" alt="Premium cars only" class="w-14 h-14"/>
                    <div class="text-center">
                        "Premium cars" <br/> "only"
                    </div>
                </div>
            </div>
        </section>
    }
}
