$modelname "../custom/ZZZ_SCA/models/weapons/c_models/Animations/Scout/pistol.mdl"
// Compile directly into the custom folder so you no longer have to move files around

$include "../c_scout_definebones.qci"

// Do not change the name of any sequences or it will crash the game due to some settings in the class_arms.mdl
$sequence "p_draw" "p_draw.smd" {
	fps 30
	activity "ACT_SECONDARY_VM_DRAW" 1
	snap
	{ event 5004 1 "Weapon_Pistol.Draw" }
}

$sequence "p_idle" "p_idle.smd" {
	fps 30
	activity "ACT_SECONDARY_VM_IDLE" 1
	loop
}

$sequence "p_fire" "p_fire.smd" {
	fps 30
	activity "ACT_SECONDARY_VM_PRIMARYATTACK" 1
	snap
}

$sequence "p_reload" "p_reload.smd" {
	fps 30
	activity "ACT_SECONDARY_VM_RELOAD" 1
	{ event 5004 1 "Weapon_Pistol.ReloadScout" }
	{ event 5004 18 "Weapon_Pistol.ClipIn" }
}