mod admin;
mod client;
mod matematika;
mod kendaraan;
mod member;
mod market;

use std::fmt::{Debug, Display, format};
use std::result;

use kendaraan as transportasi;

use crate::kendaraan::*;
use crate::kendaraan::Mesin;
use crate::member::{Feature, Level};
use crate::market::*;

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

fn min_value<T: PartialOrd>(x: T, y: T) -> T{
    if x > y {
        y
    } else {
        x
    }
}

fn compare_print<T: Display + PartialOrd>(a: T, b: T){
    if a > b {
        println!("A lebih besar dari B");
    } else if a < b{
        println!("B lebih besar dari A");
    } else {
        println!("Sama");
    }
}

#[test]
fn perbadingan() {
    let a = min_value(12, 6);
    let b = min_value(12.2, 6.2);
    let c = Wrapper{
        value: "Kanjut"
    };
    let d = Response::Succes(12);
    println!("{}, {}",a,b);
}

struct Wrapper<T>{
    value: T
}

impl<T: Debug> Wrapper<T> {
    // fn show(&self){
    //     println!("{}", self.value);
    // }
    fn print_debug(&self){
        println!("{:?}", self.value);
    }
}

enum Response<T> {
    Succes(T),
    Error(String),
}

impl<T> Response<T> {
    fn process(x: bool) -> Response<i32>{
        match x {
            x => Response::Succes(12),
            x => Response::Error(String::from("Gagal")),
        } 
    }
}

fn max_value<T: PartialOrd>(x: T, y: T) -> T{
    if x > y {
        x
    } else {
        y
    }
}

#[test]
fn latihan_2() {
    let pertama = max_value(20, 13);
    println!("{}", pertama);
    let arr = Wrapper{
        value: [1,2,3,4,5],
    };
    arr.print_debug();
    describe(String::from("Kanjut?"));
    
}   

fn describe<T>(value: T)
where
    T: Debug + Display,
{
    println!("{:?}", value);
    println!("{}", value.to_string());
}

trait ToNumber {
    fn to_number(&self) -> i32;
}
impl ToNumber for bool {
    fn to_number(&self) -> i32 {
        if *self {1} else {0}
    }
}
impl ToNumber for u32 {
    fn to_number(&self) -> i32 {
        *self as i32
    }
}

fn sum_two<T: ToNumber>(a: T, b: T) -> i32 {
    a.to_number() + b.to_number()
}
#[test]
fn latihan_4() {
    let result = sum_two(13, 14);
    let hasil = sum_two(true, false);

    println!("{},{}",result, hasil);
}

fn panjang(s: &String) -> usize{
    s.len()
}

struct Akun{
    username: String,
    level: u32,
}

impl Akun {
    fn info(&self){
        println!("Username: {}", self.username);
        println!("Level: {}", self.level);
    }
    fn level_up(&mut self){
        self.level +=1;
    }
}

enum StatusLogin{
    Succes(String),
    Error(String),
}

fn proses_login(username: &str, password: &str) -> StatusLogin{
        if password.eq("admin") {
            StatusLogin::Succes(String::from("Selamat datang admin"))
        } else {
            StatusLogin::Error(String::from("Akun tidak tersedia"))
        }
    }


trait Hitung{
    fn hitung(&self) -> i32;
}

impl Hitung for i32 {
    fn hitung(&self) -> i32 {
        *self
    }
}
impl Hitung for bool {
    fn hitung(&self) -> i32 {
        if *self{
            1
        } else {
            0
        }
    }
}

trait Hewans{
    fn suara(&self);
    
}

struct Husky;
struct Persian;


impl Hewans for Husky{
    fn suara(&self) {
        println!("Guk guk");
    }
}

impl Hewans for Persian {
    fn suara(&self) {
        println!("Meong meng");
    }
}

fn mins_value<T: PartialOrd>(x: T, y: T) -> T{
    if x < y {
        x
    } else {
        y
    }
}

struct Boxx<T> {
    item: T,
}

impl<T: Display> Boxx<T> {
    fn show(&self){
        println!("the value: {}", self.item);
    }
}

enum ResultKu<T> {
    Ok(T),
    Err(String),
}

impl<T> ResultKu<T> {
    fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(msg) => panic!("{}", msg)
        }
    }
}

fn describes<T>(value: T) where T: Debug + Display + PartialOrd{
    // let mut result = String::from("Kanjut?");
    println!("{:?}", value);
    println!("{}", value);
    if value.to_string() > "".to_string() {
        println!("Ini sukses")
    }
}


