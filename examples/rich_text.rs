

use std::ops::{Range, Add};

use vizia::*;

use femtovg::{Paint, Path, Baseline, Align};

fn main() {
    Application::new(WindowDescription::new(), |cx|{
        
        
        VStack::new(cx, |cx|{
            RichText::new(cx,
           (
                    "Hello ".color(Color::red()) + 
                    "Frances, ".color(Color::blue())
                ).color(Color::black()) +
                "let's ".color(Color::rgb(0,255,0)) +
                "go ".color(Color::rgb(0,255,255)) +
                "make " +
                "some ".color(Color::rgb(255,255,0)) +
                "food!".color(Color::rgb(255,0,255))
            );
        }).child_space(Auto).space(Pixels(100.0));
    }).run();
}

pub struct RichText {
    text: StyledText,
}

impl RichText {
    pub fn new(cx: &mut Context, text: impl StylableText) -> Handle<Self> {
        
        // let txt = text.as_text();
        // println!("Text: {} {:?}", txt.text, txt.color);

        Self {
            text: text.as_text(),
        }.build2(cx, |cx|{

        }).overflow(Overflow::Visible)
    }
}

impl View for RichText {
    fn draw(&self, cx: &Context, canvas: &mut Canvas) {
        let entity = cx.current;

        let bounds = cx.cache.get_bounds(entity);

        let font = cx.style.font.get(entity).cloned().unwrap_or_default();

        // TODO - This should probably be cached in cx to save look-up time
        let default_font = cx
            .resource_manager
            .fonts
            .get(&cx.style.default_font)
            .and_then(|font| match font {
                FontOrId::Id(id) => Some(id),
                _ => None,
            })
            .expect("Failed to find default font");

        let font_id = cx
            .resource_manager
            .fonts
            .get(&font)
            .and_then(|font| match font {
                FontOrId::Id(id) => Some(id),
                _ => None,
            })
            .unwrap_or(default_font);

        // let mut x = posx + (border_width / 2.0);
        // let mut y = posy + (border_width / 2.0);

        let mut x = bounds.x;
        let mut y = bounds.y;

        for span in self.text.color.iter() {
            let text_string = &self.text.text[span.range.clone()];

            let font_color = span.attribute;
    
    
            let mut font_color: femtovg::Color = font_color.into();
            //font_color.set_alphaf(font_color.a * opacity);
    
            let font_size = cx.style.font_size.get(entity).cloned().unwrap_or(24.0);
    
            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id.clone()]);
            paint.set_text_align(Align::Left);
            paint.set_text_baseline(Baseline::Middle);
            paint.set_anti_alias(false);

            if let Ok(text_metrics) = canvas.fill_text(x, y, &text_string, paint) {
                x += text_metrics.width();
            }

        }


    }
}

pub trait StylableText {
    fn as_str(&self) -> &str;
    fn color(&self, color: Color) -> StyledText;
    fn font_size(&self, size: f32) -> StyledText;
    fn as_text(self) -> StyledText;
}

impl StylableText for &'static str {
    fn as_str(&self) -> &str {
        self
    }

    fn as_text(self) -> StyledText {
        StyledText {
            text: self.to_string(),
            color: vec![Span {range: 0..self.len(), attribute: Color::black()}],
            font_size: vec![],
        }
    }

    fn color(&self, color: Color) -> StyledText {
        StyledText {
            text: self.to_string(),
            color: vec![Span{range: 0..self.len(), attribute: color}],
            font_size: vec![],
        }
    }

    fn font_size(&self, size: f32) -> StyledText {
        StyledText {
            text: self.to_string(),
            color: vec![],
            font_size: vec![Span{range: 0..self.len(), attribute: size}],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StyledText {
    text: String,
    color: Vec<Span<Color>>,
    font_size: Vec<Span<f32>>,
}

impl StylableText for StyledText {
    fn as_str(&self) -> &str {
        self.text.as_str()
    }

    fn as_text(self) -> StyledText {
        self
    }

    fn color(&self, color: Color) -> StyledText {
        StyledText {
            text: self.text.clone(),
            color: vec![Span{ range: 0..self.text.len(), attribute: color}],
            ..Default::default()
        }
    }

    fn font_size(&self, size: f32) -> StyledText {
        StyledText {
            text: self.text.clone(),
            font_size: vec![Span{ range: 0..self.text.len(), attribute: size}],
            ..Default::default()
        }
    }
}

impl<T: StylableText> Add<T> for StyledText {
    type Output = StyledText;

    fn add(mut self, rhs: T) -> Self::Output {

        let mut rhs = rhs.as_text();

        for span in rhs.color.iter_mut() {
            span.range = (span.range.start + self.text.len())..(span.range.end + self.text.len());
        }

        self.text = self.text + rhs.text.as_str();
        self.color.extend(rhs.color.drain(0..));
        
        self
    }
}

#[derive(Debug, Clone)]
struct Span<T> {
    range: Range<usize>,
    attribute: T,
}