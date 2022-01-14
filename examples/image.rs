use vizia::*;

fn main() {
    Application::new(WindowDescription::new(), |cx|{
        Image::new(cx, "examples/resources/moon.jpg")
            .size(Pixels(200.0))
            .background_color(Color::red());
    }).run();
}