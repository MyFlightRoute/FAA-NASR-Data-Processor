using System;
using System.Collections.Generic;
using ConsoleMenu;
using System.IO;
using System.Linq;
using System.Threading;
using ShellProgressBar;

namespace FAA_Data_Processor
{
    class Program
    {
        static void Main(string[] args)
        {
            if (!Directory.Exists("data"))
            {
                Directory.CreateDirectory("data");
            }
            else
            {
                StartUp();
            }
            
            while (true)
            {
                Console.Clear();
                MainMenu();
            }
        }

        static List<Airport> GenerateAirportList(String[] rawData, bool newData = false)
        {
            List<Airport> airports = new List<Airport>();
            List<string> airportHoldingData = new List<string>();

            if (Globals.Cifp)
            {

                using (ProgressBar progressBar = new ProgressBar(rawData.Length, "Reading raw airport data", Globals.ProgressBarOptions))
                {
                    foreach (var data in rawData)
                    {
                        if (data.Contains("SUSAP"))
                        {
                            airportHoldingData.Add(data);
                        }

                        progressBar.Tick();
                    }

                    Thread.Sleep(500);
                }
            }
            else
            {
                if (!newData)
                {
                    rawData = File.ReadAllLines("data/APT_BASE.csv");
                }
                else
                {
                    rawData = File.ReadAllLines("data/APT_BASE_NEW.csv");
                }

                foreach (var data in rawData) 
                {
                    airportHoldingData.Add(data);
                }
            }

            Console.WriteLine("Raw airport data read in successfully. {0} records added.", airportHoldingData.Count.ToString());

            Thread.Sleep(1000);

            using (ProgressBar progressBar = new ProgressBar(airportHoldingData.Count, "Processing airport data",
                        Globals.ProgressBarOptions))
            {
                for (int i = 0; i < airportHoldingData.Count; i++)
                {
                    airportHoldingData[i] = airportHoldingData[i].Replace("\"", "");
                    
                    Airport newAirport = new Airport(airportHoldingData[i]);
                    string[] splitData = airportHoldingData[i].Split(',');
                    
                    bool duplicateAirport = false;

                    if (!Globals.Cifp)
                    {
                        newAirport.StateCode = splitData[3].ToString();
                        newAirport.AirportId = splitData[4].ToString();
                        newAirport.City = splitData[5].ToString();
                        newAirport.CountryCode = splitData[6].ToString();
                        newAirport.RegionCode  = splitData[7].ToString();
                        newAirport.AdoCode = splitData[8].ToString();
                        newAirport.StateName = splitData[9].ToString();
                        newAirport.CountyName = splitData[10].ToString();
                        newAirport.CountyAssoiatedState = splitData[11].ToString();
                        
                        newAirport.AirportName = splitData[12].ToString();
                        newAirport.OwnershipTypeCode = splitData[13].ToString();
                        newAirport.FacilityUseCode = splitData[14].ToString();
                        
                        newAirport.LatitudeDegree = splitData[15].ToString();
                        newAirport.LatitudeMinutes = splitData[16].ToString();
                        newAirport.LatitudeSeconds = splitData[17].ToString();
                        newAirport.LatitudeHemisphere = splitData[18].ToString();
                        newAirport.LatitudeDecimal = splitData[19].ToString();
                        
                        newAirport.LongitudeDegree = splitData[20].ToString();
                        newAirport.LongitudeMinutes = splitData[21].ToString();
                        newAirport.LongitudeSeconds = splitData[22].ToString();
                        newAirport.LongitudeHemisphere = splitData[23].ToString();
                        newAirport.LongitudeDecimal = splitData[24].ToString();

                        newAirport.SurveyMethodCode = splitData[25].ToString();
                        newAirport.Elevation = splitData[26].ToString();
                        newAirport.ElevationMethodCode = splitData[27].ToString();
                        
                        newAirport.MagneticVariation = splitData[28].ToString();
                        newAirport.MagneticHemisphere = splitData[29].ToString();
                        newAirport.MagneticVariationYear = splitData[30].ToString();

                        newAirport.Tpa = splitData[31].ToString();

                        newAirport.ChartName = splitData[32].ToString();

                        newAirport.DistanceCityToAirport = splitData[33].ToString();
                        newAirport.DirectionCode = splitData[34].ToString();
                        newAirport.Acreage = splitData[35].ToString();

                        newAirport.RespArtccId = splitData[36].ToString();
                        newAirport.ComputerId = splitData[37].ToString();
                        newAirport.ArtccName = splitData[38].ToString();
                        newAirport.FssOnAirportFlag = splitData[39].ToString();
                        newAirport.FssId = splitData[40].ToString();
                        newAirport.FssName = splitData[41].ToString();

                        newAirport.PhoneNumber = splitData[42].ToString();
                        newAirport.TollFreeNumber = splitData[43].ToString();

                        newAirport.AltFssId = splitData[44].ToString();
                        newAirport.AltFssName = splitData[45].ToString();
                        newAirport.AltTollFreeNumber = splitData[46].ToString();

                        newAirport.NotamId = splitData[47].ToString();
                        newAirport.NotamFlag = splitData[48].ToString();

                        newAirport.ActivationDate = splitData[49].ToString();
                        newAirport.AirportStatus = splitData[50].ToString();

                        newAirport.Far139TypeCode = splitData[51].ToString();
                        newAirport.Far139CarrierSerCode = splitData[52].ToString();

                        newAirport.ArffCertTypeDate = splitData[53].ToString();
                        newAirport.NaspCode = splitData[54].ToString();
                        newAirport.AspAnalysisDtrmCode = splitData[55].ToString();
                        
                        newAirport.CustomFlag = splitData[56].ToString();
                        newAirport.LandingRightsFlag = splitData[57].ToString();
                        newAirport.JointUseFlag = splitData[58].ToString();
                        newAirport.MilitaryLandingFlag = splitData[59].ToString();

                        newAirport.InspectMethodCode = splitData[60].ToString();
                        newAirport.InspectorCode = splitData[61].ToString();
                        newAirport.LastInspection = splitData[62].ToString();
                        newAirport.LastInformationResponce = splitData[63].ToString();

                        newAirport.FuelTypes = splitData[64].ToString();
                        newAirport.AirframeRepairServiceCode = splitData[65].ToString();
                        newAirport.PowerplantRepairService = splitData[66].ToString();
                        newAirport.BottledOxygenType = splitData[67].ToString();
                        newAirport.BulkOxygenType = splitData[68].ToString();

                        newAirport.LightingSchedule = splitData[69].ToString();
                        newAirport.BeaconLightSchedule = splitData[70].ToString();
                        newAirport.TowerTypeCode = splitData[71].ToString();
                        newAirport.SegmentCircleMarkerFlag = splitData[72].ToString();
                        newAirport.BeaconLensColor = splitData[73].ToString();
                        newAirport.LandingFeeFlag = splitData[74].ToString();
                        newAirport.MedicalUseFlag = splitData[75].ToString();

                        newAirport.BasedSingleEngine = splitData[76].ToString();
                        newAirport.BasedMultiEngine = splitData[77].ToString();
                        newAirport.BasedJetEngine = splitData[78].ToString();
                        newAirport.BasedHelicopter = splitData[79].ToString();
                        newAirport.BasedGliders = splitData[80].ToString();
                        newAirport.BasedMilitaryAircraft = splitData[81].ToString();
                        newAirport.BasedUltralightAircaft = splitData[82].ToString();

                        newAirport.CommercialOps = splitData[83].ToString();
                        newAirport.CommuterOps = splitData[84].ToString();
                        newAirport.AirTaxiOps = splitData[85].ToString();
                        newAirport.LocalOps = splitData[86].ToString();
                        newAirport.IntermittentOps = splitData[87].ToString();
                        newAirport.MilitaryAircraftOps = splitData[88].ToString();

                        newAirport.AnnualOpsDate = splitData[89].ToString();
                        newAirport.AirportPositionSource = splitData[90].ToString();
                        newAirport.PositionSourceDate = splitData[91].ToString();
                        newAirport.AirportElevationSource = splitData[92].ToString();
                        newAirport.ElevationSourceDate = splitData[93].ToString();

                        newAirport.ContrFuelAvailable = splitData[94].ToString();
                        newAirport.TransientStorageBuoyFlag = splitData[95].ToString();
                        newAirport.TransientStorageHangarFlag = splitData[96].ToString();
                        newAirport.TransientStorageTieFlag = splitData[97].ToString();
                        newAirport.OtherServices = splitData[98].ToString();

                        newAirport.WindIndicatorFlag = splitData[99].ToString();
                        newAirport.IcaoId = splitData[100].ToString();
                        newAirport.MinimumOperationalNetwork = splitData[101].ToString();
                        newAirport.UserFeeFlag = splitData[102].ToString();
                        newAirport.Cta = splitData[103].ToString();
                    }

                    if (i > 1 && Globals.Cifp)
                    {
                        duplicateAirport = (newAirport.IcaoId == airports.Last().IcaoId);
                    }

                    if (!duplicateAirport && i != 0)
                    {
                        airports.Add(newAirport);
                    }

                    progressBar.Tick();
                }
            }

            Console.WriteLine("Airports list generated. There are {0} airports in the database.", airports.Count);
            
            Thread.Sleep(2000);
            
            return airports;
        }
        
