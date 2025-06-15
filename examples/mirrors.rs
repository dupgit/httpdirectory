use colored::Colorize;
use env_logger::{Env, WriteStyle};
use httpdirectory::httpdirectory::HttpDirectory;
use std::env::var;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    // note the use of `into_iter()` to consume `items`
    let tasks: Vec<_> =
        urls.into_iter().map(|url| tokio::spawn(async move { HttpDirectory::new(&url).await })).collect();
    // await the tasks for resolve's to complete and give back our items
    let mut option_httpdir_vec = vec![];
    for task in tasks {
        option_httpdir_vec.push(task.await.unwrap());
    }

    // verify that we've got the results
    for option_httpdir in &option_httpdir_vec {
        match option_httpdir {
            Ok(httpdir) => {
                let stats = httpdir.stats();

                // we know that the directory should contain at least
                // 7 files and directories (none should be 0) and that
                // every file and directory must have a date
                if httpdir.len() < 7
                    || stats.files == 0
                    || stats.dirs == 0
                    || stats.files + stats.dirs != stats.with_date
                {
                    println!("{}", "Strange result with this mirror:".red().bold());
                    println!("{}", stats.to_string().red());

                    println!("{}", httpdir.to_string().red());
                } else {
                    println!("{}", httpdir.to_string().green());
                }
            }
            Err(myerr) => println!("{myerr}"),
        }
    }
}
