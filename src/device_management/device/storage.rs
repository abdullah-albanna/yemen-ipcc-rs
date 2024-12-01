use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct DeviceStorage {
    total_storage: u16,
    used_storage: u16,
}

impl Display for DeviceStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
        Total Storage: {}GB
        Used Storage: {}GB,
        Availabel Storage: {}GB
        "#,
            self.total_storage,
            self.used_storage,
            (self.total_storage - self.used_storage)
        )
    }
}

impl DeviceStorage {
    pub fn new(total_storage: u16, used_storage: u16) -> Self {
        Self {
            total_storage,
            used_storage,
        }
    }

    pub fn get_total_storage(&self) -> String {
        format!("{}GB", self.total_storage)
    }

    pub fn get_used_storage(&self) -> String {
        format!("{}GB", self.used_storage)
    }

    pub fn get_availabel_storage(&self) -> String {
        format!("{}GB", (self.total_storage - self.used_storage))
    }
}
