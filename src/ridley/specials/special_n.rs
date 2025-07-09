use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::{
    status_kind_ex::*,
    vars::*
};


////status
//special-n
// skewer input
unsafe extern "C" fn special_n_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        VarModule::on_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
        agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_SPECIAL_N)(agent)
}
unsafe extern "C" fn special_n_status_exec(agent: &mut L2CFighterCommon) -> L2CValue {
    let stick_x_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let stick_y_tilt = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && stick_x.abs() < stick_x_tilt 
    && stick_y.abs() < stick_y_tilt {
        VarModule::on_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_LW_IS_SKEWER);
        agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
    }
    0.into()
}
//special-n-charge
// press attack to cancel into f-smash
unsafe extern "C" fn special_n_charge_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let end_frame = MotionModule::end_frame_from_hash(agent.module_accessor, Hash40::new("special_n_hold"));
    let max_charge_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_charge_frame")) as f32;
    let rate = end_frame/max_charge_frame;
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n_hold"), 0.0, rate, false, 0.0, false, false);
    }else {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_n_hold"), 0.0, rate, false, 0.0, false, false);
    }
    WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    WorkModule::set_int(agent.module_accessor, 0, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    HitModule::set_status_joint(agent.module_accessor, Hash40::new("virtualweakpoint"), HitStatus(*HIT_STATUS_NORMAL), 0);
    agent.sub_shift_status_main(L2CValue::Ptr(special_n_charge_status_main_loop as *const () as _))
}
pub unsafe fn special_n_charge_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //attack
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK.into(), false.into());
        return true.into()
    }
    //end
    if MotionModule::is_end(agent.module_accessor) 
    || ControlModule::check_button_off(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return true.into()
    }
    //air/ground transition
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != agent.global_table[global_table::PREV_SITUATION_KIND].get_i32() {
        let motion;
        if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            agent.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_FRONT));
            motion = Hash40::new("special_n_hold");
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            motion = Hash40::new("special_air_n_hold");
        }
        MotionModule::change_motion_inherit_frame_keep_rate(agent.module_accessor, motion, -1.0, 1.0, 0.0);
    }
    WorkModule::inc_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    //weak-point size
    let min_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("min_weak_size"));
    let max_weak_size = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_n"), hash40("max_weak_size"));
    let end_frame = MotionModule::end_frame(agent.module_accessor);
    let curr_frame = MotionModule::frame(agent.module_accessor);
    let weak_size = (max_weak_size-min_weak_size)*(curr_frame/end_frame);
    ModelModule::set_joint_scale(agent.module_accessor, Hash40::new("virtualweakpoint"), &Vector3f{x:weak_size, y:weak_size, z:weak_size});
    WorkModule::set_float(agent.module_accessor, weak_size, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_FLOAT_WEAK_SIZE);
    //charge level
    let max_charge_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_charge_frame")) as f32;
    let max_fire_num = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_fire_num")) as f32;
    let charge_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT) as f32;
    let fire_num = ((max_fire_num-1.0)*(charge_count/max_charge_frame))+1.0;
    WorkModule::set_int(agent.module_accessor, fire_num as i32, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_FIRE_NUM);
    false.into()
}
//special-n-shoot
unsafe extern "C" fn special_n_shoot_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK.into(), false.into());
        return true.into()
    }
    smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT)(agent)
}
//special-n-attack
// damage relative to charge level
unsafe extern "C" fn special_n_attack_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        agent.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        agent.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn special_n_attack_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_n_attack"), 0.0, 1.0, false, 0.0, false, false);
    }else {
        GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_n_attack"), 0.0, 1.0, false, 0.0, false, false);
    }
    let attack_up = WorkModule::get_param_float(agent.module_accessor, hash40("attack_s4_smash_hold_attack_up"), 0);
    let max_charge_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_n"), hash40("max_charge_frame")) as f32;
    let charge_count = WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT) as f32;
    let power_mul = ((attack_up-1.0)*(charge_count/max_charge_frame))+1.0;
    AttackModule::set_power_mul_status(agent.module_accessor, power_mul);
    agent.sub_shift_status_main(L2CValue::Ptr(special_n_attack_status_main_loop as *const () as _))
}
pub unsafe fn special_n_attack_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
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
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_FRONT));
            motion = Hash40::new("special_n_attack");
        }else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            agent.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            motion = Hash40::new("special_air_n_attack");
        }
        MotionModule::change_motion_inherit_frame(agent.module_accessor, motion, -1.0, 1.0, 0.0, false, false);
    }
    false.into()

}
unsafe extern "C" fn special_n_attack_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
////motion
//special-n-charge
unsafe extern "C" fn special_n_hold_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
}
//special-n-shoot
unsafe extern "C" fn special_n_shoot_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("mouth1"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("virtualweakpoint"), *HIT_STATUS_OFF);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_N_FLAG_SHOOT);
    }
}
//special-n-attack
unsafe extern "C" fn special_n_attack_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 20.0, 361, 80, 0, 58, 9.0, 0.0, 8.0, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn special_n_attack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}
unsafe extern "C" fn special_air_n_attack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, 15.5, -3.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -9, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_smash_bomb"), Hash40::new("top"), 0, 8.5, 15, 0, 0, 0, 1.2, true);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_mouth_fire"), Hash40::new("top"), 0, 11, 8.5, 0, 0, 0, 1, true);
    }
}
unsafe extern "C" fn special_n_attack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("vc_ridley_special_s02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_smash_s02"));
    }
}
unsafe extern "C" fn special_n_attack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, 0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //special-n
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_status_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_status_exec);
    //special-n-charge
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE, special_n_charge_status_main);
    //special-n-shoot
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_status_main);
    //special-n-attack
    agent.status(Pre, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK, special_n_attack_status_pre);
    agent.status(Main, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK, special_n_attack_status_main);
    agent.status(End, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_ATTACK, special_n_attack_status_end);
    ////motion
    //special-n-hold
    agent.game_acmd("game_specialnhold", special_n_hold_game, Priority::High);
    agent.game_acmd("game_specialairnhold", special_n_hold_game, Priority::High);
    //special-n-shoot
    agent.game_acmd("game_specialnshoot", special_n_shoot_game, Priority::High);
    agent.game_acmd("game_specialairnshoot", special_n_shoot_game, Priority::High);
    //special-n-attack
    agent.game_acmd("game_specialnattack", special_n_attack_game, Priority::High);
    agent.effect_acmd("effect_specialnattack", special_n_attack_eff, Priority::High);
    agent.sound_acmd("sound_specialnattack", special_n_attack_snd, Priority::High);
    agent.expression_acmd("expression_specialnattack", special_n_attack_exp, Priority::High);
    //special-air-n-attack
    agent.game_acmd("game_specialairnattack", special_n_attack_game, Priority::High);
    agent.effect_acmd("effect_specialairnattack", special_air_n_attack_eff, Priority::High);
    agent.sound_acmd("sound_specialairnattack", special_n_attack_snd, Priority::High);
    agent.expression_acmd("expression_specialairnattack", special_n_attack_exp, Priority::High);
}