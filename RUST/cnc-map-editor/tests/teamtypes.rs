//! TeamType authoring: undo/redo and [TeamTypes]/[TaskForces] round-trip.

use cnc_core::{ClassId, TeamMember, TeamObjective, TeamType, Vec2i};
use cnc_map_editor::{AddTeamType, History, MapModel, RemoveTeamType, SetTeamType, TaskForce};

fn model_with_start() -> MapModel {
    let mut model = MapModel::new(16, 16);
    model.player_starts.push(Vec2i::new(0, 0));
    model
}

#[test]
fn teamtype_roundtrips_through_ini_save_and_load() {
    let mut model = model_with_start();
    let tf = TaskForce {
        name: String::new(),
        members: vec![TeamMember {
            class: ClassId::VehicleE2,
            count: 3,
        }],
    };
    let tf_name = model.add_taskforce(tf);
    let mut tt = TeamType::new(0, "AttackTeam", ClassId::HouseSoviet);
    tt.taskforce.name = tf_name.clone();
    tt.taskforce.members = vec![TeamMember {
        class: ClassId::VehicleE2,
        count: 3,
    }];
    tt.objective = TeamObjective::Attack;
    tt.active_at_start = true;
    model.add_team_type(tt.clone());

    let text = model.to_ini();
    assert!(text.contains("[TaskForces]"), "{text}");
    assert!(text.contains("[TeamTypes]"), "{text}");

    let loaded = MapModel::from_ini(&text).expect("reload");
    assert_eq!(loaded.taskforces.len(), 1);
    assert_eq!(loaded.team_types.len(), 1);
    assert_eq!(loaded.team_types[0].name, "AttackTeam");
    assert_eq!(loaded.team_types[0].owner, ClassId::HouseSoviet);
    assert_eq!(loaded.team_types[0].objective, TeamObjective::Attack);
    assert_eq!(
        loaded.team_types[0].taskforce.members,
        vec![TeamMember {
            class: ClassId::VehicleE2,
            count: 3
        }]
    );
    assert!(loaded.validate().is_empty(), "{:?}", loaded.validate());

    let text2 = loaded.to_ini();
    assert_eq!(text, text2);
}

#[test]
fn team_create_is_undoable_in_one_step() {
    let mut model = model_with_start();
    let mut history = History::new(8);
    let tf_name = model.add_taskforce(TaskForce {
        name: String::new(),
        members: vec![TeamMember {
            class: ClassId::VehicleE2,
            count: 1,
        }],
    });
    let mut tt = TeamType::new(0, "T", ClassId::HouseGDI);
    tt.taskforce.name = tf_name;
    tt.taskforce.members = vec![TeamMember {
        class: ClassId::VehicleE2,
        count: 1,
    }];
    let cmd = Box::new(AddTeamType { team_type: tt });
    history.execute(&mut model, cmd);
    assert_eq!(model.team_types.len(), 1);
    history.undo(&mut model);
    assert!(model.team_types.is_empty(), "one undo removes the team");
    history.redo(&mut model);
    assert_eq!(model.team_types.len(), 1);
}

#[test]
fn invalid_house_reference_blocks_save() {
    let mut model = model_with_start();
    let mut tt = TeamType::new(0, "Bad", ClassId::InfantryE1);
    tt.taskforce.members = vec![TeamMember {
        class: ClassId::VehicleE2,
        count: 1,
    }];
    model.add_team_type(tt);
    let issues = model.validate();
    assert!(!issues.is_empty(), "expected validation error");
    assert!(
        issues.iter().any(|i| i.contains("unknown owner house")),
        "{:?}",
        issues
    );
}

#[test]
fn set_team_type_is_undoable() {
    let mut model = model_with_start();
    let mut history = History::new(8);
    let tf_name = model.add_taskforce(TaskForce {
        name: String::new(),
        members: vec![TeamMember {
            class: ClassId::VehicleE2,
            count: 1,
        }],
    });
    let mut tt = TeamType::new(0, "T", ClassId::HouseGDI);
    tt.taskforce.name = tf_name;
    tt.taskforce.members = vec![TeamMember {
        class: ClassId::VehicleE2,
        count: 1,
    }];
    let id = model.add_team_type(tt);
    let mut edited = model.team_type(id).unwrap().clone();
    edited.name = "Renamed".to_string();
    let cmd = SetTeamType::new(&model, edited).unwrap();
    history.execute(&mut model, Box::new(cmd));
    assert_eq!(model.team_type(id).unwrap().name, "Renamed");
    history.undo(&mut model);
    assert_eq!(model.team_type(id).unwrap().name, "T");
}

#[test]
fn remove_team_type_roundtrips() {
    let mut model = model_with_start();
    let mut history = History::new(8);
    let tt = TeamType::new(0, "T", ClassId::HouseGDI);
    let id = model.add_team_type(tt);
    let cmd = RemoveTeamType::new(&model, id).unwrap();
    history.execute(&mut model, Box::new(cmd));
    assert!(model.team_type(id).is_none());
    history.undo(&mut model);
    assert!(model.team_type(id).is_some());
}
