use map::{is_adj, RoomNum};
use message::{Message, Warning};
use dynamic_dispatch::game::{Hazzard, UpdateResult};

pub struct BottomlessPit {
    pub room: RoomNum
}

impl Hazzard for BottomlessPit {
    fn update(&self, player_room: RoomNum) -> Option<UpdateResult> {
        if player_room == self.room {
            let death_msg = format!("{}\n{}", Message::FELL_IN_PIT, Message::LOSE);
            Some(UpdateResult::Death(death_msg.into()))
        } else if is_adj(player_room, self.room) {
            Some(UpdateResult::Warning(Warning::PIT.into()))
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
        let update_result = pit.update(player_room);
        assert_eq!(None, update_result);
    }

    #[test]
    fn can_give_warning() {
        let pit = BottomlessPit { room: 1 };
        let player_room = 2;
        let update_result = pit.update(player_room);
        assert_eq!(
            Some(UpdateResult::Warning(Warning::PIT.into())),
            update_result
        );
    }

    #[test]
    fn can_kill_player() {
        let pit = BottomlessPit { room: 1 };
        let player_room = 1;
        let update_result = pit.update(player_room);

        let death_msg = format!("{}\n{}", Message::FELL_IN_PIT, Message::LOSE);
        assert_eq!(Some(UpdateResult::Death(death_msg.into())), update_result);
    }
}
