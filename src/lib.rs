pub trait FromRow {
    fn from_row(row_values: Vec<(String, String, String)>) -> Self;
}

#[derive(Debug)]
pub enum RibbitFlags {
    Versions,
    CDNS,
    BGDL,
    Unknown,
}

// impl default
impl Default for RibbitFlags {
    fn default() -> Self {
        Self::Unknown
    }
}

impl From<String> for RibbitFlags {
    fn from(flag: String) -> Self {
        match flag.as_str() {
            "versions" => Self::Versions,
            "cdn" => Self::CDNS,
            "bgdl" => Self::BGDL,
            _ => Self::Versions,
        }
    }
}
impl std::fmt::Display for RibbitFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RibbitFlags::Versions => write!(f, "Versions"),
            RibbitFlags::CDNS => write!(f, "CDNS"),
            RibbitFlags::BGDL => write!(f, "BGDL"),
            RibbitFlags::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Default)]
pub struct SummaryObject {
    pub product: String,
    pub seqn: i32,
    pub flags: RibbitFlags,
}

impl FromRow for SummaryObject {
    fn from_row(row_values: Vec<(String, String, String)>) -> Self {
        let mut s = Self::default();

        for (key, _, value) in row_values {
            match key.as_str() {
                "Product" => s.product = value,
                "Seqn" => s.seqn = value.parse().unwrap_or_default(),
                "Flags" => s.flags = value.into(),
                _ => {},
            }
        }

        s
    }
}

#[derive(Debug, Default)]
pub struct CdnObject {
    pub name: String,
    pub path: String,
    pub hosts: Vec<String>,
    pub servers: Vec<String>,
    pub config_path: String,
}

impl FromRow for CdnObject {
    fn from_row(row_values: Vec<(String, String, String)>) -> Self {
        let mut s = Self::default();

        for (key, _, value) in row_values {
            match key.as_str() {
                "Name" => s.name = value,
                "Path" => s.path = value,
                "Hosts" => s.hosts = value.split(' ').map(|s| s.to_string()).collect(),
                "Servers" => s.servers = value.split(' ').map(|s| s.to_string()).collect(),
                "ConfigPath" => s.config_path = value,
                _ => {},
            }
        }

        s
    }
}

#[derive(Debug, Default)]
pub struct VersionObject {
    pub region: String,
    pub build_config: String,
    pub cdn_config: String,
    pub key_ring: String,
    pub build_id: i32,
    pub version_name: String,
    pub product_config: String,
}

impl FromRow for VersionObject {
    fn from_row(row_values: Vec<(String, String, String)>) -> Self {
        let mut s = Self::default();

        for (key, _, value) in row_values {
            match key.as_str() {
                "Region" => s.region = value,
                "BuildConfig" => s.build_config = value,
                "CDNConfig" => s.cdn_config = value,
                "KeyRing" => s.key_ring = value,
                "BuildId" => s.build_id = value.parse().unwrap_or_default(),
                "VersionsName" => s.version_name = value,
                "ProductConfig" => s.product_config = value,
                _ => {},
            }
        }

        s
    }
}

#[derive(Debug, Default)]
pub struct BgdlObject {
    pub region: String,
    pub build_config: String,
    pub cdn_config: String,
    pub key_ring: String,
    pub build_id: i32,
    pub version_name: String,
    pub product_config: String,
}

impl FromRow for BgdlObject {
    fn from_row(row_values: Vec<(String, String, String)>) -> Self {
        let mut s = Self::default();

        for (key, _, value) in row_values {
            match key.as_str() {
                "Region" => s.region = value,
                "BuildConfig" => s.build_config = value,
                "CDNConfig" => s.cdn_config = value,
                "KeyRing" => s.key_ring = value,
                "BuildId" => s.build_id = value.parse().unwrap_or_default(),
                "VersionsName" => s.version_name = value,
                "ProductConfig" => s.product_config = value,
                _ => {},
            }
        }

        s
    }
}


