use uuid_by_string::generate_uuid::{generate_uuid};


/******************************************************/
//
//  UTILITY
//
/******************************************************/

#[ic_cdk_macros::query]
pub fn create_uuid() -> String {
    let ts = ic_cdk::api::time().to_string();
    //ic_cdk::print(&ts);
    generate_uuid(&ts)
}

pub fn get_variable_type<K>(_: &K) -> String {
    std::any::type_name::<K>().to_string()
}


#[ic_cdk_macros::query]
pub fn connection(id: String) -> String {
    let ts = ic_cdk::api::time().to_string();
    //ic_cdk::print(&ts);
    generate_uuid(&ts)
}


