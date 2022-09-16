use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::{
            Hash40,
            Vector2f,
            Vector3f
        },
        app::{
            sv_animcmd::frame,
            sv_animcmd::wait,
            lua_bind::*,
            *
        },
        lib::{
            lua_const::*,
            *
        }
    },
    smashline::*,
    smash_script::{
        macros::*,
        *
    }
};
extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
  }

static mut RIDLEY_FLAG_SPECIAL_LW_IS_GRAB : [bool; 8] = [false; 8];
static mut RIDLEY_FLAG_SPECIAL_LW_THROW : [bool; 8] = [false; 8];
static mut RIDLEY_FLAG_SPECIAL_LW_ENABLE_LANDING : [bool; 8] = [false; 8];
static mut RIDLEY_FLAG_SPECIAL_LW_IS_LANDING : [bool; 8] = [false; 8];
static mut RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE : [bool; 8] = [false; 8];
static mut RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV : [Vector2f; 8] = [Vector2f{x:0.0,y:0.0}; 8];
static mut RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME : [f32; 8] = [0.0; 8];
// static mut RIDLEY_FLAG_SPECIAL_S_DRAG_IS_SLIDE : [bool; 8] = [false; 8];
// static mut RIDLEY_FLOAT_SPECIAL_S_DRAG_SLIDE_VEL_X : [f32; 8] = [0.0; 8];
static mut RIDLEY_INT_SPECIAL_CATCH_ID : [u32; 8] = [0; 8];
static mut RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT : [i32; 8] = [0; 8];


pub unsafe fn air_to_ground_transition_status_func(fighter: &mut L2CFighterCommon) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

#[fighter_frame( agent = FIGHTER_KIND_RIDLEY )]
fn per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] > 0 {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            // || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP1 {
            // || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP2 {
            // || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP3 {
                RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] = 0;
            }
        }
        if RIDLEY_FLAG_SPECIAL_LW_THROW[entry_id] {
            RIDLEY_FLAG_SPECIAL_LW_THROW[entry_id] = false;
            let capture_boma = sv_battle_object::module_accessor(RIDLEY_INT_SPECIAL_CATCH_ID[entry_id]);
            if StatusModule::situation_kind(capture_boma) != *SITUATION_KIND_GROUND {
                StopModule::end_stop(capture_boma);
                StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
            }
        }
        // else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF
        // && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW) {
        //     let capture_boma = sv_battle_object::module_accessor(RIDLEY_INT_SPECIAL_CATCH_ID[entry_id]);
        //     // WorkModule::set_int64(capture_boma, hash40("down_damage_u3") as i64, *FIGHTER_STATUS_DOWN_WORK_INT_MOTION_KIND);
        //     WorkModule::set_int(capture_boma, 38, *FIGHTER_STATUS_DOWN_WORK_INT_NO_ACTION_FRAME);
        //     // MotionModule::change_motion(capture_boma,  Hash40::new("down_damage_u"), 0.0, 1.0, false, 0.0, false, false);
        //     KineticModule::add_speed(capture_boma, &Vector3f{x:-RIDLEY_FLOAT_SPECIAL_S_DRAG_SLIDE_VEL_X[entry_id], y:0.0, z:0.0});
        // }
    }
}

//////down-special
////buffing og down-special
//increased shield damage -37->-20, increased shield stun 0.2->0.4, fixed flichless hit
#[acmd_script( agent = "ridley", scripts = ["game_speciallwstab", "game_specialairlwstab"], category = ACMD_GAME )]
unsafe fn down_special_stab_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(29.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -20.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT) <= 0 {
            AttackModule::set_no_dead_all(fighter.module_accessor, true, false);
        }
        ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(29.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
//adding footstool if aerial
#[acmd_script( agent = "ridley", scripts = ["game_speciallwfinish", "game_specialairlwfinish"], category = ACMD_GAME )]
unsafe fn down_special_finish_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 57, 100, 150, 0, 2.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_stab"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_no_finish_camera(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, true, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        RIDLEY_INT_SPECIAL_CATCH_ID[entry_id] = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        RIDLEY_FLAG_SPECIAL_LW_THROW[entry_id] = true;
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        JostleModule::set_status(fighter.module_accessor, true);
        WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
    }
}

