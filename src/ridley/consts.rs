pub mod status_kind_ex {
    pub const FIGHTER_STATUS_KIND_LAST: i32 = 0x202;//FIGHTER_<fighter_kind>_STATUS_KIND_NUM 
    pub const FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK: i32 = FIGHTER_STATUS_KIND_LAST+1;
    pub const FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_POGO: i32 = FIGHTER_STATUS_KIND_LAST+2;
    pub const FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_LANDING: i32 = FIGHTER_STATUS_KIND_LAST+3;
}
pub mod param {
    //skewer
    pub const RIDLEY_INT_SPECIAL_LW_STAB_HOLD_FRAME:i32 = 35;
    //pogo
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_JUMP_SPEED_Y:f32 = 2.1;
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_MIN:f32 = 1.8;
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_MAX:f32 = 2.8;
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_CHECK_Y_MAX:f32 = 25.0;
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_X_MUL:f32 = 0.5;
    pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_Y_HIT:f32 = 1.8;
    //shield-special
    pub const RIDLEY_INT_SKEWER_INPUT_FRAME:i32 = 2;
}
pub mod vars {
    pub mod instance {//0x01??
        //flag
        // pub const RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH : i32 = 0x0100;
        pub const RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER : i32 = 0x0101;
        //int
        pub const RIDLEY_INT_SPECIAL_HI_WALL_COUNT : i32 = 0x0100;
        // float
        // pub const RIDLEY_FLOAT_ : i32 = 0x0100;
    }
    pub mod status {//0x11??
        //flag
        pub const RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD : i32 = 0x1100;
        pub const RIDLEY_FLAG_SPECIAL_LW_POGO_ENABLE_LANDING : i32 = 0x1101;
        pub const RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE : i32 = 0x1102;
        //int
        pub const RIDLEY_INT_SPECIAL_LW_FINISH_CAPTURE_ID : i32 = 0x1100;
        //float
        pub const RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME : i32 = 0x1100;
        pub const RIDLEY_FLOAT_SPECIAL_LW_STAB_PREV_MOTION_FRAME : i32 = 0x1101; 
        pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_X : i32 = 0x1102; 
        pub const RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_Y : i32 = 0x1103; 
    }
}