using System;

namespace FAA_Data_Processor
{
    public class Airport : IEquatable<Airport>
    {
        public string StateCode { get; set; }
        public string AirportId { get; set; }
        public string City { get; set; }
        public string CountryCode { get; set; }
        public string RegionCode { get; set; }
        public string AdoCode { get; set; }
        public string StateName { get; set; }
        public string CountyName { get; set; }
        public string CountyAssoiatedState { get; set; }
        public string AirportName { get; set; }
        public string OwnershipTypeCode { get; set; }
        public string FacilityUseCode { get; set; }
        public string LatitudeDegree { get; set; }
        public string LatitudeMinutes { get; set; }
        public string LatitudeSeconds { get; set; }
        public string LatitudeHemisphere { get; set; }
        public string LatitudeDecimal { get; set; }
        public string LongitudeDegree { get; set; }
        public string LongitudeMinutes { get; set; }
        public string LongitudeSeconds { get; set; }
        public string LongitudeHemisphere { get; set; }
        public string LongitudeDecimal { get; set; }
        public string SurveyMethodCode { get; set; }
        public string Elevation { get; set; }
        public string ElevationMethodCode { get; set; }
        public string MagneticVariation { get; set; }
        public string MagneticHemisphere { get; set; }
        public string MagneticVariationYear { get; set; }
        public string Tpa { get; set; }
        public string ChartName { get; set; }
        public string DistanceCityToAirport { get; set; }
        public string DirectionCode { get; set; }
        public string Acreage { get; set; }
        public string RespArtccId { get; set; }
        public string ComputerId { get; set; }
        public string ArtccName { get; set; }
        public string FssOnAirportFlag { get; set; }
        public string FssId { get; set; }
        public string FssName { get; set; }
        public string PhoneNumber { get; set; }
        public string TollFreeNumber { get; set; }
        public string AltFssId { get; set; }
        public string AltFssName { get; set; }
        public string AltTollFreeNumber { get; set; }
        public string NotamId { get; set; }
        public string NotamFlag { get; set; }
        public string ActivationDate { get; set; }
        public string AirportStatus { get; set; }
        public string Far139TypeCode { get; set; }
        public string Far139CarrierSerCode { get; set; }
        public string ArffCertTypeDate { get; set; }
        public string NaspCode { get; set; }
        public string AspAnalysisDtrmCode { get; set; }
        public string CustomFlag { get; set; }
        public string LandingRightsFlag { get; set; }
        public string JointUseFlag { get; set; }
        public string MilitaryLandingFlag { get; set; }
        public string InspectMethodCode { get; set; }
        public string InspectorCode { get; set; }
        public string LastInspection { get; set; }
        public string LastInformationResponce { get; set; }
        public string FuelTypes { get; set; }
        public string AirframeRepairServiceCode { get; set; }
        public string PowerplantRepairService { get; set; }
        public string BottledOxygenType { get; set; }
        public string BulkOxygenType { get; set; }
        public string LightingSchedule { get; set; }
        public string BeaconLightSchedule { get; set; }
        public string TowerTypeCode { get; set; }
        public string SegmentCircleMarkerFlag { get; set; }
        public string BeaconLensColor { get; set; }
        public string LandingFeeFlag { get; set; }
        public string MedicalUseFlag { get; set; }
        public string BasedSingleEngine { get; set; }
        public string BasedMultiEngine { get; set; }
        public string BasedJetEngine { get; set; }
        public string BasedHelicopter { get; set; }
        public string BasedGliders { get; set; }
        public string BasedMilitaryAircraft { get; set; }
        public string BasedUltralightAircaft { get; set; }
        public string CommercialOps { get; set; }
        public string CommuterOps { get; set; }
        public string AirTaxiOps { get; set; }
        public string LocalOps { get; set; }
        public string IntermittentOps { get; set; }
        public string MilitaryAircraftOps { get; set; }
        public string AnnualOpsDate { get; set; }
        public string AirportPositionSource { get; set; }
        public string PositionSourceDate { get; set; }
        public string AirportElevationSource { get; set; }
        public string ElevationSourceDate { get; set; }
        public string ContrFuelAvailable { get; set; }
        public string TransientStorageBuoyFlag { get; set; }
        public string TransientStorageHangarFlag { get; set; }
        public string TransientStorageTieFlag { get; set; }
        public string OtherServices { get; set; }
        public string WindIndicatorFlag { get; set; }
        public string IcaoId { get; set; }
        public string MinimumOperationalNetwork { get; set; }
        public string UserFeeFlag { get; set; }
        public string Cta { get; set; }
        public string Latitude { get; set; }
        public string Longitude { get; set; }
        public string RawString { get; }
        private char[] RawCifpCharArr { get; set; }
        
        public Airport(string rawString)
        {
            
        }

        public bool Equals(Airport otherAirport)
        {
            if (otherAirport == null)
            {
                return false;
            }

            if (this.AirportId == otherAirport.AirportId) 
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

            Airport TestAirport = obj as Airport;
            if (TestAirport == null)
                return false;
            else
                return Equals(TestAirport);
        }

        public override int GetHashCode()
        {
            return this.GetHashCode();
        }
    }
}