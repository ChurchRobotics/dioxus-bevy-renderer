use crate::{
    events::{insert_event_listener, remove_event_listener},
    parse_attributes::set_attribute,
};
use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        entity::{Entity, EntityHashMap},
        hierarchy::{Children, ChildOf},
        world::World,
    },
    prelude::{default, Text, Visibility},
    ui::{widget::ImageNode, *},
};
use std::collections::HashMap;
use dioxus::dioxus_core::{
    AttributeValue, ElementId, Template, TemplateAttribute, TemplateNode, WriteMutations,
};

pub struct MutationApplier<'a> {
    element_id_to_bevy_ui_entity: &'a mut HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: &'a mut EntityHashMap<ElementId>,
    templates: &'a mut HashMap<usize, BevyTemplate>,
    world: &'a mut World,
    asset_server: &'a AssetServer,
    stack: Vec<Entity>,
}

impl<'a> MutationApplier<'a> {
    pub fn new(
        element_id_to_bevy_ui_entity: &'a mut HashMap<ElementId, Entity>,
        bevy_ui_entity_to_element_id: &'a mut EntityHashMap<ElementId>,
        templates: &'a mut HashMap<usize, BevyTemplate>,
        root_entity: Entity,
        world: &'a mut World,
        asset_server: &'a AssetServer,
    ) -> Self {
        element_id_to_bevy_ui_entity.insert(ElementId(0), root_entity);
        bevy_ui_entity_to_element_id.insert(root_entity, ElementId(0));

        Self {
            element_id_to_bevy_ui_entity,
            bevy_ui_entity_to_element_id,
            templates,
            world,
            asset_server,
            stack: vec![root_entity],
        }
    }
}

impl<'a> WriteMutations for MutationApplier<'a> {
    fn append_children(&mut self, id: ElementId, m: usize) {
        let mut parent = self
            .world
            .entity_mut(self.element_id_to_bevy_ui_entity[&id]);
        for child in self.stack.drain((self.stack.len() - m)..) {
            parent.add_child(child);
        }
    }

    fn assign_node_id(&mut self, path: &'static [u8], id: ElementId) {
        let mut entity = *self.stack.last().unwrap();
        for index in path {
            entity = self.world.entity(entity).get::<Children>().unwrap()[*index as usize];
        }
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
    }

