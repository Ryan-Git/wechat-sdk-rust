use crate::client::{AccessToken, Client};
use crate::common::*;

use chrono::prelude::*;
use serde::Deserialize;

impl Client {
    pub fn code2session(&self, js_code: &str) -> Result<Code2SessionResponse> {
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                          self.app_id, self.secret, js_code);
        self.client
            .get(&url)
            .send()?
            .json::<Code2SessionResponse>()
            .map_err(From::from)
            .and_then(|resp| match resp.common.err_code {
                CommonResponse::SUCCESS => Ok(resp),
                CommonResponse::BUSY => Err(Error::Busy),
                40029 => Err(Error::InvalidArgument {
                    field: "js_code".to_owned(),
                    msg: resp.common.err_msg,
                }),
                45011 => Err(Error::RateLimited(100, "minute".to_owned())),
                _ => Err(Error::ServerError(
                    resp.common.err_code,
                    resp.common.err_msg,
                )),
            })
    }

    pub fn set_access_token(&self, token: AccessToken) {
        let mut t = self.access_token.write().unwrap();
        *t = Some(token);
    }

    pub fn get_access_token(&self) -> Result<AccessTokenResponse> {
        let mut token = self.access_token.write().unwrap();
        if token.is_some() {
            let token = token.as_ref().unwrap();
            if Local::now().signed_duration_since(token.dt).num_seconds() < token.expires_in {
                return Ok(AccessTokenResponse {
                    access_token: token.access_token.clone(),
                    expires_in: token.expires_in,
                    common: CommonResponse::default(),
                });
            }
        }

        let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", self.app_id, self.secret);
        self.client
            .get(&url)
            .send()?
            .json::<AccessTokenResponse>()
            .map_err(From::from)
            .and_then(|resp| match resp.common.err_code {
                CommonResponse::SUCCESS => {
                    *token = Some(AccessToken {
                        access_token: resp.access_token.clone(),
                        expires_in: resp.expires_in,
                        dt: Local::now(),
                    });
                    Ok(resp)
                }
                CommonResponse::BUSY => Err(Error::Busy),
                40001 => Err(Error::InvalidArgument {
                    field: "secret".to_owned(),
                    msg: resp.common.err_msg,
                }),
                40002 => Err(Error::InvalidArgument {
                    field: "grant_type".to_owned(),
                    msg: resp.common.err_msg,
                }),
                40013 => Err(Error::InvalidArgument {
                    field: "app_id".to_owned(),
                    msg: resp.common.err_msg,
                }),
                _ => Err(Error::ServerError(
                    resp.common.err_code,
                    resp.common.err_msg,
                )),
            })
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Code2SessionResponse {
    #[serde(alias = "openid")]
    pub open_id: String,

    pub session_key: String,

    #[serde(alias = "unionid")]
    pub union_id: String,

    #[serde(flatten)]
    common: CommonResponse,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,

    #[serde(flatten)]
    common: CommonResponse,
}
