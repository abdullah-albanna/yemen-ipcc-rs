pub mod battery;
pub mod device_model;
pub mod os;
pub mod product_types;
pub mod storage;

pub use battery::DeviceBattery;
pub use device_model::DeviceModel;
pub use os::OS;
pub use product_types::ProductType;
pub use storage::DeviceStorage;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum DeviceStatus {
    Connected,
    #[default]
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DeviceInfo {
    pub device: DeviceModel,
    pub os: OS,
    pub battery: DeviceBattery,
    pub storage: DeviceStorage,
    pub status: DeviceStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_type_parsing() {
        // Test valid product types with various formats (spaces, case, and abbreviations)
        assert_eq!(
            "iphone6plus".parse::<ProductType>().unwrap(),
            ProductType::iPhone6Plus
        );
        assert_eq!(
            "6plus".parse::<ProductType>().unwrap(),
            ProductType::iPhone6Plus
        );
        assert_eq!(
            "iphoneX".parse::<ProductType>().unwrap(),
            ProductType::iPhoneX
        );
        assert_eq!(
            "iphonex".parse::<ProductType>().unwrap(),
            ProductType::iPhoneX
        );
        assert_eq!(
            "ipxsmax".parse::<ProductType>().unwrap(),
            ProductType::iPhoneXSMax
        );
        assert_eq!(
            "iphone11ProMax".parse::<ProductType>().unwrap(),
            ProductType::iPhone11ProMax
        );
        assert_eq!(
            "13promax".parse::<ProductType>().unwrap(),
            ProductType::iPhone13ProMax
        );
        assert_eq!(
            "12 mini".parse::<ProductType>().unwrap(),
            ProductType::iPhone12Mini
        );

        // Test invalid product types (returning Err)
        assert!("notiphone".parse::<ProductType>().is_err());
        assert!("123".parse::<ProductType>().is_err());
    }

    #[test]
    fn test_product_type_case_insensitivity() {
        // Test case insensitivity
        assert_eq!(
            "IPHONE13PRO".parse::<ProductType>().unwrap(),
            ProductType::iPhone13Pro
        );
        assert_eq!(
            "IpHoNe6".parse::<ProductType>().unwrap(),
            ProductType::iPhone6
        );
        assert_eq!(
            "IPHONE11pro".parse::<ProductType>().unwrap(),
            ProductType::iPhone11Pro
        );
    }

    #[test]
    fn test_product_type_spaces() {
        // Test handling spaces
        assert_eq!(
            " iPhone 7 Plus ".parse::<ProductType>().unwrap(),
            ProductType::iPhone7Plus
        );
        assert_eq!(
            " ip 8 Plus ".parse::<ProductType>().unwrap(),
            ProductType::iPhone8Plus
        );
        assert_eq!(
            " iphone 12 pro ".parse::<ProductType>().unwrap(),
            ProductType::iPhone12Pro
        );
    }

    #[test]
    fn test_invalid_product_type() {
        // Test invalid cases
        let invalid = "nonexistent".parse::<ProductType>();
        assert!(invalid.is_err());
    }
}
