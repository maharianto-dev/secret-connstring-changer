mod args_helper;
mod dir_helper;
mod content_helper;
mod error_handler;

use args_helper::args::ProgramArgs;
use content_helper::file_writer::JsonConfig;
use dir_helper::dir_crawler::DirCrawler;

fn main() {
    let args = ProgramArgs::new();

    // let dir_crawler = DirCrawler::new(&String::from(args.secret_dir()));
    // match dir_crawler.validate().is_ok() {
    //   true => {
    //     let _ = dir_crawler.run_crawler();
    //   },
    //   false => {
    //     println!("{}", dir_crawler.validate().message());
    //   }
    // }

    let json = JsonConfig::new(args.connstring_key().to_string()).unwrap();
    let files = json.get_config_connection_string();
    println!("{}", &files.unwrap());
}
