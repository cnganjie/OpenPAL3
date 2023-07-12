use std::{cell::RefCell, rc::Rc};

use imgui::{Condition, MouseButton};

use crate::{
    as_params,
    scripting::angelscript::{
        not_implemented, ContinuationState, GlobalFunctionContinuation, GlobalFunctionState,
        ScriptGlobalContext, ScriptGlobalFunction, ScriptVm,
    },
};

use super::app_context::Pal4AppContext;

type Pal4FunctionState = GlobalFunctionState<Pal4AppContext>;
type Pal4Continuation = GlobalFunctionContinuation<Pal4AppContext>;

pub fn create_script_vm(app_context: Pal4AppContext) -> ScriptVm<Pal4AppContext> {
    let module = app_context.loader.load_script_module("script").unwrap();
    ScriptVm::new(
        Rc::new(RefCell::new(create_context())),
        module,
        0,
        app_context,
    )
}

pub fn create_context() -> ScriptGlobalContext<Pal4AppContext> {
    let mut context = ScriptGlobalContext::new();

    context.register_function(ScriptGlobalFunction::new("giIMMBegin", Box::new(imm_begin)));
    context.register_function(ScriptGlobalFunction::new("giIMMEnd", Box::new(imm_end)));
    context.register_function(ScriptGlobalFunction::new("giNewGame", Box::new(new_game)));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraCtrlYPR",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraCtrlDist",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraCtrlYPRD",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraGetDist",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraGetYaw",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraGetPitch",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraGetRoll",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaLoad",
        Box::new(arena_load),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaReady",
        Box::new(arena_ready),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaReadyRestore",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaHint",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaComeFromHere",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetLeader",
        Box::new(player_set_leader),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerAttachCollision",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerDetachCollision",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerLock",
        Box::new(player_lock),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerUnLock",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetNpcVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcCreate",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcDelete",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSystemExchange",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMonsterStopPursuit",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraSetMode",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetGoodsOpenCondition",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcPauseBeh",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcResumeBeh",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giOpenWeather",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCloseWeather",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetMinimapExpmode",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetRandnum",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetTempGameState",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlushTailYAngle",
        Box::new(not_implemented),
    ));

    context.register_function(ScriptGlobalFunction::new("giUnknown", Box::new(unknown)));

    context.register_function(ScriptGlobalFunction::new(
        "giAddCombatMonster",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giConfigCombatParam",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giConfigCombatBgm",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giConfigCombatCamera",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giStartCombat",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetObjectVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddProperty",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giDelProperty",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerInTeam",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerOutTeam",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOMTouch",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraSetCollide",
        Box::new(camera_set_collide),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraSeekToPlayer",
        Box::new(camera_seek_to_player),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraAutoSeek",
        Box::new(camera_auto_seek),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetAttr",
        Box::new(player_set_attr),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerGetLeader",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetPlayerLevel",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddPlayerEquip",
        Box::new(add_player_equip),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giOpenMovieFlag",
        Box::new(open_movie_flag),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddQuestComplatePercentage",
        Box::new(add_quest_complete_percentage),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddEquipment",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentSetVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetFullHP",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetFullMP",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetPlayerLevel",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGotoLogo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerUnHoldAct",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcUnHoldAct",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraSetDistOptEnable",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMonsterSetHide",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGameObjectSetResearch",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetPortrait",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giBGMConfigSetMusic",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giBGMConfigIsInArea",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptMusicMute",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptMusicPlay",
        Box::new(script_music_play),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptMusicStop",
        Box::new(script_music_stop),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaMusicStop",
        Box::new(arena_music_stop),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giWorldMapSetState",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetPuzzleGameResult",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAlwaysJump",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "gi2DSoundPlay",
        Box::new(dd_sound_play),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "gi2DSoundStop",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "gi2DSoundStopID",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCGEffPlay",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCGEffStop",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectPlay",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectPlayWithPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectPlayWithCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectPlayWithNPC",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectPlayWithOBJ",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectStopWithOBJ",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowHint",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerAddSkill",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMonsterSetVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerRandomPosition",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentRandomPosition",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEventVolumeVisible",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAllPlayerGarb2",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerGarb2",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAllPlayerGarb1",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerGarb1",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetVisibleObject",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetVisibleMonster",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCheckPackProperty",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGrantSystemUi",
        Box::new(grant_system_ui),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giOpenSystemUi",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGrantSmithSystem",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGrantMagicSystem",
        Box::new(grant_magic_system),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCheckMagicMastered",
        Box::new(check_magic_mastered),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSelectDialogAddItem",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSelectDialogGetLastSelect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBAttachToPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBAttachToCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBDetachFromPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBDetachFromCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectAttachToPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectAttachToCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectDetachFromPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectDetachFromCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectAttachToNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEffectDetachFromNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBAttachToNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBDetachFromNPC",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBSetPosition",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptClearCTXButCurrent",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddMoney",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPayMoney",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giHideGASkillObject",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowSignpost",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerResetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentSetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentResetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giLINGSHALegsInjured",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giLINGSHALegsHealing",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcSetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcResetEmotion",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetPropertyNumb",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentGetPosX",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentGetPosY",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentGetPosZ",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaGetName",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaGetArea",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giArenaSkillEnable",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowInnDialog",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetInnDialogResult",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerTakeARest",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giIsNightTime",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetPosRot1",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetPosRot2",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giStartUiTimer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerForbidenSkill",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetMoney",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSelectDialogSetDefaultSelect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowQuestDialog",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetQuestDialogResult",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giResetPlayerToJumpStart",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBReset",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCheckEquipInInventory",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giRemoveEquipment",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddPrescription",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEnableShadow",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddRoundTimes",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giTimeScript",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giTimeScriptTerminate",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giConfigCombatGroundCamera",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giAddPlayerFavor",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGetPalTestResult",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetMinimapLevel",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPetShow",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcAttachEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcDetachEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMstAttachEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMstDetachEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerHookEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerDetachEffect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCommonDialogGetLastSelect",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giConfigCombatVipMonster",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giEnableSTS",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giClearUiTimer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPauseUiTimer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giResumeUiTimer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giSetFullRage",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giUiTimerGetSaveData",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptMusicPause",
        Box::new(script_music_pause),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giScriptMusicResume",
        Box::new(script_music_resume),
    ));
    context.register_function(ScriptGlobalFunction::new("giWait", Box::new(wait)));
    context.register_function(ScriptGlobalFunction::new("giTalk", Box::new(talk)));
    context.register_function(ScriptGlobalFunction::new(
        "giRandTalkPush",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giRandTalk",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giRandTalkRelease",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giTalkWait",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerDoAction",
        Box::new(player_do_action),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerEndAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentDoAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentEndAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetPos",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentSetPos",
        Box::new(player_current_set_pos),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetRot",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetAng",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentSetAng",
        Box::new(player_current_set_ang),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerFaceToPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerSetDir",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerFaceToNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerWalkTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerRunTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentWalkTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerBackTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerBlendOut",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerBlendIn",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerFaceToCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCurrentPlayerFaceToNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerDoActionRepeat",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerEndActionRepeat",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerEndMove",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCurrentPlayerEndMove",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcWalkTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcRunTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcBackTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcDoAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcEndAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcSetPos",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcSetRot",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcSetDir",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcFaceToNpc",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcFaceToPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcBlendOut",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcBlendIn",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcFaceToCurrentPlayer",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcResetDir",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcDoActionRepeat",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcEndActionRepeat",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcEndMove",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNpcSetAng",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraPrepare",
        Box::new(camera_prepare),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraRunSingle",
        Box::new(camera_run_single),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraRunCircle",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giCameraWait",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashOutBlack",
        Box::new(flash_out_black),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashInBlack",
        Box::new(flash_in_black),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashOutWhite",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashInWhite",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashOutRed",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giFlashInRed",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayMovie",
        Box::new(play_movie),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giObjectDoAction",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giStartTradeSystem",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giStartPuzzleGame",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giStartJigsawGame",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giOBJBlendOut",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giOBJBlendIn",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMSTBlendOut",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giMSTBlendIn",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowCommonDialog",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowSelectDialog",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBMovment",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowTutorial",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowWorldMap",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGOBScale",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentFaceToGOB",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayerCurrentMovement",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowPoetry",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giNPCFlyTo",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giGotoLogoWait",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giShowCommonDialogInSelectMode",
        Box::new(not_implemented),
    ));
    context.register_function(ScriptGlobalFunction::new(
        "giPlayMovieFinal",
        Box::new(not_implemented),
    ));

    context
}

