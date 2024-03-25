use candid::{CandidType, Principal};
use ic_cdk::print;

use types::{
    Subscriber, Subscribers,
};

use agent::{
    subscribe,
    subscriptions,
    subscription,
    unsubscribe
};

mod types;
mod utils;
mod agent;


/******************************************************/
//
//  SUBSCRIPTION MANAGEMENT
//
/******************************************************/

#[ic_cdk_macros::update]
async fn set_subscription(topic_id: String, callback: String) -> Result<String, String> {

    let result = subscribe(topic_id, callback).await;

    if result.is_ok() {
        Ok(result?)
    } else {
        Err("Could not register subscriber".to_string())
    }
}

#[ic_cdk_macros::update]
async fn unset_subscription(subscription_id: String) -> Result<String, String> {
    let result = unsubscribe(subscription_id).await;

    if result.is_ok() {
        Ok(result?)
    } else {
        Err("Could not unregister subscriber".to_string())
    }
}

#[ic_cdk_macros::update]
async fn get_subscriptions() -> Vec<Subscribers> {
    subscriptions().await
}

#[ic_cdk_macros::update]
async fn get_subscription(subscription_id: String) -> Subscriber {
    subscription(subscription_id).await
}



/******************************************************/
//
//  MISC
//
/******************************************************/

#[ic_cdk_macros::query]
fn hello(name: String) -> String {
    name
}