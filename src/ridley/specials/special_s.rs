use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::vars::*;


////status
//side-special-landing
// changed side-special landing-lag to be relative to remaining earial end-lag
unsafe extern "C" fn special_s_failure_status_main(agent: &mut L2CFighterCommon) -> L2CValue  {
    let curr_frame = MotionModule::frame(agent.module_accessor);
    let special_air_s_frame = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, Hash40::new("special_s_start"), false);
    let cancel_frame = special_air_s_frame -curr_frame;
    VarModule::set_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME, cancel_frame);

    MotionModule::change_motion(agent.module_accessor, Hash40::new("special_s_failure"), 0.0, 1.0, false, 0.0, false, false);

    agent.clear_lua_stack();
    lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_STOP);
    let speed_x = sv_kinetic_energy::get_speed_x(agent.lua_state_agent)*WorkModule::get_param_float(agent.module_accessor, hash40("param_special_s"), hash40("failure_speed_x_mul"));

    agent.clear_lua_stack();
    lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(agent.lua_state_agent);

    agent.clear_lua_stack();
    lua_args!(agent, FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, 0.0);
    sv_kinetic_energy::set_speed(agent.lua_state_agent);

    agent.sub_shift_status_main(L2CValue::Ptr(special_s_failure_status_main_loop as *const () as _))
}
pub unsafe fn special_s_failure_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    if VarModule::get_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_S_FAILURE_CANCEL_FRAME) <= MotionModule::frame(agent.module_accessor) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
    false.into()
}
//side-special-jump
// adding early side-special drag cliff
unsafe extern "C" fn special_s_drag_jump_status_main(agent: &mut L2CFighterCommon) -> L2CValue  {
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    || ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF.into(), false.into());
        return true.into()
    }else {
        smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP)(agent)
    }
}
unsafe extern "C" fn special_s_drag_jump_status_end(agent: &mut L2CFighterCommon) -> L2CValue  {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_CLIFF {
        smashline::original_status(End, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP)(agent)
    }else {
        0.into()
    }
}
////motion
//special-s-start
// fixed deceptive grab-boxes
unsafe extern "C" fn special_s_start_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 6.0, 7.5, 5.5);
    }
    frame(agent.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 9.0, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 7.0, 0.0, 10.0, 18.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 8.0, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 25.0); 
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}
//special-s-cliff
// decreased kbg 90->55 increased bkb 75->85
unsafe extern "C" fn special_s_drag_cliff_game(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 50, 55, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 50, 55, 0, 85, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 35, 17);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 3.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_REVERT_DEGREE);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //special-s-landing
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE, special_s_failure_status_main);
    //special-s-jump
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, special_s_drag_jump_status_main);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, special_s_drag_jump_status_end);
    ////motion
    //special-s-start
    agent.game_acmd("game_specialsstart", special_s_start_game, Priority::High);
    agent.game_acmd("game_specialairsstart", special_s_start_game, Priority::High);
    //special-s-cliff
    agent.game_acmd("game_specialsdragcliff", special_s_drag_cliff_game, Priority::High);
}