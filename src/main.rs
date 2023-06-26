use hdf5::{File, H5Type, Result};
use ndarray::{arr2, s};

#[derive(H5Type, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Colors {
    R = 1,
    G = 2,
    B = 3,
}

#[derive(H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(H5Type, Clone, PartialEq, Debug)]
#[repr(C)]
pub struct Pixel {
    xy: (i64, i64),
    color: Color,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {r,g,b}
    }
}

impl Pixel {
    pub fn new(x: i64, y: i64, color: Color) -> Self {
        Self { xy: (x,y), color}
    }
}

fn write_hdf5() -> Result<()> {
    use Colors::*;
    let file = hdf5::File::create("pixels.h5")?;
    let group = file.create_group("dir")?;
    let builder = group.new_dataset_builder();
    let ds = builder
        .with_data(&arr2(&[
            [Pixel::new(1,2, Color::new(1,2,3)), Pixel::new(2,3, Color::new(4,5,6))],
            [Pixel::new(3,4,Color::new(7,8,9)), Pixel::new(4,5,Color::new(10,11,12))],
            [Pixel::new(5,6,Color::new(13,14,15)), Pixel::new(6,7,Color::new(16,17,18))],
        ]))
        .create("pixels")?;
    
    let attr = ds.new_attr::<Colors>().shape([3]).create("colors")?;
    attr.write(&[R,G,B])?;
    Ok(())
}

// fn read_hdf5() -> Result<()> {
//     use Color::*;
//     let file = File::open("pixels.h5")?;
//     let ds = file.dataset("dir/pixels")?;
//     assert_eq!(
//         ds.read_slice::<Pixel, _, _>(s![1.., ..])?,
//         arr2(&[
//             [Pixel::new(3,4,G), Pixel::new(4,5,R)],
//             [Pixel::new(5,6,B), Pixel::new(6,7,G)],
//         ])
//     );
//     let attr = ds.attr("colors")?;
//     assert_eq!(attr.read_1d::<Color>()?.as_slice().unwrap(), &[R,G,B]);
//     Ok(())
// }

fn main() -> Result<()> {
    write_hdf5()?;
    // read_hdf5()?;
    Ok(())
}