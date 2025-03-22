use std::fs::File;
use std::path::Path;
use std::{io::{self, BufRead, Write}, thread};
use crate::{ONE_SECOND};

const ROUTE_DATA_LOCATION: &str = "data/PFR_BASE.csv";
const ROUTE_FUTURE_DATA_LOCATION: &str = "data/PFR_BASE_NEW.csv";
#[derive(PartialEq, Clone)]
pub struct PreferentialRoute {
    origin_id: String,
    origin_city: String,
    origin_state_code: String,
    origin_country_code: String,
    destination_id: String,
    destination_city: String,
    destination_state_code: String,
    destination_country_code: String,
    pfr_type_code: String,
    route_number: String,
    special_area_description: String,
    altitude_description: String,
    aircraft: String,
    hours: String,
    route_dir_description: String,
    designator: String,
    nar_type: String,
    inland_fac_fix: String,
    coastal_fix: String,
    destination: String,
    route_string: String,
    route_notes: Option<String>,
    region: String,
}

#[derive(Clone)]
struct ModifiedRoute {
    current_route: Option<PreferentialRoute>,
    future_route: Option<PreferentialRoute>,
    altitude_change: Option<bool>,
    route_change: Option<bool>
}

fn read_tec_routes(future_data: bool) -> Vec<PreferentialRoute> {
    let path = if !future_data {
        ROUTE_DATA_LOCATION
    } else {
        ROUTE_FUTURE_DATA_LOCATION
    };

    let mut route_list: Vec<PreferentialRoute> = Vec::new();

    if !future_data && !Path::new(path).exists() {
        println!("Please download the TEC_ROUTE.csv file and put it in the data folder, and restart the function.");

        thread::sleep(ONE_SECOND);

        return route_list;
    } else if future_data && !Path::new(path).exists() {
        println!("Please download the upcoming changes and put it in the data folder as APT_BASE_NEW.csv, and restart the function.");

        thread::sleep(ONE_SECOND);

        return route_list;
    }

    // Attempt to open the file
    let file =  File::open(path).expect("File failed to open");

    // Create a buffered reader to efficiently read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let norcal_tec_route_ids: Vec<&str> = vec!["SJC", "MOD", "SQL", "MHR", "HWD", "LVK", "MRY", "NUQ", "OAK", "SAC", "SCK", "SFO"];
        // Check if reading the line was successful
        let clean_line = line.unwrap().replace('"', "");

        // Split the line by commas and collect the values into a vector
        let split_data: Vec<&str> = clean_line.split(',').collect();

        // Print the vector of values for the current line
        //println!("{:?}", split_data);

        // If the line is a valid TEC Route data block, create a TEC Route
        if split_data[9] == "TEC" {
            // Check the TEC Route is a Californian TEC Route
            if split_data[3] == "CA" && split_data[7] == "CA" { 
                let mut new_tec_route = PreferentialRoute {
                    origin_id: split_data[1].to_string(),
                    origin_city: split_data[2].to_string(),
                    origin_state_code: split_data[3].to_string(),
                    origin_country_code: split_data[4].to_string(),
                    destination_id: split_data[5].to_string(),
                    destination_city: split_data[6].to_string(),
                    destination_state_code: split_data[7].to_string(),
                    destination_country_code: split_data[8].to_string(),
                    pfr_type_code: split_data[9].to_string(),
                    route_number: split_data[10].to_string(),
                    special_area_description: split_data[11].to_string(),
                    altitude_description: split_data[12].to_string(),
                    aircraft: split_data[13].to_string(),
                    hours: split_data[14].to_string(),
                    route_dir_description: split_data[15].to_string(),
                    designator: split_data[16].to_string(),
                    nar_type: split_data[17].to_string(),
                    inland_fac_fix: split_data[18].to_string(),
                    coastal_fix: split_data[19].to_string(),
                    destination: split_data[20].to_string(),
                    route_string: split_data[21].to_string(),
                    route_notes: None,
                    region: String::from("TBC")
                };

                if new_tec_route.route_string == "" {
                    new_tec_route.route_string = String::from("DCT");
                }

                for route_id in norcal_tec_route_ids {
                    if new_tec_route.designator[0..3] == String::from(route_id) {
                        new_tec_route.region = String::from("NorCal");
                        break;
                    } else {
                        new_tec_route.region = String::from("SoCal");
                    }
                }

                if new_tec_route.region == "NorCal" {
                    let split_description: Vec<&str> = new_tec_route.special_area_description.split(" TO ").collect();
                    let split_departures: Vec<&str> = split_description[0].split(' ').collect();
                    let split_arrivals: Vec<&str> = split_description[1].split(' ').collect();
                    let base_route: PreferentialRoute = new_tec_route.clone();

                    for departure in split_departures.clone() {
                        for arrival in split_arrivals.clone() {
                            let new_sub_route: PreferentialRoute = PreferentialRoute {
                                origin_id: String::from(departure),
                                origin_city: "".to_string(),
                                origin_state_code: "".to_string(),
                                origin_country_code: "".to_string(),
                                destination_id: String::from(arrival),
                                destination_city: "".to_string(),
                                destination_state_code: "".to_string(),
                                destination_country_code: "".to_string(),
                                pfr_type_code: base_route.clone().pfr_type_code,
                                route_number: base_route.clone().route_number,
                                special_area_description: base_route.clone().special_area_description,
                                altitude_description: base_route.clone().altitude_description,
                                aircraft: base_route.clone().aircraft,
                                hours: base_route.clone().hours,
                                route_dir_description: base_route.clone().route_dir_description,
                                designator: base_route.clone().designator,
                                nar_type: base_route.clone().nar_type,
                                inland_fac_fix: base_route.clone().inland_fac_fix,
                                coastal_fix: base_route.clone().coastal_fix,
                                destination: base_route.clone().destination,
                                route_string: base_route.clone().route_string,
                                route_notes: None,
                                region: base_route.clone().region,
                            };

                            route_list.push(new_sub_route);
                        }
                    }
                }

                if new_tec_route.special_area_description.contains("LAXE") {
                    new_tec_route.route_notes = Option::from(String::from("LAX EAST"));
                } else if new_tec_route.special_area_description.contains("LAXW") {
                    new_tec_route.route_notes = Option::from(String::from("LAX WEST"));
                } else if new_tec_route.special_area_description.contains("SANE") {
                    new_tec_route.route_notes = Option::from(String::from("SAN EAST"));
                }

                if new_tec_route.route_dir_description == "SFOW" {
                    new_tec_route.route_notes = Option::from(String::from("SFO West"));
                } else if new_tec_route.route_dir_description == "SFOE" {
                    new_tec_route.route_notes = Option::from(String::from("SFO East"));
                }
                
                route_list.push(new_tec_route);
            }
        }
    }

    println!("TEC Routes read.");
    route_list
}

