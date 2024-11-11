use leptos::*;

#[derive(Clone, Debug)]
 struct TabsMetaProps {
     tabs: Vec<String>,
     selected: RwSignal<String>,
}

#[component]
pub fn Tabs(tabs: Vec<String>,selected: RwSignal<String>) -> impl IntoView {
    // Function to set the selected tab

    let tab_props = TabsMetaProps {
        tabs, selected
    };

    let tabs = tab_props.tabs.clone();

    view! { 
        <div class="pt-12 flex items-center justify-center gap-4">
            {
                 tabs.into_iter().map(|tab| {

                    let current_tab = tab.clone();
                    let selected_tab = tab.clone();
                   
                    let is_selected = move || tab_props.selected.get() == *current_tab.clone();
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
                            {selected_tab.clone()}
                        </button>
                    }
                } ).collect_view()
            }
        </div>
    }
}
