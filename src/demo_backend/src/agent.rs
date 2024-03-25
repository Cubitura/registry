use ic_cdk::api::call::CallResult;
use candid::{CandidType, Principal};

use crate::types::{
    CallStringResponse, CallSubscribersResponse, 
    CallSubscriberResponse, Subscribers, Subscriber
};

const PUBLISHER_ID: &str = "be2us-64aaa-aaaaa-qaabq-cai";


#[ic_cdk_macros::update]
pub async fn subscribe(topic: String, callback: String) -> Result<String, String> {

    let canister: Principal = Principal::from_text(PUBLISHER_ID).unwrap();

    let _call_result: (CallStringResponse, ) = match ic_cdk::call(canister, "agent_subscribe", (topic, callback)).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return Err(err.1);
        }
    };

    Ok(_call_result.0.result)
}

#[ic_cdk_macros::update]
pub async fn unsubscribe(subscription_id: String) -> Result<String, String> {

    let canister: Principal = Principal::from_text(PUBLISHER_ID).unwrap();

    let _call_result: (CallStringResponse, ) = match ic_cdk::call(canister, "agent_unsubscribe", (subscription_id, )).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return Err(err.1);
        }
    };

    Ok(_call_result.0.result)
}

#[ic_cdk_macros::update]
pub async fn subscription(subscription_id: String) -> Subscriber {

    let canister: Principal = Principal::from_text(PUBLISHER_ID).unwrap();

    let _call_result: (CallSubscriberResponse, ) = match ic_cdk::call(canister, "agent_subscriber", (subscription_id, )).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return Subscriber {
                id: "".to_string(),
                name: "".to_string(),
                description: "".to_string(),
                callback: "".to_string(),
                canister_id: "".to_string(),
                topic: "".to_string(),
                namespace: "".to_string(),
                active: true,
            };
        }
    };

    _call_result.0.result
}

#[ic_cdk_macros::update]
pub async fn subscriptions() -> Vec<Subscribers> {

    let canister: Principal = Principal::from_text(PUBLISHER_ID).unwrap();

    let _call_result: (CallSubscribersResponse, ) = match ic_cdk::call(canister, "agent_subscriptions", ()).await {
        Ok(result) => result,
        Err(err) => {
            ic_cdk::println!("Error invoking raw_rand: {:?} {}", err.0, err.1);
            return Vec::<Subscribers>::new();
        }
    };

    _call_result.0.result
}

