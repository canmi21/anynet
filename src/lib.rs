/* src/lib.rs */

use fancy_log::{LogLevel, log};
use get_if_addrs::get_if_addrs;
use ip_lookup::get_public_ip_addr;
use std::net::IpAddr;

#[macro_export]
macro_rules! anynet {
    (port = $port:expr) => {
        $crate::_print_addresses($port, false);
    };
    (port = $port:expr, public = $public:expr) => {
        $crate::_print_addresses($port, $public);
    };
}

#[doc(hidden)]
pub fn _print_addresses(port: u16, public: bool) {
    if port == 0 {
        log(
            LogLevel::Error,
            "✗ Invalid port number: 0. Port must be between 1 and 65535.",
        );
        return;
    }

    log(
        LogLevel::Info,
        &format!("✓ Listening on http://localhost:{}", port),
    );

    let all_ips: Vec<IpAddr> = get_if_addrs()
        .map(|interfaces| {
            interfaces
                .into_iter()
                .filter(|iface| !iface.addr.ip().is_loopback())
                .map(|iface| iface.addr.ip())
                .collect()
        })
        .unwrap_or_default();

    if !all_ips.is_empty() {
        let mut sorted_ips = all_ips;
        sorted_ips.sort_by_key(|ip| match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                if octets[0] == 192 && octets[1] == 168 {
                    (0, ip.to_string())
                } else if octets[0] == 100 {
                    (1, ip.to_string())
                } else if octets[0] == 10 {
                    (2, ip.to_string())
                } else {
                    (3, ip.to_string())
                }
            }
            IpAddr::V6(_) => (4, ip.to_string()),
        });

        let display_limit = 2;
        let ips_to_display = &sorted_ips[..display_limit.min(sorted_ips.len())];
        let more_count = sorted_ips.len() - ips_to_display.len();

        for (index, ip_addr) in ips_to_display.iter().enumerate() {
            let url = match ip_addr {
                IpAddr::V4(ip) => format!("http://{}:{}", ip, port),
                IpAddr::V6(ip) => format!("http://[{}]:{}", ip, port),
            };

            let mut display_str = format!("✓ Listening on {}", url);
            if index == ips_to_display.len() - 1 && more_count > 0 {
                display_str.push_str(&format!(" +{} more", more_count));
            }

            log(LogLevel::Info, &display_str);
        }

        if more_count > 0 {
            let hidden_ips = &sorted_ips[display_limit..];
            for ip_addr in hidden_ips {
                let url = match ip_addr {
                    IpAddr::V4(ip) => format!("http://{}:{}", ip, port),
                    IpAddr::V6(ip) => format!("http://[{}]:{}", ip, port),
                };
                log(LogLevel::Debug, &format!("➜ Listening on {}", url));
            }
        }
    }

    if public {
        if let Some(ip) = get_public_ip_addr() {
            log(
                LogLevel::Info,
                &format!("• Possible Public Network: http://{}:{}", ip, port),
            );
        } else {
            log(LogLevel::Warn, "Could not determine public IP address.");
        }
    }
}
