use std::{cmp, io::Read};

#[derive(Debug)]
#[repr(packed)]
pub struct Header {
    pub magic: u32,
    pub entries: u32,
    pub unknown_1: u32,
    pub entry_encryption_mode: u32,
    pub hash_mode: u32,
    pub entry_offset: u32,
}

#[repr(packed)]
pub struct FileEntry {
    pub id: u32,
    pub offset: u32,
    pub packed_size: u32,
    pub raw_size: u32,
    pub checksum_packed: u32,
    pub checksum_raw: u32,
    pub flags: u32,
}

#[derive(Debug)]
pub enum CompressionMode {
    None,
    Zlib,
    Lz4,
}

#[derive(Debug)]
pub enum EncryptionMode {
    None,
    SimpleCrypt,
    Rc4,
    Aes,
    SimpleCryptEx,
}

impl std::fmt::Debug for FileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id;
        let offset = self.offset;
        let packed_size = self.packed_size;
        let raw_size = self.raw_size;
        let checksum_packed = self.checksum_packed;
        let checksum_raw = self.checksum_raw;
        write!(f, "FileEntry {{ id: {:08x}, offset: {:08x}, packed_size: {}, raw_size: {}, checksum_packed: {:08x}, checksum_raw: {:08x}, is_compressed: {:?}, is_encrypted: {:?} }}",
            id,
            offset,
            packed_size,
            raw_size,
            checksum_packed,
            checksum_raw,
            self.compression_mode(),
            self.encryption_mode(),
        )
    }
}

impl FileEntry {
    pub fn compression_mode(&self) -> CompressionMode {
        let mode = self.flags & 0xff;
        match mode {
            0 => CompressionMode::None,
            1 => CompressionMode::Zlib,
            2 => CompressionMode::Lz4,
            _ => panic!("Unsupported compression mode: {:02x}", mode),
        }
    }

    pub fn encryption_mode(&self) -> EncryptionMode {
        let mode = (self.flags >> 16) & 0xff;
        match mode {
            0 => EncryptionMode::None,
            1 => EncryptionMode::SimpleCrypt,
            2 => EncryptionMode::Rc4,
            3 => EncryptionMode::Aes,
            4 => EncryptionMode::SimpleCryptEx,
            _ => panic!("Unsupported encryption mode: {:02x}", mode),
        }
    }

    pub fn unpack_data(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if data.len() != self.packed_size as usize {
            return Err("Data length mismatch".into());
        }

        let data: Box<[u8]> = match self.encryption_mode() {
            EncryptionMode::None => Box::from(data),
            EncryptionMode::SimpleCryptEx => {
                let v3: u64 = self.raw_size.into();
                let v4: u64 = self.checksum_raw.into();
                let (offset, len): (u64, u64) = if self.packed_size < 0x81 {
                    (0, self.packed_size.into())
                } else {
                    (
                        (v3 >> 1) % (self.packed_size as u64 - 0x80),
                        (((v4 << 1) & 0xffffffff) % 0x60 + 0x20),
                    )
                };

                let mut key: u8 = ((v3 ^ v4) & 0xff).try_into().unwrap();
                let mut copy = vec![0; self.packed_size as usize];
                copy.copy_from_slice(data);

                for i in offset..cmp::min(offset + len, self.packed_size as u64) {
                    copy[i as usize] ^= key;
                    key = key.wrapping_add(1);
                }

                copy.into_boxed_slice()
            }
            _ => {
                return Err("Unsupported encryption mode".into());
            }
        };

        Ok(match self.compression_mode() {
            CompressionMode::None => data.to_vec(),
            CompressionMode::Zlib => {
                let mut decompressed = Vec::with_capacity(self.raw_size as usize);
                let mut decoder = flate2::read::ZlibDecoder::new(&*data);
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            CompressionMode::Lz4 => lz4_flex::decompress(&data, self.raw_size as usize)?,
        })
    }
}
pub struct NpkIterator<'a> {
    data: &'a [u8],
    header: &'a Header,
    entries: &'a [FileEntry],

    current: usize,
}

impl<'a> NpkIterator<'a> {
    pub fn new(data: &'a [u8]) -> Result<NpkIterator<'a>, &'static str> {
        if data.len() < std::mem::size_of::<Header>() {
            return Err("Data too short");
        }

        let header = unsafe { &*(data.as_ptr() as *const Header) };
        if header.magic != 0x4B50584E {
            return Err("Invalid magic");
        }

        if data.len()
            < header.entry_offset as usize
                + header.entries as usize * std::mem::size_of::<FileEntry>()
        {
            return Err("Data too short");
        }
        let entries = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr().offset(header.entry_offset as isize) as *const FileEntry,
                header.entries as usize,
            )
        };

        Ok(NpkIterator {
            data,
            header,
            entries,
            current: 0,
        })
    }
}

impl<'a> Iterator for NpkIterator<'a> {
    type Item = (&'a FileEntry, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.header.entries as usize {
            let entry = &self.entries[self.current];
            self.current += 1;
            Some((
                entry,
                &self.data[entry.offset as usize..(entry.offset + entry.packed_size) as usize],
            ))
        } else {
            None
        }
    }
}
