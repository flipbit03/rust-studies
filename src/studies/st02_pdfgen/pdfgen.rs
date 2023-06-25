use std::{
    fs::{read, File},
    io::Write,
    path::PathBuf,
};

use skia_safe::{
    colors,
    pdf::new_document,
    utils::text_utils::{draw_str, Align},
    Color, Data, Font, Paint, PaintStyle, PathEffect, Typeface,
};

use crate::{
    structs::{ShoppingItem, ShoppingList},
    util::frame::draw_border_frame_to_canvas,
};

mod structs;
mod util;

struct PdfGenConfig {
    assets_folder: String,
}

impl PdfGenConfig {
    pub fn new() -> Self {
        Self {
            assets_folder: "assets".to_string(),
        }
    }

    pub fn get_assets_folder(&self) -> PathBuf {
        // get the current directory
        let current_dir = std::env::current_dir().unwrap();

        // append the assets folder to the current directory
        current_dir.join(&self.assets_folder)
    }
}

fn main() {
    let config = PdfGenConfig::new();
    let assets_folder = config.get_assets_folder();
    match assets_folder.exists() {
        true => println!("Assets folder exists: {:?}", assets_folder),
        false => panic!("Assets folder does not exist: {:?}", assets_folder),
    }

    let shopping_list = ShoppingList {
        title: "Mercado".to_string(),
        items: vec![
            ShoppingItem {
                name: "Milk".to_string(),
                quantity: 1,
            },
            ShoppingItem {
                name: "Bread".to_string(),
                quantity: 2,
            },
            ShoppingItem {
                name: "Eggs".to_string(),
                quantity: 12,
            },
            ShoppingItem {
                name: "Butter".to_string(),
                quantity: 1,
            },
            ShoppingItem {
                name: "Cheese".to_string(),
                quantity: 1,
            },
        ],
    };

    // Create a canvas from the surface
    let doc = new_document(None);
    let a4_page_size = (2480.0, 3508.0);
    let mut first_page = doc.begin_page(a4_page_size, None);
    let mut first_page_canvas = first_page.canvas();

    let pink = Color::from_argb(255, 255, 182, 193);
    first_page_canvas.clear(pink);

    let mut rect_paint = Paint::new(colors::BLACK, None);
    rect_paint.set_style(PaintStyle::Stroke);
    rect_paint.set_path_effect(PathEffect::dash(&[10.0, 10.0, 20.0, 5.0], 10.0));

    draw_border_frame_to_canvas(&mut first_page_canvas, 50.0, &rect_paint);

    let font_paint = Paint::new(colors::BLUE, None);

    // Load Ani.ttf contents as a raw bytetr
    let font_bytes = read(assets_folder.join("Ani.ttf")).unwrap();
    let font_data = Data::new_copy(&font_bytes);
    let typeface = Typeface::from_data(font_data, None).unwrap();
    let font = Font::from_typeface(typeface, 48.0);

    draw_str(
        first_page_canvas,
        shopping_list.title,
        (a4_page_size.0 / 2.0, 100.0),
        &font,
        &font_paint,
        Align::Center,
    );

    for (i, item) in shopping_list.items.iter().enumerate() {
        let item_str = format!("{} - {}", item.name, item.quantity);
        draw_str(
            first_page_canvas,
            item_str,
            (100.0, 200.0 + (i as f32 * 50.0)),
            &font,
            &font_paint,
            Align::Left,
        );
    }

    let doc_with_page = first_page.end_page();

    let mut page2 = doc_with_page.begin_page(a4_page_size, None);
    let canvas2 = page2.canvas();
    canvas2.clear(colors::RED);

    let closed_doc = page2.end_page();

    let final_doc = closed_doc.close();

    let pdf_bytes = final_doc.as_bytes();

    println!("Writing test.pdf...");
    let mut file = File::create("test.pdf").unwrap();
    file.write(pdf_bytes).unwrap();
}
