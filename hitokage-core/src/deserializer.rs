// we need a custom deserializer for mlua Functions and UserData

// TODO @codyduong
// https://github.com/mlua-rs/mlua/pull/436

lazy_static::lazy_static! {
  pub(crate) static ref FUNCTION_REGISTRY: std::sync::Mutex<std::collections::HashMap<usize, RegistryKey>> =
      std::sync::Mutex::new(std::collections::HashMap::new());
}

use crate::structs::reactive::Reactive;
use mlua::{AnyUserData, Error as LuaError, RegistryKey, Table, TablePairs, TableSequence, Value};
use rustc_hash::FxHashSet;
use serde::{
  de::{self, Visitor},
  forward_to_deserialize_any,
};
use std::{cell::RefCell, rc::Rc};
use std::{
  os::raw::c_void,
  sync::{Arc, Mutex},
};

/* ************************************************************* */
/* https://docs.rs/mlua/0.9.8/src/mlua/serde/de.rs.html#663-682 */
pub(crate) struct RecursionGuard {
  ptr: *const c_void,
  visited: Rc<RefCell<FxHashSet<*const c_void>>>,
}

impl RecursionGuard {
  #[inline]
  pub(crate) fn new(table: &Table, visited: &Rc<RefCell<FxHashSet<*const c_void>>>) -> Self {
    let visited = Rc::clone(visited);
    let ptr = table.to_pointer();
    visited.borrow_mut().insert(ptr);
    RecursionGuard { ptr, visited }
  }
}

impl Drop for RecursionGuard {
  fn drop(&mut self) {
    self.visited.borrow_mut().remove(&self.ptr);
  }
}

/* ******************************************************** */
/* https://docs.rs/mlua/0.9.8/src/mlua/serde/de.rs.html#96 */

pub struct LuaDeserializer<'lua> {
  value: Value,
  options: mlua::DeserializeOptions,
  visited: Rc<RefCell<FxHashSet<*const c_void>>>,
  inner: mlua::serde::de::Deserializer,
  lua: &'lua mlua::Lua,
}

impl<'lua> LuaDeserializer<'lua> {
  pub fn new(lua: &'lua mlua::Lua, value: Value, options: mlua::DeserializeOptions) -> Self {
    Self {
      value: value.clone(),
      options,
      visited: Rc::new(RefCell::new(FxHashSet::default())),
      inner: mlua::serde::de::Deserializer::new_with_options(value, options),
      lua,
    }
  }

  fn from_parts(
    lua: &'lua mlua::Lua,
    value: Value,
    options: mlua::DeserializeOptions,
    visited: Rc<RefCell<FxHashSet<*const c_void>>>,
  ) -> Self {
    LuaDeserializer {
      value: value.clone(),
      options,
      visited,
      inner: mlua::serde::de::Deserializer::new_with_options(value, options),
      lua,
    }
  }
}

impl<'de, 'lua> de::Deserializer<'de> for LuaDeserializer<'lua> {
  type Error = LuaError;

