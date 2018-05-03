use std::collections::HashMap;
use std::mem;
use std::ptr;

pub type RawID = u32;

macro_rules! specialized_id {
    ($ID:ident) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $ID { raw: RawID, }

        impl $ID {
            pub fn from_raw(raw: RawID) -> Self { Self { raw } }
        }
    };
}

specialized_id!(StringID);
specialized_id!(BlobID);
specialized_id!(TypeID);
specialized_id!(FieldID);

impl TypeID {
    pub const I32: Self = Self { raw: 0x132 };
    pub const STRING: Self = Self { raw: 0x133 };
}

// Objectif: Créer une struct custom
// struct Team { name: String, num: i32 }

#[derive(Debug, Default)]
pub struct Db {
    // Type-related information
    size_of: HashMap<TypeID, usize>,
    fields: HashMap<TypeID, Vec<FieldID>>,
    type_name: HashMap<TypeID, StringID>,
    type_default_value: HashMap<TypeID, Box<[u8]>>,
    field_name: HashMap<FieldID, StringID>,
    field_type: HashMap<FieldID, TypeID>,
    field_default_value: HashMap<FieldID, Box<[u8]>>,

    // Per-type pools
    strings: HashMap<StringID, String>,
    blobs: HashMap<BlobID, Vec<u8>>,
    objects: HashMap<TypeID, Vec<u8>>,
}

fn own_bytes<T>(x: T) -> Box<[u8]> {
    Box::from(unsafe { ::std::slice::from_raw_parts(&x as *const _ as *const u8, mem::size_of::<T>())})
}

fn main() {
    let mut db = Db::default();

    let typeid_i32 = TypeID::I32;
    let sid_i32_name = StringID::from_raw(0x453);
    db.size_of.insert(typeid_i32, mem::size_of::<i32>());
    db.type_name.insert(typeid_i32, sid_i32_name);
    db.type_default_value.insert(typeid_i32, own_bytes(0i32));
    db.strings.insert(sid_i32_name, "i32".to_owned());

    let typeid_string = TypeID::STRING;
    let sid_string_name = StringID::from_raw(0x452);
    let sid_empty = StringID::from_raw(0x422);
    db.size_of.insert(typeid_string, mem::size_of::<StringID>());
    db.type_name.insert(typeid_string, sid_string_name);
    db.type_default_value.insert(typeid_string, own_bytes(sid_empty));
    db.strings.insert(sid_string_name, "String".to_owned());
    db.strings.insert(sid_empty, "".to_owned());

    let typeid_team = TypeID::from_raw(0x489);
    let sid_team_name = StringID::from_raw(0x488);
    let fieldid_team_name = FieldID::from_raw(0x512);
    let fieldid_team_num = FieldID::from_raw(0x513);
    let sid_name_name = StringID::from_raw(0x541);
    let sid_num_name = StringID::from_raw(0x542);
    let sid_untitled = StringID::from_raw(0x548);

    db.type_name.insert(typeid_team, sid_team_name);
    db.strings.insert(sid_team_name, "Team".to_owned());
    db.fields.insert(typeid_team, vec![fieldid_team_name, fieldid_team_num]);
    db.field_type.insert(fieldid_team_name, typeid_string);
    db.field_type.insert(fieldid_team_num, typeid_i32);
    db.field_name.insert(fieldid_team_name, sid_name_name);
    db.field_name.insert(fieldid_team_num, sid_num_name);
    db.field_default_value.insert(fieldid_team_name, own_bytes(sid_untitled));
    db.field_default_value.insert(fieldid_team_num, own_bytes(42));
    db.strings.insert(sid_untitled, "<untitled>".to_owned());
    db.strings.insert(sid_name_name, "name".to_owned());
    db.strings.insert(sid_num_name, "num".to_owned());
    db.recompute_size_of_type(typeid_team);

    db.print_type(typeid_i32);
    db.print_type(typeid_string);
    db.print_type(typeid_team);

    db.create_object_array(typeid_team, 5);
    db.print_object_array(typeid_team);
}

impl Db {
    pub fn print_object_array(&self, tid: TypeID) {
        let ob_size = self.size_of[&tid];
        let array = &self.objects[&tid];
        assert_eq!(array.len() % ob_size, 0);
        let count = array.len() / ob_size;
        let mut offset = 0;
        for i in 0..count {
            println!("Object {} {{", i);
            for fid in self.fields[&tid].iter() {
                let field_type_id = self.field_type[fid];
                let field_name = &self.strings[&self.field_name[fid]];
                let field_size = self.size_of[&field_type_id];
                match field_type_id {
                    TypeID::I32 => {
                        let val: i32 = unsafe {
                            ptr::read(array[offset..].as_ptr() as *const i32)
                        };
                        println!("    {}: {},", field_name, val);
                    },
                    TypeID::STRING => {
                        let sid: StringID = unsafe {
                            ptr::read(array[offset..].as_ptr() as *const StringID)
                        };
                        println!("    {}: {},", field_name, self.strings[&sid]);
                    },
                    _ => unimplemented!{},
                };
                offset += field_size;
            }
            println!("}}");
        }
    }
    pub fn create_object_array(&mut self, tid: TypeID, count: usize) {
        let cap = count * self.size_of[&tid];
        let mut array = Vec::<u8>::with_capacity(cap);
        unsafe { array.set_len(cap) }

        let mut offset = 0;
        for _ in 0..count {
            for fid in self.fields[&tid].iter() {
                let size = self.size_of[&self.field_type[fid]];
                let default = &self.field_default_value[fid];
                assert_eq!(default.as_ref().len(), size);
                unsafe {
                    ptr::copy_nonoverlapping(default.as_ref().as_ptr(), &mut array[offset], size);
                }
                offset += size;
            }
        }

        self.objects.insert(tid, array);
    }
    pub fn recompute_size_of_type(&mut self, tid: TypeID) {
        let size = self.fields[&tid].iter().map(|fid| self.size_of[&self.field_type[fid]]).sum();
        self.size_of.insert(tid, size);
    }
    pub fn print_type(&self, typeid: TypeID) {
        let name = &self.strings[&self.type_name[&typeid]];
        let size = self.size_of[&typeid];
        if let Some(ref fields) = self.fields.get(&typeid) {
            println!("struct {} {{", name);
            for fieldid in fields.iter() {
                let name = &self.strings[&self.field_name[&fieldid]];
                let typename = &self.strings[&self.type_name[&self.field_type[fieldid]]];
                println!("    {}: {},", name, typename);
            }
            println!("}} (size: {})", size);
        } else {
            println!("type {} (size: {})", name, size);
        }
    }
}
