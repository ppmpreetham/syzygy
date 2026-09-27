pub mod sidebar;
pub mod titlebar;
pub mod content;

use strum_macros::FromRepr;

#[derive(Clone, Copy, PartialEq, Eq, FromRepr)]
#[repr(usize)]
pub enum AppTab {
    Dashboard = 0,
    Target = 1,
    Proxy = 2,
    Intruder = 3,
    Repeater = 4,
    Decoder = 5,
    Comparer = 6,
    Logger = 7,
    Organizer = 8,
    Extensions = 9,
}
