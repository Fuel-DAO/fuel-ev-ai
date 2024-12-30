use leptos::*;

#[component]
pub fn CarRental() -> impl IntoView {
    view! {
        <section class="flex items-center justify-evenly bg-green-50 p-6 gap-6  rounded-lg">
            <img src="/public/img/Group.png" alt="Left Image" class="w-1/3 h-auto" />

            <div class="flex flex-col justify-center p-4 opacity-100 lg:p-0 lg:h-auto lg:w-1/2 lg:gap-10">
                <div class="flex gap-0 items-center lg:h-auto lg:w-full">
                    <p class="font-bold leading-none text-left font-baloo text-5xl">
                        Book a premium <span class="text-green-500">EV</span> vehicle now
                    </p>
                </div>

                <div class="h-auto gap-2 lg:w-full">
                    <p class="font-normal text-left font-lato text-base leading-relaxed">
                        Take a ride in our brand new MG ZS EV, launching in Mumbai. Experience luxury with the environmental benefits of an electric vehicle.
                    </p>
                </div>

                <div class="flex">
                    <a
                        target="_blank"
                        href="https://fuelev.in"
                        class="w-40 lg:h-14 py-2 px-6 border-2 border-solid border-green-500 bg-green-500 text-white rounded-tl-md text-left"
                    >
                        Coming Soon
                    </a>
                </div>
            </div>
        </section>
    }
}
