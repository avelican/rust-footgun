use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    rand::srand(macroquad::miniquad::date::now() as _);

    let w = 400;
    let h = 400;

    // let w: u16 = 400;
    // let h: u16 = 400;


    // let file_length: i32 = (w * h).into(); // tmp

    let file_length  = w * h; // tmp

    
    let mut file_data:  Vec<u8> = vec![0u8; file_length as usize];

    for i in file_data.iter_mut() {
        *i = rand::gen_range(0, 255);
    }

    /* Prepare the raw byte array for image,
       we will transfer each byte to RGBA pixel */
    let mut raw_image_data: Vec<u8> = vec![0; (w * h * 4) as usize];

    /* Populate the raw array with pixel data in grayscale */
    for (i, byte) in file_data.iter().enumerate() {
        let pixel_base_index = i * 4;
        raw_image_data[pixel_base_index] = *byte;
        raw_image_data[pixel_base_index + 1] = *byte;
        raw_image_data[pixel_base_index + 2] = *byte;
        raw_image_data[pixel_base_index + 3] = 255; // Alpha channel
    }

    let texture = Texture2D::from_rgba8(w, h, &raw_image_data);

    loop {
        clear_background(WHITE);

        // Draw the texture 
        draw_texture_ex(
            &texture, 
            0.0, 0.0, 
            WHITE, 
            DrawTextureParams { 
                dest_size: Some(Vec2::new(screen_width(), screen_height())), // Fit to screen
                ..Default::default()
            }
        );

        next_frame().await
    }
}
