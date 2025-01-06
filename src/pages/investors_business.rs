use leptos::*;

use crate::{canister::backend::CarTravelStats, state::canisters::Canisters, time::get_day_month_time};
#[component]
pub fn InvestorsBookingDashboard() -> impl IntoView {

    let stats = create_resource(||(), |_| {
        async move {
            let backend = Canisters::get().ok_or(format!("Canisters not initialized"))?;
            let backend = backend.backend_canister().await;
            backend.car_stats().await.map_err(|f| format!("Failed to get stats {f:?}"))
        }
    });

    view! {
        <a href="/" class="flex items-center justify-between p-4 bg-white shadow">
            <img src="/public/img/app.svg" alt="FuelEV Logo" class="h-8" />
        </a>

        // Hero Section
        <section>
            <img src="/public/img/business_banner.svg" alt="Hero Image" class="w-full h-96 object-cover" />
        </section>

        <div class="text-center p-4">
            <h1 class="text-3xl font-bold">"FuelEV Business Dashboard"</h1>
        </div>

        <Suspense>
            {move || {
                stats
                    .get()
                    .map(|f| match f {
                        Ok(stats) => {
                            view! {
                                <div>
                                    // Metrics Section
                                    <Matrics clone:stats stats=stats.clone() />

                                    // Access Live Fleet Section
                                    <section class="flex items-center justify-evenly bg-green-50 p-6 gap-6  rounded-lg shadow-md">
                                        <img
                                            src="/public/img/Group.png"
                                            alt="Fleet Image"
                                            class="w-1/3 h-auto"
                                        />
                                        <div class="w-1/3 text-left">
                                            <h2 class="text-xl font-bold mb-4">
                                                "Access this Fleet Live"
                                            </h2>
                                            <p class="text-gray-600 mb-4">
                                                "Setting a benchmark in operating decentralized fleets globally, we’re introducing access to our fleet’s live dashcam and GPS data."
                                            </p>
                                            <div class="mb-4">
                                                <p class="font-bold">
                                                    "Username:" <span class="text-gray-600">"AushaFueldao"</span>
                                                </p>
                                                <p class="font-bold">
                                                    "Password:" <span class="text-gray-600">"123456"</span>
                                                </p>
                                            </div>
                                            <a target="_blank" href="http://47.241.210.146" class="bg-green-500 text-white px-4 py-2 rounded">
                                                "Click Here"
                                            </a>
                                        </div>
                                    </section>

                                    // Bookings Table
                                    <Bookings clone:stats stats />
                                </div>
                            }
                        }
                        Err(e) => view! { <div>{e}</div> },
                    })
            }}
        </Suspense>
        <Footer />
    }
}


#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="flex flex-col items-start py-32 px-8 bg-zinc-800 text-white gap-16">
            <img src="/public/img/white.png" alt="FuelEVWhite" class="h-12" />
            <div class="items-center w-full flex flex-col lg:flex-row border border-white lg:border-l-0 lg:border-r-0">
                <div class="lg:w-1/3 w-full flex justify-center items-start flex-col border-b lg:border-r self-stretch p-8 border-white">
                    <a href="mailto:info@fueldao.io">info@fueldao.io</a>
                </div>
                <div class="lg:w-1/3 w-full flex justify-center items-start self-stretch p-8 flex-col lg:border-r border-white">
                    <div>Mumbai, India</div>
                </div>
                <div class="lg:w-1/3 lg:flex justify-center items-center self-stretch p-8 flex-row gap-2 border border-white lg:border-l-0 lg:border-r-0">
                    <div>Made with " ❤️ " using</div>
                    <img src="/public/icons/icp.png" alt="Internet Computer" class="h-6 m-1" />
                </div>
            </div>

            // <form  class="flex flex-col lg:gap-4 gap-2">
            // <div class="text-xl lg:text-5xl font-thin">Stay informed, join our newsletter</div>
            // <div class="lg:text-xl font-thin pt-2">Enter your email here</div>
            // <div class="flex items-center gap-2 lg:gap-4">
            // <input
            // required
            // type="email"
            // placeholder="youremail@email.com"
            // class="lg:text-2xl text-lg border border-r-0 border-t-0 border-white/50 focus:border-white transition-colors lg:p-2 p-1 border-l-0 bg-transparent focus:outline-none placeholder:text-white/30"
            // />
            // // <Button   class="lg:text-lg lg:py-3 lg:px-8 py-2 px-6">Submit</Button>
            // </div>
            // </form>
            <div class="h-20 w-full lg:h-1"></div>
        </footer>
    }
}