  #[inline]
  fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, LuaError>
  where
    V: Visitor<'de>,
  {
    // log::error!("testest: {:?}", self.value);

    match self.value {
      Value::Nil => visitor.visit_unit(),
      Value::Boolean(b) => visitor.visit_bool(b),
      Value::Integer(i) => visitor.visit_i64(i.into()),
      Value::Number(n) => visitor.visit_f64(n.into()),
      Value::String(s) => match s.to_str() {
        Ok(s) => visitor.visit_str(&s),
        Err(_) => visitor.visit_bytes(&s.as_bytes()),
    },
      Value::Table(ref t) if t.raw_len() > 0 /* || t.is_array() */ => self.deserialize_seq(visitor),
      Value::Table(_) => self.deserialize_map(visitor),
      Value::LightUserData(ud) if ud.0.is_null() => visitor.visit_none(),
      Value::Function(f ) => {
        let id = f.to_pointer() as usize;
        let k = self.lua.create_registry_value(f.clone()).unwrap();

        FUNCTION_REGISTRY.lock().unwrap().insert(id, k);

        // let visitor_ptr = &mut visitor as *mut V;
        // unsafe {
        //   let lua_visitor: *mut LuaFnVisitor = visitor_ptr as *mut LuaFnVisitor;

        //   let lua = {
        //     let f_ptr = &f as *const mlua::Function<'lua> as *const u8;
        //     let ptr = f_ptr.add(0) as *const &mlua::Lua;
        //     let lua: &mlua::Lua = std::ptr::read(ptr);

        //     lua
        // };
        //   // wat da fuck am i doing
        //   let owned = mlua::Function::from_lua(f.into_lua(lua)?, lua).map(|s| s.into_owned())?;  
        //   if !lua_visitor.is_null() && (*lua_visitor).f.is_none() {
        //     (*lua_visitor).f = Some(owned);
        //   }
        // }

        let mut bytes = Vec::with_capacity(std::mem::size_of::<u8>() + std::mem::size_of::<usize>());
        bytes.push(0x01);
        bytes.extend_from_slice(&id.to_ne_bytes());
        visitor.visit_byte_buf(bytes)

        // let fptr = &f as *const mlua::Function as *const u8;
        // let b = unsafe {
        //   slice::from_raw_parts(fptr, std::mem::size_of::<mlua::Function>())
        // };

        // let mut bytes = Vec::with_capacity(std::mem::size_of::<u8>() + std::mem::size_of::<mlua::Function>());
        // bytes.push(0x01);
        // bytes.extend_from_slice(&id.to_ne_bytes());
        // bytes.extend_from_slice(b);
        // visitor.visit_byte_buf(bytes)
      },
      Value::UserData(ref ud) => {
        // Handle UserData specifically
        if let Ok(ud) = ud.borrow::<Reactive<String>>() {
          let ud = ud.to_owned();
          let ptr1 = Arc::into_raw(ud.value) as *const Mutex<String>;
          let ptr2 = Arc::into_raw(ud.sender) as *const Mutex<Option<std::sync::mpsc::Sender<()>>>;

          let ptr1_value = ptr1 as usize;
          let ptr2_value = ptr2 as usize;

          let mut bytes = Vec::with_capacity(std::mem::size_of::<u8>() + std::mem::size_of::<usize>() * 2);
          bytes.push(0x00);
          bytes.extend_from_slice(&ptr1_value.to_ne_bytes());
          bytes.extend_from_slice(&ptr2_value.to_ne_bytes());

          return visitor.visit_byte_buf(bytes.to_vec());
        }

        visitor.visit_unit()
      }
      _ => {
        self.inner.deserialize_any(visitor)
      }
    }
  }

  #[inline]
  fn deserialize_option<V>(self, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    self.inner.deserialize_option(visitor)
  }

