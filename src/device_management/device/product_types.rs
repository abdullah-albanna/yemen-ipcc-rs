use regex::Regex;
use std::{fmt::Display, str::FromStr};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum ProductType {
    #[default]
    Unknown,

    // iPhone 13 Series
    iPhone13ProMax,
    iPhone13Pro,
    iPhone13,
    iPhone13Mini,

    // iPhone 12 Series
    iPhone12ProMax,
    iPhone12Pro,
    iPhone12,
    iPhone12Mini,

    // iPhone 11 Series
    iPhone11ProMax,
    iPhone11Pro,
    iPhone11,

    // iPhone X Series
    iPhoneXSMax,
    iPhoneXS,
    iPhoneXR,
    iPhoneX,

    // iPhone 8 Series
    iPhone8Plus,
    iPhone8,

    // iPhone 7 Series
    iPhone7Plus,
    iPhone7,

    // iPhone 6 Series
    iPhone6SPlus,
    iPhone6S,
    iPhone6Plus,
    iPhone6,

    // iPhone SE Series
    iPhoneSE3rdGen,
    iPhoneSE2ndGen,
    iPhoneSE1stGen,
}

impl Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_text = match *self {
            ProductType::Unknown => "Unknown".to_string(),

            ProductType::iPhone13ProMax => "iPhone 13 Pro Max".to_string(),
            ProductType::iPhone13Pro => "iPhone 13 Pro".to_string(),
            ProductType::iPhone13 => "iPhone 13".to_string(),
            ProductType::iPhone13Mini => "iPhone 13 Mini".to_string(),

            ProductType::iPhone12ProMax => "iPhone 12 Pro Max".to_string(),
            ProductType::iPhone12Pro => "iPhone 12 Pro".to_string(),
            ProductType::iPhone12 => "iPhone 12".to_string(),
            ProductType::iPhone12Mini => "iPhone 12 Mini".to_string(),

            ProductType::iPhone11ProMax => "iPhone 11 Pro Max".to_string(),
            ProductType::iPhone11Pro => "iPhone 11 Pro".to_string(),
            ProductType::iPhone11 => "iPhone 11".to_string(),

            ProductType::iPhoneXSMax => "iPhone XS Max".to_string(),
            ProductType::iPhoneXS => "iPhone XS".to_string(),
            ProductType::iPhoneXR => "iPhone XR".to_string(),
            ProductType::iPhoneX => "iPhone X".to_string(),

            ProductType::iPhone8Plus => "iPhone 8 Plus".to_string(),
            ProductType::iPhone8 => "iPhone 8".to_string(),

            ProductType::iPhone7Plus => "iPhone 7 Plus".to_string(),
            ProductType::iPhone7 => "iPhone 7".to_string(),

            ProductType::iPhone6SPlus => "iPhone 6S Plus".to_string(),
            ProductType::iPhone6S => "iPhone 6S".to_string(),
            ProductType::iPhone6Plus => "iPhone 6 Plus".to_string(),
            ProductType::iPhone6 => "iPhone 6".to_string(),

            ProductType::iPhoneSE3rdGen => "iPhone SE (3rd Gen)".to_string(),
            ProductType::iPhoneSE2ndGen => "iPhone SE (2nd Gen)".to_string(),
            ProductType::iPhoneSE1stGen => "iPhone SE (1st Gen)".to_string(),
        };

        write!(f, "{}", display_text)
    }
}

//fn get_display_product_type(model_text: &str) -> String {
//    let pattern = r"(i?)[A-Z\d]([^\W\d]*|E|S|R?)";
//    let re = Regex::new(pattern).unwrap();
//
//    let model_iter = re.find_iter(model_text);
//    let mut display_model = String::new();
//
//    for cap in model_iter {
//        display_model.push_str(cap.as_str());
//    }
//
//    display_model
//}

impl FromStr for ProductType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.to_lowercase().replace(' ', "");

        match normalized.as_str() {
            // iPhone 6 series
            "iphone6" | "ip6" | "6" => Ok(ProductType::iPhone6),
            "iphone6plus" | "ip6plus" | "6plus" | "6+" => Ok(ProductType::iPhone6Plus),
            "iphone6s" | "ip6s" | "6s" => Ok(ProductType::iPhone6S),
            "iphone6splus" | "ip6splus" | "6splus" | "6s+" => Ok(ProductType::iPhone6SPlus),

            // iPhone 7 series
            "iphone7" | "ip7" | "7" => Ok(ProductType::iPhone7),
            "iphone7plus" | "ip7plus" | "7plus" | "7+" => Ok(ProductType::iPhone7Plus),

            // iPhone 8 series
            "iphone8" | "ip8" | "8" => Ok(ProductType::iPhone8),
            "iphone8plus" | "ip8plus" | "8plus" | "8+" => Ok(ProductType::iPhone8Plus),

            // iPhone X series
            "iphonex" | "ipx" | "x" => Ok(ProductType::iPhoneX),
            "iphonexr" | "ipxr" | "xr" => Ok(ProductType::iPhoneXR),
            "iphonexs" | "ipxs" | "xs" => Ok(ProductType::iPhoneXS),
            "iphonexsmax" | "ipxsmax" | "xsmax" => Ok(ProductType::iPhoneXSMax),

            // iPhone 11 series
            "iphone11" | "ip11" | "11" => Ok(ProductType::iPhone11),
            "iphone11pro" | "ip11pro" | "11pro" => Ok(ProductType::iPhone11Pro),
            "iphone11promax" | "ip11promax" | "11promax" => Ok(ProductType::iPhone11ProMax),

            // iPhone 12 series
            "iphone12" | "ip12" | "12" => Ok(ProductType::iPhone12),
            "iphone12mini" | "ip12mini" | "12mini" => Ok(ProductType::iPhone12Mini),
            "iphone12pro" | "ip12pro" | "12pro" => Ok(ProductType::iPhone12Pro),
            "iphone12promax" | "ip12promax" | "12promax" => Ok(ProductType::iPhone12ProMax),

            // iPhone 13 series
            "iphone13" | "ip13" | "13" => Ok(ProductType::iPhone13),
            "iphone13mini" | "ip13mini" | "13mini" => Ok(ProductType::iPhone13Mini),
            "iphone13pro" | "ip13pro" | "13pro" => Ok(ProductType::iPhone13Pro),
            "iphone13promax" | "ip13promax" | "13promax" => Ok(ProductType::iPhone13ProMax),

            // iPhone SE series
            "iphonese1stgen" | "ipse1" | "se1" => Ok(ProductType::iPhoneSE1stGen),
            "iphonese2ndgen" | "ipse2" | "se2" => Ok(ProductType::iPhoneSE2ndGen),
            "iphonese3rdgen" | "ipse3" | "se3" => Ok(ProductType::iPhoneSE3rdGen),

            _ => Err(()), // Error for unrecognized patterns
        }
    }
}
