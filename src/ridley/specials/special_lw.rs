use crate::imports::*;
use crate::common::consts::*;
use crate::ridley::consts::{
    status_kind_ex::*,
    vars::*,
    *
};


////status
//special-lw-pogo
// changed down-special to tail-pogo
unsafe extern "C" fn special_lw_pogo_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}
unsafe extern "C" fn special_lw_pogo_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    if agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::RIDLEY_FLOAT_SPECIAL_LW_POGO_JUMP_SPEED_Y);
    }else {
        MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_lw_pogo"), 0.0, 1.0, false, 0.0, false, false);
    }
    VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE);
    VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_ENABLE_LANDING);
    agent.sub_shift_status_main(L2CValue::Ptr(special_lw_pogo_status_main_loop as *const () as _))
}
pub unsafe fn special_lw_pogo_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) 
    && agent.sub_air_check_fall_common().get_bool() {
        return true.into()
    }
    //landing
    if agent.global_table[global_table::STATUS_FRAME].get_i32() >= 3 
    && agent.global_table[global_table::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if VarModule::is_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_ENABLE_LANDING) {
            agent.change_status(FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_LANDING.into(), false.into());
        }else {
            agent.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        return true.into()
    }
    //pogo bounce
    let v3f_tail_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(agent.module_accessor, Hash40::new("tail8"), v3f_tail_pos, false);
    let pos_x_global = PostureModule::pos_x(agent.module_accessor);
    let pos_y_global = PostureModule::pos_y(agent.module_accessor);
    let offset_x_prev = VarModule::get_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_X);
    let offset_y_prev = VarModule::get_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_Y);
    //save current tail pos relative to fighter
    VarModule::set_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_X, v3f_tail_pos.x-pos_x_global);
    VarModule::set_float(agent.module_accessor, status::RIDLEY_FLOAT_SPECIAL_LW_POGO_CHECK_PREV_Y, v3f_tail_pos.y-pos_y_global);
    if VarModule::is_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE) {
        let lr = PostureModule::lr(agent.module_accessor);
        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
        let ground_hit_rot = &mut Vector2f{x: 0.0, y: 0.0};
        let pos_start_x = offset_x_prev+pos_x_global;
        let pos_start_y = offset_y_prev+pos_y_global;
        let offset_end_x = (v3f_tail_pos.x-pos_start_x) +6.0*lr;
        let offset_end_y = (v3f_tail_pos.y-pos_start_y) -9.0;
        if GroundModule::ray_check_hit_pos_normal(
            agent.module_accessor,
            &Vector2f{x:pos_start_x, y: pos_start_y},
            &Vector2f{x: offset_end_x, y: offset_end_y},
            ground_hit_pos,
            ground_hit_rot,
            true
        ) == 1 {
            VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE);
            EffectModule::req(agent.module_accessor, Hash40::new("sys_crown"), &Vector3f{x: ground_hit_pos.x, y: ground_hit_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: -ground_hit_rot.x}, 0.2, 0, 0, false, 0);
            EffectModule::req(agent.module_accessor, Hash40::new("sys_quake"), &Vector3f{x: ground_hit_pos.x, y: ground_hit_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: -ground_hit_rot.x}, 0.5, 0, 0, false, 0);
            SoundModule::play_se(agent.module_accessor, Hash40::new("se_ridley_special_h03"), true, false, false, false, enSEType(0));
            CameraModule::req_quake(agent.module_accessor, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
            //calculates bounce hight based off distance from ground
            let bounce_speed_max = param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_MAX;
            let bounce_speed_min = param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_MIN;
            let bounce_check_y_max = param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_CHECK_Y_MAX;
            let speed_y = bounce_speed_max-(((pos_y_global-ground_hit_pos.y).clamp(0.0, bounce_check_y_max)/bounce_check_y_max)*(bounce_speed_max-bounce_speed_min));
            let speed_x = lr*KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x*param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_X_MUL);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
        //hitting a hurt-box gives set momentum
        }else if AttackModule::is_infliction(agent.module_accessor, *COLLISION_KIND_MASK_HIT) {
            VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE);
            let speed_x = lr*KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_MOTION, speed_x*param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_X_MUL);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, param::RIDLEY_FLOAT_SPECIAL_LW_POGO_BOUNCE_SPEED_Y_HIT);
        }
    }


    false.into()

}
unsafe extern "C" fn special_lw_pogo_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
//special-lw-landing
unsafe extern "C" fn special_lw_landing_status_pre(agent: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW) as u64,
        0,
        0,
        0
    );
    0.into()
}
unsafe extern "C" fn special_lw_landing_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(agent.module_accessor, Hash40::new("special_lw_landing"), 0.0, 1.0, false, 0.0, false, false);
    agent.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    agent.sub_shift_status_main(L2CValue::Ptr(special_lw_landing_status_main_loop as *const () as _))
}
pub unsafe fn special_lw_landing_status_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    //end
    if MotionModule::is_end(agent.module_accessor) {
        agent.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    //cancel
    if CancelModule::is_enable_cancel(agent.module_accessor) 
    && agent.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into()
    }
    //fall
    if agent.global_table[global_table::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        agent.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    false.into()
}
unsafe extern "C" fn special_lw_landing_status_end(_agent: &mut L2CFighterCommon) -> L2CValue {0.into()}
////motion
//special-lw-pogo
unsafe extern "C" fn special_lw_pogo_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_ENABLE_LANDING)
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE)
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("tail8"), 21.0, 305, 70, 0, 40, 3.0, 8.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail8"), 21.0, 305, 70, 0, 40, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 8.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 3, 0, Hash40::new("tail7"), 12.0, 361, 70, 0, 70, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 4, 0, Hash40::new("tail5"), 5.0, 361, 70, 0, 70, 3.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 5, 0, Hash40::new("tail3"), 5.0, 361, 70, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("tail8"), 12.0, 361, 70, 0, 40, 3.0, 7.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail8"), 12.0, 361, 70, 0, 40, 3.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_CHECK_BOUNCE);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, status::RIDLEY_FLAG_SPECIAL_LW_POGO_ENABLE_LANDING)
    }
}
unsafe extern "C" fn special_lw_pogo_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
unsafe extern "C" fn special_air_lw_pogo_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("tail8"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_flare"), Hash40::new("tail8"), 0, 0, 0, 0, 180, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ridley_death_stab_line"), Hash40::new("top"), 0, -7, -8, 60, 0, 0, 0.9, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("tail8"), 10, -0.0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
    }
}
unsafe extern "C" fn special_lw_pogo_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_jump01"));
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}
unsafe extern "C" fn special_air_lw_pogo_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l01"));
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_special_l02"));
    }
}
unsafe extern "C" fn special_lw_pogo_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_piercel"), 0);
    }
}
//special-lw-landing
unsafe extern "C" fn special_lw_landing_game(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.4);
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}
unsafe extern "C" fn special_lw_landing_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ridley_death_stab_flare"), false, true);
        EffectModule::set_visible_kind(agent.module_accessor, Hash40::new("ridley_death_stab_line"), false);
        EffectModule::set_visible_kind(agent.module_accessor, Hash40::new("sys_sp_flash"), false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), -18, 0, 0, 0, 0, 0, 0.3, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn special_lw_landing_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_landing03"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_attackair_f01"));
    }
}
unsafe extern "C" fn special_lw_landing_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, 0);
    }
}


