use leptos::*;
use crate::canister::token::CollectionMetaData;
// #[derive(Clone, Debug)]
// pub struct CollectionMetadata {
//     pub name: Option<String>,
//     pub status: String,
//     pub description: Option<String>,
//     pub weight: Option<String>,
//     pub drive_type: Option<String>,
//     pub displays: Option<String>,
//     pub seating: Option<String>,
//     pub cargo: Option<String>,
//     pub overall_height: Option<String>,
//     pub overall_width: Option<String>,
//     pub overall_length: Option<String>,
//     pub track_rear: Option<String>,
//     pub track_front: Option<String>,
//     pub ground_clearance: Option<String>,
//     pub key_features: Option<Vec<String>>,
//     pub range_per_charge: Option<String>,
//     pub acceleration: Option<String>,
//     pub charging_speed: Option<String>,
//     pub wheels: Option<String>,
//     pub purchase_price: Option<u32>, // Price in e8s or smallest unit
//     pub brochure_url: Option<String>,
//     pub battery: Option<String>,
//     pub supply_cap: Option<u64>, // Total supply cap (e.g., for NFTs)
//     pub price: Option<u64>,      // Price in e8s or smallest unit (e.g., for ICP)
//     pub collection_owner: Option<String>, // Owner's ID or name
// }

// impl Default for CollectionMetadata {
//     fn default() -> Self {
//         Self {
//             supply_cap: Some(102),
//             price: Some(1),
//             collection_owner: Some("principal".to_string()),
//             name: Some("Model S Plaid- SAMPLE".to_string()),
//             status: "Live".to_string(),
//             description: Some("The Tesla Model S Plaid is the epitome of electric luxury and performance, blending cutting-edge technology with sophisticated design. Boasting an impressive tri-motor all-wheel-drive system, it delivers a staggering 1,020 horsepower, propelling the vehicle from 0 to 60 mph in just under 2 seconds, making it one of the fastest production cars ever built. Its sleek, aerodynamic silhouette is both a visual and functional masterpiece, reducing drag while exuding elegance. Inside, the minimalist yet opulent cabin features a futuristic yoke steering wheel, a 17-inch cinematic display, and premium materials that envelop passengers in comfort and style. With an estimated range of over 390 miles on a single charge, the Model S Plaid is not just a marvel of engineering but also a paragon of sustainable luxury, setting new standards for what an electric vehicle can achieve.".to_string()),
//             weight: Some("2000".to_string()), // in Kg
//             drive_type: Some("Electric".to_string()),
//             displays: Some("Dual screen displays".to_string()), // e.g., infotainment + dashboard
//             seating: Some("5 Seater".to_string()),
//             cargo: Some("500".to_string()),            // in Liters
//             overall_height: Some("1600".to_string()),  // in mm
//             overall_width: Some("2000".to_string()),   // in mm
//             overall_length: Some("4800".to_string()),  // in mm
//             track_rear: Some("1600".to_string()),      // in mm
//             track_front: Some("1600".to_string()),     // in mm
//             ground_clearance: Some("150".to_string()), // in mm
//             key_features: Some(vec![
//                 "Autopilot".to_string(),
//                 "0-60 in 3.2s".to_string(),
//                 "Wireless charging".to_string(),
//                 "Heated seats".to_string(),
//                 "Cinematic Touchscreen Experience, Yoke Steering, Tri-zone Temperature Control, Tesla Arcade, Immersive Sound, High Impact Protection, Tesla Vision, Tesla Autopilot, Auto Lane Change, Summon, Autopark".to_string()
//             ]),
//             range_per_charge: Some("450".to_string()), // in KM
//             acceleration: Some("0-60 in 3.2s".to_string()),
//             charging_speed: Some("400".to_string()), // in Volts
//             wheels: Some("18\" Alloy".to_string()),  // size and type
//             purchase_price: Some(70000),             // in Euros or USD
//             brochure_url: Some("https://example.com/brochure.pdf".to_string()),
//             battery: Some("100".to_string()), // in KwH
//         }
//     }
// }

#[component]
pub fn SpecificationComponent(metadata: CollectionMetaData) -> impl IntoView {
    // Default values for optional fields
    let description = metadata.description;
let weight = metadata.weight;
let drive_type = metadata.drive_type;
let displays = metadata.displays;
let seating = metadata.seating;
let cargo = metadata.cargo;
let overall_height = metadata.overall_height;
let overall_width = metadata.overall_width;
let overall_length = metadata.overall_length;
let track_rear = metadata.track_rear;
let track_front = metadata.track_front;
let ground_clearance = metadata.ground_clearance;
let key_features = metadata.key_features;
let range_per_charge = metadata.range_per_charge;
let acceleration = metadata.acceleration;
let charging_speed = metadata.charging_speed;
let wheels = metadata.wheels;
let purchase_price = format!("€ {}", metadata.purchase_price.to_string());
let brochure_url = metadata.brochure_url.clone();
let battery = metadata.battery;


    view! {
        <div class="flex flex-col gap-8 py-4">
            <div class="grid grid-cols-3 gap-4">
                // Purchase price
                <SpecDetail title="Purchase price" value=purchase_price.clone() />
                <SpecDetail title="Weight" value=format!("{} Kg", weight) />
                <SpecDetail title="Drive Type" value=drive_type />
                <SpecDetail title="Displays" value=displays />
                <SpecDetail title="Seating" value=seating />
                <SpecDetail title="Cargo" value=format!("{} L", cargo) />
                <SpecDetail title="Overall Height" value=format!("{} mm", overall_height) />
                <SpecDetail title="Overall Width" value=format!("{} mm", overall_width) />
                <SpecDetail title="Overall Length" value=format!("{} mm", overall_length) />
                <SpecDetail title="Ground Clearance" value=format!("{} mm", ground_clearance) />
                <SpecDetail title="Track Front" value=format!("{} mm", track_front) />
                <SpecDetail title="Track Rear" value=format!("{} mm", track_rear) />
                <SpecDetail title="Key Features" value=key_features.join(", ") />
                <SpecDetail title="Range Per Charge" value=format!("{} KM", range_per_charge) />
                <SpecDetail title="Acceleration" value=acceleration />
                <SpecDetail title="Charging Speed" value=format!("{} V", charging_speed) />
                <SpecDetail title="Wheels" value=wheels.to_string() />

                // Brochure URL
                <div class="flex flex-col">
                    <div class="font-bold">"Brochure URL"</div>
                    <div>
                        <a target="_blank" href=brochure_url class="font-light">
                            "Link"
                        </a>
                    </div>
                </div>

                <SpecDetail title="Battery" value=battery />
            </div>

            // Description Section
            <div class="flex flex-col">
                <div class="font-bold">"Description"</div>
                <div class="font-light">{description}</div>
            </div>
        </div>
    }
}

// Helper component for displaying each spec item
#[component]
fn SpecDetail(#[prop(into)] title: String, value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <div class="font-bold">{title}</div>
            <div class="font-light">{value}</div>
        </div>
    }
}
