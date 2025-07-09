use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::{
    status_kind_ex::*,
    vars::*
};


//air-lasso
unsafe extern "C" fn air_lasso_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    agent.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
    true.into()
}
//special-s-start
unsafe extern "C" fn special_s_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_AIR_LASSO {
        VarModule::on_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH);
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        agent.sub_shift_status_main(L2CValue::Ptr(air_catch_status_main_loop as *const () as _))
    }else {
        VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH);
        smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_SPECIAL_S)(agent)
    }
}
pub unsafe fn air_catch_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    //landing
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return true.into()
    }
    false.into()
}
unsafe extern "C" fn special_s_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH {
        VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH);
    }
    smashline::original_status(End, agent, *FIGHTER_STATUS_KIND_SPECIAL_S)(agent)
}
//special-s-catch
unsafe extern "C" fn special_s_catch_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL.into(), false.into());
        true.into()
    }else {
        smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH)(agent)
    }
}
unsafe extern "C" fn special_s_catch_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL {
        VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH);
        smashline::original_status(End, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH)(agent)
    }else {
        0.into()
    }
}
//special-s-fall
unsafe extern "C" fn special_s_fall_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
        agent.sub_shift_status_main(L2CValue::Ptr(special_s_fall_status_main_loop as *const () as _)) 
    }else {
        smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL)(agent)
    }
}
pub unsafe fn special_s_fall_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //throw
    if ControlModule::check_button_on(agent.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP.into(), false.into());
        return true.into()
    }
    false.into()
}
unsafe extern "C" fn special_s_fall_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    if agent.global_table[global_table::STATUS_KIND].get_i32() != *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP {
        VarModule::off_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH);
        smashline::original_status(End, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL)(agent);
    }
    0.into()
}
//special-s-fall-jump
unsafe extern "C" fn special_s_fall_jump_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(agent.module_accessor, instance::RIDLEY_FLAG_SPECIAL_S_IS_AIR_CATCH) {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_s_fall_jump"), 0.0, 1.0, false, 0.0, false, false);
        agent.sub_shift_status_main(L2CValue::Ptr(special_s_fall_jump_status_main_loop as *const () as _))
    }else {
        smashline::original_status(Main, agent, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP)(agent)
    }
}
pub unsafe fn special_s_fall_jump_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_air_check_fall_common().get_bool() {
            return true.into()
        }
    }
    // //landing
    // if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
    //     agent.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    //     return true.into()
    // }
    false.into()
}


pub fn install(agent: &mut smashline::Agent) {
    //air-lasso
    agent.status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, air_lasso_status_main);
    //special-s-start
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_status_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_status_end);
    //special-s-catch
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH, special_s_catch_status_main);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_CATCH, special_s_catch_status_end);
    //special-s-fall
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL, special_s_fall_status_main);
    agent.status(End, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL, special_s_fall_status_end);
    //special-s-fall-jump
    agent.status(Main, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP, special_s_fall_jump_status_main);
}