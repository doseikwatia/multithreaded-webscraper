use std::{fs, path::Path, sync::Arc, thread};

use chrono::{Days, NaiveDate};
use crossbeam_channel::{unbounded, Receiver, Sender};
use threadpool::ThreadPool;
use tokio::main;
use tokio::runtime::Runtime;
use webscraper::{get_article, get_list_of_article_links, write_article_to_file, Article};
mod utils;
mod webscraper;

static BASE_URL: &str = "https://www.ghanaweb.com";
static NTHREADS: [usize; 2] = [8, 2];

fn process_links(
    article_tp_size: usize,
    arc_shared_link_chan_rx: &Arc<Receiver<String>>,
    article_chan_tx: &Sender<Article>,
    article_tp: &ThreadPool,
) {
    for _ in 0..article_tp_size {
        let link_chan_rx = Arc::clone(arc_shared_link_chan_rx);
        let shared_article_chan_tx = article_chan_tx.clone();

        article_tp.execute(move || {
            let thread_id = thread::current().id();
            let rt = Runtime::new().unwrap();
            
            while let Ok(link) = link_chan_rx.recv() {
                let articles: Vec<Article> = rt
                    .block_on(get_list_of_article_links(&link))
                    .unwrap()
                    .iter()
                    .map(|a_link| rt.block_on(get_article(a_link)).unwrap())
                    .collect();
                for a in articles {
                    if let Err(err) = shared_article_chan_tx.send(a) {
                        panic!(
                            "[{:?}] failed to send article object over the tx channel.",
                            err
                        );
                    }
                }
            }
            drop(shared_article_chan_tx);
            println!("Exiting article thread. {:?}", thread_id);
        });
    }
}


fn process_articles<'a >(article_tp_size:usize,arc_shared_article_chan_rx:&Arc<Receiver<Article>>,output_dir:&str, out_tp :&ThreadPool){
    for _ in 0..article_tp_size {
        let shared_article_chan_rx = Arc::clone(arc_shared_article_chan_rx);
        let output_dir = output_dir.to_owned();
        out_tp.execute(move || {
            let thread_id = thread::current().id();
            while let Ok(article) = shared_article_chan_rx.recv() {
                println!("[{:?}] writing {} to file.", thread_id, article.title);
                if let Err(err) = write_article_to_file(&output_dir, &article) {
                    panic!(
                        "[{:?}] failed to write article {:?} to file. {}",
                        thread_id, &article, err
                    );
                }
            }
            println!("[{:?}] exiting output thread.", thread_id);
        });
    }
}

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
    process_links(article_tp_size, &arc_shared_link_chan_rx,&article_chan_tx, &article_tp);

    //out_chan_rx --- > to json file
    process_articles(article_tp_size, &arc_shared_article_chan_rx, output_dir, &out_tp);
    
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
