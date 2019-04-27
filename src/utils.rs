use crate::kube;
use clap::ArgMatches;
use std::io::{Error, ErrorKind};

/// Helper func to convert a vector of &str to a vector of Strings
pub fn str_to_string(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|&x| x.to_string()).collect()
}

/// Parse the cli args, and return the kube::LogRecorderConfig
pub fn set_args(args: &ArgMatches) -> Result<kube::LogRecorderConfig, Box<::std::error::Error>> {
    let namespace = args.value_of("NAMESPACE").unwrap();
    let kube_config = if let Some(kube_config) = args.value_of("KUBECONFIG") {
        kube_config
    } else {
        ""
    };

    let file = args.is_present("FILE");
    let color = args.is_present("COLOR");

    Ok(kube::LogRecorderConfig::new(
        namespace.to_string(),
        kube_config.to_string(),
        file,
        color,
    ))
}

/// Since Command returns stdout or stderr attrs instead of actual errors, we need a helper
/// function to generate custom errors when dealing with Command.
pub fn generate_err(err_msg: String) -> Result<(), Box<::std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, err_msg)))
}