////
////new aerial down-special pogo stab
//status stuff
#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn down_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
    || RIDLEY_FLAG_SPECIAL_LW_IS_GRAB[entry_id] {
        RIDLEY_FLAG_SPECIAL_LW_IS_GRAB[entry_id] = false;
        original!(fighter)
    }else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH);
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id] = false;
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_LANDING[entry_id] = false;
        RIDLEY_FLAG_SPECIAL_LW_IS_LANDING[entry_id] = false;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(down_special_main_loop as *const () as _))
    }

    // WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    // WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    // RIDLEY_FLAG_SPECIAL_LW_IS_POGO[entry_id] = false;
    // if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
    //     KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    //     MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_stab"), 0.0, 1.0, false, 0.0, false, false);
    // }else {
    //     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //     RIDLEY_FLAG_SPECIAL_LW_IS_POGO[entry_id] = true;
    //     KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    //     MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
    // }
    // fighter.sub_shift_status_main(L2CValue::Ptr(down_special_main_loop as *const () as _))
}
pub unsafe fn down_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if RIDLEY_FLAG_SPECIAL_LW_IS_LANDING[entry_id] {
        if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return true.into()
        }else if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return true.into()
        }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                return true.into()
            }
        }
    }else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if RIDLEY_FLAG_SPECIAL_LW_ENABLE_LANDING[entry_id] {
            //animation for landing pogo needed
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_pogo_landing"), 0.0, 1.0, false, 0.0, false, false);
            RIDLEY_FLAG_SPECIAL_LW_IS_LANDING[entry_id] = true;
        }else  {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return true.into()
        }
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }else {
        down_special_pogo_bounce_check(fighter, RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id]);
    }
    false.into()

    // let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    // if StatusModule::situation_kind(fighter.module_accessor) == StatusModule::prev_situation_kind(fighter.module_accessor) {
    //     if RIDLEY_FLAG_SPECIAL_LW_IS_POGO[entry_id] {
    //         fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    //         return true.into()
    //     }else {
    //         air_to_ground_transition_status_func(fighter);
    //     }
    // }
    // if MotionModule::is_end(fighter.module_accessor) {
    //     if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
    //         fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    //     }else {
    //         fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    //     }
    //     return true.into()
    // }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
    //     if fighter.sub_wait_ground_check_common(false.into()).get_bool()
    //     || fighter.sub_air_check_fall_common().get_bool() {
    //         return true.into()
    //     }
    // }else if RIDLEY_FLAG_SPECIAL_LW_IS_POGO[entry_id] == false
    // &&  {//<-hit-grab check---------------------------------------------------------------------------------------------------------------------------------------------
    //     fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH.into(), false.into());
    //     return true.into()
    // }
    // false.into()
}
pub unsafe fn down_special_pogo_bounce_check(fighter: &mut L2CFighterCommon, check_hit: bool) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("tail8"), v3f_tail_pos, false);
    let pos_x_global = PostureModule::pos_x(fighter.module_accessor);
    let pos_y_global = PostureModule::pos_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);

    let pos_x_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].x;
    let pos_y_prev = RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id].y;

    RIDLEY_VEC2_SPECIAL_LW_BOUNCE_POS_CHECK_PREV[entry_id] = Vector2f{x: v3f_tail_pos.x-pos_x_global, y: v3f_tail_pos.y-pos_y_global}; //save current tail pos relative to fighter

    if check_hit {
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        if GroundModule::ray_check_hit_pos(//checks for ground between prev tail pos and curr tail pos, saves collision pos to "ground_hit_pos"
            fighter.module_accessor,
            &Vector2f{x:pos_x_prev+pos_x_global, y: pos_y_prev+pos_y_global},
            &Vector2f{x: (v3f_tail_pos.x -(pos_x_prev+pos_x_global))+(8.0*lr), y: v3f_tail_pos.y -(pos_y_prev+pos_y_global) -8.0},
            ground_hit_pos,
            true
        ) == 1 {
            //deduces angle of slope for effects by using 2 Vector2f's and triginomatry
            let mut slope_angle = 0.0;
            let slope_check_pos = &mut Vector2f{x: 0.0, y: 0.0};
            if GroundModule::ray_check_hit_pos(
                fighter.module_accessor,
                &Vector2f{x:ground_hit_pos.x+(5.0*lr), y:ground_hit_pos.y+5.0},
                &Vector2f{x:0.0, y:-10.0 },
                slope_check_pos,
                true
            ) == 1 {
                let pos_diff_y = ground_hit_pos.y-slope_check_pos.y;
                if pos_diff_y > 0.0 {
                    slope_angle = (pos_diff_y / 5.0).atan().to_degrees();
                }else {
                    slope_angle = 360.0 -((-pos_diff_y / 5.0).atan().to_degrees());
                }
                // DamageModule::add_damage(fighter.module_accessor, slope_angle, 0);
            }
            EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), (ground_hit_pos.x-pos_x_global)*lr, ground_hit_pos.y -pos_y_global, 0, slope_angle, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), (ground_hit_pos.x-pos_x_global)*lr, ground_hit_pos.y -pos_y_global, 0, slope_angle, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);

            PLAY_SE(fighter, Hash40::new("se_ridley_special_h03"));
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);

            let mut velocity_y = (-0.05*(pos_y_global -ground_hit_pos.y))+2.5; //calculates bounce hight based off distane from ground
            if pos_y_global -ground_hit_pos.y <= 0.0 {
                velocity_y = 2.5;
            }else if velocity_y < 0.0 {
                velocity_y = 0.0;
            }
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            SET_SPEED_EX(fighter, velocity_x*0.5, velocity_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id] = false;
        }else if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {//hitting a hurt-box gives set momentum
            let velocity_x = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            SET_SPEED_EX(fighter, velocity_x*0.5, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id] = false;
        }
    }
}
//air pogo scripts
#[acmd_script( agent = "ridley", script = "game_specialairlwpogo", category = ACMD_GAME )]
unsafe fn down_special_pogo_game(fighter : &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 7.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_LANDING[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id] = true;
    }
    frame(fighter.lua_state_agent, 33.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("tail8"), 21.0, 305, 70, 0, 40, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail8"), 21.0, 305, 70, 0, 40, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 3, 0, Hash40::new("tail7"), 12.0, 361, 70, 0, 70, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 4, 0, Hash40::new("tail5"), 5.0, 361, 70, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 5, 0, Hash40::new("tail3"), 5.0, 361, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        // ATTACK(fighter, 6, 0, Hash40::new("tail1"), 5.0, 361, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        // HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_XLU);
        // HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("tail8"), 12.0, 361, 70, 0, 40, 3.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 2, 0, Hash40::new("tail8"), 12.0, 361, 70, 0, 40, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 4.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_BOUNCE[entry_id] = false;
        // HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_NORMAL);
        // HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 55.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        RIDLEY_FLAG_SPECIAL_LW_ENABLE_LANDING[entry_id] = false;
    }
}
#[acmd_script( agent = "ridley", script = "expression_specialairlwpogo", category = ACMD_EXPRESSION )]
unsafe fn down_special_pogo_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_specialairlwpogo", category = ACMD_SOUND )]
unsafe fn down_special_pogo_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_l01"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_l02"));
    }
}
#[acmd_script( agent = "ridley", script = "effect_specialairlwpogo", category = ACMD_EFFECT )]
unsafe fn down_special_pogo_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
//landing pogo scripts
#[acmd_script( agent = "ridley", script = "game_speciallwpogolanding", category = ACMD_GAME )]
unsafe fn down_special_pogo_landing_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    FT_MOTION_RATE(fighter, 1.5);
    frame(fighter.lua_state_agent, 30.0);
    FT_MOTION_RATE(fighter, 1.0);
}
#[acmd_script( agent = "ridley", script = "expression_speciallwpogolanding", category = ACMD_EXPRESSION )]
unsafe fn down_special_pogo_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_speciallwpogolanding", category = ACMD_SOUND )]
unsafe fn down_special_pogo_landing_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        // SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_ridley_special_l02"), 0);
        // STOP_SE(fighter, Hash40::new("se_ridley_special_l02"));
        PLAY_SE(fighter, Hash40::new("se_ridley_landing03"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_attackair_f01"));
    }
}
#[acmd_script( agent = "ridley", script = "effect_speciallwpogolanding", category = ACMD_EFFECT )]
unsafe fn down_special_pogo_landing_eff(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(fighter.module_accessor, Hash40::new("sys_sp_flash"), false);
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}
////
//////