        static void GenerateAiportChangesList()
        {
            List<Airport> currentAirports = Globals.Airports;
            List<Airport> nextAirports = GenerateAirportList(Globals.RawCifpData, true);
            List<ModifiedAirport> modifiedAirports = new List<ModifiedAirport>();

            bool airportExistsInNewData, airportExistsInCurrentData;

            foreach (Airport airport in currentAirports)
            {
                airportExistsInNewData = nextAirports.Exists(x => x.AirportId == airport.AirportId);

                if (!airportExistsInNewData) 
                { 
                    ModifiedAirport modifiedAirport = new ModifiedAirport();
                    modifiedAirport.CurrentAirport = airport;
                    modifiedAirport.Closed = true;
                    modifiedAirport.IsModified = true;

                    modifiedAirports.Add(modifiedAirport);
                }
            }

            foreach (Airport airport in nextAirports)
            {
                airportExistsInCurrentData = currentAirports.Exists(x => x.AirportId == airport.AirportId);

                if (!airportExistsInCurrentData)
                {
                    ModifiedAirport modifiedAirport = new ModifiedAirport();
                    modifiedAirport.NewAirport = airport;
                    modifiedAirport.Opened = true;
                    modifiedAirport.IsModified = true;

                    modifiedAirports.Add(modifiedAirport);
                }
            }

            foreach (Airport currentAirport in currentAirports)
            {
                ModifiedAirport nextChangedAirport = new ModifiedAirport();

                foreach (var nextAirport in nextAirports)
                {
                    if (nextAirport.Equals(currentAirport))
                    {
                        nextChangedAirport.CurrentAirport = currentAirport;
                        nextChangedAirport.NewAirport = currentAirport;
                        nextChangedAirport.RunChanges();

                        if (nextChangedAirport.IsModified)
                        {
                            modifiedAirports.Add(nextChangedAirport);
                        }
                    }
                }
            }

            Console.WriteLine("Modified airports database created.");

            Globals.ModifiedAirports = modifiedAirports;

            Thread.Sleep(1000);
        }

