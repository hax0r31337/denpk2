use std::{env, io::Write};

use pytools::marshal;
mod mmap;
mod npk;
mod nxs;
mod opcode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = mmap::new_path(&env::args().nth(1).ok_or("No NXPK file path given")?)?;
    let extracted_path = std::path::Path::new("./extracted");

    let npk = npk::NpkIterator::new(data)?;

    for (entry, data) in npk {
        println!("{:?}", entry);

        let id = entry.id;

        let unpacked = entry.unpack_data(data)?;
        let (code, header) = if &unpacked[0..8] == nxs::NXS_MAGIC {
            (nxs::unpack(&unpacked)?, marshal::PYC_HEADER)
        } else if unpacked[0..4] == marshal::PYC_HEADER[0..4] {
            (unpacked[16..].to_vec(), unpacked[0..16].try_into()?)
        } else {
            let output = extracted_path.join(format!("blobs/{:08x}", id));
            std::fs::create_dir_all(output.parent().ok_or("No parent")?)?;

            std::fs::write(&output, &unpacked)?;
            continue;
        };

        let py_object = marshal::PyObject::read_root(&code)?;
        let py_object = opcode::map_opcode(py_object)?;

        let marshal::PyObject::Code { filename, .. } = py_object.as_ref() else {
            return Err("Unexpected object type".into());
        };

        // replace windows backslashes with forward slashes
        let filename = filename.as_str().map(|s| s.replace("\\", "/"));
        let output = extracted_path.join(format!("{}.pyc", filename.ok_or("No filename")?));

        std::fs::create_dir_all(output.parent().ok_or("No parent")?)?;

        let file = std::fs::File::create(&output)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write_all(header)?;
        py_object.write_root(&mut writer, false)?;
    }

    Ok(())
}
