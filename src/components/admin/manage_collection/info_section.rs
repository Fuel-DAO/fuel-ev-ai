// src/pages/admin/components/info_section.rs

use leptos::*;

#[component]
pub fn InfoSection(children: Children) -> impl IntoView {
    view! {
        <div class="pt-6">
            <div class="mt-6 border-t border-gray-100">
                // iter childrens
                <div class="divide-y divide-gray-100">{children()}</div>
            </div>
        </div>
    }
}
