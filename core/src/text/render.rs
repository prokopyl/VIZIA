use std::io::BufRead;

use cssparser::Delimiter::Comma;
use femtovg::{Canvas, renderer::OpenGl, Path, Paint};
use parley::swash::{scale::ScaleContext, zeno::{PathData, Command}};

use super::layout::TextLayout;



pub fn draw_text(canvas: &mut Canvas<OpenGl>, text_layout: TextLayout, x: f32, y: f32) {

    let mut scale_ctx = ScaleContext::new();
    for line in text_layout.layout.lines() {
        let mut last_x = 0.0;
        let mut last_y = 0.0;
        for glyph_run in line.glyph_runs() {
            let run = glyph_run.run();
            let color = &glyph_run.style().brush;
            let font = run.font();
            let font = font.as_ref();
            let mut first = true;
            let mut scaler = scale_ctx.builder(font).size(run.font_size()).build();
            for glyph in glyph_run.positioned_glyphs() {
                let delta_x = glyph.x - last_x;
                let delta_y = glyph.y - last_y;
                canvas.translate(delta_x, -delta_y);
                last_x = glyph.x;
                last_y = glyph.y;

                if let Some(outline) = scaler.scale_outline(glyph.id) {
                    let mut path = Path::new();
                    for command in outline.path().commands() {
                        match command {
                            Command::MoveTo(point) => {
                                path.move_to(point.x, point.y);
                            }

                            Command::LineTo(point) => {
                                path.line_to(point.x, point.y);
                            }

                            Command::CurveTo(c1, c2, point) => {
                                path.bezier_to(c1.x, c1.y, c2.x, c2.y, point.x, point.y);
                            }

                            Command::QuadTo(c, point) => {
                                path.quad_to(c.x, c.y, point.x, point.y);
                            }

                            Command::Close => {
                                path.close();
                            }
                        }
                    }
                    let mut paint = Paint::color((*color).into());
                    paint.set_anti_alias(false);
                    canvas.fill_path(&mut path, paint);
                }

            }

            canvas.translate(-(x + last_x), y + last_y)
        }
    }
}
