use once_cell::sync::OnceCell;
use javy::Runtime;
use crate::runtime;

pub(crate) static mut RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub unsafe fn init() {
    let runtime = runtime::new_runtime().unwrap();
    unsafe { RUNTIME.set(runtime).unwrap() };
}

pub unsafe fn get_runtime() -> &'static Runtime {
    let runtime = RUNTIME.get().unwrap();
    runtime
}
