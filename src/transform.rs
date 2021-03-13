use cgmath::{Deg, Matrix4, vec3};

pub struct Transform {
    transform: Matrix4<f32>
}

impl Transform {
    pub fn get_matrix(self) -> Matrix4<f32> {
        self.transform.clone()
    }

    pub fn identity() -> Self {
        Transform {
            transform: Matrix4::from_translation(vec3(0.0, 0.0, 0.0))
        }
    }

    pub fn with_position(position: (f32, f32, f32)) -> Self {
        Transform {
            transform: Matrix4::from_translation(vec3(position.0, position.1, position.2))
        }
    }

    pub fn with_rotation(rotation: (f32, f32, f32)) -> Self {
        let mut transform: Matrix4<f32> = Matrix4::from_angle_x(Deg(rotation.0));
        transform = transform * Matrix4::from_angle_y(Deg(rotation.1));
        transform = transform * Matrix4::from_angle_z(Deg(rotation.2));

        Transform {
            transform
        }
    }

    pub fn with_scale(scale: (f32, f32, f32)) -> Self {
        Transform {
            transform: Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2)
        }
    }

    pub fn with_position_and_rotation(position: (f32, f32, f32), rotation: (f32, f32, f32)) -> Self {
        let mut transform: Matrix4<f32> = Matrix4::from_translation(vec3(position.0, position.1, position.2));
        transform = transform * Matrix4::from_angle_x(Deg(rotation.0));
        transform = transform * Matrix4::from_angle_y(Deg(rotation.1));
        transform = transform * Matrix4::from_angle_z(Deg(rotation.2));

        Transform {
            transform
        }
    }

    pub fn with_position_and_scale(position: (f32, f32, f32), scale: (f32, f32, f32)) -> Self {
        let mut transform = Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2);
        transform = transform * Matrix4::from_translation(vec3(position.0, position.1, position.2));

        Transform {
            transform
        }
    }

    pub fn with_rotation_and_scale(rotation: (f32, f32, f32), scale: (f32, f32, f32)) -> Self {
        let mut transform = Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2);
        transform = transform * Matrix4::from_angle_x(Deg(rotation.0));
        transform = transform * Matrix4::from_angle_y(Deg(rotation.1));
        transform = transform * Matrix4::from_angle_z(Deg(rotation.2));

        Transform {
            transform
        }
    }

    pub fn with_position_and_rotation_and_scale(position: (f32, f32, f32),
                                                rotation: (f32, f32, f32),
                                                scale: (f32, f32, f32)) -> Self {

        let mut transform: Matrix4<f32> = Matrix4::from_translation(vec3(position.0, position.1, position.2));
        transform = transform * Matrix4::from_nonuniform_scale(scale.0, scale.1, scale.2);
        transform = transform * Matrix4::from_angle_x(Deg(rotation.0));
        transform = transform * Matrix4::from_angle_y(Deg(rotation.1));
        transform = transform * Matrix4::from_angle_z(Deg(rotation.2));

        
        Transform {
            transform
        }
    }
}