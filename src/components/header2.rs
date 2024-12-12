use leptos::*;

#[component]
pub fn Header2() -> impl IntoView {
    view! {
        <div class="w-full fixed z-50 h-20 shadow-sm flex items-center justify-between px-8 font-light transition-all bg-white/90 backdrop-blur-md">

            <button class="z-[1] hidden lg:block text-gray-700 hover:text-gray-900">
                "← Go back"
            </button>

            <button class="z-[1] lg:hidden pr-4 text-gray-700 hover:text-gray-900">"←"</button>

            <div class="absolute z-0 inset-x-0 flex items-center pl-8 justify-center">
                <a href="/">
                    <img src="/public/img/app.svg" alt="FuelEV" class="lg:h-8 h-5" />
                </a>
            </div>

            <div class="lg:hidden absolute right-8">
                <button class="text-gray-700 hover:text-gray-900">
                    <svg
                        class="h-6 w-6"
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 24 24"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M4 6h16M4 12h16M4 18h16"
                        ></path>
                    </svg>
                </button>
            </div>

            <div class="absolute z-[1] lg:flex hidden right-8 items-center gap-8">
                <a href="/collections">Collections</a>
                <a
                    role="presentation"
                    href="/login"
                    class="h-10 w-10 bg-black flex items-center text-xl select-none justify-center font-light text-white rounded-full uppercase"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="h-4 w-4"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M15.75 6a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0ZM4.501 20.118a7.5 7.5 0 0 1 14.998 0A17.933 17.933 0 0 1 12 21.75c-2.676 0-5.216-.584-7.499-1.632Z"
                        ></path>
                    </svg>
                </a>
            </div>
        </div>
    }
}
