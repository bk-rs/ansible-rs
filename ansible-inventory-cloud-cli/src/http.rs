use core::time::Duration;

use ansible_inventory::script_output::{Host, List};
use isahc::{
    config::Configurable as _, http::header::AUTHORIZATION, AsyncReadResponseExt as _, Request,
    RequestExt as _,
};

use crate::args::{HttpArgs, HttpArgsAccessTokenIn, HttpArgsHostnameIn};

//
pub async fn run(args: HttpArgs) -> Result<(), Box<dyn std::error::Error>> {
    let (url, header_authorization_bearer) = if args._common.list {
        let mut list_url = args.list_url.ok_or("args list_url missing")?;
        let mut header_authorization_bearer = None;

        match args.access_token_in.unwrap_or_default() {
            HttpArgsAccessTokenIn::HeaderAuthorizationBearer => {
                header_authorization_bearer = Some(args.access_token.to_owned());
            }
            HttpArgsAccessTokenIn::Query => {
                let access_token_query_name = args
                    .access_token_query_name
                    .as_deref()
                    .unwrap_or("access_token");
                list_url
                    .query_pairs_mut()
                    .append_pair(&access_token_query_name, &args.access_token);
            }
        }

        (list_url, header_authorization_bearer)
    } else if let Some(hostname) = &args._common.host {
        let mut host_url = args.host_url.ok_or("args host_url missing")?;
        let mut header_authorization_bearer = None;

        match args.access_token_in.unwrap_or_default() {
            HttpArgsAccessTokenIn::HeaderAuthorizationBearer => {
                header_authorization_bearer = Some(args.access_token.to_owned());
            }
            HttpArgsAccessTokenIn::Query => {
                let access_token_query_name = args
                    .access_token_query_name
                    .as_deref()
                    .unwrap_or("access_token");
                host_url
                    .query_pairs_mut()
                    .append_pair(&access_token_query_name, &args.access_token);
            }
        }

        match args.hostname_in.unwrap_or_default() {
            HttpArgsHostnameIn::Path => {
                host_url
                    .path_segments_mut()
                    .map_err(|_| "args host_url invalid".to_string())?
                    .push(hostname);
            }
            HttpArgsHostnameIn::Query => {
                let hostname_query_name = args.hostname_query_name.as_deref().unwrap_or("name");
                host_url
                    .query_pairs_mut()
                    .append_pair(&hostname_query_name, &hostname);
            }
        }

        (host_url, header_authorization_bearer)
    } else {
        return Err(
            "Usage: `ansible-inventory-cloud-cli --list` or `ansible-inventory-cloud-cli --host xxx`".into()
        );
    };

    //
    let req_builder = Request::get(url.as_str());
    let req_builder = if let Some(header_authorization_bearer) = header_authorization_bearer {
        req_builder.header(
            AUTHORIZATION,
            format!("Bearer {header_authorization_bearer}"),
        )
    } else {
        req_builder
    };

    let mut resp = req_builder
        .timeout(Duration::from_secs(10))
        .body(())
        .map_err(|err| format!("unreachable, request build failed, err:{err}"))?
        .send_async()
        .await
        .map_err(|err| format!("request send failed, err:{err}"))?;

    if !resp.status().is_success() {
        return Err(format!(
            "response status not success, status:{}",
            resp.status().as_u16()
        )
        .into());
    }

    let resp_bytes = resp
        .bytes()
        .await
        .map_err(|err| format!("response body read failed, err:{err}"))?;

    if args._common.list {
        let list = serde_json::from_slice::<List>(&resp_bytes)
            .map_err(|err| format!("response body to List failed, err:{err}"))?;

        match serde_json::to_string_pretty(&list) {
            Ok(s) => println!("{s}"),
            Err(_err) => {
                println!("{}", String::from_utf8_lossy(&resp_bytes))
            }
        }
    } else if let Some(_hostname) = &args._common.host {
        let host = serde_json::from_slice::<Host>(&resp_bytes)
            .map_err(|err| format!("response body to Host failed, err:{err}"))?;

        match serde_json::to_string_pretty(&host) {
            Ok(s) => println!("{s}"),
            Err(_err) => {
                println!("{}", String::from_utf8_lossy(&resp_bytes))
            }
        }
    } else {
        unreachable!()
    }

    Ok(())
}
