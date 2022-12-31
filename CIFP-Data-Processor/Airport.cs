using System;

namespace CIFP_Data_Processor
{
    public class Airport
    {
        public String Name { get; set; }
        public String Identifier { get; set; }
        public String Latitude { get; set; }
        public String Longitude { get; set; }
        
        public Airport(string name, string identifier, string longitude, string latitude)
        {
            this.Name = name;
            this.Identifier = identifier;
            this.Latitude = latitude;
            this.Longitude = longitude;
        }
    }
}