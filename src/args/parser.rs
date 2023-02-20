use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "fy", version = "0.1.0", about = "简单练手的socks5代理命令行工具", long_about = None)]
pub struct Args {
    /// Listening ip and port
    #[arg(short, long, default_value_t = String::from("127.0.0.1:1080"))]
    pub listen: String,
}

impl Args {
    pub fn get_args() -> Self {
        Args::parse()
    }
}