////adding air grab
#[status_script(agent = "ridley", status = FIGHTER_STATUS_KIND_AIR_LASSO, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn air_grab_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    RIDLEY_FLAG_SPECIAL_LW_IS_GRAB[entry_id] = true;
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    return true.into()
}
////

////replaced down-air with tail swat
//down-air scripts
#[acmd_script( agent = "ridley", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn down_air_game(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 19.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 2, 0, Hash40::new("tail8"), 14.0, 361, 65, 0, 60, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail7"), 12.0, 361, 55, 0, 60, 5.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 0, 0, Hash40::new("tail4"), 12.0, 361, 55, 0, 60, 5.5, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        // HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_XLU);
        // HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    FT_MOTION_RATE(fighter, 2.0);
    if is_excute(fighter) {
        // HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_NORMAL);
        // HIT_NODE(fighter, Hash40::new("tail3"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    // frame(fighter.lua_state_agent, 26.0);
    // FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 31.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
#[acmd_script( agent = "ridley", script = "expression_attackairlw", category = ACMD_EXPRESSION )]
unsafe fn down_air_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("tail7"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_attackairlw", category = ACMD_SOUND )]
unsafe fn down_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_attackair_n01"));
    }
}
#[acmd_script( agent = "ridley", script = "effect_attackairlw", category = ACMD_EFFECT )]
unsafe fn down_air_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_tail_arc"), Hash40::new("top"), 0, 12, -5, 0, -60, 90, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
}
//landing down-air scripts
// #[acmd_script( agent = "ridley", script = "game_landingairlw", category = ACMD_GAME )]
// unsafe fn down_air_landing_game(fighter : &mut L2CAgentBase) {
// }
#[acmd_script( agent = "ridley", script = "expression_landingairlw", category = ACMD_EXPRESSION )]
unsafe fn down_air_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_landingairlw", category = ACMD_SOUND )]
unsafe fn down_air_landing_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_landing02"));
    }
}
// #[acmd_script( agent = "ridley", script = "effect_landingairlw", category = ACMD_EFFECT )]
// unsafe fn down_air_landing_eff(fighter : &mut L2CAgentBase) {
// }
////