    fn create_placeholder(&mut self, id: ElementId) {
        let entity = self.world.spawn(Node::default()).id();
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn create_text_node(&mut self, value: &str, id: ElementId) {
        let entity =
            BevyTemplateNode::IntrinsicTextNode(Text::new(value.to_owned()))
                .spawn(self.world);
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn load_template(&mut self, template: Template, index: usize, id: ElementId) {
        let template_key = template.roots.as_ptr() as usize;
        if !self.templates.contains_key(&template_key) {
            self.templates.insert(
                template_key,
                BevyTemplate::from_dioxus(&template, self.asset_server),
            );
        }
        let entity = self.templates[&template_key].roots[index].spawn(self.world);
        self.element_id_to_bevy_ui_entity.insert(id, entity);
        self.bevy_ui_entity_to_element_id.insert(entity, id);
        self.stack.push(entity);
    }

    fn replace_node_with(&mut self, id: ElementId, m: usize) {
        let existing = self.element_id_to_bevy_ui_entity[&id];
        let existing_parent = self.world.entity(existing).get::<ChildOf>().unwrap().parent();
        let mut existing_parent = self.world.entity_mut(existing_parent);

        let existing_index = existing_parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        existing_parent
            .insert_children(existing_index, &self.stack.split_off(self.stack.len() - m));

        self.world.entity_mut(existing).despawn();
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&existing) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn replace_placeholder_with_nodes(&mut self, path: &'static [u8], m: usize) {
        let mut existing = self.stack[self.stack.len() - m - 1];
        for index in path {
            existing = self.world.entity(existing).get::<Children>().unwrap()[*index as usize];
        }
        let existing_parent = self.world.entity(existing).get::<ChildOf>().unwrap().parent();
        let mut existing_parent = self.world.entity_mut(existing_parent);

        let existing_index = existing_parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        existing_parent
            .insert_children(existing_index, &self.stack.split_off(self.stack.len() - m));

        self.world.entity_mut(existing).despawn();
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&existing) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn insert_nodes_after(&mut self, id: ElementId, m: usize) {
        let entity = self.element_id_to_bevy_ui_entity[&id];
        let parent = self.world.entity(entity).get::<ChildOf>().unwrap().parent();
        let mut parent = self.world.entity_mut(parent);
        let index = parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == entity)
            .unwrap();
        parent.insert_children(index + 1, &self.stack.split_off(self.stack.len() - m));
    }

    fn insert_nodes_before(&mut self, id: ElementId, m: usize) {
        let existing = self.element_id_to_bevy_ui_entity[&id];
        let parent = self.world.entity(existing).get::<ChildOf>().unwrap().parent();
        let mut parent = self.world.entity_mut(parent);
        let index = parent
            .get::<Children>()
            .unwrap()
            .iter()
            .position(|child| *child == existing)
            .unwrap();
        parent.insert_children(index, &self.stack.split_off(self.stack.len() - m));
    }

    fn set_attribute(
        &mut self,
        name: &'static str,
        _ns: Option<&'static str>,
        value: &AttributeValue,
        id: ElementId,
    ) {
        let value = match value {
            AttributeValue::Text(value) => value,
            AttributeValue::None => todo!("Remove the attribute"),
            value => {
                panic!("Encountered unsupported bevy_dioxus attribute `{name}: {value:?}`.")
            }
        };

        let (
            mut node,
            mut border_color,
            mut outline,
            mut background_color,
            mut visibility,
            mut z_index,
            mut text,
            mut image,
        ) = self
            .world
            .query::<(
                &mut Node,
                &mut BorderColor,
                &mut Outline,
                &mut BackgroundColor,
                &mut Visibility,
                &mut ZIndex,
                Option<&mut Text>,
                Option<&mut ImageNode>,
            )>()
            .get_mut(self.world, self.element_id_to_bevy_ui_entity[&id])
            .unwrap();

        set_attribute(
            name,
            &value,
            &mut node,
            &mut border_color,
            &mut outline,
            &mut background_color,
            &mut visibility,
            &mut z_index,
            text.as_deref_mut(),
            image.as_deref_mut(),
            self.asset_server,
        );
    }

    fn set_node_text(&mut self, value: &str, id: ElementId) {
        self.world
            .entity_mut(self.element_id_to_bevy_ui_entity[&id])
            .insert(Text::new(value.to_owned()));
    }

    fn create_event_listener(&mut self, name: &'static str, id: ElementId) {
        insert_event_listener(
            &name,
            self.world
                .entity_mut(self.element_id_to_bevy_ui_entity[&id]),
        );
    }

    fn remove_event_listener(&mut self, name: &'static str, id: ElementId) {
        remove_event_listener(
            &name,
            self.world
                .entity_mut(self.element_id_to_bevy_ui_entity[&id]),
        );
    }

    fn remove_node(&mut self, id: ElementId) {
        let entity = self.element_id_to_bevy_ui_entity[&id];
        self.world.entity_mut(entity).despawn();
        // TODO: We're not removing child entities from the element maps
        if let Some(existing_element_id) = self.bevy_ui_entity_to_element_id.remove(&entity) {
            self.element_id_to_bevy_ui_entity
                .remove(&existing_element_id);
        }
    }

    fn push_root(&mut self, id: ElementId) {
        self.stack.push(self.element_id_to_bevy_ui_entity[&id]);
    }
}

pub struct BevyTemplate {
    roots: Box<[BevyTemplateNode]>,
}

enum BevyTemplateNode {
    Node {
        style: StyleComponents,
        children: Box<[Self]>,
    },
    TextNode {
        text: Text,
        style: StyleComponents,
        children: Box<[Self]>,
    },
    ImageNode {
        image: ImageNode,
        style: StyleComponents,
        children: Box<[Self]>,
    },
    IntrinsicTextNode(Text),
}

impl BevyTemplate {
    fn from_dioxus(template: &Template, asset_server: &AssetServer) -> Self {
        Self {
            roots: template
                .roots
                .iter()
                .map(|node| BevyTemplateNode::from_dioxus(node, asset_server))
                .collect(),
        }
    }
}

impl BevyTemplateNode {
    fn from_dioxus(node: &TemplateNode, asset_server: &AssetServer) -> Self {
        match node {
            TemplateNode::Element {
                tag: "node",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, _, _) = parse_template_attributes(attrs, Color::srgba(0.0, 0.0, 0.0, 0.0), asset_server);
                Self::Node {
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Element {
                tag: "text",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, text, _) = parse_template_attributes(attrs, Color::srgba(0.0, 0.0, 0.0, 0.0), asset_server);
                Self::TextNode {
                    text,
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Element {
                tag: "image",
                namespace: Some("bevy_ui"),
                attrs,
                children,
            } => {
                let (style, _, image) =
                    parse_template_attributes(attrs, Color::WHITE, asset_server);
                Self::ImageNode {
                    image,
                    style,
                    children: children
                        .iter()
                        .map(|node| Self::from_dioxus(node, asset_server))
                        .collect(),
                }
            }
            TemplateNode::Text { text } => {
                Self::IntrinsicTextNode(Text::new(text.to_string()))
            }
            TemplateNode::Dynamic { id: _ } => Self::Node {
                style: StyleComponents::default(),
                children: Box::new([]),
            },
            TemplateNode::Element {
                tag,
                namespace: None,
                ..
            } => {
                bevy::log::warn!("Unsupported bevy_dioxus tag `{tag}` — rendering placeholder node.");
                Self::Node {
                    style: StyleComponents::default(),
                    children: Box::new([]),
                }
            }
            TemplateNode::Element {
                tag,
                namespace: Some(namespace),
                ..
            } => {
                bevy::log::warn!("Unsupported bevy_dioxus tag `{namespace}::{tag}` — rendering placeholder node.");
                Self::Node {
                    style: StyleComponents::default(),
                    children: Box::new([]),
                }
            }
        }
    }

    fn spawn(&self, world: &mut World) -> Entity {
        match self {
            BevyTemplateNode::Node { style, children } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn((
                        style.node.clone(),
                        style.border_color,
                        style.background_color,
                        style.visibility,
                        style.z_index,
                        style.outline,
                    ))
                    .add_children(&children)
                    .id()
            }
            BevyTemplateNode::TextNode {
                text,
                style,
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn((
                        text.clone(),
                        style.node.clone(),
                        style.border_color,
                        style.background_color,
                        style.visibility,
                        style.z_index,
                        style.outline,
                    ))
                    .add_children(&children)
                    .id()
            }
            BevyTemplateNode::ImageNode {
                image,
                style,
                children,
            } => {
                let children = children
                    .iter()
                    .map(|child| child.spawn(world))
                    .collect::<Box<[_]>>();
                world
                    .spawn((
                        image.clone(),
                        style.node.clone(),
                        style.border_color,
                        style.background_color,
                        style.visibility,
                        style.z_index,
                        style.outline,
                    ))
                    .add_children(&children)
                    .id()
            }
            Self::IntrinsicTextNode(text) => world.spawn(text.clone()).id(),
        }
    }
}

fn parse_template_attributes(
    attributes: &[TemplateAttribute],
    background_color: Color,
    asset_server: &AssetServer,
) -> (StyleComponents, Text, ImageNode) {
    let mut style = StyleComponents {
        background_color: BackgroundColor(background_color),
        ..default()
    };
    let mut text = Text::new("");
    let mut image = ImageNode::default();
    for attribute in attributes {
        if let TemplateAttribute::Static {
            name,
            value,
            namespace: _,
        } = attribute
        {
            set_attribute(
                name,
                value,
                &mut style.node,
                &mut style.border_color,
                &mut style.outline,
                &mut style.background_color,
                &mut style.visibility,
                &mut style.z_index,
                Some(&mut text),
                Some(&mut image),
                asset_server,
            );
        }
    }
    (style, text, image)
}

#[derive(Default)]
struct StyleComponents {
    node: Node,
    border_color: BorderColor,
    outline: Outline,
    background_color: BackgroundColor,
    visibility: Visibility,
    z_index: ZIndex,
}
