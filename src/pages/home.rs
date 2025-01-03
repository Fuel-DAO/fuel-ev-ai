use crate::components::{
    car_rental::CarRental, footer::Footer, header::Header, home_banner::HeroSection, powered_by::PoweredBySection, ride_sharing::DecentralizedRidesharing, trip_includes::TripIncludes
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
            // <RideSharingInfo />
            <TripIncludes />
            <CarRental />
            <PoweredBySection />
            <Footer />
        </main>
    }
}

