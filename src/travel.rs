#[derive(Debug, Clone)]
struct Passenger {
  name: String,
  age: u8,
  document: String
}

#[derive(Debug, Clone)]
struct Flight {
  flight_id: String,
  origin: String,
  destination: String,
  departure_date: String,
  departure_time: String
}

struct Agency {
  flights: Vec<Flight>,
  passengers: Vec<Passenger>
}

impl Agency {
  pub fn add_passenger(&mut self, passenger: Passenger) {
    self.passengers.push(passenger);
  }

  pub fn add_flight(&mut self, flight: Flight) {
    self.flights.push(flight);
  }

  pub fn get_passengers(&self) -> Vec<Passenger> {
    self.passengers.clone()
  }

  pub fn get_flights(&self) -> Vec<Flight> {
    self.flights.clone()
  }
}

pub fn demo() {
  let mut agency = Agency {
    flights: Vec::new(),
    passengers: Vec::new()
  };

  agency.add_flight(Flight {
    flight_id: String::from("1234"),
    origin: String::from("London"),
    destination: String::from("Paris"),
    departure_date: String::from("2021-01-01"),
    departure_time: String::from("12:00")
  });

  agency.add_flight(Flight {
    flight_id: String::from("5678"),
    origin: String::from("Paris"),
    destination: String::from("London"),
    departure_date: String::from("2021-01-01"),
    departure_time: String::from("14:00")
  });

  agency.add_passenger(Passenger {
    name: String::from("John"),
    age: 20,
    document: String::from("1234")
  });

  agency.add_passenger(Passenger {
    name: String::from("Ana"),
    age: 20,
    document: String::from("5678")
  });

  println!("{:?}", agency.get_flights());
  println!("{:?}", agency.get_passengers());
}