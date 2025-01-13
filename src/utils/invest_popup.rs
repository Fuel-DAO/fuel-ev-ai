use candid::Principal;
use leptos::*;
use crate::canister::token::BookTokensArg;
use crate::state::auth_actions::create_login_action;
use crate::state::{auth::AuthService, canisters::Canisters};
use crate::utils::button::ButtonComponent;
use crate::utils::go_back_and_come_back::go_back_and_come_back;
use crate::utils::input::InputComponent;
use crate::utils::plus_icon::PlusIcon;
use crate::utils::web::copy_to_clipboard;
use std::cell::RefCell;
use std::rc::Rc;
use crate::components::invest_info::from_e8s;

// Struct for PaymentStatus
#[derive(Default, Clone)]
struct PaymentStatus {
    is_loading: RwSignal<bool>,
    status: RwSignal<String>,
    error: RwSignal<String>,
}

// Struct for PaymentInfo
#[derive(Default, Clone)]
struct PaymentInfo {
    loaded: bool,
    transfer_to: String,
    nft_price: u64,
}

#[component]
pub fn InvestPopup(show: RwSignal<bool>, minter_can_id: String, asset_can_id: String) -> impl IntoView {
    const TRANSFER_PRICE: u64 = 10_000;

    let nft_to_buy = create_rw_signal(1u64.to_string());

    let show_clone = RwSignal::new(show.get_untracked());

    // Retrieve Canisters from context
    let canisters_signal = use_context::<RwSignal<Option<Rc<Canisters>>>>()
        .expect("Canisters ReadWriteSignal must be provided");

    // Retrieve AuthService from context
    let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    let handle_login = create_login_action(Rc::clone(&auth_service));


        let is_authenticated = RwSignal::new(auth_service.borrow().is_authenticated());
    // Reactive memo for authentication state
    create_effect(move |_| {
        let is_show = show.get();
        show_clone.set(is_show);

    is_authenticated.set(auth_service.borrow().is_authenticated());
    });

    // Reactive memo for principal
    let principall = create_memo({
        let auth_service =
        use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");
        let auth_service = Rc::clone(&auth_service);
        move |_| {
            if is_authenticated() {
                auth_service.borrow().get_principal().ok()
            } else {
                None
            }
        }
    });

    let payment_info = create_rw_signal(PaymentInfo::default());
    let step = create_rw_signal(1);
    let payment_details = create_rw_signal(PaymentStatus::default());

    let metadata = create_rw_signal(None);
    let token_balance = create_rw_signal(0);

    let minter_canister_id = minter_can_id.clone();

    let current_investment = move || {
        metadata()
            .as_ref()
            .map_or(0.0, |m:&Option<crate::canister::token::GetMetadataRet>| {
                from_e8s(
                    m.as_ref()
                        .unwrap()
                        .price
                        .to_string()
                        .parse::<u64>()
                        .unwrap(),
                ) as f64
                    * token_balance() as f64
            })
    };

    let minter_canister_id_to_check_payment_status = minter_can_id.clone();

    // Function to handle payment status check action
    let check_payment_status_action = move || {
        let canisters_signal = canisters_signal.clone();
        let minter_canister_id_clone = minter_canister_id_to_check_payment_status.clone();
        let buy = nft_to_buy.get().parse::<u64>().unwrap_or_default();
        let payment_status = payment_details.get().status.clone();
        let payment_error = payment_details.get().error.clone();

        async move {
            // Retrieve Canisters from context
            if let Some(canisters_rc) = canisters_signal.get() {
                // Check if user is authenticated and get principal
                if let Some(_user_principal) = principall() {
                    payment_details.get().is_loading.set(true);

                    // Call the `check_payment_status` function
                    check_payment_status(
                        &*canisters_rc, // Dereference Rc to &Canisters
                        minter_canister_id_clone.clone(),
                        buy,
                        payment_status.clone(),
                        payment_error.clone(),
                    )
                    .await;

                    payment_details.get().is_loading.set(false);
                } else {
                    logging::log!("User is not authenticated.");
                }
            } else {
                logging::log!("Canisters instance is not available in the context.");
            }
        }
    };

    // Create payment action
    let create_payment_action = create_action(move |()| check_payment_status_action());

    // Function to get payment info
    let get_payment_info = move |_| {
        let minter_canister_id_clone = minter_canister_id.clone();

        async move {
            // Get Canisters from context
            if let Some(canisters_rc) = canisters_signal.get() {
                if let Some(principal) = principall() {
                    let token_canister = canisters_rc
                        .token_canister(
                            Principal::from_text(minter_canister_id_clone.clone()).unwrap(),
                        )
                        .await;

                    // Use `token_canister` directly
                    if let Ok(transfer_to_account) = token_canister.get_escrow_account().await {
                        let metadata_data = match token_canister.get_metadata().await.ok() {
    Some(res) => match res {
    crate::canister::token::Result4::Ok(get_metadata_ret) => Some(get_metadata_ret),
    crate::canister::token::Result4::Err(_) => None,
},
    None => None,
};
                        let current_investment_data =
                            token_canister.get_booked_tokens(Some(principal)).await;

                        if let (Ok(current_investment), crate::canister::token::Result2::Ok(transfer_to_account) ) = (current_investment_data, transfer_to_account) {
                            let token_count = current_investment
                                .0
                                .to_string()
                                .parse::<u64>()
                                .unwrap_or_default();

                            payment_info.set(PaymentInfo {
                                loaded: true,
                                transfer_to:  transfer_to_account.account_id,
                                nft_price: metadata_data
                                    .as_ref()
                                    .map(|metadata| {
                                        metadata
                                            .price
                                            .to_string()
                                            .parse::<u64>()
                                            .unwrap_or_default()
                                    })
                                    .unwrap_or_default(),
                            });

                            metadata.set(Some(metadata_data.clone()));
                            token_balance.set(token_count);
                        } else {
                            logging::log!("Failed to get booked tokens.");
                        }
                    } else {
                        logging::log!("Failed to get escrow account.");
                    }
                } else {
                    logging::log!("User is not authenticated.");
                }
            } else {
                logging::log!("Canisters instance is not available in the context.");
            }
        }
    };
    let get_payment_info_resource = create_resource(|| (), get_payment_info);

    let transfer_price_e8s = 10_000; // Example transfer price in e8s
    let amount = move || {
        nft_to_buy.get().parse::<u64>().unwrap_or_default() as f64
            * from_e8s(payment_info().nft_price)
            + from_e8s(transfer_price_e8s)
    };



    
    // Derived view based on step
    let main_content = move || {
        let show = show.clone();
        match step.get() {
            1 => view! {
                <div class="w-full">
                    <form
                        on:submit=move |ev| {
                            ev.prevent_default();
                            step.set(2);
                        }
                        class="flex w-full flex-col items-center gap-12 relative"
                    >

                        <div class=format!(
                            "w-full gap-8 flex flex-col transition-opacity {}",
                            if payment_info().loaded { "opacity-100" } else { "opacity-0" },
                        )>
                            // NFT Price display
                            <div class="flex w-full items-center justify-between text-sm gap-4">
                                <div>"NFT Price:"</div>
                                <div class="font-bold">
                                    {format!("{} ICP", from_e8s(payment_info().nft_price))}
                                </div>
                            </div>
                            // Current investment display
                            <div class="flex w-full items-center justify-between text-sm gap-4">
                                <div>"Current investment:"</div>
                                <div class="font-bold">
                                    {format!("{} ICP", current_investment())}
                                </div>
                            </div>
                            // Input field for the number of NFTs to buy
                            <InputComponent
                                value=nft_to_buy
                                required=true
                                min=0.0
                                label="Number of NFTs to buy".to_string()
                                input_type="number".to_string()
                                placeholder="(in ICP)".to_string()
                            />
                            <hr />
                            // Amount to pay display
                            <div class="flex w-full items-center justify-between text-sm gap-4">
                                <div>"Amount to pay:"</div>
                                <div class="font-bold">{format!("{} ICP", amount())}</div>
                            </div>
                        </div>
                        // Submit button for proceeding to payment
                        <ButtonComponent
                            disabled=(move || !payment_info().loaded)()
                            submit=true
                            on_click=|_| {}
                        >
                            "Proceed to Pay"
                        </ButtonComponent>
                    </form>
                </div>
            },
            2 => view! {
                <div>
                    <Show
                        when=move || payment_details.get().status.get() != "completed"
                        fallback=move || {
                            view! {
                                <div class="flex flex-col items-center justify-center gap-4 h-full w-full">
                                    <div class="flex w-full items-start justify-between text-sm gap-4">
                                        <div>"Amount for NFT:"</div>
                                        <div class="font-bold text-xs w-1/2 break-all text-right">
                                            {format!("{} ICP", amount())}
                                        </div>
                                    </div>
                                    <div class="bg-green-100 h-24 w-24 rounded-full text-xl flex items-center justify-center text-white">
                                        "✅"
                                    </div>
                                    <div class="font-bold py-4 text-2xl">
                                        "Transaction successful"
                                    </div>
                                    <ButtonComponent on_click=move |_| {
                                        show.set(false);
                                        go_back_and_come_back();
                                    }>"Close"</ButtonComponent>
                                </div>
                            }
                        }
                    >
                        <StepTwo
                            amount=amount()
                            payment_info=payment_info
                            payment_status=payment_details
                            on_click=move || {
                                create_payment_action.dispatch(());
                            }
                        />
                    </Show>
                </div>
            },
            _ => view! { <div>"Invalid step"</div> },
        }
    };

    view! {
        <Suspense>
            {get_payment_info_resource.get()}
            <div class="fixed inset-0 flex items-center justify-center h-full w-full z-[100]">
                <div class="absolute inset-0 bg-black/50 z-[1]"></div>
                <div class="bg-white text-black z-[2] max-w-2xl w-full px-16 py-12 flex flex-col items-center gap-12 relative shadow-xl rounded-lg">
                    <button on:click=move |_| show.set(false) class="absolute top-4 right-4 z-[2]">
                        <PlusIcon class="h-5 w-5 rotate-45".into() />
                    </button>
                    <div class="text-3xl">
                        {move || if step.get() == 3 { "Pay" } else { "Invest" }}
                    </div>
                    <Show
                        when=move || is_authenticated()
                        fallback=||view! {<LoginStep />}
                    >
                        {main_content}
                    </Show>
                </div>
            </div>
        </Suspense>
    }
}


