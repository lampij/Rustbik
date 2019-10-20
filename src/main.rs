#![feature(test)]

fn main() {
    let _cube = Cube::new();
}

pub struct Side {
    top_left : u8,
    top_middle: u8,
    top_right: u8,
    mid_left: u8,
    mid_middle: u8,
    mid_right: u8,
    bot_left : u8,
    bot_middle: u8,
    bot_right: u8
}

impl Side {
    pub fn new(val: u8) -> Side{
        Side{
            top_left : val,
            top_middle : val,
            top_right: val,
            mid_left: val,
            mid_middle: val,
            mid_right: val,
            bot_left: val,
            bot_middle: val,
            bot_right: val
        }
    }
}

pub struct Cube {
    front: Side,
    bottom: Side,
    left: Side,
    top: Side,
    back: Side,
    right: Side
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            front: Side::new(0),
            top: Side::new(1),
            back: Side::new(2),
            bottom: Side::new(3),
            left: Side::new(4),
            right: Side::new(5)
        }
    }

    /*Implementation Completed*/
    pub fn left_counterclockwise(&mut self){
        //Swap front left rows with top left rows
        Cube::swap_unit_colors(&mut self.front.top_left, &mut self.top.top_left);
        Cube::swap_unit_colors(&mut self.front.mid_left, &mut self.top.mid_left);
        Cube::swap_unit_colors(&mut self.front.bot_left, &mut self.top.bot_left);

        //Swap front left rows with back right rows
        Cube::swap_unit_colors(&mut self.front.top_left, &mut self.back.bot_right);
        Cube::swap_unit_colors(&mut self.front.mid_left, &mut self.back.mid_right);
        Cube::swap_unit_colors(&mut self.front.bot_left, &mut self.back.top_right);

        //Swap front left rows with bottom left rows
        Cube::swap_unit_colors(&mut self.front.top_left, &mut self.bottom.top_left);
        Cube::swap_unit_colors(&mut self.front.mid_left, &mut self.bottom.mid_left);
        Cube::swap_unit_colors(&mut self.front.bot_left, &mut self.bottom.bot_left);

        //TODO:Perform counter clockwise turn on left side
        Cube::counterclockwise_turn(&mut self.left);
    }

    pub fn right_clockwise(&mut self){
        //Swap front left rows with top left rows
        Cube::swap_unit_colors(&mut self.front.top_right, &mut self.top.top_right);
        Cube::swap_unit_colors(&mut self.front.mid_right, &mut self.top.mid_right);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.top.bot_right);

        //Swap front left rows with back right rows
        Cube::swap_unit_colors(&mut self.front.top_right, &mut self.back.bot_left);
        Cube::swap_unit_colors(&mut self.front.mid_right, &mut self.back.mid_left);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.back.top_left);

        //Swap front left rows with bottom left rows
        Cube::swap_unit_colors(&mut self.front.top_right, &mut self.bottom.top_right);
        Cube::swap_unit_colors(&mut self.front.mid_right, &mut self.bottom.mid_right);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.bottom.bot_right);

        //TODO: Perform clockwise turn on right side
        Cube::clockwise_turn(&mut self.right);
    }

    pub fn top_clockwise(&mut self){
        //Swap top row of front with the top row of the left
        Cube::swap_unit_colors(&mut self.front.top_left, &mut self.right.top_left);
        Cube::swap_unit_colors(&mut self.front.top_middle, &mut self.right.top_middle);
        Cube::swap_unit_colors(&mut self.front.top_right, &mut self.right.top_right);

        //Swap front top tows with the back top rows
        Cube::swap_unit_colors(&mut self.right.top_left , &mut self.left.top_left);
        Cube::swap_unit_colors(&mut self.right.top_middle, &mut self.left.top_middle);
        Cube::swap_unit_colors(&mut self.right.top_right, &mut self.left.top_right);

        //Swap front top rows with the right top rows
        Cube::swap_unit_colors(&mut self.right.top_left , &mut self.back.top_left);
        Cube::swap_unit_colors(&mut self.right.top_middle, &mut self.back.top_middle);
        Cube::swap_unit_colors(&mut self.right.top_right, &mut self.back.top_right);

        //TODO:Perform counter clockwise turn on left side
        Cube::clockwise_turn(&mut self.top);
    }

    pub fn bottom_clockwise(&mut self){
        //Swap front bottom with left bottom
        Cube::swap_unit_colors(&mut self.front.bot_left, &mut self.right.bot_left);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.right.bot_middle);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.right.bot_right);

        //Swap front top tows with the back top rows
        Cube::swap_unit_colors(&mut self.front.bot_left , &mut self.back.bot_left);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.back.bot_middle);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.back.bot_right);

        //Swap front top rows with the left top rows
        Cube::swap_unit_colors(&mut self.front.bot_left , &mut self.left.bot_left);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.left.bot_middle);
        Cube::swap_unit_colors(&mut self.front.bot_right, &mut self.left.bot_right);

        //TODO: Perform clockwise turn on bottom side
        Cube::clockwise_turn(&mut self.bottom)
    }

    pub fn front_clockwise(&mut self){
        //Swap front bottom with left bottom
        Cube::swap_unit_colors(&mut self.left.top_right, &mut self.top.bot_right);
        Cube::swap_unit_colors(&mut self.left.mid_right, &mut self.top.bot_middle);
        Cube::swap_unit_colors(&mut self.left.bot_right, &mut self.top.bot_left);

        //Swap front top tows with the back top rows
        Cube::swap_unit_colors(&mut self.left.top_right , &mut self.right.bot_left);
        Cube::swap_unit_colors(&mut self.left.mid_right, &mut self.right.mid_left);
        Cube::swap_unit_colors(&mut self.left.bot_right, &mut self.right.top_left);

        //Swap front top rows with the left top rows
        Cube::swap_unit_colors(&mut self.left.top_right , &mut self.bottom.top_left);
        Cube::swap_unit_colors(&mut self.left.mid_right, &mut self.bottom.top_middle);
        Cube::swap_unit_colors(&mut self.left.bot_right, &mut self.bottom.top_right);

        Cube::clockwise_turn(&mut self.front);
    }

    pub fn back_clockwise(&mut self){
        //Swap front bottom with left bottom
        Cube::swap_unit_colors(&mut self.right.top_right, &mut self.top.top_left);
        Cube::swap_unit_colors(&mut self.right.mid_right, &mut self.top.top_middle);
        Cube::swap_unit_colors(&mut self.right.bot_right, &mut self.top.top_right);

        //Swap front top tows with the back top rows
        Cube::swap_unit_colors(&mut self.right.top_right , &mut self.left.bot_left);
        Cube::swap_unit_colors(&mut self.right.mid_right, &mut self.left.mid_left);
        Cube::swap_unit_colors(&mut self.right.bot_right, &mut self.left.top_left);

        //Swap front top rows with the left top rows
        Cube::swap_unit_colors(&mut self.right.top_right , &mut self.bottom.bot_right);
        Cube::swap_unit_colors(&mut self.right.mid_right, &mut self.bottom.bot_middle);
        Cube::swap_unit_colors(&mut self.right.bot_right, &mut self.bottom.bot_left);

        Cube::clockwise_turn(&mut self.back);
    }

    pub fn center_upward_twist(&mut self){
        //Swap front left rows with top left rows
        Cube::swap_unit_colors(&mut self.front.top_middle, &mut self.top.top_middle);
        Cube::swap_unit_colors(&mut self.front.mid_middle, &mut self.top.mid_middle);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.top.bot_middle);

        //Swap front left rows with back right rows
        Cube::swap_unit_colors(&mut self.front.top_middle, &mut self.back.top_middle);
        Cube::swap_unit_colors(&mut self.front.mid_middle, &mut self.back.mid_middle);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.back.bot_middle);

        //Swap front left rows with bottom left rows
        Cube::swap_unit_colors(&mut self.front.top_middle, &mut self.bottom.top_middle);
        Cube::swap_unit_colors(&mut self.front.mid_middle, &mut self.bottom.mid_middle);
        Cube::swap_unit_colors(&mut self.front.bot_middle, &mut self.bottom.bot_middle);
    }

    pub fn clockwise_turn(side : &mut Side){
        Cube::swap_unit_colors(&mut side.top_left, &mut side.bot_left);
        Cube::swap_unit_colors(&mut side.top_right, &mut side.bot_left);
        Cube::swap_unit_colors(&mut side.top_middle, &mut side.mid_left);
        Cube::swap_unit_colors(&mut side.mid_left, &mut side.mid_right);
        Cube::swap_unit_colors(&mut side.mid_left, &mut side.bot_middle);
        Cube::swap_unit_colors(&mut side.bot_left, &mut side.bot_right);
    }

    pub fn counterclockwise_turn(side : &mut Side){
        Cube::swap_unit_colors(&mut side.top_left, &mut side.top_right);
        Cube::swap_unit_colors(&mut side.top_right, &mut side.bot_right);
        Cube::swap_unit_colors(&mut side.top_middle, &mut side.mid_right);
        Cube::swap_unit_colors(&mut side.mid_left, &mut side.mid_right);
        Cube::swap_unit_colors(&mut side.mid_right, &mut side.bot_middle);
        Cube::swap_unit_colors(&mut side.bot_left, &mut side.bot_right);
    }

    pub fn write_cube_side(target_side: &Side){
        println!("{} {} {}", target_side.top_left, target_side.top_middle, target_side.top_right);
        println!("{} {} {}", target_side.mid_left, target_side.mid_middle, target_side.mid_right);
        println!("{} {} {}", target_side.bot_left, target_side.bot_middle, target_side.bot_right);
        println!();
    }

    pub fn is_side_sorted(target_side: &Side) -> bool{
        return target_side.top_left == target_side.top_middle
        && target_side.top_left == target_side.top_right
        && target_side.top_left == target_side.mid_left
        && target_side.top_left == target_side.mid_middle
        && target_side.top_left == target_side.mid_right
        && target_side.top_left == target_side.bot_left
        && target_side.top_left == target_side.bot_middle
        && target_side.top_left == target_side.bot_right
    }

    pub fn swap_unit_colors(unit1: &mut u8, unit2: &mut u8) {
        *unit1 = *unit1 + *unit2;
        *unit2 = *unit1 - *unit2;
        *unit1 = *unit1 - *unit2;
    }
}

