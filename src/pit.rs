use game::{Hazzard, RunResult, State, UpdateResult};
use map::{is_adj, RoomNum};
use message::Warning;

pub struct BottomlessPit {
    pub room: RoomNum
}

impl Hazzard for BottomlessPit {
    fn try_update(&self, s: &State) -> Option<UpdateResult> {
        if s.player == self.room {
            Some(UpdateResult::Death(RunResult::KilledByPit))
        } else {
            None
        }
    }

    fn try_warn(&self, player_room: RoomNum) -> Option<&str> {
        if is_adj(player_room, self.room) {
            Some(Warning::PIT)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod pit_tests {
    use super::*;

    #[test]
    fn can_do_nothing() {
        let pit = BottomlessPit { room: 1 };
        let player_room = 20;
        let update_result = pit.try_update(&State {
            player: player_room,
            ..Default::default()
        });
        assert_eq!(None, update_result);
    }

    #[test]
    fn can_give_warning() {
        let pit = BottomlessPit { room: 1 };
        let player_room = 2;
        assert_eq!(Some(Warning::PIT), pit.try_warn(player_room));
    }

    #[test]
    fn can_kill_player() {
        let pit = BottomlessPit { room: 1 };
        let player_room = 1;
        let update_result = pit.try_update(&State {
            player: player_room,
            ..Default::default()
        });
        let expected = Some(UpdateResult::Death(RunResult::KilledByPit));
        assert_eq!(expected, update_result);
    }
}
