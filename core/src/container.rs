use std::collection::HashMap;
use crate::model::types::ResourceType;
use crate::rcm::manager::Manager;

pub struct RCMContainer {
    managers: HashMap<ResourceType, Manager>
}

impl RCMContainer {
    #[must_use]
    pub const fn put_manager(resource_type: ResourceType, manager: Manager) {
        Self::managers.insert(resource_type, manager)
    }

    pub const fn fetch_manager(resource_type: ResourceType) -> Manager {
        Self::managers.get(resource_type)
    }
}