//////buffing neutral special
//hold attack to explode
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn neutral_special_shoot_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM) >= WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_fire_num"))
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_explode") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_explode"), 0.0, 1.0, false, 0.0, false, false);
        }
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_NORMAL);
        fighter.sub_shift_status_main(L2CValue::Ptr(neutral_special_shoot_main_loop as *const () as _))
    }else {
        original!(fighter)
    }
}
pub unsafe fn neutral_special_shoot_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != StatusModule::prev_situation_kind(fighter.module_accessor) {
        air_to_ground_transition_status_func(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool()
        || fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
//hold
#[acmd_script( agent = "ridley", scripts = ["game_specialnhold", "game_specialairnhold"], category = ACMD_GAME )]
unsafe fn neutral_special_hold_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
}
//shoot
#[acmd_script( agent = "ridley", scripts = ["game_specialnshoot", "game_specialairnshoot"], category = ACMD_GAME )]
unsafe fn neutral_special_shoot_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
}
////new explode scripts
//grounded
#[acmd_script( agent = "ridley", script = "game_specialnexplode", category = ACMD_GAME )]
unsafe fn neutral_special_explode_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "ridley", script = "expression_specialnexplode", category = ACMD_EXPRESSION )]
unsafe fn neutral_special_explode_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_specialnexplode", category = ACMD_SOUND )]
unsafe fn neutral_special_explode_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_ridley_special_s02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}
#[acmd_script( agent = "ridley", script = "effect_specialnexplode", category = ACMD_EFFECT )]
unsafe fn neutral_special_explode_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}
//aerial
#[acmd_script( agent = "ridley", script = "game_specialairnexplode", category = ACMD_GAME )]
unsafe fn neutral_special_explode_air_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "ridley", script = "expression_specialairnexplode", category = ACMD_EXPRESSION )]
unsafe fn neutral_special_explode_air_exp(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "sound_specialairnexplode", category = ACMD_SOUND )]
unsafe fn neutral_special_explode_air_snd(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s01"));
        PLAY_SE(fighter, Hash40::new("vc_ridley_special_s02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_smash_s02"));
    }
}
#[acmd_script( agent = "ridley", script = "effect_specialairnexplode", category = ACMD_EFFECT )]
unsafe fn neutral_special_explode_air_eff(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}
////
//////

