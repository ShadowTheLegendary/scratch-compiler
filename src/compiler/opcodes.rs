use strum::EnumString;

use serde_json::Value;

pub fn string_to_block_opcode(string: &str) -> BlockOpcode {
    if let Ok(opcode) = string.parse::<BlockOpcode>() {
        return opcode;
    }

    BlockOpcode::None
}

pub fn is_entry_point(opcode: &str) -> bool {
    if opcode.starts_with("event_when") || (opcode == "control_start_as_clone") {
        return true;
    }

    false
}

pub fn is_flow_control(block: &Value) -> bool {
    let inputs = block["inputs"].clone();
    if inputs["SUBSTACK"].is_null() {
        return false;
    }

    true
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




    // Sensing / Cyan blocks
    SensingTouchingObject,
    SensingTouchingObjectMenu,
    SensingTouchingColor,
    SensingColorIsTouchingColor,
    SensingDistanceTo,
    SensingDistanceToMenu,

    SensingAskAndWait,
    SensingAnswer,

    SensingKeyPressed,
    SensingKeyOptions,
    SensingMouseDown,
    SensingMouseX,
    SensingMouseY,

    SensingSetDragMode,

    SensingLoudness,

    SensingTimer,
    SensingResetTimer,

    SensingOf,
    SensingOfObjectMenu,

    SensingCurrent,
    SensingDaysSince2000,

    SensingOnline,
    SensingUsername,




    // Operator / Green blocks
    OperatorAdd,
    OperatorSubtract,
    OperatorMultiply,
    OperatorDivide,

    OperatorRandom,

    OperatorGt,
    OperatorLt,
    OperatorEquals,

    OperatorAnd,
    OperatorOr,
    OperatorNot,

    OperatorJoin,
    OperatorLetterOf,
    OperatorLength,
    OperatOrContains,

    OperatorMod,
    OperatorRound,

    OperatorMathOp,
}
