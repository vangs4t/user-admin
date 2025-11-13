mod admin;
mod client;
mod matematika;
mod kendaraan;

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
    mobil::info();
    motor::info();
    sepeda::info();
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
