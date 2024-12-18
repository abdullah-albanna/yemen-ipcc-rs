mod battery;
mod hardware;
mod os;
mod storage;

pub use battery::handle_device_battery;
pub use hardware::handle_device_hardware;
pub use os::handle_device_os;
pub use storage::handle_device_storage;
