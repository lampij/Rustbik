use std::borrow::Borrow;
use std::arch;

fn main() {
    let mut cube = Cube::New();
    println!("Front");
    Cube::Write_Cube_Side(&cube.front);
    println!("Top");
    Cube::Write_Cube_Side(&cube.top);
    println!("Back");
    Cube::Write_Cube_Side(&cube.back);
    println!("Bottom");
    Cube::Write_Cube_Side(&cube.bottom);

    //Do some moves
    cube.Left_Counterclockwise();
    cube.Right_Clockwise();
    cube.Center_Upward_Twist();


    println!("Front");
    Cube::Write_Cube_Side(&cube.front);
    println!("Top");
    Cube::Write_Cube_Side(&cube.top);
    println!("Back");
    Cube::Write_Cube_Side(&cube.back);
    println!("Bottom");
    Cube::Write_Cube_Side(&cube.bottom);
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
    pub fn New(val: u8) -> Side{
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
    pub fn New() -> Cube {
        Cube {
            front: Side::New(0),
            top: Side::New(1),
            back: Side::New(2),
            bottom: Side::New(3),
            left: Side::New(4),
            right: Side::New(5)
        }
    }

    /*Implementation Completed*/
    pub fn Left_Counterclockwise(&mut self){
        //Swap front left rows with top left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.top.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.top.mid_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.top.bot_left);

        //Swap front left rows with back right rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.back.bot_right);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.back.mid_right);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.back.top_right);

        //Swap front left rows with bottom left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.bottom.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.bottom.mid_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.bottom.bot_left);

        //TODO:Perform counter clockwise turn on left side
        Cube::Counterclockwise_Turn(&mut self.left);
    }

    pub fn Right_Clockwise(&mut self){
        //Swap front left rows with top left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.top.top_right);
        Cube::Swap_Unit_Colors(&mut self.front.mid_right, &mut self.top.mid_right);
        Cube::Swap_Unit_Colors(&mut self.front.bot_right, &mut self.top.bot_right);

        //Swap front left rows with back right rows
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.back.bot_left);
        Cube::Swap_Unit_Colors(&mut self.front.mid_right, &mut self.back.mid_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_right, &mut self.back.top_left);

        //Swap front left rows with bottom left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.bottom.top_right);
        Cube::Swap_Unit_Colors(&mut self.front.mid_right, &mut self.bottom.mid_right);
        Cube::Swap_Unit_Colors(&mut self.front.bot_right, &mut self.bottom.bot_right);

        //TODO: Perform clockwise turn on right side
    }

    pub fn Top_Clockwise(&mut self){
        //Swap top row of front with the top row of the left
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.left.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.left.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.left.top_right);

        //Swap front top tows with the back top rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left , &mut self.back.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.back.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.back.top_right);

        //Swap front top rows with the right top rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left , &mut self.left.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.left.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.left.top_right);

        //TODO: Perform clockwise turn on top side
    }

    pub fn Bottom_Clockwise(&mut self){
        //Swap front bottom with left bottom
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.right.bot_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.right.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_right, &mut self.right.bot_right);

        //Swap front top tows with the back top rows
        Cube::Swap_Unit_Colors(&mut self.front.bot_left , &mut self.back.bot_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.back.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_right, &mut self.back.bot_right);

        //Swap front top rows with the left top rows
        Cube::Swap_Unit_Colors(&mut self.front.bot_left , &mut self.left.bot_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.left.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.left.bot_right);

        //TODO: Perform clockwise turn on bottom side
    }

    pub fn Face_Clockwise(&mut self){
        //Swap front bottom with left bottom
        Cube::Swap_Unit_Colors(&mut self.left.top_right, &mut self.top.bot_right);
        Cube::Swap_Unit_Colors(&mut self.left.mid_right, &mut self.top.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.left.bot_right, &mut self.top.bot_left);

        //Swap front top tows with the back top rows
        Cube::Swap_Unit_Colors(&mut self.left.top_right , &mut self.right.top_left);
        Cube::Swap_Unit_Colors(&mut self.left.mid_right, &mut self.right.mid_left);
        Cube::Swap_Unit_Colors(&mut self.left.bot_right, &mut self.right.bot_left);

        //Swap front top rows with the left top rows
        Cube::Swap_Unit_Colors(&mut self.left.top_right , &mut self.bottom.bot_left);
        Cube::Swap_Unit_Colors(&mut self.left.mid_right, &mut self.bottom.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.left.bot_right, &mut self.bottom.bot_right);
    }

    pub fn Back_Clockwise(&mut self){

    }

    pub fn Center_Upward_Twist(&mut self){
        //Swap front left rows with top left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.top.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.mid_middle, &mut self.top.mid_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.top.bot_middle);

        //Swap front left rows with back right rows
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.back.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.mid_middle, &mut self.back.mid_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.back.bot_middle);

        //Swap front left rows with bottom left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.bottom.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.mid_middle, &mut self.bottom.mid_middle);
        Cube::Swap_Unit_Colors(&mut self.front.bot_middle, &mut self.bottom.bot_middle);
    }

    pub fn Clockwise_Turn(side : &mut Side){
        Cube::Swap_Unit_Colors(&mut side.top_left, &mut side.bot_left);
        Cube::Swap_Unit_Colors(&mut side.top_right, &mut side.bot_left);
        Cube::Swap_Unit_Colors(&mut side.top_middle, &mut side.mid_left);
        Cube::Swap_Unit_Colors(&mut side.mid_left, &mut side.mid_right);
        Cube::Swap_Unit_Colors(&mut side.mid_left, &mut side.bot_middle);
        Cube::Swap_Unit_Colors(&mut side.bot_left, &mut side.bot_right);
    }

    pub fn Counterclockwise_Turn(side : &mut Side){
        Cube::Swap_Unit_Colors(&mut side.top_left, &mut side.top_right);
        Cube::Swap_Unit_Colors(&mut side.top_right, &mut side.bot_right);
        Cube::Swap_Unit_Colors(&mut side.top_middle, &mut side.mid_right);
        Cube::Swap_Unit_Colors(&mut side.mid_left, &mut side.mid_right);
        Cube::Swap_Unit_Colors(&mut side.mid_right, &mut side.bot_middle);
        Cube::Swap_Unit_Colors(&mut side.bot_left, &mut side.bot_right);
    }

    pub fn Write_Cube_Side(Target_Side: &Side){
        println!("{} {} {}", Target_Side.top_left, Target_Side.top_middle, Target_Side.top_right);
        println!("{} {} {}", Target_Side.mid_left, Target_Side.mid_middle, Target_Side.mid_right);
        println!("{} {} {}", Target_Side.bot_left, Target_Side.bot_middle, Target_Side.bot_right);
        println!();
    }

    pub fn Is_Side_Sorted(Target_Side: &Side) -> bool{
        return Target_Side.top_left == Target_Side.top_middle
        && Target_Side.top_left == Target_Side.top_right
        && Target_Side.top_left == Target_Side.mid_left
        && Target_Side.top_left == Target_Side.mid_middle
        && Target_Side.top_left == Target_Side.mid_right
        && Target_Side.top_left == Target_Side.bot_left
        && Target_Side.top_left == Target_Side.bot_middle
        && Target_Side.top_left == Target_Side.bot_right
    }

    pub fn Swap_Unit_Colors(mut unit1: &mut u8, mut unit2: &mut u8) {
        *unit1 = *unit1 + *unit2;
        *unit2 = *unit1 - *unit2;
        *unit1 = *unit1 - *unit2;
    }
}

pub enum colors {
    red,
    white,
    blue,
    green,
    orange,
    purple
}

/*
Tests to ensure all moves are working correctly
*/

#[cfg(test)]
mod Tests{
    use crate::Cube;

    #[test]
    fn left_counterclockwise(){
        let mut test_cube = Cube::New();
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

        test_cube.Left_Counterclockwise();
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

    #[test]
    fn counterclockwise_turn(){
        let mut test_cube = Cube::New();

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

        Cube::Counterclockwise_Turn(&mut test_cube.left);

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
}