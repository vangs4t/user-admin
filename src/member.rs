use std::fmt::format;

use crate::kendaraan::sepeda;

pub enum Level{
    Reguler,
    Premium,
    Platinum,
}
const MAAX :i32 = 100;
pub trait Feature {
    fn rician(&self, name :Level);
    fn unlimited(&self);
    fn limit(&self, level: String) -> String;
}

// pengimplementasian match pada enum
impl Feature for Level {
    fn rician(&self, name :Level) {
        match name {
            Level::Reguler => println!("Kamu Reguler"),
            Level::Premium => println!("Kamu Premium"),
            Level::Platinum => println!("Kamu Platinum"),
        }
    }
    
    fn unlimited(&self) {
        println!("Kamu dapat segalanya");
    }
    
    fn limit(&self, level: String) -> String {
        if level.eq(&String::from("Reguler")) {
            String::from("Fiturnya sedikit")
        } else {
            String::from("Banyak Fiturnya")
        }
    }
}
