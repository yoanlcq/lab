//!  Font handling in this library was designed to be quite customizable and lets
//!  you decide what you want to use and what you want to provide. There are three
//!  different ways to use the font atlas. The first two will use your font
//!  handling scheme and only requires essential data to run nuklear. The next
//!  slightly more advanced features is font handling with vertex buffer output.
//!  Finally the most complex API wise is using nuklears font baking API.
//!
//!  # 1.) Using your own implementation without vertex buffer output
//!
//!  So first up the easiest way to do font handling is by just providing a
//!  `nk_user_font` struct which only requires the height in pixel of the used
//!  font and a callback to calculate the width of a string. This way of handling
//!  fonts is best fitted for using the normal draw shape command API where you
//!  do all the text drawing yourself and the library does not require any kind
//!  of deeper knowledge about which font handling mechanism you use.
//!  IMPORTANT: the `nk_user_font` pointer provided to nuklear has to persist
//!  over the complete life time! I know this sucks but it is currently the only
//!  way to switch between fonts.
//!
//!      float your_text_width_calculation(nk_handle handle, float height, const char *text, int len)
//!      {
//!          your_font_type *type = handle.ptr;
//!          float text_width = ...;
//!          return text_width;
//!      }
//!
//!      struct nk_user_font font;
//!      font.userdata.ptr = &your_font_class_or_struct;
//!      font.height = your_font_height;
//!      font.width = your_text_width_calculation;
//!
//!      struct nk_context ctx;
//!      nk_init_default(&ctx, &font);
//!
//!  # 2.) Using your own implementation with vertex buffer output
//!
//!  While the first approach works fine if you don't want to use the optional
//!  vertex buffer output it is not enough if you do. To get font handling working
//!  for these cases you have to provide two additional parameters inside the
//!  `nk_user_font`. First a texture atlas handle used to draw text as subimages
//!  of a bigger font atlas texture and a callback to query a character's glyph
//!  information (offset, size, ...). So it is still possible to provide your own
//!  font and use the vertex buffer output.
//!
//!      float your_text_width_calculation(nk_handle handle, float height, const char *text, int len)
//!      {
//!          your_font_type *type = handle.ptr;
//!          float text_width = ...;
//!          return text_width;
//!      }
//!      void query_your_font_glyph(nk_handle handle, float font_height, struct nk_user_font_glyph *glyph, nk_rune codepoint, nk_rune next_codepoint)
//!      {
//!          your_font_type *type = handle.ptr;
//!          glyph.width = ...;
//!          glyph.height = ...;
//!          glyph.xadvance = ...;
//!          glyph.uv[0].x = ...;
//!          glyph.uv[0].y = ...;
//!          glyph.uv[1].x = ...;
//!          glyph.uv[1].y = ...;
//!          glyph.offset.x = ...;
//!          glyph.offset.y = ...;
//!      }
//!
//!      struct nk_user_font font;
//!      font.userdata.ptr = &your_font_class_or_struct;
//!      font.height = your_font_height;
//!      font.width = your_text_width_calculation;
//!      font.query = query_your_font_glyph;
//!      font.texture.id = your_font_texture;
//!
//!      struct nk_context ctx;
//!      nk_init_default(&ctx, &font);
//!
//!  # 3.) Nuklear font baker
//!
//!  The final approach if you do not have a font handling functionality or don't
//!  want to use it in this library is by using the optional font baker.
//!  The font baker API's can be used to create a font plus font atlas texture
//!  and can be used with or without the vertex buffer output.
//!
//!  It still uses the `nk_user_font` struct and the two different approaches
//!  previously stated still work. The font baker is not located inside
//!  `nk_context` like all other systems since it can be understood as more of
//!  an extension to nuklear and does not really depend on any `nk_context` state.
//!
//!  Font baker need to be initialized first by one of the nk_font_atlas_init_xxx
//!  functions. If you don't care about memory just call the default version
//!  `nk_font_atlas_init_default` which will allocate all memory from the standard library.
//!  If you want to control memory allocation but you don't care if the allocated
//!  memory is temporary and therefore can be freed directly after the baking process
//!  is over or permanent you can call `nk_font_atlas_init`.
//!
//!  After successfull intializing the font baker you can add Truetype(.ttf) fonts from
//!  different sources like memory or from file by calling one of the `nk_font_atlas_add_xxx`.
//!  functions. Adding font will permanently store each font, font config and ttf memory block(!)
//!  inside the font atlas and allows to reuse the font atlas. If you don't want to reuse
//!  the font baker by for example adding additional fonts you can call
//!  `nk_font_atlas_cleanup` after the baking process is over (after calling nk_font_atlas_end).
//!
//!  As soon as you added all fonts you wanted you can now start the baking process
//!  for every selected glyphes to image by calling `nk_font_atlas_bake`.
//!  The baking process returns image memory, width and height which can be used to
//!  either create your own image object or upload it to any graphics library.
//!  No matter which case you finally have to call `nk_font_atlas_end` which
//!  will free all temporary memory including the font atlas image so make sure
//!  you created our texture beforehand. `nk_font_atlas_end` requires a handle
//!  to your font texture or object and optionally fills a `struct nk_draw_null_texture`
//!  which can be used for the optional vertex output. If you don't want it just
//!  set the argument to `NULL`.
//!
//!  At this point you are done and if you don't want to reuse the font atlas you
//!  can call `nk_font_atlas_cleanup` to free all truetype blobs and configuration
//!  memory. Finally if you don't use the font atlas and any of it's fonts anymore
//!  you need to call `nk_font_atlas_clear` to free all memory still being used.
//!
//!      struct nk_font_atlas atlas;
//!      nk_font_atlas_init_default(&atlas);
//!      nk_font_atlas_begin(&atlas);
//!      nk_font *font = nk_font_atlas_add_from_file(&atlas, "Path/To/Your/TTF_Font.ttf", 13, 0);
//!      nk_font *font2 = nk_font_atlas_add_from_file(&atlas, "Path/To/Your/TTF_Font2.ttf", 16, 0);
//!      const void* img = nk_font_atlas_bake(&atlas, &img_width, &img_height, NK_FONT_ATLAS_RGBA32);
//!      nk_font_atlas_end(&atlas, nk_handle_id(texture), 0);
//!
//!      struct nk_context ctx;
//!      nk_init_default(&ctx, &font->handle);
//!      while (1) {
//!
//!      }
//!      nk_font_atlas_clear(&atlas);
//!
//!  The font baker API is probably the most complex API inside this library and
//!  I would suggest reading some of my examples `example/` to get a grip on how
//!  to use the font atlas. There are a number of details I left out. For example
//!  how to merge fonts, configure a font with `nk_font_config` to use other languages,
//!  use another texture coodinate format and a lot more:
//!
//!      struct nk_font_config cfg = nk_font_config(font_pixel_height);
//!      cfg.merge_mode = nk_false or nk_true;
//!      cfg.range = nk_font_korean_glyph_ranges();
//!      cfg.coord_type = NK_COORD_PIXEL;
//!      nk_font *font = nk_font_atlas_add_from_file(&atlas, "Path/To/Your/TTF_Font.ttf", 13, &cfg);
//!
//!

use nuke_sys::*;
/// The safer equivalent to `nk_user_font`.
pub struct VirtualFont {
    
}

impl VirtualFont {
    pub fn to_nk(&self) -> nk_user_font {
        unimplemented!()
    }
}