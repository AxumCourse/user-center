use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Web配置
    pub web: WebConfig,
    /// 数据库
    pub db: DbConfig,
    // /// 消息队列
    // pub mq: MqConfig,
    // /// 邮件发送
    // pub smtp: SmtpConfig,
}

/// Web配置
#[derive(Serialize, Deserialize)]
pub struct WebConfig {
    /// 监听地址
    pub addr: String,
}
/// 数据库配置
#[derive(Serialize, Deserialize)]
pub struct DbConfig {
    /// 数据库连接信息
    pub dsn: String,
}
/// 消息队列配置
#[derive(Serialize, Deserialize)]
pub struct MqConfig {}

/// 邮件发送配置
#[derive(Serialize, Deserialize)]
pub struct SmtpConfig {}

impl Config {
    /// 从环境变量中初始化配置
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
