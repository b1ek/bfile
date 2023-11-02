use warp::{Filter, reply::Reply, reject::Rejection};

use crate::web::state::SharedState;

pub async fn help(state: SharedState) -> Result<String, Rejection> {

    let brand = format!(
        "{} \x1b[1m{}\x1b[0m  {}",
        state.config.brand.instance_emoji,
        state.config.brand.instance_name,
        {
            if state.config.brand.instance_name != "blek! File" {
                "\n\x1b[90mPowered by blek! File\x1b[0m"
            } else { "" }
        }
    );

    let mut warns: String = String::new();
    if ! state.config.api.curlapi {
        warns += "\x1b[1;31mWarning: curl API is disabled on this instance.\nYou can use the web UI to upload files.\x1b[0m\n\n"
    }
    if ! state.config.files.allow_uploads {
        warns += {
            format!(
                "\x1b[1;31mWarning: all uploads are disabled on this instance{}\x1b[0m",
                {
                    if let Some(reason) = state.config.files.upload_disable_reason {
                        format!(" for this reason:\n\"{}\"", reason)
                    } else { ".".to_string() }
                }
            ).as_str()
        }
    }

    let instance = state.env.instanceurl;
    let help =
format!(
"To upload a new file, you can POST it like this:
    \x1b[32mcurl\x1b[0m \x1b[33m-X POST\x1b[0m \x1b[34m{instance}/curlapi/upload\x1b[0m \x1b[33m-F'file=@file.txt'\x1b[0m \x1b[33m-F'tos_consent=on'\x1b[0m
You can also add a password:
    \x1b[32mcurl\x1b[0m \x1b[33m-X POST\x1b[0m \x1b[34m{instance}/curlapi/upload\x1b[0m \x1b[33m-F'file=@file.txt'\x1b[0m \x1b[33m-F'filename=uwu'\x1b[0m \x1b[33m-F'tos_consent=on'\x1b[0m \x1b[33m-F'named=on'\x1b[0m
The `named=on` switch is neede because this API is basically
the HTML used at the regular web UI form wrapped into this URL

\x1b[1;32mIMPORTANT:\x1b[0m Read the terms of service \x1b[1mbefore\x1b[0m uploading the file!
The ToS can be found here: \x1b[34m{instance}/tos\x1b[0m .

{warns}
"
);

    Ok(format!("
  \x1b[31m┓ ╻\x1b[0m \x1b[35m┏┓•┓  \x1b[0m
  \x1b[32m┣┓┃\x1b[0m \x1b[95m┣ ┓┃┏┓\x1b[0m
  \x1b[34m┗┛•\x1b[0m \x1b[35m┻ ┗┗┗━\x1b[0m

{brand}

{help}
").into())
}

pub fn get_routes(state: SharedState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::any()
        .and(warp::path!("curlapi" / "help"))
        .and(
            warp::any()
                .map(move || state.clone())
        )
        .and_then(help)
}