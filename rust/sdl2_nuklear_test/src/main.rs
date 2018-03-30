#![feature(associated_consts)]
#![feature(slice_patterns)]
#![feature(const_fn)]
//#![deny(missing_docs)]
//#![deny(warnings)]

extern crate sdl2;
extern crate nuklear_rust as nk;
use nk::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Difficulty {
    Easy, 
    Hard
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
#[repr(C, packed)]
struct GuiVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [u8; 4]
}
use std::mem::size_of;
impl GuiVertex {
    const OFFSETOF_POSITION: usize = 0;
    const OFFSETOF_UV: usize = 8;
    const OFFSETOF_COLOR: usize = 16;
}

const MAX_VERTEX_BUFFER: usize = 512 * 1024;
const MAX_ELEMENT_BUFFER: usize = 128 * 1024;


struct App<'a> {
    sdl2: sdl2::Sdl,
    rdr: sdl2::render::Renderer<'a>,
    nk: nk::NkContext,
    property: i32,
    op: Difficulty,
    aa: NkAntiAliasing,
}
enum AppEventHandling {
    Quit,
    Unhandled,
}

trait IntoNkKey {
    fn to_nk_key(&self) -> NkKey;
}

use sdl2::keyboard::Keycode;
use sdl2::keyboard::Keycode::*;
use NkKey::*;

impl IntoNkKey for Keycode {
    fn to_nk_key(&self) -> NkKey {
        match self {
            &Up => NK_KEY_UP,
            _ => NK_KEY_NONE,
        }
    }
}

impl<'a> App<'a> {
    fn new() -> Self {
        let sdl2 = sdl2::init().expect("Could not initialize SDL2");
        let sdl2_video = sdl2.video().expect("Could not initialize video subsystem");
        let window = sdl2_video.window("Foo", 800, 600).build().unwrap();
        let rdr = 
            window.renderer().accelerated().present_vsync().build().unwrap();
        use std::fs::File;
        use std::io::Read;
        let mut buf = Vec::<u8>::new();
        let mut file = File::open("/home/yoon/.local/share/fonts/basis33.ttf").unwrap();
        file.read_to_end(&mut buf).unwrap();
        drop(file);
        let mut atlas = NkFontAtlas::new(&mut NkAllocator::new_vec());
        atlas.begin();
        let mut font = atlas.add_font_with_bytes(&buf, 16f32).unwrap();
        let _ = atlas.bake(NkFontAtlasFormat::NK_FONT_ATLAS_RGBA32);
        atlas.end(NkHandle::from_ptr(std::ptr::null_mut()), None);
        let mut nk = NkContext::new(&mut NkAllocator::new_vec(), &font.handle());
        nk.style_set_font(&font.handle());
        App {
            sdl2, rdr, nk, property: 0, op: Difficulty::Easy, aa: NkAntiAliasing::NK_ANTI_ALIASING_OFF
        }
    }

    fn handle_event(&mut self, e: &sdl2::event::Event) -> Result<(), AppEventHandling> {
        use sdl2::event::Event;
        self.nk.input_begin();
        let out = match e {
            &Event::MouseMotion { x, y, .. } => {
                println!("Mouse at {:?}", (x, y));
                Ok(())
            },
            &Event::KeyDown { keycode, .. } => {
                self.nk.input_key(keycode.unwrap().to_nk_key(), true);
                println!("Key {:?} is down", keycode);
                Ok(())
            },
            &Event::KeyUp { keycode, .. } => {
                self.nk.input_key(keycode.unwrap().to_nk_key(), false);
                println!("Key {:?} is up", keycode);
                Ok(())
            },
            &Event::TextInput {..} => Ok(()),
            &Event::MouseButtonDown { mouse_btn, .. } => {
                println!("Mouse {:?} is down", mouse_btn);
                Ok(())
            },
            &Event::MouseButtonUp { mouse_btn, .. } => {
                println!("Mouse {:?} is up", mouse_btn);
                Ok(())
            },
            &Event::MouseWheel { direction, y, .. } => {
                println!("Mouse scroll: {} ({:?})", y, direction);
                Ok(())
            },
            &Event::Window {..} => Ok(()),
            &Event::Quit {..} => Err(AppEventHandling::Quit),
            _ => Err(AppEventHandling::Unhandled),
        };
        self.nk.input_end();
        out
    }

