use bevy::{
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        event::EntityEvent,
        hierarchy::ChildOf,
        observer::On,
        system::{Local, NonSendMut, Query},
        world::World,
    },
    picking::events::{Click, Out, Over, Pointer, Press, Release},
    prelude::EntityWorldMut,
    ui::RelativeCursorPosition,
};
use std::{any::Any, mem, rc::Rc};

// TODO: Other events
pub mod events {
    use bevy::picking::pointer::PointerButton;

    super::impl_event! [
        ();
        onmouse_over
        onmouse_out
        onmouse_enter
        onmouse_exit
    ];

    super::impl_event! [
        PointerButton;
        onclick
        onclick_down
        onclick_up
    ];
}

#[derive(Default)]
pub struct PendingUiEvents(pub Vec<(Entity, &'static str, Rc<dyn Any>, bool)>);

pub fn insert_event_listener(name: &str, mut entity: EntityWorldMut<'_>) {
    match name {
        "click" => entity.insert(HasClickEventListener),
        "click_down" => entity.insert(HasClickDownEventListener),
        "click_up" => entity.insert(HasClickUpEventListener),
        "mouse_over" => &mut entity,
        "mouse_out" => &mut entity,
        "mouse_enter" => entity.insert((
            HasMouseEnterEventListener,
            RelativeCursorPosition::default(),
        )),
        "mouse_exit" => {
            entity.insert((HasMouseExitEventListener, RelativeCursorPosition::default()))
        }
        _ => panic!("Encountered unsupported bevy_dioxus event `{name}`."),
    };
}

pub fn remove_event_listener(name: &str, mut entity: EntityWorldMut<'_>) {
    match name {
        "click" => entity.remove::<HasClickEventListener>(),
        "click_down" => entity.remove::<HasClickDownEventListener>(),
        "click_up" => entity.remove::<HasClickUpEventListener>(),
        "mouse_over" => &mut entity,
        "mouse_out" => &mut entity,
        "mouse_enter" => {
            entity.remove::<HasMouseEnterEventListener>();
            if !entity.contains::<HasMouseExitEventListener>() {
                entity.remove::<RelativeCursorPosition>();
            }
            &mut entity
        }
        "mouse_exit" => {
            entity.remove::<HasMouseExitEventListener>();
            if !entity.contains::<HasMouseEnterEventListener>() {
                entity.remove::<RelativeCursorPosition>();
            }
            &mut entity
        }
        _ => unreachable!(),
    };
}

#[derive(Component)]
pub struct HasClickEventListener;

#[derive(Component)]
pub struct HasClickDownEventListener;

#[derive(Component)]
pub struct HasClickUpEventListener;

#[derive(Component)]
pub struct HasMouseEnterEventListener;

#[derive(Component)]
pub struct HasMouseExitEventListener;

// ----------------------------------------------------------------------------

pub fn bubble_event(event_name: &str, target_entity: &mut Entity, world: &World) {
    match event_name {
        "click" => bubble_event_helper::<HasClickEventListener>(target_entity, world),
        "click_down" => bubble_event_helper::<HasClickDownEventListener>(target_entity, world),
        "click_up" => bubble_event_helper::<HasClickUpEventListener>(target_entity, world),
        _ => unreachable!(),
    };
}

fn bubble_event_helper<T: Component>(target_entity: &mut Entity, world: &World) {
    while !world.entity(*target_entity).contains::<T>() {
        *target_entity = match world.entity(*target_entity).get::<ChildOf>() {
            Some(child_of) => child_of.parent(),
            None => return,
        };
    }
}

// ----------------------------------------------------------------------------

pub fn generate_mouse_enter_leave_events(
    entities: Query<(Entity, &RelativeCursorPosition)>,
    mut previous_over: Local<EntityHashSet>,
    mut over: Local<EntityHashSet>,
    mut pending: NonSendMut<PendingUiEvents>,
) {
    mem::swap::<EntityHashSet>(&mut previous_over, &mut over);

    over.clear();
    for (entity, relative_cursor_position) in &entities {
        if relative_cursor_position.cursor_over() {
            over.insert(entity);
        }
    }

    for entity in over.iter().copied().filter(|e| !previous_over.contains(e)) {
        pending.0.push((entity, "mouse_enter", Rc::new(()), false));
    }

    for entity in previous_over.iter().copied().filter(|e| !over.contains(e)) {
        pending.0.push((entity, "mouse_exit", Rc::new(()), false));
    }
}

// Global observers that capture picking EntityEvents and write directly to PendingUiEvents.
// Filtered to `original_event_target()` so propagation doesn't produce duplicate entries.

pub fn on_pointer_click(on: On<Pointer<Click>>, mut pending: NonSendMut<PendingUiEvents>) {
    if on.event_target() == on.original_event_target() {
        pending.0.push((on.event_target(), "click", Rc::new(on.event.button), true));
    }
}

pub fn on_pointer_press(on: On<Pointer<Press>>, mut pending: NonSendMut<PendingUiEvents>) {
    if on.event_target() == on.original_event_target() {
        pending.0.push((on.event_target(), "click_down", Rc::new(on.event.button), true));
    }
}

pub fn on_pointer_release(on: On<Pointer<Release>>, mut pending: NonSendMut<PendingUiEvents>) {
    if on.event_target() == on.original_event_target() {
        pending.0.push((on.event_target(), "click_up", Rc::new(on.event.button), true));
    }
}

pub fn on_pointer_over(on: On<Pointer<Over>>, mut pending: NonSendMut<PendingUiEvents>) {
    if on.event_target() == on.original_event_target() {
        pending.0.push((on.event_target(), "mouse_over", Rc::new(()), false));
    }
}

pub fn on_pointer_out(on: On<Pointer<Out>>, mut pending: NonSendMut<PendingUiEvents>) {
    if on.event_target() == on.original_event_target() {
        pending.0.push((on.event_target(), "mouse_out", Rc::new(()), false));
    }
}

// ----------------------------------------------------------------------------

pub trait EventReturn<P>: Sized {
    fn spawn(self) {}
}

impl EventReturn<()> for () {}

macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident $(: $js_name:literal)?
        )*
    ) => {
        $(
            $( #[$attr] )*
            #[allow(non_camel_case_types)]
            pub struct $name;
            impl $name {
                // dioxus 0.6 rsx! macro calls `::call_with_explicit_closure` when the handler
                // is an explicit closure expression (the common case).
                #[inline]
                pub fn call_with_explicit_closure<E: crate::events::EventReturn<T>, T>(mut _f: impl FnMut(dioxus::dioxus_core::Event<$data>) -> E + 'static) -> dioxus::dioxus_core::Attribute {
                    dioxus::dioxus_core::Attribute::new(
                        crate::events::impl_event!(@name $name $($js_name)?),
                        dioxus::dioxus_core::AttributeValue::listener(move |e: dioxus::dioxus_core::Event<$data>| {
                            _f(e).spawn();
                        }),
                        None,
                        false,
                    ).into()
                }
            }
        )*
    };

    (@name $name:ident $js_name:literal) => {
        $js_name
    };
    (@name $name:ident) => {
        stringify!($name)
    };
}

use impl_event;
