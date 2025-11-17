mod admin;
mod client;
mod matematika;
mod kendaraan;
mod member;

use std::fmt::format;
use std::result;

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


trait Hewan {
    fn info(&self);
}

trait Sound{
    fn say_hello(&self) {
        println!("Halo Dari rust");
    }
    fn suara(&self);
    
}

struct Anjing;
struct Kucing{
    nama: String,
}


impl Sound for Anjing{
    fn suara(&self) {
        println!("Guk guk");
    }
}

impl Sound for Kucing {
    fn say_hello(&self) {
        println!("Halo nama saya {}", self.nama);
    }
    fn suara(&self) {
        println!("Meong meng");
    }
}

#[test]
fn suara_hewan() {
    let anjing = Anjing;


    anjing.suara();


    let kucing = Kucing{
        nama: String::from("Nekonya"),
    };


    kucing.suara();
    kucing.say_hello();
}

trait HitungLuas{
    fn luas(&self) -> f64;
}

struct Persegi{
    sisi: f64,
}
struct Lingkaran{
    radius: f64,
}

impl HitungLuas for Persegi {
    fn luas(&self) -> f64 {
        self.sisi * self.sisi
    }
}
impl HitungLuas for Lingkaran {
    fn luas(&self) -> f64 {
        3.14 * self.radius
    }
}

#[test]
fn test_luas() {
    let result = Lingkaran{ radius: 16.0};
    println!("{}", result.luas());
    let result1 = Persegi{ sisi:18.0};
    result1.luas();
    println!("{}", result1.luas());
}

trait Transportasi {
    fn info(&self);
}

struct Mobil {
    merk: String,
}

struct Motor {
    merk: String,
}

impl Transportasi for Mobil {
    fn info(&self) {
        println!("Ini kendaraan roda 4");
    }
}

impl Transportasi for Motor {
   fn info(&self) {
        println!("Ini kendaraan roda 2");
    }
}

fn tampilkan_kendaraan(k: &dyn Transportasi) {
    k.info();
}

#[test]
fn test_kendaraan() {
    let mobil = Mobil{
        merk:String::from("Daihatsu"),
    };
    tampilkan_kendaraan(&mobil);
    let motor = Motor{
        merk:String::from("Yamaha"),
    };
    tampilkan_kendaraan(&motor);
}

trait Deskripsi {
    fn deskripsi(&self);
}

impl Deskripsi for i32 {
    fn deskripsi(&self) {
        println!("Ini angka primitive {}", self.abs());
    }
}

impl Deskripsi for bool {
    fn deskripsi(&self) {
        println!("Ini kondisi {}", self);
    }
}

impl Deskripsi for String {
    fn deskripsi(&self) {
        println!("Ini data yang disimpan di heap {}", self.to_uppercase());
    }
}

#[test]
fn test_deskripsi() {
    let no: i32 = 10;
    no.deskripsi();
    let konsidi: bool = true;
    konsidi.deskripsi();
    let strr = String::from("Kanjut");
    strr.deskripsi();
}