////buffing up-special
//adding cancel frames to up-special-landing
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn up_special_landing_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param;
    let cancel;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_LANDING_F) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_landing_f"), 0.0, 1.0, false, 0.0, false, false);
        param = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_f_frame"));
        cancel = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_hi_landing_f"), false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_landing_lw"), 0.0, 1.0, false, 0.0, false, false);
        param = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_lw_frame")) ;
        cancel = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_hi_landing_lw"), false);
    }
    let rate = cancel as f32/param as f32;
    MotionModule::set_rate(fighter.module_accessor, rate);

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_landing"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    let deccel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("deccel_x_on_landing"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, deccel, 0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(up_special_landing_main_loop as *const () as _))

}
pub unsafe fn up_special_landing_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    false.into()
}
//adding hit-box to wall and ceiling bonk
#[acmd_script( agent = "ridley", script = "game_specialairhiwallf", category = ACMD_GAME )]
unsafe fn up_special_wall_f_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, -5.0, 4.0, Some(0.0), Some(18.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}
#[acmd_script( agent = "ridley", script = "game_specialairhiwallb", category = ACMD_GAME )]
unsafe fn up_special_wall_b_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, -4.0, Some(0.0), Some(23.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
#[acmd_script( agent = "ridley", script = "game_specialairhiceil", category = ACMD_GAME )]
unsafe fn up_special_ceiling_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 355, 70, 0, 80, 6.5, 0.0, 19.0, -10.0, Some(0.0), Some(19.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
//removing special-fall from wall-bonk and adding decaying bounce hight
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn up_special_wall_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] += 1;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    //was in the decomp, looks like it doesn't do anything
    // if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT) {
    //     if PostureModule::lr(fighter.module_accessor) == -1.0 {
    //     }
    // }
    // if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT) {
    //     if PostureModule::lr(fighter.module_accessor) == 1.0 {
    //     }
    // }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall_b"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_wall"), 0.0, 1.0, false, 0.0, false, false);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_wall"))*-1.0;
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    let mut speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("speed_y_on_stop_wall"))
    /RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] as f32;
    if speed_y <  0.0{
        speed_y = 0.0;
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(up_special_wall_main_loop as *const () as _))

}
pub unsafe fn up_special_wall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    false.into()
}
////

