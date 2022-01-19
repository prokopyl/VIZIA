use vizia::*;

#[derive(Lens)]
pub struct AppData {
    list: Vec<String>,
}

#[derive(Debug)]
pub enum AppEvent {
    EditItem(usize, std::ops::Range<usize>, String),
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::EditItem(index, range, text) => {
                    self.list[*index].replace_range(range.clone(), &*text);
                }
            }
        }
    }

}



fn main() {
    Application::new(WindowDescription::new().with_title("List"), |cx| {
        
        cx.add_stylesheet("examples/lists/list_style.css").unwrap();
        
        AppData { 
            list: vec!["First".to_string(), "Second".to_string(), "Third".to_string()],
        }.build(cx);

        List::new(cx, AppData::list, |cx, item| {
            //let item_text = item.get(cx).to_string();
            //Label::new(cx, &item_text);
            Textbox::new(cx, item.lens)
                .on_edit(move |cx, range, text| cx.emit(AppEvent::EditItem(item.index(), range, text)));

        }); // Center the list view in the window
    })
    .run();
}
