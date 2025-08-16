use core::cell::{RefCell, SyncUnsafeCell};
use std::collections::{HashMap, HashSet};
use core::hash::Hash;
use alloc::rc::Rc;
use std::sync::Mutex;

use rayon::prelude::*;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EID(String);

#[derive(Default)]
struct Entity {
    componentdef_names: HashSet<&'static str>,
}

#[derive(Default)]
pub struct Entities {
    map: HashMap<EID, Entity>,
}

type EIDClosure = Box<dyn FnMut(EID)>;

#[derive(Default)]
pub struct Positions {
    map: HashMap<EID, SyncUnsafeCell<f32>>,
    pre_remove: HashMap<EID, Vec<EIDClosure>>,
}

impl Positions {
    pub fn insert_ifn(&mut self, eid: EID, pos: f32, entities: &mut Entities) {
        _ = entities.map.entry(eid.clone()).or_default().componentdef_names.insert("Positions");
        _ = self.map.entry(eid).or_insert_with(|| SyncUnsafeCell::new(pos));
    }
    pub fn remove(&mut self, eid: &EID) -> Option<f32> {
        if let Some(pre_remove) = self.pre_remove.remove(eid) {
            for mut f in pre_remove {
                f(eid.clone());
            }
        }
        self.map.remove(eid).map(SyncUnsafeCell::into_inner)
    }
    pub fn get_mut(&mut self, eid: &EID) -> Option<&mut f32> {
        self.map.get_mut(eid).map(SyncUnsafeCell::get_mut)
    }
}

#[derive(Debug, Default)]
pub struct Velocities {
    map: Rc<RefCell<HashMap<EID, f32>>>,
}

impl Velocities {
    pub fn insert(
        &mut self,
        eid: EID,
        vel: f32,
        positions: &mut Positions,
        entities: &mut Entities,
    ) {
        positions.insert_ifn(eid.clone(), 0., entities);
        let map = Rc::clone(&self.map);
        positions.pre_remove.entry(eid.clone()).or_default().push(Box::new(move |eid| { _ = map.borrow_mut().remove(&eid);}));
        _ = entities.map.entry(eid.clone()).or_default().componentdef_names.insert("Velocities");
        _ = self.map.borrow_mut().insert(eid, vel);
    }

    #[expect(clippy::missing_panics_doc, reason = "This cannot panic because having a velocity implies having a position")]
    pub fn update_positions(
        &mut self,
        positions: &mut Positions,
        dt: f32,
        entities: &mut Entities,
    ) {
        let mut pending_adds = vec![];
        let mut pending_removals = vec![];
        self.map.borrow().iter().for_each(|(eid, velocity)| {
            #[expect(clippy::expect_used, reason = "See the explanation")]
            let position = positions.get_mut(eid).expect("Having a velocity implies having a position, see Velocities::insert()");
            *position += *velocity * dt;

            if *position > 5. && *position < 10. {
                pending_adds.push(
                    |positions: &mut Positions, velocities: &mut Self, entities: &mut Entities| {
                        velocities.insert(EID::default(), 1., positions, entities);
                    },
                );
            }
            if *position > 20. {
                let eid = eid.clone();
                pending_removals.push(move |positions: &mut Positions| {
                    _ = positions.remove(&eid);
                });
            }
        });
        for command in pending_adds {
            command(positions, self, entities);
        }
        for command in pending_removals {
            command(positions);
        }
    }
    #[expect(clippy::missing_panics_doc, clippy::unwrap_used, reason = "This cannot panic because the mutexes cannot be poisoned")]
    pub fn update_positions_par(
        &mut self,
        positions: &mut Positions,
        dt: f32,
        entities: &mut Entities,
    ) {
        let pending_adds = Mutex::new(vec![]);
        let pending_removals = Mutex::new(vec![]);
        {
            let positions = &positions.map;
            self.map.borrow().par_iter().for_each(|(eid, velocity)| {
                // SAFETY: positions is &mut in this function, so nobody can resize it + each iteration has a unique EID therefore there is no aliasing of mutable refs
                #[expect(unsafe_code, reason = "This is a showcase, not production code")]
                let position = unsafe { &mut *positions.get(eid).unwrap().get() };
                *position += *velocity * dt;

                if *position > 5. && *position < 10. {
                    pending_adds.lock().unwrap().push(
                        |positions: &mut Positions, velocities: &mut Self, entities: &mut Entities| {
                            velocities.insert(EID::default(), 1., positions, entities);
                        },
                    );
                }
                if *position > 20. {
                    let eid = eid.clone();
                    pending_removals.lock().unwrap().push(move |positions: &mut Positions| {
                        _ = positions.remove(&eid);
                    });
                }
            });
        };
        for command in pending_adds.into_inner().unwrap() {
            command(positions, self, entities);
        }
        for command in pending_removals.into_inner().unwrap() {
            command(positions);
        }
    }
}

pub trait ComponentDef {
    fn remove(&mut self, eid: &EID);
}

impl ComponentDef for Positions {
    fn remove(&mut self, eid: &EID) {
        _ = Self::remove(self, eid);
    }
}

impl ComponentDef for Velocities {
    fn remove(&mut self, eid: &EID) {
        _ = self.map.borrow_mut().remove(eid);
    }
}

#[derive(Default)]
pub struct Cx {
    entities: Entities,
    positions: Positions,
    velocities: Velocities,
}

impl Cx {
    pub fn remove_entity(&mut self, eid: &EID) {
        if let Some(entity) = self.entities.map.remove(eid) {
            for name in entity.componentdef_names {
                if let Some(c) = self.componentdef_mut(name) {
                    c.remove(eid);
                }
            }
        }
    }
    pub fn componentdef_mut(&mut self, name: &str) -> Option<&mut dyn ComponentDef> {
        let &mut Self {
            entities: _,
            ref mut positions,
            ref mut velocities,
        } = self;
        match name {
            "Positions" => Some(positions),
            "Velocities" => Some(velocities),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idiomatic_ecs() {
        let mut cx = Cx::default();
        cx.velocities.update_positions(&mut cx.positions, 0., &mut cx.entities);
        #[cfg(not(miri))]
        cx.velocities.update_positions_par(&mut cx.positions, 0., &mut cx.entities);
        cx.remove_entity(&EID::default());
        // TODO: when a component is removed, its entity should remove it from its list
        // TODO: multithreading
    }
}
