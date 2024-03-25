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
pub struct ResultResponse {
    pub code: String,
    pub success: bool,
    pub description: String,
}

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

// NAMESPACE ///////////////////////////////////////////

#[derive(CandidType, Deserialize)]
pub struct Namespace {
    pub name: String,
    pub description: String,
    pub subscribers: Vec<String>,
    pub active: bool,
}

#[derive(CandidType, Deserialize)]
pub struct Namespaces {
    pub id: String,
    pub name: String,
    pub description: String,
    pub subscribers: Vec<String>,
    pub active: bool,
}

// TOPIC ///////////////////////////////////////////

#[derive(CandidType, Deserialize)]
pub struct Topic {
    pub name: String,
    pub description: String,
    pub namespaces: Vec<String>,
    pub active: bool,
}

#[derive(CandidType, Deserialize)]
pub struct Topics {
    pub id: String,
    pub name: String,
    pub description: String,
    pub namespaces: Vec<String>,
    pub active: bool,
}

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

#[derive(CandidType, Deserialize, Clone)]
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

/******************************************************/
//
//  STORABLES
//
/******************************************************/

// NAMESPACE ///////////////////////////////////////////

impl Storable for Namespaces {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

// TOPIC ///////////////////////////////////////////

impl Storable for Topic {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}

// SUBSCRIBER ///////////////////////////////////////////

impl Storable for Subscribers {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
