use colored::customcolors::CustomColor;
use colored::Colorize;
use std::fs::read_to_string;

const CAMERA_DISTANCE: f32     = 1.0; // The lower this is the bigger this object appears. This is normal but it does not align with the supposed maths. WTF?
const LIGHT_DIRECTION: Vector3 = Vector3 {x: 0.0, y: 1.0, z: 0.0};
const SIZE_MULTIPLIER: f32 = 1.0;


fn main() {
    let file_path: &str = "objects/cube.obj"; // this maybe or maybe not breaks on windows idk
    let mut screen = new_screen(48,48);

    let triangles: Vec<Triangle> = read_obj(file_path);
    println!("Triangles: {:?}", triangles);

    let triangles2d = render_triangles(triangles);
    println!("Triangles: {:?}", triangles2d);
}

// Turn triangles in a 3D space into triangles in a 2D space
fn render_triangles(triangles: Vec<Triangle>) -> Vec<Triangle2D> {
    /*
     point (y; z)       ------------- this line = screen
        ----\          |
        |    ----\    \/
        |         ----\
     yi |             |----\
        |           y |    ----\
        |             |          ----\
        --------------------------------------  CAMERA
.           Z (depth)     camera_distance
    yi = y position in 3d world
    Z = z position in 3d world
    y = y position on 2d screen
    */
    // According to Thales' theorem,    yi / y == (Z + cam_dist) / cam_dist
    // We are trying to find y,      => y = yi *  ((Z + cam_dist) / cam_dist)
    // Same for x (replacing y in the equations).
    //
    // the shown maths are for a 2D -> 1D conversion. for 3D -> 2D just do this twice, once for x and once for y

    let mut triangles2d: Vec<Triangle2D> = Vec::new(); // This will be the output
    for triangle in triangles {
        let triangle2d = Triangle2D {
            // Do the maths shown above
            a: Vector2 { 
                x: triangle.a.x * ((triangle.a.z + CAMERA_DISTANCE) / CAMERA_DISTANCE) * SIZE_MULTIPLIER,
                y: triangle.a.y * ((triangle.a.z + CAMERA_DISTANCE) / CAMERA_DISTANCE),
            },
            b: Vector2 { 
                x: triangle.b.x * ((triangle.b.z + CAMERA_DISTANCE) / CAMERA_DISTANCE),
                y: triangle.b.y * ((triangle.b.z + CAMERA_DISTANCE) / CAMERA_DISTANCE),
            },
            c: Vector2 { 
                x: triangle.c.x * ((triangle.c.z + CAMERA_DISTANCE) / CAMERA_DISTANCE),
                y: triangle.c.y * ((triangle.c.z + CAMERA_DISTANCE) / CAMERA_DISTANCE),
            },
            // A · B = (A.x * B.x) + (A.y * B.y)
            // dot product of sun direction and triangle normal to get a shitty lighting model
            col: CustomColor {
                r: (((triangle.n.x * LIGHT_DIRECTION.x) + (triangle.n.y * LIGHT_DIRECTION.y) + (triangle.n.z * LIGHT_DIRECTION.z)) * 255.0) as u8,
                g: (((triangle.n.x * LIGHT_DIRECTION.x) + (triangle.n.y * LIGHT_DIRECTION.y) + (triangle.n.z * LIGHT_DIRECTION.z)) * 255.0) as u8,
                b: (((triangle.n.x * LIGHT_DIRECTION.x) + (triangle.n.y * LIGHT_DIRECTION.y) + (triangle.n.z * LIGHT_DIRECTION.z)) * 255.0) as u8,
            },
        };
        triangles2d.push(triangle2d);
    }
    return triangles2d;
}


fn draw(triangles: Vec<Triangle2D>, screen: Screen) /* -> Screen */ {
    
}


//creates a blank screen
fn new_screen(height: usize, width: usize) -> Screen {
    return Screen {
        hgt: height,
        wid: width,
        pixels: vec![CustomColor {r: 127, g: 127, b: 127,}; height*width],
    }
}

fn display(scr: Screen) {
    for y in 0..scr.hgt {
        for x in 0..scr.wid {
            print!("{}", "██".custom_color(scr.pixels[y * scr.wid + x]));
        }
        println!();
    }
}

