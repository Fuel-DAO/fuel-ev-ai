use leptos::*;

#[component]
pub fn DecentralizedRidesharing() -> impl IntoView {
    view! { 
        <section class="flex flex-col md:flex-row items-center justify-between h-screen bg-white">
            
            // Text Content Section
            <div class="p-8 md:w-1/2 max-w-lg md:max-w-xl lg:max-w-2xl">
                <h2 class="text-3xl md:text-4xl lg:text-5xl font-bold leading-tight text-green-600">
                    "Decentralized Ridesharing"
                </h2>
                <p class="mt-4 text-lg md:text-xl text-gray-600">
                    "FuelEv is a pioneering decentralized autonomous organization that offers eco-friendly transport services like electric car rentals, ride-sharing, and fuel-efficient vehicle leasing. We will be providing quality transport services to clients, starting with Mumbai and the surrounding area."
                </p>
                <p class="mt-4 text-lg md:text-xl text-gray-600">
                    "Built by a team that comes with 30+ years combined experience in Web3, FuelEv aims to make your ride experience clean and simple."
                </p>
                
                // Call-to-Action Button
                <div class="mt-6">
                    <a href="https://fueldao.notion.site/FuelEV-Whitepaper-155aa15ad3e580dbb485c667b1f96a36" target="_blank" class="bg-green-500 hover:bg-green-700 text-white font-bold py-3 px-8 rounded-full shadow-lg">
                        "Our Whitepaper"
                    </a>
                </div>
            </div>

            // Image Section
            <div class="md:w-1/2 mt-8 md:mt-0">
                <img src="/public/img/ridesharing.webp" alt="Aerial view of car on road" class="w-full h-screen/2 md:h-screen  shadow-md"/>
            </div>
        </section>
    }
}
