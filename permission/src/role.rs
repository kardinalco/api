use enum_display::EnumDisplay;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDisplay)]
#[enum_display(case = "Kebab")]
pub enum RolePermission {
    All,
    Create,
    Read,
    Update,
    Delete,
}
