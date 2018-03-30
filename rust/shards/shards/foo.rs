#[macro_use]
extern crate game;

use game::*;

shard!(|s| {
    println!("Foo: I've been loaded!");
    s.on_unload(unload);
    s.on_draw(draw);
    ShardMetadata {
        name: "The Foo Shard???",
        description: "The Foo Shard updates all the foos",
    }
});

fn unload(_: &OnUnload) {
    println!("Foo: I've been unloaded!");
}
fn draw(e: &OnDraw) {
    println!("Foo: Drawing whoops! (frame {})", e.frame_number);
}
