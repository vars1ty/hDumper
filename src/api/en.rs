use crate::structures::Person;
use json::JsonValue;
use reqwest::Client;
use std::sync::Arc;

/// Searches for people with the specified details.
pub async fn get_info(name: &str, page: usize, client: Arc<Client>) -> Option<Vec<Person>> {
    let content = client.get(format!("https://one-back.eniro.se/custom/apiSearch?brand=eniro&page={page}&query={name}&rating=0&type=persons")).send().await.unwrap().text().await.unwrap();
    if !content.is_empty() {
        let parsed = json::parse(&content).unwrap();
        let person_entries = parsed["persons"].members();

        // Don't waste time creating a new Vector if the length is 0.
        if person_entries.len() != 0 {
            let mut persons: Vec<Person> = Vec::new();

            // Scan through all the listed persons from the API.
            for person in person_entries {
                persons.push(Person {
                    name: extract_name(person),
                    gender: String::from("N/A"),
                    age: 0,
                    address: extract_address(person),
                    married: String::from("?"),
                    numbers: extract_numbers(person),
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

/// Extracts the First, Middle and Last name.
/// If a middle name isn't found, it's skipped.
fn extract_name(person: &JsonValue) -> String {
    // Grab from the API.
    let first_name = &person["name"]["firstName"];
    let middle_name = &person["name"]["middleName"];
    let last_name = &person["name"]["lastName"];

    // Placeholder names which are replaced with the proper names if they aren't null.
    let mut fixed_first_name = String::from("Unknown");
    let mut fixed_middle_name = String::default();
    let mut fixed_last_name = String::from("Unknown");
    if !first_name.is_null() {
        fixed_first_name = first_name.to_string();
    }

    if !middle_name.is_null() {
        fixed_middle_name = middle_name.to_string();
    }

    if !last_name.is_null() {
        fixed_last_name = last_name.to_string();
    }

    // Format the final name.
    // If there's no middle name, only format the first and last name, eliminating double-spaces in
    // the middle.
    if !middle_name.is_null() {
        format!("{fixed_first_name} {fixed_middle_name} {fixed_last_name}")
    } else {
        format!("{fixed_first_name} {fixed_last_name}")
    }
}

/// Extracts all phone numbers from the specified person.
fn extract_numbers(person: &JsonValue) -> String {
    let phones = person["phones"].members().collect::<Vec<&JsonValue>>();
    let mut result = String::default();
    for i in 0..phones.len() {
        result.push_str(&phones[i]["number"].to_string());
        // Only append ", " at the end of the string if we haven't reached the last entry.
        if phones.len() - i != 1 {
            result.push_str(", ")
        }
    }

    // Use ??? as the default value if its empty.
    if result.is_empty() {
        result = String::from("???")
    }

    result
}

/// Extracts the first address from the specified person.
fn extract_address(person: &JsonValue) -> String {
    let first_address = &person["addresses"][0];
    let street_name = &first_address["streetName"];
    let street_number = &first_address["streetNumber"];
    let postal_area = &first_address["postalArea"];
    format!("{street_name} {street_number}, {postal_area}")
}
