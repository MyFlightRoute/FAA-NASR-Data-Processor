using System.Collections.Generic;
using ShellProgressBar;

namespace FAA_Data_Processor
{
    public static class Globals
    {
        public static string[] RawCifpData;
        public static bool Cifp = false; // Set this to true if using CIFP data rather than NASR data

        public static readonly ProgressBarOptions ProgressBarOptions = new ProgressBarOptions()
        {
            ProgressCharacter = '-',
            ProgressBarOnBottom = true
        };

        public static List<Airport> Airports = new List<Airport>();
        public static List<ModifiedAirport> ModifiedAirports = new List<ModifiedAirport>();
    }
}