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
        
        public Airport(string rawCifpString, string icaoCode = "")
        {
            RawCifpString = rawCifpString;
            RawCifpCharArr = RawCifpString.ToCharArray();
            
            if (icaoCode != "")
            {
                IcaoCode = icaoCode;
            }
            else
            {
                generateIcaoCode();
            }
        }
        
        /*
         * Function for generating the ICAO code from the RAW CIFP String
         * Currently, this is handled in Program.cs when the object is created
         * Todo: Evaluate moving ICAO code generation into the Object.
         */
        private string generateIcaoCode()
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
            
            return icaoCode;
        }
    }
}