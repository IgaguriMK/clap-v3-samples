use std::path::PathBuf;

use clap::{ArgEnum, Parser};

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}

#[derive(Debug, Parser)]
#[clap(name = "struct", author, about, version)]
struct Args {
    /// boolフラグ。
    ///
    /// `short` を指定すると、 自動で変数名の頭文字を使用する。
    /// `long` を指定すると、 自動で変数名を使用する。
    ///
    /// `help` を指定していないため、 短いヘルプ（`-h`のとき） はこのコメントの冒頭が表示される。
    /// `long_help` を指定していないため、 長いヘルプ（`--help`のとき） はこのコメントの全体が表示される。
    #[clap(short, long)]
    bool_flag: bool,

    /// 整数を引数に取る必須フラグ。
    ///
    /// 整数型を直接指定しているため、このフラグは必須である。
    #[clap(short, long)]
    count: u64,

    /// 整数を引数に取るフラグ。
    ///
    /// デフォルト値が指定されているため、コマンドラインでは省略可能である。
    #[clap(short, long, default_value = "1")]
    default_value: u64,

    /// 文字列を引数に取るオプションのフラグ。
    ///
    /// `Option<T>` を指定すると、このフラグを指定しない事が可能。
    ///
    /// `help = "..."` でヘルプテキストを明に指定する。
    #[clap(short, long, help = "オプションのテキスト")]
    text: Option<String>,

    /// 複数指定可能なフラグ。
    #[clap(short, long)]
    multiple: Vec<String>,

    /// 複数指定可能なフラグのOption版。
    ///
    /// こちらは、指定が0個の場合は `None` になる。
    #[clap(short, long)]
    optional_multiple: Option<Vec<String>>,

    /// 出現回数を数えるタイプのフラグ。
    ///
    /// verbosity の `-vvvv` のような指定で多用される。
    #[clap(short, parse(from_occurrences))]
    verbose: usize,

    /// フラグの文字列を明に指定しているフラグ。
    ///
    /// `default_value = ".."` でデフォルト値を指定できる。
    #[clap(short = 'x', long = "xxx", default_value = "2")]
    renamed: u64,

    /// 列挙型のフラグ。
    ///
    /// [`ArgEnum`] 経由でのパースの場合は、 `arg_enum` を指定する。
    #[clap(short = 'M', arg_enum)]
    mode: Mode,

    /// 環境変数を読むフラグ。
    ///
    /// `env` feature を有効にする必要がある。
    #[clap(short, long, env)]
    username: Option<String>,

    /// 必須の位置引数。
    arg1: String,

    /// オプションの位置引数。
    ///
    /// この引数はオプションなので省略できる。
    arg2: Option<PathBuf>,
}

/// モード指定用の列挙型。
///
/// [`ArgEnum`] でコマンドラインオプション用の列挙型にできる。
#[derive(Debug, Clone, ArgEnum)]
enum Mode {
    Single,
    Multi,
}
