use std::{fs, time::Duration};
use menu_genie::{MenuAction, MenuBuilder, MgErrorKind};
use crate::airport::Airport;
use crate::modified_airport::ModifiedAirport;

pub mod airport;
pub mod preferential_route;
mod modified_airport;

const ONE_SECOND:Duration = Duration::from_secs(1);
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    fs::create_dir_all("./data").expect("Failed to create");
    
    println!("FAA NASR Data Processor v{}", VERSION.unwrap_or("Unknown"));
    main_menu();
}

fn generate_airport_changes() {
    let current_airports: Vec<Airport> = airport::read_airports(false);
    let mut future_airports: Vec<Airport> = airport::read_airports(true);
    let mut modified_airports: Vec<ModifiedAirport> = Vec::new();

    let mut airport_exists_in_new_data: bool = false;
    let mut airport_exists_in_current_data: bool = false;

    for airport in &current_airports {
        airport_exists_in_new_data = future_airports.iter().any(|x| x.airport_id == airport.airport_id);

        if !airport_exists_in_new_data {
            let modified_airport: ModifiedAirport = ModifiedAirport {
                current_airport: Option::from(airport).cloned(),
                new_airport: Option::None,
                is_modified: Option::from(true),
                renamed: Option::from(false),
                closed: Option::from(true),
                opened: Option::from(false),
            };

            modified_airports.push(modified_airport);
        }
    }

    println!("Closed airports listed");

    for airport in &future_airports {
        airport_exists_in_current_data = current_airports.iter().any(|x| x.airport_id == airport.airport_id);

        if !airport_exists_in_current_data {
            let modified_airport: ModifiedAirport = ModifiedAirport {
                current_airport: Option::None,
                new_airport: Option::from(airport).cloned(),
                is_modified: Option::from(true),
                renamed: Option::from(false),
                closed: Option::from(false),
                opened: Option::from(true),
            };

            modified_airports.push(modified_airport);
        }
    }

    println!("Opened airports listed");

    for current_airport_loop in current_airports {
        airport_exists_in_new_data = future_airports.iter().any(|x| x.airport_id == current_airport_loop.airport_id);

        if airport_exists_in_new_data {
            let future_airport: Option<&Airport> = future_airports.iter().find(|x| x.airport_id == current_airport_loop.airport_id);

            if let Some(airport) = future_airport {
                let name_change: bool = current_airport_loop.airport_name != future_airport.unwrap().airport_name;

                if name_change {
                    let modified_airport: ModifiedAirport = ModifiedAirport {
                        current_airport: Option::from(current_airport_loop),
                        new_airport: Option::from(future_airport.cloned()),
                        is_modified: Option::Some(true),
                        renamed: Option::Some(true),
                        closed: Option::Some(false),
                        opened: Option::Some(false),
                    };

                    modified_airports.push(modified_airport);
                }
            }
        }
    }

    println!("Renamed airports listed");
}

fn main_menu() {
    let mut menu = MenuBuilder::new()
        .with_menu(1)
        .with_menu_item("Export airport list", MenuAction::Nothing)
        .with_menu_item("Generate airport changes list", MenuAction::Nothing)
        .with_menu_item("Generate TEC Route changes list", MenuAction::Nothing)
        .with_quit_button()
        .build();

    loop {
        match menu.prompt() {
            Ok(tuple) => match tuple {
                (1, 1) => println!("Export Airport list"),
                (1, 2) => generate_airport_changes(),
                (1, 3) => println!("Generate TEC Route changes"),
                _ => (),
            },

            Err(e) => {
                println!("{e}");
                match e.kind() {
                    MgErrorKind::MissingMenu(_) | MgErrorKind::MissingMenuItem(..) => break,
                    _ => (),
                }
            }
        } 
    }
}