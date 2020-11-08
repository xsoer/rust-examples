use reqwest;
use serde::json;

fn main() {
    let url = "http://120.53.206.14/api/v1/channels/107124338137763840/messages"
    for i in 0..100 {
    let body = json!{
        "user_id": u,
        "content": format!("我是{},这是我发的第{i}条消息，请查收",user, i ),
    };
    

    
    let client = reqwest::Client::new();
        let res = client
        .post(url)
        .body("the exact body that is sent")
        .send()
        .await?;
    }
    

}
