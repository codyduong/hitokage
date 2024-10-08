extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  parse_macro_input, ImplItem, ItemImpl, Path,
};

struct OptionalPath {
  path: Option<Path>,
}

impl Parse for OptionalPath {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.is_empty() {
      // If the input is empty, return None
      Ok(OptionalPath { path: None })
    } else {
      // Otherwise, parse the input as a Path
      let path: Path = input.parse()?;
      Ok(OptionalPath { path: Some(path) })
    }
  }
}

#[proc_macro_attribute]
pub fn impl_lua_base(args: TokenStream, input: TokenStream) -> TokenStream {
  let mut item_impl = parse_macro_input!(input as ItemImpl);
  let path: OptionalPath = parse_macro_input!(args as OptionalPath);

  // Check if this is the regular `impl` block (not a trait impl)
  let is_regular_impl = item_impl.trait_.is_none();

  let imports = quote! {
    use hitokage_core::components::base::BaseMsgHook::{
      GetClass, GetHalign, GetHeight, GetHeightRequest, GetHexpand, GetSizeRequest, GetValign, GetVexpand, GetWidth,
      GetWidthRequest, SetClass, SetHalign, SetHeightRequest, SetHexpand, GetId, SetSizeRequest, SetValign, SetVexpand,
      SetWidthRequest,
    };
  };

  if is_regular_impl {
    let path = path.path.unwrap();

    // Code to generate for the struct's impl block
    let struct_code = quote! {
        impl_getter_fn!(get_class, #path, BaseHook, GetClass, Vec<String>);
        impl_setter_fn!(set_class, #path, BaseHook, SetClass, Vec<String>);

        impl_getter_fn!(get_halign, #path, BaseHook, GetHalign, Align);
        impl_setter_fn!(set_halign, #path, BaseHook, SetHalign, Align);

        impl_getter_fn!(get_height, #path, BaseHook, GetHeight, i32);
        impl_getter_fn!(get_height_request, #path, BaseHook, GetHeightRequest, i32);
        impl_setter_fn!(set_height_request, #path, BaseHook, SetHeightRequest, Option<i32>);

        impl_getter_fn!(get_hexpand, #path, BaseHook, GetHexpand, Option<bool>);
        impl_setter_fn!(set_hexpand, #path, BaseHook, SetHexpand, Option<bool>);

        impl_getter_fn!(get_id, #path, BaseHook, GetId, Option<String>);

        impl_getter_fn!(get_size_request, #path, BaseHook, GetSizeRequest, (i32, i32));
        impl_setter_fn!(set_size_request, #path, BaseHook, SetSizeRequest, (Option<i32>, Option<i32>));

        impl_getter_fn!(get_valign, #path, BaseHook, GetValign, Align);
        impl_setter_fn!(set_valign, #path, BaseHook, SetValign, Align);

        impl_getter_fn!(get_vexpand, #path, BaseHook, GetVexpand, Option<bool>);
        impl_setter_fn!(set_vexpand, #path, BaseHook, SetVexpand, Option<bool>);

        impl_getter_fn!(get_width, #path, BaseHook, GetWidth, i32);
        impl_getter_fn!(get_width_request, #path, BaseHook, GetWidthRequest, i32);
        impl_setter_fn!(set_width_request, #path, BaseHook, SetWidthRequest, Option<i32>);
    };

    item_impl.items.push(syn::ImplItem::Verbatim(struct_code));
  }

  // Check if this is the `impl UserData` block
  let is_userdata_impl = item_impl
    .trait_
    .as_ref()
    .map_or(false, |(_, path, _)| path.is_ident("UserData"));

  if is_userdata_impl {
    // Code to generate for the UserData implementation
    let userdata_code_add_methods = quote! {
      methods.add_method("get_class", |lua, instance, ()| lua.pack(instance.get_class()?));
      methods.add_method("set_class", |lua, this, args: mlua::Variadic<Value>| { this.set_class(lua, args) });

      methods.add_method("get_halign", |lua, instance, ()| lua.to_value(&instance.get_halign()?));
      methods.add_method("set_halign", |lua, this, value: mlua::Value| { this.set_halign(lua, value) });

      methods.add_method("get_height", |lua, instance, ()| lua.to_value(&instance.get_height()?));
      methods.add_method("get_height_request", |lua, instance, ()| lua.to_value(&instance.get_height_request()?));
      methods.add_method("set_height_request", |lua, this, args: mlua::Value| { this.set_height_request(lua, args) });

      methods.add_method("get_hexpand", |lua, instance, ()| { lua.to_value(&instance.get_hexpand()?) });
      methods.add_method("set_hexpand", |lua, this, value: mlua::Value| { this.set_hexpand(lua, value) });

      methods.add_method("get_id", |lua, instance, ()| { lua.to_value(&instance.get_id()?) });

      methods.add_method("get_size_request", |lua, instance, ()| lua.to_value(&instance.get_size_request()?));
      methods.add_method("set_size_request", |lua, this, value: mlua::Value| { this.set_size_request(lua, value) });

      methods.add_method("get_valign", |lua, instance, ()| lua.to_value(&instance.get_valign()?));
      methods.add_method("set_valign", |lua, this, value: mlua::Value| { this.set_valign(lua, value) });

      methods.add_method("get_vexpand", |lua, instance, ()| { lua.to_value(&instance.get_vexpand()?) });
      methods.add_method("set_vexpand", |lua, this, value: mlua::Value| { this.set_vexpand(lua, value) });

      methods.add_method("get_width", |lua, instance, ()| lua.to_value(&instance.get_width()?));
      methods.add_method("get_width_request", |lua, instance, ()| lua.to_value(&instance.get_width_request()?));
      methods.add_method("set_width_request", |lua, this, args: mlua::Value| { this.set_width_request(lua, args) });
    };

    if let Some(ImplItem::Fn(ref mut method)) = item_impl.items.iter_mut().find(|item| match item {
      ImplItem::Fn(method) => method.sig.ident == "add_methods",
      _ => false,
    }) {
      let old_block = &method.block;

      let combined_block = quote!({
        #userdata_code_add_methods
        #old_block
      });

      let combined_block: syn::Block = syn::parse2(combined_block).expect("Failed to parse combined block");

      method.block = combined_block;
    }
  }

  if !is_regular_impl {
    TokenStream::from(quote! {
      #imports
      #item_impl
    })
  } else {
    item_impl.to_token_stream().into()
  }
}