pub enum Colors {
    Red,
    White,
    Blue,
    Green,
    Orange,
    Purple
}

/*
Tests to ensure all moves are working correctly
*/

#[cfg(test)]
mod tests{
    extern crate test;
    use crate::Cube;
    use test::Bencher;

    //counterclockwise turn tests
    #[test]
    fn counterclockwise_turn(){
        let mut test_cube = Cube::new();

        /*Set up clear markers for the transformation*/
        test_cube.left.top_left = 1;
        test_cube.left.top_middle = 2;
        test_cube.left.top_right = 3;
        test_cube.left.mid_left = 4;
        test_cube.left.mid_middle = 5;
        test_cube.left.mid_right = 6;
        test_cube.left.bot_left = 7;
        test_cube.left.bot_middle = 8;
        test_cube.left.bot_right = 9;

        Cube::counterclockwise_turn(&mut test_cube.left);

        /*Next we need to ensure the clockwise/counterclockwise transform works*/
        assert_eq!(test_cube.left.top_left, 3);
        assert_eq!(test_cube.left.top_middle, 6);
        assert_eq!(test_cube.left.top_right, 9);
        assert_eq!(test_cube.left.mid_left, 2);
        assert_eq!(test_cube.left.mid_middle, 5);
        assert_eq!(test_cube.left.mid_right, 8);
        assert_eq!(test_cube.left.bot_left, 1);
        assert_eq!(test_cube.left.bot_middle, 4);
        assert_eq!(test_cube.left.bot_right, 7);
    }

