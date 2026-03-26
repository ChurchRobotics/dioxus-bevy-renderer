use bevy::{
    ecs::{
        component::Component,
        entity::{Entity, EntityHashSet},
        hierarchy::ChildOf,
        message::{Message, MessageWriter, Messages, MessageCursor},
        resource::Resource,
        system::{Local, Query},
        world::World,
    },
    prelude::EntityWorldMut,
    ui::RelativeCursorPosition,
};
use bevy::picking::events::{Click, Out, Over, Pointer, Press, Release};
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

#[derive(Resource, Default)]
pub struct EventReaders {
    click: MessageCursor<Pointer<Click>>,
    click_down: MessageCursor<Pointer<Press>>,
    click_up: MessageCursor<Pointer<Release>>,
    mouse_over: MessageCursor<Pointer<Over>>,
    mouse_out: MessageCursor<Pointer<Out>>,
    mouse_enter: MessageCursor<MouseEnter>,
    mouse_exit: MessageCursor<MouseExit>,
}

impl EventReaders {
    #[allow(clippy::too_many_arguments)]
    pub fn read_events(
        &mut self,
        click: &Messages<Pointer<Click>>,
        click_down: &Messages<Pointer<Press>>,
        click_up: &Messages<Pointer<Release>>,
        mouse_over: &Messages<Pointer<Over>>,
        mouse_out: &Messages<Pointer<Out>>,
        mouse_enter: &Messages<MouseEnter>,
        mouse_exit: &Messages<MouseExit>,
    ) -> Vec<(Entity, &'static str, Rc<dyn Any>, bool)> {
        let mut events: Vec<(Entity, &'static str, Rc<dyn Any>, bool)> = Vec::new();
        for event in self.click.read(click) {
            events.push((event.entity, "click", Rc::new(event.event.button), true));
        }
        for event in self.click_down.read(click_down) {
            events.push((event.entity, "click_down", Rc::new(event.event.button), true));
        }
        for event in self.click_up.read(click_up) {
            events.push((event.entity, "click_up", Rc::new(event.event.button), true));
        }
        for event in self.mouse_over.read(mouse_over) {
            events.push((event.entity, "mouse_over", Rc::new(()), false));
        }
        for event in self.mouse_out.read(mouse_out) {
            events.push((event.entity, "mouse_out", Rc::new(()), false));
        }
        for event in self.mouse_enter.read(mouse_enter) {
            events.push((event.target, "mouse_enter", Rc::new(()), false));
        }
        for event in self.mouse_exit.read(mouse_exit) {
            events.push((event.target, "mouse_exit", Rc::new(()), false));
        }
        events
    }
}

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
    mut enter: MessageWriter<MouseEnter>,
    mut leave: MessageWriter<MouseExit>,
) {
    mem::swap::<EntityHashSet>(&mut previous_over, &mut over);

    over.clear();
    for (entity, relative_cursor_position) in &entities {
        if relative_cursor_position.cursor_over() {
            over.insert(entity);
        }
    }

    enter.write_batch(
        over.iter()
            .copied()
            .filter(|entity| !previous_over.contains(entity))
            .map(|target| MouseEnter { target }),
    );

    leave.write_batch(
        previous_over
            .iter()
            .copied()
            .filter(|entity| !over.contains(entity))
            .map(|target| MouseExit { target }),
    );
}

#[derive(Message)]
pub struct MouseEnter {
    target: Entity,
}

#[derive(Message)]
pub struct MouseExit {
    target: Entity,
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
