use dotenv;

// Checks if the supplied Env
pub fn check_env_vars(env_vars: &[String]) -> Result<(), String> {
  dotenv::from_filename(".env").ok();

  for env_var in env_vars.iter() {
    check_env_var(env_var)?
  }
  Ok(())
}

// Retrieves an environmental variable
fn check_env_var(env_var: &str) -> Result<(), String> {
  match dotenv::var(env_var) {
    Ok(_) => Ok(()),
    Err(_) => Err(format!("Environmental Variable {} not found!", env_var)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[test]
  fn single_env_check() {
    env::set_var("TEST_VALUE", "VALUE");
    assert!(check_env_var("TEST_VALUE").is_ok());
  }

  #[test]
  fn combined_env_check() {
    env::set_var("TEST_VALUE", "VALUE");
    let env_arr = ["TEST_VALUE".to_string(), "POSTGRE_PORT".to_string()];
    assert!(check_env_vars(&env_arr).is_ok());
  }

  #[test]
  fn fail_combined_env_check() {
    env::set_var("TEST_VALUE", "VALUE");
    let env_arr = ["TEST_VALUE".to_string(), "POSTGRE_PORsT".to_string()];
    assert!(!check_env_vars(&env_arr).is_ok());
  }
}
