use crate::house::HousePermission;
use crate::pet::PetPermission;
use crate::provider::ProviderPermission;
use crate::user::UserPermission;
use enum_display::EnumDisplay;
use enum_iterator::{all, Sequence};

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, Sequence)]
#[enum_display(case = "Kebab")]
pub enum Resource {
    User(UserPermission),
    House(HousePermission),
    Pet(PetPermission),
    Provider(ProviderPermission),
}

impl Resource {
    pub fn get_action(&self) -> String {
        match self {
            Resource::User(permission) => permission.to_string(),
            Resource::House(house) => house.to_string(),
            Resource::Pet(pet) => pet.to_string(),
            Resource::Provider(provider) => provider.to_string(),
        }
    }

    pub fn get_resource(&self) -> String {
        self.to_string()
    }

    pub fn get_all_permissions() -> Vec<Resource> {
        vec![
            all::<UserPermission>()
                .map(|x| Resource::User(x))
                .collect::<Vec<Resource>>(),
            all::<HousePermission>()
                .map(|x| Resource::House(x))
                .collect::<Vec<Resource>>(),
            all::<PetPermission>()
                .map(|x| Resource::Pet(x))
                .collect::<Vec<Resource>>(),
            all::<ProviderPermission>()
                .map(|x| Resource::Provider(x))
                .collect::<Vec<Resource>>(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<Resource>>()
    }
}
