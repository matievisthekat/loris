use crate::environment::Environment;
use crate::utils;
use crate::values::Value;

#[derive(Debug, PartialEq)]
pub struct BindingUsage {
  name: String,
}

impl BindingUsage {
  pub fn new(s: &str) -> Result<(Self, &str), String> {
    let (name, s) = utils::extract_ident(s)?;

    Ok((
      Self {
        name: name.to_string(),
      },
      s,
    ))
  }

  pub(crate) fn eval(&self, env: &Environment) -> Result<Value, String> {
    env.get_binding_value(&self.name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_binding_usage() {
    assert_eq!(
      BindingUsage::new("abc"),
      Ok((
        BindingUsage {
          name: "abc".to_string(),
        },
        ""
      )),
    );
  }

  #[test]
  fn eval_existing_binding_usage() {
    let mut env = Environment::default();
    env.store_binding("foo".to_string(), Value::Number(10));

    assert_eq!(
      BindingUsage {
        name: "foo".to_string(),
      }
      .eval(&env),
      Ok(Value::Number(10)),
    );
  }

  #[test]
  fn eval_non_existent_binding_usage() {
    let empty_env = Environment::default();

    assert_eq!(
      BindingUsage {
        name: "i_dont_exist".to_string(),
      }
      .eval(&empty_env),
      Err("binding with name ‘i_dont_exist’ does not exist".to_string()),
    );
  }
}
