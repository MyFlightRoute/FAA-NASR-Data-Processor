use std::{path::Path, thread, io::{self, BufRead, Write}, fs};
use std::fs::File;
use serde::Serialize;
use chrono::{NaiveDate, TimeZone, Utc};

use crate::{airport, ONE_SECOND};

const AIRPORT_DATA_LOCATION: &str = "data/APT_BASE.csv";
const PREVIEW_AIRPORT_DATA_LOCATION: &str = "data/APT_BASE_NEW.csv";

#[expect(clippy::struct_field_names, reason = "fields mirror FAA CSV columns")]
#[derive(PartialEq, Clone)]
pub struct Airport {
    site_number: String,
    site_type_code: String,
    pub(crate) state_code: String,
    pub(crate) airport_id: String,
    pub(crate) city: String,
    country_code: String,
    region_code: String,
    ado_code: String,
    pub(crate) state_name: String,
    county_name: String,
    county_associated_state: String,
    pub(crate) airport_name: String,
    ownership_type_code: String,
    facility_use_code: String,
    latitude_degree: String,
    latitude_minutes: String,
    latitude_seconds: String,
    latitude_hemisphere: String,
    latitude_decimal: String,
    longitude_degree: String,
    longitude_minutes: String,
    longitude_seconds: String,
    longitude_hemisphere: String,
    longitude_decimal: String,
    survey_method_code: String,
    elevation: String,
    elevation_method_code: String,
    magnetic_variation: String,
    magnetic_hemisphere: String,
    magnetic_variation_year: String,
    tpa: String,
    chart_name: String,
    distance_city_to_airport: String,
    direction_code: String,
    acreage: String,
    resp_artcc_id: String,
    computer_id: String,
    artcc_name: String,
    fss_on_airport_flag: String,
    fss_id: String,
    fss_name: String,
    phone_number: String,
    toll_free_number: String,
    alt_fss_id: String,
    alt_fss_name: String,
    alt_toll_free_number: String,
    notam_id: String,
    notam_flag: String,
    activation_date: String,
    airport_status: String,
    far_139_type_code: String,
    far_139_carrier_ser_code: String,
    arff_cert_type_date: String,
    nasp_code: String,
    asp_analysis_dtrm_code: String,
    custom_flag: String,
    landing_flights_flag: String,
    joint_use_flag: String,
    military_landing_flag: String,
    inspect_method_code: String,
    inspector_code: String,
    last_inspection: String,
    last_information_response: String,
    fuel_types: String,
    airframe_repair_service_code: String,
    powerplant_repair_service: String,
    bottled_oxygen_type: String,
    bulk_oxygen_type: String,
    lighting_schedule: String,
    beacon_light_schedule: String,
    tower_type_code: String,
    segment_circle_marker_flag: String,
    beacon_lens_color: String,
    landing_fee_flag: String,
    medical_use_flag: String,
    airport_position_source: String,
    airport_position_source_date: String,
    airport_elevation_source: String,
    airport_elevation_source_date: String,
    contr_fuel_available: String,
    trns_strg_buoy_flag: String,
    trns_strg_hgr_flag: String,
    trns_strg_tie_flag: String,
    other_services: String,
    wind_indicator_flag: String,
    icao_id: String,
    min_op_network: String,
    user_fee_flag: String,
    cta: String
}

pub struct ModifiedAirport {
    pub(crate) current_airport: Option<Airport>,
    pub(crate) new_airport: Option<Airport>,
}

#[derive(Serialize)]
struct AirportEntry {
    airport_id: String,
    airport_name: String,
    city: String,
    state_code: String,
    state_name: String,
}

#[derive(Serialize)]
struct RenamedEntry {
    airport_id: String,
    old_name: String,
    new_name: String,
    city: String,
    state_code: String,
    state_name: String,
}

#[derive(Serialize)]
struct AirportChanges {
    cycle: String,
    effective_date: String,
    opened: Vec<AirportEntry>,
    closed: Vec<AirportEntry>,
    renamed: Vec<RenamedEntry>,
}

