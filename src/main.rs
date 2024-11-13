mod args_helper;
mod dir_helper;

use args_helper::args::ProgramArgs;
use dir_helper::dir_crawler::DirCrawler;

fn main() {
    let args = ProgramArgs::new();

    let dir_crawler = DirCrawler::new(String::from(args.secret_dir()));
    match dir_crawler.validate().is_ok() {
      true => {
        let _ = dir_crawler.run_crawler();
      },
      false => {
        println!("{}", dir_crawler.validate().message());
      }
    }

}