fn imm_begin(_name: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn imm_end(_: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn new_game(_: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn camera_prepare(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _file_str: i32);
    let file_name = get_str(vm, _file_str as usize).unwrap();
    println!("camera prepare: {}", file_name);

    vm.stack_push::<i32>(1);
    Pal4FunctionState::Completed
}

fn camera_run_single(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _file_str: i32, _sync: i32);
    let file_name = get_str(vm, _file_str as usize).unwrap();
    println!("camera run single: {}", file_name);

    vm.stack_push::<i32>(1);
    Pal4FunctionState::Completed
}

fn flash_out_black(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _duration: f32, _keep: i32, _sync: i32);
    Pal4FunctionState::Completed
}

fn flash_in_black(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _duration: f32, _sync: i32);
    Pal4FunctionState::Completed
}

fn script_music_pause(_: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn play_movie(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _name_str: i32);
    Pal4FunctionState::Completed
}

fn add_player_equip(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _player: i32, _equip: i32);
    Pal4FunctionState::Completed
}

fn open_movie_flag(_name: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _flag: i32);
    Pal4FunctionState::Completed
}

fn script_music_resume(_name: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn wait(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _time: f64);
    Pal4FunctionState::Completed
}

fn talk(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, str: i32);

    Pal4FunctionState::Yield(Box::new(move |vm| {
        let str = get_str(vm, str as usize).unwrap();
        let ui = vm.app_context().ui.ui();
        let _ = ui
            .window("dlg_box")
            .collapsible(false)
            .title_bar(false)
            .resizable(false)
            .draw_background(true)
            .no_decoration()
            .size([1024., 600.], Condition::Appearing)
            .position([0., 0.], Condition::Appearing)
            .build(|| ui.text(&str));

        if ui.is_mouse_released(MouseButton::Left) {
            ContinuationState::Completed
        } else {
            ContinuationState::Loop
        }
    }))
}

