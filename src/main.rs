pub mod http;
pub mod os_ffi;
use std::process::exit;
use std::env::args;
use std::collections::HashMap;
use http::server::HttpServer as HttpServer;

fn main() {
    let server = HttpServer {
        port: String::from("8080"),
        host: String::from("127.0.0.1"),
        root: String::from("src/")
    };
    let arguments = argument_hash();
    let false_string = String::from("false");
    let help = arguments.get("help").unwrap_or(&false_string);
    let daemonize = arguments.get("daemonize").unwrap_or(&false_string);
    if help == "true" {
        print_help();
        exit(0);
    }

    run_server(server, daemonize == "true");
}

fn run_server(server: HttpServer, daemonize: bool) {
    if daemonize {
        daemonize_server(server);
    } else {
        server.run();
    }
}

fn daemonize_server(server: HttpServer) {
    unsafe {
        let pid = os_ffi::fork();
        match pid {
            -1 => panic!("Fork failed!"),
            0  => server.run(),
            _  =>
                {
                    println!("Daemonizing with pid {}", pid);
                    exit(0);
                }
        }
    }
}

fn argument_hash() -> HashMap<String, String> {
    let mut arg_hash: HashMap<String, String> = HashMap::new();
    let args_vec: Vec<String> = args().collect();
    let mut key: &str; //= "";
    let mut val: &str; // = "";

    for i in 1..args_vec.len() {
        let ref word = args_vec[i];
        if word.starts_with("--") {
            if word == "--daemonize" {
                key = &"daemonize";
                val = &"true";
            } else if word == "--help" {
                key = &"help";
                val = &"true";
            } else {
                let keyval_pair: Vec<&str> = word.split("=").collect();
                key = keyval_pair[0];
                val = keyval_pair[1];
            }
        } else if word.starts_with("-") {
            if word == "-d" {
                key = &"daemonize";
                val = &"true";
            } else if word == "-h" {
                key = &"help";
                val = &"true";
            } else {
                key = &args_vec[i];
                val = &args_vec[i + 1];
            }
        } else {
            continue;
        }
        arg_hash.insert(String::from(key), String::from(val));
    }
    return arg_hash;
}

fn print_help() {
    let prog_full_path: String = args().next().unwrap();
    let prog_name = prog_full_path.split("/").last().unwrap();
    println!("Usage {} [OPTION]", prog_name);
    println!("Run shrimpy http server");
    println!("");
    println!("  -d, --daemonize\t\t Run server in background");
    println!("  -h, --help\t\t\t This help");
}