pub fn generate_tec_route_changes() {
    let current_routes: Vec<PreferentialRoute> = read_tec_routes( false);
    let future_routes: Vec<PreferentialRoute> = read_tec_routes( true);
    let mut new_routes: Vec<ModifiedRoute> = Vec::new();
    let mut removed_routes: Vec<ModifiedRoute> = Vec::new();
    let mut modified_routes: Vec<ModifiedRoute> = Vec::new();

    let mut route_exists_in_new_data: bool;
    let mut route_exists_in_current_data: bool;
    let mut routes_have_changed: bool = false;

    for route in &current_routes {
        route_exists_in_new_data = future_routes.iter().any(|x| x.designator == route.designator);

        if !route_exists_in_new_data {
            let modified_route: ModifiedRoute = ModifiedRoute {
                current_route: Option::from(route).cloned(),
                future_route: None,
                altitude_change: None,
                route_change: None
            };

            routes_have_changed = true;
            removed_routes.push(modified_route);
        }
    }

    println!("Deleted routes listed");

    for route in &future_routes {
        route_exists_in_current_data = current_routes.iter().any(|x| x.designator == route.designator);

        if !route_exists_in_current_data {
            let modified_route: ModifiedRoute = ModifiedRoute {
                current_route: None,
                future_route: Option::from(route).cloned(),
                altitude_change: None,
                route_change: None
            };

            routes_have_changed = true;
            new_routes.push(modified_route);
        }
    }

    println!("New routes listed");

    for current_route_loop in current_routes {
        route_exists_in_new_data = future_routes.iter().any(|x| x.designator == current_route_loop.designator);

        if route_exists_in_new_data {
            let new_route: Option<&PreferentialRoute> = future_routes.iter().find(|x| x.designator == current_route_loop.designator);
            let altitude_change: bool = current_route_loop.altitude_description != new_route.as_ref().unwrap().altitude_description;
            let route_change: bool = current_route_loop.route_string != new_route.as_ref().unwrap().route_string;
            let route_number_matches: bool = current_route_loop.route_number == new_route.as_ref().unwrap().route_number;
            let route_description_matches: bool = current_route_loop.special_area_description == new_route.as_ref().unwrap().special_area_description;

            if (route_change || altitude_change) && route_number_matches && route_description_matches {
                let modified_route: ModifiedRoute = ModifiedRoute {
                    current_route: Option::from(current_route_loop),
                    future_route: new_route.cloned(),
                    altitude_change: Option::from(altitude_change),
                    route_change: Option::from(route_change),
                };

                routes_have_changed = true;
                modified_routes.push(modified_route);
            }
        }
    }

    println!("Modified routes listed");

    // Outputting list
    let path = "data/output/changed_tec_routes.txt";

    if let Ok(mut file) = File::create(path) {
        writeln!(file, "# **TEC Route changes effective  // CYCLE**").unwrap();

        for new_route in new_routes {
            writeln!(file, "{} ({} -> {}) // ADDED", new_route.future_route.as_ref().unwrap().designator, new_route.future_route.as_ref().unwrap().origin_id, new_route.future_route.as_ref().unwrap().destination_id).unwrap();
        }

        writeln!(file, " ").unwrap();

        for removed_route in removed_routes {
            writeln!(file, "{} ({} -> {}) // REMOVED", removed_route.current_route.as_ref().unwrap().designator, removed_route.current_route.as_ref().unwrap().origin_id, removed_route.current_route.as_ref().unwrap().destination_id).unwrap();
        }

        for modified_route in modified_routes.iter().cloned() {
            if modified_route.altitude_change.unwrap() {
                writeln!(file, "{} ({} -> {}) // ALTITUDE CHANGED `{}`", modified_route.future_route.as_ref().unwrap().designator, modified_route.future_route.as_ref().unwrap().origin_id, modified_route.future_route.as_ref().unwrap().destination_id, modified_route.future_route.as_ref().unwrap().altitude_description).unwrap();
            }
        }

        for modified_route in modified_routes.iter().cloned() {
            if modified_route.route_change.unwrap() {
                writeln!(file, "{} ({} -> {}) // ROUTE CHANGED `{}`", modified_route.future_route.as_ref().unwrap().designator, modified_route.future_route.as_ref().unwrap().origin_id, modified_route.future_route.as_ref().unwrap().destination_id, modified_route.future_route.as_ref().unwrap().route_string).unwrap();
            }
        }

        if !routes_have_changed {
            writeln!(file, "** NO ROUTE CHANGES **").unwrap();
        }
    }

    println!("File outputted at {}", path);
    thread::sleep(ONE_SECOND);
}

pub fn generate_mfr_tec_route_list() {
    let tec_routes: Vec<PreferentialRoute> = read_tec_routes(false);
    let path: &str = "data/output/tec_routes.csv";

    if let Ok(mut file) = File::create(path) {
        // writeln!(file, "id,route_designator,origin_id,destination_id,altitude,aircraft,route_string,route_notes,created_at,updated_at").unwrap();

        for tec_route in tec_routes {
            writeln!(file, ",{},{},{},{},{},{},{},{},,", tec_route.designator, tec_route.origin_id, tec_route.destination_id, tec_route.altitude_description, tec_route.aircraft, tec_route.route_string, tec_route.route_notes.unwrap_or(String::from("")), tec_route.region).unwrap();
        }
    } else {
        println!("Failed to create file.");
    }
}