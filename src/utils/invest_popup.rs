use candid::{Nat, Principal};
use leptos::*;

use crate::canister::token::{BookTokensArg, CollectionMetaData};
use crate::stores::auth_client::login;
use crate::utils::input::InputComponent;
use crate::utils::plus_icon::PlusIcon;
use crate::{state::canisters::Canisters, stores::auth_client::get_current_user_principal};
use crate::utils::button::ButtonComponent;

// Adapted functions and state management
use crate::components::invest_info::from_e8s;
#[component]
pub fn InvestPopup(
     show: RwSignal<bool>,
     minter_can_id: String,
) -> impl IntoView {
    const TRANSFER_PRICE: u64 = 10_000;

    // Reactive signals for state management
    let nft_to_buy = create_rw_signal(1u64.to_string());
    let payment_info = create_rw_signal(PaymentInfo::default());
    // let poll_interval = create_rw_signal(None);
    let step = create_rw_signal(1);
    let payment_status = create_rw_signal("pending".to_string());
    let metadata = create_rw_signal(None);
    let token_balance = create_rw_signal(0);

    let minter_canister_id = minter_can_id.clone();

    // Derived signal for current investment
    let current_investment = move || {
        metadata().as_ref().map_or(0.0, |m: &Option<CollectionMetaData>| {
            from_e8s(m.as_ref().unwrap().price.0.to_string().parse::<u64>().unwrap()) * token_balance() as f64
        })
    };

    let minter_canister_id_to_check_payment_status = minter_can_id.clone();

    // // Function to check payment status
    // let check_payment_status = move || async move {
    //     let cansisters: Canisters = expect_context();
    //     let actor = cansisters.token_canister(Principal::from_text(minter_canister_id_to_check_payment_status.clone()).unwrap()).await;
    //     if let Ok(res) = actor.book_tokens( BookTokensArg{ quantity: Nat::from( nft_to_buy.get().parse::<u64>().unwrap())}).await {
    //         logging::log!("{:?}", res);
    //             payment_status.set("completed".to_string());
    //     }
    // };

    // Function to start polling
    // let start_poll =   move  ||   {
    //     let interval = set_interval_with_handle( move ||{ let value = check_payment_status.clone();
    //     async move  { value().await}; }, 10_000);
    //     poll_interval.set(Some(interval));
    // };  

    let create_payment_action = create_action( move|()|{
        let can_id = minter_canister_id_to_check_payment_status.clone();
        let buy = nft_to_buy.get().parse::<u64>().unwrap().clone();
         async move {
        check_payment_status(can_id, buy, payment_status.clone() ).await
    }});

    create_payment_action.pending();


    // Function to get payment info
    let get_payment_info = move |_|{
    let minter_canister_id_clone = minter_canister_id.clone();
        
         async move {
        let cansisters: Canisters = expect_context();
        let agent  = Canisters::agent();
        let actor = cansisters.token_canister(Principal::from_text(minter_canister_id_clone.clone()).unwrap(), &agent).await;
        if let Ok(transfer_to_account) = actor.get_escrow_account().await {
            let metadata_data = actor.get_metadata().await.ok();
            let principal = get_current_user_principal();
            let current_investment_data = actor.get_booked_tokens(principal).await;
            let token_count = current_investment_data.unwrap().0.to_string().parse::<u64>().unwrap() as u64;

            payment_info.set(PaymentInfo {
                loaded: true,
                transfer_to: transfer_to_account.accountId,
                nft_price: metadata_data.clone().unwrap().price.0.to_string().parse::<u64>().unwrap() as u64,
            });
            metadata.set(Some(metadata_data.clone()));
            token_balance.set(token_count);
        }
    }};

    let get_payment_info_resource = create_resource(||(),get_payment_info );


    let transfer_price_e8s = 10_000; // Example transfer price in e8s
    let amount = move || nft_to_buy.get().parse::<f64>().unwrap_or_default() * from_e8s(payment_info().nft_price) + from_e8s(transfer_price_e8s); 


    // Derived view based on step
    let main_content = move || {
        match step.get() {
            1 => view! {
                <div>
                    <form
                        on:submit=move |ev| {
                            ev.prevent_default();
                            step.set(2);
                            create_payment_action.dispatch(());
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
                                min=1.0.into()
                                label="Number of NFTs to buy".to_string()
                                input_type="number".to_string()
                                placeholder="(in USD)".to_string()
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
                            classes="bg-white ring-1 ring-inset ring-gray-100 hover:bg-gray-50 outline-none active:bg-gray-200".into()
                            disabled=(move || !payment_info().loaded)()
                            submit=true
                            on_click=|_|{}
                        >
                            Proceed to Pay
                        </ButtonComponent>
                    </form>
                </div>
            },
            2 => view! {
                <div>
                    <StepTwo amount=amount() payment_info copy_handler=|_| {} />
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
                        when=move || get_current_user_principal().is_some()
                        fallback=|| 
                            view! {
                                <div class="flex flex-col gap-8 items-center">
                                    <div>You need to login before you can invest</div>
                                    <ButtonComponent /* href="/login".into() */ on_click=|_|{
                                        // Remove once the login route has been added:
                                        let _ = login();
                                    }>
                                        {|| view! { <div>Invest</div> }}
                                    </ButtonComponent>
                                </div>
                            }
                        
                    >
                        {main_content}
                    </Show>
                </div>
            </div>
        </Suspense>
    }
}



// Struct for PaymentInfo
#[derive(Default, Clone)]
struct PaymentInfo {
    loaded: bool,
    transfer_to: String,
    nft_price: u64,
}
async fn check_payment_status(token_can_id: String, nft_to_buy: u64, payment_status: RwSignal<String> ) {
    let cansisters: Canisters = expect_context();
    let agent = Canisters::agent();
        let actor = cansisters.token_canister(Principal::from_text(token_can_id.clone()).unwrap(), &agent).await;
        if let Ok(res) = actor.book_tokens( BookTokensArg{ quantity: Nat::from( nft_to_buy)}).await {
            logging::log!("Payment status {:?}", res);
                payment_status.set("completed".to_string());
        }
}

#[component]
fn StepTwo(
    amount: f64,
    payment_info: RwSignal<PaymentInfo>,
    copy_handler: impl Fn(String) + 'static,
) -> impl IntoView {
    // Calculate amount to pay based on the provided formula
   

    view! {
        <div class="flex w-full items-start justify-between text-sm gap-4">
            <div>"Amount to pay:"</div>
            <div class="flex items-center gap-2 justify-end">
                <div class="font-bold whitespace-nowrap text-xs break-all text-right">
                    <span class="select-all">{amount.to_string()}</span>
                    <span class="opacity-50">" ICP"</span>
                </div>
            // <button
            // on:click=move |_|( copy_handler.clone())(amount().to_string())
            // class="w-3 h-3"
            // >
            // // CopyIcon SVG or component here
            // </button>
            </div>
        </div>

        <div class="flex w-full items-start justify-between text-sm gap-4">
            <div class="text-nowrap">"Transferring to:"</div>
            <div class="flex items-center gap-2 justify-end">
                <div class="font-bold text-xs break-all select-all text-right w-1/2">
                    {payment_info.get().transfer_to.clone()}
                </div>
                <button
                    on:click=move |_| copy_handler(payment_info.get().transfer_to.clone())
                    class="w-3 h-3"
                >// CopyIcon SVG or component here
                </button>
            </div>
        </div>

        <hr />

        <div class="h-4 w-4 animate-spin">// PlusIcon SVG or component here
        </div>

        <div class="text-center text-sm">
            <span>"Waiting for payment"</span>
            <button on:click=move |_| {} class="underline text-xs font-bold">
                "Check now"
            </button>
        </div>
    }
}