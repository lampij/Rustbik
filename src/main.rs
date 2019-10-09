use std::borrow::Borrow;

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

    pub fn Left_Counterclockwise(&mut self){
        //Swap front left rows with top left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.top.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.top.mid_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.top.bot_left);

        //Swap front left rows with back right rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.back.top_right);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.back.mid_right);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.back.bot_right);

        //Swap front left rows with bottom left rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.bottom.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.mid_left, &mut self.bottom.mid_left);
        Cube::Swap_Unit_Colors(&mut self.front.bot_left, &mut self.bottom.bot_left);

        //TODO:Perform counter clockwise turn on left side
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
        //Swap front top rows with the left top rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left, &mut self.right.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.right.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.right.top_right);

        //Swap front top tows with the back top rows
        Cube::Swap_Unit_Colors(&mut self.front.top_left , &mut self.back.top_left);
        Cube::Swap_Unit_Colors(&mut self.front.top_middle, &mut self.back.top_middle);
        Cube::Swap_Unit_Colors(&mut self.front.top_right, &mut self.back.top_right);

        //Swap front top rows with the left top rows
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
        Cube::Swap_Unit_Colors(&mut self.left.mid_right, &mut self.left.bot_middle);
        Cube::Swap_Unit_Colors(&mut self.left.bot_right, &mut self.left.bot_right);
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

    pub fn Clockwise_Turn(&mut self){
        //TODO: Implement this
    }

    pub fn Counterclockwise_Turn(&mut self){
        //TODO: Don't be lazy, actually implement this move rather than doing 3 clockwise turns
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