  #[inline]
  fn deserialize_enum<V>(
    self,
    name: &'static str,
    variants: &'static [&'static str],
    visitor: V,
  ) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    self.inner.deserialize_enum(name, variants, visitor)
  }

  #[inline]
  fn deserialize_seq<V>(self, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
            Value::Table(t) => {
                let _guard = RecursionGuard::new(&t, &self.visited);

                let len = t.raw_len();
                let mut deserializer = SeqDeserializer {
                    seq: t.sequence_values(),
                    options: self.options,
                    visited: self.visited,
                    lua: self.lua,
                };
                let seq = visitor.visit_seq(&mut deserializer)?;
                if deserializer.seq.count() == 0 {
                    Ok(seq)
                } else {
                    Err(de::Error::invalid_length(
                        len,
                        &"fewer elements in the table",
                    ))
                }
            }
            Value::UserData(ud) if false /* ud.is_serializable() */ => {
                serde_userdata(ud, |value| value.deserialize_seq(visitor))
            }
            value => Err(de::Error::invalid_type(
                de::Unexpected::Other(value.type_name()),
                &"table",
            )),
        }
  }

  #[inline]
  fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    self.deserialize_seq(visitor)
  }

  #[inline]
  fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    self.deserialize_seq(visitor)
  }

  #[inline]
  fn deserialize_map<V>(self, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::Table(t) => {
        let _guard = RecursionGuard::new(&t, &self.visited);

        let mut deserializer = MapDeserializer {
          pairs: MapPairs::new(&t)?,
          value: None,
          options: self.options,
          visited: self.visited,
          processed: 0,
          lua: self.lua,
        };
        let map = visitor.visit_map(&mut deserializer)?;
        let count = deserializer.pairs.count();
        if count == 0 {
          Ok(map)
        } else {
          Err(de::Error::invalid_length(
            deserializer.processed + count,
            &"fewer elements in the table",
          ))
        }
      }
      value => Err(de::Error::invalid_type(
        de::Unexpected::Other(value.type_name()),
        &"table",
      )),
    }
  }

  #[inline]
  fn deserialize_struct<V>(
    self,
    _name: &'static str,
    _fields: &'static [&'static str],
    visitor: V,
  ) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    self.deserialize_map(visitor)
  }

  #[inline]
  fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    log::error!("here3: {:?}", self.value);
    self.inner.deserialize_newtype_struct(name, visitor)
  }

  #[inline]
  fn deserialize_unit<V>(self, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::LightUserData(ud) if ud.0.is_null() => visitor.visit_unit(),
      _ => self.deserialize_any(visitor),
    }
  }

  #[inline]
  fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> mlua::Result<V::Value>
  where
    V: de::Visitor<'de>,
  {
    match self.value {
      Value::LightUserData(ud) if ud.0.is_null() => visitor.visit_unit(),
      _ => self.deserialize_any(visitor),
    }
  }

  forward_to_deserialize_any! {
    bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes
    byte_buf identifier ignored_any
  }

  #[inline]
  fn is_human_readable(&self) -> bool {
    true
  }
}

/* ************************************************************* */
/* https://docs.rs/mlua/0.9.8/src/mlua/serde/de.rs.html#397-434 */

struct SeqDeserializer<'lua> {
  seq: TableSequence<'lua, Value>,
  options: mlua::DeserializeOptions,
  visited: Rc<RefCell<FxHashSet<*const c_void>>>,
  lua: &'lua mlua::Lua,
}

impl<'lua, 'de> de::SeqAccess<'de> for SeqDeserializer<'lua> {
  type Error = mlua::Error;

  fn next_element_seed<T>(&mut self, seed: T) -> mlua::Result<Option<T::Value>>
  where
    T: de::DeserializeSeed<'de>,
  {
    loop {
      match self.seq.next() {
        Some(value) => {
          let value = value?;
          let skip = check_value_for_skip(&value, self.options, &self.visited)
            .map_err(|err| mlua::Error::DeserializeError(err.to_string()))?;
          if skip {
            continue;
          }
          let visited = Rc::clone(&self.visited);
          let deserializer = LuaDeserializer::from_parts(self.lua, value, self.options, visited);
          return seed.deserialize(deserializer).map(Some);
        }
        None => return Ok(None),
      }
    }
  }

  fn size_hint(&self) -> Option<usize> {
    match self.seq.size_hint() {
      (lower, Some(upper)) if lower == upper => Some(upper),
      _ => None,
    }
  }
}

/* ************************************************************* */
/* https://docs.rs/mlua/0.9.8/src/mlua/serde/de.rs.html#469-567 */

pub(crate) enum MapPairs<'lua> {
  Iter(TablePairs<'lua, Value, Value>),
  #[allow(dead_code)]
  Vec(Vec<(Value, Value)>),
}

impl<'lua> MapPairs<'lua> {
  pub(crate) fn new(t: &'lua Table /* , sort_keys: bool */) -> mlua::Result<Self> {
    // if sort_keys {
    //     let mut pairs = t.pairs::<Value, Value>().collect::<Result<Vec<_>>>()?;
    //     pairs.sort_by(|(a, _), (b, _)| b.cmp(a)); // reverse order as we pop values from the end
    //     Ok(MapPairs::Vec(pairs))
    // } else {
    //     Ok(MapPairs::Iter(t.pairs::<Value, Value>()))
    // }
    Ok(MapPairs::Iter(t.pairs::<Value, Value>()))
  }

