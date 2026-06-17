use strum::{Display, EnumString};

/*pub fn opcode_to_string(opcode: &Opcode) -> String {
    opcode.to_string()
}

pub fn string_to_opcode(string: &str) -> Opcode{
    string.parse().expect("String does not match known opcode.")
}*/

#[derive(Debug, Display, PartialEq, EnumString)]
pub enum MonitorOpcode {
    // Motion / Blue blocks
    MotionXPosition,
    MotionYPosition,

    MotionDirection,


    // Looks / Purple blocks
    LooksSize,
    LooksBackDropNumberName,
    LooksCostumeNumberName,

    
    // Sound / Pink blocks
    SoundVolume,
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
    LooksSayForSecs,
    LooksSay,
    LooksThink,
    LooksThinkForSecs,

    LooksCostume,
    LooksSwitchCostumeTo,
    LooksNextCostume,

    LooksBackdrops,
    LooksSwitchBackdropTo,
    LooksNextBackdrop,

    LooksChangeSizeBy,
    LooksSetSizeTo,
    
    LooksChangeEffectBy,
    LooksSetEffectTo,
    LooksClearGraphicEffects,

    LooksShow,
    LooksHide,

    LooksGoToFrontBack,
    LooksGoForwardBackwardLayers,
    
    // Oval blocks
    LooksCostumeNumberName,
    LooksBackdropNumberName,
    LooksSize,


    // Sound / Pink blocks
    // Rectangle blocks
    SoundsSoundsMenu,
    SoundPlayUntilDone,
    SoundPlay,
    SoundStopAllSounds,

    SoundChangeEffectBy,
    SoundSetEffectTo,
    SoundClearEffects,

    SoundChangeVolumeBy,
    SoundSetVolumeTo,

    // Oval blocks
    SoundVolume,
}