#[test]
fn references() {
    let s = String::from("Hisana");
    let mut player = Akun{ username: String::from("Ciyss"), level: 3 };
    let anjing = Husky;
    let kucing = Persian;
    let nomor = mins_value(12, 14);
    let floating = mins_value(12.4, 14.4);
    let strr = mins_value("x", "yy");
    let result = proses_login("jhon", "admin");
    let number = 10;
    let kondisi = true;
    let generik = Boxx{ item: "Kanjut?"};
    let result:ResultKu<i32> = ResultKu::Ok(100);

    result.unwrap();
    describe("Kanjut terbang");

    generik.show();
    println!("{},{},{}",nomor,floating,strr);

    kucing.suara();
    anjing.suara();
    println!("{}", kondisi.hitung());

    println!("level: {}", player.level);

    player.level_up();

    println!("level: {}", player.level);

    panjang(&s); // parameter bisa dipanggil dengan references
    panjang(&s.clone()); // meskipun pake clone tetep harus pake references karena settingan parameternya references
}

use crate::market::{Barang, Keranjang};
use crate::market::Harga;

#[test]
fn point_of_sales() {
    let barang = Barang::Buku { judul: String::from("Hijau bumi"), harga: 12000 };
    let barang1 = Barang::Buku { judul: String::from("Hitam bumi"), harga: 13000 };
    let barang2 = Barang::Pensil { jenis: String::from("Hitam bumi"), harga: 13000 };


    println!("buku ini seharga: {}", barang.harga());
    
    // Keranjang hanya bisa berisi tipe yang implement Harga (seperti Barang)
    let mut troli: Keranjang<Barang> = Keranjang {
        item: vec![]
    };
    
    // Sekarang troli bisa akses method tambah dan total_harga
    troli.tambah(barang);
    troli.tambah(barang1);
    troli.tambah(barang2);
    println!("total harga: {}", troli.total_harga());
}

// use std::fmt::Debug;
// formatter
struct Weapon{
    name: String,
    damage: u32,
}

impl Debug for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Weapons")
        .field("Nama: ", &self.name)
        .field("Damage: ", &self.damage)
        .finish()
    }   
}
/*Kebanyakan format debug digunakan untuk tipe data 
yang cukup kompleks seperti array, struct dll
sedangkan format Display untuk tipe data sederhana atau primitive
seperti number, string, dll */
#[test]
fn useable() {
    let weapon = Weapon{
        name: String::from("Destroyer"),
        damage: 100
    };

    println!("{:?}", weapon);
    let result = |value: i32| -> i32{
        value * 2
    };

    let hasil = result(6);
    println!("{hasil}");

}

// Closure atau anonymous function

#[test]
fn test_anonymous_function() {
    let nama = "Naufal";
    let umur = 20;

    println!("Nama: {}", nama);
    println!("Umur: {:02}", umur);
    println!("Biner umur: {:b}", umur);
    println!("Lebar 10 rata kanan: {:>10}", nama);
    println!("Lebar 10 rata kiri: {:<10}", nama);
    let result = info_produk(800, "Gadged", 12_000_000);
    println!("{}",result);
}

fn info_produk(id: u32, nama: &str, harga: u32) -> String {
    format!("[ID: {id}] Produk: {nama} - Harga: Rp. {harga}"
)
}




fn apply_twice<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(1) + x
}

#[test]
fn mainini() {
    let hasil = apply_twice(3, |n| n + 1); 
    println!("{}", hasil);
    // harus menghasilkan 5
}

#[test]
fn vektors() {
    let num = vec![1,2,3,4,5,6];
    let mut new_num: Vec<i32> = vec![];
    for number in num {
        if number % 2 == 0{
            new_num.push(number * 10);
        }
    }
    println!("{:?}", new_num);
}

#[test]
fn folds() {
    let angka = vec![3,5,7];
    let hasil = angka.iter().fold(0, |acc, x| acc + x);
    println!("{}", hasil);
}

#[test]
fn enumerates() {
    let data = vec!["apel", "jeruk", "mangga"];
    let result = data.iter().enumerate();

    for datas in result {
        println!("{:?}", datas);
    }
}

#[test]
fn zipper() {
    let a = vec!["Nama", "Usia", "Kota"];
    let b = vec!["Naufal", "20", "Bandung"];

    let mut c = a.into_iter().zip(b);

    for zip in c {
        println!("{:?}", zip);
    }
}

#[test]
fn merger() {
    let a = vec![1,2,3];
    let b = vec![4,5,6];

    let mut c = a.into_iter().chain(b);
    
    println!("{:?}", c);
}