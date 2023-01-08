namespace CIFP_Data_Processor
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
            GetIcaoCode();
        }
        
         // Function for generating the ICAO code from the RAW CIFP String
         private void GetIcaoCode()
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