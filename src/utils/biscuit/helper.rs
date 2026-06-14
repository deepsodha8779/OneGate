use anyhow::Result;
use biscuit_auth::builder::{date, fact, string, AuthorizerBuilder};
use biscuit_auth::{error, Biscuit, KeyPair};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub fn create_token(
    root: &KeyPair,
    user_roles: Vec<&str>,
    user_id: &str,
    society_id: &str,
) -> Result<String> {
    let mut builder = Biscuit::builder();

    // Add role facts
    for role in user_roles {
        builder = builder.fact(fact("role", &[string(role)]))?;
    }

    // Expiration 1 year
    let exp_time = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 365);

    builder = builder.fact(fact("time", &[date(&exp_time)]))?;
    builder = builder.fact(fact("user_id", &[string(user_id)]))?;
    builder = builder.fact(fact("society", &[string(society_id), string("igate")]))?;

    // Build token
    let token = builder.build(root)?;
    Ok(token.to_base64()?)
}

pub fn _authorize(required_role: &str, token: &Biscuit) -> Result<(), error::Token> {
    let mut params = HashMap::new();
    params.insert("r".to_string(), string(required_role));
    params.insert("now".to_string(), date(&SystemTime::now()));

    let mut authorizer = AuthorizerBuilder::new()
        .time()
        .code_with_params(
            r#"
            allow if role({r});
            check if time($t), $t > {now};
        "#,
            params,
            HashMap::new(),
        )?
        .build(token)?;

    authorizer.authorize()?;
    Ok(())
}
