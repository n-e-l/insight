use ash::vk;
use ash::vk::{AttachmentLoadOp, AttachmentStoreOp, ClearColorValue, ClearValue, ImageLayout, Offset2D, Rect2D, RenderingAttachmentInfo};
use cen::vulkan::{CommandBuffer, Image};

pub struct PassParameters<'a> {
    pub(crate) command_buffer: &'a mut CommandBuffer,
    pub(crate) output_image: &'a Image,
}

pub struct GraphicsPass {
    // What data do I need?
    // output color, render settings,
    // shaders
    // No objects themselves, no image outputs, no buffers
    // This is more like a function
}

impl Default for GraphicsPass {
    fn default() -> Self {
        Self {
        }
    }
}

impl GraphicsPass {
    pub fn render(&mut self, params: PassParameters) {
        let color_attachments = vec![
            RenderingAttachmentInfo::default()
                .image_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(AttachmentLoadOp::LOAD)
                .store_op(AttachmentStoreOp::STORE)
                .clear_value(ClearValue { color: ClearColorValue { float32: [1f32, 0f32, 1f32, 1f32] } })
                .image_view(params.output_image.image_view())
        ];
        let rendering_info = vk::RenderingInfoKHR::default()
            .render_area(Rect2D { offset: Offset2D { x: 0, y: 0 }, extent: params.output_image.extent() })
            .layer_count(1)
            .view_mask(0)
            .color_attachments(&color_attachments);
        params.command_buffer.begin_rendering(&rendering_info);
        params.command_buffer.end_rendering();
    }
}