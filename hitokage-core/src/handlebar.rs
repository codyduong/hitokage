use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError, RenderErrorReason};
use serde_json::Value;
use std::str::FromStr;

pub fn register_hitokage_helpers(mut reg: Handlebars<'_>) -> Handlebars<'_> {
  reg.register_helper("add", Box::new(add_helper));
  reg.register_helper("mult", Box::new(mult_helper));
  reg.register_helper("div", Box::new(div_helper));
  reg.register_helper("round", Box::new(round_helper));
  reg.register_helper("pad", Box::new(pad_helper));
  reg.register_helper("concat", Box::new(concat_helper));
  reg
}

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
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let other_value = match other {
    Value::Number(n) => n
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let result = base_value + other_value;

  write!(out, "{}", result)?;
  Ok(())
}

pub fn mult_helper(
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
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let other_value = match other {
    Value::Number(n) => n
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let result = base_value * other_value;

  write!(out, "{}", result)?;
  Ok(())
}

pub fn div_helper(
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
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let other_value = match other {
    Value::Number(n) => n
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  let result = base_value / other_value;

  write!(out, "{}", result)?;
  Ok(())
}

pub fn round_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  // Get the f64 value from the first parameter
  let base_value = match h
    .param(0)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("mult_helper", 0))?
    .value()
  {
    Value::Number(n) => n
      .as_f64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?,
    Value::String(s) => {
      f64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("First parameter is not a valid f64"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "First parameter is neither an f64 nor a string that can be parsed as f64",
    ))?,
  };

  // Get the precision from the second parameter
  let precision = match h
    .param(1)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("mult_helper", 1))?
    .value()
  {
    Value::Number(n) => n
      .as_i64()
      .ok_or_else(|| RenderErrorReason::InvalidParamType("Second parameter is not a valid integer"))?,
    Value::String(s) => {
      i64::from_str(s).map_err(|_| RenderErrorReason::InvalidParamType("Second parameter is not a valid integer"))?
    }
    _ => Err(RenderErrorReason::InvalidParamType(
      "Second parameter is neither an integer nor a string that can be parsed as an integer",
    ))?,
  } as usize;

  // Round the result to the specified precision
  let result = (base_value * 10f64.powi(precision as i32)).round() / 10f64.powi(precision as i32);

  // Write the rounded result to the output
  write!(out, "{:.precision$}", result, precision = precision)?;
  Ok(())
}

pub fn pad_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  // let foo = h
  //   .param(0)
  //   .ok_or(RenderErrorReason::ParamNotFoundForIndex("pad", 0))?
  //   .value();

  // log::info!("{:?}", foo);

  let direction = h
    .param(0)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("pad", 0))?
    .value()
    .as_str()
    .ok_or_else(|| RenderErrorReason::InvalidParamType("Direction should be a string"))?
    .to_owned();

  let input = h
    .param(1)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("pad", 1))?
    .value()
    .as_str()
    .ok_or_else(|| RenderErrorReason::InvalidParamType("Input should be a string"))?
    .to_owned();

  let total_length = h
    .param(2)
    .ok_or(RenderErrorReason::ParamNotFoundForIndex("pad", 2))?
    .value()
    .as_u64()
    .ok_or_else(|| RenderErrorReason::InvalidParamType("Total length should be a number"))?;

  let pad_char = h.param(3).map_or(" ", |v| v.value().as_str().unwrap_or(" "));

  let pad_char = if pad_char.chars().count() == 1 {
    pad_char
  } else {
    return Err(RenderErrorReason::InvalidParamType("Padding character must be a single character").into());
  };

  // Perform the padding
  let result = match direction.as_str() {
    "left" => format!("{:>pad$}", input, pad = total_length as usize).replace(" ", pad_char),
    "right" => format!("{:<pad$}", input, pad = total_length as usize).replace(" ", pad_char),
    _ => return Err(RenderErrorReason::InvalidParamType("Invalid direction. Use 'left' or 'right'.").into()),
  };

  // Write the padded string to the output
  out.write(&result)?;
  Ok(())
}

pub fn concat_helper(
  h: &Helper,
  _: &Handlebars,
  _: &Context,
  _: &mut RenderContext,
  out: &mut dyn Output,
) -> Result<(), RenderError> {
  if h.params().len() < 2 {
    return Err(
      RenderErrorReason::MissingVariable(Some("The `concat` helper expects at least two arguments.".to_string()))
        .into(),
    );
  }

  let mut result = String::new();

  for i in 0..h.params().len() {
    let param = h
      .param(i)
      .ok_or(RenderErrorReason::ParamNotFoundForIndex("concat", i))?
      .value();

    if let Some(s) = param.as_str() {
      result.push_str(s);
    } else {
      return Err(RenderErrorReason::InvalidParamType("All parameters should be strings").into());
    }
  }

  // Write the concatenated string to the output
  out.write(&result)?;
  Ok(())
}
