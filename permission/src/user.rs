use enum_display::EnumDisplay;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, Sequence)]
#[enum_display(case = "Kebab")]
pub enum UserPermission {
    All,
    Read,
    Write,
    Delete,
    Update,
    Create,
    List,
}