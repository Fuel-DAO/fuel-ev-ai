use leptos::*;

#[component]
pub fn DecentralizedRidesharing() -> impl IntoView {
    view! {
        <section id="aboutus" class="lg:h-dvh w-full flex flex-col lg:flex-row">
            // Text Content Section
            <div class="h-full bg-white text-black flex flex-col items-start justify-center gap-8 xl:px-32 px-8 py-16 lg:w-1/2">
                <h2 class="font-bold text-3xl lg:text-6xl text-green-600">
                    Decentralized <br />Ridesharing
                </h2>
                <p class="lg:text-xl text-lg text-gray-600">
                    "FuelEV is a pioneering decentralized autonomous organization that offers eco-friendly transport services like electric car rentals, ride-sharing, and fuel-efficient vehicle leasing. We will be providing quality transport services to clients, starting with Mumbai and the surrounding area."
                </p>
                <p class="lg:text-xl text-lg text-gray-600">
                    "Built by a team that comes with 30+ years combined experience in Web3, FuelEV aims to make your ride experience clean and simple."
                </p>
                <a href="https://fueldao.notion.site/FuelEV-Whitepaper-155aa15ad3e580dbb485c667b1f96a36"
                   target="_blank"
                   class="bg-green-500 hover:bg-green-700 text-white font-bold py-3 px-8 rounded-full shadow-lg lg:text-xl lg:px-6 lg:py-3">
                    "Our Whitepaper"
                </a>
            </div>

            // Image Section
            <div class="h-96 lg:h-full w-full lg:w-1/2 bg-center bg-cover shrink-0"
                 style="background-image: url('/public/img/rideshare.jpg');">
            </div>
        </section>

        <section class="lg:min-h-dvh lg:h-auto w-full flex lg:flex-row flex-col-reverse lg:items-stretch">
            // Image Section
            <div class="lg:h-auto h-96 bg-center bg-cover shrink-0 w-full lg:w-1/2"
                 style="background-image: url('/public/img/info.webp');">
            </div>

            // Features Section
            <div class="h-full w-full bg-white text-black flex flex-col items-start justify-center gap-12 lg:h-auto lg:px-32 py-16 px-8">
                <div class="flex items-start gap-4">
                    <div class="flex items-center justify-center lg:h-10 lg:w-10 h-9">
                        <div class="lg:h-5 lg:w-5 h-4 w-4 rounded-full bg-green-600"></div>
                    </div>
                    <div class="flex flex-col gap-4">
                        <h3 class="font-bold lg:text-4xl text-3xl">
                            "Decentralized Control"
                        </h3>
                        <p class="text-xl leading-normal">
                            "Fully community-owned ridesharing fleet, powered by blockchain technology."
                        </p>
                    </div>
                </div>

                <div class="flex items-start gap-4">
                    <div class="flex items-center justify-center lg:h-10 lg:w-10 h-9">
                        <div class="lg:h-5 lg:w-5 h-4 w-4 rounded-full bg-green-600"></div>
                    </div>
                    <div class="flex flex-col gap-4">
                        <h3 class="font-bold lg:text-4xl text-3xl">
                            "Premium Experience, Guaranteed"
                        </h3>
                        <p class="text-xl leading-normal">
                            "80% Premium Morris Garages cars, 20% high-end fuel cars; complete with 24/7 road assistance and insurance"
                        </p>
                    </div>
                </div>

                <div class="flex items-start gap-4">
                    <div class="flex items-center justify-center lg:h-10 lg:w-10 h-9">
                        <div class="lg:h-5 lg:w-5 h-4 w-4 rounded-full bg-green-600"></div>
                    </div>
                    <div class="flex flex-col gap-4">
                        <h3 class="font-bold lg:text-4xl text-3xl">
                            "Sustainability, Always Available"
                        </h3>
                        <p class="text-xl leading-normal">
                            "Unique AI algorithms, keeping a car available around you — always."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
