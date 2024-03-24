use std::fs::File;
use std::path::Path;
use std::{io::{self, BufRead}, thread};
use crate::ONE_SECOND;

const ROUTE_DATA_LOCATION: &str = "data/route.csv";
const ROUTE_FUTURE_DATA_LOCATION: &str = "data/future_route.csv";

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
    route_string: String
}

pub fn read_tec_routes(future_data: bool) -> Vec<PreferentialRoute> {
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
        // Check if reading the line was successful
        let clean_line = line.unwrap().replace('"', "");

        // Split the line by commas and collect the values into a vector
        let split_data: Vec<&str> = clean_line.split(',').collect();

        // Print the vector of values for the current line
        //println!("{:?}", split_data);

        // If the line is a valid TEC Route data block, create a TEC Route
        if split_data[9] != "TEC" {
            // Check the TEC Route is a Californian TEC Route
            if split_data[3] == "CA" && split_data[7] == "CA" { 
                let new_tec_route = PreferentialRoute {
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
                    route_string: split_data[21].to_string()
                };

                route_list.push(new_tec_route);
            }
        }
    }

    println!("TEC Routes read.");
    route_list
}