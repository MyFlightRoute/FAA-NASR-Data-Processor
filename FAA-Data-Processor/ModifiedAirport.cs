using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace FAA_Data_Processor
{
    internal class ModifiedAirport
    {
        public Airport CurrentAirport { get; set; }
        public Airport NewAirport { get; set; }
        public bool IsModified { get; set; }
        public bool Renamed { get; set; }
        public bool Closed { get; set; }
        public bool Opened { get; set; }

        public ModifiedAirport()
        {
            
        }

        public void RunChanges()
        {
            IsModified = !(CurrentAirport == NewAirport);

            Renamed = !(CurrentAirport.AirportName == NewAirport.AirportName);

            Closed = (NewAirport == null);
            Opened = (CurrentAirport == null);
        }
    }
}
