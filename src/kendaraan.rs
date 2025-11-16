pub mod mobil {
        pub fn info() {
            println!("Ini adalah mobil listrik 🚗⚡");
        }
}
pub mod motor {
        pub fn info() {
            println!("Ini adalah motor bensin");
        }
}
pub mod sepeda {
        pub fn info() {
            println!("Ini adalah kendaraan tenaga manusia");
        }
}

pub struct Mesin{
    pub karbulator: String,
    pub piston: String,
    pub kopling: String,
}
/*Untuk membuat trait terlihat secara publik caranya sama seperti 
struct dll
kalo trait nya udah publik methodnya juga otomatis public 
tanpa perlu menambahkan pub lagi */
pub trait Fungsi{
    fn pengapian(&self, busi: bool);
}

impl Fungsi for Mesin {
    fn pengapian(&self, busi: bool){
        if busi {
            println!("Kendaraaan nyala dengan piston {}", self.piston);
        } else {
            println!("Kendaraan mati")
        }
    }
}