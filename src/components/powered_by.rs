use leptos::*;

#[component]
pub fn PoweredBySection() -> impl IntoView {
    view! {
        <section id="powered-by" class="flex flex-col items-center justify-center gap-16 lg:px-16 px-8 lg:py-20 py-16">
            <div class="flex flex-col gap-6 items-center">
                <div class="font-normal text-2xl">
                    "Powered by"
                </div>
                <img src="/public/img/internet-computer.svg" alt="Internet Computer" class="w-64"/>
            </div>

            <div class="lg:text-lg text-lg font-normal max-w-7xl lg:leading-normal">
                <p class="text-gray-700">
                    "The Internet Computer (ICP) allows Web3 services to run 100% on-chain, providing the only platform where developers can build and users can enjoy fully decentralized applications. FuelDAO is set to leverage the power of DFINITY’s decentralized technology, specifically the SNS (Service Neuron System). Through integration with the SNS, the FuelDAO application will evolve into a DAO (Decentralized Autonomous Organization) with tokenized governance functionality, paving the way for a decentralized and community-driven ecosystem."
                </p>
            </div>
        </section>
    }
}
