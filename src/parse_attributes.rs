use bevy::{
    asset::{AssetPath, AssetServer},
    color::{Color, Srgba},
    prelude::{ImageNode, Text, Visibility},
    ui::*,
};

#[allow(clippy::too_many_arguments)]
pub fn set_attribute(
    name: &str,
    value: &str,
    node: &mut Node,
    border_color: &mut BorderColor,
    outline: &mut Outline,
    background_color: &mut BackgroundColor,
    visibility: &mut Visibility,
    z_index: &mut ZIndex,
    text: Option<&mut Text>,
    image: Option<&mut ImageNode>,
    asset_server: &AssetServer,
) {
    #[allow(unused_variables, unreachable_code)]
    match (name, value) {
        ("animate", value) => todo!(),
        ("display", "flex") => node.display = Display::Flex,
        ("display", "grid") => node.display = Display::Grid,
        ("display", "none") => node.display = Display::None,
        ("position", "relative") => node.position_type = PositionType::Relative,
        ("position", "absolute") => node.position_type = PositionType::Absolute,
        ("overflow", "visible") => node.overflow = Overflow::visible(),
        ("overflow", "clip") => node.overflow = Overflow::clip(),
        ("overflow_x", "visible") => node.overflow.x = OverflowAxis::Visible,
        ("overflow_x", "clip") => node.overflow.x = OverflowAxis::Clip,
        ("overflow_y", "visible") => node.overflow.y = OverflowAxis::Visible,
        ("overflow_y", "clip") => node.overflow.y = OverflowAxis::Clip,
        ("left", value) => node.left = parse_val(value),
        ("right", value) => node.right = parse_val(value),
        ("top", value) => node.top = parse_val(value),
        ("bottom", value) => node.bottom = parse_val(value),
        ("width", value) => node.width = parse_val(value),
        ("height", value) => node.height = parse_val(value),
        ("min_width", value) => node.min_width = parse_val(value),
        ("min_height", value) => node.min_height = parse_val(value),
        ("max_width", value) => node.max_width = parse_val(value),
        ("max_height", value) => node.max_height = parse_val(value),
        ("aspect_ratio", "none") => node.aspect_ratio = None,
        ("aspect_ratio", value) => node.aspect_ratio = Some(parse_f32(value)),
        ("align_items", "default") => node.align_items = AlignItems::Default,
        ("align_items", "start") => node.align_items = AlignItems::Start,
        ("align_items", "end") => node.align_items = AlignItems::End,
        ("align_items", "flex_start") => node.align_items = AlignItems::FlexStart,
        ("align_items", "flex_end") => node.align_items = AlignItems::FlexEnd,
        ("align_items", "center") => node.align_items = AlignItems::Center,
        ("align_items", "baseline") => node.align_items = AlignItems::Baseline,
        ("align_items", "stretch") => node.align_items = AlignItems::Stretch,
        ("justify_items", "default") => node.justify_items = JustifyItems::Default,
        ("justify_items", "start") => node.justify_items = JustifyItems::Start,
        ("justify_items", "end") => node.justify_items = JustifyItems::End,
        ("justify_items", "center") => node.justify_items = JustifyItems::Center,
        ("justify_items", "baseline") => node.justify_items = JustifyItems::Baseline,
        ("justify_items", "stretch") => node.justify_items = JustifyItems::Stretch,
        ("align_self", "auto") => node.align_self = AlignSelf::Auto,
        ("align_self", "start") => node.align_self = AlignSelf::Start,
        ("align_self", "end") => node.align_self = AlignSelf::End,
        ("align_self", "flex_start") => node.align_self = AlignSelf::FlexStart,
        ("align_self", "flex_end") => node.align_self = AlignSelf::FlexEnd,
        ("align_self", "center") => node.align_self = AlignSelf::Center,
        ("align_self", "baseline") => node.align_self = AlignSelf::Baseline,
        ("align_self", "stretch") => node.align_self = AlignSelf::Stretch,
        ("justify_self", "auto") => node.justify_self = JustifySelf::Auto,
        ("justify_self", "start") => node.justify_self = JustifySelf::Start,
        ("justify_self", "end") => node.justify_self = JustifySelf::End,
        ("justify_self", "center") => node.justify_self = JustifySelf::Center,
        ("justify_self", "baseline") => node.justify_self = JustifySelf::Baseline,
        ("justify_self", "stretch") => node.justify_self = JustifySelf::Stretch,
        ("align_content", "default") => node.align_content = AlignContent::Default,
        ("align_content", "start") => node.align_content = AlignContent::Start,
        ("align_content", "end") => node.align_content = AlignContent::End,
        ("align_content", "flex_start") => node.align_content = AlignContent::FlexStart,
        ("align_content", "flex_end") => node.align_content = AlignContent::FlexEnd,
        ("align_content", "center") => node.align_content = AlignContent::Center,
        ("align_content", "stretch") => node.align_content = AlignContent::Stretch,
        ("align_content", "space_between") => node.align_content = AlignContent::SpaceBetween,
        ("align_content", "space_evenly") => node.align_content = AlignContent::SpaceEvenly,
        ("align_content", "space_around") => node.align_content = AlignContent::SpaceAround,
        ("justify_content", "default") => node.justify_content = JustifyContent::Default,
        ("justify_content", "start") => node.justify_content = JustifyContent::Start,
        ("justify_content", "end") => node.justify_content = JustifyContent::End,
        ("justify_content", "flex_start") => node.justify_content = JustifyContent::FlexStart,
        ("justify_content", "flex_end") => node.justify_content = JustifyContent::FlexEnd,
        ("justify_content", "center") => node.justify_content = JustifyContent::Center,
        ("justify_content", "stretch") => node.justify_content = JustifyContent::Stretch,
        ("justify_content", "space_between") => {
            node.justify_content = JustifyContent::SpaceBetween;
        }
        ("justify_content", "space_evenly") => node.justify_content = JustifyContent::SpaceEvenly,
        ("justify_content", "space_around") => node.justify_content = JustifyContent::SpaceAround,
        ("margin", value) => node.margin = UiRect::all(parse_val(value)),
        ("margin_left", value) => node.margin.left = parse_val(value),
        ("margin_right", value) => node.margin.right = parse_val(value),
        ("margin_top", value) => node.margin.top = parse_val(value),
        ("margin_bottom", value) => node.margin.bottom = parse_val(value),
        ("padding", value) => node.padding = UiRect::all(parse_val(value)),
        ("padding_left", value) => node.padding.left = parse_val(value),
        ("padding_right", value) => node.padding.right = parse_val(value),
        ("padding_top", value) => node.padding.top = parse_val(value),
        ("padding_bottom", value) => node.padding.bottom = parse_val(value),
        ("border_width", value) => node.border = UiRect::all(parse_val(value)),
        ("border_width_left", value) => node.border.left = parse_val(value),
        ("border_width_right", value) => node.border.right = parse_val(value),
        ("border_width_top", value) => node.border.top = parse_val(value),
        ("border_width_bottom", value) => node.border.bottom = parse_val(value),
        ("border_color", value) => *border_color = BorderColor::all(parse_color(value)),
        ("outline_width", value) => outline.width = parse_val(value),
        ("outline_offset", value) => outline.offset = parse_val(value),
        ("outline_color", value) => outline.color = parse_color(value),
        ("flex_direction", "row") => node.flex_direction = FlexDirection::Row,
        ("flex_direction", "column") => node.flex_direction = FlexDirection::Column,
        ("flex_direction", "row_reverse") => node.flex_direction = FlexDirection::RowReverse,
        ("flex_direction", "column_reverse") => node.flex_direction = FlexDirection::ColumnReverse,
        ("flex_wrap", "no_wrap") => node.flex_wrap = FlexWrap::NoWrap,
        ("flex_wrap", "wrap") => node.flex_wrap = FlexWrap::Wrap,
        ("flex_wrap", "wrap_reverse") => node.flex_wrap = FlexWrap::WrapReverse,
        ("flex_grow", value) => node.flex_grow = parse_f32(value),
        ("flex_shrink", value) => node.flex_shrink = parse_f32(value),
        ("flex_basis", value) => node.flex_basis = parse_val(value),
        ("row_gap", value) => node.row_gap = parse_val(value),
        ("column_gap", value) => node.column_gap = parse_val(value),
        ("grid_auto_flow", "row") => node.grid_auto_flow = GridAutoFlow::Row,
        ("grid_auto_flow", "column") => node.grid_auto_flow = GridAutoFlow::Column,
        ("grid_auto_flow", "row_dense") => node.grid_auto_flow = GridAutoFlow::RowDense,
        ("grid_auto_flow", "column_dense") => node.grid_auto_flow = GridAutoFlow::ColumnDense,
        ("grid_template_rows", value) => {
            node.grid_template_rows = todo!();
        }
        ("grid_template_columns", value) => {
            node.grid_template_columns = todo!();
        }
        ("grid_auto_rows", value) => {
            node.grid_auto_rows = todo!();
        }
        ("grid_auto_columns", value) => {
            node.grid_auto_columns = todo!();
        }
        ("grid_row", value) => {
            node.grid_row = todo!();
        }
        ("grid_column", value) => {
            node.grid_column = todo!();
        }
        ("background_color", value) => background_color.0 = parse_color(value),
        ("translation" | "translation_x" | "translation_y" | "rotation" | "scale" | "scale_x" | "scale_y", _) => {
            // TODO: Transform removed from UI nodes in bevy 0.15; UiTransform not yet implemented
        }
        ("visibility", "inherited") => *visibility = Visibility::Inherited,
        ("visibility", "hidden") => *visibility = Visibility::Hidden,
        ("visibility", "visible") => *visibility = Visibility::Visible,
        ("z_index", value) => match value.split_once(':') {
            Some(("local", value)) => *z_index = ZIndex(parse_i32(value)),
            Some(("global", value)) => {
                // GlobalZIndex is a separate component; treat as local ZIndex for now
                *z_index = ZIndex(parse_i32(value));
            }
            None => *z_index = ZIndex(parse_i32(value)),
            _ => panic!("Encountered invalid bevy_dioxus ZIndex `{value}`."),
        },
        ("text", value) if text.is_some() => text.unwrap().0 = value.to_owned(),
        ("text_direction", _) if text.is_some() => {
            // removed in bevy 0.15; direction is no longer a Node field
        }
        ("text_multiline_justification", _) if text.is_some() => {
            // TODO: use TextLayout component; not accessible here
        }
        ("text_size", _) if text.is_some() => {
            // TODO: requires TextFont component; not accessible here
        }
        ("text_color", _) if text.is_some() => {
            // TODO: requires TextColor component; not accessible here
        }
        ("image_asset_path", value) if image.is_some() => {
            image.unwrap().image = asset_server.load(AssetPath::parse(value));
        }
        _ => panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value}`."),
    }
}

fn parse_color(hex: &str) -> Color {
    Srgba::hex(hex)
        .unwrap_or_else(|_| panic!("Encountered invalid bevy_dioxus Color hex `{hex}`."))
        .into()
}

fn parse_f32(float: &str) -> f32 {
    float
        .parse::<f32>()
        .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus f32 `{val}`."))
}

fn parse_i32(int: &str) -> i32 {
    int.parse::<i32>()
        .unwrap_or_else(|val| panic!("Encountered invalid bevy_dioxus i32 `{val}`."))
}

fn parse_val(val: &str) -> Val {
    if let Ok(val) = val.parse::<f32>() {
        return Val::Px(val);
    }
    if let Some((val, "")) = val.split_once("px") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Px(val);
        }
    }
    if let Some((val, "")) = val.split_once("vw") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vw(val);
        }
    }
    if let Some((val, "")) = val.split_once("vh") {
        if let Ok(val) = val.parse::<f32>() {
            return Val::Vh(val);
        }
    }
    panic!("Encountered invalid bevy_dioxus Val `{val}`.");
}
