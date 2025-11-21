pub trait Harga {
    fn harga(&self) -> u32;
}

pub enum Barang {

    Buku{judul:String, harga:u32},

    Pensil{jenis:String, harga:u32},

}

pub struct Keranjang<T>{
    pub item: Vec<T>,
}


impl Harga for Barang {
    fn harga(&self) -> u32 {
        match self {
            Self::Buku { judul, harga } => *harga,
            Self::Pensil { jenis, harga } => *harga
        }
    }
}

impl<T: Harga> Keranjang<T> {
    pub fn tambah(&mut self, item: T) {
        // simpan item, bukan harga; Keranjang menyimpan koleksi `T`
        self.item.push(item);
    }

    pub fn total_harga(&self) -> u32 {
        // jumlahkan harga dari setiap item menggunakan trait `Harga`
        self.item.iter().map(|i| i.harga()).sum()
    }
}
