using System;
using ConsoleMenu;
using System.IO;
using System.Threading;

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
                mainMenu();
            }
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
            
            Console.WriteLine("Data read in.");
            
            Thread.Sleep(2000);
            
            return data;
        }
        
        static void mainMenu()
        {
            string[] menuItems = { "Read data", "Export airports", "Exit"};
            
            Menu menu = new Menu(menuItems, "Main Menu");

            switch (menu.displayMenu())
            {
                case 0:
                    Globals.RawData = ReadCifpData();
                    break;
                
                case 1:
                    // Todo: Develop exporting airport
                    break;
                
                case 2:
                    Environment.Exit(0);
                    break;
                
                default:
                    break;
            }
        }
    }
}