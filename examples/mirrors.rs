use colored::Colorize;
use env_logger::{Env, WriteStyle};
use httpdirectory::httpdirectory::HttpDirectory;
use std::env::var;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tokio::task::JoinSet;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]), flavor = "current_thread")]
async fn main() {
    let no_color_compliance = match var("NO_COLOR").is_ok() {
        true => WriteStyle::Never,
        false => WriteStyle::Auto,
    };

    // Replace default_filter_or("") by default_filter_or("debug") to see debug message by default
    // One may want to directly use RUST_LOG=httpdirectory=debug instead
    env_logger::Builder::from_env(Env::default().default_filter_or("")).write_style(no_color_compliance).init();

    let mut urls = vec![];

    if let Ok(lines) = read_lines("./mirror.list") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            urls.push(line);
        }
    }
    let capacity = urls.len();
    println!("Total mirror sites: {capacity}");
    let mut tasks = JoinSet::new();

    // spawning tasks
    let _vec_abort_handle: Vec<_> =
        urls.into_iter().map(|url| tasks.spawn(async move { HttpDirectory::new(&url).await })).collect();

    let mut correct = 0;
    let mut errored = 0;

    while let Some(task) = tasks.join_next().await {
        let result_httpdir = task.unwrap();

        match result_httpdir {
            Ok(httpdir) => {
                let stats = httpdir.stats();

                // we know that the directory should contain at least
                // 7 files and directories (none should be 0) and that
                // every file and directory must have a date
                if httpdir.len() < 7
                    || stats.files == 0
                    || stats.dirs == 0
                    || stats.files + stats.dirs + stats.parent_dir as u32 != stats.with_date + stats.without_date
                {
                    println!("{}", "Strange result with this mirror:".red().bold());
                    println!("{}", stats.to_string().red());

                    println!("{}", httpdir.to_string().red());
                    errored += 1;
                } else {
                    println!("{}", httpdir.to_string().green());
                    correct += 1;
                }
            }
            Err(myerr) => println!("{myerr}"),
        }
    }

    println!();
    println!("May be correct: {}", correct.to_string().green());
    println!("Probably wrong: {}", errored.to_string().red());
}
