use crate::airport::Airport;

pub struct ModifiedAirport {
    pub(crate) current_airport: Option<Airport>,
    pub(crate) new_airport: Option<Airport>,
    pub(crate) is_modified: Option<bool>,
    pub(crate) renamed: Option<bool>,
    pub(crate) closed: Option<bool>,
    pub(crate) opened: Option<bool>,
}

impl ModifiedAirport {
    // Below function is no longer required due to revised processing of finding airport changes in main.rs.
    // Leaving commented for future reference, in case this processing method becomes necessary.
    /* pub fn run_changes(&mut self) {
        self.is_modified.replace(!(self.current_airport.as_mut().unwrap() != self.new_airport.as_mut().unwrap()));

        self.renamed.replace(!(self.current_airport.as_mut().unwrap().airport_name == self.new_airport.as_mut().unwrap().airport_name));

        self.closed.replace(self.new_airport.is_none());
        self.opened.replace(self.current_airport.is_none());
    } */
}