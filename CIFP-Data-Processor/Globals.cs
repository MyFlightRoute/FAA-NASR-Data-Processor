using System.Collections.Generic;
using ShellProgressBar;

namespace CIFP_Data_Processor
{
    public static class Globals
    {
        public static string[] RawData;

        public static readonly ProgressBarOptions ProgressBarOptions = new ProgressBarOptions()
        {
            ProgressCharacter = '-',
            ProgressBarOnBottom = true
        };

        public static List<Airport> Airports = new List<Airport>();
    }
}