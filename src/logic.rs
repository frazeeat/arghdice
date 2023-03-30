use rand::Rng;
struct Player{
    dice: Vec<i32>,
}
struct Game {
    players: Vec<Player>,
    dice_pile: Vec<i32>,
    dice_count: i32,
    dice_value: i32,
}

fn game_factory(number_of_players: i32) -> Game {
        
    let mut players: Vec<Player> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..number_of_players{
        let dice: Vec<i32> = (0..5).map(|_| rng.gen_range(1..=6)).collect();
        let player: Player = Player { dice };
        players.push(player);
    }
    let dice_pile: Vec<i32> =  determine_dice_pile(&players);
    return Game{
        players,
        dice_pile,
        dice_count: 1,
        dice_value: 2
    };
}

fn determine_dice_pile(players: &Vec<Player>) -> Vec<i32> {
    let mut dice_pile: Vec<i32> = Vec::new();
    for player in players{
        dice_pile.extend(&player.dice);
    }

    return dice_pile;
}

impl Game {
    /// Used to determine win state of the players. Returns true if the call is correct.
    fn call(&self) -> bool {
        let count: usize = self.dice_pile.iter().filter(|&x| *x == self.dice_value || *x == 1).count();
        return self.dice_count <= count as i32;
    }

    /// Used when a player would like to raise the current game state. Will return true and change game state if the raise is valid. 
    /// False otherwise.
    fn raise(&mut self, new_dice_count: i32, new_dice_value: i32) -> bool {
        if new_dice_count < self.dice_count || ((new_dice_value <= self.dice_value) && (new_dice_count <= self.dice_count)) {
            return  false;
        } else {
            self.dice_count = new_dice_count;
            self.dice_value = new_dice_value;
            return true;
        }

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn mock_game() -> Game {
        let p1: Player = Player {
            dice: vec![1,2,4],
        };
        let p2: Player = Player {
            dice: vec![2,6,3,1,6],
        }; 
        let p3: Player = Player {
            dice: vec![2,5,2,2,1],
        };
        let dp : Vec<i32> = vec![1,2,4,2,6,3,1,6,2,5,2,2,1];
        let mut g1: Game = Game {
            players: vec![p1,p2,p3],
            dice_pile: dp,
            dice_count: 6,
            dice_value: 3,
        };
        return g1;
    }

    #[test]
    fn call_liars(){
        let mut game = mock_game();
        assert_eq!(game.call(), false);
    }
    #[test]
    fn make_game(){
        let num_play: i32 = 3;
        let mut game : Game = game_factory(num_play);
        assert_eq!(game.players.len(), num_play as usize);
        assert_eq!(game.players[0].dice.len(), 5);
    }

    #[test]
    fn raise_game(){
        let mut game: Game = mock_game();
        assert_eq!(game.raise(7, 4), true);
        assert_eq!(game.raise(7, 4), false);
        assert_eq!(game.raise(6, 2), false);
        assert_eq!(game.raise(8, 2), true);
        assert_eq!(game.raise(8, 3), true);


    }

}