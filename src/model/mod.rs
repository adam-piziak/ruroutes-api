#[derive(Serialize, Debug)]
pub struct RoutePrediction {
    pub tag: String,
    pub name: String,
    pub stops: Vec<String>, 
    pub schedule: Vec<RouteSchedule>,
    pub campuses: Vec<String>
}

#[derive(Serialize, Debug)]
pub struct RouteSchedule {
    pub stop_title: String, // Name, not tag
    pub stop_tag: String,
    pub campus: String,
    pub times: Vec<u64>
}

#[derive(Serialize, Debug)]
pub struct RouteStatus {
    pub tag: String,
    pub name: String,
    pub campuses: Vec<String>,
    pub active: bool
}

#[derive(Serialize, Deserialize)]
pub struct Stop {
    pub tag: String,
    pub name: String,
    pub campus: String,
    pub schedule: Vec<StopSchedule>,
    pub location: Location
}

#[derive(Serialize, Deserialize)]
pub struct StopSchedule {
    pub name: String, 
    pub tag: String,
    pub times: Vec<u64>,
    pub campuses: Vec<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Location(pub f64, pub f64);

impl RouteSchedule {
    pub fn new(stop_title: String, stop_tag: String, campus: String) -> RouteSchedule {
        RouteSchedule {
            stop_title,
            stop_tag,
            times: Vec::new(),
            campus
        }
    }
}

impl RoutePrediction {
    pub fn new(name: String, tag: String, campuses: Vec<String>) -> RoutePrediction {
        RoutePrediction {
            tag,
            name,
            stops: Vec::new(),
            schedule: Vec::new(),
            campuses
        }
    }
}

impl Stop {
    pub fn new(name: String, tag: String, location: Location) -> Stop {
        Stop {
            name,
            tag,
            campus: String::from("other"),
            schedule: Vec::new(),
            location,
        }
    }
}

impl StopSchedule {
    pub fn new(name: String, tag: String, campuses: Vec<String>) -> StopSchedule {
        StopSchedule {
            name,
            tag,
            times: Vec::new(),
            campuses
        }
    }
}
impl Clone for RouteSchedule {
    fn clone(&self) -> Self {
        RouteSchedule {stop_tag: self.stop_tag.clone(), stop_title: self.stop_title.clone(), times: self.times.clone(), campus: self.campus.clone()}
    }
}

impl Clone for RoutePrediction {
    fn clone(&self) -> Self {
        RoutePrediction {tag: self.tag.clone(), name: self.name.clone(), stops: self.stops.clone(), schedule: self.schedule.clone(), campuses: self.campuses.clone()}
    }
}

impl Clone for StopSchedule {
    fn clone(&self) -> Self {
        StopSchedule { name: self.name.clone(), tag: self.tag.clone(), times: self.times.clone(), campuses: self.campuses.clone() }
    }
}

impl Clone for Location {
    fn clone(&self) -> Self {
        Location(self.0.clone(), self.1.clone())
    }
}
