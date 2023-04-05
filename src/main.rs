#[macro_use]
extern crate lazy_static;

mod constants;
#[path = "api/en.rs"]
mod en;
mod logger;
#[path = "api/rt_new.rs"]
mod ratsit;
mod structures;

use crate::{constants::*, structures::Person};
use futures::future::join_all;
use reqwest::Client;
use std::{
    env, process,
    sync::{Arc, Mutex},
};
use tabled::{object::Rows, *};
use tokio::task::JoinHandle;

lazy_static! {
    /// Holds all the pages with peoples information.
    static ref VEC: Mutex<Vec<Person>> = Mutex::new(Vec::new());
}

/// Main startup function.
#[tokio::main]
async fn main() {
    logger::print_ascii();
    logger::print("Made by nptr and varsity.");
    if env::var("SEARCH").is_err() || env::var("END_PAGE").is_err() || env::var("AREA").is_err() {
        logger::print("Invalid usage!");
        logger::print("Usage: [ SEARCH=\"QUERY\" END_PAGE=5 AREA=\"AREA\" ./hDumper ]");
        logger::print("SEARCH: Who you want to find");
        logger::print(
            "END_PAGE is where you want to stop the search at, RT has a hard limit of 3 pages.",
        );
        logger::print(
            "AREA ONLY applies to RT and tries to narrow down the search to a specific address, or area.",
        );
        process::exit(-1);
    }

    let end_page: usize = env::var("END_PAGE")
        .unwrap()
        .parse()
        .expect(PAGE_PARSE_ERROR);

    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let search = Arc::new(env::var("SEARCH").unwrap());
    let client = Arc::new(Client::new());

    // Start (x) amount of threads and scan through page 1 -> user-defined count.
    for i in 1..end_page + 1 {
        let search = Arc::clone(&search);
        let client = Arc::clone(&client);
        let t = tokio::spawn(async move {
            logger::print(&format!("Processing page {i}, please wait..."));
            scan_ratsit(&i, &search, &client).await;
            scan_eniro(&i, &search, client).await;
        });

        threads.push(t);
    }

    // Join all threads at once
    join_all(threads).await;

    // Package all pages data into one big table, then print it.
    let mut persons: Vec<&Person> = Vec::new();
    let unwrapped = VEC.lock().unwrap();
    for person in unwrapped.iter() {
        let is_address_invalid = person.address.to_lowercase().contains("null")
            || person.address.len() < 4
            || person.address.is_empty();
        if !persons.contains(&person) && !is_address_invalid {
            persons.push(person);
        }
    }

    // Remove duplicates
    persons.dedup_by(|a, b| a.address.to_lowercase().contains(&b.address.to_lowercase()));

    if persons.is_empty() {
        logger::print("No hits were found based on your query.")
    } else {
        let len = persons.len();
        let mut table = Table::new(persons);
        table.with(Style::psql());
        // Limit the width of all rows to 50, while also keeping the words that exceed the limit,
        // but on a new line.
        table.with(Modify::new(Rows::new(..)).with(Width::wrap(50).keep_words()));
        println!("{table}");
        logger::print(&format!("{len} total hits!"));
    }
}

/// Scan using ratsit.se
async fn scan_ratsit(i: &usize, search: &str, client: &Arc<Client>) {
    let area = env::var("AREA").unwrap();
    let results = ratsit::get_info(search, &area, *i, client).await;
    if results.is_none() {
        logger::print(&format!("[RT - WARN] No entries found for page {i}!"));
        return;
    }

    // Finished, add the result to the Vector.
    let len = results.as_ref().unwrap().len();
    logger::print(&format!("[RT] Scan for page {i} finished with {len} hits!"));
    for result in results.unwrap() {
        VEC.lock().unwrap().push(result);
    }
}

/// Scan using eniro.se
async fn scan_eniro(i: &usize, search: &str, client: Arc<Client>) {
    let results = en::get_info(search, *i, client).await;
    if results.is_none() {
        logger::print(format!("[EN - WARN] No entries found for page {i}!").as_str());
        return;
    }

    // Finished, add the result to the Vector.
    let len = results.as_ref().unwrap().len();
    logger::print(format!("[EN] Scan for page {i} finished with {len} hits!").as_str());
    for result in results.unwrap() {
        VEC.lock().unwrap().push(result);
    }
}