    fn gui(&mut self) {
        let nk = &mut self.nk;
        let title = NkString::from("Foo");
        let rect  = NkRect {x:20f32, y:30f32, w:200f32, h:200f32};
        use NkPanelFlags::*;
        let flags = NK_WINDOW_MOVABLE     as u32
                  | NK_WINDOW_SCALABLE    as u32
                  | NK_WINDOW_CLOSABLE    as u32
                  | NK_WINDOW_MINIMIZABLE as u32
                  | NK_WINDOW_TITLE       as u32;
        if nk.begin(title, rect, flags) {

            use NkTextAlignment::*;
            nk.menubar_begin();
            nk.layout_row_begin(NkLayoutFormat::NK_STATIC, 25f32, 2);
            nk.layout_row_push(45f32);
            if nk.menu_begin_label(NkString::from("FILE"), NK_TEXT_LEFT as u32, NkVec2 { x:120f32, y:200f32  }) {
                nk.layout_row_dynamic(30f32, 1);
                nk.menu_item_label(NkString::from("OPEN"), NK_TEXT_LEFT as u32);
                nk.menu_item_label(NkString::from("CLOSE"), NK_TEXT_LEFT as u32);
                nk.menu_end();
            }
            nk.layout_row_push(45f32);
            if nk.menu_begin_label(NkString::from("EDIT"), NK_TEXT_LEFT as u32, NkVec2 { x:120f32, y:200f32  }) {
                nk.layout_row_dynamic(30f32, 1);
                nk.menu_item_label(NkString::from("CUT"), NK_TEXT_LEFT as u32);
                nk.menu_item_label(NkString::from("COPY"), NK_TEXT_LEFT as u32);
                nk.menu_item_label(NkString::from("CLOSE"), NK_TEXT_LEFT as u32);
                nk.menu_end();
            }
            nk.layout_row_end();
            nk.menubar_end();

            nk.layout_row_static(30f32, 80, 1);
            if nk.button_label(NkString::from("button")) {
                println!("Button pressed!");
            }
            use Difficulty::*;
            nk.layout_row_dynamic(30f32, 2);
            if nk.option_label(NkString::from("Easy"), self.op == Easy) {
                self.op = Easy;
            }
            if nk.option_label(NkString::from("Hard"), self.op == Hard) {
                self.op = Hard;
            }
            nk.layout_row_dynamic(25f32, 1);
            nk.property_int(NkString::from("Compression:"), 0, &mut self.property, 100, 10, 1f32);
        }
        nk.end();
    }

    fn render_gui(&mut self) {
        let nk = &mut self.nk;
        const MAX_CMDS_BUFFER: usize = MAX_VERTEX_BUFFER;
        let mut cmd_mem = [0u8; MAX_CMDS_BUFFER];
        let mut cmds = NkBuffer::with_fixed(&mut cmd_mem);
        use NkDrawVertexLayoutFormat::*;
        use NkDrawVertexLayoutAttribute::*;
        let vlayout = NkDrawVertexLayoutElements::new(&[
            (NK_VERTEX_POSITION, NK_FORMAT_FLOAT, GuiVertex::OFFSETOF_POSITION as u32),
            (NK_VERTEX_TEXCOORD, NK_FORMAT_FLOAT, GuiVertex::OFFSETOF_UV as u32),
            (NK_VERTEX_COLOR, NK_FORMAT_R8G8B8A8, GuiVertex::OFFSETOF_COLOR as u32),
            // è_é
            (NK_VERTEX_ATTRIBUTE_COUNT, NK_FORMAT_COUNT, 0),
        ]);
        let mut config = NkConvertConfig::default();
        config.set_vertex_layout(&vlayout);
        config.set_vertex_size(size_of::<GuiVertex>());
        config.set_null(NkDrawNullTexture::default());
        config.set_circle_segment_count(22);
        config.set_curve_segment_count(22);
        config.set_arc_segment_count(22);
        config.set_global_alpha(1f32);
        config.set_shape_aa(self.aa);
        config.set_line_aa(self.aa);
        /*
        use std::slice::from_raw_parts_mut as frpm;
        let mut vmem = [0u8; MAX_VERTEX_BUFFER];
        let mut emem = [0u8; MAX_ELEMENT_BUFFER];
        let vertices = unsafe { frpm(vmem.as_mut_ptr() as *mut GuiVertex, MAX_VERTEX_BUFFER/size_of::<GuiVertex>()) };
        let elements = unsafe { frpm(emem.as_mut_ptr() as *mut u16, MAX_ELEMENT_BUFFER/size_of::<u16>()) };
        let mut vbuf = NkBuffer::with_fixed(&mut vmem);
        let mut ebuf = NkBuffer::with_fixed(&mut emem);

        nk.convert(&mut cmds, &mut ebuf, &mut vbuf, &config);

        let mut offset = 0;
        for cmd in nk.draw_command_iterator(&cmds) {
            let elem_count = cmd.elem_count();
            if elem_count <= 0 {
                continue;
            }
            for i in offset..(offset+elem_count) {
                let e = elements[i as usize] as usize;
                println!("i: {}", e);
                let v = vertices[e];
                let [x,y] = v.position;
                let [r,g,b,a] = v.color;
                let (w,h) = self.rdr.window().unwrap().size();
                let x = (w as f32*(x+1f32)/2f32) as i32;
                let y = -(h as f32*(y+1f32)/2f32) as i32;
                self.rdr.set_draw_color(sdl2::pixels::Color::RGBA(r,g,b,a));
                self.rdr.draw_point(sdl2::rect::Point::new(x,y)).unwrap();
            }
            offset += elem_count;
        }
        */
        for cmd in nk.command_iterator() {
            println!("cmd: {:?}", cmd);
        }
        nk.clear();
    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    let mut event_pump = app.sdl2.event_pump().unwrap();
    'running: loop {
        for e in event_pump.poll_iter() {
            use AppEventHandling::*;
            match app.handle_event(&e) {
                Ok(_) => (),
                Err(Quit) => break 'running,
                Err(Unhandled) => println!("Unhandled event: {:?}", &e),
            };
        }
        use sdl2::pixels::Color::*;
        use sdl2::rect::Rect;
        app.rdr.set_draw_color(RGB(255, 255, 0));
        app.rdr.clear();
        app.rdr.set_draw_color(RGB(0, 255, 255));
        app.rdr.draw_rect(Rect::new(20, 20, 50, 50)).unwrap();
        app.gui();
        app.render_gui();
        app.rdr.present();
    }
}
