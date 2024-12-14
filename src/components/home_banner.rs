use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! { 
        <section class="relative flex flex-col md:flex-row items-center justify-between h-screen p-12 bg-cover bg-center" 
                 style="background-image: url('/public/img/fuel-hero.webp');">

            // Text and CTA Section
            <div class="max-w-lg md:max-w-2xl lg:max-w-3xl mt-10 ml-2">
                <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold leading-tight text-black">
                    "Driving Shared"
                    <br />
                    <span >Mobility Forward, </span><br/>
                    "Clean & Decentralized"
                </h1>
                <div class="mt-6">
                    <a href="/collections">
                    <button class="bg-green-500 hover:bg-green-700 text-white font-bold py-3 px-8 rounded-full shadow-lg">
                        "Invest Now"
                    </button>
                    </a>
                </div>
            </div>

            // Car Image
            // <div class="mt-8 md:mt-0 md:max-w-md lg:max-w-lg">
            //     <img src="/public/img/car-charging.png" class="w-full h-auto"/>
            // </div>
        </section>
    }
}
