#[repr(C)]
#[derive(Copy, Clone)]
pub struct GroundType {
    // Terrain speed multipliers.
    cost: [f32; 8],
    // Can build on this terrain?
    buildable: i32,
}

impl GroundType {
    pub fn get_land_type_from_name() -> bool {
        unsafe {
            let f: extern "fastcall" fn() -> bool = unsafe { std::mem::transmute(0x4E as *const ()) };
            f()
        }
    }
}
