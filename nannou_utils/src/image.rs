// Based on https://github.com/nannou-org/nannou/blob/13835414afcb3853d6567d25bf11ab44c4ec8ce1/examples/vulkan/vk_image.rs

use nannou::{
    image::{ImageBuffer, Pixel},
    prelude::*,
    vk::Format,
};
use std::{cell::RefCell, ops::Deref, sync::Arc};

#[derive(Debug, Default, Clone)]
struct Vertex {
    position: [f32; 2],
}
vk::impl_vertex!(Vertex, position);

pub fn draw_image<P, Container>(img: &ImageBuffer<P, Container>, dpi: f32, app: &App, frame: &Frame)
where
    Container: Deref<Target = [<P as Pixel>::Subpixel]>,
    Container: Clone,
    P: Pixel + 'static,
    <P as Pixel>::Subpixel: 'static,
    <P as Pixel>::Subpixel: Send,
    <P as Pixel>::Subpixel: Sync,
    Format: vulkano::format::AcceptsPixels<<P as Pixel>::Subpixel>,
{
    let device = app.main_window().swapchain().device().clone();

    let vertex_buffer = vk::CpuAccessibleBuffer::from_iter(
        device.clone(),
        vk::BufferUsage::all(),
        [
            Vertex {
                position: [-1.0, -1.0],
            },
            Vertex {
                position: [-1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0],
            },
            Vertex {
                position: [1.0, 1.0],
            },
        ]
        .iter()
        .cloned(),
    )
    .unwrap();

    let vertex_shader = vs::Shader::load(device.clone()).unwrap();
    let fragment_shader = fs::Shader::load(device.clone()).unwrap();

    let render_pass = Arc::new(
        vk::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: nannou::frame::COLOR_FORMAT,
                    samples: app.main_window().msaa_samples(),
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    // Load the image to GPU memory.
    let (texture, _tex_future) = {
        let (width, height) = img.dimensions();
        let image_data = img.clone().into_raw().clone();

        vk::ImmutableImage::from_iter(
            image_data.iter().cloned(),
            vk::image::Dimensions::Dim2d { width, height },
            vk::Format::R8G8B8A8Srgb,
            app.main_window().swapchain_queue().clone(),
        )
        .unwrap()
    };

    let sampler = vk::SamplerBuilder::new().build(device.clone()).unwrap();

    let pipeline: Arc<dyn vk::GraphicsPipelineAbstract + Send + Sync> = Arc::new(
        vk::GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vertex_shader.main_entry_point(), ())
            .triangle_strip()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(vk::Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    let descriptor_set = Arc::new(
        vk::PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_sampled_image(texture.clone(), sampler.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let view_fbo = RefCell::new(ViewFbo::default());

    let [w, h] = frame.swapchain_image().dimensions();
    let viewport = vk::ViewportBuilder::new().build([w as f32 * dpi, h as f32 * dpi]);
    let dynamic_state = vk::DynamicState::default().viewports(vec![viewport]);

    // Update view_fbo in case of resize.
    view_fbo
        .borrow_mut()
        .update(frame, render_pass.clone(), |builder, img| builder.add(img))
        .unwrap();

    let clear_values = vec![[0.0, 1.0, 0.0, 1.0].into()];

    frame
        .add_commands()
        .begin_render_pass(view_fbo.borrow().expect_inner(), clear_values)
        .unwrap()
        .draw(
            pipeline.clone(),
            &dynamic_state,
            vec![vertex_buffer.clone()],
            descriptor_set.clone(),
            (),
        )
        .unwrap()
        .end_render_pass()
        .expect("failed to add `end_render_pass` command");
}

mod vs {
    nannou::vk::shaders::shader! {
    ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 tex_coords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    tex_coords = position + vec2(1);
}"
    }
}

mod fs {
    nannou::vk::shaders::shader! {
    ty: "fragment",
        src: "
#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform sampler2D tex;

void main() {
    f_color = texture(tex, tex_coords);
}"
    }
}
