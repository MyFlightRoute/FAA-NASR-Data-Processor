using System;
using System.Collections.Generic;
using ConsoleMenu;
using System.IO;
using System.Threading;
using ShellProgressBar;

namespace CIFP_Data_Processor
{
    class Program
    {
        static void Main(string[] args)
        {
            if (!Directory.Exists("data"))
            {
                Directory.CreateDirectory("data");
            }

            while (true)
            {
                Console.Clear();
                MainMenu();
            }
        }

        static List<Airport> GenerateAirportList(String[] rawData)
        {
            List<string> airportHoldingData = new List<string>();
            List<Airport> airports = new List<Airport>();
            
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

            Console.WriteLine("Raw airport data read in successfully. {0} records added.", airportHoldingData.Count.ToString());
            
            Thread.Sleep(1000);

            using (ProgressBar progressBar = new ProgressBar(airportHoldingData.Count, "Processing airport data",
                       Globals.ProgressBarOptions))
            {
                for (int i = 0; i < airportHoldingData.Count; i++)
                {
                    Airport newAirport;
                    char[] dataCurrentLineChar = airportHoldingData[i].ToCharArray();
                    bool duplicateAirport = false;
                    string airportName, faaCode, icaoCode, latitude, longitude, lastIcaoCode;

                    icaoCode = generateIcaoCode(dataCurrentLineChar);

                    if (i > 0)
                    {
                        char[] dataLastLineChar = airportHoldingData[i - 1].ToCharArray();
                        lastIcaoCode = generateIcaoCode(dataLastLineChar);

                        duplicateAirport = (icaoCode == lastIcaoCode);
                    }

                    newAirport = new Airport(airportHoldingData[i], icaoCode);

                    if (!duplicateAirport)
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

        // Todo: Evaluate moving ICAO code generation into the Object.
        private static string generateIcaoCode(char[] rawData)
        {
            string icaoCode;
            
            if (rawData[9] == ' ')
            {
                // Three letter code
                icaoCode = rawData[6].ToString() + rawData[7].ToString() + rawData[8].ToString();
                // Console.WriteLine(icaoCode);
            }
            else if (rawData[10] == 'K')
            {
                // Four letter code
                icaoCode = rawData[6].ToString() + rawData[7].ToString() + rawData[8].ToString() + rawData[9].ToString();
                // Console.WriteLine(icaoCode);
            }
            else
            {
                icaoCode = null;
            }

            return icaoCode;
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
        
        static void MainMenu()
        {
            string[] menuItems = { "Read data", "Export airports", "Exit"};
            
            Menu menu = new Menu(menuItems, "Main Menu");

            switch (menu.displayMenu())
            {
                case 0:
                    Globals.RawData = ReadCifpData();
                    break;
                
                case 1:
                    Globals.Airports = GenerateAirportList(Globals.RawData);
                    break;
                
                case 2:
                    Environment.Exit(0);
                    break;
            }
        }
    }
}