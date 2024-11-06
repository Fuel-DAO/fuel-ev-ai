use leptos::*;

#[component]
pub fn InputComponent(
    label: String,
    #[prop(optional)] value: RwSignal<String>,
    #[prop(optional)] placeholder: String,
    #[prop(optional)] input_type: String,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] required: bool,
   #[prop(optional)] min: Option<f64>,
   #[prop(optional)] max: Option<f64>,
   #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    // Classes for input styling
    let base_classes = "p-2 text-black block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-black sm:text-sm sm:leading-6";
    let combined_classes = if let Some(class) = class {
        format!("{} {}", base_classes, class)
    } else {
        base_classes.to_string()
    };

    let label_classes = format!(
        "flex flex-col gap-2 transition-opacity {}",
        if disabled { "pointer-events-none opacity-50" } else { "opacity-100" }
    );

    view! { 
        <label class=label_classes>
            <span class="text-sm font-medium leading-6 text-gray-900">
                {move || {
                    if required {
                        format!("{} *", label)
                    } else {
                        label.clone()
                    }
                }}
            </span>
            <input
                type=input_type
                prop:disabled=disabled
                prop:required=required
                min=min.map(|m| m.to_string())
                max=max.map(|m| m.to_string())
                class=combined_classes
                placeholder=placeholder
                prop:value={value().clone()}
                on:input=move |ev| {
                    let input_value = event_target_value(&ev);
                    value.set(input_value);
                }
            />
        </label>
    }
}
