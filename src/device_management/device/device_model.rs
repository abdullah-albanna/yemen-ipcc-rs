use super::ProductType;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeviceModel {
    model: ProductType,
    serial: String,
}

impl DeviceModel {
    pub fn new(model: ProductType, serial: impl Into<String>) -> Self {
        Self {
            model,
            serial: serial.into(),
        }
    }

    pub fn get_model(&self) -> ProductType {
        self.model
    }

    pub fn get_serial(&self) -> &str {
        &self.serial
    }
}
