mod apply_mutations;
pub mod colors;
mod deferred_system;
mod ecs_hooks;
mod elements;
#[macro_use]
mod events;
#[cfg(feature = "hot_reload")]
mod hot_reload;
mod parse_attributes;
mod tick;

use self::{
    apply_mutations::BevyTemplate,
    deferred_system::DeferredSystemRunQueue,
    ecs_hooks::EcsSubscriptions,
    events::{
        generate_mouse_enter_leave_events,
        on_pointer_click, on_pointer_out, on_pointer_over, on_pointer_press, on_pointer_release,
        PendingUiEvents,
    },
    tick::tick_dioxus_ui,
};
use bevy::{
    app::{App, Last, Plugin, PreUpdate},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::{Entity, EntityHashMap},
    },
    prelude::Deref,
    ui::Node,
};
use std::collections::HashMap;
use dioxus::dioxus_core::{Element, ElementId, VirtualDom};

pub mod prelude {
    pub use super::deferred_system::use_system_scheduler;
    pub use super::ecs_hooks::{
        use_query,
        use_query_filtered,
        use_resource,
        use_world,
        // use_event_reader, TODO
    };
    pub use super::elements::*;
    pub use super::{DioxusUiBundle, DioxusUiPlugin, DioxusUiRoot};
    pub use bevy::picking::pointer::PointerButton;
    pub use dioxus::prelude::{Event as DioxusEvent};
}

pub struct DioxusUiPlugin;

impl Plugin for DioxusUiPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "hot_reload")]
        dioxus_hot_reload::hot_reload_init!(dioxus_hot_reload::Config::<
            hot_reload::HotReloadContext,
        >::default());

        app.init_non_send_resource::<UiContext>()
            .init_non_send_resource::<PendingUiEvents>()
            .init_resource::<DeferredSystemRunQueue>()
            .add_observer(on_pointer_click)
            .add_observer(on_pointer_press)
            .add_observer(on_pointer_release)
            .add_observer(on_pointer_over)
            .add_observer(on_pointer_out)
            .add_systems(
                PreUpdate,
                generate_mouse_enter_leave_events,
            )
            .add_systems(Last, tick_dioxus_ui);
    }
}

#[derive(Bundle)]
pub struct DioxusUiBundle {
    pub dioxus_ui_root: DioxusUiRoot,
    pub node: Node,
}

#[derive(Component, Deref, Hash, Eq, Clone, Copy)]
pub struct DioxusUiRoot(pub fn() -> Element);

impl PartialEq for DioxusUiRoot {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.0, other.0)
    }
}

#[derive(Default)]
struct UiContext {
    roots: HashMap<(Entity, DioxusUiRoot), UiRoot>,
    subscriptions: EcsSubscriptions,
}

struct UiRoot {
    virtual_dom: VirtualDom,
    element_id_to_bevy_ui_entity: HashMap<ElementId, Entity>,
    bevy_ui_entity_to_element_id: EntityHashMap<ElementId>,
    templates: HashMap<usize, BevyTemplate>,
    needs_rebuild: bool,
}

impl UiRoot {
    fn new(root_component: DioxusUiRoot) -> Self {
        Self {
            virtual_dom: VirtualDom::new(root_component.0),
            element_id_to_bevy_ui_entity: HashMap::new(),
            bevy_ui_entity_to_element_id: EntityHashMap::default(),
            templates: HashMap::new(),
            needs_rebuild: true,
        }
    }
}
