use extendr_api::prelude::*;
//use wk_rs::bindings::{wk::{wk_handler_t, WK_CONTINUE}};
use wk_rs::bindings::{wk::*};

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod wkrsgeo;
    fn hello_world;
}



// Does Coord need to be in a known format?
use sfconversions::Geom;

// Creates an empty wk_meta_t struct based on a geometry type struct
fn rsgeo_empty_meta_t(geo_type: wk_geometery_type_enum) -> wk_meta_t {
    wk_meta_t {
        geometry_type: geo_type as u32,
        flags: 0,
        srid: WK_SRID_NONE,
        precision: WK_PRECISION_NONE,
        size: WK_SIZE_UNKNOWN,
        bounds_min: [f64::NEG_INFINITY; 4usize],
        bounds_max: [f64::INFINITY; 4usize],
    }
}

use wk_geometery_type_enum::*;
use geo_types::*;

fn rsgeo_wk_read_point(g: Geom, part_id: u32, handler: wk_handler_t) -> i32 {   

    let mut meta = rsgeo_empty_meta_t(WK_POINT);
    meta.size = 1; // set size = 1 for 1 point

    // extract the geometry_start function?
    let geo_start = handler.geometry_start.unwrap();
    // run the geostart (not sure whats happening here)
    unsafe { geo_start(&meta, part_id, handler.handler_data) };

    let handle_coord = handler.coord.unwrap();

    let crd = Point::from(g).0;
    //let mut xy = [crd.x, crd.y];
    // this is incorrect because the coord needs to be an actual coordinate
    unsafe { handle_coord(&meta, &crd.x, part_id, handler.handler_data); }

    let geo_end = handler.geometry_end.unwrap();

    unsafe { geo_end(&meta, part_id, handler.handler_data) }
}



// # - implement from the inside out
// # - start with geometry_start & geometry end
// # - then feature_start & featur_end
// # - then vector_start & vector_end
// # - deinitialize should do nothing since rust will handle cleanup
// # - each function should return 
