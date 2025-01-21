use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Tab {
    Specifications,
    Documents,
}

#[derive(Clone, Debug)]
struct TabsMetaProps {
    tabs: Vec<Tab>,
    selected: RwSignal<Tab>,
}

#[component]
pub fn Tabs(tabs: Vec<Tab>, selected: RwSignal<Tab>) -> impl IntoView {
    // Function to set the selected tab

    let tab_props = TabsMetaProps { tabs, selected };

    let tabs = tab_props.tabs.clone();

    view! {
        <div class="pt-12 flex items-center justify-center gap-4">
            {
                 tabs.into_iter().map(|tab| {

                    let current_tab = tab.clone();
                    let selected_tab = tab.clone();
                    let selected_str = format!("{selected_tab:?}");

                    let is_selected = move || tab_props.selected.get() == current_tab.clone();
                    view! {
                        <button
                            on:click=move |_| tab_props.selected.set(tab.clone())
                            class=move || format!(
                                "capitalize px-6 pb-2 whitespace-nowrap {}",
                                if is_selected() {
                                    "font-bold border-b-2 border-black"
                                } else {
                                    ""
                                }
                            )
                        >
                             {selected_str}
                        </button>
                    }
                } ).collect_view()
            }
        </div>
    }
}
