use clap::{Parser, Subcommand, ValueEnum};
use url::Url;

//
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: ArgsSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum ArgsSubcommand {
    Http(HttpArgs),
}

//
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct HttpArgs {
    #[clap(flatten)]
    pub _common: XArgsCommon,

    #[arg(long)]
    pub list_url: Option<Url>,
    #[arg(long)]
    pub host_url: Option<Url>,

    #[arg(long)]
    pub access_token: String,
    #[arg(long, value_enum)]
    pub access_token_in: Option<HttpArgsAccessTokenIn>,
    #[arg(long)]
    pub access_token_query_name: Option<String>,

    #[arg(long, value_enum)]
    pub hostname_in: Option<HttpArgsHostnameIn>,
    #[arg(long)]
    pub hostname_query_name: Option<String>,
}

//
// https://docs.ansible.com/ansible/latest/cli/ansible-inventory.html
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct XArgsCommon {
    #[arg(long)]
    pub list: bool,
    #[arg(long)]
    pub host: Option<String>,
}

//
#[derive(ValueEnum, Debug, Clone)]
pub enum HttpArgsAccessTokenIn {
    HeaderAuthorizationBearer,
    Query,
}
impl Default for HttpArgsAccessTokenIn {
    fn default() -> Self {
        Self::HeaderAuthorizationBearer
    }
}

//
#[derive(ValueEnum, Debug, Clone)]
pub enum HttpArgsHostnameIn {
    Path,
    Query,
}
impl Default for HttpArgsHostnameIn {
    fn default() -> Self {
        Self::Path
    }
}