fn add_quest_complete_percentage(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _pct: i32);
    Pal4FunctionState::Completed
}

fn arena_load(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(
        vm,
        scn_str: i32,
        block_str: i32,
        _data_str: i32,
        _show_loading: i32
    );

    let scn = get_str(vm, scn_str as usize).unwrap();
    let block = get_str(vm, block_str as usize).unwrap();

    let _ = vm.app_context().scene_manager.pop_scene();
    let scene = vm.app_context().loader.load_scene(&scn, &block).unwrap();
    vm.app_context().scene_manager.push_scene(scene);

    let module = vm.app_context().loader.load_script_module(&scn).unwrap();
    vm.set_function_by_name(module, &format!("{}_{}_init", scn, block));

    Pal4FunctionState::Completed
}

fn script_music_play(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, str: i32, _mode: i32, _fade_in: f32, _fade_out: f32);

    let str = get_str(vm, str as usize).unwrap();
    println!("music play: {}", str);
    Pal4FunctionState::Completed
}

fn script_music_stop(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _flag: i32, _fade_in: f32);
    Pal4FunctionState::Completed
}

fn arena_music_stop(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _fade_out: f32);
    Pal4FunctionState::Completed
}

fn dd_sound_play(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _name_str: i32, _times: i32);
    vm.stack_push::<i32>(0);
    Pal4FunctionState::Completed
}

fn camera_set_collide(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _collide: i32);
    Pal4FunctionState::Completed
}

fn camera_seek_to_player(_: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn camera_auto_seek(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _auto_seek: i32);
    Pal4FunctionState::Completed
}

fn player_set_leader(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _player: i32);
    Pal4FunctionState::Completed
}

fn player_set_attr(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _player: i32, _attr: i32, _value: i32);
    Pal4FunctionState::Completed
}

fn player_lock(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn player_do_action(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _player: i32, _action_str: i32, _loop: i32, _sync: i32);
    Pal4FunctionState::Completed
}

fn player_current_set_pos(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _x: f32, _y: f32, _z: f32);
    Pal4FunctionState::Completed
}

fn player_current_set_ang(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _rot: f32);
    Pal4FunctionState::Completed
}

fn arena_ready(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn grant_system_ui(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _ui: i32, _enable: i32);
    Pal4FunctionState::Completed
}

fn grant_magic_system(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    as_params!(vm, _magic: i32, _skill: i32);
    Pal4FunctionState::Completed
}

fn check_magic_mastered(_: &str, vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    vm.stack_push::<i32>(0);
    Pal4FunctionState::Completed
}

fn unknown(_: &str, _vm: &mut ScriptVm<Pal4AppContext>) -> Pal4FunctionState {
    Pal4FunctionState::Completed
}

fn get_str(vm: &mut ScriptVm<Pal4AppContext>, index: usize) -> Option<String> {
    vm.heap[index].clone()
}
