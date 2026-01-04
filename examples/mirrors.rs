use colored::Colorize;
use httpdirectory::httpdirectory::HttpDirectory;
use httpdirectory::stats::Stats;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

mod common;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// we know that the directory should contain at least
// 7 files and directories (none should be 0) and that
// the sum of files with a date and the files without a
// date must be the same than the sum of files, parent
// directory and directories
fn is_debian_invalid(httpdir: &HttpDirectory, stats: &Stats) -> bool {
    httpdir.len() < 7
        || stats.files == 0
        || stats.dirs == 0
        || stats.files + stats.dirs + u32::from(stats.parent_dir) != stats.with_date + stats.without_date
}

/// Prints a structure T as a whole into green
fn print_in_green<T>(to_print: &T)
where
    T: Display + ?Sized,
{
    println!("{}", to_print.to_string().green());
}

/// Prints a structure T as a whole into red
fn print_in_red<T>(to_print: &T)
where
    T: Display + ?Sized,
{
    println!("{}", to_print.to_string().red());
}

#[tokio::main]
#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [99]))]
async fn main() {
    // Logging system initialization with NO_COLOR compliance
    common::setup_logging_system();

    let urls: Vec<String> =
        read_lines("./mirror.list").expect("Error reading file mirror.list").map_while(Result::ok).collect();
    println!("{}", format!("Total mirror sites: {}", urls.len()).bright_blue().bold());

    let mut tasks = JoinSet::new();

    // spawning tasks
    // Limit to 16 tasks at a time
    let semaphore = Arc::new(Semaphore::new(32));

    for url in urls {
        let semaphore = semaphore.clone();
        tasks.spawn(async move {
            let _permit = semaphore.acquire().await.expect("Semaphore closed prematurely");
            HttpDirectory::new(&url, Some(30)).await
        });
    }

    let mut correct = 0;
    let mut errored = 0;

    while let Some(task) = tasks.join_next().await {
        match task {
            Ok(result_httpdir) => match result_httpdir {
                Ok(httpdir) => {
                    let stats = httpdir.stats();

                    if is_debian_invalid(&httpdir, &stats) {
                        println!("{}", "Strange result with this mirror:".red().bold());
                        print_in_red(&stats);
                        print_in_red(&httpdir);
                        errored += 1;
                    } else {
                        print_in_green(&httpdir);
                        correct += 1;
                    }
                }
                Err(myerr) => println!("{myerr}"),
            },
            Err(e) => println!("task error: {e}"),
        }
    }

    println!();
    print_in_green(&format!("May be correct: {correct}"));
    print_in_red(&format!("Probably wrong: {errored}"));
}
