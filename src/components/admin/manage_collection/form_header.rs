use leptos::*;

/// Props for the FormHeader component
#[component]
pub fn FormHeader(title: String, subtitle: String) -> impl IntoView {
    view! {
        <div class="px-4 sm:px-0 flex justify-between items-center">
            <div>
                <h3 class="text-base font-semibold leading-7 text-gray-900">{title}</h3>
                <p class="mt-1 max-w-2xl text-sm leading-6 text-gray-500">{subtitle}</p>
            </div>
        </div>
    }
}
