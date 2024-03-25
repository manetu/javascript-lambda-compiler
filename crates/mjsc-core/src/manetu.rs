use std::ffi::c_void;
use anyhow::{Error, Result};
use once_cell::sync::OnceCell;
use javy::json::{transcode_input, transcode_output};
use javy::quickjs::{from_qjs_value, JSContextRef, JSValue, JSValueRef};
use javy::Runtime;
use libc;
use libc::size_t;
use crate::manetu::exports::manetu::lambda::guest;

use quickjs_wasm_sys::{JS_AddRef, JSContext};
use crate::state;

#[derive(Debug)]
struct Function {
    fn_obj: quickjs_wasm_sys::JSValue,
}

impl Function {
    unsafe fn new(ctx: *mut JSContext, fn_obj: quickjs_wasm_sys::JSValue) -> Self {
        JS_AddRef(ctx, fn_obj);
        Self { fn_obj }
    }

    unsafe fn call(&self, args: &[JSValueRef]) -> Result<String> {
        let ctx = state::get_runtime().context();
        let x = JSValueRef::new(ctx, self.fn_obj).unwrap();
        let raw_ret = x.call(&x, args).unwrap();
        let ret = transcode_output(raw_ret).unwrap();
        Ok(String::from_utf8(ret).unwrap())
    }
}

static mut CALLBACK: OnceCell<Function> = OnceCell::new();

wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "lambda",
});

struct GuestImpl;
impl guest::Guest for GuestImpl {

    fn handle_request(request: String) -> String {
        let runtime = unsafe { state::get_runtime() };
        let ctx = runtime.context();
        let f = unsafe { CALLBACK.get().unwrap() };

        let req = transcode_input(ctx, request.as_bytes()).unwrap();
        unsafe { f.call(&[req]) }.unwrap()
    }

    fn malloc(len: u32) -> u32 {
        unsafe {
            let r = libc::malloc(len as size_t);
            r as u32
        }
    }

    fn free(ptr: u32) {
        unsafe { libc::free(ptr as *mut c_void) }
    }
}

fn query(ctx: &JSContextRef, arg: JSValueRef) -> Result<JSValue> {
    let raw_ret = manetu::lambda::sparql::query(arg.as_str().unwrap());
    let json_ret = transcode_input(ctx, raw_ret.as_bytes()).unwrap();
    Ok(from_qjs_value(json_ret).unwrap())
}

unsafe fn register_handler(ctx: *mut JSContext, callback: quickjs_wasm_sys::JSValue) -> quickjs_wasm_sys::JSValue {
    let f = Function::new(ctx, callback);
    unsafe { CALLBACK.set(f).unwrap(); }
    0
}

pub unsafe fn register(runtime: &Runtime) -> Result<(), Error> {
    let ctx = runtime.context();

    let global_object = ctx.global_object()?;

    // Registers Manetu functions into the JS global namespace.
    let lambda_object = ctx.object_value()?;
    lambda_object.set_property("query", ctx.wrap_callback(|ctx, _this, args| query(ctx, args[0]))?)?;
    lambda_object.set_property("register", ctx.new_callback(|ctx, _this, _argc, argv, _| register_handler(ctx, *argv.offset(0)))?)?;
    global_object.set_property("lambda", lambda_object)?;

    Ok(())
}

export!(GuestImpl);
