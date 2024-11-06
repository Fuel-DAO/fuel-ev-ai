use candid::{Nat, Principal};
use leptos::*;
use serde::{Deserialize, Serialize};
use crate::canister::token::{CollectionMetaData, GetSaleStatusRet};
use crate::outbound::collection_canister_calls::{get_sale_status, get_total_booked_tokens};
use crate::utils::button::ButtonComponent;
use crate::utils::invest_popup::InvestPopup;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InvestInfoMetaProps {
    pub metadata: CollectionMetaData,
    pub status: GetSaleStatusRet,
    pub booked_tokens: Nat,
}


#[component]
pub fn InvestInfo( metadata: CollectionMetaData, token_canister_id: Principal  ) -> impl IntoView {

    let sale_and_tokens = create_resource(||(), move |_| {
        let metadata = metadata.clone();
        let token_canister_id = token_canister_id.clone();
        async move  {
            let status = get_sale_status(token_canister_id).await?;
            let booked_tokens = get_total_booked_tokens(token_canister_id).await?;
            Ok::<InvestInfoMetaProps, String>(InvestInfoMetaProps {
                metadata, 
                booked_tokens,
                status
            })
        }
    });

    view! {
        <Suspense> 
            {
                sale_and_tokens.get().map(|res| match res {
                    Ok(props) => view! {
                        <div>
                        <InvenstInfoInner props token_canister_id />
                        </div>
                    },
                    Err(e) => view! {
                        <div>{e}</div>
                    },
                })
            }
        </Suspense>
    }

}

#[component]
fn InvenstInfoInner(props: InvestInfoMetaProps, token_canister_id: Principal ) -> impl IntoView {

    let booked_tokens = props.booked_tokens.clone();
    let booked_tokens_value = booked_tokens.clone();

    let show_invest_popup = RwSignal::new(false);

    logging::log!("Booked tokens{:?} ", booked_tokens);


    

    
    let supply_cap = props.metadata.clone().supply_cap.clone();
    // Function to calculate invested percentage
    let invested_percentage =  move || if booked_tokens.clone() > Nat::from(0u32) {
        // Convert `Nat` to `u128` by parsing from string
        let booked_tokens_u128: u128 = booked_tokens.0.clone().to_string().parse().unwrap_or(0);
        let supply_cap_u128: u128 = supply_cap.0.to_string().parse().unwrap_or(1); // Avoid division by zero
    
        let booked_tokens_f64 = booked_tokens_u128 as f64;
        let supply_cap_f64 = supply_cap_u128 as f64;
    
        format!("{:.2}", (booked_tokens_f64 / supply_cap_f64) * 100.0)
    } else {
        "0".to_string()
    };
    
    

    let invested_percentage_clone = invested_percentage.clone();

    let metadata_price_value = props.metadata.clone().price.clone();
    // Function to calculate invested amount in ICP
    let invested_icp = move ||{

        let booked_tokens = booked_tokens_value.clone();
        let metadata_price = metadata_price_value.clone();

        // Convert `Nat` to `u128` by parsing from string
        let booked_tokens_u128: u128 = booked_tokens.0.to_string().parse().unwrap_or(0);
        let price_u128: u128 = metadata_price.0.to_string().parse().unwrap_or(0);
    
        let booked_tokens_f64 = booked_tokens_u128 as f64;
        let price_f64 = price_u128 as u64;
    
        format!("{:.4}", booked_tokens_f64 * from_e8s(price_f64))
    };
    

    view! { 
        <div class="shrink-0 bg-primary rounded-xl flex flex-col text-white gap-3 p-6 shadow-xl h-fit">
            <div class="font-bold text-5xl">
                {match props.status {
                    GetSaleStatusRet::Live => "Open",
                    _=> "Closed",
                }

                } 
            </div>

            <div class="text-2xl">
                "Invested " <span class="font-bold">{invested_icp()} " ICP"</span>
            </div>

            <div class="w-72 bg-black/20 h-4 rounded-full relative overflow-hidden">
                <div
                    style=move || format!("width: {}%;", invested_percentage().clone())
                    class="absolute transition-all bg-white rounded-full left-0 h-full"
                />
            </div>

            <div class="text-md font-light">
                "Funded " { invested_percentage_clone()} "% " {match props.status {
                    GetSaleStatusRet::Live => "till now",
                    _ => ""
                }
                    
                }
            </div>
            <ButtonComponent secondary=true disabled= &format!("{:?}", props.status) != &format!("{:?}", GetSaleStatusRet::Live) on_click=move|_| show_invest_popup.update(|f| *f = true)>    
            // {"Invest"}
            {|| view! {<div> Invest</div>}}
            </ButtonComponent>
            
            <Show when=move||show_invest_popup.get()>
                <InvestPopup show=show_invest_popup.clone() minter_can_id=token_canister_id.to_string() />
            </Show>


            // <button 
            //     class="btn-secondary"
            //     disabled=move || &format!("{:?}", props.status) != &format!("{:?}", GetSaleStatusRet::Live)
            //     on:click=move |_| {
            //         // Handle Invest button click logic here
            //     }
            // >
            //     "Invest"
            // </button>
        </div>
    }
}

// Helper function to convert e8s to ICP
pub(crate) fn from_e8s(e8s: u64) -> f64 {
    e8s as f64 / 1e8
}