  pub(crate) fn count(self) -> usize {
    match self {
      MapPairs::Iter(iter) => iter.count(),
      MapPairs::Vec(vec) => vec.len(),
    }
  }

  pub(crate) fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      MapPairs::Iter(iter) => iter.size_hint(),
      MapPairs::Vec(vec) => (vec.len(), Some(vec.len())),
    }
  }
}

impl<'lua> Iterator for MapPairs<'lua> {
  type Item = mlua::Result<(Value, Value)>;

  fn next(&mut self) -> Option<Self::Item> {
    match self {
      MapPairs::Iter(iter) => iter.next(),
      MapPairs::Vec(vec) => vec.pop().map(Ok),
    }
  }
}

struct MapDeserializer<'lua> {
  pairs: MapPairs<'lua>,
  value: Option<Value>,
  options: mlua::DeserializeOptions,
  visited: Rc<RefCell<FxHashSet<*const c_void>>>,
  processed: usize,
  lua: &'lua mlua::Lua,
}

impl<'lua, 'de> de::MapAccess<'de> for MapDeserializer<'lua> {
  type Error = mlua::Error;

  fn next_key_seed<T>(&mut self, seed: T) -> mlua::Result<Option<T::Value>>
  where
    T: de::DeserializeSeed<'de>,
  {
    loop {
      match self.pairs.next() {
        Some(item) => {
          let (key, value) = item?;
          let skip_key = check_value_for_skip(&key, self.options, &self.visited)
            .map_err(|err| mlua::Error::DeserializeError(err.to_string()))?;
          let skip_value = check_value_for_skip(&value, self.options, &self.visited)
            .map_err(|err| mlua::Error::DeserializeError(err.to_string()))?;
          if skip_key || skip_value {
            continue;
          }
          self.processed += 1;
          self.value = Some(value);
          let visited = Rc::clone(&self.visited);
          let key_de = LuaDeserializer::from_parts(self.lua, key, self.options, visited);
          return seed.deserialize(key_de).map(Some);
        }
        None => return Ok(None),
      }
    }
  }

  fn next_value_seed<T>(&mut self, seed: T) -> mlua::Result<T::Value>
  where
    T: de::DeserializeSeed<'de>,
  {
    match self.value.take() {
      Some(value) => {
        let visited = Rc::clone(&self.visited);
        seed.deserialize(LuaDeserializer::from_parts(self.lua, value, self.options, visited))
      }
      None => Err(de::Error::custom("value is missing")),
    }
  }

  fn size_hint(&self) -> Option<usize> {
    match self.pairs.size_hint() {
      (lower, Some(upper)) if lower == upper => Some(upper),
      _ => None,
    }
  }
}

/* ****************************************************************/
/*  https://docs.rs/mlua/0.9.8/src/mlua/serde/de.rs.html#684-721 */

// Checks `options` and decides should we emit an error or skip next element
pub(crate) fn check_value_for_skip(
  value: &Value,
  options: mlua::DeserializeOptions,
  visited: &RefCell<FxHashSet<*const c_void>>,
) -> Result<bool, &'static str> {
  match value {
    Value::Table(table) => {
      let ptr = table.to_pointer();
      if visited.borrow().contains(&ptr) {
        if options.deny_recursive_tables {
          return Err("recursive table detected");
        }
        return Ok(true); // skip
      }
    }
    // Value::Function(_) | Value::Thread(_) | Value::UserData(_) | Value::LightUserData(_) | Value::Error(_)
    //   if !options.deny_unsupported_types =>
    // {
    //   return Ok(true); // skip
    // }
    Value::Thread(_) | Value::LightUserData(_) | Value::Error(_) if !options.deny_unsupported_types => {
      return Ok(true); // skip
    }
    _ => {}
  }
  Ok(false) // do not skip
}

fn serde_userdata<V>(
  ud: AnyUserData,
  f: impl FnOnce(serde_value::Value) -> std::result::Result<V, serde_value::DeserializerError>,
) -> mlua::Result<V> {
  let value = serde_value::to_value(ud).map_err(|err| mlua::Error::SerializeError(err.to_string()))?;
  f(value).map_err(|err| mlua::Error::DeserializeError(err.to_string()))
}
