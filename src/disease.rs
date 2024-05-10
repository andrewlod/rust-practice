use core::fmt;
use std::fmt::Debug;

struct Disease {
  label: String,
  symptoms: Vec<String>,
  cause: String,
  treatment: String
}

impl Disease {
  pub fn new(label: &str, symptoms: Vec<String>, cause: &str, treatment: &str) -> Disease {
    Disease {
      label: String::from(label),
      symptoms,
      cause: String::from(cause),
      treatment: String::from(treatment)
    }
  }

  pub fn get_label(&self) -> &str {
    self.label.as_str()
  }

  pub fn get_symptoms(&self) -> Vec<String> {
    self.symptoms.clone()
  }

  pub fn get_cause(&self) -> &str {
    self.cause.as_str()
  }

  pub fn get_treatment(&self) -> &str {
    self.treatment.as_str()
  }
}

impl Debug for Disease {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "Disease( label: {}, symptoms: {:?}, cause: {}, treatment: {} )", &self.label, &self.symptoms, &self.cause, &self.treatment)
  }
}

pub fn demo() {
  let symptoms = vec![String::from("Headache"), String::from("Fever")];
  let disease = Disease::new("Malaria", symptoms, "Dengue", "Antibiotics");

  println!("{:?}", &disease);
}