use crate::client::Client;
use crate::common::*;

use std::collections::HashMap;

use serde::Serialize;

impl Client {
    pub fn send(&self, req: &SendRequest) -> Result<()> {
        let token = self.get_access_token()?.access_token;
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/message/wxopen/template/send?access_token={}",
            &token
        );
        self.client
            .post(&url)
            .json(req)
            .send()?
            .json::<CommonResponse>()
            .map_err(From::from)
            .and_then(|resp| match resp.err_code {
                CommonResponse::SUCCESS => Ok(()),
                CommonResponse::BUSY => Err(Error::Busy),
                40037 => Err(Error::InvalidArgument {
                    field: "template_id".to_owned(),
                    msg: resp.err_msg,
                }),
                41028 => Err(Error::InvalidArgument {
                    field: "form_id".to_owned(),
                    msg: resp.err_msg,
                }),
                41029 => Err(Error::InvalidArgument {
                    field: "form_id".to_owned(),
                    msg: resp.err_msg,
                }),
                41030 => Err(Error::InvalidArgument {
                    field: "page".to_owned(),
                    msg: resp.err_msg,
                }),
                45009 => Err(Error::RateLimited(1_000_000, "day".to_owned())),
                _ => Err(Error::ServerError(resp.err_code, resp.err_msg)),
            })
    }
}

#[derive(Serialize)]
pub struct SendRequest<'a> {
    #[serde(rename = "touser")]
    pub to_user: &'a str,
    pub template_id: &'a str,
    pub page: Option<&'a str>,
    pub form_id: &'a str,
    pub data: HashMap<&'a str, HashMap<&'a str, &'a str>>,
    pub emphasis_keyword: Option<&'a str>,
}
