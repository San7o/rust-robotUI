use bevy::prelude::*;
use crate::UI::components::*;
use crate::UI::styles::*;
use crate::WorldRes;
use robotics_lib::runner::Runner;

pub fn spawn_ui_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    runner_res: NonSendMut<Runner>,
    world_res: Res<WorldRes>,
) {
    let ui_entity = build_ui(&mut commands, &asset_server, &runner_res, &world_res);
}

pub fn despawn_ui_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<UIDraw>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    runner_res: &NonSendMut<Runner>,
    world: &Res<WorldRes>,
) -> Entity {

    let main_menu_entity = commands.spawn(
        
         NodeBundle {
            style: Style{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_self: AlignSelf::End,
                align_items: AlignItems::Center,
                column_gap: Val::Px(8.0),
                ..default()
            },
            //background_color: NORMAL_BUTTON_COLOR.into(),
            ..default()
        }
    ).with_children( |parent| {

        //
        // INFO UI ----------------------
        //
        parent.spawn(
            (
            NodeBundle {
                style: Style{
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_self: AlignSelf::Start,
                    align_items: AlignItems::Start,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                // background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            UIDraw{},
            )
        )
        .with_children(|parent| {
            // Score 
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Score: ",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                ScoreText{},  
            ));

            // Score 
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "X: ",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                XText{},
            ));

            // Score 
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Y: ",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                YText{},
            ));

            // Score 
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Elevation: ",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                ElevationText{},
            ));
            
            // Score 
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Energy: ",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                EnergyText{},
            ));
        });

        //
        // SPEED UI ------------------------------
        //
        parent.spawn(
            (
                NodeBundle {
                    style: Style{
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_self: AlignSelf::Start,
                        align_items: AlignItems::End,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    // background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                TopLeftUI{},
            )
        )
        .with_children(|parent| {
            // Pause Button
            parent.spawn(
                (
                    ButtonBundle{
                        style: Style{
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Start,
                        align_self: AlignSelf::End,
                        align_items: AlignItems::End,
                        column_gap: Val::Px(8.0),
                        ..default()
                    },

                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PauseButton{},
                )
            )
            .with_children(|parent| {
                // Text 
                parent.spawn(
                    ImageBundle {
                        style: Style {
                            width: Val::Px(40.0),
                            height: Val::Px(40.0),
                            ..default()
                        },
                        image: asset_server.load("robot_pixelart.png").into(),
                        ..default()
                    }
                );
            });

        });

    })
    .id(); 

    main_menu_entity
}

pub fn update_bottom_bar(
    runner_res: NonSendMut<Runner>,
    world: Res<WorldRes>,
    mut set: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<XText>>,
        Query<&mut Text, With<YText>>,
        Query<&mut Text, With<ElevationText>>,
        Query<&mut Text, With<EnergyText>>,
    )>,
) {
    let robot = runner_res.get_robot();
    let score = world.score;
    let x = world.player_x;
    let y = world.player_y;
    let energy = robot.get_energy().get_energy_level();
    let elevation = world.elevation;

    if let Ok(mut old_text) = set.p0().get_single_mut() {
        old_text.sections[0].value = format!("Score: {}", score);
    }
    if let Ok(mut old_text) = set.p1().get_single_mut() {
        old_text.sections[0].value = format!("X: {}", x);
    }
   if let Ok(mut old_text) = set.p2().get_single_mut() {
        old_text.sections[0].value = format!("Y: {}", y);
    }
   if let Ok(mut old_text) = set.p3().get_single_mut() {
        old_text.sections[0].value = format!("Elevation: {}", elevation);
    }
   if let Ok(mut old_text) = set.p4().get_single_mut() {
        old_text.sections[0].value = format!("Energy: {}", energy);
    }
}


pub fn build_top_left_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {

    let top_left_entity = commands.spawn(
        (
            NodeBundle {
                style: Style{
                    width: Val::Percent(100.0),
                    height: Val::Px(25.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_self: AlignSelf::Start,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                },
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            TopLeftUI{},
        )
    )
    .with_children(|parent| {
        // Pause Button
        parent.spawn(
            (
                ButtonBundle{
                    style: BUTTON_STYLE,
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                },
                PauseButton{},
            )
        )
        .with_children(|parent| {
            // Text 
            parent.spawn(
                TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection::new(
                                "Pause",
                                get_button_text_style(asset_server),
                            )
                        ],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
            );
        });


    })
    .id(); 

    top_left_entity
}
