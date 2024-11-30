use enum_display::EnumDisplay;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDisplay, Sequence)]
#[enum_display(case = "Kebab")]
pub enum ProviderPermission {
    All,
    Create,
    Read,
    Update,
    Delete,
}
