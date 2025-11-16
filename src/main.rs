mod admin;
mod client;
mod matematika;
mod kendaraan;
mod member;

use kendaraan as transportasi;

use crate::kendaraan::*;
use crate::kendaraan::Mesin;
use crate::member::{Feature, Level};

/*untuk penggunaan kode di file yang berbeda kita harus memasukan 
modul nya terlebih dahulu ke file main atau file inti */
fn main() {
    let penjumlahan = matematika::tambah(5, 2);
    let pengurangan = matematika::kurang(5, 2);
    let perkalian = matematika::kali(5, 2);
    let pembagian = matematika::bagi(5, 2);
    let modulus = matematika::modulus(5, 2);

    println!("5 + 2 = {}",penjumlahan);
    println!("5 - 2 = {}",pengurangan);
    println!("5 x 2 = {}",perkalian);
    println!("5 : 2 = {}",pembagian);
    println!("5 % 2 = {}",modulus);
}
#[test]
fn teknologi() {
    transportasi::mobil::info();
    transportasi::motor::info();
    transportasi::sepeda::info();
}

type Meter = f64;
type Detik = f64;
type KmPerJam = f64;


fn kecepatan(jarak: Meter, waktu: Detik) -> Meter {
    jarak / waktu
}

fn speed(jarak : Meter, waktu: Detik) -> KmPerJam{
    jarak / waktu * 3.6
}

#[test]
fn e() {
    let v = kecepatan(100.0, 9.58);
    let g = speed(100.0, 9.58);
    println!("Kecepatan rata-rata: {:.2} m/s", v);
    println!("Kecepatan rata-rata: {:.2} k/h", g);
}

#[test]
fn permesinan() {
    let mesin = Mesin{
        karbulator: String::from("Daytona"),
        piston: String::from("Brt"),
        kopling: String::from("Yamanah"),

    };
    
    mesin.pengapian(true);
    motor::info();
}
/*Jadi disini aku mengimplementasikan enum dengan trait ya */
#[test]
fn feature() {
    let member = Level::Premium;
    member.rician(Level::Premium);
    member.limit(String::from("Premium"));
    member.unlimited();

}
trait Sop {
    fn salam(&self);
    fn penawaran(&self, varian: String);
}

struct Hisana{
    nama_gerai: String,
    lokasi: String,
}

impl Sop for Hisana {
    fn salam(&self) {
        println!("Selamat datang hisana selalu hangat");
    }

    fn penawaran(&self, varian: String) {
        println!("Barangkali mau nambah varian {}", varian);
    }
}

#[test]
fn pelayanan() {
    let kasir = Hisana{
        nama_gerai: String::from("Ciparay"),
        lokasi: String::from("Bandung selatan")
    };

    kasir.salam();
    kasir.penawaran(String::from("Kentang"));
}

/*Penggunaan trait pada parameter */
fn test_parameter(kacab: impl Sop) {
    kacab.salam();
    kacab.penawaran(String::from("Jamur enoki"));
}

#[test]
fn futur() {
    test_parameter(Hisana {nama_gerai: String::from("Ciperna"), lokasi: String::from("Kuningan")});
}
