use leptos::*;
use leptos::prelude::*;
use crate::canister::token::CollectionMetaData;
use crate::state::canisters::Canisters;
use crate::outbound::accept_or_reject_sale::{accept_sale, reject_sale};
use crate::outbound::collection_canister_calls::get_total_booked_tokens;
use std::rc::Rc;
use candid::{Principal, Nat};
use leptos::logging::log;
use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

#[component]
pub fn ConcludeSaleAdminComponent(
    metadata: CollectionMetaData,
    token_canister_id: Principal,
) -> impl IntoView {
    // Reactive signals for loading states
    let loading_accept = create_rw_signal(false);
    let loading_reject = create_rw_signal(false);

    // Reactive signals for error and success messages
    let error_message_accept = create_rw_signal(String::new());
    let error_message_reject = create_rw_signal(String::new());
    let success_message_accept = create_rw_signal(String::new());
    let success_message_reject = create_rw_signal(String::new());

    // Reactive signal for confirmation dialog visibility
    let show_confirmation = create_rw_signal(false);
    let action_type = create_rw_signal(String::new()); // "accept" or "reject"

    // Access Canisters from context
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Fetch booked_tokens
    let booked_tokens_resource = create_resource(
        || (),
        move |_| {
            let token_canister_id = token_canister_id.clone();
            let canisters_signal = canisters_signal.clone();

            async move {
                if let Some(canisters) = canisters_signal.get() {
                    let booked_tokens = get_total_booked_tokens(&canisters, token_canister_id).await?;
                    Ok::<Nat, String>(booked_tokens)
                } else {
                    Err("Canisters instance not available.".to_string())
                }
            }
        },
    );

    // Function to handle showing the confirmation dialog
    let initiate_action = move |action: String| {
        action_type.set(action.clone());
        show_confirmation.set(true);
    };

    // Function to handle confirming the action
    let confirm_action = {
        let loading_accept = loading_accept.clone();
        let loading_reject = loading_reject.clone();
        let error_message_accept = error_message_accept.clone();
        let error_message_reject = error_message_reject.clone();
        let success_message_accept = success_message_accept.clone();
        let success_message_reject = success_message_reject.clone();
        let canisters_signal = canisters_signal.clone();
        let action_type = action_type.clone();
        let show_confirmation = show_confirmation.clone();
        let token_canister_id = token_canister_id.clone();

        move || {
            let action = action_type.get();
            show_confirmation.set(false);

            match action.as_str() {
                "accept" => {
                    loading_accept.set(true);
                    error_message_accept.set(String::new());
                    success_message_accept.set(String::new());

                    let canisters = match canisters_signal.get().as_ref() {
                        Some(cans) => cans.clone(),
                        None => {
                            error_message_accept
                                .set("Canisters not available. Please log in.".to_string());
                            loading_accept.set(false);
                            return;
                        }
                    };

                    spawn_local(async move {
                        match accept_sale(&canisters, token_canister_id.clone()).await {
                            Ok(_) => {
                                success_message_accept
                                    .set("Sale successfully accepted.".to_string());
                                log!("Sale successfully accepted.");
                            }
                            Err(e) => {
                                error_message_accept.set(format!("Error accepting sale: {}", e));
                                log!("Error accepting sale: {}", e);
                            }
                        }
                        loading_accept.set(false);
                    });
                }
                "reject" => {
                    loading_reject.set(true);
                    error_message_reject.set(String::new());
                    success_message_reject.set(String::new());

                    let canisters = match canisters_signal.get().as_ref() {
                        Some(cans) => cans.clone(),
                        None => {
                            error_message_reject
                                .set("Canisters not available. Please log in.".to_string());
                            loading_reject.set(false);
                            return;
                        }
                    };

                    spawn_local(async move {
                        match reject_sale(&canisters, token_canister_id.clone()).await {
                            Ok(_) => {
                                success_message_reject
                                    .set("Sale successfully rejected.".to_string());
                                log!("Sale successfully rejected.");
                            }
                            Err(e) => {
                                error_message_reject.set(format!("Error rejecting sale: {}", e));
                                log!("Error rejecting sale: {}", e);
                            }
                        }
                        loading_reject.set(false);
                    });
                }
                _ => {}
            }
        }
    };

    // Function to handle cancelling the confirmation dialog
    let cancel_action = move || {
        show_confirmation.set(false);
    };

    // The component's view
    view! {
        <Suspense fallback=move || {
            view! { <div>"Loading..."</div> }
        }>
            {booked_tokens_resource
                .read()
                .map(|booked_tokens_result| match booked_tokens_result {
                    Ok(booked_tokens) => {
                        let supply_cap_biguint: BigUint = metadata.supply_cap.clone().into();
                        let booked_tokens_biguint: BigUint = booked_tokens.clone().into();
                        let invested_percentage = move || {
                            if !supply_cap_biguint.is_zero() {
                                let percentage = (&booked_tokens_biguint * 100u64)
                                    / &supply_cap_biguint;
                                percentage.to_f64().unwrap_or(0.0)
                            } else {
                                0.0
                            }
                        };
                        view! {
                            // Convert 'booked_tokens' and 'metadata.supply_cap' to BigUint
                            // Convert Nat to BigUint

                            // Compute investedPercentage
                            // Convert to f64 for UI display

                            <div class="shrink-0 border border-primary bg-green-50 border-dashed rounded-xl flex flex-col text-black gap-3 p-6 shadow-xl h-fit">
                                <div class="font-bold text-xl">"Admin"</div>
                                <div class="text-lg font-medium">"Conclude sale"</div>
                                <Show when=move || invested_percentage() < 100.0 fallback=|| ()>
                                    <div class="text-xs text-red-600">
                                        "Warning: Sale has not been completed fully yet"
                                    </div>
                                </Show>

                                <div class="flex gap-2">
                                    // Accept Sale Button
                                    <button
                                        role="presentation"
                                        type="button"
                                        class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                        disabled=loading_accept.get()
                                        on:click=move |_| initiate_action("accept".to_string())
                                    >
                                        <div class=if loading_accept.get() {
                                            "opacity-0 transition-opacity"
                                        } else {
                                            ""
                                        }>"Accept sale"</div>
                                        {move || {
                                            if loading_accept.get() {
                                                view! {
                                                    <div class="absolute inset-0 flex items-center justify-center">
                                                        // You can add a loading spinner here if desired
                                                        <div class="loader"></div>
                                                    </div>
                                                }
                                            } else {
                                                view! { <div></div> }
                                            }
                                        }}
                                    </button>

                                    // Reject Sale Button
                                    <button
                                        role="presentation"
                                        type="button"
                                        class="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200 text-black ring-0 px-4 py-2 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                        disabled=loading_reject.get()
                                        on:click=move |_| initiate_action("reject".to_string())
                                    >
                                        <div class=if loading_reject.get() {
                                            "opacity-0 transition-opacity"
                                        } else {
                                            ""
                                        }>"Reject sale"</div>
                                        {move || {
                                            if loading_reject.get() {
                                                view! {
                                                    <div class="absolute inset-0 flex items-center justify-center">
                                                        // You can add a loading spinner here if desired
                                                        <div class="loader"></div>
                                                    </div>
                                                }
                                            } else {
                                                view! { <div></div> }
                                            }
                                        }}
                                    </button>
                                </div>

                                <Show when=show_confirmation.clone() fallback=|| ()>
                                    <div class="border border-yellow-500 bg-yellow-100 p-4 rounded-lg">
                                        <div class="text-sm font-bold">"Caution!"</div>
                                        <div class="text-xs">
                                            "You are about to "
                                            <span class="underline">{action_type.get().clone()}</span>
                                            " the sale." <br /> " This will stop and complete the sale."
                                        </div>
                                        <div class="flex gap-2 mt-2">
                                            // Cancel Button
                                            <button
                                                role="presentation"
                                                type="button"
                                                class="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200 text-black ring-0 px-4 py-2 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                                on:click=move |_| cancel_action()
                                            >
                                                "Cancel"
                                            </button>

                                            // Confirm Button
                                            <button
                                                role="presentation"
                                                type="button"
                                                class="bg-primary hover:bg-green-600 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 text-white focus-visible:outline-green-300 ring-0 px-4 py-2 text-gray-900 inline-flex relative items-center w-fit h-fit rounded-full transition-all text-sm font-semibold shadow-md active:translate-y-[1px] text-nowrap disabled:opacity-30"
                                                disabled=false
                                                on:click=move |_| confirm_action()
                                            >
                                                <div class=if (action_type.get() == "accept"
                                                    && loading_accept.get())
                                                    || (action_type.get() == "reject" && loading_reject.get())
                                                {
                                                    "opacity-0 transition-opacity"
                                                } else {
                                                    ""
                                                }>"Confirm"</div>
                                                {move || {
                                                    if (action_type.get() == "accept" && loading_accept.get())
                                                        || (action_type.get() == "reject" && loading_reject.get())
                                                    {
                                                        view! {
                                                            <div class="absolute inset-0 flex items-center justify-center">
                                                                // You can add a loading spinner here if desired
                                                                <div class="loader"></div>
                                                            </div>
                                                        }
                                                    } else {
                                                        view! { <div></div> }
                                                    }
                                                }}
                                            </button>
                                        </div>
                                    </div>
                                </Show>

                                <div>
                                    <Show
                                        when=move || {
                                            !success_message_accept.get().is_empty()
                                                || !success_message_reject.get().is_empty()
                                                || !error_message_accept.get().is_empty()
                                                || !error_message_reject.get().is_empty()
                                        }
                                        fallback=|| ()
                                    >
                                        <Show
                                            when=move || !success_message_accept.get().is_empty()
                                            fallback=|| ()
                                        >
                                            <div class="text-xs text-green-600 mt-2">
                                                {success_message_accept.get()}
                                            </div>
                                        </Show>
                                        <Show
                                            when=move || !success_message_reject.get().is_empty()
                                            fallback=|| ()
                                        >
                                            <div class="text-xs text-green-600 mt-2">
                                                {success_message_reject.get()}
                                            </div>
                                        </Show>
                                        <Show
                                            when=move || !error_message_accept.get().is_empty()
                                            fallback=|| ()
                                        >
                                            <div class="text-xs text-red-600 mt-2">
                                                {error_message_accept.get()}
                                            </div>
                                        </Show>
                                        <Show
                                            when=move || !error_message_reject.get().is_empty()
                                            fallback=|| ()
                                        >
                                            <div class="text-xs text-red-600 mt-2">
                                                {error_message_reject.get()}
                                            </div>
                                        </Show>
                                    </Show>
                                </div>
                            </div>
                        }
                    }
                    Err(e) => {
                        view! {
                            <div class="text-red-600">{"Error fetching booked tokens: "}{e}</div>
                        }
                    }
                })}
        </Suspense>
    }
}

