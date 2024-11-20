use leptos::*;

// Subcomponent for Basic Info Tab
#[component]
pub fn BasicInfo(
    name: RwSignal<String>,
    treasury: RwSignal<String>,
    price: RwSignal<f64>,
    supply_cap: RwSignal<u32>,
    symbol: RwSignal<String>,
    description: RwSignal<String>,
) -> impl IntoView {
    // Access the loading state from context
    let loading =
        use_context::<ReadSignal<bool>>().unwrap_or_else(|| create_rw_signal(false).read_only());
    view! {
        <div class="flex flex-col gap-4">
            <label class="w-full">
                Title
                <input
                    type="text"
                    placeholder="Enter a title"
                    disabled=move || loading.get()
                    value=name.get()
                    on:input=move |e| name.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                Treasury Principal ID
                <input
                    type="text"
                    placeholder="Enter a valid principal id"
                    disabled=move || loading.get()
                    value=treasury.get()
                    on:input=move |e| treasury.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                Price (in ICP)
                <input
                    type="number"
                    disabled=move || loading.get()
                    value=price.get()
                    on:input=move |e| price.set(event_target_value(&e).parse().unwrap_or(0.0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                Supply Cap (number of NFTs)
                <input
                    type="number"
                    disabled=move || loading.get()
                    value=supply_cap.get()
                    on:input=move |e| supply_cap.set(event_target_value(&e).parse().unwrap_or(0))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                />
            </label>

            <label class="w-full">
                Symbol
                <select
                    disabled=move || loading.get()
                    value=symbol.get()
                    on:change=move |e| symbol.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                >
                    <option value="ICP">ICP</option>
                </select>
            </label>

            <label class="w-full">
                Description
                <textarea
                    placeholder="Enter a description"
                    disabled=move || loading.get()
                    value=description.get()
                    on:input=move |e| description.set(event_target_value(&e))
                    class="mt-1 block w-full border border-gray-300 rounded-md p-2"
                ></textarea>
            </label>
        </div>
    }
}
