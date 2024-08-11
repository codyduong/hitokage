use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError, RenderErrorReason};
use serde_json::Value;
use std::str::FromStr;

pub fn add_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  // get parameter from helper or throw an error
  let base = h
    .param(0)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("format", 0))?
    .value();
  let other = h
    .param(1)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("format", 0))?
    .value();

  let base_value = match base {
    Value::Number(n) => n
      .as_i64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid i64"))?,
    Value::String(s) => {
      i64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid i64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an i64 nor a string that can be parsed as i64",
    ))?,
  };

  let other_value = match other {
    Value::Number(n) => n
      .as_i64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid i64"))?,
    Value::String(s) => {
      i64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid i64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an i64 nor a string that can be parsed as i64",
    ))?,
  };

  let result = base_value + other_value;

  write!(out, "{}", result)?;
  Ok(())
}