pub fn read_airports(future_data: bool) -> Result<Vec<Airport>, Box<dyn std::error::Error>> {
    let path = if future_data {
        PREVIEW_AIRPORT_DATA_LOCATION
    } else {
        AIRPORT_DATA_LOCATION
    };

    let mut airport_list: Vec<Airport> = Vec::new();

    if !future_data && !Path::new(AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the APT_BASE.csv file and put it in the data folder, and restart the function.");

        thread::sleep(ONE_SECOND);

        return Err("APT Base missing".into())
    } else if future_data && !Path::new(PREVIEW_AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the upcoming changes and put it in the data folder as APT_BASE_NEW.csv, and restart the function.");

        thread::sleep(ONE_SECOND);

        return Err("New APT Base missing".into())
    }

    // Attempt to open the file
    let file =  File::open(path).expect("File failed to open");

    // Create a buffered reader to efficiently read the file line by line
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        // Check if reading the line was successful

        let clean_line = line?.replace('"', "");

        // Split the line by commas and collect the values into a vector
        let split_data: Vec<&str> = clean_line.split(',').collect();

        // Print the vector of values for the current line
        //println!("{:?}", split_data);

        // If the line is a valid airport data block, create an airport
        if split_data.first().expect("Unable to check if line is EFF_DATE").to_string() != "EFF_DATE" {
            let mut new_airport = Airport {
                site_number: split_data.get(1).expect("No value").to_string(),
                site_type_code: split_data.get(2).expect("No value").to_string(),
                state_code: split_data.get(3).expect("No value").to_string(),
                airport_id: split_data.get(4).expect("No value").to_string(),
                city: split_data.get(5).expect("No value").to_string(),
                country_code: split_data.get(6).expect("No value").to_string(),
                region_code: split_data.get(7).expect("No value").to_string(),
                ado_code: split_data.get(8).expect("No value").to_string(),
                state_name: split_data.get(9).expect("No value").to_string(),
                county_name: split_data.get(10).expect("No value").to_string(),
                county_associated_state: split_data.get(11).expect("No value").to_string(),
                airport_name: split_data.get(12).expect("No value").to_string(),
                ownership_type_code: split_data.get(13).expect("No value").to_string(),
                facility_use_code: split_data.get(14).expect("No value").to_string(),
                latitude_degree: split_data.get(15).expect("No value").to_string(),
                latitude_minutes: split_data.get(16).expect("No value").to_string(),
                latitude_seconds: split_data.get(17).expect("No value").to_string(),
                latitude_hemisphere: split_data.get(18).expect("No value").to_string(),
                latitude_decimal: split_data.get(19).expect("No value").to_string(),
                longitude_degree: split_data.get(20).expect("No value").to_string(),
                longitude_minutes: split_data.get(21).expect("No value").to_string(),
                longitude_seconds: split_data.get(22).expect("No value").to_string(),
                longitude_hemisphere: split_data.get(23).expect("No value").to_string(),
                longitude_decimal: split_data.get(24).expect("No value").to_string(),
                survey_method_code: split_data.get(25).expect("No value").to_string(),
                elevation: split_data.get(26).expect("No value").to_string(),
                elevation_method_code: split_data.get(27).expect("No value").to_string(),
                magnetic_variation: split_data.get(28).expect("No value").to_string(),
                magnetic_hemisphere: split_data.get(29).expect("No value").to_string(),
                magnetic_variation_year: split_data.get(30).expect("No value").to_string(),
                tpa: split_data.get(31).expect("No value").to_string(),
                chart_name: split_data.get(32).expect("No value").to_string(),
                distance_city_to_airport: split_data.get(33).expect("No value").to_string(),
                direction_code: split_data.get(34).expect("No value").to_string(),
                acreage: split_data.get(35).expect("No value").to_string(),
                resp_artcc_id: split_data.get(36).expect("No value").to_string(),
                computer_id: split_data.get(37).expect("No value").to_string(),
                artcc_name: split_data.get(38).expect("No value").to_string(),
                fss_on_airport_flag: split_data.get(39).expect("No value").to_string(),
                fss_id: split_data.get(40).expect("No value").to_string(),
                fss_name: split_data.get(41).expect("No value").to_string(),
                phone_number: split_data.get(42).expect("No value").to_string(),
                toll_free_number: split_data.get(43).expect("No value").to_string(),
                alt_fss_id: split_data.get(44).expect("No value").to_string(),
                alt_fss_name: split_data.get(45).expect("No value").to_string(),
                alt_toll_free_number: split_data.get(46).expect("No value").to_string(),
                notam_id: split_data.get(47).expect("No value").to_string(),
                notam_flag: split_data.get(48).expect("No value").to_string(),
                activation_date: split_data.get(49).expect("No value").to_string(),
                airport_status: split_data.get(50).expect("No value").to_string(),
                far_139_type_code: split_data.get(51).expect("No value").to_string(),
                far_139_carrier_ser_code: split_data.get(52).expect("No value").to_string(),
                arff_cert_type_date: split_data.get(53).expect("No value").to_string(),
                nasp_code: split_data.get(54).expect("No value").to_string(),
                asp_analysis_dtrm_code: split_data.get(55).expect("No value").to_string(),
                custom_flag: split_data.get(56).expect("No value").to_string(),
                landing_flights_flag: split_data.get(57).expect("No value").to_string(),
                joint_use_flag: split_data.get(58).expect("No value").to_string(),
                military_landing_flag: split_data.get(59).expect("No value").to_string(),
                inspect_method_code: split_data.get(60).expect("No value").to_string(),
                inspector_code: split_data.get(61).expect("No value").to_string(),
                last_inspection: split_data.get(62).expect("No value").to_string(),
                last_information_response: split_data.get(63).expect("No value").to_string(),
                fuel_types: split_data.get(64).expect("No value").to_string(),
                airframe_repair_service_code: split_data.get(65).expect("No value").to_string(),
                powerplant_repair_service: split_data.get(66).expect("No value").to_string(),
                bottled_oxygen_type: split_data.get(67).expect("No value").to_string(),
                bulk_oxygen_type: split_data.get(68).expect("No value").to_string(),
                lighting_schedule: split_data.get(69).expect("No value").to_string(),
                beacon_light_schedule: split_data.get(70).expect("No value").to_string(),
                tower_type_code: split_data.get(71).expect("No value").to_string(),
                segment_circle_marker_flag: split_data.get(72).expect("No value").to_string(),
                beacon_lens_color: split_data.get(73).expect("No value").to_string(),
                landing_fee_flag: split_data.get(74).expect("No value").to_string(),
                medical_use_flag: split_data.get(75).expect("No value").to_string(),
                airport_position_source: split_data.get(76).expect("No value").to_string(),
                airport_position_source_date: split_data.get(77).expect("No value").to_string(),
                airport_elevation_source: split_data.get(78).expect("No value").to_string(),
                airport_elevation_source_date: split_data.get(79).expect("No value").to_string(),
                contr_fuel_available: split_data.get(80).expect("No value").to_string(),
                trns_strg_buoy_flag: split_data.get(81).expect("No value").to_string(),
                trns_strg_hgr_flag: split_data.get(82).expect("No value").to_string(),
                trns_strg_tie_flag: split_data.get(83).expect("No value").to_string(),
                other_services: split_data.get(84).expect("No value").to_string(),
                wind_indicator_flag: split_data.get(85).expect("No value").to_string(),
                icao_id: split_data.get(86).expect("No value").to_string(),
                min_op_network: split_data.get(87).expect("No value").to_string(),
                user_fee_flag: split_data.get(88).expect("No value").to_string(),
                cta: split_data.get(89).expect("No value").to_string(),
            };

            if new_airport.icao_id.clone().is_empty() {
                new_airport.icao_id = new_airport.airport_id.clone();
            }

            airport_list.push(new_airport);

            // println!("New airport generated");

        }
    }

    println!("Airports read.");
    Ok(airport_list)
}

