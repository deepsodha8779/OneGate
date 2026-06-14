use crate::utils::env_helper::AppEnv;
use anyhow::Result;
use awc::Client;
use dotenv::var;
use log::info;
use urlencoding::encode;

pub async fn send_sms(message: &str, numbers: Vec<String>, sender: &str) -> Result<()> {
    info!(target: "TextLocal","Sending SMS");
    let test = match AppEnv::current_env()? {
        AppEnv::Development => {
            info!(target: "TextLocal", "Development");
            true
        }
        AppEnv::Production => {
            info!(target: "TextLocal", "Production");
            false
        }
    };
    let apikey = var("TEXT_LOCAL_API_KEY")?;
    info!(target: "TextLocal", "API Key: {}", apikey);
    let client = Client::default();

    //TODO: will not work for now due to some government regulations

    let url = format!(
        "https://api.textlocal.in/send/?apikey={}&numbers={}&message={}&sender={}&test={}",
        encode(&apikey),
        encode(&numbers.join(",")),
        encode(&message),
        encode(&sender),
        test
    );
    info!(target: "TextLocal", "URL: {}", url);
    let res = client.get(url).send().await;

    info!( target: "Text Local Response:", "{:?}", res.unwrap().body().await?);

    Ok(())
}
