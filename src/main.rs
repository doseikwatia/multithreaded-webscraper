use std::{fs, path::Path, sync::Arc, thread};
use chrono::{Days, NaiveDate};
use jobs::{process_articles, process_links};
use threadpool::ThreadPool;
use tokio::main;
use crossbeam_channel::unbounded;
use webscraper::Article;

mod webscraper;
mod utils;
mod jobs;



static BASE_URL: &str = "https://www.ghanaweb.com";
static NTHREADS: [usize; 2] = [16, 2];

#[main]
async fn main() {
    let archive_url = format!(
        "{}/GhanaHomePage/NewsArchive/browse.archive.php?date=$date",
        BASE_URL
    );

    let link_date_fmt = "%Y%m%d";
    let output_dir = "output/";
    let mut cur_date = NaiveDate::parse_from_str("20241231", link_date_fmt).unwrap();
    let end_date = NaiveDate::parse_from_str("20251231", link_date_fmt).unwrap();

    //ensure output directory exists and create it if it does not exist
    if !Path::new(output_dir).exists() {
        match fs::create_dir_all(output_dir) {
            Ok(_) => println!("output directory created successfully"),
            Err(err) => panic!("failed to create output directory. {}", err),
        };
    }

    //initialize communication channels
    let (link_chan_tx, link_chan_rx) = unbounded::<String>();
    let arc_shared_link_chan_rx = Arc::new(link_chan_rx);
    let (article_chan_tx, article_chan_rx) = unbounded::<Article>();
    let arc_shared_article_chan_rx = Arc::new(article_chan_rx);

    //set threadpools up
    let [article_tp_size, out_tp_size] = NTHREADS;
    let article_tp = ThreadPool::new(article_tp_size);
    let out_tp = ThreadPool::new(out_tp_size);

    //start threads
    //link thread --- article_chan_tx ---|> out_chan_rx
    process_links(
        article_tp_size,
        &arc_shared_link_chan_rx,
        &article_chan_tx,
        &article_tp,
    );

    //out_chan_rx --- > to json file
    process_articles(
        article_tp_size,
        &arc_shared_article_chan_rx,
        output_dir,
        &out_tp,
    );

    //main thread --- link_chan ---|> article thread
    while cur_date != end_date {
        let new_link = archive_url.replace("$date", &cur_date.format(link_date_fmt).to_string());
        if let Err(err) = link_chan_tx.send(new_link) {
            panic!("failed to send link. {}", err);
        }
        cur_date = cur_date.checked_add_days(Days::new(1)).unwrap();
    }
    let thread_id = thread::current().id();
    println!("[{:?}] done generating links thread.", thread_id);

    //closing the channel
    drop(link_chan_tx);

    //wait for all threadpools
    out_tp.join();
    article_tp.join();
}
