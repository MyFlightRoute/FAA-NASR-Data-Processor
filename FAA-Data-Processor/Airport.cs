namespace FAA_Data_Processor
{
    public class Airport
    {
        public string Name { get; private set; }
        public string IcaoCode { get; private set; }
        public string FaaCode { get; private set; }
        public string Latitude { get; private set; }
        public string Longitude { get; private set; }
        public string RawCifpString { get; }
        private char[] RawCifpCharArr { get; set; }
        
        public Airport(string rawCifpString)
        {
            RawCifpString = rawCifpString;
            RawCifpCharArr = RawCifpString.ToCharArray();
            IcaoCode = SetIcaoCode();
            FaaCode = SetFaaCode();
            Name = SetAirportName();
            SetCoordinates();
        }

        private void SetCoordinates()
        {
            string latitude = RawCifpCharArr[32].ToString() + RawCifpCharArr[33].ToString() +
                              RawCifpCharArr[34].ToString() + RawCifpCharArr[35].ToString() +
                              RawCifpCharArr[36].ToString() + RawCifpCharArr[37].ToString() +
                              RawCifpCharArr[38].ToString() + RawCifpCharArr[49].ToString() +
                              RawCifpCharArr[40].ToString();
            
            string longitude = RawCifpCharArr[41].ToString() + RawCifpCharArr[42].ToString() +
                              RawCifpCharArr[43].ToString() + RawCifpCharArr[44].ToString() +
                              RawCifpCharArr[45].ToString() + RawCifpCharArr[46].ToString() +
                              RawCifpCharArr[47].ToString() + RawCifpCharArr[48].ToString() +
                              RawCifpCharArr[49].ToString() + RawCifpCharArr[50].ToString();
            
            Latitude = latitude;
            Longitude = longitude;
        }
        
         // Function for generating the ICAO code from the RAW CIFP String
         private string SetIcaoCode()
        {
            string icaoCode;
            
            if (RawCifpCharArr[9] == ' ')
            {
                // Three letter code
                icaoCode = RawCifpCharArr[6].ToString() + RawCifpCharArr[7].ToString() + RawCifpCharArr[8].ToString();
                // Console.WriteLine(icaoCode);
            }
            else if (RawCifpCharArr[10] == 'K')
            {
                // Four letter code
                icaoCode = RawCifpCharArr[6].ToString() + RawCifpCharArr[7].ToString() + RawCifpCharArr[8].ToString() + RawCifpCharArr[9].ToString();
                // Console.WriteLine(icaoCode);
            }
            else
            {
                icaoCode = null;
            }

            return icaoCode;
        }

        private string SetFaaCode()
        {
            string faaCode;

            faaCode = RawCifpCharArr[13].ToString() + RawCifpCharArr[14].ToString() + RawCifpCharArr[15].ToString();

            if (faaCode.Trim() == "")
            {
                faaCode = IcaoCode;
            }

            return faaCode;
        }

        private string SetAirportName()
        {
            string airportName = "";
            char[] airportNameChars = RawCifpCharArr[93..121];
            
            foreach (char c in airportNameChars) 
            {
                airportName += c.ToString();
            }

            airportName = airportName.Trim();

            return airportName;
        }
    }
}