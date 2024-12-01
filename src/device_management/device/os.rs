#[derive(Debug, Clone, PartialEq, Default)]
pub struct OS {
    ios_ver: String,
    build_num: String,
}

impl OS {
    pub fn new(ios_ver: impl Into<String>, build_num: impl Into<String>) -> Self {
        Self {
            ios_ver: ios_ver.into(),
            build_num: build_num.into(),
        }
    }

    pub fn get_ios_ver(&self) -> &str {
        &self.ios_ver
    }

    pub fn get_build_num(&self) -> &str {
        &self.build_num
    }
}
