use imgui::TextureId;

use crate::rendering::{
    ComponentFactory, Material, MaterialDef, RenderObject, RenderingComponent, Shader, ShaderDef,
    Texture, TextureDef, VertexBuffer, VideoPlayer,
};

use super::{
    material::VitaGLMaterial, render_object::VitaGLRenderObject, shader::VitaGLShader,
    texture::VitaGLTexture,
};

pub struct VitaGLComponentFactory {}

impl ComponentFactory for VitaGLComponentFactory {
    fn create_texture(&self, texture_def: &TextureDef) -> Box<dyn Texture> {
        let rgba_image = texture_def
            .image()
            .unwrap_or_else(|| &TEXTURE_MISSING_IMAGE);
        Box::new(VitaGLTexture::new(
            rgba_image.width(),
            rgba_image.height(),
            &rgba_image,
        ))
    }

    fn create_imgui_texture(
        &self,
        buffer: &[u8],
        row_length: u32,
        width: u32,
        height: u32,
        texture_id: Option<TextureId>,
    ) -> (Box<dyn Texture>, TextureId) {
        let texture = VitaGLTexture::new(width, height, buffer);
        let texture_id = texture.texture_id();
        (Box::new(texture), TextureId::new(texture_id as usize))
    }

    fn create_shader(&self, shader_def: &ShaderDef) -> Box<dyn Shader> {
        Box::new(VitaGLShader::new().unwrap())
    }

    fn create_material(&self, material_def: &MaterialDef) -> Box<dyn Material> {
        Box::new(VitaGLMaterial::new())
    }

    fn create_render_object(
        &self,
        vertices: VertexBuffer,
        indices: Vec<u32>,
        material_def: &MaterialDef,
        host_dynamic: bool,
    ) -> Box<dyn RenderObject> {
        let material = self.create_material(material_def);
        let x = Box::new(VitaGLRenderObject::new().unwrap());
        x
    }

    fn create_rendering_component(
        &self,
        objects: Vec<Box<dyn RenderObject>>,
    ) -> RenderingComponent {
        let mut component = RenderingComponent::new();
        for o in objects {
            component.push_render_object(o);
        }

        component
    }

    fn create_video_player(&self) -> Box<VideoPlayer> {
        Box::new(VideoPlayer::new())
    }
}

impl VitaGLComponentFactory {
    pub fn new() -> Self {
        Self {}
    }
}

lazy_static::lazy_static! {
    static ref TEXTURE_MISSING_IMAGE: image::RgbaImage
         = image::load_from_memory(radiance_assets::TEXTURE_MISSING_TEXTURE_FILE).unwrap().to_rgba8();
}