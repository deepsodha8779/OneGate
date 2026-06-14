use anyhow::{bail, Result};
use biscuit_auth::Biscuit;

pub fn _get_user_detail(biscuit: &Biscuit) -> Result<(String, String)> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, String)> =
        authorizer.query_all("data($id, $name) <- user($id, $name)")?;
    let user = res.first();
    match user {
        Some(u) => Ok(u.to_owned()),
        None => bail!("Not found"),
    }
}

pub fn get_society_detail(biscuit: &Biscuit) -> Result<(String, String)> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, String)> =
        authorizer.query_all("data($id, $name) <- society($id, $name)")?;
    let user = res.first();
    match user {
        Some(u) => Ok(u.to_owned()),
        None => bail!("Not found"),
    }
}
