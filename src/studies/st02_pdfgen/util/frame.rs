use skia_safe::{Canvas, Paint, Rect};

pub fn draw_border_frame_to_canvas(canvas: &mut Canvas, radius: f32, paint: &Paint) {
    let dimensions = canvas.image_info().dimensions();
    let rect = Rect::from_xywh(
        radius,
        radius,
        dimensions.width as f32 - (radius * 2.0),
        dimensions.height as f32 - (radius * 2.0),
    );
    canvas.draw_rect(rect, paint);
}
