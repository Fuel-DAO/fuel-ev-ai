use leptos::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section id="hero" class="h-dvh w-full flex items-start justify-start relative">
            // Text and CTA Section
            <div class="xl:px-32 z-[2] px-8 pt-48">
                <div class="text-4xl xl:text-6xl xl:leading-[1.05] font-extrabold mb-10">
                    "Driving Shared" <br />
                    "Mobility Forward," <br />
                    "Clean & Decentralized"
                </div>
                <a href="/collections">
                    <button class="bg-green-500 hover:bg-green-700 text-white font-bold xl:text-2xl xl:px-8 xl:py-3 px-6 py-3 rounded-full shadow-lg">
                        "Invest Now"
                    </button>
                </a>
            </div>

            // Background Image
            <div class="absolute inset-0 z-[1] overflow-hidden">
                <div class="h-full w-full lg:bg-cover bg-contain bg-no-repeat lg:bg-right bg-bottom scale-150 lg:scale-100 origin-bottom lg:origin-center"
                     style="background-image: url('/public/img/fuel-hero.webp');">
                </div>
            </div>
        </section>
    }
}