#[component] 
fn LoginStep() -> impl IntoView {
    let auth_service =
    use_context::<Rc<RefCell<AuthService>>>().expect("AuthService context must be provided");

    let handle_login = create_login_action(Rc::clone(&auth_service));
    view! {
        <div class="flex flex-col gap-8 items-center">
            <div>"You need to login before you can invest"</div>
            <button on:click= move|_| {
                handle_login.dispatch(());
            } class="bg-green-500 hover:bg-green-700 text-white font-bold xl:text-2xl xl:px-8 xl:py-3 px-6 py-3 rounded-full shadow-lg">
                        "Click to Login"
            </button>
        </div>
    }
}
// fn go_back_and_come_back() {
//     if let Some(win) = web_sys::window() {

//         let current_url = window().location().href().ok();
//         if current_url.is_some() {

//             let current_loc = current_url.unwrap();

//         let navigator = use_navigate();

//         // Go back to the previous page
//         win.history().unwrap().back().unwrap();

//         // Use setTimeout to navigate back to the current location after a short delay
//         let closure = Closure::wrap(Box::new(move || {
//             navigator(&current_loc, Default::default());
//         }) as Box<dyn Fn()>);

//         win.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000).unwrap();
//         closure.forget(); 

//         } 
    
//         // Prevent the closure from being dropped immediately
//     }
// }
 
