use vizia::*;

const STYLE: &str = r#"
    hstack {
        color: green;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    list: Vec<String>,
}

impl Model for AppData {

}

fn main() {

    let app_data = AppData {
        list: vec!["First".to_string(), "Second".to_string()],
    };

    let lens = AppData::list.index(1);

    let value = lens.view(&app_data);

    println!("{:?}", value);


}

// .border_shape_top_left(BorderCornerShape::Bevel)
