# Rust Agent
The agent is a subscriber-side wrapper for the functionality of the service bus, and is used for easy setup of the subscription to topics. This agent is for Rust developers, there will also be a Motoko agent (release TBD).

## Functions  

### Subscribe to topic
```
pub async fn subscribe(topic_name: String, callback: String)
```
Register topic subscription. Messages will be delivered to the callback function.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*topic_name*: The name (string) of the topic to subscribe to <br/>
&nbsp;&nbsp;&nbsp;&nbsp;*callback*: The function the messages will be delivered to <br/>
      
**Return**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;Result&lt;String, String&gt;<br/><br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Ok*: Subscription ID<br/>
&nbsp;&nbsp;&nbsp;&nbsp;- *Err*: "Could not register the subscriber"<br/><br/>

### Unsubscribe from topic
```
async fn unsubscribe(subscription_id: String)
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
async fn subscription(subscription_id: String) 
```
Get the subscription with a specific ID.

**Parameters**<br/>
&nbsp;&nbsp;&nbsp;&nbsp;*subscription_id*: The ID string of the subscription to get 

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
pub async fn subscriptions() 
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

