use clap::{Parser, Subcommand};

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}

/// サブコマンドを持つコマンドライン引数の構造体。
#[derive(Debug, Parser)]
#[clap(name = "subcommand", author, about, version)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// サブコマンドの定義
#[derive(Debug, Subcommand)]
enum Commands {
    /// コマンドライン引数をその場で指定しているサブコマンド。
    Add { x: i64, y: i64 },

    /// 指定を別構造体に切り出したサブコマンド。
    ///
    /// 実際には中の構造体を別モジュールで定義する際などに有用。
    Complex(Complex),
}

#[derive(Debug, clap::Args)]
struct Complex {
    u: u64,
    v: u64,
    x: u64,
    y: u64,
}