        static void WriteAirportChanges(List<ModifiedAirport> modifiedAirports)
        {
            string path = "data/changed_airports.txt";
            string[] States = { "CALIFORNIA", "OREGON", "Washington", "NEVADA", "UTAH", "ARIZONA", "NEW MEXICO", "COLORADO", "WYOMING", "IDAHO", "MONTANA" };

/*            if (File.Exists(path)) 
            { 
                File.Delete(path);
                File.Create(path);
            }*/

            using(FileStream fileStream = new FileStream(path, FileMode.Append))
            {
                using(StreamWriter writer = new StreamWriter(fileStream))
                {
                    writer.WriteLine("**Airport changes effective  // CYCLE**");

                    foreach (ModifiedAirport airport in modifiedAirports)
                    {
                        if (airport.CurrentAirport == null)
                        {
                            if (States.Contains(airport.NewAirport.StateName))
                            {
                                if (airport.Opened)
                                {
                                    writer.WriteLine("{0} - {1} - OPENED", airport.NewAirport.AirportId, airport.NewAirport.AirportName);
                                }
                                else if (airport.Closed)
                                {
                                    writer.WriteLine("{0} - {1} - CLOSED", airport.CurrentAirport.AirportId, airport.CurrentAirport.AirportName);
                                }
                                else if (airport.Renamed)
                                {
                                    writer.WriteLine("{0} - {1} - RENAMED {2}", airport.CurrentAirport.AirportId, airport.CurrentAirport.AirportName, airport.NewAirport.AirportName);
                                }
                            }
                        }
                        else
                        {
                            if (States.Contains(airport.CurrentAirport.StateName))
                            {
                                if (airport.Opened)
                                {
                                    writer.WriteLine("{0} - {1} - OPENED", airport.NewAirport.AirportId, airport.NewAirport.AirportName);
                                }
                                else if (airport.Closed)
                                {
                                    writer.WriteLine("{0} - {1} - CLOSED", airport.CurrentAirport.AirportId, airport.CurrentAirport.AirportName);
                                }
                                else if (airport.Renamed)
                                {
                                    writer.WriteLine("{0} - {1} - RENAMED {2}", airport.CurrentAirport.AirportId, airport.CurrentAirport.AirportName, airport.NewAirport.AirportName);
                                }
                            }
                        }
                    }
                }
            }

            Console.WriteLine("File outputted at {0}", path);

            Thread.Sleep(1000);
        }

