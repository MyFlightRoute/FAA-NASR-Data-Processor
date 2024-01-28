use std::{path::Path, thread, io::{self, BufRead}};
use read_lines_into::traits::ReadLinesIntoStringsOnRefSelf;
use std::fs::File;

use crate::ONE_SECOND;

const AIRPORT_DATA_LOCATION: &str = "data/APT_BASE.csv";
const PREVIEW_AIRPORT_DATA_LOCATION: &str = "data/APT_BASE_NEW.csv";

pub struct Airport {
    state_code: &str,
    airport_id: &str,
    city: &str,
    country_code: &str,
    region_code: &str,
    ado_code: &str,
    state_name: &str,
    county_name: &str,
    county_assoiated_state: &str,
    airport_name: &str,
    ownership_type_code: &str,
    facility_use_code: &str,
    latitude_degree: &str,
    latitude_minutes: &str,
    latitude_seconds: &str,
    latitide_hemisphere: &str,
    latitude_decimal: &str,
    longitude_degree: &str,
    longitude_minutes: &str,
    longitude_seconds: &str,
    longitude_hemisphere: &str,
    longitude_decimal: &str,
    survey_method_code: &str,
    elevation: &str,
    elevation_method_code: &str,
    magnetic_variation: &str,
    magnetic_hemisphere: &str,
    magnetic_variation_year: &str,
    tpa: &str,
    chart_name: &str,
    distance_city_to_airport: &str,
    direction_code: &str,
    acreage: &str,
    resp_artcc_id: &str,
    computer_id: &str,
    artcc_name: &str,
    fss_on_airport_flag: &str,
    fss_id: &str,
    fss_name: &str,
    phone_number: &str,
    toll_free_number: &str,
    alt_fss_id: &str,
    alt_fss_name: &str,
    alt_toll_free_number: &str,
    notam_id: &str,
    notam_flag: &str,
    activation_date: &str,
    airport_status: &str,
    far_139_type_code: &str,
    far_139_carrier_ser_code: &str,
    arff_cert_type_date: &str,
    nasp_code: &str,
    asp_analysis_dtrm_code: &str,
    custom_flag: &str,
    landing_flights_flag: &str,
    joint_use_flag: &str,
    military_landing_flag: &str,
    inspect_method_code: &str,
    inspector_code: &str,
    last_inspection: &str,
    last_information_responce: &str,
    fuel_types: &str,
    airframe_repair_service_code: &str,
    powerplant_repair_service: &str,
    bottled_oxygen_type: &str,
    bulk_oxygen_type: &str,
    lighting_schedule: &str,
    beacon_light_schedule: &str,
    tower_type_code: &str,
    segment_circle_marker_flag: &str,
    beacon_lens_color: &str,
    landing_fee_flag: &str,
    medical_use_flag: &str,
    based_single_engine: &str,
    based_multi_engine: &str,
    based_jet_engine: &str,
    based_helicopter: &str,
    based_gliders: &str,
    based_military_aircraft: &str,
    based_ultralight_aircraft: &str,
    commercial_ops: &str,
    commuter_ops: &str,
    air_taxi_ops: &str,
    local_ops: &str,
    intermittent_ops: &str,
    military_aircraft_ops: &str,
    annual_ops_date: &str,
    airport_position_source: &str,
    position_source_date: &str,
    elevation_source_date: &str,
    contr_fuel_available: &str,
    transient_storage_buoy_flag: &str,
    transient_storage_hangar_flag: &str,
    transient_storage_tie_flag: &str,
    other_services: &str,
    wind_indicator_flag: &str,
    icao_id: &str,
    minimum_operational_network: &str,
    user_fee_flag: &str,
    cta: &str,
}

pub fn read_airports(future_data: bool) -> io::Result<()>{
    let path: &str;

    if !future_data && !Path::new(AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the APT_BASE.csv file and put it in the data folder, and restart the function.");

        thread::sleep(ONE_SECOND);
    } else if future_data && !Path::new(PREVIEW_AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the upcoming changes and put it in the data folder as APT_BASE_NEW.csv, and restart the function.");

        thread::sleep(ONE_SECOND);
    }

    if !future_data {
        path = AIRPORT_DATA_LOCATION;
    } else {
        path = PREVIEW_AIRPORT_DATA_LOCATION;
    }

    // Attempt to open the file
    if let Ok(file) = File::open(&path) {
        // Create a buffered reader to efficiently read the file line by line
        let reader = io::BufReader::new(file);

        // Iterate over each line in the file
        for line in reader.lines() {
            // Check if reading the line was successful
            if let Ok(line) = line {
                let clean_line = line.replace('"', "");

                // Split the line by commas and collect the values into a vector
                let values: Vec<&str> = clean_line.split(',').collect();

                // Print the vector of values for the current line
                println!("{:?}", values);

                if values[0] != "EFF_DATE" {
                    let new_airport = Airport {
                        state_code: values[3],
                        airport_id: values[4]
                    };
                }
            }
        }
    } else {
        // Print an error message if opening the file fails
        println!("Error opening the file: {}", path);
    }

    Ok(())
}