//Program that calculates inertial values of basic shapes for gazebo
//Data for inertial values: https://en.wikipedia.org/wiki/List_of_moments_of_inertia
//Currently supports:
//  Solid Rectangular Prism,
//  Solid Sphere,
//  Solid Cylinder,

use std::fmt::{Display, Formatter};

trait Dimensions {
    fn load_dimensions(&mut self);
}

#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Debug)]
struct Rectangle {
    mass: f32,
    depth: f32,
    width: f32,
    height: f32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_matrix(self.get_ix(), self.get_iy(), self.get_iz()))
    }
}

impl Dimensions for Rectangle {
    fn load_dimensions(&mut self) {
        println!("Solid Rectangle Selected:");
        self.mass = get_input("Please enter mass:");
        self.depth = get_input("Please enter depth: ");
        self.width = get_input("Please enter width: ");
        self.height = get_input("Please enter height: ");
        let _str = format!("{}", &self);
        println!("{}", _str);
    }
}

impl Rectangle {
    fn get_ix(&self) -> f32 {
        (self.mass * (self.height.powf(2.) + self.depth.powf(2.))) / 12.
    }

    fn get_iy(&self) -> f32 {
        (self.mass * (self.width.powf(2.) + self.height.powf(2.))) / 12.
    }

    fn get_iz(&self) -> f32 {
        (self.mass * (self.width.powf(2.) + self.depth.powf(2.))) / 12.
    }
}

#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Debug)]
struct Sphere {
    mass: f32,
    radius: f32,
}

impl Display for Sphere {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_matrix(
            self.get_ixyz(),
            self.get_ixyz(),
            self.get_ixyz(),
        ))
    }
}

impl Dimensions for Sphere {
    fn load_dimensions(&mut self) {
        println!("Solid Sphere Selected:");
        self.mass = get_input("Please enter mass:");
        self.radius = get_input("Please enter radius: ");
        let _str = format!("{}", &self);
        println!("{}", _str);
    }
}

impl Sphere {
    fn get_ixyz(&self) -> f32 {
        0.4 * (self.mass * self.radius.powf(2.))
    }
}

#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Debug)]
struct Cylinder {
    mass: f32,
    radius: f32,
    height: f32,
}

impl Display for Cylinder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format_matrix(
            self.get_ixy(),
            self.get_ixy(),
            self.get_iz(),
        ))
    }
}

impl Dimensions for Cylinder {
    fn load_dimensions(&mut self) {
        println!("Solid Rectangle Selected:");
        self.mass = get_input("Please enter mass:");
        self.radius = get_input("Please enter radius: ");
        self.height = get_input("Please enter height: ");
        let _str = format!("{}", &self);
        println!("{}", _str);
    }
}

impl Cylinder {
    fn get_ixy(&self) -> f32 {
        (self.mass * (3. * self.radius.powf(2.) + self.height.powf(2.))) / 12.
    }

    fn get_iz(&self) -> f32 {
        (self.mass * self.radius.powf(2.)) / 2.
    }
}

pub fn get_input<U: std::str::FromStr>(prompt: &str) -> U {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error! Failed to read line!");
        let input: U = match input.trim().parse::<U>() {
            Ok(parsed) => parsed,
            Err(_) => continue,
        };
        return input;
    }
}

pub fn format_matrix(ix: f32, iy: f32, iz: f32) -> String {
    return format!(
        "Moment of inertia matrix:\n\nixx={} ixy={} ixz={}
iyx={} iyy={} iyz={}
izx={} izy={} izz={}\n",
        ix, 0., 0., 0., iy, 0., 0., 0., iz
    );
}

fn main() {
    //let mut input = String::new();
    let mut quit = false;
    while !quit {
        println!(
            "Please select one of the following shapes:\n
Solid Cuboid: 1
Solid Sphere: 2
Solid Cylinder: 3"
        );
        let input: String = get_input("");
        let mut shape: Box<dyn Dimensions> = match input.as_str().trim() {
            "1" => Box::new(Rectangle::default()),
            "2" => Box::new(Sphere::default()),
            "3" => Box::new(Cylinder::default()),
            _ => continue,
        };
        shape.load_dimensions();
        println!("Feel free to copy/paste this data between the <inertia> tags in your URDF file!");
        println!("Process another? (Y/N)");
        let input: String = get_input("");
        match input.as_str().trim() {
            "y" | "Y" | "Yes" | "yes" => quit = false,
            "n" | "N" | "No" | "no" => quit = true,
            _ => {
                println!("Invalid input! Please enter yes or no(y/n)")
            }
        }
    }
}
