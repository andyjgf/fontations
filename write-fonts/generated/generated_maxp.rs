// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

/// [`maxp`](https://docs.microsoft.com/en-us/typography/opentype/spec/maxp)
#[derive(Clone, Debug)]
pub struct Maxp {
    /// The number of glyphs in the font.
    pub num_glyphs: u16,
    /// Maximum points in a non-composite glyph.
    pub max_points: Option<u16>,
    /// Maximum contours in a non-composite glyph.
    pub max_contours: Option<u16>,
    /// Maximum points in a composite glyph.
    pub max_composite_points: Option<u16>,
    /// Maximum contours in a composite glyph.
    pub max_composite_contours: Option<u16>,
    /// 1 if instructions do not use the twilight zone (Z0), or 2 if
    /// instructions do use Z0; should be set to 2 in most cases.
    pub max_zones: Option<u16>,
    /// Maximum points used in Z0.
    pub max_twilight_points: Option<u16>,
    /// Number of Storage Area locations.
    pub max_storage: Option<u16>,
    /// Number of FDEFs, equal to the highest function number + 1.
    pub max_function_defs: Option<u16>,
    /// Number of IDEFs.
    pub max_instruction_defs: Option<u16>,
    /// Maximum stack depth across Font Program ('fpgm' table), CVT
    /// Program ('prep' table) and all glyph instructions (in the
    /// 'glyf' table).
    pub max_stack_elements: Option<u16>,
    /// Maximum byte count for glyph instructions.
    pub max_size_of_instructions: Option<u16>,
    /// Maximum number of components referenced at “top level” for
    /// any composite glyph.
    pub max_component_elements: Option<u16>,
    /// Maximum levels of recursion; 1 for simple components.
    pub max_component_depth: Option<u16>,
}

impl FontWrite for Maxp {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        let version = self.compute_version() as Version16Dot16;
        version.write_into(writer);
        self.num_glyphs.write_into(writer);
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_points
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_contours
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_composite_points
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_composite_contours
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_zones
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_twilight_points
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_storage
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_function_defs
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_instruction_defs
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_stack_elements
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_size_of_instructions
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_component_elements
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
        version.compatible(Version16Dot16::VERSION_1_0).then(|| {
            self.max_component_depth
                .as_ref()
                .expect("missing versioned field should have failed validation")
                .write_into(writer)
        });
    }
}

impl Validate for Maxp {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Maxp", |ctx| {
            let version: Version16Dot16 = self.compute_version();
            ctx.in_field("max_points", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0) && self.max_points.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_contours", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0) && self.max_contours.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_composite_points", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_composite_points.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_composite_contours", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_composite_contours.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_zones", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0) && self.max_zones.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_twilight_points", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_twilight_points.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_storage", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0) && self.max_storage.is_none() {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_function_defs", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_function_defs.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_instruction_defs", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_instruction_defs.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_stack_elements", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_stack_elements.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_size_of_instructions", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_size_of_instructions.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_component_elements", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_component_elements.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
            ctx.in_field("max_component_depth", |ctx| {
                if version.compatible(Version16Dot16::VERSION_1_0)
                    && self.max_component_depth.is_none()
                {
                    ctx.report(format!("field must be present for version {version}"));
                }
            });
        })
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromObjRef<read_fonts::tables::maxp::Maxp<'a>> for Maxp {
    fn from_obj_ref(obj: &read_fonts::tables::maxp::Maxp<'a>, _: FontData) -> Self {
        Maxp {
            num_glyphs: obj.num_glyphs(),
            max_points: obj.max_points(),
            max_contours: obj.max_contours(),
            max_composite_points: obj.max_composite_points(),
            max_composite_contours: obj.max_composite_contours(),
            max_zones: obj.max_zones(),
            max_twilight_points: obj.max_twilight_points(),
            max_storage: obj.max_storage(),
            max_function_defs: obj.max_function_defs(),
            max_instruction_defs: obj.max_instruction_defs(),
            max_stack_elements: obj.max_stack_elements(),
            max_size_of_instructions: obj.max_size_of_instructions(),
            max_component_elements: obj.max_component_elements(),
            max_component_depth: obj.max_component_depth(),
        }
    }
}

#[cfg(feature = "parsing")]
impl<'a> FromTableRef<read_fonts::tables::maxp::Maxp<'a>> for Maxp {}

#[cfg(feature = "parsing")]
impl<'a> FontRead<'a> for Maxp {
    fn read(data: FontData<'a>) -> Result<Self, ReadError> {
        <read_fonts::tables::maxp::Maxp as FontRead>::read(data).map(|x| x.to_owned_table())
    }
}
