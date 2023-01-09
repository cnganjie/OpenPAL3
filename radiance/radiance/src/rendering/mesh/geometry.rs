use crate::{
    math::{Vec2, Vec3},
    rendering::{MaterialDef, VertexBuffer, VertexComponents},
};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Option<Vec3>,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Option<Vec3>) -> Self {
        Self { position, normal }
    }
}

pub struct TexCoord {
    pub u: f32,
    pub v: f32,
}

impl TexCoord {
    pub fn new(u: f32, v: f32) -> Self {
        Self { u, v }
    }
}

pub struct Geometry {
    pub material: MaterialDef,
    pub vertices: VertexBuffer,
    pub indices: Vec<u32>,
}

impl Geometry {
    pub fn new(
        vertices: &Vec<Vertex>,
        texcoord: &Vec<Vec<TexCoord>>,
        indices: Vec<u32>,
        material: MaterialDef,
        has_alpha: u32,
    ) -> Self {
        let components = if material.textures().len() == 1 {
            VertexComponents::POSITION | VertexComponents::TEXCOORD
        } else {
            VertexComponents::POSITION | VertexComponents::TEXCOORD | VertexComponents::TEXCOORD2
        };

        let mut buffer = VertexBuffer::new(components, vertices.len());

        for i in 0..vertices.len() {
            let vert = &vertices[i];

            let texcoord1 = if texcoord.len() > 0 {
                Some(Vec2::new(texcoord[0][i].u, texcoord[0][i].v))
            } else {
                None
            };

            let texcoord2 = if texcoord.len() > 1 {
                Some(Vec2::new(texcoord[1][i].u, texcoord[1][i].v))
            } else {
                None
            };

            buffer.set_data(
                i,
                Some(&Vec3::new(
                    vert.position.x,
                    vert.position.y,
                    vert.position.z,
                )),
                vert.normal.as_ref(),
                texcoord1.as_ref(),
                texcoord2.as_ref(),
            );
        }
        Self {
            material,
            vertices: buffer,
            indices,
        }
    }
}