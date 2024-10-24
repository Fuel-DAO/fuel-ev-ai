use leptos::*;
use leptos_meta::Title;
use crate::components::{header::Header, footer::Footer, home_banner::HeroSection, ride_sharing::DecentralizedRidesharing, ridesharing_info::RideSharingInfo, trip_includes::TripIncludes, powered_by::PoweredBySection};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelDao" />
        <main>
                <Header />
                <HeroSection />
                <DecentralizedRidesharing />
                <RideSharingInfo />
                // <TripIncludes />
                <PoweredBySection />
                <Footer />
        </main>
    }
}