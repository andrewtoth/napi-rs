use std::convert::TryInto;

use napi::{CallContext, JsExternal, JsNumber, Module, Result};

struct NativeObject {
  count: i32,
}

#[js_function(1)]
pub fn create_external(ctx: CallContext) -> Result<JsExternal> {
  let count = ctx.get::<JsNumber>(0)?.try_into()?;
  let native = NativeObject { count };
  ctx.env.create_external(native)
}

#[js_function(1)]
pub fn get_external_count(ctx: CallContext) -> Result<JsNumber> {
  let attached_obj = ctx.get::<JsExternal>(0)?;
  let native_object = ctx.env.get_value_external::<NativeObject>(&attached_obj)?;
  ctx.env.create_int32(native_object.count)
}

pub fn register_js(module: &mut Module) -> Result<()> {
  module.create_named_method("createExternal", create_external)?;
  module.create_named_method("getExternalCount", get_external_count)?;
  Ok(())
}
