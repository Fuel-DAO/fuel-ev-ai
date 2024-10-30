use leptos::*;

#[component]
pub fn InfoTitle( #[prop(into)] title: String, #[prop(into)] classes: String) -> impl IntoView {
    view! { 
        <div class="flex items-center gap-1">
            <div class=format!("{} text-lg w-fit", classes)>
                {title}
            </div>
        </div>
    }
}
