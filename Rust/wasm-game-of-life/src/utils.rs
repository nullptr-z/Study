pub fn set_panic_hook() {
    // 当启用 `console_error_panic_hook` 功能时，我们可以调用
    // `set_panic_hook` 函数在初始化期间至少运行一次，然后

    // 如果运行是发生panic，错误会抛到控制台
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// extern crate web_sys;
// fn now() -> f64 {
//     web_sys::window()
//         .expect("should have a Window")
//         .performance()
//         .expect("should have a Performance")
//         .now()
// }
