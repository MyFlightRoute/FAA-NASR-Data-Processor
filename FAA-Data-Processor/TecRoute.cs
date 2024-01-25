using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace FAA_Data_Processor
{
    public class TecRoute : IEquatable<TecRoute>
    {
        public List<string> OriginId { get; set; }
        public List<string> OriginCity { get; set; }
        public string OriginStateCode { get; set; }
        public string OriginCountryCode { get; set; }
        public List<string> DestinationId { get; set; }
        public List<string> DestinationCity { get; set; }
        public string DestinationStateCode { get; set; }
        public string DestinationCountryCode { get; set; }
        public string PreferencialRouteTypeCode { get; set; }
        public string RouteNumber { get; set; }
        public string SpecialAreaDescription { get; set; }
        public string AltitudeDescription { get; set; }
        public string Aircraft { get; set; }
        public string Hours { get; set; }
        public string RouteDirectionDescription { get; set; }
        public string NarType { get; set; }
        public string Designator { get; set; }
        public string InlandFacFix { get; set; }
        public string CoastalFix { get; set; }
        public string Destination { get; set; }
        public string RouteString { get; set; }
        public List<string> Notes { get; set; }

        public TecRoute()
        {
            OriginId = new List<string>();
            OriginCity = new List<string>();
            DestinationId = new List<string>();
            DestinationCity = new List<string>();
            Notes = new List<string>();
        }

        public void CalculateOriginDestination()
        {
            string[] splitDescriptionMain = SpecialAreaDescription.Split(" TO ");
            this.OriginId.Clear();
            this.DestinationId.Clear();

            string departureAerodromesRaw = splitDescriptionMain[0];
            string[] splitDepartureAerodromes = departureAerodromesRaw.Split(" ");

            string arrivalAerodromesRaw = splitDescriptionMain[1];
            string[] splitArrivalAerodromes = arrivalAerodromesRaw.Split(" ");

            foreach (var aerodrome in splitDepartureAerodromes)
            {
                if (aerodrome != "(LAXE)")
                {
                    this.OriginId.Add(aerodrome);
                }
                else
                {
                    Notes.Add(aerodrome);
                }
            }

            foreach (var aerodrome in splitArrivalAerodromes)
            {
                if (aerodrome != "(LAXE)")
                {
                    this.DestinationId.Add(aerodrome);
                }
                else
                {
                    Notes.Add(aerodrome);
                }
            }
        }

        public bool Equals(TecRoute otherTecRoute)
        {
            if (otherTecRoute == null)
            {
                return false;
            }

            if (this.RouteDirectionDescription == otherTecRoute.RouteDirectionDescription)
            {
                return true;
            }
            else
            {
                return false;
            }
        }

        public override bool Equals(Object obj)
        {
            if (obj == null)
                return false;

            TecRoute TestTecRoute = obj as TecRoute;
            if (TestTecRoute == null)
                return false;
            else
                return Equals(TestTecRoute);
        }

        public override int GetHashCode()
        {
            return this.GetHashCode();
        }

    }
}
