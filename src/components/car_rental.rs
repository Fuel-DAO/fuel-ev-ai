use leptos::*;

#[component]
pub fn CarRental() -> impl IntoView {
    view! {
        <section class="flex flex-col lg:flex-row items-center justify-evenly bg-green-50 p-10 gap-6  rounded-lg">
            <img src="/public/img/Group.png" alt="Left Image" class="w-1/3 h-auto p-8" />

            <div class="flex flex-col justify-center p-4 opacity-100 lg:p-0 lg:h-auto lg:w-1/2 lg:gap-10">
                <div class="flex gap-0 items-center lg:h-auto lg:w-full">
                    <p class="font-bold leading-none text-left font-baloo text-5xl">
                        Book a premium <span class="text-green-500">EV</span>{format!(" vehicle now",)}
                    </p>
                </div>

                <div class="h-auto gap-2 lg:w-full">
                    <p class="font-normal text-left font-lato text-base leading-relaxed">
                        Take a ride in our brand new MG ZS EV, launching in Mumbai. Experience luxury with the environmental benefits of an electric vehicle.
                    </p>
                </div>

                <a target="_blank" href="https://fuelev.in">
                    <button class="bg-green-500 hover:bg-green-700 text-white font-bold xl:text-2xl xl:px-8 xl:py-3 px-6 py-3 rounded-full shadow-lg">
                        "Invest Now"
                    </button>
                </a>
            </div>
        </section>
    }
}
