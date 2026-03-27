use bevy::{prelude::*, reflect::TypeInfo, winit::WinitSettings};

use dioxus::prelude::*;
use dioxus_bevy::{
    colors::*,
    prelude::{use_resource, *},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DioxusUiPlugin))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(DioxusUiBundle {
                dioxus_ui_root: DioxusUiRoot(MissionControl),
                node: Node::default(),
            });
            commands.spawn((Camera2d::default(), Name::new("Camera")));
            commands.spawn(Name::new("Player"));
            commands.spawn(Name::new("Enemy"));
            commands.spawn(Name::new("Terrain"));
        })
        .run();
}

// Root

#[component]
fn MissionControl() -> Element {
    let selected_tab = use_signal(|| 0usize);
    let selected_entity = use_signal_sync(|| Option::<Entity>::None);

    rsx! {
        node {
            width: "100vw",
            height: "100vh",
            Sidebar { selected_tab }
            node {
                flex_grow: "1",
                flex_direction: "column",
                background_color: SLATE_950,
                padding: "24",
                if selected_tab() == 0 {
                    OverviewPanel {}
                } else if selected_tab() == 1 {
                    EntitiesPanel { selected_entity, selected_tab }
                } else if selected_tab() == 2 {
                    InspectorPanel { selected_entity }
                } else {
                    SettingsPanel {}
                }
            }
        }
    }
}

// Sidebar

const TABS: [&str; 4] = ["Overview", "Entities", "Inspector", "Settings"];

#[component]
fn Sidebar(selected_tab: Signal<usize>) -> Element {
    rsx! {
        node {
            flex_direction: "column",
            width: "220",
            background_color: SLATE_900,
            node {
                padding: "20",
                padding_bottom: "12",
                text { text: "= Mission Control", text_size: "18", text_color: INDIGO_400 }
            }
            node { height: "1", background_color: SLATE_700, margin_bottom: "8" }
            for (i, &label) in TABS.iter().enumerate() {
                NavItem {
                    label,
                    selected: selected_tab() == i,
                    onclick: move |_| selected_tab.set(i),
                }
            }
        }
    }
}

// Panels

