use std::{sync::Arc, thread};

use threadpool::ThreadPool;
use tokio::runtime::Runtime;
use futures::future::join_all;
// use webscraper::{get_article, get_list_of_article_links, write_article_to_file, Article};
use crossbeam_channel::{Receiver, Sender};

use crate::webscraper::{get_article, get_list_of_article_links, write_article_to_file, Article};


pub fn process_links(
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
                let articles: Vec<Article> = rt.block_on(async {
                    let article_links = get_list_of_article_links(&link).await.expect("Error getting links");
                    let article_futures = article_links.iter().map(|a_link| get_article(a_link));
                    join_all(article_futures).await.into_iter().filter_map(|a|a).collect()
                });

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

pub fn process_articles<'a>(
    article_tp_size: usize,
    arc_shared_article_chan_rx: &Arc<Receiver<Article>>,
    output_dir: &str,
    out_tp: &ThreadPool,
) {
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
