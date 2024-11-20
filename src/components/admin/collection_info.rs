use leptos::*;

#[component]
pub fn CollectionInfo(
    purchase_price: RwSignal<f64>,
    weight: RwSignal<f64>,
    drive_type: RwSignal<String>,
    displays: RwSignal<String>,
    seating: RwSignal<String>,
    cargo: RwSignal<f64>,
    overall_height: RwSignal<f64>,
    overall_width: RwSignal<f64>,
    overall_length: RwSignal<f64>,
    track_front: RwSignal<f64>,
    track_rear: RwSignal<f64>,
    ground_clearance: RwSignal<f64>,
    key_features: RwSignal<String>,
    range_per_charge: RwSignal<f64>,
    acceleration: RwSignal<String>,
    charging_speed: RwSignal<String>,
    wheels: RwSignal<u32>,
    brochure_url: RwSignal<String>,
    battery: RwSignal<String>,
) -> impl IntoView {
    // Access the loading state from context
    let loading =
        use_context::<ReadSignal<bool>>().unwrap_or_else(|| create_rw_signal(false).read_only());

    view! {
        <div class="flex flex-col gap-4">
            <label class="w-full">
                "Purchase price (EUR)"
                <input
                    type="number"
                    placeholder="Enter purchase price in EUR"
                    disabled=move || loading.get()
                    value=purchase_price.get()
                    on:input=move |e| {
                        purchase_price.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Weight (KGs)"
                <input
                    type="number"
                    placeholder="Enter weight in KGs"
                    disabled=move || loading.get()
                    value=weight.get()
                    on:input=move |e| weight.set(event_target_value(&e).parse().unwrap_or(0.0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Drive Type"
                <input
                    type="text"
                    placeholder="Enter drive type"
                    disabled=move || loading.get()
                    value=drive_type.get()
                    on:input=move |e| drive_type.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Displays"
                <input
                    type="text"
                    placeholder="Enter displays information"
                    disabled=move || loading.get()
                    value=displays.get()
                    on:input=move |e| displays.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Seating"
                <input
                    type="text"
                    placeholder="Enter seating details"
                    disabled=move || loading.get()
                    value=seating.get()
                    on:input=move |e| seating.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Cargo (Litres)"
                <input
                    type="number"
                    placeholder="Enter cargo capacity in Litres"
                    disabled=move || loading.get()
                    value=cargo.get()
                    on:input=move |e| cargo.set(event_target_value(&e).parse().unwrap_or(0.0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Overall height (mm)"
                <input
                    type="number"
                    placeholder="Enter overall height in mm"
                    disabled=move || loading.get()
                    value=overall_height.get()
                    on:input=move |e| {
                        overall_height.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Overall width (mm)"
                <input
                    type="number"
                    placeholder="Enter overall width in mm"
                    disabled=move || loading.get()
                    value=overall_width.get()
                    on:input=move |e| {
                        overall_width.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Overall length (mm)"
                <input
                    type="number"
                    placeholder="Enter overall length in mm"
                    disabled=move || loading.get()
                    value=overall_length.get()
                    on:input=move |e| {
                        overall_length.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Track front (mm)"
                <input
                    type="number"
                    placeholder="Enter front track in mm"
                    disabled=move || loading.get()
                    value=track_front.get()
                    on:input=move |e| track_front.set(event_target_value(&e).parse().unwrap_or(0.0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Track Rear (mm)"
                <input
                    type="number"
                    placeholder="Enter rear track in mm"
                    disabled=move || loading.get()
                    value=track_rear.get()
                    on:input=move |e| track_rear.set(event_target_value(&e).parse().unwrap_or(0.0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Ground clearance (mm)"
                <input
                    type="number"
                    placeholder="Enter ground clearance in mm"
                    disabled=move || loading.get()
                    value=ground_clearance.get()
                    on:input=move |e| {
                        ground_clearance.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Key Features (separate by comma)"
                <input
                    type="text"
                    placeholder="e.g., Feature1, Feature2, Feature3"
                    disabled=move || loading.get()
                    value=key_features.get()
                    on:input=move |e| key_features.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Range per charge (KMs)"
                <input
                    type="number"
                    placeholder="Enter range per charge in KMs"
                    disabled=move || loading.get()
                    value=range_per_charge.get()
                    on:input=move |e| {
                        range_per_charge.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Acceleration"
                <input
                    type="text"
                    placeholder="Enter acceleration details"
                    disabled=move || loading.get()
                    value=acceleration.get()
                    on:input=move |e| acceleration.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Charging Speed (V)"
                <input
                    type="text"
                    placeholder="Enter charging speed in volts"
                    disabled=move || loading.get()
                    value=charging_speed.get()
                    on:input=move |e| charging_speed.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Wheels"
                <input
                    type="number"
                    placeholder="Enter number of wheels"
                    disabled=move || loading.get()
                    value=wheels.get()
                    on:input=move |e| wheels.set(event_target_value(&e).parse().unwrap_or(0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Brochure URL"
                <input
                    type="text"
                    placeholder="Enter brochure URL"
                    disabled=move || loading.get()
                    value=brochure_url.get()
                    on:input=move |e| brochure_url.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                "Battery (kWh)"
                <input
                    type="text"
                    placeholder="Enter battery capacity in kWh"
                    disabled=move || loading.get()
                    value=battery.get()
                    on:input=move |e| battery.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>
        </div>
    }
}
