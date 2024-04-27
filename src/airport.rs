use std::{path::Path, thread, io::{self, BufRead, Write}};
use std::fs::File;

use crate::ONE_SECOND;

const AIRPORT_DATA_LOCATION: &str = "data/APT_BASE.csv";
const PREVIEW_AIRPORT_DATA_LOCATION: &str = "data/APT_BASE_NEW.csv";

#[derive(PartialEq, Clone)]
pub struct Airport {
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
    based_single_engine: String,
    based_multi_engine: String,
    based_jet_engine: String,
    based_helicopter: String,
    based_gliders: String,
    based_military_aircraft: String,
    based_ultralight_aircraft: String,
    commercial_ops: String,
    commuter_ops: String,
    air_taxi_ops: String,
    local_ops: String,
    intermittent_ops: String,
    military_aircraft_ops: String,
    annual_ops_date: String,
    airport_position_source: String,
    position_source_date: String,
    airport_elevation_source: String,
    elevation_source_date: String,
    contr_fuel_available: String,
    transient_storage_buoy_flag: String,
    transient_storage_hangar_flag: String,
    transient_storage_tie_flag: String,
    other_services: String,
    wind_indicator_flag: String,
    icao_id: String,
    minimum_operational_network: String,
    user_fee_flag: String,
    cta: String,
}

pub struct ModifiedAirport {
    pub(crate) current_airport: Option<Airport>,
    pub(crate) new_airport: Option<Airport>,
}

