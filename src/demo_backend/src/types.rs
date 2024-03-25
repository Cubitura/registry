use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{Storable, storable::Bound};
use std::{borrow::Cow};

const MAX_VALUE_SIZE: u32 = 1024;


/******************************************************/
//
//  GENERAL PURPOSE
//
/******************************************************/

#[derive(CandidType, Deserialize)]
pub struct CallStringResponse {
    pub result: String,
}

#[derive(CandidType, Deserialize)]
pub struct CallSubscriberResponse {
    pub result: Subscriber,
}

#[derive(CandidType, Deserialize)]
pub struct CallSubscribersResponse {
    pub result: Vec<Subscribers>,
}

/******************************************************/
//
//  STRUCTS
//
/******************************************************/


// SUBSCRIBER ///////////////////////////////////////////

#[derive(CandidType, Deserialize)]
pub struct Subscriber {
    pub id: String,
    pub canister_id: String,
    pub callback: String,
    pub name: String,
    pub description: String,
    pub topic: String,
    pub namespace: String,
    pub active: bool,
}

#[derive(CandidType, Deserialize)]
pub struct Subscribers {
    pub id: String,
    pub canister_id: String,
    pub callback: String,
    pub name: String,
    pub description: String,
    pub topic: String,
    pub namespace: String,
    pub active: bool,
}

#[derive(CandidType, Deserialize)]
pub struct Sub {
    pub callback: String,
    pub topic: String,
}

