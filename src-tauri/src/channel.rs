use sasuke::channel::RELEASE_CHANNEL;
use sasuke::storage::StoragePathConfig;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DesktopChannelConfig {
    pub channel: &'static str,
    pub app_name: &'static str,
    pub app_key: &'static str,
    pub config_dir_name: &'static str,
    pub home_env_var: &'static str,
    pub updater_endpoint: &'static str,
    pub updater_public_key: &'static str,
    pub allow_http_updater: bool,
    pub metrics_enabled: bool,
    pub feedback_enabled: bool,
    pub metrics_toggle_locked: bool,
    pub metrics_base_url: &'static str,
    pub metrics_api_key: &'static str,
    pub silent_update_enabled: bool,
    pub builtin_mcp_servers_json: &'static str,
    pub multica_enabled: bool,
    pub multica_toggle_locked: bool,
    pub multica_base_url: &'static str,
    pub multica_app_url: &'static str,
}

pub fn current_channel_config() -> DesktopChannelConfig {
    let config = DesktopChannelConfig {
        channel: RELEASE_CHANNEL,
        app_name: option_env!("SASUKE_APP_NAME").unwrap_or("sasuke"),
        app_key: option_env!("SASUKE_APP_KEY").unwrap_or("sasuke"),
        config_dir_name: option_env!("SASUKE_CONFIG_DIR_NAME").unwrap_or(".sasuke"),
        home_env_var: option_env!("SASUKE_HOME_ENV_VAR").unwrap_or("SASUKE_HOME"),
        updater_endpoint: option_env!("SASUKE_UPDATER_ENDPOINT")
            .unwrap_or("https://github.com/klauswg/sasuke/releases/latest/download/latest.json"),
        updater_public_key: option_env!("SASUKE_UPDATER_PUBLIC_KEY").unwrap_or("dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IEYwQkQwNjYyMTA0MjdDQ0IKUldUTGZFSVFZZ2E5OEN3QnY2eHRkM0xVRnlreC9UMFNpSWdXSC9oK0ZWMlpsWXpuZ0hhbEFnWGQK"),
        allow_http_updater: option_env!("SASUKE_ALLOW_HTTP_UPDATER") == Some("true"),
        metrics_enabled: option_env!("SASUKE_METRICS_ENABLED") == Some("true"),
        feedback_enabled: option_env!("SASUKE_FEEDBACK_ENABLED") == Some("true"),
        metrics_toggle_locked: option_env!("SASUKE_METRICS_TOGGLE_LOCKED") == Some("true"),
        metrics_base_url: option_env!("SASUKE_METRICS_BASE_URL").unwrap_or(""),
        metrics_api_key: option_env!("SASUKE_METRICS_API_KEY").unwrap_or(""),
        silent_update_enabled: option_env!("SASUKE_SILENT_UPDATE_ENABLED") == Some("true"),
        builtin_mcp_servers_json: option_env!("SASUKE_BUILTIN_MCP_SERVERS").unwrap_or("[]"),
        multica_enabled: option_env!("SASUKE_MULTICA_ENABLED") == Some("true"),
        multica_toggle_locked: option_env!("SASUKE_MULTICA_TOGGLE_LOCKED") == Some("true"),
        multica_base_url: option_env!("SASUKE_MULTICA_BASE_URL").unwrap_or(""),
        multica_app_url: option_env!("SASUKE_MULTICA_APP_URL").unwrap_or(""),
    };
    config
}

pub fn storage_path_config() -> StoragePathConfig {
    let config = current_channel_config();
    StoragePathConfig {
        app_key: config.app_key,
        config_dir_name: config.config_dir_name,
        home_env_var: config.home_env_var,
    }
}

#[cfg(test)]
mod tests {
    /// 桌面渠道身份取自 core crate 的编译期常量；构建脚本注入值来自 `configs/channels/<channel>.json`
    /// 的校验结果，两者必须一致，否则内置能力目录会与桌面渠道身份分叉。
    #[test]
    fn desktop_channel_matches_core_release_channel() {
        assert_eq!(
            option_env!("SASUKE_RELEASE_CHANNEL").unwrap_or("default"),
            sasuke::channel::RELEASE_CHANNEL
        );
    }
}
