use leptos::*;

#[component]
pub fn PoweredBySection() -> impl IntoView {
    view! { 
        <section class="py-12 bg-white">
            <div class="container mx-auto px-6 lg:px-8">
                <div class="flex flex-col justify-center items-center">
                <h3 class="text-xl lg:text-2xl font-semibold">"Powered by"</h3>
                            <div class="flex justify-center p-6 ">
                            <img src="/public/img/internet-computer.svg" alt="Internet Computer Logo" class="w-48 h-auto"/>
                            </div>
                    
                    <div class="text-center lg:text-left">
                        
                        <p class="text-gray-700 text-base leading-relaxed">
                            "The Internet Computer (ICP) allows Web3 services to run 100% on-chain, providing the only platform where developers can build and users can enjoy fully decentralized applications. FuelDAO is set to leverage the power of DFINITY's decentralized technology, specifically the SNS (Service Neuron System). Through integration with the SNS, the FuelDAO application will evolve into a DAO (Decentralized Autonomous Organization) with tokenized governance functionality, paving the way for a decentralized and community-driven ecosystem."
                        </p>
                    </div>

                    

                </div>
            </div>
        </section>
    }
}
