pub mod cameras {
    pub mod camera;
    pub mod orthographic;
    pub mod perspective;
}
pub mod filters {
    pub mod box_filter;
    pub mod filter;
    pub mod gaussian;
    pub mod lanczos_sinc;
    pub mod mitchell;
    pub mod triangle;
}
pub mod film;
pub mod geometry {
    pub mod bounding_box;
    pub mod matrix;
    pub mod normal;
    pub mod point;
    pub mod quaternion;
    pub mod ray;
    pub mod transform;
    pub mod vector;
}
pub mod math;
pub mod primitive;
pub mod renderer;
pub mod sampler;
pub mod scene;
pub mod shapes {
    pub mod cone;
    pub mod cylinder;
    pub mod disc;
    pub mod paraboloid;
    pub mod plane;
    pub mod rectangular_prism;
    pub mod shape;
    pub mod sphere;
    pub mod triangle;
}
