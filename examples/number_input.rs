use vizia::*;

#[derive(Lens)]
pub struct AppData {
    number: u32,
}

#[derive(Debug)]
pub enum AppEvent {
    UpdateNumber(u32),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::UpdateNumber(num) => {
                    self.number = *num;
                }
            }
        } 
    }
}

#[derive(Lens)]
pub struct TextData {
    text: String,
}

#[derive(Debug)]
pub enum TextEvent {
    UpdateText(std::ops::Range<usize>, String),
}

impl Model for TextData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(text_event) = event.message.downcast() {
            match text_event {
                TextEvent::UpdateText(range, text) => {
                    let mut new_text = self.text.clone();
                    new_text.replace_range(range.clone(), &*text);

                    if let Ok(num) = new_text.parse::<u32>() {
                        self.text = new_text.clone();
                        cx.emit(AppEvent::UpdateNumber(num));
                    } else {
                        println!("Failed to parse: {}", new_text);
                    }
                }
            }
        }
    }
}

fn main() {
    Application::new(WindowDescription::new(), |cx|{
        AppData {
            number: 5,
        }.build(cx);

        Binding::new(cx, AppData::number, |cx, number|{

            println!("Number: {}", number.get(cx));
            TextData {
                text: number.get(cx).to_string(),
            }.build(cx);

            Textbox::new(cx, TextData::text)
                .width(Pixels(100.0))
                .height(Pixels(30.0))
                .on_edit(|cx, range, text| {
                    cx.emit(TextEvent::UpdateText(range, text));
                });
        });
    }).run();
}