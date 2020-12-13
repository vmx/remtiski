use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, mxcfb_rect, waveform_mode, DISPLAYHEIGHT, DISPLAYWIDTH,
};
use libremarkable::framebuffer::core::Framebuffer;
use libremarkable::framebuffer::{FramebufferBase, FramebufferIO, FramebufferRefresh};
use tiny_skia::{Canvas, LineCap, LineJoin, Paint, PathBuilder, PixmapMut, Stroke};

fn draw_in_pixmap(pixmap: PixmapMut) {
    let mut canvas = Canvas::from(pixmap);

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    let path = {
        const TOP: f32 = 500.00;
        const BOTTOM: f32 = 1200.00;
        let mut builder = PathBuilder::new();
        builder.move_to(75.0, TOP);
        builder.line_to(275.0, BOTTOM);
        builder.line_to(475.0, TOP);
        builder.move_to(325.0, BOTTOM);
        builder.line_to(525.0, TOP);
        builder.line_to(725.0, BOTTOM);
        builder.line_to(925.0, TOP);
        builder.line_to(1325.0, BOTTOM);
        builder.move_to(925.0, BOTTOM);
        builder.line_to(1325.0, TOP);
        builder.finish().unwrap()
    };

    let stroke = Stroke {
        width: 20.0,
        line_cap: LineCap::Butt,
        line_join: LineJoin::Bevel,
        ..Default::default()
    };

    canvas.stroke_path(&path, &paint, &stroke);
}

fn main() {
    let mut buffer = vec![0; DISPLAYWIDTH as usize * DISPLAYHEIGHT as usize * 4];
    let pixmap =
        PixmapMut::from_bytes(&mut buffer, DISPLAYWIDTH.into(), DISPLAYHEIGHT.into()).unwrap();
    draw_in_pixmap(pixmap);

    let mut framebuffer = Framebuffer::new("/dev/fb0");

    // ARGB to RGB565
    let remarkable_buffer: Vec<u8> = buffer
        .chunks(4)
        .flat_map(|pixel| {
            let (red, green, blue) = (pixel[1], pixel[2], pixel[3]);
            let col = color::RGB(red, green, blue);
            col.as_native().to_vec()
        })
        .collect();

    // The framebuffer is bigger than the canvas that can display something (it's 4 bytes longer
    // per line), hence don't put it into the framebuffer directly, but use the `restore_region()`
    // call.
    framebuffer
        .restore_region(
            mxcfb_rect {
                top: 0,
                left: 0,
                width: DISPLAYWIDTH as u32,
                height: DISPLAYHEIGHT as u32,
            },
            &remarkable_buffer,
        )
        .unwrap();

    framebuffer.full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16_FAST,
        display_temp::TEMP_USE_AMBIENT,
        dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
        0,
        true,
    );
}