#[component]
fn Matrics(stats: CarTravelStats) -> impl IntoView {
    let total_revenue = stats.total_revenue / 84.0;
    let total_distance = if stats.total_distance_travelled.is_sign_negative() {
        stats.total_distance_travelled * -1.0
    } else {
        stats.total_distance_travelled
    };
    let reduced_carbon_emissions = (0.285)*total_distance - (0.103) * total_distance;
    let yield_percentage = (stats.total_revenue /stats.total_investment) * 100.0;
    view! {
        <section class="grid grid-cols-2 gap-6 p-8 text-center">
            <div class="border rounded p-4 shadow">
                <p class="text-xl font-bold">${format!("{:.2}", total_revenue)}</p>
                <p class="text-gray-500">"Total Revenue"</p>
            </div>
            <div class="border rounded p-4 shadow">
                <p class="text-xl font-bold">{format!("{:.3}",total_distance)} km</p>
                <p class="text-gray-500">"Distance Covered"</p>
            </div>
            <div class="border rounded p-4 shadow">
                <p class="text-xl font-bold">{format!("{:.3}", yield_percentage)}%</p>
                <p class="text-gray-500">"Yield Percentage"</p>
            </div>
            <div class="border rounded p-4 shadow">
                <p class="text-xl font-bold">{format!("{:.3}", reduced_carbon_emissions)} kg</p>
                <p class="text-gray-500">"Reduced Carbon Emissions"</p>
            </div>
        </section>
    }

}

#[component]
fn Bookings(stats: CarTravelStats) -> impl IntoView {

    view! {
        <section class="p-8">
                                        <h2 class="text-2xl font-bold mb-4 text-center">
                                            "List of Bookings"
                                        </h2>
                                        <table class="min-w-full border-collapse border border-gray-200">
                                            <thead>
                                                <tr class="bg-gray-100">
                                                    <th class="border border-gray-300 px-4 py-2">
                                                        "Booking ID"
                                                    </th>
                                                    <th class="border border-gray-300 px-4 py-2">
                                                        "Booking Start Date"
                                                    </th>
                                                    <th class="border border-gray-300 px-4 py-2">
                                                        "Booking End Date"
                                                    </th>
                                                    <th class="border border-gray-300 px-4 py-2">
                                                        "Booking Revenue"
                                                    </th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                            {
                                                move || {
                                                    let mut rows = Vec::new();
                                                    for booking in stats.rentals.iter() {
                                                        rows.push(view! {
                                                            <tr>
                                                                <td class="border border-gray-300 px-4 py-2  text-center">
                                                                    {booking.car_id}-{booking.booking_id}
                                                                </td>
                                                                <td class="border border-gray-300 px-4 py-2  text-center">
                                                                    {format!("{}", get_day_month_time(booking.start_timestamp)) }
                                                                </td>
                                                                <td class="border border-gray-300 px-4 py-2  text-center">
                                                                    {format!("{}", get_day_month_time(booking.end_timestamp)) }
                                                                </td>
                                                                <td class="border border-gray-300 px-4 py-2  text-center">
                                                                    ${format!("{:.2}", booking.total_amount/84.0)}
                                                                </td>
                                                            </tr>
                                                        });
                                                    }
                                                    rows
                                                }
                                            }
                                            // Add more rows here...
                                            </tbody>
                                        </table>
                                    </section>
    }

}