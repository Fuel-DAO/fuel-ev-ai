use leptos::*;
use leptos_router::A;

use crate::utils::plus_icon::PlusIcon;

#[component]
pub fn ButtonComponent<N, EF>(
    #[prop(optional)] secondary: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] submit: bool,
    #[prop(optional)] href: Option<String>,
    #[prop(optional)] loading: bool,
    #[prop(optional)] icon_only: bool,
    #[prop(optional)] target: Option<String>,
    #[prop(optional)] classes: Option<String>,
    children: EF,
)  -> impl IntoView  
where 
N: IntoView + 'static,
EF: Fn() -> N + 'static + Clone,
{

let children = move || {
    (children)().into_view()
};

    //  let secondary = false;
	//  let disabled = false;
	//  let submit = false;
	//  let loading = false;
	//  let iconOnly = false;

    let button_classes = {
        let base_classes = if secondary {
            "bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200"
        } else {
            "bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0"
        };

        let size_classes = if icon_only { "p-1" } else { "px-4 py-2" };
        let custom_classes = classes.unwrap_or_default();

        format!(
            "{} {} text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full \
            transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap \
            disabled:opacity-30 {}",
            base_classes, size_classes, custom_classes
        )
    };

    view! {
        {if let Some(link) = href {
            view! {
                <div>
                <A
                    href=link
                    target=target.unwrap_or("_self".to_string())
                    class=button_classes
                    // role="presentation"
                    on:click=move |_| {}
                >
                    <div class=if loading {
                        "opacity-0 transition-opacity"
                    } else {
                        ""
                    }>{children}</div>
                    {move || {
                        if loading {
                            view! {
                                <div class="absolute inset-0 flex items-center justify-center">
                                <PlusIcon class="w-5 h-5 animate-spin".into()/>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }
                    }}
                </A>
                </div>
            }
        } else {
            view! {
                <div>
                <button
                    type=if submit { "submit" } else { "button" }
                    disabled=disabled
                    class=button_classes
                    on:click=move |_| {}
                >
                    <div class=if loading {
                        "opacity-0 transition-opacity"
                    } else {
                        ""
                    }>{children}</div>
                    {move || {
                        if loading {
                            view! {
                                <div class="absolute inset-0 flex items-center justify-center">
                                <PlusIcon class="w-5 h-5 animate-spin".into()/>
                                </div>
                            }
                        } else {
                            view! { <div></div> }
                        }
                    }}
                </button>
                </div>
            }
        }}
    }
}
