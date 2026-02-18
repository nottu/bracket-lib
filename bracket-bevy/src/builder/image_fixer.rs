use bevy::{image::ImageSampler, prelude::*};

#[derive(Resource)]
pub(crate) struct ImagesToLoad(pub(crate) Vec<UntypedHandle>);

pub(crate) fn fix_images(mut fonts: ResMut<ImagesToLoad>, mut images: ResMut<Assets<Image>>) {
    if fonts.0.is_empty() {
        return;
    }

    let mut to_remove = Vec::new();
    for (id, img) in images.iter_mut() {
        if let Some(i) = fonts
            .0
            .iter()
            .enumerate()
            .find(|(_i, h)| h.id() == id.untyped())
        {
            img.sampler = ImageSampler::nearest();
            to_remove.push(i.0);
        }
    }
    // Remove in reverse order to preserve indices
    to_remove.sort_unstable();
    for i in to_remove.into_iter().rev() {
        fonts.0.remove(i);
    }
}
