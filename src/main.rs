use colored::Colorize;
use colored::customcolors::CustomColor;

use std::fs::read_to_string;

fn main() {
    let file_path: &str = "src/cube.obj";
    let mut screen = new_screen(24,24);
    draw(screen);
}


struct Screen {
    hgt: usize,
    wid: usize,
    pixels: Vec<CustomColor>,
}
//creates a blank screen
fn new_screen(height: usize, width: usize) -> Screen {
    return Screen {
        hgt: height,
        wid: width,
        pixels: vec![CustomColor {r: 255, g: 127, b: 255,}; height*width],
    }
}
fn draw(scr: Screen) {
    for y in 0..scr.hgt {
        for x in 0..scr.wid {
            print!("{}", "██".custom_color(scr.pixels[y * scr.hgt + x]));
        }
        println!();
    }
}


#[derive(Clone, Debug)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}


#[derive(Clone, Debug)]
struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
    n: Vector3, //normal vector
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
                    SQUARE     =>     triangles
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
