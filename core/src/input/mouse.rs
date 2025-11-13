use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Hash, Default, Serialize, Deserialize)]
pub enum MouseButton {
	#[default]
	Unknown = -1,
	Button1,
	Button2,
	Button3,
	Button4,
	Button5,
	Button6,
	Button7,
	Button8,
	Button9,
	Button10
}

impl MouseButton {
	pub const Left: MouseButton = MouseButton::Button1;
	pub const Right: MouseButton = MouseButton::Button2;
	pub const Middle: MouseButton = MouseButton::Button3;
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Hash, Default, Serialize, Deserialize)]
pub enum MouseScrollWheel {
	#[default]
	None = -1,
	Up,
	Down,
	Left,
	Right
}
