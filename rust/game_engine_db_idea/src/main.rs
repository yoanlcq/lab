use std::collections::HashMap;
use std::mem;

pub type ID = u32;

// Objectif: Créer une struct custom
// struct Team { name: String, num: i32 }

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Db {
    names: HashMap<ID, ID>,
    sizes: HashMap<ID, usize>,
    strings: HashMap<ID, String>,
    fields: HashMap<ID, Vec<ID>>,
    field_type: HashMap<ID, ID>,
}

fn main() {
    let mut db = Db::default();

    let typeid_i32 = 0x132;
    let sid_i32_name = 0x453;
    db.sizes.insert(typeid_i32, mem::size_of::<i32>());
    db.names.insert(typeid_i32, sid_i32_name);
    db.strings.insert(sid_i32_name, "i32".to_owned());

    let typeid_string = 0x131;
    let sid_string_name = 0x452;
    db.sizes.insert(typeid_string, mem::size_of::<ID>());
    db.names.insert(typeid_string, sid_string_name);
    db.strings.insert(sid_string_name, "String".to_owned());

    let typeid_team = 0x489;
    let sid_team_name = 0x488;
    let fieldid_team_name = 0x512;
    let fieldid_team_num = 0x513;
    let sid_name_name = 0x541;
    let sid_num_name = 0x542;

    db.names.insert(typeid_team, sid_team_name);
    db.strings.insert(sid_team_name, "Team".to_owned());
    db.fields.insert(typeid_team, vec![fieldid_team_name, fieldid_team_num]);
    db.field_type.insert(fieldid_team_name, typeid_string);
    db.field_type.insert(fieldid_team_num, typeid_i32);
    db.names.insert(fieldid_team_name, sid_name_name);
    db.names.insert(fieldid_team_num, sid_num_name);
    db.strings.insert(sid_name_name, "name".to_owned());
    db.strings.insert(sid_num_name, "num".to_owned());
    {
        let size = db.fields[&typeid_team].iter().map(|fid| db.sizes[&db.field_type[fid]]).sum();
        db.sizes.insert(typeid_team, size);
    }

    db.print_type(typeid_i32);
    db.print_type(typeid_string);
    db.print_type(typeid_team);
}

impl Db {
    fn print_type(&self, typeid: ID) {
        let name = &self.strings[&self.names[&typeid]];
        let size = self.sizes[&typeid];
        if let Some(ref fields) = self.fields.get(&typeid) {
            println!("struct {} {{", name);
            for fieldid in fields.iter() {
                let name = &self.strings[&self.names[fieldid]];
                let typename = &self.strings[&self.names[&self.field_type[fieldid]]];
                println!("    {}: {},", name, typename);
            }
            println!("}} (size: {})", size);
        } else {
            println!("type {} (size: {})", name, size);
        }
    }
}
