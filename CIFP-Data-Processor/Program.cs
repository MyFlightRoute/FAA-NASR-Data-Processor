using System;
using System.Collections.Generic;
using ConsoleMenu;
using System.IO;
using System.Linq;
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

            StartUp();
            
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
                    Airport newAirport = new Airport(airportHoldingData[i]);
                    bool duplicateAirport = false;
                    
                    if (i > 0)
                    {
                        duplicateAirport = (newAirport.IcaoCode == airports.Last().IcaoCode);
                    }
                    
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
            Globals.RawData = ReadCifpData();

            Globals.Airports = GenerateAirportList(Globals.RawData);
        }
        
        static void MainMenu()
        {
            string[] menuItems = { "Reread data", "Export airports", "Exit"};
            
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
                    Environment.Exit(0);
                    break;
            }
        }
    }
}