#[component]
fn OverviewPanel() -> Element {
    let mut eq = use_query_filtered::<(Entity, Option<&Name>), Without<Node>>();
    let entities_q = eq.query();
    let entity_count = entities_q.iter().count();
    let mut entities: Vec<_> = entities_q.into_iter().collect();
    entities.sort_by_key(|(e, _)| *e);

    rsx! {
        node {
            flex_direction: "column",
            row_gap: "16",
            text { text: "Dashboard Overview", text_size: "26", text_color: SLATE_100 }
            node {
                column_gap: "12",
                StatCard { icon: "Entities", value: entity_count.to_string(), label: "Live" }
                StatCard { icon: "Systems", value: "24".to_owned(), label: "Active" }
                StatCard { icon: "Components", value: "142".to_owned(), label: "Registered" }
                StatCard { icon: "FPS", value: "60".to_owned(), label: "Frames/sec" }
            }
            text { text: "Active Entities", text_size: "18", text_color: SLATE_300 }
            node {
                flex_direction: "column",
                row_gap: "4",
                if entities.is_empty() {
                    node {
                        padding: "12",
                        text { text: "No entities in scene", text_color: SLATE_500, text_size: "13" }
                    }
                } else {
                    for (entity, name) in entities {
                        node {
                            padding: "8",
                            background_color: SLATE_800,
                            border_width: "1",
                            border_color: SLATE_700,
                            text {
                                text: match name {
                                    Some(n) => format!("{n}"),
                                    None => format!("Entity ({:?})", entity),
                                },
                                text_color: SLATE_300,
                                text_size: "13",
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EntitiesPanel(
    selected_entity: Signal<Option<Entity>, SyncStorage>,
    selected_tab: Signal<usize>,
) -> Element {
    let system_scheduler = use_system_scheduler();
    let mut eq = use_query_filtered::<(Entity, Option<&Name>), Without<Node>>();
    let entities_q = eq.query();
    let mut entities: Vec<_> = entities_q.into_iter().collect();
    entities.sort_by_key(|(e, _)| *e);

    rsx! {
        node {
            flex_direction: "column",
            row_gap: "8",
            text { text: "Entities", text_size: "26", text_color: SLATE_100 }
            node {
                flex_direction: "column",
                flex_grow: "1",
                row_gap: "4",
                if entities.is_empty() {
                    node {
                        padding: "16",
                        text {
                            text: "No entities. Spawn one below.",
                            text_color: SLATE_500,
                            text_size: "14",
                        }
                    }
                } else {
                    for (entity, name) in entities {
                        Button {
                            onclick: move |event: DioxusEvent<PointerButton>| {
                                if *event.data == PointerButton::Primary {
                                    selected_entity.set(Some(entity));
                                    selected_tab.set(2);
                                    event.stop_propagation();
                                }
                            },
                            base_color: if selected_entity() == Some(entity) {
                                Some(INDIGO_600.to_owned())
                            } else {
                                None
                            },
                            hover_color: if selected_entity() == Some(entity) {
                                Some(INDIGO_500.to_owned())
                            } else {
                                Some(SLATE_700.to_owned())
                            },
                            click_color: if selected_entity() == Some(entity) {
                                Some(INDIGO_400.to_owned())
                            } else {
                                Some(SLATE_600.to_owned())
                            },
                            match name {
                                Some(n) => format!("{n}"),
                                None => format!("Entity ({:?})", entity),
                            }
                        }
                    }
                }
            }
            Button {
                onclick: move |event: DioxusEvent<PointerButton>| {
                    if *event.data == PointerButton::Primary {
                        system_scheduler.schedule(move |world: &mut World| {
                            let e = world.spawn_empty().id();
                            selected_entity.set(Some(e));
                        });
                        event.stop_propagation();
                    }
                },
                base_color: Some(INDIGO_600.to_owned()),
                hover_color: Some(INDIGO_500.to_owned()),
                click_color: Some(INDIGO_400.to_owned()),
                text { text: "Spawn Entity", text_size: "14", text_color: WHITE }
            }
        }
    }
}

#[component]
fn InspectorPanel(selected_entity: Signal<Option<Entity>, SyncStorage>) -> Element {
    let world = use_world();
    let type_registry = use_resource::<AppTypeRegistry>().read();
    let components = selected_entity()
        .and_then(|e| {
            let entity_ref = world.get_entity(e).ok()?;
            let mut comps = entity_ref
                .archetype()
                .components()
                .iter()
                .filter_map(|cid| {
                    let info = world.components().get_info(*cid)?;
                    let type_info = info
                        .type_id()
                        .and_then(|tid| type_registry.get_type_info(tid));
                    let full = info.name();
                    let full_str: &str = &*full;
                    let name = full_str
                        .rsplit_once("::")
                        .map(|(_, s)| s)
                        .unwrap_or(full_str)
                        .to_owned();
                    let krate = full_str
                        .split_once("::")
                        .map(|(s, _)| s)
                        .unwrap_or(full_str)
                        .to_owned();
                    Some((name, krate, type_info))
                })
                .collect::<Vec<_>>();
            comps.sort_by_key(|(name, _, _)| name.clone());
            Some(comps)
        })
        .unwrap_or_default();

    rsx! {
        node {
            flex_direction: "column",
            row_gap: "8",
            text { text: "Inspector", text_size: "26", text_color: SLATE_100 }
            if selected_entity().is_none() {
                node {
                    padding: "16",
                    background_color: SLATE_800,
                    border_width: "1",
                    border_color: SLATE_700,
                    text {
                        text: "Select an entity in the Entities tab",
                        text_color: SLATE_400,
                        text_size: "14",
                    }
                }
            } else {
                node {
                    flex_direction: "column",
                    row_gap: "8",
                    for (name, krate, type_info) in components {
                        node {
                            flex_direction: "column",
                            padding: "12",
                            background_color: SLATE_800,
                            border_width: "1",
                            border_color: SLATE_700,
                            row_gap: "4",
                            node {
                                column_gap: "8",
                                align_items: "baseline",
                                text { text: name, text_size: "16", text_color: SLATE_100 }
                                text { text: krate, text_size: "12", text_color: SLATE_500 }
                            }
                            if let Some(info) = type_info {
                                { component_inspector(info) }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn component_inspector(type_info: &TypeInfo) -> Element {
    rsx! {
        match type_info {
            TypeInfo::Struct(info) => rsx! {
                for field in info.iter() {
                    node {
                        padding_top: "2",
                        text {
                            text: format!("{}: {}", field.name(), field.type_path()),
                            text_size: "12",
                            text_color: SLATE_400,
                        }
                    }
                }
            },
            _ => rsx! {},
        }
    }
}

#[component]
fn SettingsPanel() -> Element {
    let vsync = use_signal(|| true);
    let fullscreen = use_signal(|| false);
    let high_dpi = use_signal(|| true);
    let limit_fps = use_signal(|| false);
    let bg_tasks = use_signal(|| true);
    let debug_mode = use_signal(|| false);
    let show_fps = use_signal(|| true);
    let wireframe = use_signal(|| false);
    let log_events = use_signal(|| false);

    rsx! {
        node {
            flex_direction: "column",
            row_gap: "20",
            text { text: "Settings", text_size: "26", text_color: SLATE_100 }
            SettingsGroup {
                title: "Display",
                Toggle { on: vsync, label: "VSync" }
                Toggle { on: fullscreen, label: "Fullscreen" }
                Toggle { on: high_dpi, label: "High DPI Scaling" }
            }
            SettingsGroup {
                title: "Performance",
                Toggle { on: limit_fps, label: "Limit FPS to 60" }
                Toggle { on: bg_tasks, label: "Background Task Scheduling" }
            }
            SettingsGroup {
                title: "Debug",
                Toggle { on: debug_mode, label: "Debug Mode" }
                Toggle { on: show_fps, label: "Show FPS Counter" }
                Toggle { on: wireframe, label: "Wireframe Overlay" }
                Toggle { on: log_events, label: "Log ECS Events" }
            }
        }
    }
}

// Reusable components

#[component]
fn StatCard(icon: &'static str, value: String, label: &'static str) -> Element {
    rsx! {
        node {
            flex_direction: "column",
            flex_grow: "1",
            padding: "16",
            background_color: SLATE_800,
            border_width_left: "3",
            border_color: INDIGO_500,
            row_gap: "6",
            node {
                column_gap: "8",
                align_items: "center",
                text { text: icon, text_size: "13", text_color: SLATE_400 }
                text { text: value, text_size: "22", text_color: WHITE }
            }
            text { text: label, text_size: "13", text_color: SLATE_400 }
        }
    }
}

#[component]
fn SettingsGroup(title: &'static str, children: Element) -> Element {
    rsx! {
        node {
            flex_direction: "column",
            row_gap: "2",
            node {
                padding_bottom: "8",
                text { text: title, text_size: "15", text_color: SLATE_400 }
            }
            node {
                flex_direction: "column",
                background_color: SLATE_800,
                border_width: "1",
                border_color: SLATE_700,
                { children }
            }
        }
    }
}

#[component]
fn Toggle(on: Signal<bool>, label: &'static str) -> Element {
    rsx! {
        node {
            justify_content: "space_between",
            align_items: "center",
            padding: "12",
            border_width_bottom: "1",
            border_color: SLATE_700,
            text { text: label, text_color: SLATE_300, text_size: "14" }
            node {
                onclick: move |_| on.set(!on()),
                width: "44",
                height: "24",
                background_color: if on() { INDIGO_500 } else { SLATE_600 },
                align_items: "center",
                justify_content: if on() { "flex_end" } else { "flex_start" },
                padding: "2",
                node {
                    width: "20",
                    height: "20",
                    background_color: WHITE,
                }
            }
        }
    }
}

#[allow(non_snake_case)]
fn NavItem(props: NavItemProps) -> Element {
    let mut hovered = use_signal(|| false);
    let bg = if props.selected {
        INDIGO_600
    } else if hovered() {
        SLATE_700
    } else {
        TRANSPARENT
    };

    rsx! {
        node {
            onclick: move |_| props.onclick.call(()),
            onmouse_enter: move |_| hovered.set(true),
            onmouse_exit: move |_| hovered.set(false),
            padding: "12",
            padding_left: "16",
            background_color: bg,
            text {
                text: props.label,
                text_size: "15",
                text_color: if props.selected { INDIGO_400 } else { SLATE_400 },
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct NavItemProps {
    label: &'static str,
    selected: bool,
    onclick: EventHandler<()>,
}

#[allow(non_snake_case)]
fn Button(props: ButtonProps) -> Element {
    let mut clicked = use_signal(|| false);
    let mut hovered = use_signal(|| false);
    let background_color = if clicked() {
        props.click_color.unwrap_or(NEUTRAL_500.to_owned())
    } else if hovered() {
        props.hover_color.unwrap_or(NEUTRAL_600.to_owned())
    } else {
        props.base_color.unwrap_or(NEUTRAL_800.to_owned())
    };

    rsx! {
        node {
            onclick: move |event| props.onclick.call(event),
            onclick_down: move |event| if *event.data == PointerButton::Primary { clicked.set(true) },
            onclick_up: move |event| if *event.data == PointerButton::Primary { clicked.set(false) },
            onmouse_enter: move |_| hovered.set(true),
            onmouse_exit: move |_| { hovered.set(false); clicked.set(false) },
            padding: "8",
            background_color,
            { &props.children }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ButtonProps {
    onclick: EventHandler<DioxusEvent<PointerButton>>,
    base_color: Option<String>,
    click_color: Option<String>,
    hover_color: Option<String>,
    children: Element,
}
