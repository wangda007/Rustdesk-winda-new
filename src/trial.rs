// 测试版的有效期控制：以本次构建日期（BUILD_DATE）为起点，TRIAL_DAYS 天后到期。
// 出正式版时只需把 TRIAL_DAYS 改成 0，这里的所有限制都会自动关闭。
use base::config::keys;
use chrono::{NaiveDateTime, TimeZone, Utc};
use hbb_common::{config::LocalConfig, get_time};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::OnceLock;

/// 试用天数（含构建当天）。0 表示正式版，不做任何限制。
pub const TRIAL_DAYS: i64 = 2;

/// 试用期间允许同时存在的远程控制连接数。
pub const MAX_REMOTE_CONNS: usize = 1;

const DAY_MS: i64 = 24 * 60 * 60 * 1000;
/// 记录程序见过的最大时间，防止把系统时间调小后一直用。
const KEY_MAX_SEEN: &str = "trial-max-seen";
const FLUSH_STEP_MS: i64 = 60 * 60 * 1000;

/// 试用期间不开放的功能，界面和会话权限都会关掉。
const DISABLED_OPTIONS: [&str; 2] = [
    keys::OPTION_ENABLE_FILE_TRANSFER,
    keys::OPTION_ENABLE_REMOTE_PRINTER,
];

// 这两句会发给控制端显示，直接用中文，避免对端没有我们的语言文件时显示英文。
pub const MSG_EXPIRED: &str = "测试版已到期，请联系供应商获取正式版";
pub const MSG_TOO_MANY_CONNS: &str = "测试版同一时间只允许一路连接";
// 本机界面提示，走语言文件翻译。
pub const MSG_DAYS_LEFT: &str = "Test build, {} days left";

static BASE_SEEN: OnceLock<i64> = OnceLock::new();
static LAST_FLUSH: AtomicI64 = AtomicI64::new(0);
// 连接里的消息循环每次都会检查到期，这里缓存到期时刻，避免反复解析构建时间。
static EXPIRE_MS: OnceLock<Option<i64>> = OnceLock::new();

/// 是否是受限制的测试版；无法解析构建时间时按不限制处理。
pub fn is_trial() -> bool {
    expire_ms().is_some()
}

fn expire_ms() -> Option<i64> {
    *EXPIRE_MS.get_or_init(|| {
        if TRIAL_DAYS <= 0 {
            return None;
        }
        let built = NaiveDateTime::parse_from_str(crate::BUILD_DATE, "%Y-%m-%d %H:%M").ok()?;
        let end = built.date().and_hms_opt(23, 59, 59)?;
        Some(Utc.from_utc_datetime(&end).timestamp_millis() + (TRIAL_DAYS - 1) * DAY_MS)
    })
}

/// 当前时间，取系统时间与历史最大值中的较大者，避免改系统时间续期。
fn now_ms() -> i64 {
    let now = get_time();
    let base = *BASE_SEEN.get_or_init(|| {
        LocalConfig::get_option(KEY_MAX_SEEN)
            .parse()
            .unwrap_or(0)
    });
    if now < base {
        return base;
    }
    let last = LAST_FLUSH.load(Ordering::Relaxed);
    if now - last >= FLUSH_STEP_MS {
        LAST_FLUSH.store(now, Ordering::Relaxed);
        LocalConfig::set_option(KEY_MAX_SEEN.to_owned(), now.to_string());
    }
    now
}

pub fn expired() -> bool {
    expire_ms().map_or(false, |expire| now_ms() >= expire)
}

/// 剩余天数，用于界面提示；不限制时返回 -1。
pub fn days_left() -> i64 {
    expire_ms().map_or(-1, |expire| {
        ((expire - now_ms() + DAY_MS - 1) / DAY_MS).max(0)
    })
}

/// 该功能选项在测试版是否被禁用。
pub fn option_disabled(key: &str) -> bool {
    is_trial() && DISABLED_OPTIONS.contains(&key)
}

/// 本机界面上的提示；正式版返回空串，界面据此不显示。
pub fn notice() -> String {
    if !is_trial() {
        return "".to_owned();
    }
    if expired() {
        translate(MSG_EXPIRED)
    } else {
        translate(MSG_DAYS_LEFT).replace("{}", &days_left().to_string())
    }
}

fn translate(key: &str) -> String {
    crate::lang::translate(key.to_owned())
}
