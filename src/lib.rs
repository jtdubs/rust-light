pub mod filter;
pub mod math;
pub mod sampler;
pub mod film;
pub mod aabb;
pub mod scene;
pub mod primitive;
pub mod renderer;
pub mod cameras {
    pub mod camera;
    pub mod orthographic;
    pub mod perspective;
}
pub mod geometry {
    pub mod matrix;
    pub mod normal;
    pub mod point;
    pub mod quaternion;
    pub mod ray;
    pub mod transform;
    pub mod vector;
}
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
