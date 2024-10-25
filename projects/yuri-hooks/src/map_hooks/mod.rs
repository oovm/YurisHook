#[repr(C)]
#[derive(Copy, Clone)]
pub struct GroundType {
    // Terrain speed multipliers.
    cost: [f32; 8],
    // Can build on this terrain?
    buildable: i32,
}