fn read_obj(path: &str) -> Vec<Triangle> {
//turn .obj file into a list of triangles
/*
For each line in the obj file:
    If it's a v (vertex) line:
        Grab the text from the file and parse it to f32s.
        Create a Vector3 with those 3 new numbers.
        Add it to the VERTICES vector.
        
    If it's a vn (normal vector) line:
        Grab the text from the file and parse it to f32s.
        Create a Vector3 with those 3 new numbers.
        Add it to the NORMALS vector.

    If it's a f (face) line:
        Grab the first character that comes after every space (it'll always be an index to a vertex)
        If it's a triangle (3 indices):
            Create a Triangle using the vertices referred to by the aforementioned indices.
            Give it a normal vector by using "nv_index" as an index.
            Append to triangles vector. (the list of triangles)
            nv_index += 1.
        If it's a square (4 indices):
            Create two triangles using the vertices with indices #0 #1 #2 and #2 #3 #0
            and apply same normal to both with "nv_index".
            Append both to triangles vector.
            nv_index += 1.
        Otherwise return nothing.

Return the triangles vector.
*/
    let mut vertices:  Vec<Vector3>  = Vec::new();
    let mut normals:   Vec<Vector3>  = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut nv_index: usize = 0; //(normal vector index) +1 is added every newly made triangle (MORE IMPORTANTLY when two triangles are made from 1 square (they are given the same normal)). This makes sure normal vectors get assigned properly. note: this might be a little messy, but i hope it works for models with both triangles and squares.

    for line in read_to_string(path).unwrap().lines() {
        //find a vertex line (v ), add it to vertices Vec
        if line.chars().nth(0).unwrap() == 'v' && line.chars().nth(1).unwrap() == ' ' {
            //create a vector with the index of each space
            let mut spaces: Vec<usize> = Vec::new();
            for (i, item) in line.as_bytes().iter().enumerate() {
                if item == &b' ' {
                    spaces.push(i);
                }
            }
            let string1: String = line[spaces[0]..spaces[1]].to_string();
            let string2: String = line[spaces[1]..spaces[2]].to_string();
            let string3: String = line[spaces[2]..].to_string();
            let vertex = Vector3 {
                x: string1.trim().parse().unwrap_or(0.0),
                y: string2.trim().parse().unwrap_or(0.0),
                z: string3.trim().parse().unwrap_or(0.0),
            };
            vertices.push(vertex);
        }

        //find a normal vector line (vn), add it to normals Vec
        if line.chars().nth(0).unwrap() == 'v' && line.chars().nth(1).unwrap() == 'n' {
            //create a vector with the index of each space
            let mut spaces: Vec<usize> = Vec::new();
            for (i, item) in line.as_bytes().iter().enumerate() {
                if item == &b' ' {
                    spaces.push(i);
                }
            }
            let string1: String = line[spaces[0]..spaces[1]].to_string();
            let string2: String = line[spaces[1]..spaces[2]].to_string();
            let string3: String = line[spaces[2]..].to_string();
            //create the normal
            let normal = Vector3 {
                x: string1.trim().parse().unwrap_or(0.0),
                y: string2.trim().parse().unwrap_or(0.0),
                z: string3.trim().parse().unwrap_or(0.0),
            };
            normals.push(normal);
        }

        //CREATE TRIANGLES
        //find a face line (f )
        if line.chars().nth(0).unwrap() == 'f' && line.chars().nth(1).unwrap() == ' ' {
            //create a vector with the index of each space
            let mut spaces: Vec<usize> = Vec::new();
            for (i, item) in line.as_bytes().iter().enumerate() {
                if item == &b' ' {
                    spaces.push(i);
                }
            }
            //handle triangles + apply nv
            if spaces.len() == 3 {
                let mut indices: Vec<usize> = Vec::new();
                for i in 0..3 {
                    indices.push(
                        line[spaces[i] + 1..spaces[i] + 2]
                            .to_string()
                            .trim()
                            .parse()
                            .unwrap_or(0),
                    );
                }
                //simple as hek, we just make the triangle.
                triangles.push(Triangle {
                    a: vertices[indices[0]-1].clone(),
                    b: vertices[indices[1]-1].clone(),
                    c: vertices[indices[2]-1].clone(),
                    n: normals[nv_index].clone(),
                });
                nv_index += 1;
            }
            //handle squares + apply nv
            else if spaces.len() == 4 {
                let mut indices: Vec<usize> = Vec::new();
                for i in 0..4 {
                    indices.push(
                        line[spaces[i] + 1..spaces[i] + 2]
                            .to_string()
                            .trim()
                            .parse()
                            .unwrap_or(0),
                    );
                }
                //we turn the square into 2 triangles
                /*
                SQUARE     =>      triangles
                 1—-2           1—-2          2
                 | /|   ====>   | /    +    / |
                 4—-3           4          4—-3
                1-2-3-4    =>   1-2-4  +  2-3-4
                this representation should be counter-clockwise but idc
                */
                
                triangles.push(Triangle {
                    a: vertices[indices[0]-1].clone(),
                    b: vertices[indices[1]-1].clone(),
                    c: vertices[indices[3]-1].clone(),
                    n: normals[nv_index].clone(),
                });
                triangles.push(Triangle {
                    a: vertices[indices[1]-1].clone(),
                    b: vertices[indices[2]-1].clone(),
                    c: vertices[indices[3]-1].clone(),
                    n: normals[nv_index].clone(),
                });
                nv_index += 1;
            }
            else { println!("What the fuck is a {} sided polygon?", spaces.len()); return Vec::new(); }
        }
    }
    return triangles;
}


struct Screen {
    hgt:    usize,
    wid:    usize,
    pixels: Vec<CustomColor>, // Pixels are just a list of colours that are iterated through using (x + y * screen width) as an index
} 

#[derive(Clone, Debug)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Debug)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Clone, Debug)]
struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
    n: Vector3, //normal vector
}

#[derive(Clone, Debug)]
struct Triangle2D {
    a:   Vector2,
    b:   Vector2,
    c:   Vector2,
    col: CustomColor,
}