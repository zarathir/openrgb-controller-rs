use core::time;
use std::{error::Error, thread};

use openrgb::{
    data::{self, Color},
    OpenRGB,
};
use rgb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = OpenRGB::connect().await?;

    let controllers = client.get_controller_count().await?;

    let mut keyboard_id: u32 = 0;
    let mut mouse_id: u32 = 0;
    let mut mouse_mat_id: u32 = 0;

    for controller_id in 0..controllers {
        let controller = client.get_controller(controller_id).await?;

        if controller.r#type == data::DeviceType::Keyboard {
            keyboard_id = controller_id;
        }

        if controller.r#type == data::DeviceType::Mouse {
            mouse_id = controller_id;
        }

        if controller.r#type == data::DeviceType::MouseMat {
            mouse_mat_id = controller_id;
        }
    }

    let keyboard = client.get_controller(keyboard_id).await?;
    let mouse = client.get_controller(mouse_id).await?;
    let mouse_mat = client.get_controller(mouse_mat_id).await?;

    let mut keyboard_colors = Vec::<Color>::new();
    let mut mouse_colors = Vec::<Color>::new();
    let mut mouse_mat_colors = Vec::<Color>::new();

    let mut r: u8 = 255;
    let mut g: u8 = 0;
    let mut b: u8 = 0;

    let handle = tokio::task::spawn(async move {
        loop {
            let rgb = RGB8::new(r, g, b);

            for _ in &keyboard.leds {
                keyboard_colors.push(rgb);
            }

            for _ in &mouse.leds {
                mouse_colors.push(rgb);
            }

            for _ in &mouse_mat.leds {
                mouse_mat_colors.push(rgb)
            }

            client
                .update_leds(keyboard_id, keyboard_colors.to_vec())
                .await
                .unwrap();

            client
                .update_leds(mouse_id, mouse_colors.to_vec())
                .await
                .unwrap();

            client
                .update_leds(mouse_mat_id, mouse_mat_colors.to_vec())
                .await
                .unwrap();

            if r == 255 && g < 255 {
                g += 1;
                if b > 0 {
                    b -= 1;
                }
            }
            if r > 0 && g == 255 && b < 255 {
                b += 1;
                r -= 1;
            }
            if r < 255 && g > 0 && b == 255 {
                r += 1;
                g -= 1;
            }

            keyboard_colors.clear();
            mouse_colors.clear();
            mouse_mat_colors.clear();

            thread::sleep(time::Duration::from_millis(50))
        }
    });

    handle.await?;

    Ok(())
}
