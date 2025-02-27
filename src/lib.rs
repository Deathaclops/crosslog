
use log::{Level, error, warn, info, debug, trace};

#[cfg(target_arch = "wasm32")]


pub mod prelude {
	pub use log::{Level, error, warn, info, debug, trace};
	pub use super::println;
	pub use super::log;
} // mod prelude

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use WebAssembly::*;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod WebAssembly {

	#[cfg(feature = "debug")]
	pub use console_error_panic_hook;

	pub use web_sys::console::log_1;
	pub use wasm_bindgen::JsValue;

	#[macro_export]
	macro_rules! println {
		($($arg:expr),+) => (
		{ $crate::log_1 (&$crate::JsValue::from(format!($($arg),+))); }
		);
	} // end macro_rules! println
	pub use println as log;

	pub fn init ( level: log::Level ) {

		#[cfg(feature = "debug")]
		console_error_panic_hook::set_once();
		wasm_logger::init(wasm_logger::Config::new(
			level
		)); // end wasm_logger:init

	} // end fn init

} // end mod WebAssembly

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use Native::*;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
mod Native {

	use env_logger::{Builder, Env};
	use std::io::Write;
	pub use std::println;
	pub use std::println as log;

	pub fn init ( level: log::Level ) {
		Builder::from_env(Env::default().default_filter_or(
			level.as_str()
		)).format(|buf, record| {
			writeln! ( buf, "{}:{} [{}] {}",
				record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
				record.level(),
                record.args()		)
		}).init();
	} // end fn init

} // end mod Native

