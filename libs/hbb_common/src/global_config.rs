// libs/hbb_common/src/global_config.rs
// 全局服务器配置文件

/// 全局服务器配置常量
pub struct GlobalServerConfig;

impl GlobalServerConfig {
    /// ID 服务器地址
    pub const ID_SERVER: &'static str = "ks.mdolink.com:21116";
    /// 中继服务器地址
    pub const RELAY_SERVER: &'static str = "ks.mdolink.com:21117";
    /// API 服务器地址
    pub const API_SERVER: &'static str = "http://ks.mdolink.com:21114";
    /// 服务器公钥
    pub const SERVER_KEY: &'static str = "j9KLXIAqPG850mEB8GRZMAdRzffMawdwqPEC15OciRg=";
}