pub fn generate_airport_changes() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter cycle (e.g. 2510): ");
    io::Write::flush(&mut io::stdout())?;
    let mut cycle = String::new();
    io::stdin().read_line(&mut cycle)?;
    let cycle = cycle.trim().to_string();

    print!("Enter effective date (e.g. 2025-10-09): ");
    io::Write::flush(&mut io::stdout())?;
    let mut effective_date = String::new();
    io::stdin().read_line(&mut effective_date)?;
    let effective_date = effective_date.trim().to_string();
    let discord_timestamp = discord_timestamp(&effective_date)?.to_string();

    let current_airports: Vec<Airport> = airport::read_airports(false)?;
    let future_airports: Vec<Airport> = airport::read_airports(true)?;
    let mut opened_airports: Vec<ModifiedAirport> = Vec::new();
    let mut closed_airports: Vec<ModifiedAirport> = Vec::new();
    let mut renamed_airports: Vec<ModifiedAirport> = Vec::new();

    let mut airport_exists_in_new_data: bool;
    let mut airport_exists_in_current_data: bool;

    for airport in &current_airports {
        airport_exists_in_new_data = future_airports.iter().any(|x| x.airport_id == airport.airport_id);

        if !airport_exists_in_new_data {
            let modified_airport: ModifiedAirport = ModifiedAirport {
                current_airport: Option::from(airport).cloned(),
                new_airport: None,
            };

            closed_airports.push(modified_airport);
        }
    }

    println!("Closed airports listed");

    for airport in &future_airports {
        airport_exists_in_current_data = current_airports.iter().any(|x| x.airport_id == airport.airport_id);

        if !airport_exists_in_current_data {
            let modified_airport: ModifiedAirport = ModifiedAirport {
                current_airport: None,
                new_airport: Option::from(airport).cloned(),
            };

            opened_airports.push(modified_airport);
        }
    }

    println!("Opened airports listed");

    for current_airport_loop in current_airports {
        airport_exists_in_new_data = future_airports.iter().any(|x| x.airport_id == current_airport_loop.airport_id);

        if airport_exists_in_new_data {
            let future_airport: Option<&Airport> = future_airports.iter().find(|x| x.airport_id == current_airport_loop.airport_id);
            let name_change: bool = current_airport_loop.airport_name != future_airport.expect("New airport name not defined").airport_name;

            if name_change {
                let modified_airport: ModifiedAirport = ModifiedAirport {
                    current_airport: Option::from(current_airport_loop),
                    new_airport: future_airport.cloned(),
                };

                renamed_airports.push(modified_airport);
            }
        }
    }

    println!("Renamed airports listed");

    // Outputting list
    let path = format!("data/output/airport/{cycle}.txt");
    let states = ["CALIFORNIA", "OREGON", "WASHINGTON", "NEVADA", "UTAH", "ARIZONA", "NEW MEXICO", "COLORADO", "WYOMING", "IDAHO", "MONTANA"];

    let mut changes = AirportChanges {
        cycle,
        effective_date,
        opened: Vec::new(),
        closed: Vec::new(),
        renamed: Vec::new(),
    };

    if let Ok(mut file) = File::create(&path) {
        writeln!(file, "# **Airport changes effective {} // CYCLE {} **", discord_timestamp, changes.cycle)?;

        for modified_airport in opened_airports {
            if states.contains(&modified_airport.new_airport.as_ref().expect("State not defined").state_name.as_str()) {
                let a = modified_airport.new_airport.as_ref().expect("Airport not defined");
                writeln!(file, "{} - {} // OPENED ({}, {})", a.airport_id, a.airport_name, a.city, a.state_code)?;
                changes.opened.push(AirportEntry {
                    airport_id: a.airport_id.clone(),
                    airport_name: a.airport_name.clone(),
                    city: a.city.clone(),
                    state_code: a.state_code.clone(),
                    state_name: a.state_name.clone(),
                });
            }
        }

        writeln!(file, " ")?;

        for modified_airport in closed_airports {
            if states.contains(&modified_airport.current_airport.as_ref().expect("State not defined").state_name.as_str()) {
                let a = modified_airport.current_airport.as_ref().expect("Airport not defined");
                writeln!(file, "{} - {} // CLOSED ({}, {})", a.airport_id, a.airport_name, a.city, a.state_code)?;
                changes.closed.push(AirportEntry {
                    airport_id: a.airport_id.clone(),
                    airport_name: a.airport_name.clone(),
                    city: a.city.clone(),
                    state_code: a.state_code.clone(),
                    state_name: a.state_name.clone(),
                });
            }
        }

        writeln!(file, " ")?;

        for modified_airport in renamed_airports {
            if states.contains(&modified_airport.current_airport.as_ref().expect("State not defined").state_name.as_str()) {
                let cur = modified_airport.current_airport.as_ref().expect("Current Airport not defined");
                let new = modified_airport.new_airport.as_ref().expect("New Airport not defined");
                writeln!(file, "{} - {} // RENAMED {} ({}, {})", cur.airport_id, cur.airport_name, new.airport_name, cur.city, cur.state_code)?;
                changes.renamed.push(RenamedEntry {
                    airport_id: cur.airport_id.clone(),
                    old_name: cur.airport_name.clone(),
                    new_name: new.airport_name.clone(),
                    city: cur.city.clone(),
                    state_code: cur.state_code.clone(),
                    state_name: cur.state_name.clone(),
                });
            }
        }
    }

    println!("File outputted at {path}");

    let json_path = format!("data/output/airport/{}.json", changes.cycle);
    let json_data = serde_json::to_string_pretty(&changes)?;
    fs::write(&json_path, json_data)?;
    println!("JSON file outputted at {json_path}");

    thread::sleep(ONE_SECOND);

    Ok(())
}

