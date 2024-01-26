use std::{path::Path, thread};
use read_lines_into::traits::ReadLinesIntoStringsOnRefSelf;

use crate::ONE_SECOND;

const AIRPORT_DATA_LOCATION: &str = "data/APT_BASE.csv";
const PREVIEW_AIRPORT_DATA_LOCATION: &str = "data/APT_BASE_NEW.csv";

pub struct Airport {
    state_code: String,
    airport_id: String,
    city: String,
    country_code: String,
    region_code: String,
    ado_code: String,
    state_name: String,
    county_name: String,
    county_assoiated_state: String,
    airport_name: String,
    ownership_type_code: String,
    facility_use_code: String,
    latitude_degree: String,
    latitude_minutes: String,
    latitude_seconds: String,
    latitide_hemisphere: String,
    latitude_decimal: String,
    longitude_degree: String,
    longitude_minutes: String,
    longitude_seconds: String,
    longitude_hemisphere: String,
    longitude_decimal: String,
    survey_method_code: String,
    elevation: f32,
    elevation_method_code: String,
    magnetic_variation: String,
    magnetic_hemisphere: String,
    magnetic_variation_year: String,
    tpa: String,
    chart_name: String,
    distance_city_to_airport: String,
    direction_code: String,
    acreage: f32,
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
    last_information_responce: String,
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

fn read_airports(future_data: bool){
    let mut path: &Path;

    if !future_data && !Path::new(AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the APT_BASE.csv file and put it in the data folder, and restart the function.");

        thread::sleep(ONE_SECOND);

        return;
    } else if future_data && !Path::new(PREVIEW_AIRPORT_DATA_LOCATION).exists() {
        println!("Please download the upcoming changes and put it in the data folder as APT_BASE_NEW.csv, and restart the function.");

        thread::sleep(ONE_SECOND);

        return;
    }

    if !future_data {
        path = Path::new(AIRPORT_DATA_LOCATION);
    } else {
        path = Path::new(PREVIEW_AIRPORT_DATA_LOCATION);
    }

    let mut raw_data: Vec<String> = path.read_lines_into_vec_string().unwrap();

    for i in 1..raw_data.len() {
        let data_line: String = raw_data.get(i).unwrap().to_string();

        let split_data = data_line.split(',');

    }
}