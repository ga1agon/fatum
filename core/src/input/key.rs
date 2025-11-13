use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, IntoPrimitive, Hash, Default, Serialize, Deserialize)]
pub enum Key {
	#[default]
	Unknown = -1,
	A = 65, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
	Num0 = 48, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
	Space = 32,
	Apostrophe = 39,
	Comma = 44, Minus, Period, Slash,
	LeftBracket = 91, Backslash, RightBracket,
	Grave = 96,
	Semicolon = 59,
	Equal = 61,
	Escape = 256, Enter, Tab, Backspace, Insert, Delete,
	Right, Left, Down, Up,
	PageUp, PageDown,
	Home, End,
	CapsLock = 280, ScrollLock, NumLock,
	PrintScreen, Pause,
	F1 = 290, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
	F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24,
	LeftShift = 340, LeftControl, LeftAlt, LeftSuper,
	RightShift, RightControl, RightAlt, RightSuper,
	Menu
}
