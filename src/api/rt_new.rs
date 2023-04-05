use std::sync::Arc;

use crate::{
    constants::{RT_PG_LIMIT, USER_AGENT},
    logger,
    structures::Person,
};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

/// The API URL.
const API_URL: &str = "https://www.ratsit.se/api/search/combined";

/// Searches for people with the specified details.
pub async fn get_info(
    name: &str,
    area: &str,
    page: usize,
    client: &Arc<Client>,
) -> Option<Vec<Person>> {
    if page > 3 {
        // Clamp the page down to 3.
        logger::print(RT_PG_LIMIT)
    }

    let page = page.clamp(1, 3);
    let json_request = format!(
        r#"
{{
  "who": "{name}",
  "age": [
    "16",
    "120"
  ],
  "phoneticSearch": true,
  "companyName": "",
  "orgNr": "",
  "firstName": "",
  "lastName": "",
  "personNumber": "",
  "phone": "",
  "address": "{area}",
  "postnr": "",
  "postort": "",
  "kommun": "",
  "page": {page}
}}
                                  "#
    );

    let mut headers = HeaderMap::new();

    // We're requesting JSON output, nothing else.
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    // Spoof the User Agent.
    headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));

    let content = &client
        .post(API_URL)
        .body(json_request)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    if !content.is_empty() {
        let parsed = json::parse(content).unwrap();
        // Companies is 0.
        let person_entries = parsed.entries().nth(1).unwrap().1;
        if !person_entries.is_empty() {
            let mut persons: Vec<Person> = Vec::new();

            // Scan through the hits.
            for person in person_entries["hits"].members() {
                // Check if the person is married.
                let married = if let Some(married) = person["married"].as_bool() {
                    if married {
                        "Y" // Married
                    } else {
                        "N" // Not married, or at least not from public knowledge
                    }
                } else {
                    "?" // Unknown
                }
                .to_owned();

                persons.push(Person {
                    name: format!("{} {}", person["firstName"], person["lastName"]),
                    gender: person["gender"].to_string(),
                    age: person["age"].to_string().parse().unwrap(),
                    address: format!("{}, {}", person["streetAddress"], person["city"]),
                    married,
                    numbers: String::from("???"),
                    source: file!().to_string(),
                })
            }

            return Some(persons);
        } else {
            return None;
        }
    }

    None
}