//////buffing side-special
////fixing side-special landing lag to be relative to remaining earial end-lag
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn side_special_failure_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let cancel_frame = (FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_start"), false) -MotionModule::frame(fighter.module_accessor)) +WorkModule::get_param_int(fighter.module_accessor, hash40("landing_heavy_frame"), 0) as f32;
    if cancel_frame < 1.0 {
        RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME[entry_id] = 1.0;
    }else {
        RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME[entry_id] = cancel_frame;
    }

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_failure"), 0.0, 1.0, false, 0.0, false, false);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)*WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("failure_speed_x_mul"));

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);

    fighter.sub_shift_status_main(L2CValue::Ptr(side_special_failure_main_loop as *const () as _))
}
pub unsafe fn side_special_failure_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }else if RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME[entry_id] <= MotionModule::frame(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    false.into()
}
////fixed deceptive grab-boxes
#[acmd_script( agent = "ridley", scripts = ["game_specialsstart", "game_specialairsstart"], category = ACMD_GAME, low_priority )]
unsafe fn side_special_start(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(fighter.lua_state_agent, 19.0);
        FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 7.0, 6.0, 7.5, 5.5);
    }
    frame(fighter.lua_state_agent, 23.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
        // HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        CATCH(fighter, 0, Hash40::new("top"), 9.0, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        CATCH(fighter, 1, Hash40::new("top"), 7.0, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        CATCH(fighter, 2, Hash40::new("top"), 5.0, 0.0, 8.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 28.0);
    FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 37.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(fighter.module_accessor, false);
        // HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}
////
////decreased kbg 90->55 increased bkg 75->85
#[acmd_script( agent = "ridley", script = "game_specialsdragcliff", category = ACMD_GAME )]
unsafe fn side_special_drag_cliff_game(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 50, 55, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 50, 55, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 35, 17);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.5);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_REVERT_DEGREE);
    }
}
////
////adding early side-special drag cliff
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn side_special_drag_jump_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        //dissabling slide because buggy
        // let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        // RIDLEY_FLOAT_SPECIAL_S_DRAG_SLIDE_VEL_X[entry_id] = PostureModule::lr(fighter.module_accessor) * KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        // RIDLEY_FLAG_SPECIAL_S_DRAG_IS_SLIDE[entry_id] = true;
        fighter.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF.into(), false.into());
        return true.into()
    }else {
        original!(fighter)
    }
}
#[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn side_special_drag_jump_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF {
        original!(fighter)
    }else {
        0.into()
    }
}
// #[status_script(agent = "ridley", status = FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
// unsafe fn side_special_drag_cliff_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     let ret = original!(fighter);
//     if RIDLEY_FLAG_SPECIAL_S_DRAG_IS_SLIDE[entry_id] {
//         RIDLEY_FLAG_SPECIAL_S_DRAG_IS_SLIDE[entry_id] = false;
//         RIDLEY_INT_SPECIAL_CATCH_ID[entry_id] = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
//         MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_drag_slide"), 0.0, 1.0, false, 0.0, false, false);
//     }
//     ret
// }
// //new side-special drag slide scripts
// #[acmd_script( agent = "ridley", script = "game_specialsdragslide", category = ACMD_GAME )]
// unsafe fn side_special_drag_slide_game(fighter : &mut L2CAgentBase) {
//     let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     if is_excute(fighter) {
//         ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 270, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//         ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 50, 90, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
//     }
//     frame(fighter.lua_state_agent, 3.0);
//     if is_excute(fighter) {
//         SET_SPEED_EX(fighter, RIDLEY_FLOAT_SPECIAL_S_DRAG_SLIDE_VEL_X[entry_id], 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if is_excute(fighter) {
//         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_REVERT_DEGREE);
//     }
// }
// #[acmd_script( agent = "ridley", script = "expression_specialsdragslide", category = ACMD_EXPRESSION )]
// unsafe fn side_special_drag_slide_exp(fighter : &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 3.0);
//     if is_excute(fighter) {
//         // QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackl"), 0, false, 0);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
//     }
//     frame(fighter.lua_state_agent, 56.0);
//     if is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
//     }
// }
// #[acmd_script( agent = "ridley", script = "sound_specialsdragslide", category = ACMD_SOUND )]
// unsafe fn side_special_drag_slide_snd(fighter : &mut L2CAgentBase) {
//     if is_excute(fighter) {
//         PLAY_SE(fighter, Hash40::new("se_ridley_dash_stop"));
//     }
//     frame(fighter.lua_state_agent, 5.0);
//     if is_excute(fighter) {
//         PLAY_SE(fighter, Hash40::new("se_ridley_special_s04"));
//     }
//     frame(fighter.lua_state_agent, 35.0);
//     if is_excute(fighter) {
//         PLAY_SE(fighter, Hash40::new("vc_ridley_special_s01"));
//     }
// }
// #[acmd_script( agent = "ridley", script = "effect_specialsdragslide", category = ACMD_EFFECT )]
// unsafe fn side_special_drag_slide_eff(fighter : &mut L2CAgentBase) {
//     if is_excute(fighter) {
//         EFFECT_FOLLOW(fighter, Hash40::new("ridley_grabbing_catch"), Hash40::new("havel"), -1, 0, 0, 0, 0, 0, 1, true);
//     }
//     frame(fighter.lua_state_agent, 26.0);
//     if is_excute(fighter) {
//         EFFECT_OFF_KIND(fighter, Hash40::new("ridley_grabbing_catch"), false, true);
//     }
// }
////
//////

