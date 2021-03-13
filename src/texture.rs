use std::ffi::c_void;
use image::GenericImageView;

pub enum WrappingType {
    Repeat,
    MirroredRepeat,
    ClampEdge,
    ClampBorder(Vec<f32>),
}

pub enum FilteringType {
    Nearest,
    Linear,
}

pub struct TextureData {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>
}

pub struct Texture {
    texture: u32
}

impl Texture {
    pub fn new(wrapping_type: WrappingType,
        filtering_type: FilteringType,
        mipmap_filtering_type: FilteringType,
        mipmap_level: i32,
        texture_data: TextureData) -> Self {

        let mut texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                mipmap_level,
                gl::RGB as i32,
                texture_data.width,
                texture_data.height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                texture_data.data.as_ptr() as *const c_void
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            match &wrapping_type {
                WrappingType::Repeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                },
                WrappingType::MirroredRepeat => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);
                },
                WrappingType::ClampEdge => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                },
                WrappingType::ClampBorder(x) => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

                    gl::TexParameterfv(gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, &x[0]);
                }
            }

            match &filtering_type {
                FilteringType::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                },
                FilteringType::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                }
            }

            match &mipmap_filtering_type {
                FilteringType::Linear => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                },
                FilteringType::Nearest => {
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                }
            }
        }

        Texture {
            texture,
        }
    }

    pub fn use_texture(&self, index: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as u32);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    pub fn get_from_location(location: &str) -> TextureData {
        let img = image::open(location).unwrap();

        TextureData {
            width: img.dimensions().0 as i32,
            height: img.dimensions().1 as i32,
            data: img.to_bytes()
        }
    }
}