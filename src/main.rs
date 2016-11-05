pub mod http;
pub mod os_ffi;
pub mod argument_parser;
use std::process::exit;
use std::env::args;
use http::server::HttpServer as HttpServer;
use argument_parser::Argument as Argument;
use argument_parser::ArgumentHash as AHash;

fn main() {
    let arguments = AHash::new().with_arg(Argument {
        key: String::from("help"),
        long_argument: String::from("help"),
        short_argument: String::from("h"),
        // default: String::from("false"),
        // name: String::from("Help"),
        takes_value: false,
        // required: false
    }).with_arg(Argument {
        key: String::from("daemonize"),
        long_argument: String::from("daemonize"),
        short_argument: String::from("d"),
        // default: String::from("false"),
        // name: String::from("Daemonize"),
        takes_value: false,
        // required: false
    }).with_arg(Argument {
        key: String::from("root"),
        long_argument: String::from("root"),
        short_argument: String::from("r"),
        takes_value: true,
    }).as_hash();

    let false_string = String::from("false");
    let default_root = String::from("/shrimpy/");
    let help = arguments.get("help").unwrap_or(&false_string);
    let daemonize = arguments.get("daemonize").unwrap_or(&false_string);
    let root = arguments.get("root").unwrap_or(&default_root);

    if help == "true" {
        print_help();
        exit(0);
    }

    let server = HttpServer {
        port: String::from("8080"),
        host: String::from("127.0.0.1"),
        root: (*root).clone()
    };

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

fn print_help() {
    let prog_full_path: String = args().next().unwrap();
    let prog_name = prog_full_path.split("/").last().unwrap();
    println!("Usage {} [OPTION]", prog_name);
    println!("Run shrimpy http server");
    println!("");
    println!("  -d, --daemonize\t\t Run server in background");
    println!("  -h, --help\t\t\t This help");
}