////adding quake effects
//landing
#[acmd_script( agent = "ridley", script = "expression_landingairhi", category = ACMD_EXPRESSION )]
unsafe fn up_air_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "expression_landingairf", category = ACMD_EXPRESSION )]
unsafe fn forward_air_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "expression_landingairb", category = ACMD_EXPRESSION )]
unsafe fn back_air_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "expression_landingairn", category = ACMD_EXPRESSION )]
unsafe fn neutral_air_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
#[acmd_script( agent = "ridley", script = "expression_landingheavy", category = ACMD_EXPRESSION )]
unsafe fn heavy_landing_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}
//down-smash
#[acmd_script( agent = "ridley", script = "expression_attacklw4", category = ACMD_EXPRESSION )]
unsafe fn down_smash_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        slope!(fighter,*MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}
////

//// added wind-box
#[acmd_script( agent = "ridley", scripts = ["game_appealhil", "game_appealhir"], category = ACMD_GAME, low_priority )]
unsafe fn up_taunt_game(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 0, 100, 22, 0, 14.0, 0.0, 12.0, 10.0, Some(0.0), Some(12.0), Some(-15.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 7, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
#[acmd_script( agent = "ridley", scripts = ["expression_appealhil", "expression_appealhil"], category = ACMD_EXPRESSION )]
unsafe fn up_taunt_exp(fighter : &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter, 0, 3, 110, 2, 1, 0, 15, 30, 30, 50);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_aerial"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}
////

////up-throw kills before down-throw
// increased kbg 120->125 decressed bkb 55->55
#[acmd_script( agent = "ridley", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn up_throw_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
         ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 89, 125, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
         macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("tail8"), 8.0, 89, 100, 0, 55, 4.5, 5.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL,   false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        ATTACK(fighter, 1, 0, Hash40::new("tail8"), 8.0, 89, 100, 0, 55, 4.5, -2.0, 1.0, 1.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 2, 28);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.8);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 0.0, y: 8.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
// decressed kbg 115->90 increased bkb 40->55
#[acmd_script( agent = "ridley", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn down_throw_game(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
         ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 74, 90, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
         ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 90, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, 4, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        CHECK_FINISH_CAMERA(fighter, 19, 2);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(FIGHTER_CUTIN_MANAGER, 1.2);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(FIGHTER_CUTIN_MANAGER, Vector3f{x: 3.0, y: -2.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
////


pub fn install() {
    smashline::install_agent_frames!(
        per_fighter_frame
    );
    smashline::install_status_scripts!(
        down_special_status_main,
        air_grab_status_main,
        neutral_special_shoot_status_main,
        up_special_landing_status_main,
        up_special_wall_status_main,
        side_special_failure_status_main,
        side_special_drag_jump_status_main,
        side_special_drag_jump_status_end//,
        // side_special_drag_cliff_status_main
    );
    smashline::install_acmd_scripts!(
        down_special_stab_game,
        down_special_finish_game,

        down_special_pogo_game,
        down_special_pogo_exp,
        down_special_pogo_snd,
        down_special_pogo_eff,

        down_special_pogo_landing_game,
        down_special_pogo_landing_exp,
        down_special_pogo_landing_snd,
        down_special_pogo_landing_eff,

        down_air_game,
        down_air_exp,
        down_air_snd,
        down_air_eff,

        // down_air_landing_game,
        down_air_landing_exp,
        down_air_landing_snd,
        // down_air_landing_eff,

        neutral_special_hold_game,
        neutral_special_shoot_game,

        neutral_special_explode_game,
        neutral_special_explode_exp,
        neutral_special_explode_snd,
        neutral_special_explode_eff,

        neutral_special_explode_air_game,
        neutral_special_explode_air_exp,
        neutral_special_explode_air_snd,
        neutral_special_explode_air_eff,

        up_special_wall_f_game,
        up_special_wall_b_game,
        up_special_ceiling_game,

        side_special_start,

        side_special_drag_cliff_game,

        // side_special_drag_slide_game,
        // side_special_drag_slide_exp,
        // side_special_drag_slide_snd,
        // side_special_drag_slide_eff,

        up_air_landing_exp,
        forward_air_landing_exp,
        back_air_landing_exp,
        neutral_air_landing_exp,
        heavy_landing_exp,
        down_smash_exp,

        up_taunt_game,
        up_taunt_exp,

        up_throw_game,
        down_throw_game
    );
}