use cpal::traits::{DeviceTrait, HostTrait};

/// 利用可能なすべてのオーディオ出力デバイスの名前を取得する
pub fn available_output_device_names() -> Vec<String> {
    let host = cpal::default_host();
    let mut names = Vec::new();

    if let Ok(devices) = host.output_devices() {
        for device in devices {
            #[allow(deprecated)] // Fallback due to compilation issue with name() usage
            names.push(
                device
                    .name()
                    .unwrap_or_else(|_| "Unknown Device".to_string()),
            );
        }
    }

    names
}

/// システムのデフォルトオーディオ出力デバイスの名前を取得する
pub fn default_output_device_name() -> Option<String> {
    let host = cpal::default_host();
    #[allow(deprecated)] // Fallback due to compilation issue with name() usage
    host.default_output_device()
        .map(|dev| dev.name().unwrap_or_else(|_| "Unknown Device".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_functions_do_not_panic() {
        // CI環境などデバイスがない場合でもパニックしないことをテスト
        let _names = available_output_device_names();
        let _default = default_output_device_name();
    }
}