pub fn install(agent: &mut smashline::Agent) {
    ////status
    //special-lw-pogo
    agent.status(Pre, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_POGO, special_lw_pogo_status_pre);
    agent.status(Main, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_POGO, special_lw_pogo_status_main);
    agent.status(End, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_POGO, special_lw_pogo_status_end);
    //special-lw-landing
    agent.status(Pre, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_LANDING, special_lw_landing_status_pre);
    agent.status(Main, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_LANDING, special_lw_landing_status_main);
    agent.status(End, FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_LW_LANDING, special_lw_landing_status_end);
    ////motion
    //special-lw-pogo
    agent.game_acmd("game_speciallwpogo", special_lw_pogo_game, Priority::High);
    agent.effect_acmd("effect_speciallwpogo", special_lw_pogo_eff, Priority::High);
    agent.sound_acmd("sound_speciallwpogo", special_lw_pogo_snd, Priority::High);
    agent.expression_acmd("expression_speciallwpogo", special_lw_pogo_exp, Priority::High);
    //special-air-lw-pogo
    agent.game_acmd("game_specialairlwpogo", special_lw_pogo_game, Priority::High);
    agent.effect_acmd("effect_specialairlwpogo", special_air_lw_pogo_eff, Priority::High);
    agent.sound_acmd("sound_specialairlwpogo", special_air_lw_pogo_snd, Priority::High);
    agent.expression_acmd("expression_specialairlwpogo", special_lw_pogo_exp, Priority::High);
    //special-lw-landing
    agent.game_acmd("game_speciallwlanding", special_lw_landing_game, Priority::High);
    agent.effect_acmd("effect_speciallwlanding", special_lw_landing_eff, Priority::High);
    agent.sound_acmd("sound_speciallwlanding", special_lw_landing_snd, Priority::High);
    agent.expression_acmd("expression_speciallwlanding", special_lw_landing_exp, Priority::High);
}