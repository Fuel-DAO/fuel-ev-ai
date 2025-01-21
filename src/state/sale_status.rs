
use std::collections::BTreeMap;

use candid::Principal;
use leptos::*;

use crate::{canister::token::{GetMetadataRet, SaleStatus}, outbound::collection_canister_calls::{CollectionData, CollectionId}};

#[derive(Default, Clone)]
pub struct SaleStatusState {
    pub sale_status: RwSignal<BTreeMap<Principal, SaleStatus>>, 
    pub metadata: RwSignal<BTreeMap<Principal, Option<GetMetadataRet>>>, 
}

impl SaleStatusState {
    pub fn set_global() {
        provide_context(Self::default());
    }

    pub fn get() -> Self {
        expect_context()
    }

    pub fn get_listing_status(canister: Principal) -> impl Fn() -> SaleStatus {
       move || Self::get().sale_status.get().get(&canister).unwrap_or(&SaleStatus::Rejected).clone()
    }

    pub fn set_listing_satatus(canister: Principal, status: SaleStatus) {
        Self::get().sale_status.update(|f|{ f.insert(canister, status);});
    }

    pub fn get_listing_metadata(canister: Principal) -> impl Fn() -> Option<GetMetadataRet> {
       move || Self::get().metadata.get().get(&canister).map(|f| f.clone()).flatten()
    }

    pub fn set_listing_metadata(canister: Principal, metadata: GetMetadataRet) {
        Self::get().metadata.update(|f|{ f.insert(canister, Some(metadata));});
    }

    pub fn collection_data() -> impl Fn() ->  Vec<CollectionData> {
        move ||  Self::get().metadata.get().iter().map(|f| (f.0.clone() ,f.1.clone(),)).filter(|f| f.1.is_some()).map(|f| (f.0, f.1.unwrap())).map(|f | CollectionData {
            id: CollectionId {
                asset_canister: f.1.asset_canister, 
                token_canister: f.0
            },
            name: f.1.clone().name,
            status: Self::get().sale_status.get().get(&f.0).unwrap_or(&SaleStatus::Rejected).humanize(),
            metadata: Some(f.1),
        }).collect()
    }
}

impl SaleStatus {
    pub fn humanize(&self) -> String {
        match self {
            SaleStatus::Live => "Open".into(),
                    _ => "Closed".into(),
        }
    }
}