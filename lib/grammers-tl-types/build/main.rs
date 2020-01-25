mod enums;
mod grouper;
mod loader;
mod rustifier;
mod structs;

use grammers_tl_parser::Category;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

fn main() -> std::io::Result<()> {
    // TODO maybe a config to determine which files to generate?
    let api = loader::load_tl("tl/api.tl")?;
    let _mtproto = loader::load_tl("tl/mtproto.tl")?; // TODO use

    let mut file = BufWriter::new(File::create("src/generated.rs")?);

    // TODO if a parameter's type is raw (e.g. `vector<Foo>` or `foo`,
    // starting lowercase) then use the type, not the enum, because the
    // constructor code should not be serialized and it cannot be any
    // other type.
    structs::write_category_mod(&mut file, Category::Types, &api)?;
    structs::write_category_mod(&mut file, Category::Functions, &api)?;
    enums::write_enums_mod(&mut file, &api)?;

    file.flush()?;

    Ok(())
}
