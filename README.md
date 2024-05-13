# FAA NASR Data Processor
![GitHub Release](https://img.shields.io/github/v/release/MyFlightRoute/FAA-NASR-Data-Processor)
![Discord](https://img.shields.io/discord/1183045672379887741)

## Description
The NASR Data processor is a tool for extracting data from the FAA NASR files and make them useful for MyFlightRoute. Additionally, it creates changelogs for Californian TEC Routes and Airport changes inside of the [PilotEdge](https://pilotedge.net) service area.

## Requirements
Running the Program
- [NASR Data](https://www.faa.gov/air_traffic/flight_info/aeronav/Aero_Data/NASR_Subscription/)
  - Currently, only Airport and Preferred Route/Tower En route Control data is used.


## Usage
1. Download the latest release from GitHub.
2. Download two copies of the NASR data from different data cycles. APT and PFR copies are required for the application to fully function.
3. Create a folder in the same as the executable called "data". Alternatively, run the program and it will create it for you
4. Extract the APT_BASE.csv and PFR_BASE.csv files from the **newer** nav data cycle into the data folder. 
5. Rename these files to APT_BASE_NEW.csv and PFR_BASE_NEW.csv
6. Extract the APT_BASE.csv and PFR_BASE.csv from the **older** nav data cycle into the data folder.
7. Run the program and enjoy.

## Contributing
Please see [CONTRIBUTING.md](CONTRIBUTING)

## Contact
To contact me, please email contact@myflightroute.com or join our [Discord](https://discord.gg/RcGaSD4Wcm) server.