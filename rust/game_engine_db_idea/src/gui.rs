use std::collections::HashMap;

pub type EID = u32;

pub struct Aabr<T> {
    pub min: (T, T),
    pub max: (T, T),
}

// Stores the state of the GUI
pub struct GuiDB {
    // ----
    pub focus: Option<EID>,
    pub focus_area: Option<EID>,
    pub hover: Option<EID>,
    pub root: EID,

    // ----
    pub area: HashMap<EID, Area>,
}

pub enum Gravity {
    North,
    South,
    East,
    West,
}

pub enum SplitLine {
    H, V,
}

pub enum Area {
    Whole(EID),
    Split {
        line: SplitLine,
        line_offset: f32, // Offset from the center of the split, from -1 to 1. X goes right, and Y goes down.
        child_areas: (EID, EID),
    },
}

pub struct BaseWidget {
    pub aabr: Aabr<u32>,
}

impl GuiDB {
    pub fn new() -> Self {
        Self {
            focus: None,
            focus_area: Some(0),
            hover: None,
            root: 0,
            area: HashMap::new(),
        }
    }
    pub fn new_eid(&mut self) -> EID {
        unimplemented!()
    }
    pub fn focused_area(&self) -> Option<EID> {
        self.focus_area
    }
    pub fn focus_area(&mut self, eid: EID) {
        self.focus_area = Some(eid);
    }
    pub fn split(&mut self, line: SplitLine) {
        let eid = self.focused_area().unwrap();
        match self.area.get(&eid) {
            Some(&Area::Whole(area_id)) => {
                let child_areas = (area_id, self.new_eid());
                self.area.insert(eid, Area::Split { line, line_offset: 0., child_areas, });
            },
            _ => unimplemented!(),
        }
    }
}

fn main() {
    // Root area
    // - Split H
    // 
    // Pump events, forward them to GUI system, which changes state as necessary
    // Recompute layout animation caused by changes
    // Draw the GUI

    // - Clickable text: Text + OnClick
    // - Button: Text + OnMouseDown + OnMouseMove + OnMouseUp + ButtonStyle
    // - Slider: float + OnMouseDown + OnMouseMove + SliderStyle

    // ui similar to blender
    // Docker
    // - Place elements one by one (block)
}