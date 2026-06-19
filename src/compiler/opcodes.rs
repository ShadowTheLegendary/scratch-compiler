use strum::EnumString;

pub fn string_to_block_opcode(string: &str) -> BlockOpcode {
    if let Ok(opcode) = string.parse::<BlockOpcode>() {
        return opcode;
    }

    BlockOpcode::None
}

impl std::fmt::Display for BlockOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
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

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum BlockOpcode {
    // Error code
    None,
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

    // Events / Yellow blocks
    // Code entrypoints
    EventWhenFlagClicked,
    EventWhenKeyPressed,
    EventWhenThisSpriteClicked,
    EventWhenBackdropSwitchesTo,
    EventWhenGreaterThan,
    EventWhenBroadcastReceived,

    // Rectangle blocks
    EventBroadcast,
    EventBroadcastAndWait,


    // Control / Orange blocks
    ControlWait,

    ControlRepeat,
    ControlForever,

    ControlIf,
    ControlIfElse,
    ControlWaitUntil,
    ControlRepeatUntil,

    ControlStop,

    ControlStartAsClone,
    ControlCreateCloneOf,
    ControlCreateCloneOfMenu,
    ControlDeleteThisClone,
}