pub fn export_airport_list() -> Result<(), Box<dyn std::error::Error>> {
    let airports: Vec<Airport> = read_airports(false)?;
    let states = ["CALIFORNIA", "OREGON", "WASHINGTON", "NEVADA", "UTAH", "ARIZONA", "NEW MEXICO", "COLORADO", "WYOMING", "IDAHO", "MONTANA"];
    let path = "data/output/airports.csv";
    let i: i32 = 0;

    if let Ok(mut file) = File::create(path) {
        // writeln!(file, "id,AirportCode,AirportName,Class,AfdLink,CTAF,Atis,AtisPhone,Tower,Ground,Clearance,Elevation,TPA,noK,Ownership,Use,Latitude,Longitude,PEInactive,ARTCC,County,City,StateCode,StateName,created_at,updated_at").unwrap();

        for airport in airports {
            if states.contains(&&*airport.state_name) {
                writeln!(file, "{},{},{},tbc,tbc,tbc,tbc,tbc,tbc,tbc,tbc,{},{},tbc,{},{},{},{},false,{},{},{},{},{},,{}", i, airport.airport_id, airport.airport_name, airport.elevation, airport.tpa, airport.ownership_type_code, airport.facility_use_code, airport.latitude_decimal, airport.longitude_decimal, airport.artcc_name, airport.county_name, airport.city, airport.state_code, airport.state_name, airport.icao_id)?;
                i.checked_add(1).expect("Airport counter overflowed");
            }
        }
    }

    println!("Airports exported.");
    thread::sleep(ONE_SECOND);

    Ok(())
}

fn discord_timestamp(effective_date: &str) -> Result<String, chrono::ParseError> {
    let date = NaiveDate::parse_from_str(effective_date, "%Y-%m-%d")?;
    let datetime = date.and_hms_opt(9, 0, 0).expect("valid time");
    let utc_datetime = Utc.from_utc_datetime(&datetime);

    Ok(format!("<t:{}:R>", utc_datetime.timestamp()))
}