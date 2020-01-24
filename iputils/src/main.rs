extern crate regex;

use std::env;
use std::ffi;
use std::path;
use std::process;
use std::net;
use std::str;

/* Sample output:
$ ip route get 192.168.1.1
local 192.168.1.1 dev lo src 192.168.1.1
    cache <local>
$ ip route get 192.168.1.2
192.168.1.2 dev enp4s0 src 192.168.1.1
    cache
*/

#[derive(Debug)]
struct Route {
    local: bool,
    dst: net::IpAddr,
    src: net::IpAddr,
    device: String
}

impl Route {
    fn unspecified() -> Route {
        Route {
            local: false,
            dst: net::IpAddr::V4(net::Ipv4Addr::UNSPECIFIED),
            src: net::IpAddr::V4(net::Ipv4Addr::UNSPECIFIED),
            device: String::from("")
        }
    }
}

fn main() {
    if env::args().len() != 2 {
        println!("usage: {} <ip>", env::current_exe().unwrap_or(path::PathBuf::from("iputils"))
                                    .file_stem().unwrap_or(&ffi::OsString::from("iputils")).to_string_lossy());
        process::exit(1);
    }

    let route = iproute(env::args().last().unwrap().parse().expect("argument not an IP address"));
    println!("{:#?}", route);
}

fn iproute(ip: net::IpAddr) -> Route {
    let re_dst = regex::Regex::new(r"^(?:(local) )?([0-9a-f:.]+)").unwrap();
    let re_src = regex::Regex::new(r"\bsrc ([0-9a-f:.]+)\b").unwrap();
    let re_dev = regex::Regex::new(r"\bdev ([^ ]+)\b").unwrap();

    let iproute = process::Command::new("ip").arg("route").arg("get").arg(format!("{}", ip))
        .output()
        .expect("failed to execute `ip route`")
        .stdout;
    let iproute = str::from_utf8(&iproute).unwrap();

    let mut route = Route::unspecified();

    let caps = re_dst.captures(&iproute)
        .expect("bad `ip route` output: no dst");
    route.local = caps.get(1).is_some();
    route.dst = caps.get(2).unwrap().as_str().parse().unwrap();

    let caps = re_src.captures(&iproute)
        .expect("bad `ip route` output: no src");
    route.src = caps.get(1).unwrap().as_str().parse().unwrap();

    let caps = re_dev.captures(&iproute)
        .expect("bad `ip route` output: no dev");
    route.device = caps.get(1).unwrap().as_str().to_string();

    route
}
