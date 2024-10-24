use leptos::*;

#[component]
pub fn RideSharingInfo() -> impl IntoView {
    view! { 
        <section class="flex h-screen lg:flex-row flex-col-reverse lg:items-stretch">
            <div class="lg:w-1/2 w-full h-1/2 lg:h-full">
                <img src="/public/img/info.webp" alt="Car on the road" class="w-full h-full object-cover" />
            </div>

            <div class="lg:w-1/2 w-full px-6 py-8 lg:py-20 lg:px-12 bg-white flex flex-col gap-4 justify-evenly ">
                <div class="mb-12">
                    <div class="flex items-center">
                        <span class="h-3 w-3 bg-green-500 rounded-full inline-block mr-4"></span>
                        <h2 class="font-bold lg:text-4xl text-3xl">"Decentralized control"</h2>
                    </div>
                    <p class="text-gray-700 mt-4">
                        "Fully community-owned ridesharing fleet, powered by blockchain technology"
                    </p>
                </div>

                <div class="mb-12">
                    <div class="flex items-center">
                        <span class="h-3 w-3 bg-green-500 rounded-full inline-block mr-4"></span>
                        <h2 class="font-bold lg:text-4xl text-3xl">"Premium experience, guaranteed"</h2>
                    </div>
                    <p class="text-gray-700 mt-4">
                        "80% Premium Tesla cars, 20% high-end fuel cars; complete with 24/7 road assistance and insurance"
                    </p>
                </div>

                <div>
                    <div class="flex items-center">
                        <span class="h-3 w-3 bg-green-500 rounded-full inline-block mr-4"></span>
                        <h2 class="font-bold lg:text-4xl text-3xl">"Sustainability, always available"</h2>
                    </div>
                    <p class="text-gray-700 mt-4">
                        "Unique AI algorithms, keeping a car available around you—always"
                    </p>
                </div>
            </div>
        </section>
    }
}
