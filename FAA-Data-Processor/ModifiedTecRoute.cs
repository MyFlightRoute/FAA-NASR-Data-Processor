using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace FAA_Data_Processor
{
    public class ModifiedTecRoute
    {
        public TecRoute CurrentRoute { get; set; }
        public TecRoute NewRoute { get; set; }
        public bool IsChanged { get; set; }
        public bool CreatedRoute { get; set; }
        public bool RemovedRoute { get; set; }
        public bool DepartureAirportChanged { get; set; }
        public bool ArrivalAirportChanged { get; set; }
        public bool AltitudeChanged { get; set; }
        public bool RouteChanged { get; set; }

        public void RunChanges()
        {
            IsChanged = (CurrentRoute == NewRoute);
            AltitudeChanged = (CurrentRoute.AltitudeDescription == NewRoute.AltitudeDescription);
            RouteChanged = (CurrentRoute.RouteString == NewRoute.RouteString);

            foreach(var originAirport in this.CurrentRoute.OriginId)
            {
                DepartureAirportChanged = !(NewRoute.OriginId.Contains(originAirport));

                if (DepartureAirportChanged)
                {
                    IsChanged = true;
                    break;
                }
            }

            foreach (var destinationAirport in CurrentRoute.DestinationId)
            {
                ArrivalAirportChanged = !(NewRoute.DestinationId.Contains(destinationAirport));

                if (ArrivalAirportChanged) 
                {
                    IsChanged = true;
                    break;
                }
            }
        }
    }
}
