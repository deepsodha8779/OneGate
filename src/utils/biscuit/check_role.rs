use anyhow::{bail, Result};
use biscuit_auth::Biscuit;
use std::collections::HashSet;

// pub fn check_role_exist(role: Vec<String>, biscuit: &Biscuit) -> Result<()> {
//     let mut authorizer = biscuit.authorizer()?;
//     let res: Vec<(String, bool)> = authorizer.query_all("r($name, true) <- role($name)")?;
//     let r = res.iter().find(|(x, _)| x.eq(&role)).is_some();
//     if r {
//         Ok(())
//     } else {
//         bail!("Role not found!")
//     }
// }

pub fn check_role_exist(role: Vec<String>, biscuit: &Biscuit) -> Result<()> {
    let mut authorizer = biscuit.authorizer()?;
    let res: Vec<(String, bool)> = authorizer.query_all("r($name, true) <- role($name)")?;

    let role_set: HashSet<_> = role.iter().collect();
    let r = res.iter().any(|(x, _)| role_set.contains(x));

    if r {
        Ok(())
    } else {
        bail!("Role not found!")
    }
}
