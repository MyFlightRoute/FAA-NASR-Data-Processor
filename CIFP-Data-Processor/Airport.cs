namespace CIFP_Data_Processor
{
    public class Airport
    {
        public string Name { get; set; }
        public string IcaoCode { get; set; }
        public string FaaCode { get; set; }
        public string Latitude { get; set; }
        public string Longitude { get; set; }
        public string RawCifpString { get; set; }
        private char[] RawCifpCharArr { get; set; }
        
        public Airport(string rawCifpString)
        {
            RawCifpString = rawCifpString;
            RawCifpCharArr = RawCifpString.ToCharArray();
            GenerateIcaoCode();
        }
        
         // Function for generating the ICAO code from the RAW CIFP String
         private void GenerateIcaoCode()
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

            IcaoCode = icaoCode;
        }
    }
}