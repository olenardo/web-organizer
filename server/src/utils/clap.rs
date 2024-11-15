use std::{net::IpAddr, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
pub struct Opt {
    /// Bind an Ip address (IPv4/IPv6)
    #[clap(long = "ip", default_value = "0.0.0.0")]
    pub ip: IpAddr,

    /// Bind a port
    #[clap(long = "port", default_value = "5000")]
    pub port: u16,

    /// Flag to enable logs
    #[clap(long = "log", default_value = "false")]
    pub log: bool,

    
    // !!! Server can run with a certificate (HTTPS mode) and without (HTTP mode). 

    /// Path to the cert.pem file (OPTIONAL)
    #[clap(long = "cert")]
    pub cert: Option<PathBuf>,

    /// Path to the key.pem file (OPTIONAL)
    #[clap(long = "key")]
    pub key: Option<PathBuf>,
}