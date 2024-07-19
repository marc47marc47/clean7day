use chrono::{Duration, Utc, Local};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "clean_archivelogs")]
struct Opt {
    /// 目標目錄
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    /// 副檔名，例如: log
    extension: String,

    /// 天數
    days: i64,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    // 計算今天日期減去指定天數的日期
    let cutoff_date = Utc::now() - Duration::days(opt.days);
    let cutoff_date_str = cutoff_date.format("%Y-%m-%d").to_string();
    println!("截止日期: {}", cutoff_date_str);

    // 生成日誌文件名
    let log_file_name = Local::now().format("%Y%m%d.log").to_string();
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_name)?;

    writeln!(log_file, "開始清理操作: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    writeln!(log_file, "截止日期: {}", cutoff_date_str)?;

    // 遍歷目錄並刪除超過指定天數的文件
    for entry in WalkDir::new(&opt.path) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            let file_extension = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
            if file_extension == opt.extension {
                let modified_time = metadata.modified()?;
                let modified_date = chrono::DateTime::<Utc>::from(modified_time);

                if modified_date < cutoff_date {
                    writeln!(log_file, "刪除文件: {:?}", entry.path())?;
                    println!("刪除文件: {:?}", entry.path());
                    //fs::remove_file(entry.path())?;
                }
            }
        }
    }

    writeln!(log_file, "清理操作結束: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    Ok(())
}

