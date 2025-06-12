use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use http::header::{HeaderMap, LOCATION};
use lambda_runtime::{handler_fn, Context, Error};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn my_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let c = event.query_string_parameters.get("c").cloned().unwrap_or_default();

    let client = reqwest::Client::new();
    let response = client.get(format!("https://www.youtube.com/@{}/live", c)).send().await.unwrap();

    let html = response.text().await.unwrap();
    
    // NOTE: An AI wrote the regex
    let re = Regex::new(r#""popoutLiveChatEndpoint":\{"url":"(?P<chatUrl>https://www\.youtube\.com/live_chat\?is_popout=1\\u0026v=[^"]+)""#).unwrap();
    let mut url = String::new();
    if let Some(caps) = re.captures(&html) {
            url = caps.name("chatUrl").unwrap().as_str().to_string().replace(r"\u0026", "&");
            url.push_str("&dark_theme=1");
            url.push_str("&chat_mode=all");
    }

    let mut headers = HeaderMap::new();
        headers.insert(
        LOCATION, 
        url.parse().unwrap()
    );

    let resp = ApiGatewayProxyResponse {
        status_code: 302,
        headers,
        multi_value_headers: HeaderMap::new(),
        body: None,
        is_base64_encoded: Some(false),
    };
    
    Ok(resp)
}
