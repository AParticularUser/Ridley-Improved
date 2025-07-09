use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


////status
//special-lw-stab
unsafe extern "C" fn special_lw_stab_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_POGO.into(), false.into());
        return true.into()
    }
    VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
    WorkModule::set_int64(agent.module_accessor, hash40("special_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    let motion;
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        motion = Hash40::new("special_lw_stab");
    }else {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        motion = Hash40::new("special_air_lw_stab");
    }
    MotionModule::change_motion(agent.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH);
    VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD);
    agent.sub_shift_status_main(L2CValue::Ptr(special_lw_stab_status_main_loop as *const () as _))
}
pub unsafe fn special_lw_stab_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) 
    && !VarModule::is_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD) {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        let motion = MotionModule::motion_kind(agent.module_accessor);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, Hash40::new_raw(motion), false);
        if MotionModule::frame(agent.module_accessor) >= cancel_frame {
            if agent.sub_wait_ground_check_common(false.into()).get_bool()
            || agent.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    //hit
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_TO_FINISH) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH.into(), false.into());
        return true.into()
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        let motion;
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            motion = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
        }
        MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0);
    }
    //hold
    if VarModule::is_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD) {
        if !ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        || MotionModule::is_end(agent.module_accessor) {
            VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD);
            //change back to stab
            if MotionModule::motion_kind(agent.module_accessor) == hash40("special_lw_stab_hold") 
            || MotionModule::motion_kind(agent.module_accessor) == hash40("special_air_lw_stab_hold") {
                WorkModule::set_int64(agent.module_accessor, hash40("special_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
                WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_stab") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
                let motion;
                if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                    motion = Hash40::new("special_lw_stab");
                }else {
                    motion = Hash40::new("special_air_lw_stab");
                }
                let frame = VarModule::get_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_STAB_PREV_MOTION_FRAME);
                MotionAnimcmdModule::flush_current_motion(agent.module_accessor);
                MotionModule::change_motion(agent.module_accessor, motion, frame, 1.0, false, 0.0, true, false);
            }
            //turn around on release
            let lr = PostureModule::lr(agent.module_accessor);
            let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
            let stick_x = ControlModule::get_stick_x(agent.module_accessor);
            if stick_x*lr <= -stick_x_tilt {
                PostureModule::reverse_lr(agent.module_accessor);
                PostureModule::update_rot_y_lr(agent.module_accessor);
            }
        //change to hold
        }else if MotionModule::motion_kind(agent.module_accessor) == hash40("special_lw_stab") 
        || MotionModule::motion_kind(agent.module_accessor) == hash40("special_air_lw_stab") {
            WorkModule::set_int64(agent.module_accessor, hash40("special_lw_stab_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
            WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_stab_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
            let motion;
            if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                motion = Hash40::new("special_lw_stab_hold");
            }else {
                motion = Hash40::new("special_air_lw_stab_hold");
            }
            VarModule::set_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_STAB_PREV_MOTION_FRAME, MotionModule::frame(agent.module_accessor));
            let end_frame = MotionModule::end_frame_from_hash(agent.module_accessor, Hash40::new("special_lw_stab_hold"));
            let rate = end_frame/(param::RIDLEY_INT_SPECIAL_LW_STAB_HOLD_FRAME as f32);
            MotionModule::change_motion(agent.module_accessor, motion, 0.0, rate, false, 0.0, false, false);
        }
    }
    false.into()
}
//special-lw-finish
unsafe extern "C" fn special_lw_finish_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        motion = Hash40::new("special_lw_finish");
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }else {
        motion = Hash40::new("special_air_lw_finish");
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
        let max_speed_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
        sv_kinetic_energy!(reset_energy, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
        sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_speed_y);
        sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_speed_y);
    }
    MotionModule::change_motion(agent.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    AreaModule::enable_area(agent.module_accessor, *FIGHTER_AREA_KIND_BODY, false, -1);
    let unk = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_lw"), 0x16ad221e35);
    WorkModule::set_int(agent.module_accessor, unk, *FIGHTER_RIDLEY_INSTANCE_WORK_ID_INT_DISABLE_SPECIAL_LW_FINISH_COUNT);
    WorkModule::set_int(agent.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_UNIQ, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW_FINISH);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_NORMAL_GRAVITY);
    agent.sub_shift_status_main(L2CValue::Ptr(special_lw_finish_status_main_loop as *const () as _))
}
pub unsafe fn special_lw_finish_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        let motion;
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            motion = Hash40::new("special_lw_finish");
        }else {
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_NORMAL_GRAVITY) {
                KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                let accel_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_lw"), hash40("accel_y"));
                let max_speed_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_lw"), hash40("max_speed_y"));
                sv_kinetic_energy!(set_accel, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -accel_y);
                sv_kinetic_energy!(set_limit_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_speed_y);
                sv_kinetic_energy!(set_stable_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, max_speed_y);
            }else {
                KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
            motion = Hash40::new("special_air_lw_finish");
        }
        MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, motion, -1.0, 1.0, 0.0);
    }
    //gravity
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_ENABLE_GRAVITY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_NORMAL_GRAVITY);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    //one frame after throw
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW_FINISH) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW_FINISH);
        if LinkModule::is_linked(agent.module_accessor, *LINK_NO_CAPTURE) {
            LinkModule::send_event_nodes(agent.module_accessor, *LINK_NO_CAPTURE, Hash40::new_raw(0x15f8f946a4), 0);
        }
        let capture_id = VarModule::get_int(agent.module_accessor, status::RIDLEY_INT_SPECIAL_LW_FINISH_CAPTURE_ID);
        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
        SoundModule::stop_se(capture_boma, Hash40::new("se_common_step_jump"), 0); 
    }
    //throw
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_LW_FLAG_THROW_FINISH);
        if LinkModule::is_linked(agent.module_accessor, *LINK_NO_CAPTURE) {
            let capture_id = LinkModule::get_node_object_id(agent.module_accessor, *LINK_NO_CAPTURE);
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            if StatusModule::situation_kind(capture_boma) != *SITUATION_KIND_GROUND {
                StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
                let hit_stop = StopModule::get_hit_stop_real_frame(capture_boma) as i32;
                ShakeModule::req(capture_boma, Hash40::new("damage_ground"), hit_stop, false, &Vector2f{x:0.0, y:0.0}, 1.0, 0.0, false, false);
                VarModule::set_int(agent.module_accessor, status::RIDLEY_INT_SPECIAL_LW_FINISH_CAPTURE_ID, capture_id as i32);
            }else {
                AttackModule::hit_absolute_joint(agent.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
            }
        }
        GroundModule::set_shape_flag(agent.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FIX as u16, false);
        AreaModule::enable_area(agent.module_accessor, *FIGHTER_AREA_KIND_BODY, true, -1);
    }
    false.into()
}
////motion
//special-lw-stab
unsafe extern "C" fn special_lw_stab_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_STAB_HOLD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(30.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, -20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 45.0, 25, 10, 0, 70, 2.2, 0.0, 7.0, 24.5, Some(0.0), Some(7.0), Some(30.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, -20, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 0.4);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(30.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 50, 0, 30, 2.2, 0.0, 7.0, 8.0, Some(0.0), Some(7.0), Some(31.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn special_lw_stab_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8, 18, 0, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
unsafe extern "C" fn special_air_lw_stab_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, 8, 18, 0, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
unsafe extern "C" fn special_lw_stab_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}
unsafe extern "C" fn special_lw_stab_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //special-lw-stab
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_stab_status_main);
    //special-lw-finish
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_FINISH, special_lw_finish_status_main);
    ////motion
    //special-lw-stab
    agent.game_acmd("game_speciallwstab", special_lw_stab_game, Priority::High);
    agent.effect_acmd("effect_speciallwstab", special_lw_stab_eff, Priority::High);
    agent.sound_acmd("sound_speciallwstab", special_lw_stab_snd, Priority::High);
    agent.expression_acmd("expression_speciallwstab", special_lw_stab_exp, Priority::High);
    //special-air-lw-stab
    agent.game_acmd("game_specialairlwstab", special_lw_stab_game, Priority::High);
    agent.effect_acmd("effect_specialairlwstab", special_air_lw_stab_eff, Priority::High);
    agent.sound_acmd("sound_specialairlwstab", special_lw_stab_snd, Priority::High);
    agent.expression_acmd("expression_specialairlwstab", special_lw_stab_exp, Priority::High);
}