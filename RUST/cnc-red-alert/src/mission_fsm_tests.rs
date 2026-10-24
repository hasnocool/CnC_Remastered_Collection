#[cfg(test)]
mod behavior {
    use crate::game::{Game, MissionState};
    use cnc_core::{ClassId, Fixed16_16, Vec2i};
    #[test]
    fn mission_state_enum_exists() {
        // TDD: MissionState must have Guard and Attack variants
        let s = MissionState::Guard;
        assert_eq!(format!("{:?}", s), "Guard");
    }

    #[test]
    fn game_unit_has_guard_post() {
        // TDD: GameUnit must carry guard_post
        let mut game = Game::new(40, 40);
        let id = game.spawn_unit(0, ClassId::InfantryE1, Vec2i::new(10, 10));
        let u = game.unit(id).expect("unit");
        assert!(u.guard_post.is_none());
    }

    #[test]
    fn guard_unit_returns_to_post_when_path_empty() {
        // TDD: A unit in Guard state with a guard_post and no path should
        // get a path back to its post when it is far enough away.
        let mut game = Game::new(40, 40);
        let id = game.spawn_unit(0, ClassId::InfantryE1, Vec2i::new(10, 10));
        {
            let unit = game.unit_mut(id).expect("unit");
            unit.mission = MissionState::Guard;
            // (20,20) is river on the default skirmish map; (15,15) is land.
            unit.guard_post = Some(Vec2i::new(15, 15));
            unit.position = cnc_core::Vec2::new(
                Fixed16_16::from_f32(5.0 * 24.0),
                Fixed16_16::from_f32(5.0 * 24.0),
            );
            unit.cell = Vec2i::new(5, 5);
            unit.path.clear();
        }

        game.update(Fixed16_16::from_f32(0.5));

        let u = game.unit(id).expect("unit");
        assert!(
            !u.path.is_empty(),
            "guard unit should have a path back to its post"
        );
    }

    #[test]
    fn attack_unit_moves_toward_destination() {
        // TDD: A unit in Attack state with a mission_destination and no path
        // should get a path toward the destination.
        let mut game = Game::new(40, 40);
        let id = game.spawn_unit(0, ClassId::InfantryE1, Vec2i::new(5, 5));
        {
            let unit = game.unit_mut(id).expect("unit");
            unit.mission = MissionState::Attack;
            // (20,20) is river on the default skirmish map; (15,15) is land.
            unit.mission_destination = Some(Vec2i::new(15, 15));
            unit.path.clear();
        }

        game.update(Fixed16_16::from_f32(0.5));

        let u = game.unit(id).expect("unit");
        assert!(
            !u.path.is_empty(),
            "attack unit should have a path toward its destination"
        );
    }

    #[test]
    fn idle_unit_does_not_get_a_path() {
        // TDD: An Idle unit with no path should stay without a path.
        let mut game = Game::new(40, 40);
        let id = game.spawn_unit(0, ClassId::InfantryE1, Vec2i::new(5, 5));
        {
            let unit = game.unit_mut(id).expect("unit");
            unit.mission = MissionState::Idle;
            unit.path.clear();
        }

        game.update(Fixed16_16::from_f32(0.5));

        let u = game.unit(id).expect("unit");
        assert!(u.path.is_empty(), "idle unit should not get a path");
    }
}
