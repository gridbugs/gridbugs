use gridbugs::{
    chargrid::{control_flow, core::app},
    chargrid_wgpu::{Config, Context, Dimensions, FontBytes},
};

const CELL_SIZE: f64 = 48.;

fn main() {
    let context = Context::new(Config {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin-custom.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA-custom.ttf").to_vec(),
        },
        title: "Hello, World!".to_string(),
        window_dimensions_px: Dimensions {
            width: 960.,
            height: 720.,
        },
        cell_dimensions_px: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        font_scale: Dimensions {
            width: CELL_SIZE,
            height: CELL_SIZE,
        },
        underline_width_cell_ratio: 0.1,
        underline_top_offset_cell_ratio: 0.8,
        resizable: false,
        force_secondary_adapter: false,
    });
    let app = control_flow::styled_string::<()>("Hello, World!".to_string(), Default::default())
        .centre()
        .press_any_key()
        .map(|_| app::Exit);
    context.run(app);
}
