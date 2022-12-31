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