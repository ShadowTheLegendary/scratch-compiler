use strum::{Display, EnumString};

/*pub fn opcode_to_string(opcode: &Opcode) -> String {
    opcode.to_string()
}

pub fn string_to_opcode(string: &str) -> Opcode{
    string.parse().expect("String does not match known opcode.")
}*/

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum MonitorOpcode {
    MotionXPosition,
    MotionYPosition,

    MotionDirection,


    LooksSize,
    LooksBackDropNumberName,
    LooksCostumeNumberName,
}


#[derive(Debug, Display, PartialEq, EnumString)]
pub enum BlockOpcode {
    // Motion / Blue blocks
    // Rectangle blocks
    MotionMoveSteps,

    MotionTurnRight,
    MotionTurnLeft,

    MotionGoTo,
    MotionGoToMenu,
    MotionGoToXY,

    MotionGlideTo,
    MotionGlideToMenu,
    MotionGlideSecsToXY,
    
    MotionPointIndirection,
    MotionPointTowards,
    MotionPointTowardsMenu,

    MotionChangeXBy,
    MotionSetX,
    MotionChangeYBy,
    MotionSetY,

    MotionIfOnEdgeBounce,

    MotionSetRotationStyle,

    // Oval blocks
    MotionXPosition,
    MotionYPosition,

    MotionDirection,

    // Looks / Purple blocks
    // Rectangle blocks

    // Oval blocks
}