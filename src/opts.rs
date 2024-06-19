use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "cli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Convert a CSV file to other formats")]
    Csv(CsvOpts),
    // #[command(name = "json", about = "Convert a CSV file to other formats")]
    // Json(JsonOpts)
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// 这里的short 表示可以使用 -i 来获取参数
    /// value_parser = verify_input_file  验证输入文件是否存在
    #[arg(short, long, value_parser = verify_input_file )]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
