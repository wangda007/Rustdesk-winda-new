// libs/hbb_common/src/global_config.rs
// 全局服务器配置文件

/// 全局服务器配置常量
pub struct GlobalServerConfig;

impl GlobalServerConfig {
    /// ID 服务器地址
    pub const ID_SERVER: &'static str = "rustdesk.m-inai.com";
    /// 中继服务器地址
    pub const RELAY_SERVER: &'static str = "rustdesk.m-inai.com";
    /// API 服务器地址
    pub const API_SERVER: &'static str = "https://rustdesk.m-inai.com";
    /// 服务器公钥
    pub const SERVER_KEY: &'static str = "ccTm5i5tnaaqk2VRw0ZtQ6vcINWqA0ztgHgjNc21Nyo=";
}
