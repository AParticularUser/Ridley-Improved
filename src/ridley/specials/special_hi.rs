use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::vars::*;


////status
//special-hi-landing
// adding cancel frames to up-special-landing
unsafe extern "C" fn special_hi_landing_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    let param;
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_LANDING_F) {
        motion = Hash40::new("special_hi_landing_f");
        param = hash40("landing_f_frame");
    }else {
        motion = Hash40::new("special_hi_landing_lw");
        param = hash40("landing_lw_frame");
    }
    let landing_frame = WorkModule::get_param_int(agent.module_accessor, hash40("param_special_hi"), param) as f32;
    let cancel = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, motion, false);
    let rate = cancel/landing_frame;
    MotionModule::change_motion(agent.module_accessor, motion, 0.0, rate, false, 0.0, false, false);
    //kinetic stuff
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(agent.lua_state_agent);
    let landing_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_landing"));
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x*landing_mul, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(agent.lua_state_agent);
    let deccel = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("deccel_x_on_landing"));
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, deccel, 0.0);
    sv_kinetic_energy::set_brake(agent.lua_state_agent);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_landing_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_landing_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //fall
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    false.into()
}
//special-hi-wall
// removing special-fall from wall-bonk and adding decaying bounce hight
unsafe extern "C" fn special_hi_wall_stop_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_WORK_INT_CHARGE_STATUS) == *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_CHARGE_B {
        motion = Hash40::new("special_air_hi_wall_b");
    }else {
        motion = Hash40::new("special_air_hi_wall");
    }
    MotionModule::change_motion(agent.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    //kinetic stuff
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(agent.lua_state_agent);
    let speed_x_mul = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("speed_x_mul_on_stop_wall"))*-1.0;
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, speed_x*speed_x_mul, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(agent.lua_state_agent);
    VarModule::inc_int(agent.module_accessor, instance::RIDLEY_INT_SPECIAL_HI_WALL_COUNT);
    let wall_bounce = VarModule::get_int(agent.module_accessor, instance::RIDLEY_INT_SPECIAL_HI_WALL_COUNT) as f32;
    let speed_y = WorkModule::get_param_float(agent.module_accessor, hash40("param_special_hi"), hash40("speed_y_on_stop_wall"));
    agent.clear_lua_stack();
    lua_args!(agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, (speed_y/wall_bounce).clamp(0.0, speed_y), 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(agent.lua_state_agent);
    WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    agent.sub_shift_status_main(L2CValue::Ptr(special_hi_wall_stop_status_main_loop as *const () as _))
}
pub unsafe fn special_hi_wall_stop_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //landing
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return true.into()
    }
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    //control
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    false.into()
}
////motion
//adding hit-box to wall and ceiling bonk
unsafe extern "C" fn special_air_hi_wall_f_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, -5.0, 4.0, Some(0.0), Some(18.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}
unsafe extern "C" fn special_air_hi_wall_b_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 70, 0, 80, 6.5, 0.0, 0.0, -4.0, Some(0.0), Some(23.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}
unsafe extern "C" fn special_air_hi_ceil_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 355, 70, 0, 80, 6.5, 0.0, 19.0, -10.0, Some(0.0), Some(19.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //special-hi-landing
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_LANDING, special_hi_landing_status_main);
    //special-hi-wall
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL, special_hi_wall_stop_status_main);
    ////motion
    agent.game_acmd("game_specialairhiwallf", special_air_hi_wall_f_game, Priority::High);
    agent.game_acmd("game_specialairhiwallb", special_air_hi_wall_b_game, Priority::High);
    agent.game_acmd("game_specialairhiceil", special_air_hi_ceil_game, Priority::High);
}