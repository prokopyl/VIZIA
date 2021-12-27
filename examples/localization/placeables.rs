use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Localization"), |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, LocalizedString::new("remove-bookmark").with_arg("title", "test"));
        });
    })
    .run();
}
