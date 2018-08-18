
pub mod db {
    use std::ptr;
    use std::ops::Range;

    // Private macro
    macro_rules! uuid {
        ($i:expr) => { $i as u128 + (1 << 120) };
    }

    static NAME_OF: [&'static str; NB_PRIMITIVE_TYPES as _] = [
        "u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128", "f32", "f64"
    ];
    static SIZE_OF: [usize; NB_PRIMITIVE_TYPES as _] = [
        1, 1, 2, 2, 4, 4, 8, 8, 16, 16, 4, 4
    ];
    static UUID_OF: [u128; NB_PRIMITIVE_TYPES as _] = [
        uuid!(U8), uuid!(I8), uuid!(U16), uuid!(I16), uuid!(U32), uuid!(I32), uuid!(U64), uuid!(I64), uuid!(U128), uuid!(I128), uuid!(F32), uuid!(F64)
    ];

    pub fn is_primitive(t: u32) -> bool {
        t < NB_PRIMITIVE_TYPES as _
    }
    pub fn name_of_primitive(t: u32) -> &'static str {
        assert!(is_primitive(t));
        NAME_OF[t as usize] 
    }
    pub fn size_of_primitive(t: u32) -> usize {
        assert!(is_primitive(t));
        SIZE_OF[t as usize]
    }
    pub fn uuid_of_primitive(t: u32) -> u128 {
        assert!(is_primitive(t));
        UUID_OF[t as usize]
    }
    pub fn instantiate_primitive(t: u32, mem: &mut [u8], init: &[&str]) {
        if init.is_empty() {
            default_instantiate_primitive(t, mem);
        } else {
            primitive_from_str(t, mem, init[0]).unwrap();
        }
    }
    pub fn default_instantiate_primitive(t: u32, mem: &mut [u8]) {
        assert!(mem.len() == size_of_primitive(t));
        unsafe {
            ptr::write_bytes(mem.as_mut_ptr(), 0, mem.len())
        }
    }
    pub fn primitive_to_string(t: u32, mem: &[u8]) -> String {
        assert!(mem.len() == size_of_primitive(t));
        unsafe {
            match t {
                self::U8   => format!("{}", { *(mem.as_ptr() as *const u8  ) }),
                self::I8   => format!("{}", { *(mem.as_ptr() as *const i8  ) }),
                self::U16  => format!("{}", { *(mem.as_ptr() as *const u16 ) }),
                self::I16  => format!("{}", { *(mem.as_ptr() as *const i16 ) }),
                self::U32  => format!("{}", { *(mem.as_ptr() as *const u32 ) }),
                self::I32  => format!("{}", { *(mem.as_ptr() as *const i32 ) }),
                self::U64  => format!("{}", { *(mem.as_ptr() as *const u64 ) }),
                self::I64  => format!("{}", { *(mem.as_ptr() as *const i64 ) }),
                self::U128 => format!("{}", { *(mem.as_ptr() as *const u128) }),
                self::I128 => format!("{}", { *(mem.as_ptr() as *const i128) }),
                self::F32  => format!("{}", { *(mem.as_ptr() as *const f32 ) }),
                self::F64  => format!("{}", { *(mem.as_ptr() as *const f64 ) }),
                _ => unreachable!{},
            }
        }
    }
    pub fn primitive_from_str(t: u32, mem: &mut [u8], s: &str) -> Result<(), String> {
        assert!(mem.len() == size_of_primitive(t));
        unsafe {
            match t {
                self::U8   => *(mem.as_mut_ptr() as *mut u8  ) = s.parse().map_err(|e| format!("{}", e))?,
                self::I8   => *(mem.as_mut_ptr() as *mut i8  ) = s.parse().map_err(|e| format!("{}", e))?,
                self::U16  => *(mem.as_mut_ptr() as *mut u16 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::I16  => *(mem.as_mut_ptr() as *mut i16 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::U32  => *(mem.as_mut_ptr() as *mut u32 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::I32  => *(mem.as_mut_ptr() as *mut i32 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::U64  => *(mem.as_mut_ptr() as *mut u64 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::I64  => *(mem.as_mut_ptr() as *mut i64 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::U128 => *(mem.as_mut_ptr() as *mut u128) = s.parse().map_err(|e| format!("{}", e))?,
                self::I128 => *(mem.as_mut_ptr() as *mut i128) = s.parse().map_err(|e| format!("{}", e))?,
                self::F32  => *(mem.as_mut_ptr() as *mut f32 ) = s.parse().map_err(|e| format!("{}", e))?,
                self::F64  => *(mem.as_mut_ptr() as *mut f64 ) = s.parse().map_err(|e| format!("{}", e))?,
                _ => unreachable!{},
            };
            Ok(())
        }
    }


    // Primitive types
    pub const U8  : u32 =  0;
    pub const I8  : u32 =  1;
    pub const U16 : u32 =  2;
    pub const I16 : u32 =  3;
    pub const U32 : u32 =  4;
    pub const I32 : u32 =  5;
    pub const U64 : u32 =  6;
    pub const I64 : u32 =  7;
    pub const U128: u32 =  8;
    pub const I128: u32 =  9;
    pub const F32 : u32 = 10;
    pub const F64 : u32 = 11;
    pub const NB_PRIMITIVE_TYPES: u32 = F64 + 1;
    pub const ALL_PRIMITIVE_TYPES: Range<u32> = U8 .. F64;

    // Semantics applied on top of primitive types
    pub const BOOL: u32 = 32;
    pub const CHAR: u32 = 33;
    pub const UUID: u32 = 34;

    // Sized composites
    pub const ARRAY : u32 = 64;
    pub const STRUCT: u32 = 65;
    pub const UNION : u32 = 66;
    pub const SUM   : u32 = 67;
    pub const ENUM  : u32 = 68;

    // Unsized composites
    pub const VEC   : u32 = 69;

    // Meta
    pub const PRIMITIVE_TYPE: u32 = 96;
    pub const PRIMITIVE_TYPE_UUID: u128 = uuid!(PRIMITIVE_TYPE);

    pub const HIGHEST_RESERVED_ID_EXCLUSIVE: u32 = 256;
}

use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::fs::File;

#[derive(Default)]
pub struct DB {
    highest_id: u32,
    // Use BTreeMap for sorted export
    pub uuid: BTreeMap<u32, u128>,
    pub uuid_reverse: BTreeMap<u128, u32>,
    pub name: HashMap<u32, String>,
    pub type_: HashMap<u32, u32>,
    pub struct_: HashMap<u32, u32>,
    pub offset: HashMap<u32, usize>,
    pub size: HashMap<u32, usize>,
}

pub mod datamap;
use datamap::{DenseDataMap, DataMapKey};

// Goals: 
// - Represent mapping between Entity and "chunk of data which size is uniform and know only at run-time"
// - Allow retrieval of "data chunk" by entity;
// - Allow fast traversal of all chunks in one go;
pub struct Arena {
    map: DenseDataMap,
    index: HashMap<u128, DataMapKey>,
}

pub struct WorldDB {
    // For each type, contains all instances of that type, keyed by entity.
    pub arena: HashMap<u128, Arena>,
}

fn main() {
    let mut db = DB::new();

    // Init with primitive types
    db.uuid.insert(db::PRIMITIVE_TYPE, db::PRIMITIVE_TYPE_UUID);
    db.uuid_reverse.insert(db::PRIMITIVE_TYPE_UUID, db::PRIMITIVE_TYPE);
    db.name.insert(db::PRIMITIVE_TYPE, "PrimitiveType".to_owned());
    for i in db::ALL_PRIMITIVE_TYPES {
        db.type_.insert(i, db::PRIMITIVE_TYPE);
        db.name.insert(i, db::name_of_primitive(i).to_owned());
        db.size.insert(i, db::size_of_primitive(i));
        db.uuid.insert(i, db::uuid_of_primitive(i));
        db.uuid_reverse.insert(db::uuid_of_primitive(i), i);
    }

    // Now, create a Vec3<f32> struct
    let id_vec3f   = db.id_from_uuid(0x20000000000000000000000000000001);
    let id_vec3f_x = db.id_from_uuid(0x20000000000000000000000000000002);
    let id_vec3f_y = db.id_from_uuid(0x20000000000000000000000000000003);
    let id_vec3f_z = db.id_from_uuid(0x20000000000000000000000000000004);
    db.name.insert(id_vec3f, "Vec3f".to_owned());
    db.name.insert(id_vec3f_x, "x".to_owned());
    db.name.insert(id_vec3f_y, "y".to_owned());
    db.name.insert(id_vec3f_z, "z".to_owned());
    db.type_.insert(id_vec3f_x, db::F32);
    db.type_.insert(id_vec3f_y, db::F32);
    db.type_.insert(id_vec3f_z, db::F32);
    db.struct_.insert(id_vec3f_x, id_vec3f);
    db.struct_.insert(id_vec3f_y, id_vec3f);
    db.struct_.insert(id_vec3f_z, id_vec3f);
    db.offset.insert(id_vec3f_x, 0);
    db.offset.insert(id_vec3f_y, 4);
    db.offset.insert(id_vec3f_z, 8);
    {
        let size = db.struct_size(id_vec3f);
        db.size.insert(id_vec3f, size);
    }

    db.print_struct(id_vec3f);

    let mut map = DenseDataMap::new(db.size[&id_vec3f]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["22", "53.57"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["22", "53.57"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["22", "53.57"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["42", "13.56"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["42", "13.56"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["42", "13.56"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["22", "53.57"]);
    db.instantiate(id_vec3f, map.insert_uninitialized().1, &["22", "53.57"]);
    for (k, v) in map.iter() {
        println!("Key: {:?}", k);
        db.print_struct_instance(id_vec3f, v);
    }

    db.write_struct_rs(File::create("gen.rs").unwrap(), id_vec3f);

    db.export(File::create("db.ini").unwrap());
}

impl DB {
    pub fn export<W: Write>(&self, mut w: W) {
        for (uuid, id) in &self.uuid_reverse {
            writeln!(w, "[{:#x}]", uuid);
            if let Some(name) = self.name.get(id) {
                writeln!(w, "name = \"{}\"", name);
            }
            if let Some(struct_) = self.struct_.get(id) {
                writeln!(w, "# {}", self.name[struct_]);
                writeln!(w, "struct = {:#x}", self.uuid[struct_]);
            }
            if let Some(ty) = self.type_.get(id) {
                writeln!(w, "# {}", self.name[ty]);
                writeln!(w, "type = {:#x}", self.uuid[ty]);
            }
            if let Some(offset) = self.offset.get(id) {
                writeln!(w, "offset = {}", offset);
            }
            if let Some(size) = self.size.get(id) {
                writeln!(w, "size = {}", size);
            }
            writeln!(w);
        }
    }
    pub fn new() -> Self {
        Self {
            highest_id: db::HIGHEST_RESERVED_ID_EXCLUSIVE,
            .. Default::default()
        }
    }
    fn gen_id(&mut self) -> u32 {
        self.highest_id = self.highest_id.wrapping_add(1);
        self.highest_id
    }
    fn add_new_uuid(&mut self, uuid: u128) -> u32 {
        let id = self.gen_id();
        self.uuid.insert(id, uuid);
        self.uuid_reverse.insert(uuid, id);
        id
    }
    pub fn id_from_uuid(&mut self, uuid: u128) -> u32 {
        self.uuid_reverse.get(&uuid).map(|x| *x).unwrap_or_else(|| self.add_new_uuid(uuid))
    }
    pub fn instantiate(&self, t: u32, mem: &mut [u8], init: &[&str]) {
        let mut i = 0;
        if db::is_primitive(t) {
            db::instantiate_primitive(t, mem, init);
        } else {
            for (mi, m) in self.struct_fields(t).into_iter().enumerate() {
                let mt = self.type_[&m];
                let j = i + self.size[&mt];
                self.instantiate(mt, &mut mem[i..j], if mi < init.len() { &init[mi..mi+1] } else { &[] } );
                i = j;
            }
        }
    }
    pub fn struct_fields(&self, s: u32) -> Vec<u32> {
        let mut fields: Vec<_> = self.struct_.iter().filter(|(_, &v)| v == s).map(|(k, _)| *k).collect();
        fields.sort_by(|a, b| self.offset[a].cmp(&self.offset[b]));
        fields
    }
    pub fn struct_size(&self, s: u32) -> usize {
        self.struct_fields(s).iter().map(|m| self.size[&self.type_[&m]]).sum()
    }
    pub fn write_struct_rs<W: Write>(&self, mut w: W, s: u32) {
        writeln!(w, "#[repr(C)]");
        writeln!(w, "#[derive(Debug, Default, Copy, Clone, PartialEq)]");
        writeln!(w, "pub struct {} {{", self.name[&s]);
        for m in self.struct_fields(s) {
            writeln!(w, "    pub {}: {},", self.name[&m], self.name[&self.type_[&m]]);
        }
        writeln!(w, "}}");

        writeln!(w);
        writeln!(w, "pub const UUID: u128 = {:#x};", self.uuid[&s]);
    }

    pub fn print_struct(&self, s: u32) {
        println!("struct {} {{", self.name[&s]);
        for m in self.struct_fields(s) {
            println!("    {}: {},", self.name[&m], self.name[&self.type_[&m]]);
        }
        println!("}}");
    }
    pub fn print_struct_instance(&self, s: u32, mem: &[u8]) {
        println!("{} {{", self.name[&s]);
        for m in self.struct_fields(s) {
            let mt = self.type_[&m];
            let i = self.offset[&m];
            let j = i + self.size[&mt];
            println!("    {}: {},", self.name[&m], db::primitive_to_string(mt, &mem[i..j]));
        }
        println!("}}");
    }
}