// Updated check_payment_status function
async fn check_payment_status(
    canisters: &Canisters,
    token_can_id: String,
    nft_to_buy: u64,
    payment_status: RwSignal<String>,
    payment_error: RwSignal<String>,
) {
    // Retrieve the token canister actor
    let actor = canisters
        .token_canister(Principal::from_text(token_can_id.clone()).unwrap(),)
        .await;

    // Use `actor` directly
    if let Ok(res) = actor
        .book_tokens(BookTokensArg {
            quantity: nft_to_buy as u32,
        })
        .await
    {
        logging::log!("{:?}", res);
        match res {
            crate::canister::token::Result_::Ok(_) => {
                payment_status.set("completed".to_string());
            }
            crate::canister::token::Result_::Err(error) => {
                payment_error.set(error);
            }
           
        }
    } else {
        payment_error.set("Failed to book tokens.".to_string());
    }
}

#[component]
fn StepTwo(
    amount: f64,
    payment_info: RwSignal<PaymentInfo>,
    payment_status: RwSignal<PaymentStatus>,
    on_click: impl Fn() + 'static,
) -> impl IntoView {

    view! {

        <div class="flex flex-col gap-8">

        <div class="flex w-full items-start justify-between text-sm gap-4">
            <div class="text-nowrap">"Transferring to:"</div>
            <div class="flex items-center gap-2 justify-end">
                <div class="font-bold text-xs break-all select-all text-right w-1/2">
                    {payment_info.get().transfer_to.clone()}
                </div>
                <button
                    on:click=move |_| {
                        copy_to_clipboard(&payment_info.get().transfer_to.clone());
                    }
                    class="w-3 h-3"
                >
                    <img src="/public/icons/copy_to_clipboard.svg" alt="Copy to clipboard" />
                </button>
            </div>
        </div>

        <div class="flex w-full items-start justify-between text-sm gap-4">
            <div>"Amount to pay:"</div>
            <div class="flex items-center gap-2 justify-end">
                <div class="font-bold whitespace-nowrap text-xs break-all text-right">
                    <span class="select-all">{amount.to_string()}</span>
                    <span class="opacity-50">" ICP"</span>
                </div>
                <button
                    on:click=move |_| {
                        copy_to_clipboard(&amount.to_string());
                    }
                    class="w-3 h-3"
                >
                    <img src="/public/icons/copy_to_clipboard.svg" alt="Copy to clipboard" />
                </button>
            </div>
        </div>

        <hr />
        </div>

        <Show when=move || payment_status.get().is_loading.get()>
            <div class="text-center mt-2 p-4 animate-pulse">Loading...</div>
        </Show>

        <div class="text-red-500 mt-2 p-4 text-center">
            {move || payment_status.get().error.get()}
        </div>
       
        <div class="text-center text-sm">
            <span>"Waiting for payment "</span>
            <button
                on:click=move |_| {
                    payment_status.get().error.set("".into());
                    on_click();
                }
                class="underline text-xs font-bold"
            >
                " Check now "
            </button>
        </div>

        <div class="mt-2 p-4 text-center text-sm">
        <a
            href="https://nns.ic0.app/wallet/?u=qoctq-giaaa-aaaaa-aaaea-cai"
            target="_blank"
            class="underline text-xs font-bold"
        >
            "Click here to invest via NNS"
        </a>
    </div>
    }
}

