# FAA NASR Data Processor

## Description

The NASR Data processor is a tool for extracting data from the FAA NASR files and make them useful for MyFlightRoute.

With some tweaking, it will be possible to modify the output to make it useful for other applications, using upcoming config files.

## Requirements

- [NASR Data]([28 Day NASR Subscription](https://www.faa.gov/air_traffic/flight_info/aeronav/Aero_Data/NASR_Subscription/))
  - Currently, only Airport and Preferred Route/Tower Enroute Control data is used.

## Usage

- Download the latest release from the repository and extract it.
- In the Data folder, add the current airport csv data file. For the preview file for the next subscription, add _NEW at the end, and then put it in the data folder also. Repeat for the PFR csv files.
- Generate the differences.

## Contributing

I am currently working on porting the project to Rust. When this is completed, I will add more information here.
