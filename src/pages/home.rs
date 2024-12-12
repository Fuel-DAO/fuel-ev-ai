use crate::components::{
    footer::Footer, header::Header, home_banner::HeroSection, powered_by::PoweredBySection,
    ride_sharing::DecentralizedRidesharing, ridesharing_info::RideSharingInfo,
    trip_includes::TripIncludes,
};
use leptos::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="FuelEV" />
        <main>
            <Header />
            <HeroSection />
            <DecentralizedRidesharing />
            <RideSharingInfo />
            <TripIncludes />
            <PoweredBySection />
            <Footer />
        </main>
    }
}

