use std::{env, fs, path::PathBuf};

#[derive(Debug, PartialEq)]
pub enum Platform {
    Windows,
    Unixlike,
}

pub struct AppDirs {
    platform: Platform,
    app_name: String,
}

/// 应用目录管理
/// Macos 和 Linux 遵循 XDG 规范
/// Windows 放在程序的目录下
impl AppDirs {
    pub fn from(app_name: String) -> Self {
        let platform = Self::detect_platform();
        AppDirs {
            platform,
            app_name: app_name,
        }
    }

    pub fn from_target_platform(platform: Platform, app_name: String) -> Self {
        AppDirs {
            platform,
            app_name: app_name,
        }
    }

    fn detect_platform() -> Platform {
        if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Unixlike
        }
    }

    fn get_base_path() -> PathBuf {
        // 获取exe所在目录
        let exe_path = std::env::current_exe().expect("Faild to get exe path");
        let base_path = exe_path.parent().unwrap().to_path_buf();

        // 不存在时创建
        if !base_path.exists() {
            fs::create_dir_all(&base_path).expect("Faild to create base path");
        }

        base_path
    }

    fn get_home_path() -> PathBuf {
        env::home_dir().expect("Faild to get home dir")
    }

    pub fn config_dir(&self) -> PathBuf {
        let config_path = match self.platform {
            Platform::Windows => AppDirs::get_base_path().join("config"),
            Platform::Unixlike => AppDirs::get_home_path()
                .join(".config")
                .join(&self.app_name),
        };

        if !config_path.exists() {
            fs::create_dir_all(&config_path).expect("Faild to create config path");
        }

        config_path
    }

    pub fn data_dir(&self) -> PathBuf {
        let data_path = match self.platform {
            Platform::Windows => AppDirs::get_base_path().join("data"),
            Platform::Unixlike => AppDirs::get_home_path()
                .join(".local")
                .join("share")
                .join(&self.app_name),
        };

        if !data_path.exists() {
            fs::create_dir_all(&data_path).expect("Faild to create data path");
        }

        data_path
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn check_config_path_windows() {
        let app_dirs = AppDirs::from_target_platform(Platform::Windows, "brush".to_string());
        let config_path = app_dirs.config_dir();
        println!("{}", config_path.to_str().unwrap());
    }

    #[test]
    pub fn check_config_path_macos() {
        let app_dirs = AppDirs::from_target_platform(Platform::Unixlike, "brush".to_string());
        let config_path = app_dirs.config_dir();
        println!("{}", config_path.to_str().unwrap());
    }
}
