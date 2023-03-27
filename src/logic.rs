/// Used to determine win state of the players. Returns true if the call is correct.
fn call(dice_pile: &Vec<i32>, dice_count: i32, dice_value: i32) -> bool {
    let count: usize = dice_pile.iter().filter(|&x| *x == dice_value || *x == 1).count();
    return dice_count <= count as i32;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn call_liars(){
        let dp : Vec<i32> = vec![1,2,4,2,6,3,1,6,2,5,2,2,1];
        assert_eq!(call(&dp, 6, 3), false);
        assert_eq!(call(&dp, 4, 3), true);
        assert_eq!(call(&dp, 20, 4), false);
        assert_eq!(call(&dp, 2, 2), true);
        assert_eq!(call(&dp, 4, 6), true);
        assert_eq!(call(&dp, 6, 6), false);
    }

}