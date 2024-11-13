use leptos::*;

// Subcomponent for Collection Info Tab
#[component]
pub fn CollectionInfo(
    purchase_price: RwSignal<f64>,
    weight: RwSignal<f64>,
    drive_type: RwSignal<String>,
    // ... other fields
) -> impl IntoView {
    // Access the loading state from context
let loading = use_context::<ReadSignal<bool>>()
        .unwrap_or_else(|| create_rw_signal(false).read_only());
    view! {
        <div class="flex flex-col gap-4">
            <label>
                "Purchase price (EUR)"
                <input
                    type="number"
                    disabled=move || loading.get()
                    value=purchase_price.get()
                    on:input=move |e| {
                        purchase_price.set(event_target_value(&e).parse().unwrap_or(0.0))
                    }
                />
            </label>
            <label>
                "Weight (KGs)"
                <input
                    type="number"
                    disabled=move || loading.get()
                    value=weight.get()
                    on:input=move |e| weight.set(event_target_value(&e).parse().unwrap_or(0.0))
                />
            </label>
            <label>
                "Drive Type"
                <input
                    type="text"
                    disabled=move || loading.get()
                    value=drive_type.get()
                    on:input=move |e| drive_type.set(event_target_value(&e))
                />
            </label>
        </div>
    }
}