pub fn read_airports(future_data: bool) -> Vec<Airport> {
    let path = if !future_data {
        AIRPORT_DATA_LOCATION
    } else {
        PREVIEW_AIRPORT_DATA_LOCATION
    };

    let mut airport_list: Vec<Airport> = Vec::new();

    if !future_data && !Path::new(AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the APT_BASE.csv file and put it in the data folder, and restart the function.");

        thread::sleep(ONE_SECOND);

        return airport_list;
    } else if future_data && !Path::new(PREVIEW_AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the upcoming changes and put it in the data folder as APT_BASE_NEW.csv, and restart the function.");

        thread::sleep(ONE_SECOND);

        return airport_list;
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

        // If the line is a valid airport data block, create an airport
        if split_data[0] != "EFF_DATE" {
            let new_airport = Airport {
                state_code: split_data[3].to_string(),
                airport_id: split_data[4].to_string(),
                city: split_data[5].to_string(),
                country_code: split_data[6].to_string(),
                region_code: split_data[7].to_string(),
                ado_code: split_data[8].to_string(),
                state_name: split_data[9].to_string(),
                county_name: split_data[10].to_string(),
                county_associated_state: split_data[11].to_string(),
                airport_name: split_data[12].to_string(),
                ownership_type_code: split_data[13].to_string(),
                facility_use_code: split_data[14].to_string(),
                latitude_degree: split_data[15].to_string(),
                latitude_minutes: split_data[16].to_string(),
                latitude_seconds: split_data[17].to_string(),
                latitude_hemisphere: split_data[18].to_string(),
                latitude_decimal: split_data[19].to_string(),
                longitude_degree: split_data[20].to_string(),
                longitude_minutes: split_data[21].to_string(),
                longitude_seconds: split_data[22].to_string(),
                longitude_hemisphere: split_data[23].to_string(),
                longitude_decimal: split_data[24].to_string(),
                survey_method_code: split_data[25].to_string(),
                elevation: split_data[26].to_string(),
                elevation_method_code: split_data[27].to_string(),
                magnetic_variation: split_data[28].to_string(),
                magnetic_hemisphere: split_data[29].to_string(),
                magnetic_variation_year: split_data[30].to_string(),
                tpa: split_data[31].to_string(),
                chart_name: split_data[32].to_string(),
                distance_city_to_airport: split_data[33].to_string(),
                direction_code: split_data[34].to_string(),
                acreage: split_data[35].to_string(),
                resp_artcc_id: split_data[36].to_string(),
                computer_id: split_data[37].to_string(),
                artcc_name: split_data[38].to_string(),
                fss_on_airport_flag: split_data[39].to_string(),
                fss_id: split_data[40].to_string(),
                fss_name: split_data[41].to_string(),
                phone_number: split_data[42].to_string(),
                toll_free_number: split_data[43].to_string(),
                alt_fss_id: split_data[44].to_string(),
                alt_fss_name: split_data[45].to_string(),
                alt_toll_free_number: split_data[46].to_string(),
                notam_id: split_data[47].to_string(),
                notam_flag: split_data[48].to_string(),
                activation_date: split_data[49].to_string(),
                airport_status: split_data[50].to_string(),
                far_139_type_code: split_data[51].to_string(),
                far_139_carrier_ser_code: split_data[52].to_string(),
                arff_cert_type_date: split_data[53].to_string(),
                nasp_code: split_data[54].to_string(),
                asp_analysis_dtrm_code: split_data[55].to_string(),
                custom_flag: split_data[56].to_string(),
                landing_flights_flag: split_data[57].to_string(),
                joint_use_flag: split_data[58].to_string(),
                military_landing_flag: split_data[59].to_string(),
                inspect_method_code: split_data[60].to_string(),
                inspector_code: split_data[61].to_string(),
                last_inspection: split_data[62].to_string(),
                last_information_response: split_data[63].to_string(),
                fuel_types: split_data[64].to_string(),
                airframe_repair_service_code: split_data[65].to_string(),
                powerplant_repair_service: split_data[66].to_string(),
                bottled_oxygen_type: split_data[67].to_string(),
                bulk_oxygen_type: split_data[68].to_string(),
                lighting_schedule: split_data[69].to_string(),
                beacon_light_schedule: split_data[70].to_string(),
                tower_type_code: split_data[71].to_string(),
                segment_circle_marker_flag: split_data[72].to_string(),
                beacon_lens_color: split_data[73].to_string(),
                landing_fee_flag: split_data[74].to_string(),
                medical_use_flag: split_data[75].to_string(),
                based_single_engine: split_data[76].to_string(),
                based_multi_engine: split_data[77].to_string(),
                based_jet_engine: split_data[78].to_string(),
                based_helicopter: split_data[79].to_string(),
                based_gliders: split_data[80].to_string(),
                based_military_aircraft: split_data[81].to_string(),
                based_ultralight_aircraft: split_data[82].to_string(),
                commercial_ops: split_data[83].to_string(),
                commuter_ops: split_data[84].to_string(),
                air_taxi_ops: split_data[85].to_string(),
                local_ops: split_data[86].to_string(),
                intermittent_ops: split_data[87].to_string(),
                military_aircraft_ops: split_data[88].to_string(),
                annual_ops_date: split_data[89].to_string(),
                airport_position_source: split_data[90].to_string(),
                position_source_date: split_data[91].to_string(),
                airport_elevation_source: split_data[92].to_string(),
                elevation_source_date: split_data[93].to_string(),
                contr_fuel_available: split_data[94].to_string(),
                transient_storage_buoy_flag: split_data[95].to_string(),
                transient_storage_hangar_flag: split_data[96].to_string(),
                transient_storage_tie_flag: split_data[97].to_string(),
                other_services: split_data[98].to_string(),
                wind_indicator_flag: split_data[99].to_string(),
                icao_id: split_data[100].to_string(),
                minimum_operational_network: split_data[101].to_string(),
                user_fee_flag: split_data[102].to_string(),
                cta: split_data[103].to_string(),
            };

        airport_list.push(new_airport);

        // println!("New airport generated");

        }
    }

    println!("Airports read.");
    airport_list
}

pub fn export_airport_list() {
    let airports: Vec<Airport> = read_airports(false);
    let states = ["CALIFORNIA", "OREGON", "WASHINGTON", "NEVADA", "UTAH", "ARIZONA", "NEW MEXICO", "COLORADO", "WYOMING", "IDAHO", "MONTANA"];
    let path = "data/output/airports.csv";
    let mut i = 0;

    if let Ok(mut file) = File::create(path) {
        writeln!(file, "id,AirportCode,AirportName,Class,AfdLink,CTAF,Atis,AtisPhone,Tower,Ground,Clearance,Elevation,TPA,noK,Ownership,Use,Latitude,Longitude,PEInactive,ARTCC,County,City,StateCode,StateName,created_at,updated_at").unwrap();

        for airport in airports {
            if states.contains(&&*airport.state_name) {
                writeln!(file, "{},{},{},tbc,tbc,tbc,tbc,tbc,tbc,tbc,tbc,{},{},tbc,{},{},{},{},false,{},{},{},{},{},,", i.to_string(), airport.airport_id, airport.airport_name, airport.elevation, airport.tpa, airport.ownership_type_code, airport.facility_use_code, airport.latitude_decimal, airport.longitude_decimal, airport.artcc_name, airport.county_name, airport.city, airport.state_code, airport.state_name).unwrap();
                i += 1;
            }
        }
    }

    println!("Airports exported.");
    thread::sleep(ONE_SECOND);
}