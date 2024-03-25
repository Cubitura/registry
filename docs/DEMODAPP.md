# Demo Dapp
The demo dapp implements the functionality of the service bus, and is used for both testing and demonstration of how to implement the agent. This demo dapp is for Rust developers, there will also be a Motoko demo dapp (release TBD).

## Backend Functions
The backend is very simple, the main purpose of the functions is to implement the agent. All communication with the service bus system goes through the agent, so the demo dapp doesn't need to know about .

### Add subscription  
```
async fn set_subscription(topic_name: String, callback: String)
```
Register topic subscription. Messages will be delivered to the callback function.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*topic_name*: The name (string) of the topic to subscribe to <br/>
&nbsp;&nbsp;&nbsp;&nbsp;*callback*: The function the messages will be delivered to <br/>
      
**Return**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;Result&lt;String, String&gt;<br/><br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Ok*: Subscription ID<br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Err*: "Could not register the subscriber"<br/><br/>

### Remove subscription
```
async fn unset_subscription(subscription_id: String)
```
Unregister topic subscription.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*subscription_id*: The ID string of the subscription to remove 

**Return**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;Result&lt;String, String&gt;<br/><br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Ok*: The removed subscription ID<br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Err*: "Could not unregister the subscriber"<br/><br/>

### Get subscription details
```
async fn get_subscription(subscription_id: String) 
```
Get the subscription with a specific ID.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;(none) 

**Return**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;Subscriber<br/><br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *record {*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*id: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*canister_id: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*callback: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*name: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*description: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*topic: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*namespace: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*active: bool;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*};*<br/><br/>

### Get all subscriptions
```
async fn get_subscriptions() -> Vec<Subscribers> 
```
Get all subscriptions registered to this canister.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;(none) 

**Return**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;Vec&lt;Subscribers&gt;<br/><br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *vec {*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*record {*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*id: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*canister_id: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*callback: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*name: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*description: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*topic: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*namespace: text;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*active: bool;*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;*}*<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*};*<br/>


## Frontend
At the moment there's no frontend for the demo dapp, but the Candid UI is an easier and more user friendly way of test the backend functionality, without having to use the command line instructions. Check the actual Candid UI URL when deployed, but for a local deployment it will most likely be:

```
http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
```