    #[bench]
    fn bench_counterclockwise_turn_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                Cube::counterclockwise_turn(&mut test_cube.left);
            }
        });
    }

    //clockwise turn tests
    #[test]
    fn clockwise_turn(){
        let mut test_cube = Cube::new();

        /*Set up clear markers for the transformation*/
        test_cube.left.top_left = 1;
        test_cube.left.top_middle = 2;
        test_cube.left.top_right = 3;
        test_cube.left.mid_left = 4;
        test_cube.left.mid_middle = 5;
        test_cube.left.mid_right = 6;
        test_cube.left.bot_left = 7;
        test_cube.left.bot_middle = 8;
        test_cube.left.bot_right = 9;

        Cube::clockwise_turn(&mut test_cube.left);

        /*Next we need to ensure the clockwise/counterclockwise transform works*/
        assert_eq!(test_cube.left.top_left, 7);
        assert_eq!(test_cube.left.top_middle, 4);
        assert_eq!(test_cube.left.top_right, 1);
        assert_eq!(test_cube.left.mid_left, 8);
        assert_eq!(test_cube.left.mid_middle, 5);
        assert_eq!(test_cube.left.mid_right, 2);
        assert_eq!(test_cube.left.bot_left, 9);
        assert_eq!(test_cube.left.bot_middle, 6);
        assert_eq!(test_cube.left.bot_right, 3);
    }

    #[bench]
    fn bench_clockwise_turn_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                Cube::clockwise_turn(&mut test_cube.left);
            }
        });
    }

    //top clockwise turn tests
    #[test]
    fn top_clockwise_turn(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.front.top_left = 1;
        test_cube.front.top_middle = 2;
        test_cube.front.top_right = 3;

        test_cube.right.top_left = 4;
        test_cube.right.top_middle = 5;
        test_cube.right.top_right = 6;

        test_cube.back.top_left = 7;
        test_cube.back.top_middle = 8;
        test_cube.back.top_right = 9;

        test_cube.left.top_left = 10;
        test_cube.left.top_middle = 11;
        test_cube.left.top_right = 12;

        test_cube.top_clockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.front.top_left, 4);
        assert_eq!(test_cube.front.top_middle, 5);
        assert_eq!(test_cube.front.top_right, 6);

        assert_eq!(test_cube.right.top_left, 7);
        assert_eq!(test_cube.right.top_middle, 8);
        assert_eq!(test_cube.right.top_right, 9);

        assert_eq!(test_cube.back.top_left, 10);
        assert_eq!(test_cube.back.top_middle, 11);
        assert_eq!(test_cube.back.top_right, 12);

        assert_eq!(test_cube.left.top_left, 1);
        assert_eq!(test_cube.left.top_middle, 2);
        assert_eq!(test_cube.left.top_right, 3);
    }

    #[bench]
    fn bench_top_clockwise_turn_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.top_clockwise()
            }
        });
    }

    //left counterclockwise tests
    #[test]
    fn left_counterclockwise(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.front.top_left = 1;
        test_cube.front.mid_left = 2;
        test_cube.front.bot_left = 3;

        test_cube.top.top_left = 4;
        test_cube.top.mid_left = 5;
        test_cube.top.bot_left = 6;

        test_cube.back.top_right = 7;
        test_cube.back.mid_right = 8;
        test_cube.back.bot_right = 9;

        test_cube.bottom.top_left = 10;
        test_cube.bottom.mid_left = 11;
        test_cube.bottom.bot_left = 12;

        test_cube.left_counterclockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.front.top_left, 10);
        assert_eq!(test_cube.front.mid_left, 11);
        assert_eq!(test_cube.front.bot_left, 12);

        assert_eq!(test_cube.top.top_left, 1);
        assert_eq!(test_cube.top.mid_left, 2);
        assert_eq!(test_cube.top.bot_left, 3);

        assert_eq!(test_cube.back.top_right, 6);
        assert_eq!(test_cube.back.mid_right, 5);
        assert_eq!(test_cube.back.bot_right, 4);

        assert_eq!(test_cube.bottom.top_left, 9);
        assert_eq!(test_cube.bottom.mid_left, 8);
        assert_eq!(test_cube.bottom.bot_left, 7);
    }

    #[bench]
    fn bench_left_counterclockwise_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.left_counterclockwise();
            }
        });
    }

    //left counterclockwise tests
    #[test]
    fn right_clockwise(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.front.top_right = 1;
        test_cube.front.mid_right = 2;
        test_cube.front.bot_right = 3;

        test_cube.top.top_right = 4;
        test_cube.top.mid_right = 5;
        test_cube.top.bot_right = 6;

        test_cube.back.top_left = 7;
        test_cube.back.mid_left = 8;
        test_cube.back.bot_left = 9;

        test_cube.bottom.top_right = 10;
        test_cube.bottom.mid_right = 11;
        test_cube.bottom.bot_right = 12;

        test_cube.right_clockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.front.top_right, 10);
        assert_eq!(test_cube.front.mid_right, 11);
        assert_eq!(test_cube.front.bot_right, 12);

        assert_eq!(test_cube.top.top_right, 1);
        assert_eq!(test_cube.top.mid_right, 2);
        assert_eq!(test_cube.top.bot_right, 3);

        assert_eq!(test_cube.back.top_left, 6);
        assert_eq!(test_cube.back.mid_left, 5);
        assert_eq!(test_cube.back.bot_left, 4);

        assert_eq!(test_cube.bottom.top_right, 9);
        assert_eq!(test_cube.bottom.mid_right, 8);
        assert_eq!(test_cube.bottom.bot_right, 7);
    }

    #[bench]
    fn bench_right_clockwise_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.right_clockwise();
            }
        });
    }

    //bottom clockwise tests
    #[test]
    fn bottom_clockwise(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.front.bot_left = 1;
        test_cube.front.bot_middle = 2;
        test_cube.front.bot_right = 3;

        test_cube.right.bot_left = 4;
        test_cube.right.bot_middle = 5;
        test_cube.right.bot_right = 6;

        test_cube.back.bot_left = 7;
        test_cube.back.bot_middle = 8;
        test_cube.back.bot_right = 9;

        test_cube.left.bot_left = 10;
        test_cube.left.bot_middle = 11;
        test_cube.left.bot_right = 12;

        test_cube.bottom_clockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.front.bot_left, 10);
        assert_eq!(test_cube.front.bot_middle, 11);
        assert_eq!(test_cube.front.bot_right, 12);

        assert_eq!(test_cube.right.bot_left, 1);
        assert_eq!(test_cube.right.bot_middle, 2);
        assert_eq!(test_cube.right.bot_right, 3);

        assert_eq!(test_cube.back.bot_left, 4);
        assert_eq!(test_cube.back.bot_middle, 5);
        assert_eq!(test_cube.back.bot_right, 6);

        assert_eq!(test_cube.left.bot_left, 7);
        assert_eq!(test_cube.left.bot_middle, 8);
        assert_eq!(test_cube.left.bot_right, 9);
    }

    #[bench]
    fn bench_bottom_clockwise_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.right_clockwise();
            }
        });
    }

    //bottom clockwise tests
    #[test]
    fn front_clockwise(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.left.top_right = 3;
        test_cube.left.mid_right = 2;
        test_cube.left.bot_right = 1;

        test_cube.top.bot_left = 4;
        test_cube.top.bot_middle = 5;
        test_cube.top.bot_right = 6;

        test_cube.right.top_left = 7;
        test_cube.right.mid_left = 8;
        test_cube.right.bot_left = 9;

        test_cube.bottom.top_left = 12;
        test_cube.bottom.top_middle = 11;
        test_cube.bottom.top_right = 10;

        test_cube.front_clockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.left.top_right, 12);
        assert_eq!(test_cube.left.mid_right, 11);
        assert_eq!(test_cube.left.bot_right, 10);

        assert_eq!(test_cube.top.bot_left, 1);
        assert_eq!(test_cube.top.bot_middle, 2);
        assert_eq!(test_cube.top.bot_right, 3);

        assert_eq!(test_cube.right.top_left, 4);
        assert_eq!(test_cube.right.mid_left, 5);
        assert_eq!(test_cube.right.bot_left, 6);

        assert_eq!(test_cube.bottom.top_left, 9);
        assert_eq!(test_cube.bottom.top_middle, 8);
        assert_eq!(test_cube.bottom.top_right, 7);
    }

    #[bench]
    fn bench_front_clockwise_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.front_clockwise();
            }
        });
    }

    //bottom clockwise tests
    #[test]
    fn back_clockwise(){
        let mut test_cube = Cube::new();
        /*We need to manually set some values so that our tests are valid*/
        test_cube.right.top_right = 3;
        test_cube.right.mid_right = 2;
        test_cube.right.bot_right = 1;

        test_cube.top.top_left = 6;
        test_cube.top.top_middle = 5;
        test_cube.top.top_right = 4;

        test_cube.left.top_left = 7;
        test_cube.left.mid_left = 8;
        test_cube.left.bot_left = 9;

        test_cube.bottom.bot_left = 10;
        test_cube.bottom.bot_middle = 11;
        test_cube.bottom.bot_right = 12;

        test_cube.back_clockwise();
        /*In this test, we need to make sure all of the faces
        have the correct values*/

        /*First we need to test that the 4 impacted sides*/
        assert_eq!(test_cube.right.top_right, 12);
        assert_eq!(test_cube.right.mid_right, 11);
        assert_eq!(test_cube.right.bot_right, 10);

        assert_eq!(test_cube.top.top_left, 3);
        assert_eq!(test_cube.top.top_middle, 2);
        assert_eq!(test_cube.top.top_right, 1);

        assert_eq!(test_cube.left.top_left, 4);
        assert_eq!(test_cube.left.mid_left, 5);
        assert_eq!(test_cube.left.bot_left, 6);

        assert_eq!(test_cube.bottom.bot_left, 7);
        assert_eq!(test_cube.bottom.bot_middle, 8);
        assert_eq!(test_cube.bottom.bot_right, 9);
    }

    #[bench]
    fn bench_back_clockwise_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            //Run 1 million loops
            for i in 1..1000000 {
                test_cube.back_clockwise();
            }
        });
    }

    #[bench]
    fn bench_side_sorted_1m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            let n = test::black_box(&test_cube);
            //Run 1 million loops
            for i in 1..1000000 {
                let sort_val = Cube::is_side_sorted(&test_cube.front);
            }
        });
    }

    #[bench]
    fn bench_unit_swap_100m(bnchr: &mut Bencher){
        let mut test_cube = Cube::new();
        bnchr.iter(|| {
            let n = test::black_box(&test_cube);
            //Run 1 million loops
            for i in 1..100000000 {
                let sort_val = Cube::swap_unit_colors(&mut test_cube.front.top_right, &mut test_cube.back.top_left);
            }
        });
    }
}