        static List<TecRoute> GenerateTecRouteList(bool newData = false)
        {
            List<TecRoute> routes = new List<TecRoute>();
            string[] rawData, rawDataSplit;
            

            if (!newData)
            {
                rawData = File.ReadAllLines("data/PFR_BASE.csv");
            }
            else
            {
                rawData = File.ReadAllLines("data/PFR_BASE_NEW.csv");
            }

            for (int i = 1; i < rawData.Length; i++)
            {
                TecRoute route = new TecRoute();
                rawData[i] = rawData[i].Replace("\"", "");
                rawDataSplit = rawData[i].Split(',');

                route.OriginId.Add(rawDataSplit[1]);
                route.OriginCity.Add(rawDataSplit[2]);
                route.OriginStateCode = rawDataSplit[3];
                route.OriginCountryCode = rawDataSplit[4];
                route.DestinationId.Add(rawDataSplit[5]);
                route.DestinationCity.Add(rawDataSplit[6]);
                route.DestinationStateCode = rawDataSplit[7];
                route.DestinationCountryCode = rawDataSplit[8];
                route.PreferencialRouteTypeCode = rawDataSplit[9];
                route.RouteNumber = rawDataSplit[10];
                route.SpecialAreaDescription = rawDataSplit[11];
                route.AltitudeDescription = rawDataSplit[12];
                route.Aircraft = rawDataSplit[13];
                route.Hours = rawDataSplit[14];
                route.RouteDirectionDescription = rawDataSplit[15];
                route.NarType = rawDataSplit[16];
                route.Designator = rawDataSplit[17];
                route.InlandFacFix = rawDataSplit[18];
                route.CoastalFix = rawDataSplit[19];
                route.Destination = rawDataSplit[20];
                route.RouteString = rawDataSplit[21];

                if (route.PreferencialRouteTypeCode == "TEC" && route.OriginStateCode == "CA" && route.DestinationStateCode == "CA")
                {
                    if (!routes.Exists(x => x.RouteDirectionDescription == route.RouteDirectionDescription))
                    {
                        route.CalculateOriginDestination();
                        routes.Add(route);
                    }
                }
            }

            return routes;
        }

        static string[] ReadCifpData()
        {
            if (!File.Exists("data/FAACIFP18"))
            {
                Console.WriteLine("Please download the FAA CIFP Data and try again.");
                
                Thread.Sleep(2000);

                return null;
            }

            string[] data = File.ReadAllLines("data/FAACIFP18");
            
            Console.WriteLine("Data read in. {0} records imported.", data.Length.ToString());
            
            Thread.Sleep(2000);
            
            return data;
        }

        static void StartUp()
        {
            if (Globals.Cifp)
            {
                Globals.RawCifpData = ReadCifpData();
            } else
            {
                Globals.RawCifpData = null; // Todo: Read NASR Data
            }

            if ((File.Exists("data/FAACIFP18") && Globals.Cifp) || File.Exists("data/APT_BASE.csv"))
            {
                Globals.Airports = GenerateAirportList(Globals.RawCifpData);
            }

            if (!Globals.Cifp)
            {
                GenerateTecRouteList();
            }
        }
        
        static void MainMenu()
        {
            string[] menuItems = { "Reread data", "Export airports", "Create changes list", "Exit"};
            
            Menu menu = new Menu(menuItems, "Main Menu");

            switch (menu.displayMenu())
            {
                case 0:
                    StartUp();
                    break;
                
                case 1:
                    // Todo: Export airport list
                    break;
                
                case 2:
                    GenerateAiportChangesList();
                    WriteAirportChanges(Globals.ModifiedAirports);
                    break;
                case 3:
                    Environment.Exit(0);
                    break;
            }
        }
    }
}