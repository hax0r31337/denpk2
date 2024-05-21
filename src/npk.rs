use std::io::Read;

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
}

#[derive(Debug)]
pub enum EncryptionMode {
    None,
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
        match self.flags & 0xff {
            0 => CompressionMode::None,
            1 => CompressionMode::Zlib,
            _ => panic!("Unsupported compression mode"),
        }
    }

    pub fn encryption_mode(&self) -> EncryptionMode {
        match (self.flags >> 16) & 0xff {
            0 => EncryptionMode::None,
            _ => panic!("Unsupported encryption mode"),
        }
    }

    pub fn unpack_data(&self, data: &[u8]) -> Vec<u8> {
        match self.compression_mode() {
            CompressionMode::None => data.to_vec(),
            CompressionMode::Zlib => {
                let mut decompressed = Vec::new();
                let mut decoder = flate2::read::ZlibDecoder::new(data);
                decoder.read_to_end(&mut decompressed).unwrap();
                decompressed
            }
        }
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
