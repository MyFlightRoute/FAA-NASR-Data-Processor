using System.Collections.Generic;
using ShellProgressBar;

namespace FAA_Data_Processor
{
    public static class Globals
    {
        public static string[] RawCifpData;

        public static readonly ProgressBarOptions ProgressBarOptions = new ProgressBarOptions()
        {
            ProgressCharacter = '-',
            ProgressBarOnBottom = true
        };

        public static List<Airport> Airports = new List<Airport>();
        public static List<ModifiedAirport> ModifiedAirports = new List<ModifiedAirport>();

        public static List<TecRoute> TecRoutes = new List<TecRoute>();
    }
}