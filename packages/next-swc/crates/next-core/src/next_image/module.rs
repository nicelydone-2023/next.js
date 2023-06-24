use anyhow::Result;
use indexmap::indexmap;
use turbo_tasks::Vc;
use turbopack_binding::{
    turbo::tasks::Value,
    turbopack::{
        core::{
            asset::Asset,
            context::AssetContext,
            plugin::CustomModuleType,
            reference_type::{InnerAssets, ReferenceType},
            resolve::ModulePart,
        },
        r#static::StaticModuleAsset,
    },
};

use super::source_asset::StructuredImageSourceAsset;

#[turbo_tasks::value(serialization = "auto_for_input")]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, Hash)]
pub enum BlurPlaceholderMode {
    /// Do not generate a blur placeholder at all.
    None,
    /// Generate a blur placeholder as data url and embed it directly into the
    /// JavaScript code. This needs to compute the blur placeholder eagerly and
    /// has a higher computation overhead.
    DataUrl,
    /// Avoid generating a blur placeholder eagerly and uses `/_next/image`
    /// instead to compute one on demand. This changes the UX slightly (blur
    /// placeholder is shown later than it should be) and should
    /// only be used for development.
    NextImageUrl,
}

/// Module type that analyzes images and offers some meta information like
/// width, height and blur placeholder as export from the module.
#[turbo_tasks::value]
pub struct StructuredImageModuleType {
    pub blur_placeholder_mode: BlurPlaceholderMode,
}

impl StructuredImageModuleType {
    pub(crate) fn create_module(
        source: Vc<Box<dyn Asset>>,
        blur_placeholder_mode: BlurPlaceholderMode,
        context: Vc<Box<dyn AssetContext>>,
    ) -> Vc<Box<dyn Asset>> {
        let static_asset = StaticModuleAsset::new(source, context);
        context.process(
            StructuredImageSourceAsset {
                image: source,
                blur_placeholder_mode,
            }
            .cell()
            .into(),
            Value::new(ReferenceType::Internal(Vc::cell(indexmap!(
                "IMAGE".to_string() => static_asset.into()
            )))),
        )
    }
}

#[turbo_tasks::value_impl]
impl StructuredImageModuleType {
    #[turbo_tasks::function]
    pub fn new(blur_placeholder_mode: Value<BlurPlaceholderMode>) -> Vc<Self> {
        StructuredImageModuleType::cell(StructuredImageModuleType {
            blur_placeholder_mode: blur_placeholder_mode.into_value(),
        })
    }
}

#[turbo_tasks::value_impl]
impl CustomModuleType for StructuredImageModuleType {
    #[turbo_tasks::function]
    fn create_module(
        &self,
        source: Vc<Box<dyn Asset>>,
        context: Vc<Box<dyn AssetContext>>,
        _part: Option<Vc<ModulePart>>,
    ) -> Vc<Box<dyn Asset>> {
        StructuredImageModuleType::create_module(source, self.blur_placeholder_mode, context)
    }
}
