use std::sync::Arc;

use pytools::marshal;

mod mapping;

pub fn map_opcode(obj: Arc<marshal::PyObject>) -> Result<Arc<marshal::PyObject>, std::io::Error> {
    let marshal::PyObject::Code {
        arg_count,
        pos_only_arg_count,
        kw_only_arg_count,
        stacksize,
        flags,
        code,
        consts,
        names,
        locals_plus_names,
        locals_plus_kinds,
        filename,
        name,
        qualname,
        firstlineno,
        linetable,
        exceptiontable,
    } = obj.as_ref()
    else {
        return Ok(obj);
    };

    let code = code.as_bytes().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid code object")
    })?;
    let mut mapped_code = vec![0u8; code.len()];
    mapped_code.copy_from_slice(code);

    let mut i = 0;
    while i < code.len() {
        let opcode = mapping::OPCODE_MAPPING[code[i] as usize];

        mapped_code[i] = opcode as u8;

        i += 2;
        i += opcode.cache_size();
    }

    // map children objects
    let marshal::PyObject::Tuple(consts) = consts.as_ref() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid type for consts",
        ));
    };

    let consts_mapped = consts
        .iter()
        .map(|obj| map_opcode(obj.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Arc::new(marshal::PyObject::Code {
        arg_count: *arg_count,
        pos_only_arg_count: *pos_only_arg_count,
        kw_only_arg_count: *kw_only_arg_count,
        stacksize: *stacksize,
        flags: *flags,
        code: Arc::new(marshal::PyObject::String(mapped_code.into_boxed_slice())),
        consts: Arc::new(marshal::PyObject::Tuple(consts_mapped.into_boxed_slice())),
        names: names.clone(),
        locals_plus_names: locals_plus_names.clone(),
        locals_plus_kinds: locals_plus_kinds.clone(),
        filename: filename.clone(),
        name: name.clone(),
        qualname: qualname.clone(),
        firstlineno: *firstlineno,
        linetable: linetable.clone(),
        exceptiontable: exceptiontable.clone(),
    }))
}
