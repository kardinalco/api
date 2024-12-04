use enum_display::EnumDisplay;
use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumDisplay, Sequence)]
#[enum_display(case = "Kebab")]
pub enum HousePermission {
    All,
    Create,
    List,
    ReadSelf,
    Read,
    UpdateSelf,
    Update,
    DeleteSelf,
    Delete,
    Invite,
    InviteSelf,
    Revoke,
    RevokeSelf,
    ListInvitationSelf,
    ListInvitation,
    AcceptInvitation,
    AcceptInvitationSelf,
    DeclineInvitation,
    DeclineInvitationSelf,
}
