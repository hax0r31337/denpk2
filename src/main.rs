use std::env;

// use decompyle_rs::{disassemble, marshal};
mod mmap;
mod npk;
mod nxs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = mmap::new_path(&env::args().nth(1).ok_or("No NXPK file path given")?)?;
    let extracted_path = std::path::Path::new("./extracted");

    let npk = npk::NpkIterator::new(data)?;

    for (entry, data) in npk {
        println!("{:?}", entry);

        let id = entry.id;
        let output = extracted_path.join(format!("{:08x}", id));

        let unpacked = entry.unpack_data(data)?;
        std::fs::write(&output, &unpacked)?;

        // let pyc = nxs::unpack(&unpacked)?;

        // let py_object = marshal::PyObject::read_root(&pyc)?;

        // match py_object.as_ref() {
        //     marshal::PyObject::Code {
        //         filename,
        //         arg_count,
        //         pos_only_arg_count,
        //         kw_only_arg_count,
        //         stacksize,
        //         flags,
        //         code,
        //         consts,
        //         names,
        //         locals_plus_names,
        //         locals_plus_kinds,
        //         name,
        //         qualname,
        //         firstlineno,
        //         linetable,
        //         exceptiontable,
        //     } => {
        //         println!("{}", filename.as_str().unwrap());
        //     }
        //     _ => {}
        // }

        // std::fs::write(&output, &pyc)?;
    }

    Ok(())
}
