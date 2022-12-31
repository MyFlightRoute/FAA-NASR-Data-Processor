using System;
using ConsoleMenu;

namespace CIFP_Data_Processor
{
    class Program
    {
        static void Main(string[] args)
        {
            mainMenu();
        }

        static void mainMenu()
        {
            string[] menuItems = new[] { "Read data", "Export airports", "Exit"};
            
            Menu menu = new Menu(menuItems, "Main Menu");

            switch (menu.displayMenu())
            {
                case 1:
                    // Todo: Develop reading data in
                
                case 2:
                    // Todo: Develop exporting airport
                
                case 3:
                    Environment.Exit(0);
                    break;
                
                default:
                    break;
            }
        }
    }
}