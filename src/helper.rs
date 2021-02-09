pub async fn post<T, D>(
    context: &Context,
    body: T,
    url: String
) -> Result<D, juniper::FieldError> where T: serde::Serialize, D: serde::de::DeserializeOwned {
    let base_url = &context.base;
    let final_url = base_url.to_owned() + &url;
    let res = context
        .client
        .post(&final_url)
        .header(
            "Cookie",
            "ajs_anonymous_id=%2286af43bb-5e00-4cd2-8255-2ccf92bafedd%22; WZRK_G=775da19b01524940b53caff859541c59; _ga=amp-H8eiH9g1yFARZU_jE3mq0w; G_ENABLED_IDPS=google; connect.sid=s%3AI5vgVKhdyELwu5H6ZvzYcBuSH_qFw_cT.JUuce2CGUxXHp%2BcVZUWZK6GSx4hkbWJfyNdixGSCRS4; dh_user_id=50155f00-1ab6-11eb-8436-95acf3119b5b; __csrf=fbbb9ca2-89c1-454a-a185-be7948b97e1c; WZRK_S_W4R-49K-494Z=%7B%22p%22%3A7%2C%22s%22%3A1612264886%2C%22t%22%3A1612265207%7D".to_owned(),
        )
        .json(&body)
        .send()
        .await;
    
    match res {
        Err(_) => Err(field_error("Request failure")),
        Ok(data) => {
            
            let decode= data.json::<D>().await;
            
            decode
            .map_err(|_| field_error("JSON decode failure"))
            .and_then(|gql| {
                Ok(gql)    
            })
        }
    }
}
