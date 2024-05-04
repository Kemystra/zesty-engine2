# Zesty Engine 2

## Min. Features
> Nothing ELSE other than this for now.
1. Able to show 1 rotating object in space that follows these rules:
    - Between near and far plane of camera
    - Big enough relative to camera
    - Not too big that it goes outside of screen
    - Shows simple colors (preferably hard-coded)
2. Read OBJ files
    - Parsing OBJ files should be handled by an external crate
    - Do NOT implement it; it's out of scope
3. Basic command line interface
    - mandatory argument `FILE` specify OBJ filepath

## External crate list
1. `softbuffer` - To provide drawing buffer to be passed to `winit`
2. `winit` - To draw window (out of scope of this project)
3. `num` - To have number types generalization
4. `clap` - To parse command line arguments and options

## Implementation Steps
1. Basic math library
    - Vector2D<T> and Vector3D<T> (where T: float OR integer)
        - Able to add, substract, and multiply with scalar (restricted to its type)
    - TransformMatrix and FullMatrix (use f64 to represent values)
        - Methods: `apply_rotation(Quaternion q)`
    - Quaternion (fml)
        - Methods: `normalize()`
    - Library functions to apply transformation to vectors

2. Rendering
    - Renderer
        - Will construct Softbuffer's buffer internally
        - Directly draw to Softbuffer's buffer
        - Main rendering functions goes here
    - Color(u8) unittype
        - Methods: `from_rgba(u8 r, u8 g, u8 b)`

3. Scene
    - Just a simple struct to hold our camera and objects

4. Camera
    - Contains crucial viewpoint data
    - Fields: `FOV: f64, near_clip_dist: f64, far_clip_dist: f64, proj_matrix: FullMatrix`

5. Object
    - Have a constructor that takes OBJ filepath
    - Must have vertices, edges, and facets definitions

