use super::Options;

pub fn base_url(proto: &str, options: &Options) -> String {
    let scheme = if options.no_ssl {
        proto.to_string()
    } else {
        format!("{proto}s")
    };
    format!("{scheme}://{}", options.host)
}
