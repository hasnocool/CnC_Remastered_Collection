# Tiberiandawn — Function Index

**Pinned source:** f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae  
**Interpretation:** extracted signatures only; behavior contracts remain to be reviewed.

| Source file | Line | Kind | Symbol | Scope | Signature | Documentation | Review state |
|---|---:|---|---|---|---|---|---|
| TIBERIANDAWN/AADATA.CPP | 309 | function (callable) | AircraftTypeClass | AircraftTypeClass | (AircraftType airtype,int name,char const * ininame,unsigned char level,long pre,bool is_leader,bool is_twoshooter,bool is_transporter,bool is_fixedwing,bool is_rotorequipped,bool is_rotorcustom,bool is_landable,bool is_crushable,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,bool is_theater,bool is_repairable,bool is_buildable,bool is_crew,int ammo,unsigned short strength,int sightrange,int cost,int scenario,int risk,int reward,int ownable,WeaponType primary,WeaponType secondary,ArmorType armor,MPHType maxspeed,int rot,MissionType deforder) | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 406 | function (callable) | From_Name | AircraftTypeClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 434 | function (callable) | One_Time | AircraftTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 482 | function (callable) | Create_One_Of | AircraftTypeClass | (HouseClass * house) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 504 | function (callable) | Prep_For_Add | AircraftTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 533 | function (callable) | Display | AircraftTypeClass | (int x,int y,WindowNumberType window,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 561 | function (callable) | Occupy_List | AircraftTypeClass | (bool) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 582 | function (callable) | Overlap_List | AircraftTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 612 | function (callable) | Who_Can_Build_Me | AircraftTypeClass | (bool,bool legal,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 648 | function (callable) | Repair_Cost | AircraftTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 668 | function (callable) | Repair_Step | AircraftTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 689 | function (callable) | Max_Pips | AircraftTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 717 | function (callable) | Create_And_Place | AircraftTypeClass | (CELL,HousesType) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 741 | function (callable) | Init | AircraftTypeClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 788 | function (callable) | Dimensions | AircraftTypeClass | (int & width,int & height) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AADATA.CPP | 795 | function (callable) | What_Am_I | AircraftTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 55 | function (callable) | Distance | AbstractClass | (TARGET target) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 96 | function (callable) | AbstractTypeClass | AbstractTypeClass | (int name,char const * ini) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 103 | function (callable) | What_Am_I | AbstractTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 104 | function (callable) | Coord_Fixup | AbstractTypeClass | (COORDINATE coord) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 105 | function (callable) | Full_Name | AbstractTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 106 | function (callable) | Get_Ownable | AbstractTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.CPP | 111 | function (callable) | Delete_This | AbstractClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 78 | function (callable) | AbstractClass | AbstractClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 79 | function (callable) | ~AbstractClass | AbstractClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 84 | function (callable) | Owner | AbstractClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 89 | function (callable) | Center_Coord | AbstractClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 90 | function (callable) | Target_Coord | AbstractClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 96 | function (callable) | Direction | AbstractClass | (AbstractClass const * object) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 97 | function (callable) | Direction | AbstractClass | (COORDINATE coord) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 98 | function (callable) | Direction | AbstractClass | (TARGET target) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 99 | function (callable) | Direction | AbstractClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 101 | function (callable) | Distance | AbstractClass | (COORDINATE coord) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 102 | function (callable) | Distance | AbstractClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 103 | function (callable) | Distance | AbstractClass | (AbstractClass const * object) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 108 | function (callable) | Can_Enter_Cell | AbstractClass | (CELL,FacingType=FACING_NONE) const | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 113 | function (callable) | AI | AbstractClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ABSTRACT.H | 123 | function (callable) | Set_Active | AbstractClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ABSTRACT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ADATA.CPP | 2562 | function (callable) | AnimTypeClass | AnimTypeClass | (AnimType anim,char const * name,int size,int biggest,bool isnormal,bool iswhitetrans,bool isscorcher,bool iscrater,bool issticky,bool ground,bool istrans,bool isflame,unsigned int damage,int delaytime,int start,int loopstart,int loopend,int stages,int loops,VocType sound,AnimType chainto,int virtualstages,int virtualscale,AnimType virtualanim) | RUST/original-source-reference/TIBERIANDAWN/ADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ADATA.CPP | 2610 | function (callable) | One_Time | AnimTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ADATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 115 | function (callable) | Validate | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 128 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 147 | function (callable) | As_Target | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 170 | function (callable) | operator new | AircraftClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 194 | function (callable) | operator delete | AircraftClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 222 | function (callable) | AircraftClass | AircraftClass | (AircraftType classid,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 289 | function (callable) | Unlimbo | AircraftClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 343 | function (callable) | Draw_It | AircraftClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 487 | function (callable) | Read_INI | AircraftClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 558 | function (callable) | Write_INI | AircraftClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 615 | function (callable) | Mission_Hunt | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 719 | function (callable) | AI | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 951 | function (callable) | Mark | AircraftClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 981 | function (callable) | Overlap_List | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1016 | function (callable) | Init | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1044 | function (callable) | Mission_Unload | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1325 | function (callable) | Is_LZ_Clear | AircraftClass | (TARGET target) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1364 | function (callable) | In_Which_Layer | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1393 | function (callable) | Sort_Y | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1417 | function (callable) | Mission_Retreat | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1499 | function (callable) | Exit_Object | AircraftClass | (TechnoClass * unit) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1553 | function (callable) | Fire_At | AircraftClass | (TARGET target,int which) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1610 | function (callable) | Take_Damage | AircraftClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1667 | function (callable) | Mission_Move | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1842 | function (callable) | Enter_Idle_Mode | AircraftClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1952 | function (callable) | Process_Fly_To | AircraftClass | (bool slowdown) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 1999 | function (callable) | Debug_Dump | AircraftClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2054 | function (callable) | Active_Click_With | AircraftClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2091 | function (callable) | Active_Click_With | AircraftClass | (ActionType action,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2134 | function (callable) | Player_Assign_Mission | AircraftClass | (MissionType mission,TARGET target,TARGET destination) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2165 | function (callable) | What_Action | AircraftClass | (ObjectClass * target) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2204 | function (callable) | What_Action | AircraftClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2235 | function (callable) | Pose_Dir | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2261 | function (callable) | Mission_Attack | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2478 | function (callable) | New_LZ | AircraftClass | (TARGET oldlz,bool stable) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2528 | function (callable) | Fire_Coord | AircraftClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2535 | function (callable) | Target_Coord | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2563 | function (callable) | Receive_Message | AircraftClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2717 | function (callable) | Desired_Load_Dir | AircraftClass | (ObjectClass * object,CELL & moveto) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2747 | function (callable) | Process_Take_Off | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2796 | function (callable) | Process_Landing | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2832 | function (callable) | Can_Enter_Cell | AircraftClass | (CELL cell,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2863 | function (callable) | Good_Fire_Location | AircraftClass | (TARGET target) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2935 | function (callable) | Cell_Seems_Ok | AircraftClass | (CELL cell,bool strict) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 2972 | function (callable) | Pip_Count | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3007 | function (callable) | Mission_Enter | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3129 | function (callable) | Good_LZ | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3183 | function (callable) | Set_Speed | AircraftClass | (int speed) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3206 | function (callable) | Fire_Direction | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3228 | function (callable) | ~AircraftClass | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3269 | function (callable) | Scatter | AircraftClass | (COORDINATE,bool,bool) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3298 | function (callable) | Rearm_Delay | AircraftClass | (bool second) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3326 | function (callable) | Threat_Range | AircraftClass | (int control) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3354 | function (callable) | Mission_Guard | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3472 | function (callable) | Mission_Guard_Area | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3508 | function (callable) | Response_Attack | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3539 | function (callable) | Response_Move | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3571 | function (callable) | Response_Select | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.CPP | 3588 | function (callable) | Set_Reinforcement_Delay | AircraftClass | (long delay) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.H | 54 | function (callable) | operator AircraftType | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.H | 55 | function (callable) | AircraftClass | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.H | 58 | function (callable) | What_Am_I | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.H | 86 | function (callable) | Class_Of | AircraftClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AIRCRAFT.H | 165 | function (callable) | INI_Name | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AIRCRAFT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 99 | function (callable) | DPMI_Lock |  | (VOID const * ptr,long const size) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 144 | function (callable) | DPMI_Unlock |  | (void const * ptr,long const size) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 193 | function (callable) | Alloc |  | (unsigned long bytes_to_alloc,MemoryFlagType flags) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 384 | function (callable) | Free |  | (void const * pointer) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 503 | function (callable) | Resize_Alloc |  | (void * original_ptr,unsigned long new_size_in_bytes) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 539 | function (callable) | Ram_Free |  | (MemoryFlagType) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 560 | function (callable) | Heap_Size |  | (MemoryFlagType) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ALLOC.CPP | 584 | function (callable) | Total_Ram_Free |  | (MemoryFlagType) | RUST/original-source-reference/TIBERIANDAWN/ALLOC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 81 | function (callable) | Validate | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 94 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 117 | function (callable) | Shorten_Attached_Anims |  | (ObjectClass * obj) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 147 | function (callable) | Sort_Y | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 186 | function (callable) | Center_Coord | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 211 | function (callable) | Render | AnimClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 239 | function (callable) | Draw_It | AnimClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 341 | function (callable) | Mark | AnimClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 369 | function (callable) | Overlap_List | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 467 | function (callable) | Occupy_List | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 491 | function (callable) | Init | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 518 | function (callable) | operator new | AnimClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 543 | function (callable) | operator delete | AnimClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 576 | function (callable) | AnimClass | AnimClass | (AnimType animnum,COORDINATE coord,unsigned char timedelay,char loop,bool alt) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 665 | function (callable) | ~AnimClass | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 725 | function (callable) | AI | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 900 | function (callable) | Attach_To | AnimClass | (ObjectClass * obj) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 930 | function (callable) | Sort_Above | AnimClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 952 | function (callable) | In_Which_Layer | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 979 | function (callable) | Start | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1031 | function (callable) | Middle | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1207 | function (callable) | Chain | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1243 | function (callable) | As_Target | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1260 | function (callable) | Adjust_Coord | AnimClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1301 | function (callable) | Detach | AnimClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1319 | function (callable) | Set_Owner | AnimClass | (HousesType owner) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.CPP | 1327 | function (callable) | Set_Visible_Flags | AnimClass | (unsigned flags) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 50 | function (callable) | AnimClass | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 53 | function (callable) | operator AnimType | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 54 | function (callable) | What_Am_I | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 55 | function (callable) | Owner | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 64 | function (callable) | Make_Invisible | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 65 | function (callable) | Make_Visible | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 66 | function (callable) | Kill_At | AnimClass | (unsigned long long kill_time) | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 73 | function (callable) | Get_Visible_Flags | AnimClass | () const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 76 | function (callable) | Can_Place_Here | AnimClass | (COORDINATE) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ANIM.H | 82 | function (callable) | Class_Of | AnimClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/ANIM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 59 | function (callable) | File_Stream_Sample |  | (char const * filename,BOOL real_time_start) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 60 | function (callable) | File_Stream_Sample_Vol |  | (char const * filename,int volume,BOOL real_time_start) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 61 | function (callable) | Sound_Callback |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 62 | function (callable) | maintenance_callback |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 63 | function (callable) | Load_Sample |  | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 64 | function (callable) | Load_Sample_Into_Buffer |  | (char const * filename,void * buffer,long size) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 65 | function (callable) | Sample_Read |  | (int fh,void * buffer,long size) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 66 | function (callable) | Free_Sample |  | (void const * sample) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 67 | function (callable) | Audio_Init |  | (HWND window,int bits_per_sample,BOOL stereo,int rate,int reverse_channels) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 68 | function (callable) | Sound_End |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 69 | function (callable) | Stop_Sample |  | (int handle) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 70 | function (callable) | Sample_Status |  | (int handle) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 71 | function (callable) | Is_Sample_Playing |  | (void const * sample) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 72 | function (callable) | Stop_Sample_Playing |  | (void const * sample) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 73 | function (callable) | Play_Sample |  | (void const * sample,int priority,int volume,signed short panloc) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 74 | function (callable) | Play_Sample_Handle |  | (void const * sample,int priority,int volume,signed short panloc,int id) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 75 | function (callable) | Set_Sound_Vol |  | (int volume) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 76 | function (callable) | Set_Score_Vol |  | (int volume) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 77 | function (callable) | Fade_Sample |  | (int handle,int ticks) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 78 | function (callable) | Get_Free_Sample_Handle |  | (int priority) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 79 | function (callable) | Get_Digi_Handle |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 80 | function (callable) | Sample_Length |  | (void const * sample) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 81 | function (callable) | Restore_Sound_Buffers |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 82 | function (callable) | Set_Primary_Buffer_Format |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 83 | function (callable) | Start_Primary_Sound_Buffer |  | (BOOL forced) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 84 | function (callable) | Stop_Primary_Sound_Buffer |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 292 | function (callable) | Sound_Effect |  | (VocType voc,COORDINATE coord,int variation) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 360 | function (callable) | Sound_Effect |  | (VocType voc,VolType volume,int variation,signed short pan_value) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 521 | function (callable) | Speak |  | (VoxType voice,HouseClass * house,COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 562 | function (callable) | Speak_AI |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 605 | function (callable) | Stop_Speaking |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.CPP | 630 | function (callable) | Is_Speaking |  | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.H | 64 | function (callable) | AudioClass | AudioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.H | 72 | function (callable) | AudioClass | AudioClass | (char const * name,MemoryClass & mem) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/AUDIO.H | 84 | function (callable) | ~AudioClass | AudioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/AUDIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 65 | function (callable) | operator == | BaseNodeClass | (BaseNodeClass const & node) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 86 | function (callable) | operator != | BaseNodeClass | (BaseNodeClass const & node) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 107 | function (callable) | operator > | BaseNodeClass | (BaseNodeClass const &) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 133 | function (callable) | Read_INI | BaseClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 204 | function (callable) | Write_INI | BaseClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 250 | function (callable) | Load | BaseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 307 | function (callable) | Save | BaseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 362 | function (callable) | Is_Built | BaseClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 387 | function (callable) | Get_Building | BaseClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 434 | function (callable) | Is_Node | BaseClass | (BuildingClass * obj) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 459 | function (callable) | Get_Node | BaseClass | (BuildingClass * obj) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.CPP | 488 | function (callable) | Next_Buildable | BaseClass | (StructType type) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 44 | function (callable) | BaseNodeClass | BaseNodeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 65 | function (callable) | BaseClass | BaseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 66 | function (callable) | ~BaseClass | BaseClass | () | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 71 | function (callable) | Init | BaseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 78 | function (callable) | INI_Name | BaseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 81 | function (callable) | Code_Pointers | BaseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 82 | function (callable) | Decode_Pointers | BaseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BASE.H | 103 | function (callable) | Get_Node | BaseClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/BASE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BBDATA.CPP | 555 | function (callable) | BulletTypeClass | BulletTypeClass | (BulletType type,char const * ininame,bool is_high,bool is_homing,bool is_arcing,bool is_dropping,bool is_invisible,bool is_proximity_armed,bool is_flame_equipped,bool is_fueled,bool is_faceless,bool is_inaccurate,bool is_translucent,bool is_antiaircraft,int arming,int range,MPHType maxspeed,unsigned rot,WarheadType warhead,AnimType explosion) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BBDATA.CPP | 612 | function (callable) | One_Time | BulletTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 60 | macro (function-like macro) | XYCELL |  | (x,y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 3944 | function (callable) | BuildingTypeClass | BuildingTypeClass | (StructType type,int name,char const * ininame,COORDINATE exitpoint,unsigned char level,long pre,bool is_scanner,bool is_regulated,bool is_bibbed,bool is_nominal,bool is_wall,bool is_factory,bool is_captureable,bool is_flammable,bool is_simpledamage,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,bool is_theater,bool is_turret_equipped,bool is_twoshooter,bool is_repairable,bool is_buildable,bool is_crew,bool is_sturdy,RTTIType tobuild,DirType sframe,unsigned short strength,int sightrange,int cost,int scenario,int risk,int reward,int ownable,WeaponType primary,WeaponType secondary,ArmorType armor,unsigned long canenter,unsigned capacity,int power,int drain,BSizeType size,short const * exitlist,short const * sizelist,short const * overlap,bool is_unsellable) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4086 | function (callable) | One_Time | BuildingTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4203 | function (callable) | From_Name | BuildingTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4237 | function (callable) | Display | BuildingTypeClass | (int x,int y,WindowNumberType window,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4263 | function (callable) | Prep_For_Add | BuildingTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4290 | function (callable) | Create_And_Place | BuildingTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4320 | function (callable) | Create_One_Of | BuildingTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4352 | function (callable) | Init_Anim | BuildingTypeClass | (BStateType state,int start,int count,int rate) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4381 | function (callable) | Legal_Placement | BuildingTypeClass | (CELL pos) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4454 | function (callable) | Who_Can_Build_Me | BuildingTypeClass | (bool intheory,bool legal,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4489 | function (callable) | Init | BuildingTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4538 | function (callable) | Dimensions | BuildingTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4578 | function (callable) | As_Reference | BuildingTypeClass | (StructType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4602 | function (callable) | Occupy_List | BuildingTypeClass | (bool placement) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4677 | function (callable) | Overlap_List | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4702 | function (callable) | Width | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4733 | function (callable) | Height | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4766 | function (callable) | Repair_Cost | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4791 | function (callable) | Repair_Step | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4816 | function (callable) | Bib_And_Offset | BuildingTypeClass | (SmudgeType & bib,CELL & cell) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4862 | function (callable) | Max_Pips | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4885 | function (callable) | Full_Name | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4897 | function (callable) | Raw_Cost | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BDATA.CPP | 4921 | function (callable) | Cost_Of | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 157 | function (callable) | Validate | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 170 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 198 | function (callable) | Receive_Message | BuildingClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 439 | function (callable) | Debug_Dump | BuildingClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 504 | function (callable) | Draw_It | BuildingClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 714 | function (callable) | Mark | BuildingClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 819 | function (callable) | Fire_At | BuildingClass | (TARGET target,int which) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 889 | function (callable) | AI | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1304 | function (callable) | Unlimbo | BuildingClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1465 | function (callable) | Take_Damage | BuildingClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1696 | function (callable) | Look | BuildingClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1729 | function (callable) | Look | BuildingClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1758 | function (callable) | operator new | BuildingClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1784 | function (callable) | operator delete | BuildingClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1818 | function (callable) | BuildingClass | BuildingClass | (StructType type,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1887 | function (callable) | ~BuildingClass | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 1922 | function (callable) | Drop_Debris | BuildingClass | (TARGET source) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2038 | function (callable) | Active_Click_With | BuildingClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2068 | function (callable) | Active_Click_With | BuildingClass | (ActionType action,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2101 | function (callable) | Assign_Target | BuildingClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2126 | function (callable) | Init | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2163 | function (callable) | Exit_Object | BuildingClass | (TechnoClass * base) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2398 | function (callable) | Update_Buildables | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2496 | function (callable) | Update_Buildables | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2567 | function (callable) | Fire_Out | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2591 | function (callable) | Limbo | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2739 | function (callable) | Fire_Data | BuildingClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2772 | function (callable) | Fire_Coord | BuildingClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2824 | function (callable) | Greatest_Threat | BuildingClass | (ThreatType threat) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2861 | function (callable) | Grand_Opening | BuildingClass | (bool captured) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 2952 | function (callable) | Repair | BuildingClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3011 | function (callable) | Sell_Back | BuildingClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3073 | function (callable) | What_Action | BuildingClass | (ObjectClass * object) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3150 | function (callable) | What_Action | BuildingClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3188 | function (callable) | Begin_Mode | BuildingClass | (BStateType bstate) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3226 | function (callable) | Read_INI | BuildingClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3332 | function (callable) | Write_INI | BuildingClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3387 | function (callable) | As_Target | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3408 | function (callable) | Center_Coord | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3414 | function (callable) | Occupy_List_Contains |  | (const short * list,short cell) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3425 | function (callable) | Target_Coord | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3449 | function (callable) | Docking_Coord | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3481 | function (callable) | Can_Fire | BuildingClass | (TARGET target,int which) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3543 | function (callable) | Toggle_Primary | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3590 | function (callable) | Captured | BuildingClass | (HouseClass * newowner) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3788 | function (callable) | Sort_Y | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3824 | function (callable) | Can_Enter_Cell | BuildingClass | (CELL cell,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3853 | function (callable) | Can_Demolish | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3867 | function (callable) | Can_Demolish_Unit | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3873 | function (callable) | Can_Capture | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3905 | function (callable) | Mission_Guard | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 3991 | function (callable) | Mission_Construction | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4051 | function (callable) | Mission_Deconstruction | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4265 | function (callable) | Mission_Attack | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4473 | function (callable) | Mission_Harvest | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4558 | function (callable) | Mission_Repair | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4720 | function (callable) | Mission_Missile | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4857 | function (callable) | Revealed | BuildingClass | (HouseClass * house) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4901 | function (callable) | Enter_Idle_Mode | BuildingClass | (bool initial) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4936 | function (callable) | Update_Specials | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4957 | function (callable) | Pip_Count | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 4980 | function (callable) | Death_Announcement | BuildingClass | (TechnoClass const *) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5028 | function (callable) | Fire_Direction | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5054 | function (callable) | Remap_Table | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5078 | function (callable) | Mission_Unload | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5180 | function (callable) | Power_Output | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5207 | function (callable) | Detach | BuildingClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5232 | function (callable) | Refund_Amount | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5263 | function (callable) | Crew_Type | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5302 | function (callable) | Detach_All | BuildingClass | (bool all) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5390 | function (callable) | Flush_For_Placement | BuildingClass | (TechnoClass * techno,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5417 | function (callable) | Hidden | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5426 | function (callable) | Find_Exit_Cell | BuildingClass | (TechnoClass const * techno) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5459 | function (callable) | Can_Player_Move | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.CPP | 5485 | function (callable) | Passes_Proximity_Check | BuildingClass | (CELL homecell) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 59 | function (callable) | operator StructType | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 164 | function (callable) | BuildingClass | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 167 | function (callable) | What_Am_I | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 175 | function (callable) | Fetch_Anim_Control | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 189 | function (callable) | Class_Of | BuildingClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BUILDING.H | 280 | function (callable) | INI_Name | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BUILDING.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 74 | function (callable) | Validate | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 87 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 107 | function (callable) | BulletClass | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 136 | function (callable) | operator new | BulletClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 161 | function (callable) | operator delete | BulletClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 191 | function (callable) | BulletClass | BulletClass | (BulletType id) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 224 | function (callable) | Occupy_List | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 278 | function (callable) | Mark | BulletClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 307 | function (callable) | AI | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 506 | function (callable) | Draw_It | BulletClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 580 | function (callable) | Init | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 611 | function (callable) | Detach | BulletClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 644 | function (callable) | Unlimbo | BulletClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.CPP | 791 | function (callable) | As_Target | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 54 | function (callable) | operator BulletType | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 74 | function (callable) | ~BulletClass | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 75 | function (callable) | What_Am_I | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 82 | function (callable) | Assign_Target | BulletClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 84 | function (callable) | In_Which_Layer | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 85 | function (callable) | Class_Of | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/BULLET.H | 91 | function (callable) | Overlap_List | BulletClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/BULLET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.CPP | 57 | function (callable) | Debug_Dump | CargoClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.CPP | 84 | function (callable) | Attach | CargoClass | (FootClass * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.CPP | 141 | function (callable) | Detach_Object | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.CPP | 172 | function (callable) | Attached_Object | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.H | 49 | function (callable) | CargoClass | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.H | 58 | function (callable) | AI | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.H | 60 | function (callable) | How_Many | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CARGO.H | 61 | function (callable) | Is_Something_Attached | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 86 | function (callable) | DDE_Callback |  | (unsigned char * data,long length) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 108 | function (callable) | DDEServerClass | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 128 | function (callable) | Enable | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 152 | function (callable) | Disable | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 179 | function (callable) | ~DDEServerClass | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 202 | function (callable) | Callback | DDEServerClass | (unsigned char * data,long length) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 310 | function (callable) | Get_MPlayer_Game_Info | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 331 | function (callable) | Delete_MPlayer_Game_Info | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 355 | function (callable) | Time_Since_Heartbeat | DDEServerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.CPP | 379 | function (callable) | Send_Data_To_DDE_Server |  | (char * data,int length,int packet_type) | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCDDE.H | 53 | function (callable) | Get_MPlayer_Game_Info_Length | DDEServerClass | () | RUST/original-source-reference/TIBERIANDAWN/CCDDE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 79 | function (callable) | Error | CCFileClass | (int,int,char const *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 120 | function (callable) | CCFileClass | CCFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 146 | function (callable) | CCFileClass | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 172 | function (callable) | Write | CCFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 205 | function (callable) | Read | CCFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 274 | function (callable) | Seek | CCFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 315 | function (callable) | Size | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 338 | function (callable) | Is_Available | CCFileClass | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 362 | function (callable) | Is_Open | CCFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 388 | function (callable) | Close | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 415 | function (callable) | Open | CCFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 484 | function (callable) | Set_Search_Drives |  | (char const *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 491 | function (callable) | Open_File |  | (char const * file_name,int mode) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 506 | function (callable) | Close_File |  | (int handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 513 | function (callable) | Read_File |  | (int handle,VOID * buf,ULONG bytes) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 521 | function (callable) | Write_File |  | (int handle,VOID const * buf,ULONG bytes) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 529 | function (callable) | Find_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 536 | function (callable) | Delete_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 541 | function (callable) | Create_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 546 | function (callable) | Load_Data |  | (char const * name,VOID * ptr,ULONG size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 552 | function (callable) | Load_Alloc_Data |  | (char const * name,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 559 | function (callable) | File_Size |  | (int handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 568 | function (callable) | Write_Data |  | (char const * name,VOID const * ptr,ULONG size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 574 | function (callable) | Seek_File |  | (int handle,LONG offset,int starting) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 582 | function (callable) | WWDOS_Shutdown |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 590 | function (callable) | Multi_Drive_Search |  | (bool on) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 596 | function (callable) | WWDOS_Init |  | (VOID) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 600 | function (callable) | WWDOS_Shutdown |  | (VOID) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 604 | function (callable) | Find_Disk_Number |  | (char const *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.CPP | 624 | function (callable) | Unfragment_File_Cache |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.H | 54 | function (callable) | ~CCFileClass | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.H | 58 | function (callable) | Open | CCFileClass | (char const * filename,int rights=READ) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CCFILE.H | 108 | function (callable) | CCFileClass | CCFileClass | (CCFileClass const &) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2493 | function (callable) | TemplateTypeClass | TemplateTypeClass | (TemplateType iconset,int theater,char const * ininame,int fullname,LandType land,int width,int height,LandType altland,char const * alticons) | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2525 | function (callable) | From_Name | TemplateTypeClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2556 | function (callable) | Occupy_List | TemplateTypeClass | (bool) const | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2595 | function (callable) | Init | TemplateTypeClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2635 | function (callable) | Display | TemplateTypeClass | (int x,int y,WindowNumberType window,HousesType) const | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2696 | function (callable) | Prep_For_Add | TemplateTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2722 | function (callable) | Create_And_Place | TemplateTypeClass | (CELL cell,HousesType) const | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2747 | function (callable) | Create_One_Of | TemplateTypeClass | (HouseClass *) const | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDATA.CPP | 2765 | function (callable) | One_Time | TemplateTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 54 | function (callable) | Is_Disk_Inserted |  | (int disk) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 68 | function (callable) | CDFileClass | CDFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 76 | function (callable) | CDFileClass | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 98 | function (callable) | Open | CDFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 118 | function (callable) | Refresh_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 165 | function (callable) | Set_Search_Drives | CDFileClass | (char * pathlist) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 260 | function (callable) | Set_CD_Drive | CDFileClass | (int drive) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 283 | function (callable) | Add_Search_Drive | CDFileClass | (char * path) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 330 | function (callable) | Clear_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 371 | function (callable) | Set_Name | CDFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 447 | function (callable) | Open | CDFileClass | (char const * filename,int rights) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 477 | function (callable) | GetCDDrive |  | (VOID) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 487 | function (callable) | Get_CD_Drive |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.CPP | 513 | function (callable) | Get_Search_Path | CDFileClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 56 | function (callable) | ~CDFileClass | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 62 | function (callable) | Searching | CDFileClass | (int on) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 64 | function (callable) | Is_There_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 70 | function (callable) | Get_CD_Drive | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 71 | function (callable) | Get_Last_CD_Drive | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CDFILE.H | 74 | function (callable) | Reset_Raw_Path | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 101 | function (callable) | Validate | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 114 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 133 | function (callable) | CellClass | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 163 | function (callable) | Cell_Color | CellClass | (bool override) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 198 | function (callable) | Cell_Techno | CellClass | (int x,int y) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 246 | function (callable) | Cell_Find_Object | CellClass | (RTTIType rtti) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 279 | function (callable) | Cell_Building | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 303 | function (callable) | Cell_Terrain | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 331 | function (callable) | Cell_Object | CellClass | (int x,int y) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 374 | function (callable) | Redraw_Objects | CellClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 430 | function (callable) | Is_Generally_Clear | CellClass | (bool ignore_cloaked) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 488 | function (callable) | Recalc_Attributes | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 554 | function (callable) | Occupy_Down | CellClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 642 | function (callable) | Occupy_Up | CellClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 713 | function (callable) | Overlap_Down | CellClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 789 | function (callable) | Overlap_Up | CellClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 818 | function (callable) | Cell_Unit | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 841 | function (callable) | Cell_Infantry | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 865 | function (callable) | Get_Template_Info | CellClass | (char * template_name,int & icon,void * & image_data) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 913 | function (callable) | Draw_It | CellClass | (int x,int y,int draw_type) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1158 | function (callable) | Concrete_Calc | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1381 | function (callable) | Wall_Update | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1461 | function (callable) | Cell_Coord | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1483 | function (callable) | Reduce_Tiberium | CellClass | (int levels) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1519 | function (callable) | Reduce_Wall | CellClass | (int damage) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1580 | function (callable) | Get_Trigger | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1606 | function (callable) | Spot_Index | CellClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1657 | function (callable) | Closest_Free_Spot | CellClass | (COORDINATE coord,bool any) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1748 | function (callable) | Clear_Icon | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1777 | function (callable) | Incoming | CellClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1815 | function (callable) | Adjacent_Cell | CellClass | (FacingType face) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1847 | function (callable) | Adjust_Threat | CellClass | (HousesType house,int threat_value) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1885 | function (callable) | Tiberium_Adjust | CellClass | (bool pregame) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 1941 | function (callable) | Goodie_Check | CellClass | (FootClass * object,bool check_steel) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2386 | function (callable) | Flag_Place | CellClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2414 | function (callable) | Flag_Remove | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2428 | function (callable) | Flag_Update | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2438 | function (callable) | Flag_Create | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2458 | function (callable) | Flag_Destroy | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2481 | function (callable) | Shimmer | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2509 | function (callable) | Cell_Occupier | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2545 | function (callable) | Set_Mapped | CellClass | (HousesType house,bool set) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2563 | function (callable) | Set_Mapped | CellClass | (HouseClass * player,bool set) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2580 | function (callable) | Is_Mapped | CellClass | (HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2593 | function (callable) | Is_Mapped | CellClass | (HouseClass * player) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2608 | function (callable) | Set_Visible | CellClass | (HousesType house,bool set) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2626 | function (callable) | Set_Visible | CellClass | (HouseClass * player,bool set) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2643 | function (callable) | Is_Visible | CellClass | (HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2656 | function (callable) | Is_Visible | CellClass | (HouseClass * player) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2664 | function (callable) | Override_Land_Type | CellClass | (LandType type) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.CPP | 2701 | function (callable) | Is_Clear_To_Move | CellClass | (bool ignoreinfantry,bool ignorevehicles) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 182 | function (callable) | ~CellClass | CellClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 184 | function (callable) | operator == | CellClass | (CellClass const & cell) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 191 | function (callable) | Is_Spot_Free | CellClass | (int spot_index) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 193 | function (callable) | Free_Spot | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 195 | function (callable) | As_Target | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 198 | function (callable) | Adjacent_Cell | CellClass | (FacingType face) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 202 | function (callable) | Land_Type | CellClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 256 | function (callable) | operator != | CellClass | (CellClass const &) const | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CELL.H | 286 | function (callable) | CellClass | CellClass | (CellClass const &) | RUST/original-source-reference/TIBERIANDAWN/CELL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHECKBOX.CPP | 54 | function (callable) | Draw_Me | CheckBoxClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/CHECKBOX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHECKBOX.H | 43 | function (callable) | CheckBoxClass | CheckBoxClass | (unsigned id,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/CHECKBOX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.CPP | 64 | function (callable) | CheckListClass | CheckListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.CPP | 88 | function (callable) | Check_Item | CheckListClass | (int index,int checked) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.CPP | 111 | function (callable) | Is_Checked | CheckListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.CPP | 136 | function (callable) | Action | CheckListClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.H | 53 | function (callable) | ~CheckListClass | CheckListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CHEKLIST.H | 70 | function (callable) | Set_Read_Only | CheckListClass | (int rdonly) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 63 | function (callable) | ColorListClass | ColorListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 87 | function (callable) | ~ColorListClass | ColorListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 109 | function (callable) | Add_Item | ColorListClass | (char const * text,char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 132 | function (callable) | Add_Item | ColorListClass | (int text,char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 153 | function (callable) | Remove_Item | ColorListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 179 | function (callable) | Set_Selected_Style | ColorListClass | (SelectStyleType style,int color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COLRLIST.CPP | 204 | function (callable) | Draw_Entry | ColorListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBAT.CPP | 69 | function (callable) | Modify_Damage |  | (int damage,WarheadType warhead,ArmorType armor,int distance) | RUST/original-source-reference/TIBERIANDAWN/COMBAT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBAT.CPP | 129 | function (callable) | Explosion_Damage |  | (COORDINATE coord,unsigned strength,TechnoClass * source,WarheadType warhead) | RUST/original-source-reference/TIBERIANDAWN/COMBAT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 69 | function (callable) | CommBufferClass | CommBufferClass | (int numsend,int numreceive,int maxlen) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 120 | function (callable) | ~CommBufferClass | CommBufferClass | () | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 159 | function (callable) | Init | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 212 | function (callable) | Init_Send_Queue | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 253 | function (callable) | Queue_Send | CommBufferClass | (void * buf,int buflen) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 327 | function (callable) | UnQueue_Send | CommBufferClass | (void * buf,int * buflen,int index) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 389 | function (callable) | Get_Send | CommBufferClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 416 | function (callable) | Queue_Receive | CommBufferClass | (void * buf,int buflen) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 497 | function (callable) | UnQueue_Receive | CommBufferClass | (void * buf,int * buflen,int index) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 558 | function (callable) | Get_Receive | CommBufferClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 594 | function (callable) | Add_Delay | CommBufferClass | (unsigned long delay) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 631 | function (callable) | Avg_Response_Time | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 653 | function (callable) | Max_Response_Time | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 675 | function (callable) | Reset_Response_Time | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 709 | function (callable) | Configure_Debug | CommBufferClass | (int offset,int size,char ** names,int maxnames) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 737 | function (callable) | Mono_Debug_Print | CommBufferClass | (int refresh) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.CPP | 873 | function (callable) | Mono_Debug_Print2 | CommBufferClass | (int refresh) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 101 | function (callable) | Num_Send | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 102 | function (callable) | Max_Send | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 104 | function (callable) | Send_Total | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 111 | function (callable) | Num_Receive | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 112 | function (callable) | Max_Receive | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMBUF.H | 114 | function (callable) | Receive_Total | CommBufferClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMBUF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMPAT.H | 47 | macro (function-like macro) | movmem |  | (a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMPAT.H | 64 | macro (function-like macro) | Set_Font_Palette |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 76 | function (callable) | CommQueueClass | CommQueueClass | (int numsend,int numreceive,int maxlen) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 124 | function (callable) | ~CommQueueClass | CommQueueClass | () | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 160 | function (callable) | Init | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 228 | function (callable) | Queue_Send | CommQueueClass | (void * buf,int buflen) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 282 | function (callable) | UnQueue_Send | CommQueueClass | (void * buf,int * buflen) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 332 | function (callable) | Next_Send | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 361 | function (callable) | Get_Send | CommQueueClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 393 | function (callable) | Queue_Receive | CommQueueClass | (void * buf,int buflen) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 449 | function (callable) | UnQueue_Receive | CommQueueClass | (void * buf,int * buflen) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 499 | function (callable) | Next_Receive | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 528 | function (callable) | Get_Receive | CommQueueClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 569 | function (callable) | Add_Delay | CommQueueClass | (unsigned long delay) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 607 | function (callable) | Avg_Response_Time | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 629 | function (callable) | Max_Response_Time | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 651 | function (callable) | Reset_Response_Time | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 685 | function (callable) | Configure_Debug | CommQueueClass | (int offset,int size,char ** names,int maxnames) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 713 | function (callable) | Mono_Debug_Print | CommQueueClass | (int refresh) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.CPP | 849 | function (callable) | Mono_Debug_Print2 | CommQueueClass | (int refresh) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 110 | function (callable) | Num_Send | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 111 | function (callable) | Max_Send | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 113 | function (callable) | Send_Total | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 121 | function (callable) | Num_Receive | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 122 | function (callable) | Max_Receive | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COMQUEUE.H | 124 | function (callable) | Receive_Total | CommQueueClass | (void) | RUST/original-source-reference/TIBERIANDAWN/COMQUEUE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONFDLG.CPP | 41 | function (callable) | Process | ConfirmationClass | (int text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONFDLG.CPP | 57 | function (callable) | Process | ConfirmationClass | (char const * string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONFDLG.H | 48 | function (callable) | ConfirmationClass | ConfirmationClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.CPP | 84 | function (callable) | ConnectionClass | ConnectionClass | (int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.CPP | 136 | function (callable) | ~ConnectionClass | ConnectionClass | () | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.CPP | 161 | function (callable) | Service | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.CPP | 209 | function (callable) | Time | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.CPP | 240 | function (callable) | Command_Name | ConnectionClass | (int command) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 249 | function (callable) | Init | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 273 | function (callable) | Magic_Num | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 274 | function (callable) | Retry_Delta | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 275 | function (callable) | Set_Retry_Delta | ConnectionClass | (unsigned long delta) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 276 | function (callable) | Max_Retries | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 277 | function (callable) | Set_Max_Retries | ConnectionClass | (unsigned long retries) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 278 | function (callable) | Time_Out | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 279 | function (callable) | Set_TimeOut | ConnectionClass | (unsigned long t) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNECT.H | 280 | function (callable) | Max_Packet_Len | ConnectionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CONNECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNMGR.H | 87 | function (callable) | ConnManClass | ConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONNMGR.H | 88 | function (callable) | ~ConnManClass | ConnManClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 105 | macro (function-like macro) | ARRAY_SIZE |  | (a) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 135 | function (callable) | Main_Game |  | (int argc,char * argv[]) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 430 | function (callable) | Keyboard_Process |  | (KeyNumType & input) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 750 | function (callable) | Message_Input |  | (KeyNumType & input) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1041 | function (callable) | Color_Cycle |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1126 | function (callable) | Call_Back |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1262 | function (callable) | Language_Name |  | (char const * basename) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1290 | function (callable) | Source_From_Name |  | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1318 | function (callable) | Name_From_Source |  | (SourceType source) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1342 | function (callable) | Theater_From_Name |  | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1374 | function (callable) | KN_To_Facing |  | (int input) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1423 | function (callable) | Sync_Delay |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1469 | function (callable) | Main_Loop |  | () | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1713 | function (callable) | Map_Edit_Loop |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1758 | function (callable) | Go_Editor |  | (bool flag) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1829 | function (callable) | Init_VQ_Threading |  | (CCFileClass * file) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1842 | function (callable) | Cleanup_VQ_Threading |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1851 | function (callable) | Thread_Read |  | (void * file) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1900 | function (callable) | Read_VQ_Thread_Block |  | (CCFileClass * file) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1916 | function (callable) | VQ_Thread_Read |  | (CCFileClass * file,void * buffer,long bytes) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1955 | function (callable) | VQ_Thread_Seek |  | (long bytes) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 1987 | function (callable) | MixFileHandler |  | (VQAHandle * vqa,long action,void * buffer,long nbytes) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2125 | function (callable) | MixFileHandler |  | (VQAHandle * vqa,long action,void * buffer,long nbytes) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2229 | function (callable) | Rebuild_Interpolated_Palette |  | (unsigned char * interpal) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2244 | function (callable) | Load_Interpolated_Palettes |  | (char const * filename,BOOL add) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2290 | function (callable) | Free_Interpolated_Palettes |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2329 | function (callable) | Play_Movie |  | (char const * name,ThemeType theme,bool clrscrn) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2497 | function (callable) | Unselect_All |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2518 | function (callable) | Unselect_All_Except |  | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2558 | function (callable) | Fading_Table_Name |  | (char const * base,TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2581 | function (callable) | Get_Radar_Icon |  | (void const * shapefile,int shapenum,int frames,int zoomfactor) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2680 | function (callable) | CC_Texture_Fill |  | (void const * shapefile,int shapenum,int xpos,int ypos,int width,int height) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2733 | function (callable) | CC_Draw_Shape |  | (ObjectClass * object,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,int scale,int width,int height) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2746 | function (callable) | CC_Draw_Shape |  | (ObjectClass * object,const char * shape_file_name,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,char override_owner) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2759 | function (callable) | CC_Draw_Pip |  | (ObjectClass * object,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2770 | function (callable) | CC_Draw_Line |  | (int x,int y,int x1,int y1,unsigned char color,int frame,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2814 | function (callable) | CC_Draw_Shape |  | (void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2903 | function (callable) | Fetch_Techno_Type |  | (RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 2938 | function (callable) | Trap_Object |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3069 | function (callable) | VQA_PauseAudio |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3072 | function (callable) | VQ_Call_Back |  | (unsigned char *,long) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3125 | function (callable) | Handle_Team |  | (int team,int action) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3271 | function (callable) | Handle_View |  | (int view,int action) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3284 | function (callable) | Heap_Dump_Check |  | (char * string) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3370 | function (callable) | Dump_Heap_Pointers |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3592 | function (callable) | Error_In_Heap_Pointers |  | (char * string) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3621 | function (callable) | Get_CD_Index |  | (int cd_drive,int timeout) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 3715 | function (callable) | Force_CD_Available |  | (int cd) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4006 | function (callable) | Disk_Space_Available |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4038 | function (callable) | Validate_Error |  | (char * name) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4069 | function (callable) | Do_Record_Playback |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4194 | function (callable) | Hires_Retrieve |  | (char * name) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4205 | function (callable) | Get_Resolution_Factor |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4225 | function (callable) | Blit_Hid_Page_To_Seen_Buff |  | (void) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONQUER.CPP | 4246 | function (callable) | Shake_The_Screen |  | (int shakes,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/CONQUER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONTROL.CPP | 70 | function (callable) | ControlClass | ControlClass | (unsigned id,int x,int y,int w,int h,unsigned flags,int sticky) | RUST/original-source-reference/TIBERIANDAWN/CONTROL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONTROL.CPP | 99 | function (callable) | Action | ControlClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/CONTROL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONTROL.CPP | 142 | function (callable) | Make_Peer | ControlClass | (GadgetClass & gadget) | RUST/original-source-reference/TIBERIANDAWN/CONTROL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONTROL.CPP | 165 | function (callable) | Get_ID | ControlClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/CONTROL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CONTROL.CPP | 193 | function (callable) | Draw_Me | ControlClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/CONTROL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 71 | function (callable) | Coord_Spillage_List |  | (COORDINATE coord,int maxsize) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 171 | function (callable) | Coord_Move |  | (COORDINATE start,register DirType dir,unsigned short distance) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 313 | function (callable) | Cardinal_To_Fixed |  | (unsigned base,unsigned cardinal) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 346 | function (callable) | Fixed_To_Cardinal |  | (unsigned base,unsigned fixed) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 386 | function (callable) | Coord_Scatter |  | (COORDINATE coord,unsigned distance,bool lock) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 403 | function (callable) | calcx |  | (signed short param1,short distance) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 421 | function (callable) | calcy |  | (signed short param1,short distance) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORD.CPP | 471 | function (callable) | Move_Point |  | (short & x,short & y,register DirType dir,unsigned short distance) | RUST/original-source-reference/TIBERIANDAWN/COORD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORDA.ASM | 36 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORDA.ASM | 37 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/COORDA.ASM | 38 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CREDITS.CPP | 56 | function (callable) | CreditClass | CreditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CREDITS.CPP | 86 | function (callable) | Graphic_Logic | CreditClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CREDITS.CPP | 138 | function (callable) | AI | CreditClass | (bool forced,HouseClass * player_ptr,bool logic_only) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CREW.H | 55 | function (callable) | CrewClass | CrewClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CREW.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/CREW.H | 57 | function (callable) | Made_A_Kill | CrewClass | (void) | RUST/original-source-reference/TIBERIANDAWN/CREW.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 81 | function (callable) | Instance_Class | Instance_Class | (LPSTR name1,LPSTR name2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 148 | function (callable) | ~Instance_Class | Instance_Class | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 170 | function (callable) | Enable_Callback | Instance_Class | (BOOL flag) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 225 | function (callable) | Test_Server_Running | Instance_Class | (HSZ name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 254 | function (callable) | Open_Poke_Connection | Instance_Class | (HSZ name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 286 | function (callable) | Close_Poke_Connection | Instance_Class | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 317 | function (callable) | Poke_Server | Instance_Class | (LPBYTE poke_data,DWORD poke_length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DDE.CPP | 361 | function (callable) | dde_callback | Instance_Class | (UINT dde_event,UINT uFmt,HCONV,HSZ hsz1,HSZ hsz2,HDDEDATA hdata,DWORD,DWORD) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEBUG.CPP | 60 | function (callable) | Debug_Key |  | (unsigned input) | RUST/original-source-reference/TIBERIANDAWN/DEBUG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEBUG.CPP | 641 | function (callable) | Self_Regulate |  | (void) | RUST/original-source-reference/TIBERIANDAWN/DEBUG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 419 | function (callable) | operator \| |  | (ThreatType a,ThreatType b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 422 | function (callable) | operator & |  | (ThreatType a,ThreatType b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 425 | function (callable) | operator ~ |  | (ThreatType a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 1921 | function (callable) | operator \| |  | (TextPrintType a,TextPrintType b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 1924 | function (callable) | operator & |  | (TextPrintType a,TextPrintType b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 1927 | function (callable) | operator ~ |  | (TextPrintType a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2088 | function (callable) | operator + |  | (FacingType f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2092 | function (callable) | operator + |  | (FacingType f1,int f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2097 | function (callable) | operator - |  | (FacingType f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2101 | function (callable) | operator - |  | (FacingType f1,int f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2106 | function (callable) | operator += |  | (FacingType & f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2111 | function (callable) | operator += |  | (FacingType & f1,int f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2132 | function (callable) | operator + |  | (DirType f1,DirType f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2136 | function (callable) | operator + |  | (DirType f1,int f2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DEFINES.H | 2865 | macro (function-like macro) | size_of |  | (typ,id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DESCDLG.CPP | 51 | function (callable) | Process | DescriptionClass | (char * string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DESCDLG.H | 60 | function (callable) | DescriptionClass | DescriptionClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIAL8.CPP | 61 | function (callable) | Dial8Class | Dial8Class | (int id,int x,int y,int w,int h,DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIAL8.CPP | 149 | function (callable) | Action | Dial8Class | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIAL8.CPP | 230 | function (callable) | Draw_Me | Dial8Class | (int forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIAL8.CPP | 287 | function (callable) | Get_Direction | Dial8Class | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIAL8.CPP | 308 | function (callable) | Set_Direction | Dial8Class | (DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 64 | function (callable) | Dialog_Box |  | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 96 | function (callable) | Draw_Box |  | (int x,int y,int w,int h,BoxStyleEnum up,bool filled) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 173 | function (callable) | Format_Window_String |  | (char * string,int maxlinelen,int & width,int & height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 244 | function (callable) | Window_Box |  | (WindowNumberType window,BoxStyleEnum style) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 315 | function (callable) | Simple_Text_Print |  | (char const * text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 595 | function (callable) | Fancy_Text_Print |  | (int text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 652 | function (callable) | Fancy_Text_Print |  | (char const * text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DIALOG.CPP | 712 | function (callable) | Conquer_Clip_Text_Print |  | (char const * text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag,unsigned width,int const * tabs) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 159 | function (callable) | DisplayClass | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 198 | function (callable) | One_Time | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 269 | function (callable) | Init_Clear | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 310 | function (callable) | Init_IO | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 339 | function (callable) | Init_Theater | DisplayClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 566 | function (callable) | Text_Overlap_List | DisplayClass | (char const * text,int x,int y,int lines) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 637 | function (callable) | Set_View_Dimensions | DisplayClass | (int x,int y,int width,int height) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 747 | function (callable) | Set_Cursor_Shape | DisplayClass | (short const * list) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 790 | function (callable) | Passes_Proximity_Check | DisplayClass | (ObjectTypeClass const * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 882 | function (callable) | Passes_Proximity_Check | DisplayClass | (ObjectTypeClass const * object,HousesType house,short const * list,CELL trycell) const | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 959 | function (callable) | Set_Cursor_Pos | DisplayClass | (CELL pos) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1047 | function (callable) | Get_Occupy_Dimensions | DisplayClass | (int & w,int & h,short const * list) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1105 | function (callable) | Cursor_Mark | DisplayClass | (CELL pos,bool on) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1170 | function (callable) | AI | DisplayClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1208 | function (callable) | Submit | DisplayClass | (ObjectClass const * object,LayerType layer) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1234 | function (callable) | Remove | DisplayClass | (ObjectClass const * object,LayerType layer) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1260 | function (callable) | Click_Cell_Calc | DisplayClass | (int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1294 | function (callable) | Read_INI | DisplayClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1435 | function (callable) | Write_INI | DisplayClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1510 | function (callable) | Scroll_Map | DisplayClass | (DirType facing,int & distance,bool really) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1602 | function (callable) | Refresh_Cells | DisplayClass | (CELL cell,short const * list) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1635 | function (callable) | Cell_Shadow | DisplayClass | (CELL cell,HouseClass * house) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1716 | function (callable) | Cell_Shadow | DisplayClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1798 | function (callable) | Map_Cell | DisplayClass | (CELL cell,HouseClass * house,bool and_for_allies) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1921 | function (callable) | Coord_To_Pixel | DisplayClass | (COORDINATE coord,int & x,int & y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1940 | function (callable) | Coord_To_Pixel | DisplayClass | (COORDINATE coord,int & x,int & y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 1982 | function (callable) | Push_Onto_TacMap | DisplayClass | (COORDINATE & source,COORDINATE & dest) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2031 | function (callable) | Cell_Object | DisplayClass | (CELL cell,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2073 | function (callable) | Draw_It | DisplayClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2410 | function (callable) | Redraw_Icons | DisplayClass | (int draw_flags) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2467 | function (callable) | Redraw_Shadow | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2517 | function (callable) | Redraw_Shadow_Rects | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2569 | function (callable) | Next_Object | DisplayClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2609 | function (callable) | Prev_Object | DisplayClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2650 | function (callable) | Pixel_To_Coord | DisplayClass | (int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2694 | function (callable) | Calculated_Cell | DisplayClass | (SourceType dir,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2915 | function (callable) | should_exclude_from_selection |  | (ObjectClass * obj) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 2922 | function (callable) | Select_These | DisplayClass | (COORDINATE coord1,COORDINATE coord2,bool additive) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3084 | function (callable) | Refresh_Band | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3147 | function (callable) | Action | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3344 | function (callable) | Selection_At_Mouse | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3422 | function (callable) | Command_Object | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3496 | function (callable) | Mouse_Right_Press | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3545 | function (callable) | Mouse_Left_Up | DisplayClass | (bool shadow,ObjectClass * object,ActionType action,bool wwsmall) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3786 | function (callable) | Mouse_Left_Release | DisplayClass | (CELL cell,int x,int y,ObjectClass * object,ActionType action,bool wwsmall) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3946 | function (callable) | Mouse_Left_Press | DisplayClass | (int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 3973 | function (callable) | Mouse_Left_Held | DisplayClass | (int x,int y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4030 | function (callable) | Set_Tactical_Position | DisplayClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4065 | function (callable) | Compute_Start_Pos | DisplayClass | (long & x,long & y) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4137 | function (callable) | Sell_Mode_Control | DisplayClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4184 | function (callable) | Repair_Mode_Control | DisplayClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4231 | function (callable) | In_View | DisplayClass | (register CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4258 | function (callable) | Closest_Free_Spot | DisplayClass | (COORDINATE coord,bool any) const | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4267 | function (callable) | Is_Spot_Free | DisplayClass | (COORDINATE coord) const | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4292 | function (callable) | Center_Map | DisplayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4324 | function (callable) | get_action_priority |  | (ActionType action) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4335 | function (callable) | index_of |  | (const DynamicVectorClass<T * > & list,T * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4346 | function (callable) | Best_Object_With_ActionT |  | (DynamicVectorClass<ObjectClass * > & objects,T subject) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4375 | function (callable) | Best_Object_With_Action |  | (DynamicVectorClass<ObjectClass * > & objects,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4380 | function (callable) | Best_Object_With_Action |  | (DynamicVectorClass<ObjectClass * > & objects,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4385 | function (callable) | Best_Object_Action |  | (DynamicVectorClass<ObjectClass * > & objects,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4391 | function (callable) | Best_Object_Action |  | (DynamicVectorClass<ObjectClass * > & objects,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4397 | function (callable) | Best_Object_With_Action |  | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4402 | function (callable) | Best_Object_With_Action |  | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4407 | function (callable) | Best_Object_Action |  | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.CPP | 4412 | function (callable) | Best_Object_Action |  | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.H | 152 | function (callable) | Help_Text | DisplayClass | (int,int=-1,int=-1,int=YELLOW,bool=false,int=0) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.H | 161 | function (callable) | Put_Place_Back | DisplayClass | (TechnoClass *) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.H | 194 | function (callable) | Flag_Cell | DisplayClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.H | 199 | function (callable) | Is_Cell_Flagged | DisplayClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DISPLAY.H | 280 | function (callable) | TacticalClass | DisplayClass::TacticalClass | (void) | RUST/original-source-reference/TIBERIANDAWN/DISPLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 218 | function (callable) | Set_Event_Callback | DLLExportClass | (CNC_Event_Callback_Type event_callback) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 268 | function (callable) | Get_Shadow_Shapes | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 269 | function (callable) | Get_Shadow_Trans | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 283 | function (callable) | Get_Game_Over | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 382 | function (callable) | Play_Movie_GlyphX |  | (const char * movie_name,ThemeType theme) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 392 | function (callable) | On_Sound_Effect |  | (int sound_index,int variation,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 432 | function (callable) | On_Speech |  | (int speech_index,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 444 | function (callable) | On_Ping |  | (const HouseClass * player_ptr,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 450 | function (callable) | GlyphX_Debug_Print |  | (const char * debug_text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 456 | function (callable) | On_Achievement_Event |  | (const HouseClass * player_ptr,const char * achievement_type,const char * achievement_reason) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 475 | function (callable) | CNC_Version |  | (unsigned int version_in) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 497 | function (callable) | CNC_Init |  | (const char * command_line,CNC_Event_Callback_Type event_callback) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 521 | function (callable) | DLL_Shutdown |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 541 | function (callable) | CNC_Config |  | (const CNCRulesDataStruct & rules) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 560 | function (callable) | CNC_Add_Mod_Path |  | (const char * mod_path) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 580 | function (callable) | CNC_Get_Visible_Page |  | (unsigned char * buffer_in,unsigned int & width,unsigned int & height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 622 | function (callable) | CNC_Get_Palette |  | (unsigned char (& palette_in)[256][3]) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 642 | function (callable) | CNC_Set_Multiplayer_Data |  | (int scenario_index,CNCMultiplayerOptionsStruct & game_options,int num_players,CNCPlayerInfoStruct * player_list,int max_players) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 723 | function (callable) | CNC_Clear_Object_Selection |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 734 | function (callable) | CNC_Select_Object |  | (uint64 player_id,int object_type_id,int object_to_select_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 838 | function (callable) | GlyphX_Assign_Houses |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1009 | function (callable) | CNC_Start_Instance |  | (int scenario_index,int build_level,const char * faction,const char * game_type,const char * content_directory,int sabotaged_structure,const char * override_map_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1024 | function (callable) | HandleSabotagedStructure |  | (int structure_type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1067 | function (callable) | CNC_Read_INI |  | (int scenario_index,int scenario_variation,int scenario_direction,const char * content_directory,const char * override_map_name,char * ini_buffer,int _ini_buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1142 | function (callable) | CNC_Set_Home_Cell |  | (int x,int y,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1161 | function (callable) | CNC_Start_Instance_Variation |  | (int scenario_index,int scenario_variation,int scenario_direction,int build_level,const char * faction,const char * game_type,const char * content_directory,int sabotaged_structure,const char * override_map_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1286 | function (callable) | CNC_Start_Custom_Instance |  | (const char * content_directory,const char * directory_path,const char * scenario_name,int build_level,bool multiplayer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1376 | function (callable) | Debug_Write_Shape_Type |  | (const ObjectTypeClass * type,int shapenum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1394 | function (callable) | Debug_Write_Shape |  | (const char * file_name,void const * shapefile,int shapenum,int flags,void const * ghostdata) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1443 | function (callable) | CNC_Advance_Instance |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1678 | function (callable) | CNC_Save_Load |  | (bool save,const char * file_path_and_name,const char * game_type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1735 | function (callable) | CNC_Set_Difficulty |  | (int difficulty) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1755 | function (callable) | CNC_Handle_Player_Switch_To_AI |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1843 | function (callable) | CNC_Handle_Human_Team_Wins |  | (uint64 quitting_player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1855 | function (callable) | CNC_Start_Mission_Timer |  | (int time) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1867 | function (callable) | CNC_Get_Start_Game_Info |  | (uint64 player_id,int & start_location_waypoint_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1891 | function (callable) | Init | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1914 | function (callable) | Shutdown | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1936 | function (callable) | Add_Mod_Path | DLLExportClass | (const char * mod_path) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1956 | function (callable) | Set_Content_Directory | DLLExportClass | (const char * content_directory) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 1997 | function (callable) | Config | DLLExportClass | (const CNCRulesDataStruct & rules) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2023 | function (callable) | Set_Home_Cell | DLLExportClass | (int x,int y,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2043 | function (callable) | On_Play_Movie | DLLExportClass | (const char * movie_name,ThemeType theme,bool immediate) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2067 | function (callable) | On_Display_Briefing_Text | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2084 | function (callable) | On_Sound_Effect | DLLExportClass | (const HouseClass * player_ptr,int sound_effect_index,const char * extension,int variation,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2150 | function (callable) | On_Speech | DLLExportClass | (const HouseClass * player_ptr,int speech_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2186 | function (callable) | TD_Calculate_Efficiency | DLLExportClass | (unsigned int harvested_credits,unsigned int initial_credits,unsigned int available_credits) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2206 | function (callable) | TD_Calculate_Leadership | DLLExportClass | (int house,unsigned int units_lost,unsigned int buildings_lost) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2236 | function (callable) | TD_Calculate_Score | DLLExportClass | (unsigned int leadership,unsigned int efficiency,unsigned int build_level) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2245 | function (callable) | Calculate_Single_Player_Score | DLLExportClass | (EventCallbackStruct & event) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2324 | function (callable) | Convert_Action_Type | DLLExportClass | (ActionType type,ObjectClass * object,TARGET target,DllActionTypeEnum & dll_type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2386 | function (callable) | On_Game_Over | DLLExportClass | (uint64 glyphx_Player_id,bool player_wins) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2457 | function (callable) | On_Multiplayer_Game_Over | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2575 | function (callable) | On_Message | DLLExportClass | (const HouseClass * player_ptr,const char * message,float timeout_seconds,EventCallbackMessageEnum message_type,int64 message_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2608 | function (callable) | On_Message |  | (const char * message,float timeout_seconds,EventCallbackMessageEnum message_type,int64 message_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2613 | function (callable) | On_Message |  | (const char * message,float timeout_seconds,int64 message_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2618 | function (callable) | On_Defeated_Message |  | (const char * message,float timeout_seconds) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2636 | function (callable) | On_Achievement | DLLExportClass | (const HouseClass * player_ptr,const char * achievement_type,const char * achievement_reason) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2656 | function (callable) | On_Center_Camera | DLLExportClass | (const HouseClass * player_ptr,int coord_x,int coord_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2676 | function (callable) | On_Ping | DLLExportClass | (const HouseClass * player_ptr,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2707 | function (callable) | On_Debug_Output | DLLExportClass | (const char * debug_text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2725 | function (callable) | Force_Human_Team_Wins | DLLExportClass | (uint64 quitting_player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2774 | function (callable) | CNC_Get_Game_State |  | (GameStateRequestEnum state_type,uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2914 | function (callable) | CNC_Handle_Game_Request |  | (GameRequestEnum request_type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2925 | function (callable) | CNC_Handle_Game_Settings_Request |  | (int health_bar_display_mode,int resource_bar_display_mode) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2948 | function (callable) | DLL_Draw_Intercept |  | (int shape_number,int x,int y,int width,int height,int flags,ObjectClass * object,const char * shape_file_name=NULL,char override_owner=HOUSE_NONE,int scale=0x100) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2954 | function (callable) | DLL_Draw_Pip_Intercept |  | (const ObjectClass * object,int pip) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2960 | function (callable) | DLL_Draw_Line_Intercept |  | (int x,int y,int x1,int y1,unsigned char color,int frame) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 2966 | function (callable) | DLL_Draw_Intercept | DLLExportClass | (int shape_number,int x,int y,int width,int height,int flags,ObjectClass * object,const char * shape_file_name,char override_owner,int scale) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3278 | function (callable) | DLL_Draw_Pip_Intercept | DLLExportClass | (const ObjectClass * object,int pip) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3298 | function (callable) | DLL_Draw_Line_Intercept | DLLExportClass | (int x,int y,int x1,int y1,unsigned char color,int frame) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3327 | function (callable) | Get_Layer_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3437 | function (callable) | Convert_Type | DLLExportClass | (const ObjectClass * object,CNCObjectStruct & object_out) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3513 | function (callable) | CNC_Handle_Input |  | (InputRequestEnum input_event,unsigned char special_key_flags,uint64 player_id,int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3748 | function (callable) | CNC_Handle_Structure_Request |  | (StructureRequestEnum request_type,uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3790 | function (callable) | CNC_Handle_Unit_Request |  | (UnitRequestEnum request_type,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3843 | function (callable) | CNC_Handle_Sidebar_Request |  | (SidebarRequestEnum request_type,uint64 player_id,int buildable_type,int buildable_id,short cell_x,short cell_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3896 | function (callable) | CNC_Handle_SuperWeapon_Request |  | (SuperWeaponRequestEnum request_type,uint64 player_id,int buildable_type,int buildable_id,int x1,int y1) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3922 | function (callable) | CNC_Handle_ControlGroup_Request |  | (ControlGroupRequestEnum request_type,uint64 player_id,unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 3957 | function (callable) | Get_Sidebar_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4421 | function (callable) | Calculate_Placement_Distances | DLLExportClass | (BuildingTypeClass * placement_type,unsigned char * placement_distance) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4464 | function (callable) | Recalculate_Placement_Distances |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4469 | function (callable) | Recalculate_Placement_Distances | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4488 | function (callable) | Get_Placement_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4559 | function (callable) | Passes_Proximity_Check | DLLExportClass | (CELL cell_in,BuildingTypeClass * placement_type,unsigned char * placement_distance) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4597 | function (callable) | Start_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4623 | function (callable) | Hold_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4647 | function (callable) | Cancel_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4671 | function (callable) | Construction_Action | DLLExportClass | (SidebarRequestEnum construction_action,uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 4867 | function (callable) | MP_Construction_Action | DLLExportClass | (SidebarRequestEnum construction_action,uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5073 | function (callable) | Start_Placement | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5116 | function (callable) | Cancel_Placement | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5148 | function (callable) | Place | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id,short cell_x,short cell_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5218 | function (callable) | Get_Pending_Placement_Object | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5426 | function (callable) | Place_Super_Weapon | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5448 | function (callable) | Create_Control_Group | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5460 | function (callable) | Add_To_Control_Group | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5472 | function (callable) | Toggle_Control_Group_Selection | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5492 | function (callable) | Get_Shroud_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5586 | function (callable) | Get_Occupier_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5671 | function (callable) | Get_Player_Info_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5767 | function (callable) | Get_Dynamic_Map_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 5888 | function (callable) | Cell_Class_Draw_It | DLLExportClass | (CNCDynamicMapStruct * dynamic_map,int & entry_index,CellClass * cell_ptr,int xpixel,int ypixel,bool debug_output) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6027 | function (callable) | Glyphx_Queue_AI | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6132 | function (callable) | Reset_Sidebars | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6154 | function (callable) | Set_Player_Context | DLLExportClass | (uint64 glyphx_player_id,bool force) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6206 | function (callable) | Reset_Player_Context | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6228 | function (callable) | Logic_Switch_Player_Context |  | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6245 | function (callable) | Logic_Switch_Player_Context | DLLExportClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6277 | function (callable) | Logic_Switch_Player_Context |  | (HouseClass * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6294 | function (callable) | Logic_Switch_Player_Context | DLLExportClass | (HouseClass * house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6345 | function (callable) | Calculate_Start_Positions | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6383 | function (callable) | Get_GlyphX_Player_ID | DLLExportClass | (const HouseClass * house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6431 | function (callable) | Adjust_Internal_View | DLLExportClass | (bool force_ignore_view_constraints) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6465 | function (callable) | Get_Current_Context_Sidebar | DLLExportClass | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6479 | function (callable) | Get_Current_Context_Sidebar |  | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6496 | function (callable) | Repair_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6519 | function (callable) | Repair | DLLExportClass | (uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6557 | function (callable) | Sell_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6580 | function (callable) | Sell | DLLExportClass | (uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6619 | function (callable) | Repair_Sell_Cancel | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6635 | function (callable) | Scatter_Selected | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6660 | function (callable) | Select_Next_Unit | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6687 | function (callable) | Select_Previous_Unit | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6714 | function (callable) | Selected_Guard_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6743 | function (callable) | Selected_Stop | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6771 | function (callable) | Units_Queued_Movement_Toggle | DLLExportClass | (uint64,bool) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6781 | function (callable) | Team_Units_Formation_Toggle_On | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6808 | function (callable) | CNC_Handle_Debug_Request |  | (DebugRequestEnum debug_request_type,uint64 player_id,const char * object_name,int x,int y,bool unshroud,bool enemy) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6896 | function (callable) | CNC_Handle_Beacon_Request |  | (BeaconRequestEnum beacon_request_type,uint64 player_id,int pixel_x,int pixel_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6941 | function (callable) | Try_Debug_Spawn_Unlimbo | DLLExportClass | (TechnoClass * techno,int & cell_x,int & cell_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 6998 | function (callable) | Debug_Spawn_All | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7127 | function (callable) | Debug_Spawn_Unit | DLLExportClass | (const char * object_name,int x,int y,bool enemy) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7248 | function (callable) | Debug_Kill_Unit | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7275 | function (callable) | Debug_Heal_Unit | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7344 | function (callable) | Legacy_Render_Enabled | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7373 | function (callable) | Computer_Message | DLLExportClass | (bool last_player_taunt) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7423 | function (callable) | Set_Special_Key_Flags | DLLExportClass | (unsigned char special_key_flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7439 | function (callable) | Clear_Special_Key_Flags | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7455 | function (callable) | Get_Input_Key_State | DLLExportClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7482 | function (callable) | DLL_Export_Get_Input_Key_State |  | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7489 | function (callable) | DLLSave |  | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7494 | function (callable) | DLLLoad |  | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7511 | function (callable) | Save | DLLExportClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7639 | function (callable) | Load | DLLExportClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7763 | function (callable) | Code_Pointers | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7787 | function (callable) | Decode_Pointers | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7810 | function (callable) | DLL_Code_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.cpp | 7815 | function (callable) | DLL_Decode_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.h | 577 | function (callable) | GameOverMultiPlayerStatsStruct | GameOverMultiPlayerStatsStruct | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.h | 612 | function (callable) | EventCallbackStruct | EventCallbackStruct::EventCallbackStruct | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterface.h | 921 | function (callable) | CarryoverObjectStruct | CarryoverObjectStruct | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 63 | function (callable) | CNC_Editor_Startup |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 85 | function (callable) | CNC_Editor_Cleanup |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 112 | function (callable) | CNC_Editor_Load_Mix_Files |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 132 | function (callable) | CNC_Editor_Setup_Content_Directory |  | (char * cncdata_directory,char * cd_directory) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 147 | function (callable) | CNC_Editor_Setup_Content_Directory |  | (char * cncdata_directory,int CD_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 169 | function (callable) | CNC_Editor_Load_Map |  | (char * cncdata_directory,char * house_name,int scenario_index,char * east_west,char * variant) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 301 | function (callable) | CNC_Editor_Load_Map_By_Scenario_Name |  | (char * cncdata_directory,char * scenario_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 312 | function (callable) | CNC_Editor_Clear_Map |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 347 | function (callable) | CNC_Editor_Get_Map_Stats |  | (int & map_width,int & map_height,int & theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 374 | function (callable) | CNC_Editor_Get_Cell_Data_By_Index |  | (int cell_index,char * cell_name,unsigned long cell_name_size,int & template_type,int & template_icon_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 407 | function (callable) | CNC_Editor_Get_Cell_Data |  | (int x,int y,char * cell_name,unsigned long cell_name_size,int & template_type,int & template_icon_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 451 | function (callable) | CNC_Editor_Get_Cell_Texture_Buffer |  | (int x,int y,int & out_width,int & out_height,SAFEARRAY * & out_texture_array) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 555 | function (callable) | CNC_Editor_Get_Template_Data |  | (int template_type_index,SAFEARRAY * & template_points) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DLLInterfaceEditor.cpp | 612 | function (callable) | CNC_Editor_Get_Scenario_Names |  | (char * cncdata_directory,int CD) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.CPP | 58 | function (callable) | DoorClass | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.CPP | 81 | function (callable) | AI | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.CPP | 118 | function (callable) | Open_Door | DoorClass | (int rate,int stages) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.CPP | 150 | function (callable) | Close_Door | DoorClass | (int rate,int stages) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.CPP | 183 | function (callable) | Door_Stage | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 72 | function (callable) | Time_To_Redraw | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 73 | function (callable) | Clear_Redraw_Flag | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 76 | function (callable) | Is_Door_Opening | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 77 | function (callable) | Is_Door_Closing | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 80 | function (callable) | Is_Door_Open | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 81 | function (callable) | Is_Door_Closed | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 87 | function (callable) | Code_Pointers | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DOOR.H | 88 | function (callable) | Decode_Pointers | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.CPP | 47 | function (callable) | Swap | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.CPP | 84 | function (callable) | Swap | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.CPP | 103 | function (callable) | Copy | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.CPP | 125 | function (callable) | Copy_To | DOSSegmentClass | (void * source,int dest,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.CPP | 149 | function (callable) | Copy_From | DOSSegmentClass | (void * dest,int source,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 61 | function (callable) | Push_DS | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 62 | function (callable) | Pop_DS | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 103 | function (callable) | DOSSegmentClass | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 108 | function (callable) | ~DOSSegmentClass | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 112 | function (callable) | Copy_Word_To | DOSSegmentClass | (short data,int dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 117 | function (callable) | Copy_Byte_To | DOSSegmentClass | (char data,int dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 122 | function (callable) | Copy_DWord_To | DOSSegmentClass | (long data,int dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 127 | function (callable) | DOSSegmentClass | DOSSegmentClass | (unsigned short segment,long) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 132 | function (callable) | Assign | DOSSegmentClass | (unsigned short segment,long) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 137 | function (callable) | Copy_To | DOSSegmentClass | (void * source,int dest,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 142 | function (callable) | Copy_From | DOSSegmentClass | (void * dest,int source,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 147 | function (callable) | Copy | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 151 | function (callable) | Copy_Word_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 156 | function (callable) | Copy_Byte_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 161 | function (callable) | Copy_DWord_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DPMI.H | 166 | function (callable) | Get_Selector | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 58 | function (callable) | DriveClass | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 77 | function (callable) | Do_Turn | DriveClass | (DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 128 | function (callable) | Force_Track | DriveClass | (int track,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 151 | function (callable) | Tiberium_Load | DriveClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 178 | function (callable) | Approach_Target | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 226 | function (callable) | Overrun_Square | DriveClass | (CELL cell,bool threaten) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 291 | function (callable) | DriveClass | DriveClass | (UnitType classid,HousesType house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 334 | function (callable) | Debug_Dump | DriveClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 362 | function (callable) | Exit_Map | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 416 | function (callable) | Smooth_Turn | DriveClass | (COORDINATE adj,DirType * dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 466 | function (callable) | Assign_Destination | DriveClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 584 | function (callable) | While_Moving | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 832 | function (callable) | Per_Cell_Process | DriveClass | (bool center) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 886 | function (callable) | Start_Of_Move | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1290 | function (callable) | AI | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1370 | function (callable) | Fixup_Path | DriveClass | (PathType * path) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1545 | function (callable) | Lay_Track | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1583 | function (callable) | Mark_Track | DriveClass | (COORDINATE headto,MarkType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1641 | function (callable) | Offload_Tiberium_Bail | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1681 | function (callable) | Ok_To_Move | DriveClass | (DirType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.CPP | 1701 | function (callable) | Class_Of | DriveClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.H | 102 | function (callable) | ~DriveClass | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/DRIVE.H | 103 | function (callable) | operator UnitType | DriveClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 80 | function (callable) | EditClass | EditClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w,int h,EditStyle style) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 120 | function (callable) | ~EditClass | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 142 | function (callable) | Set_Text | EditClass | (char * text,int max_len) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 166 | function (callable) | Draw_Me | EditClass | (int forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 216 | function (callable) | Action | EditClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 315 | function (callable) | Draw_Background | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 337 | function (callable) | Draw_Text | EditClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.CPP | 383 | function (callable) | Handle_Key | EditClass | (KeyASCIIType ascii) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.H | 54 | function (callable) | Set_Color | EditClass | (int color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.H | 56 | function (callable) | Set_Read_Only | EditClass | (int rdonly) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.H | 103 | function (callable) | operator \| |  | (EditClass::EditStyle a,EditClass::EditStyle b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.H | 106 | function (callable) | operator & |  | (EditClass::EditStyle a,EditClass::EditStyle b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EDIT.H | 109 | function (callable) | operator ~ |  | (EditClass::EditStyle a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ENDING.CPP | 38 | function (callable) | GDI_Ending |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ENDING.CPP | 111 | function (callable) | Nod_Ending |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 128 | function (callable) | EventClass | EventClass | (SpecialClass data) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 154 | function (callable) | EventClass | EventClass | (EventType type,TARGET target) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 178 | function (callable) | EventClass | EventClass | (EventType type) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 199 | function (callable) | EventClass | EventClass | (EventType type,int val) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 227 | function (callable) | EventClass | EventClass | (EventType type,TARGET src,TARGET dest) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 253 | function (callable) | EventClass | EventClass | (AnimType anim,HousesType owner,COORDINATE coord,int visible) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 286 | function (callable) | EventClass | EventClass | (TARGET src,MissionType mission,TARGET target,TARGET destination) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 317 | function (callable) | EventClass | EventClass | (EventType type,RTTIType object,int id) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 347 | function (callable) | EventClass | EventClass | (EventType type,RTTIType object,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 375 | function (callable) | EventClass | EventClass | (EventType type,int id,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.CPP | 400 | function (callable) | Execute | EventClass | (void) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.H | 201 | function (callable) | EventClass | EventClass | (void) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EVENT.H | 217 | function (callable) | operator == | EventClass | (EventClass & q) | RUST/original-source-reference/TIBERIANDAWN/EVENT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EXPAND.CPP | 39 | function (callable) | Expansion_Present |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EXPAND.CPP | 52 | function (callable) | EListClass | EListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EXPAND.CPP | 60 | function (callable) | Draw_Entry | EListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EXPAND.CPP | 81 | function (callable) | Expansion_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/EXPAND.CPP | 262 | function (callable) | Bonus_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.CPP | 58 | function (callable) | FacingClass | FacingClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.CPP | 83 | function (callable) | Set_Desired | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.CPP | 109 | function (callable) | Set_Current | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.CPP | 139 | function (callable) | Rotation_Adjust | FacingClass | (int rate) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 48 | function (callable) | FacingClass | FacingClass | (DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 49 | function (callable) | operator DirType | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 51 | function (callable) | Current | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 52 | function (callable) | Desired | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 57 | function (callable) | Set | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 62 | function (callable) | Get | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 64 | function (callable) | Is_Rotating | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 66 | function (callable) | Difference | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACING.H | 67 | function (callable) | Difference | FacingClass | (DirType facing) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 73 | function (callable) | Validate | FactoryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 86 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 105 | function (callable) | FactoryClass | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 134 | function (callable) | ~FactoryClass | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 158 | function (callable) | Init | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 179 | function (callable) | operator new | FactoryClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 204 | function (callable) | operator delete | FactoryClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 231 | function (callable) | AI | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 319 | function (callable) | Force_Complete | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 348 | function (callable) | Has_Changed | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 379 | function (callable) | Set | FactoryClass | (TechnoTypeClass const & object,HouseClass & house) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 430 | function (callable) | Set | FactoryClass | (int const & type,HouseClass & house) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 477 | function (callable) | Set | FactoryClass | (TechnoClass & object) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 508 | function (callable) | Suspend | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 536 | function (callable) | Start | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 579 | function (callable) | Abandon | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 633 | function (callable) | Completion | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 655 | function (callable) | Has_Completed | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 682 | function (callable) | Get_Object | FactoryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 699 | function (callable) | Get_Special_Item | FactoryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 720 | function (callable) | Cost_Per_Tick | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.CPP | 751 | function (callable) | Completed | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.H | 64 | function (callable) | Is_Building | FactoryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.H | 74 | function (callable) | Get_House | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.H | 75 | function (callable) | Is_Blocked | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FACTORY.H | 76 | function (callable) | Set_Is_Blocked | FactoryClass | (bool set) | RUST/original-source-reference/TIBERIANDAWN/FACTORY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 38 | function (callable) | FieldClass | FieldClass | (char * id,char data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 48 | function (callable) | FieldClass | FieldClass | (char * id,unsigned char data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 58 | function (callable) | FieldClass | FieldClass | (char * id,short data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 68 | function (callable) | FieldClass | FieldClass | (char * id,unsigned short data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 78 | function (callable) | FieldClass | FieldClass | (char * id,long data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 88 | function (callable) | FieldClass | FieldClass | (char * id,unsigned long data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 98 | function (callable) | FieldClass | FieldClass | (char * id,char * data) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 108 | function (callable) | FieldClass | FieldClass | (char * id,void * data,int length) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 129 | function (callable) | Host_To_Net | FieldClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.CPP | 175 | function (callable) | Net_To_Host | FieldClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FIELD.H | 61 | function (callable) | FieldClass | FieldClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FIELD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 84 | macro (function-like macro) | CELL_FACING |  | (a,b) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 122 | function (callable) | Opposite |  | (FacingType face) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 127 | function (callable) | Draw_Cell_Point |  | (CELL cell,bool passable,int threat_stage,int overide=0) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 152 | function (callable) | Next_Direction |  | (FacingType facing,FacingType dir) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 206 | function (callable) | Point_Relative_To_Line |  | (int x,int z,int x1,int z1,int x2,int z2) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 239 | function (callable) | Unravel_Loop | FootClass | (PathType * path,CELL & cell,FacingType & dir,int sx,int sy,int dx,int dy,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 328 | function (callable) | Register_Cell | FootClass | (PathType * path,CELL cell,FacingType dir,int cost,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 423 | function (callable) | Register_Cell | FootClass | (PathType * path,CELL cell,FacingType dir,int cost,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 528 | function (callable) | Find_Path | FootClass | (CELL dest,FacingType * final_moves,int maxlen,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 934 | function (callable) | Follow_Edge | FootClass | (CELL start,CELL target,PathType * path,FacingType search,FacingType olddir,int threat,int threat_stage,int max_cells,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 1204 | function (callable) | Optimize_Moves | FootClass | (PathType * path,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 1382 | function (callable) | Safety_Point | FootClass | (CELL src,CELL dst,int start,int max) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 1442 | function (callable) | Passable_Cell | FootClass | (CELL cell,FacingType face,int threat,MoveType threshhold) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 1505 | function (callable) | Debug_Draw_Map | FootClass | (char * txt,CELL start,CELL dest,bool pause) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FINDPATH.CPP | 1546 | function (callable) | Debug_Draw_Path | FootClass | (PathType * path) | RUST/original-source-reference/TIBERIANDAWN/FINDPATH.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLASHER.CPP | 56 | function (callable) | Debug_Dump | FlasherClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/FLASHER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLASHER.CPP | 80 | function (callable) | Process | FlasherClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FLASHER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLASHER.CPP | 117 | function (callable) | Get_Flashing_Flags | FlasherClass | () const | RUST/original-source-reference/TIBERIANDAWN/FLASHER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLASHER.H | 55 | function (callable) | FlasherClass | FlasherClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FLASHER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLY.CPP | 59 | function (callable) | Physics | FlyClass | (COORDINATE & coord,DirType facing) | RUST/original-source-reference/TIBERIANDAWN/FLY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLY.CPP | 126 | function (callable) | Fly_Speed | FlyClass | (int speed,MPHType maximum) | RUST/original-source-reference/TIBERIANDAWN/FLY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLY.H | 53 | function (callable) | FlyClass | FlyClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FLY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FLY.H | 63 | function (callable) | Get_Speed | FlyClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FLY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 94 | function (callable) | FootClass | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 134 | function (callable) | ~FootClass | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 156 | function (callable) | FootClass | FootClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 198 | function (callable) | Debug_Dump | FootClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 201 | macro (function-like macro) | Path_To_String |  | (a) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 271 | function (callable) | Set_Speed | FootClass | (int speed) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 300 | function (callable) | Mark | FootClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 351 | function (callable) | Basic_Path | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 553 | function (callable) | Mission_Move | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 582 | function (callable) | Mission_Capture | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 610 | function (callable) | Mission_Attack | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 636 | function (callable) | Mission_Guard | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 661 | function (callable) | Mission_Hunt | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 693 | function (callable) | Mission_Timed_Hunt | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 757 | function (callable) | Stop_Driver | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 786 | function (callable) | Start_Driver | FootClass | (COORDINATE & headto) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 825 | function (callable) | Sort_Y | FootClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 854 | function (callable) | Stun | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 882 | function (callable) | Approach_Target | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 987 | function (callable) | Mission_Guard_Area | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1057 | function (callable) | Unlimbo | FootClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1094 | function (callable) | Assign_Mission | FootClass | (MissionType order) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1118 | function (callable) | Limbo | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1152 | function (callable) | Take_Damage | FootClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1264 | function (callable) | Active_Click_With | FootClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1335 | function (callable) | Active_Click_With | FootClass | (ActionType action,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1390 | function (callable) | Per_Cell_Process | FootClass | (bool center) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1471 | function (callable) | Override_Mission | FootClass | (MissionType mission,TARGET tarcom,TARGET navcom) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1492 | function (callable) | Restore_Mission | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1522 | function (callable) | Receive_Message | FootClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1627 | function (callable) | Mission_Enter | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1696 | function (callable) | Assign_Destination | FootClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1718 | function (callable) | Detach_All | FootClass | (bool all) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1746 | function (callable) | Rescue_Mission | FootClass | (TARGET tarcom) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1823 | function (callable) | Death_Announcement | FootClass | (TechnoClass const * source) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1886 | function (callable) | Greatest_Threat | FootClass | (ThreatType method) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1924 | function (callable) | Detach | FootClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 1976 | function (callable) | Offload_Tiberium_Bail | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 2004 | function (callable) | Can_Enter_Cell | FootClass | (CELL,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 2025 | function (callable) | Can_Demolish | FootClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 2062 | function (callable) | Sell_Back | FootClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.CPP | 2092 | function (callable) | Likely_Coord | FootClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.H | 208 | function (callable) | Head_To_Coord | FootClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FOOT.H | 264 | function (callable) | Fixup_Path | FootClass | (PathType *) | RUST/original-source-reference/TIBERIANDAWN/FOOT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 47 | function (callable) | TCountDownTimerClass | TCountDownTimerClass | (long set=0) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 52 | function (callable) | ~TCountDownTimerClass | TCountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 54 | function (callable) | operator long | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 57 | function (callable) | Set | TCountDownTimerClass | (long set) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 62 | function (callable) | Clear | TCountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 66 | function (callable) | Get_Start | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 69 | function (callable) | Get_Delay | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 72 | function (callable) | Active | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 75 | function (callable) | Expired | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FTIMER.H | 76 | function (callable) | Time | TCountDownTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 197 | function (callable) | Coord_XCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 198 | function (callable) | Coord_YCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 738 | macro (function-like macro) | XYP_COORD |  | (x,y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 739 | function (callable) | Dir_Facing |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 740 | function (callable) | Facing_Dir |  | (FacingType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 741 | function (callable) | Cell_To_Lepton |  | (int cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 742 | function (callable) | Lepton_To_Cell |  | (int lepton) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 743 | function (callable) | XY_Cell |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 744 | function (callable) | XY_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 745 | function (callable) | Coord_X |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 746 | function (callable) | Coord_Y |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 747 | function (callable) | Cell_X |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 748 | function (callable) | Cell_Y |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 749 | function (callable) | Dir_Diff |  | (DirType dir1,DirType dir2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 750 | function (callable) | Coord_XLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 751 | function (callable) | Coord_YLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 753 | function (callable) | Coord_Add |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 754 | function (callable) | Coord_Sub |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 755 | function (callable) | Coord_Snap |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 756 | function (callable) | Coord_Mid |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 757 | function (callable) | Cell_Coord |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 758 | function (callable) | XYPixel_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 760 | function (callable) | Facing_To_32 |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 761 | function (callable) | Direction256 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 762 | function (callable) | Direction |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 763 | function (callable) | Direction8 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 765 | function (callable) | Direction |  | (CELL cell1,CELL cell2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 766 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,FacingType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 767 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 768 | function (callable) | Adjacent_Cell |  | (CELL cell,FacingType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 769 | function (callable) | Adjacent_Cell |  | (CELL cell,DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 770 | function (callable) | Lepton_To_Pixel |  | (int lepton) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 771 | function (callable) | Pixel_To_Lepton |  | (int pixel) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 773 | function (callable) | XYP_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 774 | function (callable) | Text_String |  | (int string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 784 | function (callable) | Random_Picky |  | (T a,T b,char * sfile,int line) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 791 | macro (function-like macro) | Random_Pick |  | (low,high) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 794 | function (callable) | Sim_Random_Pick |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 801 | macro (function-like macro) | Check_Ptr |  | (ptr,file,line) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 811 | macro (function-like macro) | Check_Ptr |  | (ptr,file,line) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 817 | function (callable) | MPlayerID_To_ColorIndex |  | (unsigned short id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 818 | function (callable) | MPlayerID_To_HousesType |  | (unsigned short id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 819 | function (callable) | Build_MPlayerID |  | (int c_idx,HousesType htype) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 893 | function (callable) | Distance |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 928 | function (callable) | Distance |  | (CELL coord1,CELL coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUNCTION.H | 958 | function (callable) | Cell_Number | CellClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.CPP | 54 | function (callable) | FuseClass | FuseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.CPP | 91 | function (callable) | Arm_Fuse | FuseClass | (COORDINATE location,COORDINATE target,int timeto,int arming) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.CPP | 117 | function (callable) | Fuse_Checkup | FuseClass | (COORDINATE newlocation) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.CPP | 163 | function (callable) | Fuse_Write | FuseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.CPP | 186 | function (callable) | Fuse_Read | FuseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/FUSE.H | 87 | function (callable) | Fuse_Target | FuseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/FUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 101 | function (callable) | GadgetClass | GadgetClass | (int x,int y,int w,int h,unsigned flags,int sticky) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 133 | function (callable) | ~GadgetClass | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 153 | function (callable) | Clicked_On | GadgetClass | (KeyNumType & key,unsigned flags,int mousex,int mousey) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 195 | function (callable) | Enable | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 218 | function (callable) | Disable | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 241 | function (callable) | Remove | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 263 | function (callable) | Get_Next | GadgetClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 284 | function (callable) | Get_Prev | GadgetClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 305 | function (callable) | Delete_List | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 351 | function (callable) | Action | GadgetClass | (unsigned flags,KeyNumType &) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 383 | function (callable) | Draw_Me | GadgetClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 408 | function (callable) | Draw_All | GadgetClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 430 | function (callable) | Input | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 636 | function (callable) | Extract_Gadget | GadgetClass | (unsigned id) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 668 | function (callable) | Flag_To_Redraw | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 691 | function (callable) | Sticky_Process | GadgetClass | (unsigned flags) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 718 | function (callable) | Set_Focus | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 745 | function (callable) | Clear_Focus | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 769 | function (callable) | Has_Focus | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.CPP | 789 | function (callable) | Is_List_To_Redraw | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 112 | function (callable) | GadgetClass | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 123 | function (callable) | Flag_List_To_Redraw | GadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 133 | function (callable) | Get_ID | GadgetClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 135 | function (callable) | Peer_To_Peer | GadgetClass | (unsigned,KeyNumType &,ControlClass &) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 236 | function (callable) | operator \| |  | (GadgetClass::FlagEnum a,GadgetClass::FlagEnum b) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 239 | function (callable) | operator & |  | (GadgetClass::FlagEnum a,GadgetClass::FlagEnum b) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GADGET.H | 242 | function (callable) | operator ~ |  | (GadgetClass::FlagEnum a) | RUST/original-source-reference/TIBERIANDAWN/GADGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAMEDLG.CPP | 54 | function (callable) | Process | GameControlsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GAMEDLG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAMEDLG.H | 43 | function (callable) | GameControlsClass | GameControlsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GAMEDLG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 59 | function (callable) | GaugeClass | GaugeClass | (unsigned id,int x,int y,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 90 | function (callable) | Set_Maximum | GaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 117 | function (callable) | Set_Value | GaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 146 | function (callable) | Pixel_To_Value | GaugeClass | (int pixel) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 181 | function (callable) | Value_To_Pixel | GaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 210 | function (callable) | Draw_Me | GaugeClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 272 | function (callable) | Action | GaugeClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 367 | function (callable) | Draw_Thumb | GaugeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 399 | function (callable) | TriColorGaugeClass | TriColorGaugeClass | (unsigned id,int x,int y,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 418 | function (callable) | Set_Red_Limit | TriColorGaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 446 | function (callable) | Set_Yellow_Limit | TriColorGaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.CPP | 474 | function (callable) | Draw_Me | TriColorGaugeClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.H | 48 | function (callable) | Get_Value | GaugeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.H | 49 | function (callable) | Use_Thumb | GaugeClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GAUGE.H | 51 | function (callable) | Thumb_Pixels | GaugeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GAUGE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GOPTIONS.CPP | 48 | function (callable) | Adjust_Variables_For_Resolution | GameOptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GOPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GOPTIONS.CPP | 80 | function (callable) | Process | GameOptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GOPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GOPTIONS.CPP | 507 | function (callable) | Draw_Caption |  | (int text,int x,int y,int w) | RUST/original-source-reference/TIBERIANDAWN/GOPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GOPTIONS.H | 79 | function (callable) | GameOptionsClass | GameOptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/GOPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 71 | function (callable) | GScreenClass | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 102 | function (callable) | One_Time | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 136 | function (callable) | Init | GScreenClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 163 | function (callable) | Init_Clear | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 196 | function (callable) | Init_Theater | GScreenClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 216 | function (callable) | Init_IO | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 247 | function (callable) | Flag_To_Redraw | GScreenClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 273 | function (callable) | Input | GScreenClass | (KeyNumType & key,int & x,int & y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 322 | function (callable) | Add_A_Button | GScreenClass | (GadgetClass & gadget) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 363 | function (callable) | Remove_A_Button | GScreenClass | (GadgetClass & gadget) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 387 | function (callable) | Render | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 445 | function (callable) | Add_Current_Screen |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.CPP | 493 | function (callable) | Blit_Display | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.H | 60 | function (callable) | AI | GScreenClass | (KeyNumType &,int,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.H | 79 | function (callable) | Draw_It | GScreenClass | (bool=false) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/GSCREEN.H | 103 | function (callable) | Shadow_Address | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HDATA.CPP | 216 | function (callable) | HouseTypeClass | HouseTypeClass | (HousesType house,char const * ini,int fullname,char const * ext,int lemon,int color,int bright_color,PlayerColorType remapcolor,unsigned char const * remap,char prefix) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HDATA.CPP | 266 | function (callable) | From_Name | HouseTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HDATA.CPP | 291 | function (callable) | One_Time | HouseTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HDATA.CPP | 319 | function (callable) | As_Reference | HouseTypeClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 77 | function (callable) | FixedHeapClass | FixedHeapClass | (int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 102 | function (callable) | ~FixedHeapClass | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 128 | function (callable) | Set_Heap | FixedHeapClass | (int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 187 | function (callable) | Allocate | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 218 | function (callable) | Free | FixedHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 253 | function (callable) | ID | FixedHeapClass | (void const * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 278 | function (callable) | Clear | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 309 | function (callable) | Free_All | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 335 | function (callable) | Free_All | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 342 | function (callable) | Clear | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 349 | function (callable) | Set_Heap | FixedIHeapClass | (int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 360 | function (callable) | Allocate | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 387 | function (callable) | Free | FixedIHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 409 | function (callable) | Save | TFixedIHeapClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 459 | function (callable) | Load | TFixedIHeapClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 524 | function (callable) | Code_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.CPP | 547 | function (callable) | Decode_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 54 | function (callable) | Count | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 55 | function (callable) | Length | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 56 | function (callable) | Avail | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 65 | function (callable) | operator [] | FixedHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 117 | function (callable) | TFixedHeapClass | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 118 | function (callable) | ~TFixedHeapClass | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 120 | function (callable) | ID | TFixedHeapClass | (T const * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 122 | function (callable) | Alloc | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 123 | function (callable) | Free | TFixedHeapClass | (T * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 126 | function (callable) | operator [] | TFixedHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 139 | function (callable) | FixedIHeapClass | FixedIHeapClass | (int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 140 | function (callable) | ~FixedIHeapClass | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 148 | function (callable) | Active_Ptr | FixedIHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 169 | function (callable) | TFixedIHeapClass | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 170 | function (callable) | ~TFixedIHeapClass | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 172 | function (callable) | ID | TFixedIHeapClass | (T const * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 173 | function (callable) | Alloc | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 174 | function (callable) | Free | TFixedIHeapClass | (T * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 175 | function (callable) | Free | TFixedIHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 181 | function (callable) | Ptr | TFixedIHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 182 | function (callable) | Raw_Ptr | TFixedIHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 199 | function (callable) | Save | TFixedIHeapClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 249 | function (callable) | Load | TFixedIHeapClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 314 | function (callable) | Code_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HEAP.H | 337 | function (callable) | Decode_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 79 | function (callable) | HelpClass | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 104 | function (callable) | Init_Clear | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 128 | function (callable) | Overlap_List | HelpClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 158 | function (callable) | AI | HelpClass | (KeyNumType & key,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 217 | function (callable) | Help_Text | HelpClass | (int text,int x,int y,int color,bool quick,int cost) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 268 | function (callable) | Draw_It | HelpClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 314 | function (callable) | Set_Text | HelpClass | (int text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 365 | function (callable) | Scroll_Map | HelpClass | (DirType facing,int & distance,bool really) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 391 | function (callable) | Set_Cost | HelpClass | (int cost) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HELP.CPP | 397 | function (callable) | Set_Tactical_Position | HelpClass | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 125 | function (callable) | Validate | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 138 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 157 | function (callable) | operator HousesType | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 181 | function (callable) | As_Pointer | HouseClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 207 | function (callable) | One_Time | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 239 | function (callable) | Assign_Handicap | HouseClass | (DiffType handicap) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 287 | function (callable) | Debug_Dump | HouseClass | (MonoClass *) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 312 | function (callable) | operator new | HouseClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 337 | function (callable) | operator delete | HouseClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 361 | function (callable) | HouseClass | HouseClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 528 | function (callable) | ~HouseClass | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 553 | function (callable) | Can_Build | HouseClass | (TechnoTypeClass const * type,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 762 | function (callable) | Can_Build | HouseClass | (StructType s,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 793 | function (callable) | Can_Build | HouseClass | (InfantryType infantry,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 822 | function (callable) | Can_Build | HouseClass | (UnitType unit,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 848 | function (callable) | Can_Build | HouseClass | (AircraftType aircraft,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 871 | function (callable) | Init | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 904 | function (callable) | AI | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1672 | function (callable) | Attacked | HouseClass | (BuildingClass * source) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1717 | function (callable) | Harvested | HouseClass | (unsigned tiberium) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1747 | function (callable) | Available_Money | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1771 | function (callable) | Spend_Money | HouseClass | (unsigned money) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1805 | function (callable) | Refund_Money | HouseClass | (unsigned money) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1829 | function (callable) | Adjust_Capacity | HouseClass | (int adjust,bool inanger) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1871 | function (callable) | Silo_Redraw_Check | HouseClass | (long oldtib,long oldcap) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1907 | function (callable) | Read_INI | HouseClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 1978 | function (callable) | Write_INI | HouseClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2018 | function (callable) | Is_Ally | HouseClass | (HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2042 | function (callable) | Is_Ally | HouseClass | (HouseClass const * house) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2066 | function (callable) | Is_Ally | HouseClass | (ObjectClass const * object) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2092 | function (callable) | Make_Ally | HouseClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2158 | function (callable) | Make_Enemy | HouseClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2198 | function (callable) | Remap_Table | HouseClass | (bool blushing,bool unit) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2251 | function (callable) | Suggested_New_Team | HouseClass | (bool alertcheck) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2275 | function (callable) | Adjust_Threat | HouseClass | (int region,int threat) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2325 | function (callable) | Begin_Production | HouseClass | (RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2431 | function (callable) | Suspend_Production | HouseClass | (RTTIType type) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2515 | function (callable) | Abandon_Production | HouseClass | (RTTIType type) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2618 | function (callable) | Special_Weapon_AI | HouseClass | (SpecialWeaponType id) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2669 | function (callable) | Place_Special_Blast | HouseClass | (SpecialWeaponType id,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2823 | function (callable) | Place_Object | HouseClass | (RTTIType type,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 2981 | function (callable) | Manual_Place | HouseClass | (BuildingClass * builder,BuildingClass * object) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3024 | function (callable) | Init_Ion_Cannon | HouseClass | (SpecialControlType control) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3100 | function (callable) | Init_Ion_Cannon | HouseClass | (bool first_time,bool one_time_effect) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3150 | function (callable) | Remove_Ion_Cannon | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3180 | function (callable) | Clobber_All | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3242 | function (callable) | Init_Nuke_Bomb | HouseClass | (bool first_time,bool one_time_effect) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3287 | function (callable) | Remove_Nuke_Bomb | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3314 | function (callable) | Init_Air_Strike | HouseClass | (bool first_time,bool one_time_effect) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3366 | function (callable) | Remove_Air_Strike | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3391 | function (callable) | Make_Air_Strike_Available | HouseClass | (bool present,bool one_time_effect) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3416 | function (callable) | Add_Nuke_Piece | HouseClass | (int piece) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3453 | function (callable) | Detach | HouseClass | (TARGET,bool) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3477 | function (callable) | Does_Enemy_Building_Exist | HouseClass | (StructType btype) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3509 | function (callable) | Suggest_New_Object | HouseClass | (RTTIType objecttype) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3811 | function (callable) | Flag_Remove | HouseClass | (TARGET target,bool set_home) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3876 | function (callable) | Flag_Attach | HouseClass | (CELL cell,bool set_home) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 3994 | function (callable) | Flag_Attach | HouseClass | (UnitClass * object,bool set_home) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4027 | function (callable) | MPlayer_Defeated | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4368 | function (callable) | Blowup_All | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4510 | function (callable) | Flag_To_Die | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4540 | function (callable) | Flag_To_Win | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4571 | function (callable) | Flag_To_Lose | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4606 | function (callable) | Init_Data | HouseClass | (PlayerColorType color,HousesType house,int credits) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4669 | function (callable) | Power_Fraction | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4700 | function (callable) | Has_Nuke_Device | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4723 | function (callable) | Sell_Wall | HouseClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4800 | function (callable) | Check_Pertinent_Structures | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4871 | function (callable) | Init_Unit_Trackers | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 4901 | function (callable) | Free_Unit_Trackers | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5000 | macro (function-like macro) | ARRAY_SIZE |  | (a) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5011 | function (callable) | Percent_Chance |  | (int percent) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5029 | function (callable) | Round_Up |  | (unsigned short val) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5041 | function (callable) | fixed |  | (int val) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5061 | function (callable) | Suggest_New_Building | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5093 | function (callable) | Find_Building | HouseClass | (StructType type,ZoneType zone) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5135 | function (callable) | Find_Build_Location | HouseClass | (BuildingClass * building) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5253 | function (callable) | Recalc_Center | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5383 | function (callable) | Expert_AI | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5679 | function (callable) | Check_Build_Power | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5714 | function (callable) | Check_Build_Defense | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5727 | function (callable) | Check_Build_Offense | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5743 | function (callable) | Check_Attack | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5757 | function (callable) | Check_Build_Income | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5771 | function (callable) | Check_Fire_Sale | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5787 | function (callable) | Check_Build_Engineer | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5804 | function (callable) | Check_Raise_Money | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5823 | function (callable) | Check_Lower_Power | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5840 | function (callable) | Check_Raise_Power | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5857 | function (callable) | AI_Attack | HouseClass | (UrgencyType) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5918 | function (callable) | AI_Build_Power | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5930 | function (callable) | AI_Build_Defense | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5941 | function (callable) | AI_Build_Offense | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5952 | function (callable) | AI_Build_Income | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5960 | function (callable) | AI_Fire_Sale | HouseClass | (UrgencyType urgency) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5975 | function (callable) | AI_Build_Engineer | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 5986 | function (callable) | AI_Lower_Power | HouseClass | (UrgencyType) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 6022 | function (callable) | AI_Raise_Power | HouseClass | (UrgencyType urgency) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 6094 | function (callable) | AI_Raise_Money | HouseClass | (UrgencyType urgency) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 6181 | function (callable) | AI_Base_Defense | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.CPP | 6270 | function (callable) | AI_Building | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 416 | function (callable) | HouseClass | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 441 | function (callable) | Make_Ally | HouseClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 443 | function (callable) | Make_Enemy | HouseClass | (ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 447 | function (callable) | Get_Allies | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 467 | function (callable) | Adjust_Power | HouseClass | (int adjust) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 468 | function (callable) | Adjust_Drain | HouseClass | (int adjust) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 472 | function (callable) | Tiberium_Fraction | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 473 | function (callable) | Begin_Production | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 572 | function (callable) | Can_Make_Money | HouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 881 | function (callable) | operator new | HouseClass::BuildChoiceClass | (size_t,void * ptr) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 885 | function (callable) | BuildChoiceClass | HouseClass::BuildChoiceClass | (UrgencyType u,StructType s) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 886 | function (callable) | Save | HouseClass::BuildChoiceClass | (FileClass &) const | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 887 | function (callable) | Load | HouseClass::BuildChoiceClass | (FileClass &) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 888 | function (callable) | Code_Pointers | HouseClass::BuildChoiceClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/HOUSE.H | 889 | function (callable) | Decode_Pointers | HouseClass::BuildChoiceClass | (void) | RUST/original-source-reference/TIBERIANDAWN/HOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1620 | function (callable) | InfantryTypeClass | InfantryTypeClass | (InfantryType type,int name,char const * ininame,unsigned char level,long pre,bool is_female,bool is_leader,bool is_crawling,bool is_civilian,bool is_nominal,bool is_fraidycat,bool is_capture,bool is_theater,int ammo,int * do_table,int firelaunch,int pronelaunch,unsigned short strength,int sightrange,int cost,int scenario,int risk,int reward,int ownable,WeaponType primary,WeaponType secondary,MPHType maxspeed) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1823 | function (callable) | Create_One_Of | InfantryTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1847 | function (callable) | Create_And_Place | InfantryTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1880 | function (callable) | Occupy_List | InfantryTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1909 | function (callable) | Display | InfantryTypeClass | (int x,int y,WindowNumberType window,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1940 | function (callable) | Prep_For_Add | InfantryTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1966 | function (callable) | From_Name | InfantryTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 1994 | function (callable) | One_Time | InfantryTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 2045 | function (callable) | Init | InfantryTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 2107 | function (callable) | Who_Can_Build_Me | InfantryTypeClass | (bool intheory,bool legal,HousesType house) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IDATA.CPP | 2147 | function (callable) | Full_Name | InfantryTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 154 | function (callable) | Validate | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 167 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 187 | function (callable) | Debug_Dump | InfantryClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 219 | function (callable) | InfantryClass | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 238 | function (callable) | InfantryClass | InfantryClass | (InfantryType classid,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 300 | function (callable) | ~InfantryClass | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 331 | function (callable) | operator new | InfantryClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 355 | function (callable) | operator delete | InfantryClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 390 | function (callable) | Take_Damage | InfantryClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 580 | function (callable) | Draw_It | InfantryClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 669 | function (callable) | Per_Cell_Process | InfantryClass | (bool center) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 805 | function (callable) | Detach | InfantryClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 830 | function (callable) | As_Target | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 852 | function (callable) | Init | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 882 | function (callable) | Look | InfantryClass | (bool incremental) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 915 | function (callable) | Assign_Destination | InfantryClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 995 | function (callable) | Assign_Target | InfantryClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1029 | function (callable) | AI | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1483 | function (callable) | Blocking_Object | InfantryClass | (TechnoClass const * techno,CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1566 | function (callable) | Can_Enter_Cell | InfantryClass | (CELL cell,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1778 | function (callable) | Overlap_List | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1804 | function (callable) | Can_Fire | InfantryClass | (TARGET target,int which) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1873 | function (callable) | Enter_Idle_Mode | InfantryClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 1937 | function (callable) | Random_Animate | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2050 | function (callable) | Scatter | InfantryClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2131 | function (callable) | Do_Action | InfantryClass | (DoType todo,bool force) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2204 | function (callable) | Stop_Driver | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2253 | function (callable) | Start_Driver | InfantryClass | (COORDINATE & headto) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2303 | function (callable) | Set_Primary_Facing | InfantryClass | (DirType facing,bool) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2326 | function (callable) | Limbo | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2357 | function (callable) | Fire_At | InfantryClass | (TARGET target,int which) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2437 | function (callable) | Unlimbo | InfantryClass | (COORDINATE coord,DirType facing) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2490 | function (callable) | Greatest_Threat | InfantryClass | (ThreatType threat) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2559 | function (callable) | Response_Select | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2622 | function (callable) | Response_Move | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2684 | function (callable) | Response_Attack | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2740 | function (callable) | Fire_Coord | InfantryClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2773 | function (callable) | Receive_Message | InfantryClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2837 | function (callable) | Rearm_Delay | InfantryClass | (bool second) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2863 | function (callable) | Assign_Mission | InfantryClass | (MissionType order) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2891 | function (callable) | What_Action | InfantryClass | (ObjectClass * object) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 2962 | function (callable) | Read_INI | InfantryClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3084 | function (callable) | Write_INI | InfantryClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3146 | function (callable) | Active_Click_With | InfantryClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3186 | function (callable) | Made_A_Kill | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3209 | function (callable) | Set_Occupy_Bit | InfantryClass | (CELL cell,int spot_index) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3241 | function (callable) | Clear_Occupy_Bit | InfantryClass | (CELL cell,int spot_index) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3281 | function (callable) | Full_Name | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3307 | function (callable) | Mission_Attack | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3319 | function (callable) | What_Am_I | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3325 | function (callable) | What_Action | InfantryClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3331 | function (callable) | Class_Of | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.CPP | 3337 | function (callable) | Is_Infantry | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.H | 53 | function (callable) | operator InfantryType | InfantryClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.H | 177 | function (callable) | Set_Occupy_Bit | InfantryClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.H | 179 | function (callable) | Clear_Occupy_Bit | InfantryClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INFANTRY.H | 208 | function (callable) | INI_Name | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/INFANTRY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 81 | function (callable) | Set_Scenario_Name |  | (char * buf,int scenario,ScenarioPlayerType player,ScenarioDirType dir,ScenarioVarType var) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 210 | function (callable) | Read_Scenario_Ini |  | (char * root,bool fresh) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 708 | function (callable) | Read_Scenario_Ini_File |  | (char * scenario_file_name,char * bin_file_name,const char * root,bool fresh) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1086 | function (callable) | Read_Movies_From_Scenario_Ini |  | (char * root,bool fresh) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1227 | function (callable) | Write_Scenario_Ini |  | (char * root) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1350 | function (callable) | Assign_Houses |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1485 | function (callable) | Remove_AI_Players |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1548 | function (callable) | Reserve_Infantry |  | () | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1557 | function (callable) | Reserve_Unit |  | () | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 1565 | function (callable) | Create_Units |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 2010 | function (callable) | Scan_Place_Object |  | (ObjectClass * obj,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 2118 | function (callable) | Sort_Cells |  | (CELL * cells,int numcells,CELL * outcells) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 2172 | function (callable) | Furthest_Cell |  | (CELL * cells,int numcells,CELL * tcells,int numtcells) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 2234 | function (callable) | Clip_Scatter |  | (CELL cell,int maxdist) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INI.CPP | 2309 | function (callable) | Clip_Move |  | (CELL cell,FacingType facing,int dist) | RUST/original-source-reference/TIBERIANDAWN/INI.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 66 | function (callable) | Start_Game_Proc |  | (HWND hwnd,UINT message,UINT wParam,LONG lParam) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 107 | function (callable) | Init_Game |  | (int,char * []) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 722 | function (callable) | Uninit_Game |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 785 | function (callable) | Select_Game |  | (bool fade) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 1773 | function (callable) | Play_Intro |  | (bool for_real) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 1931 | function (callable) | Anim_Init |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2010 | function (callable) | Parse_Command_Line |  | (int argc,char * argv[]) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2554 | function (callable) | Parse_INI_File |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2643 | function (callable) | Version_Number |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2786 | function (callable) | Save_Recording_Values |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2836 | function (callable) | Load_Recording_Values |  | (void) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INIT.CPP | 2896 | function (callable) | Obfuscate |  | (char const * string) | RUST/original-source-reference/TIBERIANDAWN/INIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 96 | function (callable) | Check_From_WChat |  | (char * wchat_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 236 | function (callable) | Read_Game_Options |  | (char * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 337 | function (callable) | Just_Path |  | (char * path,char * destpath) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 366 | function (callable) | Is_User_WChat_Registered |  | (char * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 467 | function (callable) | Spawn_WChat |  | (bool can_launch) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 612 | function (callable) | Spawn_Registration_App |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERNET.CPP | 676 | function (callable) | Do_The_Internet_Menu_Thang |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERPAL.CPP | 87 | function (callable) | Read_Interpolation_Palette |  | (char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERPAL.CPP | 115 | function (callable) | Write_Interpolation_Palette |  | (char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERPAL.CPP | 142 | function (callable) | Create_Palette_Interpolation_Table |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERPAL.CPP | 267 | function (callable) | Increase_Palette_Luminance |  | (unsigned char * palette,int red_percentage,int green_percentage,int blue_percentage,int cap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTERPAL.CPP | 321 | function (callable) | Interpolate_2X_Scale |  | (GraphicBufferClass * source,GraphicViewPortClass * dest,char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTRO.CPP | 43 | function (callable) | Open_Movie |  | (char * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/INTRO.CPP | 76 | function (callable) | Choose_Side |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 85 | function (callable) | Should_Save | CellClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 115 | function (callable) | Load | CellClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 152 | function (callable) | Save | CellClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 197 | function (callable) | Code_Pointers | CellClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 247 | function (callable) | Decode_Pointers | CellClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 339 | function (callable) | Load | MouseClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 439 | function (callable) | Save | MouseClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 515 | function (callable) | Code_Pointers | MouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 541 | function (callable) | Decode_Pointers | MouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 569 | function (callable) | Code_Pointers | ScrollClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 593 | function (callable) | Decode_Pointers | ScrollClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 619 | function (callable) | Code_Pointers | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 643 | function (callable) | Decode_Pointers | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 669 | function (callable) | Code_Pointers | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 693 | function (callable) | Decode_Pointers | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 719 | function (callable) | Code_Pointers | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 743 | function (callable) | Decode_Pointers | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 769 | function (callable) | Code_Pointers | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 797 | function (callable) | Decode_Pointers | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 827 | function (callable) | Code_Pointers | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 850 | function (callable) | Decode_Pointers | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 875 | function (callable) | Code_Pointers | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 899 | function (callable) | Decode_Pointers | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 925 | function (callable) | Code_Pointers | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 976 | function (callable) | Decode_Pointers | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 1021 | function (callable) | Code_Pointers | MapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 1061 | function (callable) | Decode_Pointers | MapClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 1096 | function (callable) | Code_Pointers | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOMAP.CPP | 1119 | function (callable) | Decode_Pointers | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 150 | function (callable) | Load | TeamTypeClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 171 | function (callable) | Save | TeamTypeClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 197 | function (callable) | Code_Pointers | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 226 | function (callable) | Decode_Pointers | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 250 | function (callable) | Load | TeamClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 271 | function (callable) | Save | TeamClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 297 | function (callable) | Code_Pointers | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 335 | function (callable) | Decode_Pointers | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 384 | function (callable) | Load | TriggerClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 416 | function (callable) | Save | TriggerClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 442 | function (callable) | Code_Pointers | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 468 | function (callable) | Decode_Pointers | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 489 | function (callable) | Load | AircraftClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 510 | function (callable) | Save | AircraftClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 536 | function (callable) | Code_Pointers | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 569 | function (callable) | Decode_Pointers | AircraftClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 597 | function (callable) | Load | AnimClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 618 | function (callable) | Save | AnimClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 644 | function (callable) | Code_Pointers | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 691 | function (callable) | Decode_Pointers | AnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 735 | function (callable) | Load | BuildingClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 756 | function (callable) | Save | BuildingClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 782 | function (callable) | Code_Pointers | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 823 | function (callable) | Decode_Pointers | BuildingClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 858 | function (callable) | Load | BulletClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 879 | function (callable) | Save | BulletClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 905 | function (callable) | Code_Pointers | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 945 | function (callable) | Decode_Pointers | BulletClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 982 | function (callable) | Load | InfantryClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1003 | function (callable) | Save | InfantryClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1029 | function (callable) | Code_Pointers | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1061 | function (callable) | Decode_Pointers | InfantryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1088 | function (callable) | Load | OverlayClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1109 | function (callable) | Save | OverlayClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1135 | function (callable) | Code_Pointers | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1167 | function (callable) | Decode_Pointers | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1194 | function (callable) | Load | SmudgeClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1215 | function (callable) | Save | SmudgeClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1241 | function (callable) | Code_Pointers | SmudgeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1273 | function (callable) | Decode_Pointers | SmudgeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1300 | function (callable) | Load | TemplateClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1321 | function (callable) | Save | TemplateClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1347 | function (callable) | Code_Pointers | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1379 | function (callable) | Decode_Pointers | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1406 | function (callable) | Load | TerrainClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1427 | function (callable) | Save | TerrainClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1453 | function (callable) | Code_Pointers | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1486 | function (callable) | Decode_Pointers | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1514 | function (callable) | Load | UnitClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1535 | function (callable) | Save | UnitClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1561 | function (callable) | Code_Pointers | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1589 | function (callable) | Decode_Pointers | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1612 | function (callable) | Load | FactoryClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1633 | function (callable) | Save | FactoryClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1659 | function (callable) | Code_Pointers | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1689 | function (callable) | Decode_Pointers | FactoryClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1715 | function (callable) | Load | LayerClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1759 | function (callable) | Save | LayerClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1805 | function (callable) | Code_Pointers | LayerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1834 | function (callable) | Decode_Pointers | LayerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1858 | function (callable) | Load | HouseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1879 | function (callable) | Save | HouseClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1905 | function (callable) | Code_Pointers | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1932 | function (callable) | Decode_Pointers | HouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1954 | function (callable) | Load | ScoreClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1975 | function (callable) | Save | ScoreClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 1996 | function (callable) | Code_Pointers | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2016 | function (callable) | Decode_Pointers | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2041 | function (callable) | Code_Pointers | FlyClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2064 | function (callable) | Decode_Pointers | FlyClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2089 | function (callable) | Code_Pointers | FuseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2112 | function (callable) | Decode_Pointers | FuseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2137 | function (callable) | Code_Pointers | TarComClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2161 | function (callable) | Decode_Pointers | TarComClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2187 | function (callable) | Code_Pointers | TurretClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2211 | function (callable) | Decode_Pointers | TurretClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2237 | function (callable) | Code_Pointers | DriveClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2269 | function (callable) | Decode_Pointers | DriveClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2304 | function (callable) | Code_Pointers | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2335 | function (callable) | Decode_Pointers | FootClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2371 | function (callable) | Code_Pointers | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2402 | function (callable) | Decode_Pointers | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2436 | function (callable) | Code_Pointers | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2470 | function (callable) | Decode_Pointers | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2507 | function (callable) | Code_Pointers | FlasherClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2530 | function (callable) | Decode_Pointers | FlasherClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2555 | function (callable) | Code_Pointers | CargoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2584 | function (callable) | Decode_Pointers | CargoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2616 | function (callable) | Code_Pointers | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2640 | function (callable) | Decode_Pointers | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2666 | function (callable) | Code_Pointers | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IOOBJ.CPP | 2696 | function (callable) | Decode_Pointers | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/IOOBJ.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 78 | function (callable) | IPX_SPX_Installed |  | (void) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 186 | function (callable) | IPX_Open_Socket |  | (unsigned short socket) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 242 | function (callable) | IPX_Close_Socket |  | (unsigned short socket) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 298 | function (callable) | IPX_Get_Connection_Number |  | (void) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 355 | function (callable) | IPX_Get_1st_Connection_Num |  | (char * username) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 499 | function (callable) | IPX_Get_Internet_Address |  | (int connection_number,unsigned char * network_number,unsigned char * physical_node) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 627 | function (callable) | IPX_Get_User_ID |  | (int connection_number,char * user_id) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 774 | function (callable) | IPX_Listen_For_Packet |  | (struct ECB * ecb_ptr) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 851 | function (callable) | IPX_Send_Packet |  | (struct ECB * ecb_ptr) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 910 | function (callable) | IPX_Get_Local_Target |  | (unsigned char * dest_network,unsigned char * dest_node,unsigned short dest_socket,unsigned char * bridge_address) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 1025 | function (callable) | IPX_Cancel_Event |  | (struct ECB * ecb_ptr) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX.CPP | 1082 | function (callable) | Let_IPX_Breath |  | (void) | RUST/original-source-reference/TIBERIANDAWN/IPX.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 49 | function (callable) | IPX_Initialise |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 50 | function (callable) | IPX_Get_Outstanding_Buffer95 |  | (unsigned char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 51 | function (callable) | IPX_Shut_Down95 |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 52 | function (callable) | IPX_Send_Packet95 |  | (unsigned char *,unsigned char *,int,unsigned char *,unsigned char *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 53 | function (callable) | IPX_Broadcast_Packet95 |  | (unsigned char *,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 54 | function (callable) | IPX_Start_Listening95 |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 55 | function (callable) | IPX_Open_Socket95 |  | (int socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 56 | function (callable) | IPX_Close_Socket95 |  | (int socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 57 | function (callable) | IPX_Get_Connection_Number95 |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 58 | function (callable) | IPX_Get_Local_Target95 |  | (unsigned char *,unsigned char *,unsigned short,unsigned char *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 63 | function (callable) | IPX_Open_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 70 | function (callable) | IPX_Close_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 77 | function (callable) | IPX_Get_Connection_Number |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 85 | function (callable) | IPX_Broadcast_Packet |  | (unsigned char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPX95.CPP | 96 | function (callable) | IPX_Get_Local_Target |  | (unsigned char * dest_network,unsigned char * dest_node,unsigned short dest_socket,unsigned char * bridge_address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 66 | function (callable) | IPXAddressClass | IPXAddressClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 98 | function (callable) | IPXAddressClass | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 124 | function (callable) | IPXAddressClass | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 148 | function (callable) | Set_Address | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 174 | function (callable) | Set_Address | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 231 | function (callable) | Get_Address | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 255 | function (callable) | Get_Address | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 278 | function (callable) | Is_Broadcast | IPXAddressClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 316 | function (callable) | operator == | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 370 | function (callable) | operator != | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 419 | function (callable) | operator > | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 441 | function (callable) | operator < | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 463 | function (callable) | operator >= | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXADDR.CPP | 485 | function (callable) | operator <= | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 89 | function (callable) | IPXConnClass | IPXConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,IPXAddressClass * address,int id,char * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 161 | function (callable) | Init | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 204 | function (callable) | Configure | IPXConnClass | (unsigned short socket,int conn_num,ECBType * listen_ecb,ECBType * send_ecb,IPXHeaderType * listen_header,IPXHeaderType * send_header,char * listen_buf,char * send_buf,long handler_rm_ptr,int maxpacketlen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 249 | function (callable) | Start_Listening | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 347 | function (callable) | Stop_Listening | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 400 | function (callable) | Send | IPXConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 429 | function (callable) | Open_Socket | IPXConnClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 483 | function (callable) | Close_Socket | IPXConnClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 531 | function (callable) | Send_To | IPXConnClass | (char * buf,int buflen,IPXAddressClass * address,NetNodeType immed) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.CPP | 640 | function (callable) | Broadcast | IPXConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXCONN.H | 82 | function (callable) | ~IPXConnClass | IPXConnClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 68 | function (callable) | IPXGlobalConnClass | IPXGlobalConnClass | (int numsend,int numreceive,int maxlen,unsigned short product_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 106 | function (callable) | Send_Packet | IPXGlobalConnClass | (void * buf,int buflen,IPXAddressClass * address,int ack_req) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 177 | function (callable) | Receive_Packet | IPXGlobalConnClass | (void * buf,int buflen,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 268 | function (callable) | Get_Packet | IPXGlobalConnClass | (void * buf,int * buflen,IPXAddressClass * address,unsigned short * product_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 334 | function (callable) | Send | IPXGlobalConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 386 | function (callable) | Service_Receive_Queue | IPXGlobalConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.CPP | 457 | function (callable) | Set_Bridge | IPXGlobalConnClass | (NetNumType bridge) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXGCONN.H | 107 | function (callable) | ~IPXGlobalConnClass | IPXGlobalConnClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 89 | function (callable) | IPXManagerClass | IPXManagerClass | (int glb_maxlen,int pvt_maxlen,int glb_num_packets,int pvt_num_packets,unsigned short socket,unsigned short product_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 183 | function (callable) | ~IPXManagerClass | IPXManagerClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 236 | function (callable) | Init | IPXManagerClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 338 | function (callable) | Is_IPX | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 366 | function (callable) | Set_Timing | IPXManagerClass | (unsigned long retrydelta,unsigned long maxretries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 407 | function (callable) | Create_Connection | IPXManagerClass | (int id,char * name,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 479 | function (callable) | Delete_Connection | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 543 | function (callable) | Num_Connections | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 578 | function (callable) | Connection_ID | IPXManagerClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 603 | function (callable) | Connection_Name | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 633 | function (callable) | Connection_Address | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 663 | function (callable) | Connection_Index | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 695 | function (callable) | Send_Global_Message | IPXManagerClass | (void * buf,int buflen,int ack_req,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 731 | function (callable) | Get_Global_Message | IPXManagerClass | (void * buf,int * buflen,IPXAddressClass * address,unsigned short * product_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 761 | function (callable) | Send_Private_Message | IPXManagerClass | (void * buf,int buflen,int ack_req,int conn_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 877 | function (callable) | Get_Private_Message | IPXManagerClass | (void * buf,int * buflen,int * conn_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 960 | function (callable) | Service | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1238 | function (callable) | Get_Bad_Connection | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1260 | function (callable) | Global_Num_Send | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1288 | function (callable) | Global_Num_Receive | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1316 | function (callable) | Private_Num_Send | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1367 | function (callable) | Private_Num_Receive | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1421 | function (callable) | Set_Socket | IPXManagerClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1444 | function (callable) | Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1484 | function (callable) | Global_Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1509 | function (callable) | Reset_Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1538 | function (callable) | Oldest_Send | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1598 | function (callable) | Set_Bridge | IPXManagerClass | (NetNumType bridge) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1631 | function (callable) | Configure_Debug | IPXManagerClass | (int index,int offset,int size,char ** names,int maxnames) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1660 | function (callable) | Mono_Debug_Print | IPXManagerClass | (int index,int refresh) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1735 | function (callable) | Alloc_RealMode_Mem | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXMGR.CPP | 1926 | function (callable) | Free_RealMode_Mem | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXPROT.ASM | 38 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXPROT.ASM | 39 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXPROT.ASM | 40 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXPROT.ASM | 41 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXPROT.ASM | 58 | label (assembly label candidate) | include |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXREAL.ASM | 35 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXREAL.ASM | 36 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXREAL.ASM | 37 | label (assembly label candidate) | P386N |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/IPXREAL.ASM | 38 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 66 | function (callable) | Small_Icon |  | (void const * iconptr,int iconnum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 118 | function (callable) | Set_Window |  | (int window,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 144 | function (callable) | Fatal |  | (char const * message,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 164 | function (callable) | File_Fatal |  | (char const * message) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 199 | function (callable) | Load_Uncompress |  | (FileClass & file,BuffType & uncomp_buff,BuffType & dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 293 | function (callable) | Load_Alloc_Data |  | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 322 | function (callable) | Translucent_Table_Size |  | (int count) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 355 | function (callable) | Build_Translucent_Table |  | (void const * palette,TLucentType const * control,int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.CPP | 409 | function (callable) | Conquer_Build_Translucent_Table |  | (void const * palette,TLucentType const * control,int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 46 | function (callable) | Get | Keyboard | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 47 | function (callable) | Check | Keyboard | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 48 | function (callable) | To_ASCII | Keyboard | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 49 | function (callable) | Clear | Keyboard | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 50 | function (callable) | Stuff | Keyboard | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 51 | function (callable) | Down | Keyboard | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 52 | function (callable) | Mouse_X | Keyboard | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 53 | function (callable) | Mouse_Y | Keyboard | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 58 | function (callable) | operator delete |  | (void * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 63 | function (callable) | operator delete[]  |  | (void * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 75 | function (callable) | operator ++ |  | (T & a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 80 | function (callable) | operator ++ |  | (T & a,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 86 | function (callable) | operator -- |  | (T & a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 91 | function (callable) | operator -- |  | (T & a,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 97 | function (callable) | operator \| |  | (T t1,T t2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 101 | function (callable) | operator & |  | (T t1,T t2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/JSHELL.H | 105 | function (callable) | operator ~ |  | (T t1) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 149 | macro (function-like macro) | USE |  | func varname |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 229 | label (assembly label candidate) | BLIT_TRANSPARENT |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 230 | label (assembly label candidate) | BLIT_GHOST |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 231 | label (assembly label candidate) | BLIT_FADING |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 232 | label (assembly label candidate) | BLIT_PREDATOR |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 233 | label (assembly label candidate) | BLIT_SKIP |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 234 | label (assembly label candidate) | BLIT_ALL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 273 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 274 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 275 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 298 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 299 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 300 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 315 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 318 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 321 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 324 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 327 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 330 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 333 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 336 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 339 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 342 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 345 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 348 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 351 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 354 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 357 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 360 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 957 | label (assembly label candidate) | AllFlagsJumpTable |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1022 | label (assembly label candidate) | Setup_Shape_Header |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1044 | label (assembly label candidate) | outer_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1051 | label (assembly label candidate) | inner_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1069 | label (assembly label candidate) | not_transp |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1076 | label (assembly label candidate) | not_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1088 | label (assembly label candidate) | not_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1092 | label (assembly label candidate) | end_lp |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1113 | label (assembly label candidate) | not_all_trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1114 | label (assembly label candidate) | no_transparencies |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1191 | label (assembly label candidate) | Buffer_Frame_To_Page |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1253 | label (assembly label candidate) | is_ordinary_shape |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1267 | label (assembly label candidate) | do_args |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1271 | label (assembly label candidate) | check_centering |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1285 | label (assembly label candidate) | check_trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1294 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1307 | label (assembly label candidate) | check_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1324 | label (assembly label candidate) | new_shape |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1328 | label (assembly label candidate) | setup_headers |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1340 | label (assembly label candidate) | no_header_setup |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1347 | label (assembly label candidate) | not_shape_predator |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1352 | label (assembly label candidate) | not_shape_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1358 | label (assembly label candidate) | not_shape_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1364 | label (assembly label candidate) | not_shape_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1373 | label (assembly label candidate) | headers_set |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1393 | label (assembly label candidate) | set_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1404 | label (assembly label candidate) | single_fade |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1409 | label (assembly label candidate) | check_predator |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1426 | label (assembly label candidate) | check_range |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1429 | label (assembly label candidate) | pred_cont |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1435 | label (assembly label candidate) | pred_neg_init |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1439 | label (assembly label candidate) | pred_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1452 | label (assembly label candidate) | check_partial |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1461 | label (assembly label candidate) | setupfunc |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1520 | label (assembly label candidate) | dest_left_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1529 | label (assembly label candidate) | dest_bottom_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1536 | label (assembly label candidate) | dest_right_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1543 | label (assembly label candidate) | do_blit |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1590 | label (assembly label candidate) | use_old_stuff |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1602 | label (assembly label candidate) | real_out |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1608 | label (assembly label candidate) | no_mmx_cleanup |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1622 | label (assembly label candidate) | BF_Copy |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1628 | label (assembly label candidate) | forward_loop_dword |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1649 | label (assembly label candidate) | forward_loop_bytes |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1690 | label (assembly label candidate) | slt_mask_map_lp |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1705 | label (assembly label candidate) | slt_end_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1710 | label (assembly label candidate) | slt_skip |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1728 | label (assembly label candidate) | slt_skip2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1736 | label (assembly label candidate) | slt_round_again |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1751 | label (assembly label candidate) | slt_skip_end_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1766 | label (assembly label candidate) | Long_Single_Line_Copy |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1776 | label (assembly label candidate) | LSLC_aligned |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1790 | label (assembly label candidate) | proc_out |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1801 | label (assembly label candidate) | Short_Single_Line_Copy |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1819 | label (assembly label candidate) | Single_Line_Skip |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1834 | label (assembly label candidate) | Single_Line_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1838 | label (assembly label candidate) | slg_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1850 | label (assembly label candidate) | slg_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1867 | label (assembly label candidate) | Single_Line_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1873 | label (assembly label candidate) | slgt_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1878 | label (assembly label candidate) | slgt_not_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1889 | label (assembly label candidate) | slgt_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1900 | label (assembly label candidate) | slgt_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1905 | label (assembly label candidate) | slgt_round_again |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1918 | label (assembly label candidate) | slgt_out |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1928 | label (assembly label candidate) | slgt4 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1949 | label (assembly label candidate) | slgt4_store1 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1951 | label (assembly label candidate) | slgt4_trans1 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1981 | label (assembly label candidate) | Single_Line_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 1989 | label (assembly label candidate) | slf_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2016 | label (assembly label candidate) | Single_Line_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2024 | label (assembly label candidate) | slft_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2031 | label (assembly label candidate) | slft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2037 | label (assembly label candidate) | slft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2058 | label (assembly label candidate) | Single_Line_Single_Fade |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2063 | label (assembly label candidate) | slsf_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2083 | label (assembly label candidate) | Single_Line_Single_Fade_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2088 | label (assembly label candidate) | slsft_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2102 | label (assembly label candidate) | slsft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2117 | label (assembly label candidate) | slsft_next_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2132 | label (assembly label candidate) | Single_Line_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2137 | label (assembly label candidate) | SLGF_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2150 | label (assembly label candidate) | slgf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2154 | label (assembly label candidate) | slgf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2176 | label (assembly label candidate) | Single_Line_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2184 | label (assembly label candidate) | SLGFT_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2199 | label (assembly label candidate) | slgft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2203 | label (assembly label candidate) | slgft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2209 | label (assembly label candidate) | slgft_trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2220 | label (assembly label candidate) | slgft4 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2242 | label (assembly label candidate) | slgft4_fade |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2245 | label (assembly label candidate) | slgft4_fade_lp |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2251 | label (assembly label candidate) | slgft4_trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2262 | label (assembly label candidate) | slgft4_out |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2273 | label (assembly label candidate) | Single_Line_Predator |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2277 | label (assembly label candidate) | slp_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2287 | label (assembly label candidate) | slp_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2298 | label (assembly label candidate) | slp_skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2318 | label (assembly label candidate) | Single_Line_Predator_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2322 | label (assembly label candidate) | slpt_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2346 | label (assembly label candidate) | slpt_skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2363 | label (assembly label candidate) | Single_Line_Predator_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2367 | label (assembly label candidate) | slpg_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2376 | label (assembly label candidate) | slpg_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2386 | label (assembly label candidate) | slpg_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2400 | label (assembly label candidate) | slpg_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2420 | label (assembly label candidate) | Single_Line_Predator_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2423 | label (assembly label candidate) | slpgt_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2436 | label (assembly label candidate) | slpgt_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2446 | label (assembly label candidate) | slpgt_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2460 | label (assembly label candidate) | slpgt_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2462 | label (assembly label candidate) | slpgt_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2480 | label (assembly label candidate) | Single_Line_Predator_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2485 | label (assembly label candidate) | slpf_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2505 | label (assembly label candidate) | slpf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2510 | label (assembly label candidate) | slpf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2533 | label (assembly label candidate) | Single_Line_Predator_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2537 | label (assembly label candidate) | slpft_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2549 | label (assembly label candidate) | slpft_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2560 | label (assembly label candidate) | slpft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2565 | label (assembly label candidate) | slpft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2571 | label (assembly label candidate) | slpft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2589 | label (assembly label candidate) | Single_Line_Predator_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2594 | label (assembly label candidate) | slpgf_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2604 | label (assembly label candidate) | slpgf_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2615 | label (assembly label candidate) | slpgf_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2629 | label (assembly label candidate) | slpgf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2634 | label (assembly label candidate) | slpgf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2639 | label (assembly label candidate) | slpgf_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2658 | label (assembly label candidate) | Single_Line_Predator_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2663 | label (assembly label candidate) | slpgft_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2676 | label (assembly label candidate) | slpgft_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2687 | label (assembly label candidate) | slpgft_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2701 | label (assembly label candidate) | slpgft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2706 | label (assembly label candidate) | slpgft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2711 | label (assembly label candidate) | slpgft_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2713 | label (assembly label candidate) | slpgft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2733 | label (assembly label candidate) | BF_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2746 | label (assembly label candidate) | trans_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2753 | label (assembly label candidate) | trans_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2764 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2768 | label (assembly label candidate) | trans_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2784 | label (assembly label candidate) | BF_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2807 | label (assembly label candidate) | ghost_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2811 | label (assembly label candidate) | ghost_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2830 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2836 | label (assembly label candidate) | ghost_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2853 | label (assembly label candidate) | BF_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2876 | label (assembly label candidate) | ghost_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2880 | label (assembly label candidate) | ghost_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2903 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2906 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2911 | label (assembly label candidate) | ghost_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2928 | label (assembly label candidate) | BF_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2951 | label (assembly label candidate) | fading_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2957 | label (assembly label candidate) | fading_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2960 | label (assembly label candidate) | fading_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2968 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2978 | label (assembly label candidate) | fading_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 2996 | label (assembly label candidate) | BF_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3019 | label (assembly label candidate) | fading_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3025 | label (assembly label candidate) | fading_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3028 | label (assembly label candidate) | fading_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3040 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3047 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3052 | label (assembly label candidate) | fading_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3070 | label (assembly label candidate) | BF_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3093 | label (assembly label candidate) | ghost_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3098 | label (assembly label candidate) | ghost_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3101 | label (assembly label candidate) | ghost_f_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3123 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3127 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3132 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3138 | label (assembly label candidate) | ghost_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3156 | label (assembly label candidate) | BF_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3179 | label (assembly label candidate) | ghost_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3184 | label (assembly label candidate) | ghost_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3187 | label (assembly label candidate) | ghost_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3213 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3217 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3222 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3225 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3230 | label (assembly label candidate) | ghost_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3248 | label (assembly label candidate) | BF_Predator |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3271 | label (assembly label candidate) | predator_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3275 | label (assembly label candidate) | predator_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3290 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3309 | label (assembly label candidate) | skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3314 | label (assembly label candidate) | predator_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3331 | label (assembly label candidate) | BF_Predator_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3354 | label (assembly label candidate) | predator_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3358 | label (assembly label candidate) | predator_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3377 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3388 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3391 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3396 | label (assembly label candidate) | predator_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3413 | label (assembly label candidate) | BF_Predator_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3436 | label (assembly label candidate) | predator_g_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3440 | label (assembly label candidate) | predator_g_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3456 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3467 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3481 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3487 | label (assembly label candidate) | predator_g_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3504 | label (assembly label candidate) | BF_Predator_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3527 | label (assembly label candidate) | predator_g_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3531 | label (assembly label candidate) | predator_g_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3551 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3562 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3576 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3579 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3584 | label (assembly label candidate) | predator_g_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3601 | label (assembly label candidate) | BF_Predator_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3624 | label (assembly label candidate) | predator_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3629 | label (assembly label candidate) | predator_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3632 | label (assembly label candidate) | predator_f_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3648 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3659 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3663 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3673 | label (assembly label candidate) | predator_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3691 | label (assembly label candidate) | BF_Predator_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3714 | label (assembly label candidate) | predator_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3719 | label (assembly label candidate) | predator_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3722 | label (assembly label candidate) | predator_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3742 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3753 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3757 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3764 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3769 | label (assembly label candidate) | predator_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3787 | label (assembly label candidate) | BF_Predator_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3810 | label (assembly label candidate) | predator_g_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3815 | label (assembly label candidate) | predator_g_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3818 | label (assembly label candidate) | predator_g_f_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3836 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3847 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3862 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3866 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3871 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3877 | label (assembly label candidate) | predator_g_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3895 | label (assembly label candidate) | BF_Predator_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3918 | label (assembly label candidate) | predator_g_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3923 | label (assembly label candidate) | predator_g_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3926 | label (assembly label candidate) | predator_g_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3948 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3959 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3974 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3978 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3983 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3986 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 3991 | label (assembly label candidate) | predator_g_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFBUFF.ASM | 4008 | label (assembly label candidate) | Not_Supported |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 44 | macro (function-like macro) | Apply_Delta |  | (buffer,delta) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 94 | function (callable) | Get_Shape_Header_Data |  | (void * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 106 | function (callable) | Get_Last_Frame_Length |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 113 | function (callable) | Reset_Theater_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 129 | function (callable) | Reallocate_Big_Shape_Buffer |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 153 | function (callable) | Check_Use_Compressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 183 | function (callable) | Disable_Uncompressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 204 | function (callable) | Enable_Uncompressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 212 | function (callable) | Build_Frame |  | (void const * dataptr,unsigned short framenumber,void * buffptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 512 | function (callable) | Get_Build_Frame_Count |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 521 | function (callable) | Get_Build_Frame_X |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 530 | function (callable) | Get_Build_Frame_Y |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 554 | function (callable) | Get_Build_Frame_Width |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 578 | function (callable) | Get_Build_Frame_Height |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/KEYFRAME.CPP | 587 | function (callable) | Get_Build_Frame_Palette |  | (void const * dataptr,void * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LAYER.CPP | 59 | function (callable) | Submit | LayerClass | (ObjectClass const * object,bool sort) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LAYER.CPP | 89 | function (callable) | Sort | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LAYER.CPP | 118 | function (callable) | Sorted_Add | LayerClass | (ObjectClass const * const object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LAYER.H | 52 | function (callable) | Init | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LAYER.H | 53 | function (callable) | One_Time | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LED.H | 35 | function (callable) | Scroll_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LED.H | 36 | function (callable) | Caps_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LED.H | 37 | function (callable) | Num_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 67 | function (callable) | LinkClass | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 89 | function (callable) | LinkClass | LinkClass | (LinkClass & link) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 110 | function (callable) | ~LinkClass | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 133 | function (callable) | Zap | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 162 | function (callable) | operator = | LinkClass | (LinkClass & link) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 185 | function (callable) | Get_Next | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 206 | function (callable) | Get_Prev | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 227 | function (callable) | Head_Of_List | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 253 | function (callable) | Tail_Of_List | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 280 | function (callable) | Add | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 320 | function (callable) | Add_Head | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 353 | function (callable) | Add_Tail | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.CPP | 390 | function (callable) | Remove | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.H | 55 | function (callable) | Head_Of_List | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LINK.H | 60 | function (callable) | Tail_Of_List | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 79 | function (callable) | ListClass | ListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 122 | function (callable) | ~ListClass | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 139 | function (callable) | Add_Item | ListClass | (char const * text) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 177 | function (callable) | Add_Item | ListClass | (int text) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 198 | function (callable) | Remove_Item | ListClass | (char const * text) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 253 | function (callable) | Action | ListClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 309 | function (callable) | Draw_Me | ListClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 366 | function (callable) | Bump | ListClass | (int up) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 391 | function (callable) | Step | ListClass | (int up) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 417 | function (callable) | Get_Item | ListClass | (int index) const | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 440 | function (callable) | Current_Item | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 461 | function (callable) | Current_Index | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 487 | function (callable) | Peer_To_Peer | ListClass | (unsigned flags,KeyNumType &,ControlClass & whom) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 524 | function (callable) | Set_View_Index | ListClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 554 | function (callable) | Add_Scroll_Bar | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 622 | function (callable) | Remove_Scroll_Bar | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 654 | function (callable) | Set_Tabs | ListClass | (int const * tabs) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 681 | function (callable) | Draw_Entry | ListClass | (int index,int x,int y,int width,int selected) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 721 | function (callable) | Add | ListClass | (LinkClass & list) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 752 | function (callable) | Add_Head | ListClass | (LinkClass & list) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 789 | function (callable) | Add_Tail | ListClass | (LinkClass & list) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 826 | function (callable) | Remove | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 858 | function (callable) | Set_Selected_Index | ListClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.CPP | 888 | function (callable) | Step_Selected_Index | ListClass | (int step) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LIST.H | 67 | function (callable) | Count | ListClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LIST.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 61 | function (callable) | LoadOptionsClass | LoadOptionsClass | (LoadStyleType style) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 83 | function (callable) | ~LoadOptionsClass | LoadOptionsClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 107 | function (callable) | Process | LoadOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 549 | function (callable) | Clear_List | LoadOptionsClass | (ListClass * list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 586 | function (callable) | Fill_List | LoadOptionsClass | (ListClass * list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 700 | function (callable) | Num_From_Ext | LoadOptionsClass | (char * fname) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOADDLG.CPP | 725 | function (callable) | Compare | LoadOptionsClass | (const void * p1,const void * p2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOGIC.CPP | 63 | function (callable) | Debug_Dump | LogicClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/LOGIC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOGIC.CPP | 165 | function (callable) | AI | LogicClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LOGIC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/LOGIC.CPP | 335 | function (callable) | Clear_Recently_Created_Bits | LogicClass | (void) | RUST/original-source-reference/TIBERIANDAWN/LOGIC.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 94 | function (callable) | One_Time | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 115 | function (callable) | One_Time_Editor | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 149 | function (callable) | Init_Clear | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 178 | function (callable) | Alloc_Cells | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 210 | function (callable) | Free_Cells | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 235 | function (callable) | Init_Cells | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 268 | function (callable) | Set_Map_Dimensions | MapClass | (int x,int y,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 306 | function (callable) | Sight_From | MapClass | (HouseClass * house,CELL cell,int sightrange,bool incremental) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 392 | function (callable) | Cell_Distance | MapClass | (CELL cell1,CELL cell2) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 428 | function (callable) | In_Radar | MapClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 454 | function (callable) | Place_Down | MapClass | (CELL cell,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 497 | function (callable) | Pick_Up | MapClass | (CELL cell,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 543 | function (callable) | Overlap_Down | MapClass | (CELL cell,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 579 | function (callable) | Overlap_Up | MapClass | (CELL cell,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 611 | function (callable) | Overpass | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 646 | function (callable) | Read_Binary | MapClass | (char const * root,unsigned long *) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 743 | function (callable) | Read_Binary_File | MapClass | (char const * fname,unsigned long * crc) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 831 | function (callable) | Write_Binary | MapClass | (char const * root) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 897 | function (callable) | Logic | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1046 | function (callable) | Cell_Region | MapClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1065 | function (callable) | Cell_Threat | MapClass | (CELL cell,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1091 | function (callable) | Place_Random_Crate | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1132 | function (callable) | Validate | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1272 | function (callable) | Clean | MapClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1393 | function (callable) | Close_Object | MapClass | (COORDINATE coord) const | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1480 | macro (function-like macro) | ARRAY_SIZE |  | (a) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.CPP | 1507 | function (callable) | Nearby_Location | MapClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAP.H | 64 | function (callable) | Detach | MapClass | (ObjectClass *) | RUST/original-source-reference/TIBERIANDAWN/MAP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 74 | function (callable) | New_Scenario | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 181 | function (callable) | Load_Scenario | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 270 | function (callable) | Save_Scenario | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 383 | function (callable) | Pick_Scenario | MapEditClass | (char const * caption,int * scen_nump,ScenarioPlayerType * playerp,ScenarioDirType * dirp,ScenarioVarType * varp,int multi) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 883 | function (callable) | Size_Map | MapEditClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 1533 | function (callable) | Scenario_Dialog | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 2130 | function (callable) | Handle_Triggers | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 2252 | function (callable) | Select_Trigger | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 2588 | function (callable) | Edit_Trigger | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 3204 | function (callable) | Import_Triggers | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDDLG.CPP | 3543 | function (callable) | Import_Teams | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 105 | function (callable) | MapEditClass | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 144 | function (callable) | One_Time | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 252 | function (callable) | Init_IO | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 292 | function (callable) | Read_INI | MapEditClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 319 | function (callable) | Write_INI | MapEditClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 350 | function (callable) | Clear_List | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 384 | function (callable) | Add_To_List | MapEditClass | (ObjectTypeClass const * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 502 | function (callable) | AI | MapEditClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1357 | function (callable) | Draw_It | MapEditClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1437 | function (callable) | Mouse_Moved | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1509 | function (callable) | Main_Menu | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1699 | function (callable) | AI_Menu | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1807 | function (callable) | Verify_House | MapEditClass | (HousesType house,ObjectTypeClass const * objtype) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1832 | function (callable) | Cycle_House | MapEditClass | (HousesType curhouse,ObjectTypeClass const * objtype) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1889 | function (callable) | Fatal | MapEditClass | (int txt) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1899 | function (callable) | Scroll_Map | MapEditClass | (DirType facing,int & distance,bool really) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDIT.CPP | 1913 | function (callable) | Detach | MapEditClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 98 | function (callable) | Placement_Dialog | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 845 | function (callable) | Start_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 949 | function (callable) | Place_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1178 | function (callable) | Cancel_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1222 | function (callable) | Place_Next | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1312 | function (callable) | Place_Prev | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1398 | function (callable) | Place_Next_Category | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1488 | function (callable) | Place_Prev_Category | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1590 | function (callable) | Place_Home | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1663 | function (callable) | Toggle_House | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1721 | function (callable) | Set_House_Buttons | MapEditClass | (HousesType house,GadgetClass * btnlist,int base_id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1765 | function (callable) | Start_Trigger_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1787 | function (callable) | Stop_Trigger_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1810 | function (callable) | Place_Trigger | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1877 | function (callable) | Start_Base_Building | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1913 | function (callable) | Cancel_Base_Building | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDPLC.CPP | 1949 | function (callable) | Build_Base_To | MapEditClass | (int percent) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 63 | function (callable) | Select_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 158 | function (callable) | Select_Next | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 266 | function (callable) | Popup_Controls | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 426 | function (callable) | Grab_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 457 | function (callable) | Move_Grabbed_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDSEL.CPP | 555 | function (callable) | Change_House | MapEditClass | (HousesType newhouse) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 62 | function (callable) | Handle_Teams | MapEditClass | (char const * caption) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 176 | function (callable) | Select_Team | MapEditClass | (char const * caption) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 513 | function (callable) | Edit_Team | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 1474 | function (callable) | Team_Members | MapEditClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 1964 | function (callable) | Draw_Member | MapEditClass | (TechnoTypeClass const * ptr,int index,int quant,HousesType house,int pic_x,int pic_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPEDTM.CPP | 2038 | function (callable) | Build_Mission_List | MapEditClass | (int missioncount,TeamMissionStruct * missions,char missionbuf[TeamTypeClass::MAX_TEAM_MISSIONS][20],ListClass * list) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 210 | function (callable) | Map_Selection |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 817 | function (callable) | Print_Statistics |  | (int country,int xpos,int ypos) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 1069 | function (callable) | Fading_Byte_Blit |  | (int srcx,int srcy,int destx,int desty,int w,int h,GraphicBufferClass * src,GraphicBufferClass * dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 1137 | function (callable) | Cycle_Call_Back_Delay |  | (int time,unsigned char * pal) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 1166 | function (callable) | LowMedHiStr |  | (int percentage) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 1205 | function (callable) | Bit_It_In_Scale |  | (int x,int y,int w,int h,GraphicBufferClass * src,GraphicBufferClass * dest,GraphicViewPortClass *,int delay,int dagger) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MAPSEL.CPP | 1270 | function (callable) | Bit_It_In |  | (int x,int y,int w,int h,GraphicBufferClass * src,GraphicBufferClass * dest,int delay,int dagger) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 66 | function (callable) | Select_To_Entry |  | (int select,unsigned long bitfield,int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 98 | function (callable) | Flash_Line |  | (char const * text,int xpix,int ypix,unsigned nfgc,unsigned hfgc,unsigned bgc) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 122 | function (callable) | Coordinates_In_Region |  | (int x,int y,int inx1,int iny1,int inx2,int iny2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 141 | function (callable) | Find_Menu_Items |  | (int maxitems,unsigned long field,char index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 168 | function (callable) | Setup_Menu |  | (int menu,char const * text[],unsigned long field,int index,int skip) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 202 | function (callable) | Check_Menu |  | (int menu,char const * text[],char *,long field,int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 365 | function (callable) | Do_Menu |  | (char const ** strings,bool blue) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MENUS.CPP | 451 | function (callable) | Main_Menu |  | (unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 64 | function (callable) | MissionClass | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 74 | function (callable) | Mission_Sleep | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 75 | function (callable) | Mission_Ambush | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 76 | function (callable) | Mission_Attack | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 77 | function (callable) | Mission_Capture | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 78 | function (callable) | Mission_Guard | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 79 | function (callable) | Mission_Guard_Area | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 80 | function (callable) | Mission_Harvest | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 81 | function (callable) | Mission_Hunt | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 82 | function (callable) | Mission_Timed_Hunt | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 83 | function (callable) | Mission_Move | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 84 | function (callable) | Mission_Retreat | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 85 | function (callable) | Mission_Return | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 86 | function (callable) | Mission_Stop | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 87 | function (callable) | Mission_Unload | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 88 | function (callable) | Mission_Enter | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 89 | function (callable) | Mission_Construction | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 90 | function (callable) | Mission_Deconstruction | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 91 | function (callable) | Mission_Repair | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 92 | function (callable) | Mission_Missile | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 111 | function (callable) | Set_Mission | MissionClass | (MissionType mission) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 134 | function (callable) | Get_Mission | MissionClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 156 | function (callable) | Debug_Dump | MissionClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 185 | function (callable) | AI | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 297 | function (callable) | Commence | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 331 | function (callable) | Assign_Mission | MissionClass | (MissionType order) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 358 | function (callable) | Mission_From_Name | MissionClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 388 | function (callable) | Mission_Name | MissionClass | (MissionType mission) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 411 | function (callable) | Override_Mission | MissionClass | (MissionType mission,TARGET,TARGET) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.CPP | 435 | function (callable) | Restore_Mission | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MISSION.H | 69 | function (callable) | ~MissionClass | MissionClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MISSION.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 57 | function (callable) | Compare |  | (T const * obj1,T const * obj2) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 85 | function (callable) | Free | MixFileClass | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 98 | function (callable) | Free_All | MixFileClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 124 | function (callable) | ~MixFileClass | MixFileClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 172 | function (callable) | MixFileClass | MixFileClass | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 250 | function (callable) | Retrieve | MixFileClass | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 277 | function (callable) | Finder | MixFileClass | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 306 | function (callable) | Cache | MixFileClass | (char const * filename) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 333 | function (callable) | Cache | MixFileClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 388 | function (callable) | Free | MixFileClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 429 | function (callable) | compfunc |  | (void const * ptr1,void const * ptr2) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.CPP | 470 | function (callable) | Offset | MixFileClass | (char const * filename,void ** realptr,MixFileClass ** mixfile,long * offset,long * size) | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.H | 62 | function (callable) | operator < | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.H | 63 | function (callable) | operator > | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MIXFILE.H | 64 | function (callable) | operator == | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/TIBERIANDAWN/MIXFILE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MMX.ASM | 57 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MMX.ASM | 77 | label (assembly label candidate) | Detect_MMX_Availability |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MMX.ASM | 200 | label (assembly label candidate) | Init_MMX |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MMX.ASM | 240 | label (assembly label candidate) | MMX_Done |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MMX.ASM | 275 | label (assembly label candidate) | MMX_Single_Line_Trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 100 | function (callable) | MonoClass | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 134 | function (callable) | ~MonoClass | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 163 | function (callable) | Draw_Box | MonoClass | (int x,int y,int w,int h,char attrib,BoxStyleType thick) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 235 | function (callable) | Set_Cursor | MonoClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 291 | function (callable) | Clear | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 329 | function (callable) | Scroll | MonoClass | (int lines) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 371 | function (callable) | Printf | MonoClass | (char const * text,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 394 | function (callable) | Printf | MonoClass | (int text,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 432 | function (callable) | Print | MonoClass | (char const * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 528 | function (callable) | Text_Print | MonoClass | (char const * text,int x,int y,char attrib) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 543 | function (callable) | Text_Print | MonoClass | (int text,int x,int y,char attrib) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 559 | function (callable) | Print | MonoClass | (int text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 581 | function (callable) | operator = | MonoClass | (MonoClass const & src) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 609 | function (callable) | View | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 656 | function (callable) | Mono_Set_Cursor |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 668 | function (callable) | Mono_Printf |  | (char const * string,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 692 | function (callable) | Mono_Clear_Screen |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 704 | function (callable) | Mono_Text_Print |  | (void const * text,int x,int y,int attrib) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 716 | function (callable) | Mono_Draw_Rect |  | (int x,int y,int w,int h,int attrib,int thick) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 728 | function (callable) | Mono_Print |  | (void const * text) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 740 | function (callable) | Mono_X |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 753 | function (callable) | Mono_Y |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 767 | function (callable) | Mono_Put_Char |  | (char,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 771 | function (callable) | Mono_Scroll |  | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 775 | function (callable) | Mono_View_Page |  | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.CPP | 781 | function (callable) | Mono_Printf |  | (int string,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 103 | function (callable) | Enable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 104 | function (callable) | Disable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 105 | function (callable) | Is_Enabled | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 106 | function (callable) | Get_Current | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 109 | function (callable) | Set_Default_Attribute | MonoClass | (char attrib) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 119 | function (callable) | Get_X | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 120 | function (callable) | Get_Y | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 143 | function (callable) | Offset | MonoClass | (int x=0,int y=0) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MONOC.H | 145 | function (callable) | Store_Cell | MonoClass | (CellType & cell,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 76 | function (callable) | Set_Default_Mouse | MouseClass | (MouseType mouse,bool size) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 98 | function (callable) | Revert_Mouse_Shape | MouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 104 | function (callable) | Mouse_Small | MouseClass | (bool wwsmall) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 146 | function (callable) | Override_Mouse_Shape | MouseClass | (MouseType mouse,bool wwsmall) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 207 | function (callable) | AI | MouseClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 251 | function (callable) | MouseClass | MouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 274 | function (callable) | One_Time | MouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.CPP | 308 | function (callable) | Init_Clear | MouseClass | (void) | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MOUSE.H | 55 | function (callable) | Get_Mouse_Shape | MouseClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/MOUSE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 68 | function (callable) | Select_MPlayer_Game |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 491 | function (callable) | Read_MultiPlayer_Settings |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 853 | function (callable) | Write_MultiPlayer_Settings |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 975 | function (callable) | Read_Scenario_Descriptions |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 1050 | function (callable) | Free_Scenario_Descriptions |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 1096 | function (callable) | Computer_Message |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 1165 | function (callable) | Garble_Message |  | (char * buf) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MPLAYER.CPP | 1250 | function (callable) | Surrender_Dialog |  | (void) | RUST/original-source-reference/TIBERIANDAWN/MPLAYER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGBOX.CPP | 42 | function (callable) | CCMessageBox | CCMessageBox | (int caption,bool pict) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGBOX.CPP | 76 | function (callable) | Process | CCMessageBox | (const char * msg,const char * b1txt,const char * b2txt,const char * b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGBOX.CPP | 450 | function (callable) | Process | CCMessageBox | (int msg,int b1txt,int b2txt,int b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGBOX.CPP | 476 | function (callable) | Process | CCMessageBox | (char const * msg,int b1txt,int b2txt,int b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 70 | function (callable) | MessageListClass | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 108 | function (callable) | ~MessageListClass | MessageListClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 131 | function (callable) | Init | MessageListClass | (int x,int y,int max_msg,int maxchars,int height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 195 | function (callable) | Add_Message | MessageListClass | (char * txt,int color,TextPrintType style,int timeout,unsigned short magic_number,unsigned short crc) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 460 | function (callable) | Add_Edit | MessageListClass | (int color,TextPrintType style,char * to,int width) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 503 | function (callable) | Get_Edit_Buf | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 529 | function (callable) | Manage | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 611 | function (callable) | Input | MessageListClass | (KeyNumType & input) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 738 | function (callable) | Draw | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 763 | function (callable) | Num_Messages | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MSGLIST.CPP | 797 | function (callable) | Set_Width | MessageListClass | (int width) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 31 | function (callable) | Mem_Copy |  | (void const * source,void * dest,unsigned long bytes_to_copy) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 54 | function (callable) | Distance_Coord |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 104 | function (callable) | Desired_Facing16 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 204 | function (callable) | Desired_Facing256 |  | (LONG srcx,LONG srcy,LONG dstx,LONG dsty) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 371 | function (callable) | Desired_Facing8 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 496 | function (callable) | Desired_Facing16 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 784 | function (callable) | Set_Bit |  | (void * array,int bit,int value) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 802 | function (callable) | Get_Bit |  | (void const * array,int bit) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 815 | function (callable) | First_True_Bit |  | (void const * array) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 831 | function (callable) | First_False_Bit |  | (void const * array) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 848 | function (callable) | Bound |  | (int original,int min,int max) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 875 | function (callable) | Coord_Cell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 908 | function (callable) | Shake_Screen |  | (int shakes) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1078 | function (callable) | Conquer_Build_Fading_Table |  | (void const * palette,void * dest,int color,int frac) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1268 | function (callable) | Reverse_Long |  | (long number) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1279 | function (callable) | Reverse_Short |  | (short number) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1289 | function (callable) | Swap_Long |  | (long number) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1331 | function (callable) | strtrim |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1410 | function (callable) | Fat_Put_Pixel |  | (int x,int y,int color,int siz,GraphicViewPortClass & gpage) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1496 | function (callable) | Calculate_CRC |  | (void * buffer,long length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/MiscAsm.cpp | 1570 | function (callable) | Set_Palette_Range |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 174 | function (callable) | Init_Network |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 223 | function (callable) | Shutdown_Network |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 303 | function (callable) | Process_Global_Packet |  | (GlobalPacketType * packet,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 380 | function (callable) | Destroy_Connection |  | (int id,int error) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 462 | function (callable) | Remote_Connect |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 568 | function (callable) | Server_Remote_Connect |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 625 | function (callable) | Client_Remote_Connect |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 745 | function (callable) | Net_Join_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 1955 | function (callable) | Clear_Game_List |  | (ListClass * gamelist) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 1998 | function (callable) | Clear_Player_List |  | (ListClass * playerlist) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 2045 | function (callable) | Request_To_Join |  | (char * playername,int join_index,ListClass * playerlist,HousesType house,int color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 2157 | function (callable) | Send_Join_Queries |  | (int curgame,int gamenow,int playernow) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 2249 | function (callable) | Get_Join_Responses |  | (JoinStateType * joinstate,ListClass * gamelist,ColorListClass * playerlist,int join_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 2663 | function (callable) | Net_New_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 3786 | function (callable) | Get_NewGame_Responses |  | (ColorListClass * playerlist) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 3979 | function (callable) | Compute_Name_CRC |  | (char * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 4014 | function (callable) | Net_Reconnect_Dialog |  | (int reconn,int fresh,int oldest_index,unsigned long timeval) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 4109 | function (callable) | Wait_For_Focus |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 4158 | function (callable) | Net_Fake_New_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NETDLG.CPP | 4750 | function (callable) | Net_Fake_Join_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 69 | function (callable) | NonSequencedConnClass | NonSequencedConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 98 | function (callable) | ~NonSequencedConnClass | NonSequencedConnClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 119 | function (callable) | Init | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 155 | function (callable) | Send_Packet | NonSequencedConnClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 214 | function (callable) | Receive_Packet | NonSequencedConnClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 431 | function (callable) | Get_Packet | NonSequencedConnClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 506 | function (callable) | Service_Send_Queue | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NOSEQCON.CPP | 654 | function (callable) | Service_Receive_Queue | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 66 | function (callable) | NullModemConnClass | NullModemConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 102 | function (callable) | ~NullModemConnClass | NullModemConnClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 124 | function (callable) | Init | NullModemConnClass | (HANDLE port_handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 147 | function (callable) | Send | NullModemConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 218 | function (callable) | Compute_CRC | NullModemConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.CPP | 253 | function (callable) | Packet_Overhead_Size | NullModemConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLCONN.H | 94 | function (callable) | Actual_Max_Packet | NullModemConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 115 | function (callable) | Init_Null_Modem |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 146 | function (callable) | Shutdown_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 176 | function (callable) | Modem_Signoff |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 212 | function (callable) | Test_Null_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 507 | function (callable) | Reconnect_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 559 | function (callable) | Reconnect_Null_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 756 | function (callable) | Destroy_Null_Connection |  | (int id,int error) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 857 | function (callable) | Select_Serial_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 1362 | function (callable) | Advanced_Modem_Settings |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 1645 | function (callable) | Com_Settings_Dialog |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 2971 | function (callable) | Build_Init_String_Listbox |  | (ListClass * list,EditClass * edit,char * buf,int * index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 3042 | function (callable) | Init_String_Compare |  | (const void * p1,const void * p2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 3092 | function (callable) | Com_Scenario_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 4555 | function (callable) | Com_Show_Scenario_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 5699 | function (callable) | Phone_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6187 | function (callable) | Build_Phone_Listbox |  | (ListClass * list,EditClass * edit,char * buf) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6280 | function (callable) | Phone_Compare |  | (const void * p1,const void * p2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6315 | function (callable) | Edit_Phone_Dialog |  | (PhoneEntryClass * phone) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6644 | function (callable) | Dial_Modem |  | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6820 | function (callable) | Answer_Modem |  | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6985 | function (callable) | Modem_Echo |  | (char c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 6998 | function (callable) | Smart_Printf |  | (char * format,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 7024 | function (callable) | Hex_Dump_Data |  | (char * buffer,int length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 7108 | function (callable) | itoh |  | (int i,char * s) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 7164 | function (callable) | Com_Fake_Scenario_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 7677 | function (callable) | Com_Show_Fake_Scenario_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 8225 | function (callable) | Smart_Printf |  | (char * format,...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 8251 | function (callable) | Hex_Dump_Data |  | (char * buffer,int length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLDLG.CPP | 8335 | function (callable) | itoh |  | (int i,char * s) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 111 | function (callable) | NullModemClass | NullModemClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 173 | function (callable) | ~NullModemClass | NullModemClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 202 | function (callable) | Init | NullModemClass | (int port,int,char * dev_name,int baud,char parity,int wordlen,int stopbits,int flowcontrol) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 381 | function (callable) | Num_Connections | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 402 | function (callable) | Delete_Connection | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 444 | function (callable) | Init_Send_Queue | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 471 | function (callable) | Detect_Port | NullModemClass | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 593 | function (callable) | Shutdown | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 632 | function (callable) | Set_Timing | NullModemClass | (unsigned long retrydelta,unsigned long maxretries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 678 | function (callable) | Send_Message | NullModemClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 711 | function (callable) | Get_Message | NullModemClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 735 | function (callable) | Service | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 956 | function (callable) | Num_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 978 | function (callable) | Num_Receive | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1000 | function (callable) | Response_Time | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1022 | function (callable) | Reset_Response_Time | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1042 | function (callable) | Oldest_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1090 | function (callable) | Configure_Debug | NullModemClass | (int,int offset,int size,char ** names,int maxnames) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1113 | function (callable) | Mono_Debug_Print | NullModemClass | (int,int refresh) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1158 | function (callable) | Timer_Test |  | (int line,char * file) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1197 | function (callable) | Detect_Modem | NullModemClass | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1462 | function (callable) | Dial_Modem | NullModemClass | (char * string,DialMethodType method,bool reconnect) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1725 | function (callable) | Answer_Modem | NullModemClass | (bool reconnect) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 1973 | function (callable) | Hangup_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2069 | function (callable) | Setup_Modem_Echo | NullModemClass | (void (* func)(char c)) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2091 | function (callable) | Remove_Modem_Echo | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2113 | function (callable) | Print_EchoBuf | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2141 | function (callable) | Reset_EchoBuf | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2162 | function (callable) | Abort_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2207 | function (callable) | Setup_Abort_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2227 | function (callable) | Remove_Abort_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2249 | function (callable) | Change_IRQ_Priority | NullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2269 | function (callable) | Get_Modem_Status | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2308 | function (callable) | Send_Modem_Command | NullModemClass | (char * command,char terminator,char * buffer,int buflen,int delay,int retries) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.CPP | 2328 | function (callable) | Verify_And_Convert_To_Int | NullModemClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 98 | function (callable) | Connection_ID | NullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 99 | function (callable) | Connection_Index | NullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 115 | function (callable) | Send_Private_Message | NullModemClass | (void * buf,int buflen,int ack_req=1,int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 118 | function (callable) | Get_Private_Message | NullModemClass | (void * buf,int * buflen,int *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 143 | function (callable) | Global_Num_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 144 | function (callable) | Global_Num_Receive | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 145 | function (callable) | Private_Num_Send | NullModemClass | (int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/NULLMGR.H | 147 | function (callable) | Private_Num_Receive | NullModemClass | (int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 90 | function (callable) | Is_Infantry | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 113 | function (callable) | ObjectTypeClass | ObjectTypeClass | (bool is_sentient,bool is_flammable,bool is_crushable,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,int name,char const * ini,ArmorType armor,unsigned short strength) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 158 | function (callable) | Max_Pips | ObjectTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 182 | function (callable) | Dimensions | ObjectTypeClass | (int & width,int & height) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 205 | function (callable) | Cost_Of | ObjectTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 226 | function (callable) | Time_To_Build | ObjectTypeClass | (HousesType) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 246 | function (callable) | Who_Can_Build_Me | ObjectTypeClass | (bool,bool,HousesType) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 268 | function (callable) | Get_Cameo_Data | ObjectTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 292 | function (callable) | ObjectClass | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 324 | function (callable) | What_Am_I | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 348 | function (callable) | What_Action | ObjectClass | (ObjectClass *) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 373 | function (callable) | What_Action | ObjectClass | (CELL) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 394 | function (callable) | In_Which_Layer | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 417 | function (callable) | Is_Techno | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 438 | function (callable) | Get_Ownable | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 459 | function (callable) | Can_Repair | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 480 | function (callable) | Can_Demolish | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 486 | function (callable) | Can_Demolish_Unit | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 492 | function (callable) | Can_Capture | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 514 | function (callable) | Can_Player_Fire | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 536 | function (callable) | Can_Player_Move | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 558 | function (callable) | Target_Coord | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 563 | function (callable) | Center_Coord | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 564 | function (callable) | Render_Coord | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 565 | function (callable) | Docking_Coord | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 566 | function (callable) | Sort_Y | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 567 | function (callable) | Fire_Data | ObjectClass | (int which) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 568 | function (callable) | Fire_Coord | ObjectClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 569 | function (callable) | Record_The_Kill | ObjectClass | (TechnoClass *) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 570 | function (callable) | Do_Shimmer | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 571 | function (callable) | Exit_Object | ObjectClass | (TechnoClass *) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 572 | function (callable) | Hidden | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 573 | function (callable) | Look | ObjectClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 574 | function (callable) | Active_Click_With | ObjectClass | (ActionType,ObjectClass *) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 575 | function (callable) | Active_Click_With | ObjectClass | (ActionType,CELL) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 576 | function (callable) | Clicked_As_Target | ObjectClass | (HousesType house,int) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 577 | function (callable) | In_Range | ObjectClass | (COORDINATE,int) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 578 | function (callable) | Weapon_Range | ObjectClass | (int) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 579 | function (callable) | As_Target | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 580 | function (callable) | Scatter | ObjectClass | (COORDINATE,bool,bool) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 581 | function (callable) | Catch_Fire | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 599 | function (callable) | Fire_Out | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 623 | function (callable) | Value | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 645 | function (callable) | Get_Mission | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 670 | function (callable) | Repair | ObjectClass | (int) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 694 | function (callable) | Sell_Back | ObjectClass | (int) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 715 | function (callable) | Move | ObjectClass | (FacingType facing) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 745 | function (callable) | Unselect | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 781 | function (callable) | Unselect_All_Players | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 813 | function (callable) | Unselect_All_Players_Except_Owner | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 857 | function (callable) | Select | ObjectClass | (bool allow_mixed) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 915 | function (callable) | Render | ObjectClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 996 | function (callable) | Debug_Dump | ObjectClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1029 | function (callable) | Occupy_List | ObjectTypeClass | (bool) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1053 | function (callable) | Overlap_List | ObjectTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1075 | function (callable) | One_Time | ObjectTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1111 | function (callable) | Mark_For_Redraw | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1142 | function (callable) | Limbo | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1193 | function (callable) | Unlimbo | ObjectClass | (COORDINATE coord,DirType) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1242 | function (callable) | Detach_All | ObjectClass | (bool all) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1288 | function (callable) | Detach_This_From_All | ObjectClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1338 | function (callable) | Receive_Message | ObjectClass | (RadioClass *,RadioMessageType message,long &) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1385 | function (callable) | Take_Damage | ObjectClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1491 | function (callable) | Mark | ObjectClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1589 | function (callable) | Init | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1610 | function (callable) | Revealed | ObjectClass | (HouseClass * house) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1628 | function (callable) | Set_Selected_By_Player | ObjectClass | (HouseClass * player) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1663 | function (callable) | Set_Unselected_By_Player | ObjectClass | (HouseClass * player) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1692 | function (callable) | Is_Selected_By_Player | ObjectClass | (HouseClass * player) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1708 | function (callable) | Occupy_List | ObjectClass | (bool placement) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1709 | function (callable) | Overlap_List | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1710 | function (callable) | Who_Can_Build_Me | ObjectClass | (bool intheory,bool legal) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1711 | function (callable) | Health_Ratio | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.CPP | 1712 | function (callable) | Full_Name | ObjectClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.H | 138 | function (callable) | ~ObjectClass | ObjectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.H | 140 | function (callable) | operator < | ObjectClass | (ObjectClass const & object) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.H | 141 | function (callable) | operator > | ObjectClass | (ObjectClass const & object) const | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OBJECT.H | 183 | function (callable) | Detach | ObjectClass | (TARGET,bool) | RUST/original-source-reference/TIBERIANDAWN/OBJECT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 612 | function (callable) | OverlayTypeClass | OverlayTypeClass | (OverlayType iconset,char const * ininame,int fullname,LandType ground,int damagelevels,int damagepoints,bool isradarvisible,bool iswooden,bool istarget,bool iscrushable,bool istiberium,bool high,bool theater,bool walltype,bool iscrate) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 670 | function (callable) | One_Time | OverlayTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 692 | function (callable) | From_Name | OverlayTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 723 | function (callable) | Occupy_List | OverlayTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 744 | function (callable) | Radar_Icon | OverlayTypeClass | (int data) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 770 | function (callable) | Display | OverlayTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 804 | function (callable) | Prep_For_Add | OverlayTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 831 | function (callable) | Create_And_Place | OverlayTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 856 | function (callable) | Create_One_Of | OverlayTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 880 | function (callable) | Draw_It | OverlayTypeClass | (int x,int y,int data) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/ODATA.CPP | 902 | function (callable) | Init | OverlayTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 78 | function (callable) | OptionsClass | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 117 | function (callable) | One_Time | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 138 | function (callable) | Process | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 158 | function (callable) | Set_Shuffle | OptionsClass | (int on) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 179 | function (callable) | Set_Repeat | OptionsClass | (int on) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 200 | function (callable) | Set_Score_Volume | OptionsClass | (int volume) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 226 | function (callable) | Set_Sound_Volume | OptionsClass | (int volume,int feedback) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 251 | function (callable) | Set_Brightness | OptionsClass | (int brightness) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 276 | function (callable) | Get_Brightness | OptionsClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 297 | function (callable) | Set_Color | OptionsClass | (int color) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 323 | function (callable) | Get_Color | OptionsClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 344 | function (callable) | Set_Contrast | OptionsClass | (int contrast) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 369 | function (callable) | Get_Contrast | OptionsClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 389 | function (callable) | Set_Tint | OptionsClass | (int tint) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 414 | function (callable) | Get_Tint | OptionsClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 445 | function (callable) | Adjust_Palette | OptionsClass | (void * oldpal,void * newpal,unsigned char brightness,unsigned char color,unsigned char tint,unsigned char contrast) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 510 | function (callable) | Load_Settings | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 678 | function (callable) | Save_Settings | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 734 | function (callable) | Set | OptionsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 768 | function (callable) | Normalize_Delay | OptionsClass | (int delay) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 788 | function (callable) | Fixup_Palette | OptionsClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OPTIONS.CPP | 794 | function (callable) | Normalize_Sound | OptionsClass | (int volume) const | RUST/original-source-reference/TIBERIANDAWN/OPTIONS.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 53 | function (callable) | OverlayClass | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 72 | function (callable) | Validate | OverlayClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 85 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 104 | function (callable) | Init | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 131 | function (callable) | operator new | OverlayClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 156 | function (callable) | operator delete | OverlayClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 183 | function (callable) | OverlayClass | OverlayClass | (OverlayType type,CELL pos,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 211 | function (callable) | Mark | OverlayClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 349 | function (callable) | Read_INI | OverlayClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.CPP | 400 | function (callable) | Write_INI | OverlayClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 56 | function (callable) | ~OverlayClass | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 57 | function (callable) | operator OverlayType | OverlayClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 58 | function (callable) | What_Am_I | OverlayClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 67 | function (callable) | INI_Name | OverlayClass | (void) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 77 | function (callable) | Class_Of | OverlayClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/OVERLAY.H | 78 | function (callable) | Draw_It | OverlayClass | (int,int,WindowNumberType) | RUST/original-source-reference/TIBERIANDAWN/OVERLAY.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 55 | function (callable) | ~PacketClass | PacketClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 79 | function (callable) | Add_Field | PacketClass | (FieldClass * field) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 97 | function (callable) | PacketClass | PacketClass | (char * curbuf) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 173 | function (callable) | Create_Comms_Packet | PacketClass | (int & size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 260 | function (callable) | Find_Field | PacketClass | (char * id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 284 | function (callable) | Get_Field | PacketClass | (char * id,char & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 308 | function (callable) | Get_Field | PacketClass | (char * id,unsigned char & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 332 | function (callable) | Get_Field | PacketClass | (char * id,short & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 356 | function (callable) | Get_Field | PacketClass | (char * id,unsigned short & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 380 | function (callable) | Get_Field | PacketClass | (char * id,long & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 405 | function (callable) | Get_Field | PacketClass | (char * id,char * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 428 | function (callable) | Get_Field | PacketClass | (char * id,unsigned long & data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.CPP | 453 | function (callable) | Get_Field | PacketClass | (char * id,void * data,int & length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 46 | function (callable) | PacketClass | PacketClass | (short id=0) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 65 | function (callable) | Add_Field | PacketClass | (char * field,char data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 66 | function (callable) | Add_Field | PacketClass | (char * field,unsigned char data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 67 | function (callable) | Add_Field | PacketClass | (char * field,short data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 68 | function (callable) | Add_Field | PacketClass | (char * field,unsigned short data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 69 | function (callable) | Add_Field | PacketClass | (char * field,long data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 70 | function (callable) | Add_Field | PacketClass | (char * field,unsigned long data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 71 | function (callable) | Add_Field | PacketClass | (char * field,char * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PACKET.H | 72 | function (callable) | Add_Field | PacketClass | (char * field,void * data,int length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 36 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 37 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 38 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 39 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 40 | label (assembly label candidate) | WARN |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 41 | label (assembly label candidate) | JUMPS |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 57 | label (assembly label candidate) | DATASEG |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PAGFAULT.ASM | 67 | label (assembly label candidate) | CODESEG |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 45 | function (callable) | PhoneEntryClass | PhoneEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 46 | function (callable) | ~PhoneEntryClass | PhoneEntryClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 48 | function (callable) | operator == | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 50 | function (callable) | operator != | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 52 | function (callable) | operator > | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 54 | function (callable) | operator < | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 56 | function (callable) | operator >= | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PHONE.H | 58 | function (callable) | operator <= | PhoneEntryClass | (PhoneEntryClass & obj) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 69 | function (callable) | PowerClass | PowerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 101 | function (callable) | Init_Clear | PowerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 131 | function (callable) | One_Time | PowerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 169 | function (callable) | Draw_It | PowerClass | (bool complete) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 273 | function (callable) | AI | PowerClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 374 | function (callable) | Refresh_Cells | PowerClass | (CELL cell,short const * list) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 396 | function (callable) | Power_Height | PowerClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.CPP | 439 | function (callable) | Action | PowerClass::PowerButtonClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/POWER.H | 78 | function (callable) | PowerButtonClass | PowerClass::PowerButtonClass | (void) | RUST/original-source-reference/TIBERIANDAWN/POWER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 51 | function (callable) | Read_Private_Config_Struct |  | (char * profile,NewConfigType * config) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 76 | function (callable) | WWGetPrivateProfileHex |  | (char const * section,char const * entry,char * profile) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 118 | function (callable) | WWGetPrivateProfileInt |  | (char const * section,char const * entry,int def,char * profile) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 160 | function (callable) | WWWritePrivateProfileInt |  | (char const * section,char const * entry,int value,char * profile) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 210 | function (callable) | WWGetPrivateProfileString |  | (char const * section,char const * entry,char const * def,char * retbuffer,int retlen,char * profile) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/PROFILE.CPP | 520 | function (callable) | WWWritePrivateProfileString |  | (char const * section,char const * entry,char const * string,char * profile) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 236 | function (callable) | Queue_Mission |  | (TARGET whom,MissionType mission,TARGET target,TARGET destination) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 264 | function (callable) | Queue_Options |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 291 | function (callable) | Queue_Exit |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 318 | function (callable) | Queue_AI |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 371 | function (callable) | Queue_AI_Normal |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 514 | function (callable) | Queue_AI_Multiplayer |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 800 | function (callable) | Wait_For_Players |  | (int first_time,ConnManClass * net,int resend_delta,int dialog_time,int timeout,char * multi_packet_buf,int my_sent,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1078 | function (callable) | Generate_Timing_Event |  | (ConnManClass * net,int my_sent) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1157 | function (callable) | Generate_Real_Timing_Event |  | (ConnManClass * net,int my_sent) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1268 | function (callable) | Generate_Process_Time_Event |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1328 | function (callable) | Process_Send_Period |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1377 | function (callable) | Send_Packets |  | (ConnManClass * net,char * multi_packet_buf,int multi_packet_max,int max_ahead,int my_sent) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1505 | function (callable) | Send_FrameSync |  | (ConnManClass * net,int cmd_count) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1575 | function (callable) | Process_Receive_Packet |  | (ConnManClass * net,char * multi_packet_buf,int id,int packetlen,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1689 | function (callable) | Process_Serial_Packet |  | (char * multi_packet_buf,int first_time) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1820 | function (callable) | Can_Advance |  | (ConnManClass * net,int max_ahead,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1888 | function (callable) | Process_Reconnect_Dialog |  | (CountDownTimerClass * timeout_timer,long * their_frame,int num_conn,int reconn,int fresh) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 1976 | function (callable) | Handle_Timeout |  | (ConnManClass * net,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2059 | function (callable) | Stop_Game |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2108 | function (callable) | Build_Send_Packet |  | (void * buf,int bufsize,int frame_delay,int num_cmds,int cap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2210 | function (callable) | Add_Uncompressed_Events |  | (void * buf,int bufsize,int frame_delay,int size,int cap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2286 | function (callable) | Add_Compressed_Events |  | (void * buf,int bufsize,int frame_delay,int size,int cap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2541 | function (callable) | Breakup_Receive_Packet |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2579 | function (callable) | Extract_Uncompressed_Events |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2637 | function (callable) | Extract_Compressed_Events |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 2839 | function (callable) | Execute_DoList |  | (int,HousesType,ConnManClass * net,TCountDownTimerClass *,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3092 | function (callable) | Clean_DoList |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3140 | function (callable) | Queue_Record |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3190 | function (callable) | Queue_Playback |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3323 | function (callable) | Compute_Game_CRC |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3402 | function (callable) | Add_CRC |  | (unsigned long * crc,unsigned long val) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3435 | function (callable) | Print_CRCs |  | (EventClass * ev) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 3998 | function (callable) | Init_Queue_Mono |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 4078 | function (callable) | Update_Queue_Mono |  | (ConnManClass * net,int flow_index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 4128 | function (callable) | Print_Framesync_Values |  | (long curframe,unsigned long max_ahead,int num_connections,unsigned short * their_recv,unsigned short * their_sent,unsigned short my_sent) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.CPP | 4184 | function (callable) | Dump_Packet_Too_Late_Stuff |  | (EventClass * event) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 134 | function (callable) | QueueClass | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 155 | function (callable) | Init | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 179 | function (callable) | Add | QueueClass | (T const & q) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 211 | function (callable) | Next | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 242 | function (callable) | operator [] | QueueClass | (int index) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/QUEUE.H | 267 | function (callable) | First | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 97 | function (callable) | RadarClass | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 126 | function (callable) | One_Time | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 170 | function (callable) | Init_Clear | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 212 | function (callable) | Radar_Activate | RadarClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 326 | function (callable) | Draw_It | RadarClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 494 | function (callable) | Render_Terrain | RadarClass | (CELL cell,int x,int y,int size) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 580 | function (callable) | Render_Infantry | RadarClass | (CELL cell,int x,int y,int size) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 635 | function (callable) | Render_Overlay | RadarClass | (CELL cell,int x,int y,int size) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 665 | function (callable) | Zoom_Mode | RadarClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 763 | function (callable) | Plot_Radar_Pixel | RadarClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 876 | function (callable) | Radar_Pixel | RadarClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 909 | function (callable) | Click_In_Radar | RadarClass | (int & ptr_x,int & ptr_y,bool change) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 957 | function (callable) | Click_Cell_Calc | RadarClass | (int x,int y) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 987 | function (callable) | Map_Cell | RadarClass | (CELL cell,HouseClass * house,bool and_for_allies) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 996 | function (callable) | Cursor_Cell | RadarClass | (CELL cell,int value) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1020 | function (callable) | Mark_Radar | RadarClass | (int x1,int y1,int x2,int y2,int value,int barlen) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1089 | function (callable) | Cell_XY_To_Radar_Pixel | RadarClass | (int cellx,int celly,int & x,int & y) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1115 | function (callable) | Radar_Cursor | RadarClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1248 | function (callable) | Radar_Anim | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1295 | function (callable) | AI | RadarClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1361 | function (callable) | Action | RadarClass::TacticalClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1552 | function (callable) | Refresh_Cells | RadarClass | (CELL cell,short const * list) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1576 | function (callable) | Set_Radar_Position | RadarClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1722 | function (callable) | Radar_Position | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1745 | function (callable) | Set_Map_Dimensions | RadarClass | (int x,int y,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1768 | function (callable) | Set_Tactical_Position | RadarClass | (COORDINATE coord) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1790 | function (callable) | Cell_On_Radar | RadarClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1820 | function (callable) | Player_Names | RadarClass | (bool on) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.CPP | 1848 | function (callable) | Draw_Names | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 89 | function (callable) | Is_Radar_Active | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 90 | function (callable) | Is_Radar_Activating | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 91 | function (callable) | Is_Radar_Existing | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 103 | function (callable) | Is_Player_Names | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 105 | function (callable) | Is_Zoomed | RadarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADAR.H | 150 | function (callable) | TacticalClass | RadarClass::TacticalClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.CPP | 97 | function (callable) | Debug_Dump | RadioClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.CPP | 132 | function (callable) | Receive_Message | RadioClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.CPP | 188 | function (callable) | Transmit_Message | RadioClass | (RadioMessageType message,long & param,RadioClass * to) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.CPP | 240 | function (callable) | Limbo | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.CPP | 249 | function (callable) | Transmit_Message | RadioClass | (RadioMessageType message,RadioClass * to) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.H | 79 | function (callable) | RadioClass | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.H | 80 | function (callable) | ~RadioClass | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.H | 85 | function (callable) | In_Radio_Contact | RadioClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.H | 86 | function (callable) | Radio_Off | RadioClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RADIO.H | 87 | function (callable) | Contact_With_Whom | RadioClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/RADIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAND.CPP | 55 | function (callable) | Sim_Random |  | (void) | RUST/original-source-reference/TIBERIANDAWN/RAND.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAND.CPP | 104 | function (callable) | Sim_IRandom |  | (int minval,int maxval) | RUST/original-source-reference/TIBERIANDAWN/RAND.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 93 | function (callable) | Error | RawFileClass | (int,int,char const *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 115 | function (callable) | RawFileClass | RawFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 149 | function (callable) | Set_Name | RawFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 191 | function (callable) | Open | RawFileClass | (char const * filename,int rights) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 216 | function (callable) | Open | RawFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 342 | function (callable) | Is_Available | RawFileClass | (int forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 454 | function (callable) | Close | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 529 | function (callable) | Read | RawFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 660 | function (callable) | Write | RawFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 780 | function (callable) | Seek | RawFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 849 | function (callable) | Size | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 938 | function (callable) | Create | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 976 | function (callable) | Delete | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 1066 | function (callable) | Get_Date_Time | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 1144 | function (callable) | Set_Date_Time | RawFileClass | (unsigned long datetime) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 1232 | function (callable) | Bias | RawFileClass | (int start,int length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.CPP | 1275 | function (callable) | Raw_Seek | RawFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.H | 242 | function (callable) | File_Name | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.H | 264 | function (callable) | RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.H | 297 | function (callable) | ~RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RAWFILE.H | 323 | function (callable) | Is_Open | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 208 | function (callable) | Coord_XCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 209 | function (callable) | Coord_YCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 718 | macro (function-like macro) | XYP_COORD |  | (x,y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 719 | function (callable) | Dir_Facing |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 720 | function (callable) | Facing_Dir |  | (FacingType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 721 | function (callable) | Cell_To_Lepton |  | (int cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 722 | function (callable) | Lepton_To_Cell |  | (int lepton) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 723 | function (callable) | XY_Cell |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 724 | function (callable) | XY_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 725 | function (callable) | Coord_X |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 726 | function (callable) | Coord_Y |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 727 | function (callable) | Cell_X |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 728 | function (callable) | Cell_Y |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 729 | function (callable) | Dir_Diff |  | (DirType dir1,DirType dir2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 730 | function (callable) | Coord_XLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 731 | function (callable) | Coord_YLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 733 | function (callable) | Coord_Add |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 734 | function (callable) | Coord_Sub |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 735 | function (callable) | Coord_Snap |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 736 | function (callable) | Coord_Mid |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 737 | function (callable) | Cell_Coord |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 738 | function (callable) | XYPixel_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 740 | function (callable) | Facing_To_32 |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 741 | function (callable) | Direction256 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 742 | function (callable) | Direction |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 743 | function (callable) | Direction8 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 745 | function (callable) | Direction |  | (CELL cell1,CELL cell2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 746 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,FacingType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 747 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 748 | function (callable) | Adjacent_Cell |  | (CELL cell,FacingType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 749 | function (callable) | Adjacent_Cell |  | (CELL cell,DirType dir) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 750 | function (callable) | Lepton_To_Pixel |  | (int lepton) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 751 | function (callable) | Pixel_To_Lepton |  | (int pixel) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 753 | function (callable) | XYP_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 754 | function (callable) | Text_String |  | (int string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 757 | function (callable) | Random_Pick |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 762 | function (callable) | Sim_Random_Pick |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 769 | macro (function-like macro) | Check_Ptr |  | (ptr,file,line) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 779 | macro (function-like macro) | Check_Ptr |  | (ptr,file,line) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 785 | function (callable) | MPlayerID_To_ColorIndex |  | (unsigned short id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 786 | function (callable) | MPlayerID_To_HousesType |  | (unsigned short id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 787 | function (callable) | Build_MPlayerID |  | (int c_idx,HousesType htype) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 858 | function (callable) | Distance |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 893 | function (callable) | Distance |  | (CELL coord1,CELL coord2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REAL.H | 923 | function (callable) | Cell_Number | CellClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 41 | function (callable) | RegionClass | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 42 | function (callable) | ~RegionClass | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 43 | function (callable) | operator != | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 44 | function (callable) | operator == | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 45 | function (callable) | operator > | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 46 | function (callable) | operator < | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 48 | function (callable) | Reset_Threat | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 49 | function (callable) | Adjust_Threat | RegionClass | (int threat,int neg) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REGION.H | 50 | function (callable) | Threat_Value | RegionClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REINF.CPP | 60 | function (callable) | Do_Reinforcements |  | (TeamTypeClass * teamtype) | RUST/original-source-reference/TIBERIANDAWN/REINF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REINF.CPP | 491 | function (callable) | Create_Special_Reinforcement |  | (HouseClass * house,TechnoTypeClass const * type,TechnoTypeClass const * another,TeamMissionType mission,int argument) | RUST/original-source-reference/TIBERIANDAWN/REINF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/REINF.CPP | 611 | function (callable) | Create_Air_Reinforcement |  | (HouseClass * house,AircraftType air,int number,MissionType mission,TARGET tarcom,TARGET navcom) | RUST/original-source-reference/TIBERIANDAWN/REINF.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RULES.CPP | 57 | function (callable) | DifficultyClass | DifficultyClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RULES.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/RULES.CPP | 90 | function (callable) | RulesClass | RulesClass | (void) | RUST/original-source-reference/TIBERIANDAWN/RULES.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVEDLG.H | 63 | function (callable) | SaveOptionsClass | SaveOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 129 | function (callable) | Save_Game |  | (int id,char * descr) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 145 | function (callable) | Save_Game |  | (const char * file_name,const char * descr) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 347 | function (callable) | Load_Game |  | (int id) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 362 | function (callable) | Load_Game |  | (const char * file_name) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 593 | function (callable) | Save_Misc_Values |  | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 689 | function (callable) | Load_Misc_Values |  | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 792 | function (callable) | Code_All_Pointers |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 878 | function (callable) | Decode_All_Pointers |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1004 | function (callable) | Read_Object |  | (void * ptr,int class_size,FileClass & file,bool has_vtable) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1069 | function (callable) | Write_Object |  | (void * ptr,int class_size,FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1167 | function (callable) | Read_Object |  | (void * ptr,int base_size,int class_size,FileClass & file,void * vtable) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1230 | function (callable) | Write_Object |  | (void * ptr,int class_size,FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1268 | function (callable) | Get_Savefile_Info |  | (int id,char * buf,unsigned * scenp,HousesType * housep) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1344 | function (callable) | TechnoType_To_Target |  | (TechnoTypeClass const * ptr) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1391 | function (callable) | Target_To_TechnoType |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1425 | function (callable) | Get_VTable |  | (void * ptr,int base_size) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SAVELOAD.CPP | 1448 | function (callable) | Set_VTable |  | (void * ptr,int base_size,void * vtable) | RUST/original-source-reference/TIBERIANDAWN/SAVELOAD.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 68 | function (callable) | Start_Scenario |  | (char * root,bool briefing) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 166 | function (callable) | Set_Scenario_Difficulty |  | (int difficulty) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 206 | function (callable) | Read_Scenario |  | (char * root) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 267 | function (callable) | Fill_In_Data |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 311 | function (callable) | Clear_Scenario |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 370 | function (callable) | Do_Win |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 596 | function (callable) | Do_Lose |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 691 | function (callable) | Do_Restart |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 724 | function (callable) | Restate_Mission |  | (char const * name,int button1,int button2) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCENARIO.CPP | 781 | function (callable) | Fixup_Scenario |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCENARIO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 209 | function (callable) | ScoreAnimClass | ScoreAnimClass | (int x,int y,void const * data) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 220 | function (callable) | ScoreTimeClass | ScoreTimeClass | (int xpos,int ypos,void const * data,int max,int timer) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 228 | function (callable) | Update | ScoreTimeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 241 | function (callable) | ScoreCredsClass | ScoreCredsClass | (int xpos,int ypos,void const * data,int max,int timer) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 252 | function (callable) | Update | ScoreCredsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 273 | function (callable) | ScorePrintClass | ScorePrintClass | (int string,int xpos,int ypos,void const * palette,int background) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 282 | function (callable) | ScorePrintClass | ScorePrintClass | (void const * string,int xpos,int ypos,void const * palette,int background) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 291 | function (callable) | Update | ScorePrintClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 350 | function (callable) | MultiStagePrintClass | MultiStagePrintClass | (int string,int xpos,int ypos,void const * palette,int background) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 359 | function (callable) | MultiStagePrintClass | MultiStagePrintClass | (void const * string,int xpos,int ypos,void const * palette,int background) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 368 | function (callable) | Update | MultiStagePrintClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 430 | function (callable) | ScoreScaleClass | ScoreScaleClass | (void const * string,int xpos,int ypos,char const palette[]) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 438 | function (callable) | Update | ScoreScaleClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 479 | function (callable) | Alloc_Object |  | (ScoreAnimClass * obj) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 497 | function (callable) | TextBlitClass | TextBlitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 504 | function (callable) | Add | TextBlitClass | (int x,int y,int dx,int dy,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 520 | function (callable) | Clear | TextBlitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 528 | function (callable) | Update | TextBlitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 571 | function (callable) | Presentation | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1006 | function (callable) | Cycle_Wait_Click |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1074 | function (callable) | Do_Nod_Buildings_Graph | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1197 | function (callable) | Do_GDI_Graph | ScoreClass | (void const * yellowptr,void const * redptr,int gkilled,int nkilled,int ypos) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1266 | function (callable) | Do_Nod_Casualties_Graph | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1382 | function (callable) | Show_Credits | ScoreClass | (int house,char const pal[]) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1456 | function (callable) | Print_Minutes | ScoreClass | (int minutes) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1489 | function (callable) | Count_Up_Print | ScoreClass | (char * str,int percent,int max,int xpos,int ypos) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1536 | function (callable) | Input_Name | ScoreClass | (char str[],int xpos,int ypos,char const pal[]) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1609 | function (callable) | Animate_Cursor |  | (int pos,int ypos) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1651 | function (callable) | Draw_InfantryMen |  | () | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1687 | function (callable) | Draw_InfantryMan |  | (int index) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1738 | function (callable) | New_Infantry_Anim |  | (int index,int anim) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1766 | function (callable) | Draw_Bar_Graphs |  | (int i,int gkilled,int nkilled,int ckilled) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1834 | function (callable) | Call_Back_Delay |  | (int time) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1869 | function (callable) | Animate_Score_Objs |  | () | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1879 | function (callable) | Int_Print |  | (int a) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 1901 | function (callable) | Multi_Score_Presentation |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 2084 | function (callable) | CCWrite_LBM_File |  | (CCFileClass & lbmhandle,BufferClass & buff,short bitplanes,unsigned char * palette) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 2130 | function (callable) | CCWrite_BMHD |  | (CCFileClass & lbmhandle,short bitplanes) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 2162 | function (callable) | CCWrite_CMAP |  | (CCFileClass & lbmhandle,unsigned char * palette,short bitplanes) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 2210 | function (callable) | CCWrite_BODY |  | (CCFileClass & lbmhandle,BufferClass & buff,short bitplanes) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.CPP | 2264 | function (callable) | CCWrite_Row |  | (CCFileClass & lbmhandle,unsigned char * buffer) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 55 | function (callable) | Init | ScoreClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 90 | function (callable) | Update | ScoreAnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 91 | function (callable) | ~ScoreAnimClass | ScoreAnimClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 104 | function (callable) | ~ScoreCredsClass | ScoreCredsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 114 | function (callable) | ~ScoreTimeClass | ScoreTimeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 125 | function (callable) | ~ScorePrintClass | ScorePrintClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 137 | function (callable) | ~MultiStagePrintClass | MultiStagePrintClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCORE.H | 147 | function (callable) | ~ScoreScaleClass | ScoreScaleClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCORE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCREEN.H | 51 | function (callable) | ScreenClass | ScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCROLL.CPP | 60 | function (callable) | ScrollClass | ScrollClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCROLL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCROLL.CPP | 88 | function (callable) | AI | ScrollClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/SCROLL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCROLL.CPP | 238 | function (callable) | Set_Autoscroll | ScrollClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/SCROLL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SCROLL.H | 74 | function (callable) | Init_IO | ScrollClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SCROLL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 216 | function (callable) | SmudgeTypeClass | SmudgeTypeClass | (SmudgeType smudge,char const * ininame,int fullname,int width,int height,bool isbib,bool iscrater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 242 | function (callable) | From_Name | SmudgeTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 272 | function (callable) | Occupy_List | SmudgeTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 302 | function (callable) | Init | SmudgeTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 334 | function (callable) | Display | SmudgeTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 367 | function (callable) | Prep_For_Add | SmudgeTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 395 | function (callable) | Create_And_Place | SmudgeTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 422 | function (callable) | Create_One_Of | SmudgeTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 443 | function (callable) | Draw_It | SmudgeTypeClass | (int x,int y,int data) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SDATA.CPP | 467 | function (callable) | One_Time | SmudgeTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 71 | function (callable) | SequencedConnClass | SequencedConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 106 | function (callable) | ~SequencedConnClass | SequencedConnClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 127 | function (callable) | Init | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 156 | function (callable) | Send_Packet | SequencedConnClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 213 | function (callable) | Receive_Packet | SequencedConnClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 344 | function (callable) | Get_Packet | SequencedConnClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 393 | function (callable) | Service_Send_Queue | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SEQCONN.CPP | 505 | function (callable) | Service_Receive_Queue | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SHAPEBTN.CPP | 54 | function (callable) | ShapeButtonClass | ShapeButtonClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SHAPEBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SHAPEBTN.CPP | 84 | function (callable) | ShapeButtonClass | ShapeButtonClass | (unsigned id,void const * shape,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/SHAPEBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SHAPEBTN.CPP | 94 | function (callable) | Set_Shape | ShapeButtonClass | (void const * data) | RUST/original-source-reference/TIBERIANDAWN/SHAPEBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SHAPEBTN.CPP | 120 | function (callable) | Draw_Me | ShapeButtonClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/SHAPEBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 145 | function (callable) | SidebarClass | SidebarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 171 | function (callable) | One_Time | SidebarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 237 | function (callable) | Init_Clear | SidebarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 265 | function (callable) | Init_IO | SidebarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 390 | function (callable) | Init_Theater | SidebarClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 414 | function (callable) | Which_Column | SidebarClass | (RTTIType type) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 443 | function (callable) | Factory_Link | SidebarClass | (int factory,RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 468 | function (callable) | Refresh_Cells | SidebarClass | (CELL cell,short const * list) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 498 | function (callable) | Activate_Repair | SidebarClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 546 | function (callable) | Activate_Upgrade | SidebarClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 591 | function (callable) | Activate_Demolish | SidebarClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 636 | function (callable) | Add | SidebarClass | (RTTIType type,int id,bool via_capture) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 677 | function (callable) | Scroll | SidebarClass | (bool up,int column) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 703 | function (callable) | Draw_It | SidebarClass | (bool complete) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 776 | function (callable) | AI | SidebarClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 922 | function (callable) | Recalc | SidebarClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 959 | function (callable) | Activate | SidebarClass | (int control) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1063 | function (callable) | StripClass | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1095 | function (callable) | One_Time | SidebarClass::StripClass | (int) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1145 | function (callable) | Get_Special_Cameo | SidebarClass::StripClass | (int type) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1163 | function (callable) | Init_Clear | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1199 | function (callable) | Init_IO | SidebarClass::StripClass | (int id) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1242 | function (callable) | Init_Theater | SidebarClass::StripClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1312 | function (callable) | Activate | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1342 | function (callable) | Deactivate | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1373 | function (callable) | sortfunc |  | (void const * ptr1,void const * ptr2) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1438 | function (callable) | Add | SidebarClass::StripClass | (RTTIType type,int id,bool via_capture) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1483 | function (callable) | Scroll | SidebarClass::StripClass | (bool up) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1530 | function (callable) | Flag_To_Redraw | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1559 | function (callable) | AI | SidebarClass::StripClass | (KeyNumType & input,int,int) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1727 | function (callable) | Draw_It | SidebarClass::StripClass | (bool complete) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 1975 | function (callable) | Recalc | SidebarClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2102 | function (callable) | SelectClass | SidebarClass::StripClass::SelectClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2133 | function (callable) | Set_Owner | SidebarClass::StripClass::SelectClass | (StripClass & strip,int index) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2160 | function (callable) | Action | SidebarClass::StripClass::SelectClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2440 | function (callable) | Action | SidebarClass::SBGadgetClass | (unsigned,KeyNumType &) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2470 | function (callable) | Factory_Link | SidebarClass::StripClass | (int factory,RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2507 | function (callable) | Abandon_Production | SidebarClass | (RTTIType type,int factory) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.CPP | 2529 | function (callable) | Abandon_Production | SidebarClass::StripClass | (int factory) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBAR.H | 361 | function (callable) | SBGadgetClass | SidebarClass::SBGadgetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBAR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 69 | function (callable) | SidebarGlyphxClass | SidebarGlyphxClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 92 | function (callable) | Init_Clear | SidebarGlyphxClass | (HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 123 | function (callable) | Init_IO | SidebarGlyphxClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 151 | function (callable) | Which_Column | SidebarGlyphxClass | (RTTIType type) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 180 | function (callable) | Factory_Link | SidebarGlyphxClass | (int factory,RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 203 | function (callable) | Add | SidebarGlyphxClass | (RTTIType type,int id,bool via_capture) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 247 | function (callable) | AI | SidebarGlyphxClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 275 | function (callable) | Recalc | SidebarGlyphxClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 297 | function (callable) | StripClass | SidebarGlyphxClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 324 | function (callable) | Init_Clear | SidebarGlyphxClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 360 | function (callable) | Add | SidebarGlyphxClass::StripClass | (RTTIType type,int id,bool via_capture) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 404 | function (callable) | AI | SidebarGlyphxClass::StripClass | (KeyNumType & input,int,int) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 484 | function (callable) | Recalc | SidebarGlyphxClass::StripClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 566 | function (callable) | Factory_Link | SidebarGlyphxClass::StripClass | (int factory,RTTIType type,int id) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 599 | function (callable) | Abandon_Production | SidebarGlyphxClass | (RTTIType type,int factory) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 621 | function (callable) | Abandon_Production | SidebarGlyphxClass::StripClass | (int factory) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 663 | function (callable) | Code_Pointers | SidebarGlyphxClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 685 | function (callable) | Decode_Pointers | SidebarGlyphxClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 708 | function (callable) | Load | SidebarGlyphxClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 733 | function (callable) | Save | SidebarGlyphxClass | (FileClass & file) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 742 | function (callable) | Sidebar_Glyphx_Init_Clear |  | (HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 750 | function (callable) | Sidebar_Glyphx_Init_IO |  | (HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 758 | function (callable) | Sidebar_Glyphx_Abandon_Production |  | (RTTIType type,int factory,HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 768 | function (callable) | Sidebar_Glyphx_Add |  | (RTTIType type,int id,HouseClass * player_ptr,bool via_capture) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 778 | function (callable) | Sidebar_Glyphx_Recalc |  | (HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 786 | function (callable) | Sidebar_Glyphx_AI |  | (HouseClass * player_ptr,KeyNumType & input) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 794 | function (callable) | Sidebar_Glyphx_Factory_Link |  | (int factory,RTTIType type,int id,HouseClass * player_ptr) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 804 | function (callable) | Sidebar_Glyphx_Save |  | (FileClass & file,SidebarGlyphxClass * sidebar) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 812 | function (callable) | Sidebar_Glyphx_Load |  | (FileClass & file,SidebarGlyphxClass * sidebar) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 820 | function (callable) | Sidebar_Glyphx_Code_Pointers |  | (SidebarGlyphxClass * sidebar) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.CPP | 827 | function (callable) | Sidebar_Glyphx_Decode_Pointers |  | (SidebarGlyphxClass * sidebar) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SIDEBARGlyphx.H | 126 | function (callable) | Set_Parent_Sidebar | SidebarGlyphxClass::StripClass | (SidebarGlyphxClass * parent) | RUST/original-source-reference/TIBERIANDAWN/SIDEBARGlyphx.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 62 | function (callable) | SliderClass | SliderClass | (unsigned id,int x,int y,int w,int h,int belong_to_list) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 94 | function (callable) | ~SliderClass | SliderClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 119 | function (callable) | Set_Maximum | SliderClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 142 | function (callable) | Set_Thumb_Size | SliderClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 163 | function (callable) | Set_Value | SliderClass | (int value) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 187 | function (callable) | Recalc_Thumb | SliderClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 211 | function (callable) | Action | SliderClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 275 | function (callable) | Bump | SliderClass | (int up) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 295 | function (callable) | Step | SliderClass | (int up) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 315 | function (callable) | Draw_Thumb | SliderClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 335 | function (callable) | Draw_Me | SliderClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.CPP | 393 | function (callable) | Peer_To_Peer | SliderClass | (unsigned flags,KeyNumType &,ControlClass & whom) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SLIDER.H | 70 | function (callable) | Thumb_Pixels | SliderClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SLIDER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 72 | function (callable) | Validate | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 85 | macro (function-like macro) | Validate |  | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 104 | function (callable) | operator new | SmudgeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 128 | function (callable) | operator delete | SmudgeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 159 | function (callable) | SmudgeClass | SmudgeClass | (SmudgeType type,COORDINATE pos,HousesType house) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 187 | function (callable) | Init | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 215 | function (callable) | Mark | SmudgeClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 291 | function (callable) | Read_INI | SmudgeClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 337 | function (callable) | Write_INI | SmudgeClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.CPP | 388 | function (callable) | Disown | SmudgeClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 56 | function (callable) | SmudgeClass | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 57 | function (callable) | operator SmudgeType | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 58 | function (callable) | ~SmudgeClass | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 59 | function (callable) | What_Am_I | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 68 | function (callable) | INI_Name | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 74 | function (callable) | Class_Of | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SMUDGE.H | 76 | function (callable) | Draw_It | SmudgeClass | (int,int,WindowNumberType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.CPP | 42 | function (callable) | MusicListClass | MusicListClass | (int id,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.CPP | 45 | function (callable) | ~MusicListClass | MusicListClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.CPP | 50 | function (callable) | Init | SoundControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.CPP | 112 | function (callable) | Process | SoundControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.CPP | 412 | function (callable) | Draw_Entry | MusicListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SOUNDDLG.H | 109 | function (callable) | SoundControlsClass | SoundControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SPECIAL.CPP | 42 | function (callable) | Special_Dialog |  | (void) | RUST/original-source-reference/TIBERIANDAWN/SPECIAL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SPECIAL.H | 41 | function (callable) | Init | SpecialClass | (void) | RUST/original-source-reference/TIBERIANDAWN/SPECIAL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 60 | function (callable) | StageClass | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 66 | function (callable) | Fetch_Stage | StageClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 67 | function (callable) | Fetch_Rate | StageClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 68 | function (callable) | Set_Stage | StageClass | (int stage) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 69 | function (callable) | Set_Rate | StageClass | (unsigned char rate) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 70 | function (callable) | AI | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 71 | function (callable) | Graphic_Logic | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 83 | function (callable) | Debug_Dump | StageClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 92 | function (callable) | Code_Pointers | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STAGE.H | 93 | function (callable) | Decode_Pointers | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 69 | function (callable) | CD_Test |  | (void) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 121 | function (callable) | DllMain |  | (HINSTANCE instance,unsigned int fdwReason,void * lpvReserved) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 160 | function (callable) | DLL_Startup |  | (const char * command_line_in) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 695 | function (callable) | Prog_End |  | (const char * why,bool fatal) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 752 | function (callable) | Delete_Swap_Files |  | (void) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 766 | function (callable) | Print_Error_End_Exit |  | (char * string) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 778 | function (callable) | Print_Error_Exit |  | (char * string) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STARTUP.CPP | 807 | function (callable) | Read_Setup_Options |  | (RawFileClass * config_file) | RUST/original-source-reference/TIBERIANDAWN/STARTUP.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STATS.CPP | 154 | function (callable) | Send_Statistics_Packet |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STATS.CPP | 663 | function (callable) | Register_Game_Start_Time |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/STATS.CPP | 671 | function (callable) | Register_Game_End_Time |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 68 | function (callable) | SuperClass | SuperClass | (int recharge,VoxType ready,VoxType charging,VoxType impatient,VoxType suspend) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 100 | function (callable) | Suspend | SuperClass | (bool on) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 138 | function (callable) | Enable | SuperClass | (bool onetime,bool player,bool quiet) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 167 | function (callable) | Remove | SuperClass | (bool forced) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 194 | function (callable) | Recharge | SuperClass | (bool player) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 234 | function (callable) | Discharged | SuperClass | (bool player) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 267 | function (callable) | AI | SuperClass | (bool player) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 310 | function (callable) | Anim_Stage | SuperClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 343 | function (callable) | Impatient_Click | SuperClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.CPP | 374 | function (callable) | Forced_Charge | SuperClass | (bool player) | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.H | 52 | function (callable) | Is_Ready | SuperClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.H | 53 | function (callable) | Is_Present | SuperClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.H | 54 | function (callable) | Is_One_Time | SuperClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPER.H | 57 | function (callable) | Get_Recharge_Time | SuperClass | () const | RUST/original-source-reference/TIBERIANDAWN/SUPER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPPORT.ASM | 39 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPPORT.ASM | 40 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPPORT.ASM | 41 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/SUPPORT.ASM | 43 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/Shape.cpp | 52 | function (callable) | Set_Shape_Buffer |  | (void const * buffer,int size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 59 | function (callable) | TabClass | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 84 | function (callable) | Draw_It | TabClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 132 | function (callable) | Draw_Credits_Tab | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 153 | function (callable) | Hilite_Tab | TabClass | (int tab) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 184 | function (callable) | AI | TabClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 236 | function (callable) | Set_Active | TabClass | (int select) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.CPP | 252 | function (callable) | One_Time | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.H | 52 | function (callable) | Redraw_Tab | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TAB.H | 59 | function (callable) | Get_Tab_Height | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARCOM.CPP | 55 | function (callable) | ~TarComClass | TarComClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARCOM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARCOM.CPP | 75 | function (callable) | Debug_Dump | TarComClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/TARCOM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARCOM.CPP | 96 | function (callable) | AI | TarComClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARCOM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARCOM.H | 52 | function (callable) | TarComClass | TarComClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARCOM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARCOM.H | 53 | function (callable) | TarComClass | TarComClass | (UnitType classid,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/TARCOM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 69 | function (callable) | As_Trigger |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 94 | function (callable) | As_Team |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 119 | function (callable) | As_TeamType |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 140 | function (callable) | As_Animation |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 165 | function (callable) | As_Bullet |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 190 | function (callable) | As_Aircraft |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 216 | function (callable) | As_Techno |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 244 | function (callable) | As_Object |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 313 | function (callable) | As_Unit |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 339 | function (callable) | As_Infantry |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 350 | function (callable) | As_Terrain |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 372 | function (callable) | As_Building |  | (TARGET target,bool check_active) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 399 | function (callable) | Target_Legal |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 426 | function (callable) | As_Cell |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 448 | function (callable) | As_Coord |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.CPP | 504 | function (callable) | As_Movement_Coord |  | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 65 | function (callable) | Target_Kind |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 66 | function (callable) | Target_Value |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 68 | function (callable) | Is_Target_Team |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 69 | function (callable) | Is_Target_TeamType |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 70 | function (callable) | Is_Target_Trigger |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 71 | function (callable) | Is_Target_Infantry |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 72 | function (callable) | Is_Target_Bullet |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 73 | function (callable) | Is_Target_Terrain |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 74 | function (callable) | Is_Target_Cell |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 75 | function (callable) | Is_Target_Unit |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 76 | function (callable) | Is_Target_Building |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 77 | function (callable) | Is_Target_Template |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 78 | function (callable) | Is_Target_Aircraft |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 79 | function (callable) | Is_Target_Animation |  | (TARGET a) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 81 | function (callable) | Build_Target |  | (KindType kind,int value) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 82 | function (callable) | As_Target |  | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 101 | function (callable) | TargetClass | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 107 | function (callable) | TargetClass | TargetClass | (int val,KindType kind) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 108 | function (callable) | TargetClass | TargetClass | (int val) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 109 | function (callable) | operator int | TargetClass | () | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 112 | function (callable) | Is_Filled | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 113 | function (callable) | Invalidate | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 114 | function (callable) | Is_Cell | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 115 | function (callable) | Is_Unit | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 116 | function (callable) | Is_Building | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 117 | function (callable) | Is_Aircraft | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 118 | function (callable) | As_Value | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 119 | function (callable) | As_Kind | TargetClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TARGET.H | 122 | function (callable) | operator == | TargetClass | (TargetClass t1) | RUST/original-source-reference/TIBERIANDAWN/TARGET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 85 | function (callable) | TcpipManagerClass | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 111 | function (callable) | ~TcpipManagerClass | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 132 | function (callable) | Close | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 190 | function (callable) | Init | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 250 | function (callable) | Start_Server | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 345 | function (callable) | Read | TcpipManagerClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 390 | function (callable) | Write | TcpipManagerClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 437 | function (callable) | Add_Client | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 545 | function (callable) | Message_Handler | TcpipManagerClass | (HWND,UINT message,UINT,LONG lParam) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 806 | function (callable) | Copy_To_In_Buffer | TcpipManagerClass | (int bytes) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 832 | function (callable) | Set_Host_Address | TcpipManagerClass | (char * address) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 854 | function (callable) | Start_Client | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 970 | function (callable) | Close_Socket | TcpipManagerClass | (SOCKET s) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 981 | function (callable) | Set_Protocol_UDP | TcpipManagerClass | (BOOL state) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.CPP | 988 | function (callable) | Clear_Socket_Error | TcpipManagerClass | (SOCKET socket) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.H | 105 | function (callable) | Get_Connected | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TCPIP.H | 120 | function (callable) | Get_Connection_Status | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 803 | function (callable) | TerrainTypeClass | TerrainTypeClass | (TerrainType terrain,int theater,COORDINATE centerbase,bool is_spawn,bool is_destroyable,bool is_transformable,bool is_flammable,bool is_crushable,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,char const * ininame,int fullname,unsigned short strength,ArmorType armor,short const * occupy,short const * overlap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 848 | function (callable) | Init | TerrainTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 895 | function (callable) | From_Name | TerrainTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 928 | function (callable) | Display | TerrainTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 950 | function (callable) | Prep_For_Add | TerrainTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 975 | function (callable) | Create_And_Place | TerrainTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 1001 | function (callable) | Create_One_Of | TerrainTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 1007 | function (callable) | Occupy_List | TerrainTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TDATA.CPP | 1018 | function (callable) | Overlap_List | TerrainTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 89 | function (callable) | Validate | TeamClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 102 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 121 | function (callable) | Init | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 135 | function (callable) | operator new | TeamClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 145 | function (callable) | operator delete | TeamClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 154 | function (callable) | ~TeamClass | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 169 | function (callable) | TeamClass | TeamClass | (TeamTypeClass const * type,HouseClass * owner) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 210 | function (callable) | Assign_Mission_Target | TeamClass | (TARGET new_target) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 280 | function (callable) | AI | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 650 | function (callable) | Add | TeamClass | (FootClass * obj,int typeindex) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 764 | function (callable) | Remove | TeamClass | (FootClass * obj,int typeindex) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 872 | function (callable) | Recruit | TeamClass | (int typeindex) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 960 | function (callable) | Detach | TeamClass | (TARGET target,bool) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 994 | function (callable) | As_Target | TeamClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1018 | function (callable) | Calc_Center | TeamClass | (CELL & center,CELL & obj_center) const | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1073 | function (callable) | Took_Damage | TeamClass | (FootClass *,ResultType result,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1120 | function (callable) | Coordinate_Attack | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1172 | function (callable) | Coordinate_Regroup | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1234 | function (callable) | Coordinate_Move | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1314 | function (callable) | Lagging_Units | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1379 | function (callable) | Coordinate_Unload | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1436 | function (callable) | Coordinate_Conscript | TeamClass | (FootClass * unit) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1471 | function (callable) | Is_A_Member | TeamClass | (void const * who) const | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.CPP | 1497 | function (callable) | Suspend_Teams | TeamClass | (int priority) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.H | 167 | function (callable) | TeamClass | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.H | 170 | function (callable) | What_Am_I | TeamClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAM.H | 186 | function (callable) | Force_Active | TeamClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAM.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 93 | function (callable) | Validate | TeamTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 106 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 122 | function (callable) | TeamTypeClass | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 159 | function (callable) | Init | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 193 | function (callable) | Read_INI | TeamTypeClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 272 | function (callable) | Fill_In | TeamTypeClass | (char * name,char * entry) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 435 | function (callable) | Write_INI | TeamTypeClass | (char * buffer,bool refresh) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 545 | function (callable) | Read_Old_INI | TeamTypeClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 714 | function (callable) | As_Pointer | TeamTypeClass | (char * name) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 747 | function (callable) | Remove | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 785 | function (callable) | Mission_From_Name | TeamTypeClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 816 | function (callable) | Name_From_Mission | TeamTypeClass | (TeamMissionType order) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 841 | function (callable) | operator new | TeamTypeClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 866 | function (callable) | operator delete | TeamTypeClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 876 | function (callable) | Create_One_Of | TeamTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 885 | function (callable) | As_Target | TeamTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 892 | function (callable) | Destroy_All_Of | TeamTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.CPP | 929 | function (callable) | Suggested_New_Team | TeamTypeClass | (HouseClass * house,long utypes,long itypes,bool alerted) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.H | 93 | function (callable) | ~TeamTypeClass | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEAMTYPE.H | 107 | function (callable) | INI_Name | TeamTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEAMTYPE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 149 | function (callable) | TechnoTypeClass | TechnoTypeClass | (int name,char const * ininame,unsigned char level,long pre,bool is_leader,bool is_scanner,bool is_nominal,bool is_transporter,bool is_flammable,bool is_crushable,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,bool is_theater,bool is_twoshooter,bool is_turret_equipped,bool is_repairable,bool is_buildable,bool is_crew,int ammo,unsigned short strength,MPHType maxspeed,int sightrange,int cost,int scenario,int risk,int reward,int ownable,WeaponType primary,WeaponType secondary,ArmorType armor) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 248 | function (callable) | Raw_Cost | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 269 | function (callable) | Get_Ownable | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 291 | function (callable) | Time_To_Build | TechnoTypeClass | (HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 365 | function (callable) | Cost_Of | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 387 | function (callable) | Get_Cameo_Data | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 409 | function (callable) | Repair_Cost | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 431 | function (callable) | Repair_Step | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 453 | function (callable) | Debug_Dump | TechnoClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 491 | function (callable) | TechnoClass | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 544 | function (callable) | Revealed | TechnoClass | (HouseClass * house) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 640 | function (callable) | Revealed | TechnoClass | (HouseClass * house) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 708 | function (callable) | Hidden | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 736 | function (callable) | Mark | TechnoClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 774 | function (callable) | Receive_Message | TechnoClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 899 | function (callable) | TechnoClass | TechnoClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 966 | function (callable) | Per_Cell_Process | TechnoClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1019 | function (callable) | Draw_It | TechnoClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1155 | function (callable) | Unlimbo | TechnoClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1186 | function (callable) | In_Range | TechnoClass | (TARGET target,int which,bool reciprocal_check) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1240 | function (callable) | In_Range | TechnoClass | (ObjectClass const * target,int which,bool reciprocal_check) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1291 | function (callable) | In_Range | TechnoClass | (COORDINATE coord,int which) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1333 | function (callable) | Evaluate_Object | TechnoClass | (ThreatType method,int mask,int range,TechnoClass const * object,int & value) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1534 | function (callable) | Evaluate_Cell | TechnoClass | (ThreatType method,int mask,CELL cell,int range,TechnoClass const ** object,int & value) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1584 | function (callable) | Greatest_Threat | TechnoClass | (ThreatType method) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1824 | function (callable) | Owner | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1846 | function (callable) | Clicked_As_Target | TechnoClass | (HousesType house,int count) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 1881 | function (callable) | AI | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2049 | function (callable) | Select | TechnoClass | (bool allow_mixed) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2088 | function (callable) | Can_Fire | TechnoClass | (TARGET target,int which) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2175 | function (callable) | Stun | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2203 | function (callable) | Assign_Target | TechnoClass | (TARGET target) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2253 | function (callable) | Rearm_Delay | TechnoClass | (bool second) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2282 | function (callable) | Fire_At | TechnoClass | (TARGET target,int which) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2508 | function (callable) | Player_Assign_Mission | TechnoClass | (MissionType mission,TARGET target,TARGET destination) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2537 | function (callable) | What_Action | TechnoClass | (ObjectClass * object) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2652 | function (callable) | What_Action | TechnoClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2734 | function (callable) | Can_Player_Move | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2756 | function (callable) | Can_Player_Fire | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2780 | function (callable) | Is_Weapon_Equipped | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2803 | function (callable) | Can_Repair | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2829 | function (callable) | Weapon_Range | TechnoClass | (int which) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2870 | function (callable) | Override_Mission | TechnoClass | (MissionType mission,TARGET tarcom,TARGET navcom) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2890 | function (callable) | Restore_Mission | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 2917 | function (callable) | Captured | TechnoClass | (HouseClass * newowner) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3003 | function (callable) | Take_Damage | TechnoClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3066 | function (callable) | Max_Passengers | TechnoTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3092 | function (callable) | Record_The_Kill | TechnoClass | (TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3228 | function (callable) | Nearby_Location | TechnoClass | (TechnoClass const *) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3289 | function (callable) | Do_Uncloak | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3314 | function (callable) | Do_Cloak | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3342 | function (callable) | Do_Shimmer | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3369 | function (callable) | Visual_Character | TechnoClass | (bool raw) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3427 | function (callable) | Techno_Draw_Object | TechnoClass | (void const * shapefile,int shapenum,int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3476 | function (callable) | Techno_Draw_Object_Virtual | TechnoClass | (void const * shapefile,int shapenum,int x,int y,WindowNumberType window,const char * shape_name) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3527 | function (callable) | Remap_Table | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3554 | function (callable) | Detach | TechnoClass | (TARGET target,bool all) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3597 | function (callable) | Kill_Cargo | TechnoClass | (TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3625 | function (callable) | Crew_Type | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3659 | function (callable) | Value | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3701 | function (callable) | Threat_Range | TechnoClass | (int control) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3730 | function (callable) | Base_Is_Attacked | TechnoClass | (TechnoClass const * enemy) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3959 | function (callable) | Get_Ownable | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 3980 | function (callable) | Is_Techno | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4000 | function (callable) | Risk | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4026 | function (callable) | Tiberium_Load | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4052 | function (callable) | Desired_Load_Dir | TechnoClass | (ObjectClass *,CELL & moveto) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4075 | function (callable) | Pip_Count | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4097 | function (callable) | Fire_Direction | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4118 | function (callable) | Response_Select | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4138 | function (callable) | Response_Move | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4158 | function (callable) | Response_Attack | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4179 | function (callable) | Target_Something_Nearby | TechnoClass | (ThreatType threat) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4224 | function (callable) | Exit_Object | TechnoClass | (TechnoClass *) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4247 | function (callable) | Random_Animate | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4267 | function (callable) | Assign_Destination | TechnoClass | (TARGET) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4291 | function (callable) | Scatter | TechnoClass | (COORDINATE,bool,bool) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4314 | function (callable) | Enter_Idle_Mode | TechnoClass | (bool) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4338 | function (callable) | Draw_Pips | TechnoClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4410 | function (callable) | Find_Docking_Bay | TechnoClass | (StructType b,bool friendly) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4472 | function (callable) | Find_Exit_Cell | TechnoClass | (TechnoClass const *) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4493 | function (callable) | Refund_Amount | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4538 | function (callable) | Is_Discovered_By_Player | TechnoClass | (HouseClass * player) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4563 | function (callable) | Set_Discovered_By_Player | TechnoClass | (HouseClass * player) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4592 | function (callable) | Clear_Discovered_By_Players | TechnoClass | () | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4612 | function (callable) | Is_Owned_By_Player | TechnoClass | (HouseClass * player) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4621 | function (callable) | Is_Cloaked | TechnoClass | (HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4627 | function (callable) | Is_Cloaked | TechnoClass | (HouseClass const * house) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4633 | function (callable) | Is_Cloaked | TechnoClass | (ObjectClass const * object) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4668 | function (callable) | Anti_Air | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4711 | function (callable) | Anti_Armor | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4761 | function (callable) | Anti_Infantry | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.CPP | 4794 | function (callable) | Legal_Placement | TechnoTypeClass | (CELL pos) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.H | 223 | function (callable) | ~TechnoClass | TechnoClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TECHNO.H | 233 | function (callable) | Techno_Type_Class | TechnoClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TECHNO.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 73 | function (callable) | Validate | TemplateClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 86 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 105 | function (callable) | Read_INI | TemplateClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 145 | function (callable) | Write_INI | TemplateClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 193 | function (callable) | TemplateClass | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 214 | function (callable) | As_Target | TemplateClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 236 | function (callable) | Init | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 265 | function (callable) | Mark | TemplateClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 352 | function (callable) | operator new | TemplateClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 377 | function (callable) | operator delete | TemplateClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.CPP | 402 | function (callable) | TemplateClass | TemplateClass | (TemplateType type,CELL pos) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 56 | function (callable) | ~TemplateClass | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 57 | function (callable) | operator TemplateType | TemplateClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 58 | function (callable) | What_Am_I | TemplateClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 65 | function (callable) | Class_Of | TemplateClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 76 | function (callable) | Draw_It | TemplateClass | (int,int,WindowNumberType) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEMPLATE.H | 93 | function (callable) | INI_Name | TemplateClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEMPLATE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 88 | function (callable) | Validate | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 101 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 120 | function (callable) | ~TerrainClass | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 149 | function (callable) | Take_Damage | TerrainClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 208 | function (callable) | As_Target | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 230 | function (callable) | operator new | TerrainClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 255 | function (callable) | operator delete | TerrainClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 282 | function (callable) | TerrainClass | TerrainClass | (TerrainType type,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 318 | function (callable) | Mark | TerrainClass | (MarkType mark) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 365 | function (callable) | Draw_It | TerrainClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 411 | function (callable) | Init | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 440 | function (callable) | Can_Enter_Cell | TerrainClass | (CELL cell,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 474 | function (callable) | Catch_Fire | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 509 | function (callable) | Fire_Out | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 540 | function (callable) | AI | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 618 | function (callable) | Debug_Dump | TerrainClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 648 | function (callable) | Unlimbo | TerrainClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 673 | function (callable) | Start_To_Crumble | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 699 | function (callable) | Limbo | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 724 | function (callable) | Center_Coord | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 746 | function (callable) | TerrainClass | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 774 | function (callable) | Radar_Icon | TerrainClass | (CELL cell) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 817 | function (callable) | Read_INI | TerrainClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.CPP | 864 | function (callable) | Write_INI | TerrainClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 50 | function (callable) | operator TerrainType | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 60 | function (callable) | What_Am_I | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 72 | function (callable) | Class_Of | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 79 | function (callable) | Render_Coord | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 80 | function (callable) | Sort_Y | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 81 | function (callable) | Target_Coord | TerrainClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 101 | function (callable) | Clicked_As_Target | TerrainClass | (HousesType,int) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TERRAIN.H | 128 | function (callable) | INI_Name | TerrainClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TERRAIN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBLIT.H | 25 | function (callable) | ~TextBlitClass | TextBlitClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 65 | function (callable) | TextButtonClass | TextButtonClass | (unsigned id,char const * text,TextPrintType style,int x,int y,int w,int h,int blackborder) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 97 | function (callable) | TextButtonClass | TextButtonClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 136 | function (callable) | TextButtonClass | TextButtonClass | (unsigned id,int text,TextPrintType style,int x,int y,int w,int h,int blackborder) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 174 | function (callable) | Draw_Me | TextButtonClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 221 | function (callable) | Set_Text | TextButtonClass | (char const * text,bool resize) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 251 | function (callable) | Set_Text | TextButtonClass | (int text,bool resize) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 275 | function (callable) | Draw_Background | TextButtonClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.CPP | 330 | function (callable) | Draw_Text | TextButtonClass | (char const * text) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TEXTBTN.H | 50 | function (callable) | Set_Style | TextButtonClass | (TextPrintType style) | RUST/original-source-reference/TIBERIANDAWN/TEXTBTN.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 115 | function (callable) | Base_Name | ThemeClass | (ThemeType theme) const | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 138 | function (callable) | ThemeClass | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 162 | function (callable) | Full_Name | ThemeClass | (ThemeType theme) const | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 187 | function (callable) | AI | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 228 | function (callable) | Next_Song | ThemeClass | (ThemeType theme) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 280 | function (callable) | Queue_Song | ThemeClass | (ThemeType theme) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 307 | function (callable) | Play_Song | ThemeClass | (ThemeType theme) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 351 | function (callable) | Theme_File_Name | ThemeClass | (ThemeType theme) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 383 | function (callable) | Track_Length | ThemeClass | (ThemeType theme) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 407 | function (callable) | Stop | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 434 | function (callable) | Still_Playing | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 459 | function (callable) | Is_Allowed | ThemeClass | (ThemeType index) const | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 503 | function (callable) | From_Name | ThemeClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.CPP | 550 | function (callable) | Scan | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.H | 69 | function (callable) | Max_Themes | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.H | 75 | function (callable) | What_Is_Playing | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/THEME.H | 77 | function (callable) | Fade_Out | ThemeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/THEME.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TOGGLE.CPP | 62 | function (callable) | ToggleClass | ToggleClass | (unsigned id,int x,int y,int w,int h) | RUST/original-source-reference/TIBERIANDAWN/TOGGLE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TOGGLE.CPP | 86 | function (callable) | Turn_On | ToggleClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TOGGLE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TOGGLE.CPP | 108 | function (callable) | Turn_Off | ToggleClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TOGGLE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TOGGLE.CPP | 137 | function (callable) | Action | ToggleClass | (unsigned flags,KeyNumType & key) | RUST/original-source-reference/TIBERIANDAWN/TOGGLE.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 126 | function (callable) | Validate | TriggerClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 139 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 158 | function (callable) | Event_Need_Object | TriggerClass | (EventType event) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 187 | function (callable) | Event_Need_House | TriggerClass | (EventType event) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 223 | function (callable) | Event_Need_Data | TriggerClass | (EventType event) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 253 | function (callable) | Action_Need_Team | TriggerClass | (TriggerClass::ActionType action) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 280 | function (callable) | TriggerClass | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 308 | function (callable) | ~TriggerClass | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 332 | function (callable) | Init | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 358 | function (callable) | Spring | TriggerClass | (EventType event,ObjectClass * obj) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 554 | function (callable) | Spring | TriggerClass | (EventType event,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 748 | function (callable) | Spring | TriggerClass | (EventType event,HousesType house,long data) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 931 | function (callable) | Remove | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1026 | function (callable) | Read_INI | TriggerClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1112 | function (callable) | Fill_In | TriggerClass | (char * name,char * entry) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1181 | function (callable) | Write_INI | TriggerClass | (char * buffer,bool refresh) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1252 | function (callable) | As_Pointer | TriggerClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1285 | function (callable) | operator new | TriggerClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1310 | function (callable) | operator delete | TriggerClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1334 | function (callable) | Event_From_Name | TriggerClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1367 | function (callable) | Name_From_Event | TriggerClass | (EventType event) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1388 | function (callable) | Action_From_Name | TriggerClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1421 | function (callable) | Name_From_Action | TriggerClass | (ActionType action) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1439 | function (callable) | As_Target | TriggerClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.CPP | 1462 | function (callable) | Do_All_To_Hunt |  | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.H | 139 | function (callable) | INI_Name | TriggerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.H | 161 | function (callable) | Get_Name | TriggerClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TRIGGER.H | 162 | function (callable) | Set_Name | TriggerClass | (char const * buf) | RUST/original-source-reference/TIBERIANDAWN/TRIGGER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 64 | function (callable) | ~TurretClass | TurretClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 83 | function (callable) | TurretClass | TurretClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 105 | function (callable) | TurretClass | TurretClass | (UnitType classid,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 128 | function (callable) | Debug_Dump | TurretClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 156 | function (callable) | Ok_To_Move | TurretClass | (DirType dir) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 187 | function (callable) | AI | TurretClass | (void) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 248 | function (callable) | Fire_At | TurretClass | (TARGET target,int which) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 290 | function (callable) | Can_Fire | TurretClass | (TARGET target,int which) const | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 364 | function (callable) | Fire_Data | TurretClass | (int which) const | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 422 | function (callable) | Fire_Coord | TurretClass | (int which) const | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 519 | function (callable) | Unlimbo | TurretClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TURRET.CPP | 546 | function (callable) | Fire_Direction | TurretClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/TURRET.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTLABEL.CPP | 57 | function (callable) | TextLabelClass | TextLabelClass | (char * txt,int x,int y,int color,TextPrintType style) | RUST/original-source-reference/TIBERIANDAWN/TXTLABEL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTLABEL.CPP | 84 | function (callable) | Draw_Me | TextLabelClass | (int forced) | RUST/original-source-reference/TIBERIANDAWN/TXTLABEL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTLABEL.H | 54 | function (callable) | Set_Text | TextLabelClass | (char * txt) | RUST/original-source-reference/TIBERIANDAWN/TXTLABEL.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 44 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 61 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 62 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 63 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 64 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 164 | label (assembly label candidate) | Buffer_Print |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 270 | label (assembly label candidate) | next_char |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 356 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 370 | label (assembly label candidate) | draw_char |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 375 | label (assembly label candidate) | while_data |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 384 | label (assembly label candidate) | skiplo |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 398 | label (assembly label candidate) | skiphi |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 404 | label (assembly label candidate) | nextrow |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 423 | label (assembly label candidate) | loop_bottom |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 441 | label (assembly label candidate) | force_line_feed |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 450 | label (assembly label candidate) | line_feed |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 476 | label (assembly label candidate) | lfeed |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 485 | label (assembly label candidate) | overflow |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 487 | label (assembly label candidate) | done |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TXTPRNT.ASM | 508 | label (assembly label candidate) | Get_Font_Palette_Ptr |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 252 | function (callable) | AbstractTypeClass | AbstractTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 258 | function (callable) | Set_Name | AbstractTypeClass | (char const * buf) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 378 | function (callable) | Get_Image_Data | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 379 | function (callable) | Get_Radar_Data | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 382 | function (callable) | Display | ObjectTypeClass | (int,int,WindowNumberType,HousesType) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 790 | function (callable) | What_Am_I | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 791 | function (callable) | operator StructType | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 804 | function (callable) | Coord_Fixup | BuildingTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 813 | function (callable) | Get_Buildup_Data | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1037 | function (callable) | What_Am_I | UnitTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1170 | function (callable) | What_Am_I | InfantryTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1173 | function (callable) | As_Reference | InfantryTypeClass | (InfantryType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1178 | function (callable) | Dimensions | InfantryTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1342 | function (callable) | What_Am_I | BulletTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1344 | function (callable) | As_Reference | BulletTypeClass | (BulletType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1345 | function (callable) | Init | BulletTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1348 | function (callable) | Create_And_Place | BulletTypeClass | (CELL,HousesType=HOUSE_NONE) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1349 | function (callable) | Create_One_Of | BulletTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1425 | function (callable) | What_Am_I | TerrainTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1428 | function (callable) | As_Reference | TerrainTypeClass | (TerrainType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1430 | function (callable) | One_Time | TerrainTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1433 | function (callable) | Coord_Fixup | TerrainTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1502 | function (callable) | What_Am_I | TemplateTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1505 | function (callable) | As_Reference | TemplateTypeClass | (TemplateType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1510 | function (callable) | Coord_Fixup | TemplateTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1706 | function (callable) | What_Am_I | AnimTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1708 | function (callable) | As_Reference | AnimTypeClass | (AnimType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1709 | function (callable) | Init | AnimTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1712 | function (callable) | Create_And_Place | AnimTypeClass | (CELL,HousesType=HOUSE_NONE) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1713 | function (callable) | Create_One_Of | AnimTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1797 | function (callable) | As_Reference | AircraftTypeClass | (AircraftType a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1921 | function (callable) | What_Am_I | OverlayTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1924 | function (callable) | As_Reference | OverlayTypeClass | (OverlayType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1929 | function (callable) | Coord_Fixup | OverlayTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1994 | function (callable) | What_Am_I | SmudgeTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 1997 | function (callable) | As_Reference | SmudgeTypeClass | (SmudgeType type) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/TYPE.H | 2005 | function (callable) | Overlap_List | SmudgeTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1406 | function (callable) | UnitTypeClass | UnitTypeClass | (UnitType type,int name,char const * ininame,AnimType exp,unsigned char level,long pre,bool is_goodie,bool is_leader,bool is_eight,bool is_nominal,bool is_transporter,bool is_crushable,bool is_crusher,bool is_harvest,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,bool is_turret_equipped,bool is_twoshooter,bool is_repairable,bool is_buildable,bool is_crew,bool is_radar_equipped,bool is_fire_anim,bool is_lock_turret,bool is_tracked,bool is_gigundo,bool is_chunky,bool is_cloakable,bool is_animating,int ammo,unsigned short strength,int sightrange,int cost,int scenario,int risk,int reward,int ownable,WeaponType primary,WeaponType secondary,ArmorType armor,SpeedType speed,MPHType maxSpeed,unsigned rot,int toffset,MissionType order) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1523 | function (callable) | Occupy_List | UnitTypeClass | (bool) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1552 | function (callable) | From_Name | UnitTypeClass | (char const * name) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1586 | function (callable) | Display | UnitTypeClass | (int x,int y,WindowNumberType window,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1614 | function (callable) | Prep_For_Add | UnitTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1640 | function (callable) | One_Time | UnitTypeClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1726 | function (callable) | Init | UnitTypeClass | (TheaterType theater) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1774 | function (callable) | Create_And_Place | UnitTypeClass | (CELL cell,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1801 | function (callable) | Create_One_Of | UnitTypeClass | (HouseClass * house) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1831 | function (callable) | Who_Can_Build_Me | UnitTypeClass | (bool intheory,bool legal,HousesType house) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1870 | function (callable) | As_Reference | UnitTypeClass | (UnitType type) | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1894 | function (callable) | Dimensions | UnitTypeClass | (int & width,int & height) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1921 | function (callable) | Repair_Cost | UnitTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1942 | function (callable) | Repair_Step | UnitTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UDATA.CPP | 1964 | function (callable) | Max_Pips | UnitTypeClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UDATA.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 120 | function (callable) | Validate | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 133 | macro (function-like macro) | Validate |  | () | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 156 | function (callable) | Recoil_Adjust |  | (DirType dir,int & x,int & y) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 219 | function (callable) | Turret_Adjust |  | (DirType dir,int & x,int & y) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 279 | function (callable) | As_Target | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 302 | function (callable) | Debug_Dump | UnitClass | (MonoClass * mono) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 351 | function (callable) | Sort_Y | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 376 | function (callable) | AI | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 535 | function (callable) | Can_Fire | UnitClass | (TARGET target,int which) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 588 | function (callable) | Receive_Message | UnitClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 803 | function (callable) | Unlimbo | UnitClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 886 | function (callable) | Take_Damage | UnitClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1116 | function (callable) | operator new | UnitClass | (size_t) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1142 | function (callable) | operator delete | UnitClass | (void * ptr) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1168 | function (callable) | ~UnitClass | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1210 | function (callable) | UnitClass | UnitClass | (UnitType classid,HousesType house) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1255 | function (callable) | Active_Click_With | UnitClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1278 | function (callable) | Active_Click_With | UnitClass | (ActionType action,CELL cell) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1281 | function (callable) | Player_Assign_Mission | UnitClass | (MissionType mission,TARGET target,TARGET destination) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1316 | function (callable) | Enter_Idle_Mode | UnitClass | (bool initial) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1414 | function (callable) | Find_LZ | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1448 | function (callable) | Unload_Hovercraft_Process | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1568 | function (callable) | Goto_Clear_Spot | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1698 | function (callable) | Try_To_Deploy | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 1797 | function (callable) | Per_Cell_Process | UnitClass | (bool center) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2071 | function (callable) | Draw_It | UnitClass | (int x,int y,WindowNumberType window) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2323 | function (callable) | Tiberium_Check | UnitClass | (CELL & center,int x,int y) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2347 | function (callable) | Goto_Tiberium | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2423 | function (callable) | Find_Best_Refinery | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2461 | function (callable) | Harvesting | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2519 | function (callable) | Mission_Unload | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2707 | function (callable) | Mission_Harvest | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2855 | function (callable) | Mission_Hunt | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2909 | function (callable) | Mission_Enter | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2952 | function (callable) | Look | UnitClass | (bool incremental) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 2984 | function (callable) | Overlap_List | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3024 | function (callable) | Blocking_Object | UnitClass | (TechnoClass const * techno,CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3133 | function (callable) | Can_Enter_Cell | UnitClass | (CELL cell,FacingType) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3363 | function (callable) | Init | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3391 | function (callable) | Target_Coord | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3419 | function (callable) | Scatter | UnitClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3467 | function (callable) | Stop_Driver | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3525 | function (callable) | Start_Driver | UnitClass | (COORDINATE & headto) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3551 | function (callable) | Limbo | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3581 | function (callable) | Response_Select | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3618 | function (callable) | Response_Move | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3655 | function (callable) | Response_Attack | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3694 | function (callable) | What_Action | UnitClass | (ObjectClass * object) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3749 | function (callable) | What_Action | UnitClass | (CELL cell) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3776 | function (callable) | Can_Player_Move | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3802 | function (callable) | Read_INI | UnitClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3897 | function (callable) | Write_INI | UnitClass | (char * buffer) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3951 | macro (function-like macro) | XYCELL |  | (x,y) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 3952 | function (callable) | Exit_Repair | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4005 | function (callable) | Mission_Guard | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4060 | function (callable) | Mission_Move | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4103 | function (callable) | Desired_Load_Dir | UnitClass | (ObjectClass * passenger,CELL & moveto) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4195 | function (callable) | Mission_Attack | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4221 | function (callable) | Flag_Attach | UnitClass | (HousesType house) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4248 | function (callable) | Flag_Remove | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4275 | function (callable) | Stun | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4301 | function (callable) | Pip_Count | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4328 | function (callable) | APC_Close_Door | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4349 | function (callable) | APC_Open_Door | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4379 | function (callable) | Remap_Table | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4405 | function (callable) | Crew_Type | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.CPP | 4434 | function (callable) | What_Am_I | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.H | 64 | function (callable) | UnitClass | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.H | 66 | function (callable) | operator UnitType | UnitClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UNIT.H | 186 | function (callable) | INI_Name | UnitClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UNIT.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 64 | function (callable) | UnitTrackerClass | UnitTrackerClass | (int unit_count) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 87 | function (callable) | ~UnitTrackerClass | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 108 | function (callable) | Increment_Unit_Total | UnitTrackerClass | (int unit_type) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 128 | function (callable) | Decrement_Unit_Total | UnitTrackerClass | (int unit_type) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 148 | function (callable) | Get_All_Totals | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 168 | function (callable) | Clear_Unit_Total | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 189 | function (callable) | To_Network_Format | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 214 | function (callable) | To_PC_Format | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.CPP | 226 | function (callable) | Get_Unit_Total | UnitTrackerClass | (int unit_type) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/UTRACKER.H | 56 | function (callable) | Get_Unit_Count | UnitTrackerClass | (void) | RUST/original-source-reference/TIBERIANDAWN/UTRACKER.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 77 | function (callable) | VectorClass | VectorClass | (unsigned size,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 114 | function (callable) | ~VectorClass | VectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 136 | function (callable) | VectorClass | VectorClass | (VectorClass<T> const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 161 | function (callable) | operator = | VectorClass | (VectorClass<T> const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 198 | function (callable) | operator == | VectorClass | (VectorClass<T> const & vector) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 231 | function (callable) | ID | VectorClass | (T const * ptr) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 254 | function (callable) | ID | VectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 282 | function (callable) | Clear | VectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 313 | function (callable) | Resize | VectorClass | (unsigned newsize,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 398 | function (callable) | DynamicVectorClass | DynamicVectorClass | (unsigned size,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 426 | function (callable) | Resize | DynamicVectorClass | (unsigned newsize,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 455 | function (callable) | ID | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 482 | function (callable) | Add | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 513 | function (callable) | Add_Head | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 565 | function (callable) | Delete | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 594 | function (callable) | Delete | DynamicVectorClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 636 | function (callable) | BooleanVectorClass | BooleanVectorClass | (unsigned size,unsigned char * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 659 | function (callable) | BooleanVectorClass | BooleanVectorClass | (BooleanVectorClass const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 682 | function (callable) | operator = | BooleanVectorClass | (BooleanVectorClass const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 708 | function (callable) | operator == | BooleanVectorClass | (const BooleanVectorClass & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 730 | function (callable) | Resize | BooleanVectorClass | (unsigned size) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 788 | function (callable) | Clear | BooleanVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 810 | function (callable) | Reset | BooleanVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 833 | function (callable) | Set | BooleanVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.CPP | 863 | function (callable) | Fixup | BooleanVectorClass | (int index) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 63 | function (callable) | operator new |  | (size_t,void * pointer) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 64 | function (callable) | operator new[] |  | (size_t,void * pointer) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 85 | function (callable) | operator [] | VectorClass | (unsigned index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 86 | function (callable) | operator [] | VectorClass | (unsigned index) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 91 | function (callable) | Length | VectorClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 134 | function (callable) | Clear | DynamicVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 137 | function (callable) | Count | DynamicVectorClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 150 | function (callable) | Delete_All | DynamicVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 153 | function (callable) | Set_Growth_Step | DynamicVectorClass | (int step) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 156 | function (callable) | Growth_Step | DynamicVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 158 | function (callable) | ID | DynamicVectorClass | (T const * ptr) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 189 | function (callable) | DynamicVectorArrayClass | DynamicVectorArrayClass | () | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 191 | function (callable) | Set_Active_Context | DynamicVectorArrayClass | (int active) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 196 | function (callable) | Clear_All | DynamicVectorArrayClass | () | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 204 | function (callable) | Clear | DynamicVectorArrayClass | () | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 209 | function (callable) | Count | DynamicVectorArrayClass | () const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 214 | function (callable) | Add | DynamicVectorArrayClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 219 | function (callable) | Add_Head | DynamicVectorArrayClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 224 | function (callable) | Delete | DynamicVectorArrayClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 229 | function (callable) | Delete_All | DynamicVectorArrayClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 239 | function (callable) | Delete_All_Except | DynamicVectorArrayClass | (T const & object,int except) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 252 | function (callable) | Delete | DynamicVectorArrayClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 257 | function (callable) | operator [] | DynamicVectorArrayClass | (unsigned index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 262 | function (callable) | operator [] | DynamicVectorArrayClass | (unsigned index) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 267 | function (callable) | Clear | DynamicVectorArrayClass | (int context) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 272 | function (callable) | Count | DynamicVectorArrayClass | (int context) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 277 | function (callable) | Add | DynamicVectorArrayClass | (int context,T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 282 | function (callable) | Add_Head | DynamicVectorArrayClass | (int context,T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 287 | function (callable) | Delete | DynamicVectorArrayClass | (int context,T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 292 | function (callable) | Delete | DynamicVectorArrayClass | (int context,int index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 297 | function (callable) | Raw_Count | DynamicVectorArrayClass | () const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 302 | function (callable) | Raw | DynamicVectorArrayClass | () | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 307 | function (callable) | Raw | DynamicVectorArrayClass | (int context) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 341 | function (callable) | Length | BooleanVectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 356 | function (callable) | operator [] | BooleanVectorClass | (int index) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 360 | function (callable) | operator [] | BooleanVectorClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 366 | function (callable) | Is_True | BooleanVectorClass | (int index) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 372 | function (callable) | First_False | BooleanVectorClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 386 | function (callable) | First_True | BooleanVectorClass | (void) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 452 | function (callable) | VectorClass | VectorClass | (unsigned size,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 489 | function (callable) | ~VectorClass | VectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 511 | function (callable) | VectorClass | VectorClass | (VectorClass<T> const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 536 | function (callable) | operator = | VectorClass | (VectorClass<T> const & vector) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 573 | function (callable) | operator == | VectorClass | (VectorClass<T> const & vector) const | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 606 | function (callable) | ID | VectorClass | (T const * ptr) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 629 | function (callable) | ID | VectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 657 | function (callable) | Clear | VectorClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 688 | function (callable) | Resize | VectorClass | (unsigned newsize,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 773 | function (callable) | DynamicVectorClass | DynamicVectorClass | (unsigned size,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 801 | function (callable) | Resize | DynamicVectorClass | (unsigned newsize,T const * array) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 830 | function (callable) | ID | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 857 | function (callable) | Add | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 888 | function (callable) | Add_Head | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 940 | function (callable) | Delete | DynamicVectorClass | (T const & object) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VECTOR.H | 969 | function (callable) | Delete | DynamicVectorClass | (int index) | RUST/original-source-reference/TIBERIANDAWN/VECTOR.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VISUDLG.CPP | 39 | function (callable) | Init | VisualControlsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VISUDLG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VISUDLG.CPP | 71 | function (callable) | Process | VisualControlsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VISUDLG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/VISUDLG.H | 69 | function (callable) | VisualControlsClass | VisualControlsClass | (void) | RUST/original-source-reference/TIBERIANDAWN/VISUDLG.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 98 | function (callable) | DPMI_Lock |  | (VOID const *,long const) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 114 | function (callable) | DPMI_Unlock |  | (void const *,long const) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 147 | function (callable) | Alloc |  | (unsigned long bytes_to_alloc,MemoryFlagType flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 319 | function (callable) | Free |  | (void const * pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 412 | function (callable) | Resize_Alloc |  | (void * original_ptr,unsigned long new_size_in_bytes) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 447 | function (callable) | Ram_Free |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 474 | function (callable) | Heap_Size |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ALLOC.CPP | 498 | function (callable) | Total_Ram_Free |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.CPP | 62 | function (callable) | BufferClass | BufferClass | (VOID * buffer,LONG size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.CPP | 85 | function (callable) | BufferClass | BufferClass | (LONG size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.CPP | 106 | function (callable) | BufferClass | BufferClass | (VOID) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.CPP | 123 | function (callable) | ~BufferClass | BufferClass | (VOID) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.H | 103 | function (callable) | Get_Size | BufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/BUFFER.H | 117 | function (callable) | Get_Buffer | BufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 85 | function (callable) | Process_DD_Result |  | (HRESULT result,int display_ok_msg) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 408 | function (callable) | Check_Overlapped_Blit_Capability |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 451 | function (callable) | Set_Video_Mode |  | (HWND hwnd,int w,int h,int bits_per_pixel) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 549 | function (callable) | Reset_Video_Mode |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 584 | function (callable) | Get_Free_Video_Memory |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 623 | function (callable) | Get_Video_Hardware_Capabilities |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 686 | function (callable) | Wait_Vert_Blank |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 719 | function (callable) | Set_DD_Palette |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 777 | function (callable) | Wait_Blit |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 804 | function (callable) | SurfaceMonitorClass | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 829 | function (callable) | Add_DD_Surface | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE new_surface) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 858 | function (callable) | Remove_DD_Surface | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE old_surface) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 885 | function (callable) | Got_Surface_Already | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE test_surface) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 911 | function (callable) | Restore_Surfaces | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 966 | function (callable) | Set_Surface_Focus | SurfaceMonitorClass | (BOOL in_focus) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.CPP | 989 | function (callable) | Release | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 36 | macro (function-like macro) | MAKE_DDHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 368 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDraw,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 398 | macro (function-like macro) | IDirectDraw_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 399 | macro (function-like macro) | IDirectDraw_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 400 | macro (function-like macro) | IDirectDraw_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 401 | macro (function-like macro) | IDirectDraw_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 402 | macro (function-like macro) | IDirectDraw_CreateClipper |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 403 | macro (function-like macro) | IDirectDraw_CreatePalette |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 404 | macro (function-like macro) | IDirectDraw_CreateSurface |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 405 | macro (function-like macro) | IDirectDraw_DuplicateSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 406 | macro (function-like macro) | IDirectDraw_EnumDisplayModes |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 407 | macro (function-like macro) | IDirectDraw_EnumSurfaces |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 408 | macro (function-like macro) | IDirectDraw_FlipToGDISurface |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 409 | macro (function-like macro) | IDirectDraw_GetCaps |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 410 | macro (function-like macro) | IDirectDraw_GetDisplayMode |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 411 | macro (function-like macro) | IDirectDraw_GetFourCCCodes |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 412 | macro (function-like macro) | IDirectDraw_GetGDISurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 413 | macro (function-like macro) | IDirectDraw_GetMonitorFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 414 | macro (function-like macro) | IDirectDraw_GetScanLine |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 415 | macro (function-like macro) | IDirectDraw_GetVerticalBlankStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 416 | macro (function-like macro) | IDirectDraw_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 417 | macro (function-like macro) | IDirectDraw_RestoreDisplayMode |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 418 | macro (function-like macro) | IDirectDraw_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 419 | macro (function-like macro) | IDirectDraw_SetDisplayMode |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 420 | macro (function-like macro) | IDirectDraw_WaitForVerticalBlank |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 428 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDraw2,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 459 | macro (function-like macro) | IDirectDraw2_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 460 | macro (function-like macro) | IDirectDraw2_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 461 | macro (function-like macro) | IDirectDraw2_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 462 | macro (function-like macro) | IDirectDraw2_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 463 | macro (function-like macro) | IDirectDraw2_CreateClipper |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 464 | macro (function-like macro) | IDirectDraw2_CreatePalette |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 465 | macro (function-like macro) | IDirectDraw2_CreateSurface |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 466 | macro (function-like macro) | IDirectDraw2_DuplicateSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 467 | macro (function-like macro) | IDirectDraw2_EnumDisplayModes |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 468 | macro (function-like macro) | IDirectDraw2_EnumSurfaces |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 469 | macro (function-like macro) | IDirectDraw2_FlipToGDISurface |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 470 | macro (function-like macro) | IDirectDraw2_GetCaps |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 471 | macro (function-like macro) | IDirectDraw2_GetDisplayMode |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 472 | macro (function-like macro) | IDirectDraw2_GetFourCCCodes |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 473 | macro (function-like macro) | IDirectDraw2_GetGDISurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 474 | macro (function-like macro) | IDirectDraw2_GetMonitorFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 475 | macro (function-like macro) | IDirectDraw2_GetScanLine |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 476 | macro (function-like macro) | IDirectDraw2_GetVerticalBlankStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 477 | macro (function-like macro) | IDirectDraw2_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 478 | macro (function-like macro) | IDirectDraw2_RestoreDisplayMode |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 479 | macro (function-like macro) | IDirectDraw2_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 480 | macro (function-like macro) | IDirectDraw2_SetDisplayMode |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 481 | macro (function-like macro) | IDirectDraw2_WaitForVerticalBlank |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 482 | macro (function-like macro) | IDirectDraw2_GetAvailableVidMem |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 493 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawPalette,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 507 | macro (function-like macro) | IDirectDrawPalette_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 508 | macro (function-like macro) | IDirectDrawPalette_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 509 | macro (function-like macro) | IDirectDrawPalette_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 510 | macro (function-like macro) | IDirectDrawPalette_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 511 | macro (function-like macro) | IDirectDrawPalette_GetEntries |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 512 | macro (function-like macro) | IDirectDrawPalette_Initialize |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 513 | macro (function-like macro) | IDirectDrawPalette_SetEntries |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 524 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawClipper,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 540 | macro (function-like macro) | IDirectDrawClipper_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 541 | macro (function-like macro) | IDirectDrawClipper_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 542 | macro (function-like macro) | IDirectDrawClipper_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 543 | macro (function-like macro) | IDirectDrawClipper_GetClipList |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 544 | macro (function-like macro) | IDirectDrawClipper_GetHWnd |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 545 | macro (function-like macro) | IDirectDrawClipper_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 546 | macro (function-like macro) | IDirectDrawClipper_IsClipListChanged |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 547 | macro (function-like macro) | IDirectDrawClipper_SetClipList |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 548 | macro (function-like macro) | IDirectDrawClipper_SetHWnd |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 559 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawSurface,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 602 | macro (function-like macro) | IDirectDrawSurface_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 603 | macro (function-like macro) | IDirectDrawSurface_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 604 | macro (function-like macro) | IDirectDrawSurface_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 605 | macro (function-like macro) | IDirectDrawSurface_AddAttachedSurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 606 | macro (function-like macro) | IDirectDrawSurface_AddOverlayDirtyRect |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 607 | macro (function-like macro) | IDirectDrawSurface_Blt |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 608 | macro (function-like macro) | IDirectDrawSurface_BltBatch |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 609 | macro (function-like macro) | IDirectDrawSurface_BltFast |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 610 | macro (function-like macro) | IDirectDrawSurface_DeleteAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 611 | macro (function-like macro) | IDirectDrawSurface_EnumAttachedSurfaces |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 612 | macro (function-like macro) | IDirectDrawSurface_EnumOverlayZOrders |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 613 | macro (function-like macro) | IDirectDrawSurface_Flip |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 614 | macro (function-like macro) | IDirectDrawSurface_GetAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 615 | macro (function-like macro) | IDirectDrawSurface_GetBltStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 616 | macro (function-like macro) | IDirectDrawSurface_GetCaps |  | (p,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 617 | macro (function-like macro) | IDirectDrawSurface_GetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 618 | macro (function-like macro) | IDirectDrawSurface_GetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 619 | macro (function-like macro) | IDirectDrawSurface_GetDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 620 | macro (function-like macro) | IDirectDrawSurface_GetFlipStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 621 | macro (function-like macro) | IDirectDrawSurface_GetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 622 | macro (function-like macro) | IDirectDrawSurface_GetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 623 | macro (function-like macro) | IDirectDrawSurface_GetPixelFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 624 | macro (function-like macro) | IDirectDrawSurface_GetSurfaceDesc |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 625 | macro (function-like macro) | IDirectDrawSurface_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 626 | macro (function-like macro) | IDirectDrawSurface_IsLost |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 627 | macro (function-like macro) | IDirectDrawSurface_Lock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 628 | macro (function-like macro) | IDirectDrawSurface_ReleaseDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 629 | macro (function-like macro) | IDirectDrawSurface_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 630 | macro (function-like macro) | IDirectDrawSurface_SetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 631 | macro (function-like macro) | IDirectDrawSurface_SetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 632 | macro (function-like macro) | IDirectDrawSurface_SetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 633 | macro (function-like macro) | IDirectDrawSurface_SetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 634 | macro (function-like macro) | IDirectDrawSurface_Unlock |  | (p,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 635 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlay |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 636 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlayDisplay |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 637 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlayZOrder |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 645 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawSurface2,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 692 | macro (function-like macro) | IDirectDrawSurface2_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 693 | macro (function-like macro) | IDirectDrawSurface2_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 694 | macro (function-like macro) | IDirectDrawSurface2_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 695 | macro (function-like macro) | IDirectDrawSurface2_AddAttachedSurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 696 | macro (function-like macro) | IDirectDrawSurface2_AddOverlayDirtyRect |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 697 | macro (function-like macro) | IDirectDrawSurface2_Blt |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 698 | macro (function-like macro) | IDirectDrawSurface2_BltBatch |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 699 | macro (function-like macro) | IDirectDrawSurface2_BltFast |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 700 | macro (function-like macro) | IDirectDrawSurface2_DeleteAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 701 | macro (function-like macro) | IDirectDrawSurface2_EnumAttachedSurfaces |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 702 | macro (function-like macro) | IDirectDrawSurface2_EnumOverlayZOrders |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 703 | macro (function-like macro) | IDirectDrawSurface2_Flip |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 704 | macro (function-like macro) | IDirectDrawSurface2_GetAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 705 | macro (function-like macro) | IDirectDrawSurface2_GetBltStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 706 | macro (function-like macro) | IDirectDrawSurface2_GetCaps |  | (p,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 707 | macro (function-like macro) | IDirectDrawSurface2_GetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 708 | macro (function-like macro) | IDirectDrawSurface2_GetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 709 | macro (function-like macro) | IDirectDrawSurface2_GetDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 710 | macro (function-like macro) | IDirectDrawSurface2_GetFlipStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 711 | macro (function-like macro) | IDirectDrawSurface2_GetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 712 | macro (function-like macro) | IDirectDrawSurface2_GetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 713 | macro (function-like macro) | IDirectDrawSurface2_GetPixelFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 714 | macro (function-like macro) | IDirectDrawSurface2_GetSurfaceDesc |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 715 | macro (function-like macro) | IDirectDrawSurface2_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 716 | macro (function-like macro) | IDirectDrawSurface2_IsLost |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 717 | macro (function-like macro) | IDirectDrawSurface2_Lock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 718 | macro (function-like macro) | IDirectDrawSurface2_ReleaseDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 719 | macro (function-like macro) | IDirectDrawSurface2_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 720 | macro (function-like macro) | IDirectDrawSurface2_SetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 721 | macro (function-like macro) | IDirectDrawSurface2_SetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 722 | macro (function-like macro) | IDirectDrawSurface2_SetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 723 | macro (function-like macro) | IDirectDrawSurface2_SetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 724 | macro (function-like macro) | IDirectDrawSurface2_Unlock |  | (p,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 725 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlay |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 726 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlayDisplay |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 727 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlayZOrder |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 728 | macro (function-like macro) | IDirectDrawSurface2_GetDDInterface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 729 | macro (function-like macro) | IDirectDrawSurface2_PageLock |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DDRAW.H | 730 | macro (function-like macro) | IDirectDrawSurface2_PageUnlock |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP | 72 | function (callable) | Fixup_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP | 114 | function (callable) | Dip_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP | 200 | function (callable) | UnDip_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DIPTHONG.CPP | 315 | function (callable) | Extract_String |  | (void const * data,int string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DPLAY.H | 33 | macro (function-like macro) | MAKE_DPHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DPLAY.H | 136 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectPlay,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DRAWRECT.CPP | 60 | function (callable) | Draw_Rect | GraphicViewPortClass | (int x1_pixel,int y1_pixel,int x2_pixel,int y2_pixel,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 34 | macro (function-like macro) | MAKE_DSHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 140 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectSound,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 159 | macro (function-like macro) | IDirectSound_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 160 | macro (function-like macro) | IDirectSound_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 161 | macro (function-like macro) | IDirectSound_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 162 | macro (function-like macro) | IDirectSound_CreateSoundBuffer |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 163 | macro (function-like macro) | IDirectSound_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 164 | macro (function-like macro) | IDirectSound_DuplicateSoundBuffer |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 165 | macro (function-like macro) | IDirectSound_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 166 | macro (function-like macro) | IDirectSound_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 167 | macro (function-like macro) | IDirectSound_GetSpeakerConfig |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 168 | macro (function-like macro) | IDirectSound_SetSpeakerConfig |  | (p,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 169 | macro (function-like macro) | IDirectSound_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 180 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectSoundBuffer,IUnknown) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 209 | macro (function-like macro) | IDirectSoundBuffer_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 210 | macro (function-like macro) | IDirectSoundBuffer_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 211 | macro (function-like macro) | IDirectSoundBuffer_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 212 | macro (function-like macro) | IDirectSoundBuffer_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 213 | macro (function-like macro) | IDirectSoundBuffer_GetCurrentPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 214 | macro (function-like macro) | IDirectSoundBuffer_GetFormat |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 215 | macro (function-like macro) | IDirectSoundBuffer_GetVolume |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 216 | macro (function-like macro) | IDirectSoundBuffer_GetPan |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 217 | macro (function-like macro) | IDirectSoundBuffer_GetFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 218 | macro (function-like macro) | IDirectSoundBuffer_GetStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 219 | macro (function-like macro) | IDirectSoundBuffer_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 220 | macro (function-like macro) | IDirectSoundBuffer_Lock |  | (p,a,b,c,d,e,f,g) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 221 | macro (function-like macro) | IDirectSoundBuffer_Play |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 222 | macro (function-like macro) | IDirectSoundBuffer_SetCurrentPosition |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 223 | macro (function-like macro) | IDirectSoundBuffer_SetFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 224 | macro (function-like macro) | IDirectSoundBuffer_SetVolume |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 225 | macro (function-like macro) | IDirectSoundBuffer_SetPan |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 226 | macro (function-like macro) | IDirectSoundBuffer_SetFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 227 | macro (function-like macro) | IDirectSoundBuffer_Stop |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 228 | macro (function-like macro) | IDirectSoundBuffer_Unlock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DSOUND.H | 229 | macro (function-like macro) | IDirectSoundBuffer_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 37 | function (callable) | IconCacheClass | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 46 | function (callable) | ~IconCacheClass | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 62 | function (callable) | Invalidate_Cached_Icons |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 63 | function (callable) | Restore_Cached_Icons |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 64 | function (callable) | Register_Icon_Set |  | (void * icon_data,BOOL pre_cache) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 69 | function (callable) | Clear_Icon_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 70 | function (callable) | Cache_Copy_Icon |  | (void const * icon_ptr,void *,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 71 | function (callable) | Is_Icon_Cached |  | (void const * icon_data,int icon) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 72 | function (callable) | Get_Icon_Index |  | (void * icon_ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 73 | function (callable) | Get_Free_Index |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 74 | function (callable) | Cache_New_Icon |  | (int icon_index,void * icon_ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 75 | function (callable) | Get_Free_Cache_Slot |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 77 | function (callable) | Draw_It | IconCacheClass | (LPDIRECTDRAWSURFACE dest_surface,int x_pixel,int y_pixel,int window_left,int window_top,int window_width,int window_height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 85 | function (callable) | Set_Font_Palette_Range |  | (void const * palette,INT start_idx,INT end_idx) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 111 | function (callable) | Buffer_Draw_Line |  | (void * this_object,int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 1060 | function (callable) | Buffer_Fill_Rect |  | (void * thisptr,int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 1307 | function (callable) | Buffer_Clear |  | (void * this_object,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 1394 | function (callable) | Linear_Blit_To_Linear |  | (void * this_object,void * dest,int x_pixel,int y_pixel,int dest_x0,int dest_y0,int pixel_width,int pixel_height,BOOL trans) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 1876 | function (callable) | Linear_Scale_To_Linear |  | (void * this_object,void * dest,int src_x,int src_y,int dst_x,int dst_y,int src_width,int src_height,int dst_width,int dst_height,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 2687 | function (callable) | Init_Stamps |  | (unsigned int icondata) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 2759 | function (callable) | Buffer_Draw_Stamp |  | (void const * this_object,void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 2962 | function (callable) | Buffer_Draw_Stamp_Clip |  | (void const * this_object,void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap,int min_x,int min_y,int max_x,int max_y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 3292 | function (callable) | Buffer_Remap |  | (void * this_object,int sx,int sy,int width,int height,void * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 3525 | function (callable) | Apply_XOR_Delta |  | (char * target,char * delta) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 3661 | function (callable) | Apply_XOR_Delta_To_Page_Or_Viewport |  | (void * target,void * delta,int width,int nextrow,int copy) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 3727 | function (callable) | XOR_Delta_Buffer |  | (int nextrow) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 3878 | function (callable) | Copy_Delta_Buffer |  | (int nextrow) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4071 | function (callable) | Build_Fading_Table |  | (void const * palette,void const * dest,long int color,long int frac) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4343 | function (callable) | Set_Palette_Range |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4390 | function (callable) | Bump_Color |  | (void * pal,int color,int desired) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4505 | function (callable) | Buffer_Put_Pixel |  | (void * this_object,int x_pixel,int y_pixel,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4614 | function (callable) | Clip_Rect |  | (int * x,int * y,int * w,int * h,int width,int height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4767 | function (callable) | Confine_Rect |  | (int * x,int * y,int w,int h,int width,int height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 4881 | function (callable) | LCW_Uncompress |  | (void * source,void * dest,unsigned long length_) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 5191 | function (callable) | Buffer_To_Page |  | (int x_pixel,int y_pixel,int pixel_width,int pixel_height,void * src,void * dest) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/DrawMisc.cpp | 5464 | function (callable) | Buffer_Get_Pixel |  | (void * this_object,int x_pixel,int y_pixel) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/FACINGFF.ASM | 42 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/FONT.CPP | 60 | function (callable) | Char_Pixel_Width |  | (char chr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/FONT.CPP | 87 | function (callable) | String_Pixel_Width |  | (char const * string) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/FONT.CPP | 126 | function (callable) | Get_Next_Text_Print_XY |  | (GraphicViewPortClass & gp,unsigned long offset,INT * x,INT * y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 81 | function (callable) | GraphicViewPortClass | GraphicViewPortClass | (GraphicBufferClass * gbuffer,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 98 | function (callable) | GraphicViewPortClass | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 112 | function (callable) | ~GraphicViewPortClass | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 140 | function (callable) | Attach | GraphicViewPortClass | (GraphicBufferClass * gbuffer,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 208 | function (callable) | Change | GraphicViewPortClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 238 | function (callable) | DD_Init | GraphicBufferClass | (GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 282 | function (callable) | Attach_DD_Surface | GraphicBufferClass | (GraphicBufferClass * attach_buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 304 | function (callable) | Init | GraphicBufferClass | (int w,int h,void * buffer,long size,GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 353 | function (callable) | Un_Init | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 387 | function (callable) | GraphicBufferClass | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 410 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 426 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 446 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 461 | function (callable) | ~GraphicBufferClass | GraphicBufferClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 484 | function (callable) | Set_Logic_Page |  | (GraphicViewPortClass * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 503 | function (callable) | Set_Logic_Page |  | (GraphicViewPortClass & ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 528 | function (callable) | Lock | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 616 | function (callable) | Unlock | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.CPP | 669 | function (callable) | DD_Linear_Blit_To_Linear | GraphicViewPortClass | (GraphicViewPortClass & dest,int source_x,int source_y,int dest_x,int dest_y,int width,int height,BOOL mask) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 152 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 163 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 164 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 310 | function (callable) | Texture_Fill_Rect | GraphicViewPortClass | (int xpos,int ypos,int width,int height,void const * shape_pointer,int source_width,int source_height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 393 | function (callable) | Get_LockCount | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 414 | function (callable) | Get_IsDirectDraw | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 435 | function (callable) | Get_DD_Surface | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 458 | function (callable) | Lock | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 484 | function (callable) | Unlock | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 505 | function (callable) | Get_Offset | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 520 | function (callable) | Get_Height | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 535 | function (callable) | Get_Width | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 551 | function (callable) | Get_XAdd | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 565 | function (callable) | Get_XPos | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 583 | function (callable) | Get_YPos | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 598 | function (callable) | Get_Graphic_Buffer | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 615 | function (callable) | Size_Of_Region | GraphicViewPortClass | (int w,int h) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 633 | function (callable) | Put_Pixel | GraphicViewPortClass | (int x,int y,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 656 | function (callable) | Get_Pixel | GraphicViewPortClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 680 | function (callable) | Clear | GraphicViewPortClass | (unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 701 | function (callable) | To_Buffer | GraphicViewPortClass | (int x,int y,int w,int h,void * buff,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 723 | function (callable) | To_Buffer | GraphicViewPortClass | (int x,int y,int w,int h,BufferClass * buff) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 745 | function (callable) | To_Buffer | GraphicViewPortClass | (BufferClass * buff) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 767 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,int x_pixel,int y_pixel,int dx_pixel,int dy_pixel,int pixel_width,int pixel_height,BOOL trans) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 804 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,int dx,int dy,BOOL trans) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 842 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,BOOL trans) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 883 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,int src_x,int src_y,int dst_x,int dst_y,int src_w,int src_h,int dst_w,int dst_h,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 909 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,int src_x,int src_y,int dst_x,int dst_y,int src_w,int src_h,int dst_w,int dst_h,char * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 935 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 960 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,char * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 984 | function (callable) | Print | GraphicViewPortClass | (char const * str,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1007 | function (callable) | Print | GraphicViewPortClass | (int num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1030 | function (callable) | Print | GraphicViewPortClass | (short num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1053 | function (callable) | Print | GraphicViewPortClass | (long num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1076 | function (callable) | Draw_Stamp | GraphicViewPortClass | (void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1100 | function (callable) | Draw_Stamp | GraphicViewPortClass | (void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap,int clip_window) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1148 | function (callable) | Draw_Line | GraphicViewPortClass | (int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1168 | function (callable) | Fill_Rect | GraphicViewPortClass | (int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1232 | function (callable) | Remap | GraphicViewPortClass | (int sx,int sy,int width,int height,VOID * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1241 | function (callable) | Fill_Quad | GraphicViewPortClass | (VOID * span_buff,int x0,int y0,int x1,int y1,int x2,int y2,int x3,int y3,int color) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1260 | function (callable) | Remap | GraphicViewPortClass | (VOID * remap) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1268 | function (callable) | Get_Pitch | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1290 | function (callable) | Buffer_To_Page |  | (int x,int y,int w,int h,void * Buffer,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1315 | function (callable) | To_Page | BufferClass | (int w,int h,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1338 | function (callable) | To_Page | BufferClass | (GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GBUFFER.H | 1361 | function (callable) | To_Page | BufferClass | (int x,int y,int w,int h,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 69 | function (callable) | Get_Shape_Size |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 104 | function (callable) | Get_Shape_Uncomp_Size |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 137 | function (callable) | Get_Shape_Data |  | (VOID const * shape,WORD data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 169 | function (callable) | Extract_Shape_Count |  | (VOID const * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 196 | function (callable) | Extract_Shape |  | (VOID const * buffer,int shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 233 | function (callable) | Get_Shape_Width |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 259 | function (callable) | Get_Shape_Height |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 290 | function (callable) | Set_Shape_Height |  | (VOID const * shape,WORD newheight) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 319 | function (callable) | Restore_Shape_Height |  | (VOID * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/GETSHAPE.CPP | 349 | function (callable) | Get_Shape_Original_Height |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONCACH.H | 140 | function (callable) | Get_Is_Cached | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 111 | function (callable) | Load_Icon_Set |  | (char const * filename,void * iconsetptr,long buffsize) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 246 | function (callable) | Free_Icon_Set |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 259 | function (callable) | Get_Icon_Set_Size |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 272 | function (callable) | Get_Icon_Set_Width |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 285 | function (callable) | Get_Icon_Set_Height |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 298 | function (callable) | Get_Icon_Set_Icondata |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 307 | function (callable) | Get_Icon_Set_Trans |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 320 | function (callable) | Get_Icon_Set_Count |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/ICONSET.CPP | 333 | function (callable) | Get_Icon_Set_Map |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | 71 | function (callable) | Open_Iff_File |  | (char const * filename) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | 125 | function (callable) | Close_Iff_File |  | (int fh) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | 144 | function (callable) | Get_Iff_Chunk_Size |  | (int fh,long id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | 212 | function (callable) | Read_Iff_Chunk |  | (int fh,long id,void * buffer,unsigned long maxsize) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.CPP | 272 | function (callable) | Write_Iff_Chunk |  | (int file,long id,void * buffer,long length) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.H | 55 | macro (function-like macro) | MAKE_ID |  | (a,b,c,d) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.H | 56 | macro (function-like macro) | IFFize_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IFF.H | 57 | macro (function-like macro) | IFFize_LONG |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IRANDOM.CPP | 51 | function (callable) | IRandom |  | (int minval,int maxval) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IRANDOM.CPP | 70 | function (callable) | Random |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/IRANDOM.CPP | 91 | function (callable) | Get_Random_Mask |  | (int maxval) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 59 | function (callable) | WWKeyboardClass | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 123 | function (callable) | Buff_Get | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 140 | function (callable) | Is_Mouse_Key | WWKeyboardClass | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 159 | function (callable) | Check | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 180 | function (callable) | Get | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 207 | function (callable) | Put | WWKeyboardClass | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 234 | function (callable) | Put_Key_Message | WWKeyboardClass | (UINT vk_key,BOOL release,BOOL dbl) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 279 | function (callable) | Clear | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 284 | function (callable) | To_ASCII | WWKeyboardClass | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 291 | function (callable) | Down | WWKeyboardClass | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 296 | function (callable) | Split | WWKeyboardClass | (int & key,int & shift,int & ctrl,int & alt,int & rls,int & dbl) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 313 | function (callable) | Message_Handler | WWKeyboardClass | (HWND,UINT message,UINT wParam,LONG lParam) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 397 | function (callable) | Message_Loop |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 434 | function (callable) | Check_Key |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 440 | function (callable) | Clear_KeyBuffer |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 446 | function (callable) | Check_Key_Num |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 463 | function (callable) | Get_Key_Num |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 479 | function (callable) | KN_To_KA |  | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 502 | function (callable) | KN_To_VK |  | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 517 | function (callable) | Key_Down |  | (int key) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/KEYBOARD.CPP | 523 | function (callable) | Get_Key |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 74 | function (callable) | Load_Data |  | (char const * name,void * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 104 | function (callable) | Write_Data |  | (char const * name,void * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 136 | function (callable) | Load_Alloc_Data |  | (char const * name,MemoryFlagType flags) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 169 | function (callable) | Load_Uncompress |  | (char const * file,BufferClass & uncomp_buff,BufferClass & dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 244 | function (callable) | Load_Uncompress |  | (char const * file,BuffType uncomp_buff,BuffType dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOAD.CPP | 330 | function (callable) | Uncompress_Data |  | (void const * src,void * dst) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOADFONT.CPP | 82 | function (callable) | Load_Font |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/LOADPAL.CPP | 73 | function (callable) | Load_Palette |  | (char * palette_file_name,void * palette_pointer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 132 | function (callable) | Mem_Init |  | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 189 | function (callable) | Mem_Alloc |  | (void * poolptr,long lsize,unsigned long id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 311 | function (callable) | Mem_Free |  | (void * poolptr,void * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 368 | function (callable) | Mem_Reference |  | (void * node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 399 | function (callable) | Mem_Lock_Block |  | (void * node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 427 | function (callable) | Mem_In_Use |  | (void * node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 462 | function (callable) | Mem_Find |  | (void * poolptr,unsigned long id) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 507 | function (callable) | Mem_Get_ID |  | (void * node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 540 | function (callable) | Mem_Find_Oldest |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 614 | function (callable) | Mem_Free_Oldest |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 639 | function (callable) | Mem_Pool_Size |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 673 | function (callable) | Mem_Avail |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 703 | function (callable) | Mem_Largest_Avail |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 760 | function (callable) | Mem_Cleanup |  | (void * poolptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 883 | function (callable) | MemNode_Unlink |  | (MemPool_Type * pool,int freechain,MemChain_Type * node) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEM.CPP | 947 | function (callable) | MemNode_Insert |  | (MemPool_Type * pool,int freechain,MemChain_Type * node,unsigned int size,unsigned long id,int merge) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEMFLAG.H | 75 | function (callable) | operator new |  | (size_t size,MemoryFlagType flag) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEMFLAG.H | 79 | function (callable) | operator new[]  |  | (size_t size,MemoryFlagType flag) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MEMFLAG.H | 95 | function (callable) | Add_Long_To_Pointer |  | (void const * ptr,long size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 35 | function (callable) | Get_Modem_Name | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 37 | function (callable) | Get_Modem_Device_Name | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 39 | function (callable) | Get_Modem_Error_Correction_Enable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 41 | function (callable) | Get_Modem_Error_Correction_Disable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 43 | function (callable) | Get_Modem_Compression_Enable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 45 | function (callable) | Get_Modem_Compression_Disable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 47 | function (callable) | Get_Modem_Hardware_Flow_Control | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MODEMREG.H | 49 | function (callable) | Get_Modem_No_Flow_Control | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 101 | function (callable) | Enable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 102 | function (callable) | Disable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 103 | function (callable) | Is_Enabled | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 104 | function (callable) | Get_Current | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 107 | function (callable) | Set_Default_Attribute | MonoClass | (char attrib) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 117 | function (callable) | Get_X | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 118 | function (callable) | Get_Y | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 141 | function (callable) | Offset | MonoClass | (int x=0,int y=0) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MONO.H | 143 | function (callable) | Store_Cell | MonoClass | (CellType & cell,int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP | 45 | macro (function-like macro) | SCALE |  | (a,b,c) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP | 76 | function (callable) | Morph_Palette |  | (void * src_pal,void * dst_pal,unsigned int delay,void (* callback)(void)) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MORPHPAL.CPP | 130 | function (callable) | Palette_To_Palette |  | (void * src_palette,void * dst_palette,unsigned long current_time,unsigned long delay) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 51 | function (callable) | WWMouseClass | WWMouseClass | (GraphicViewPortClass * scr,int mouse_max_width,int mouse_max_height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 102 | function (callable) | ~WWMouseClass | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 122 | function (callable) | Block_Mouse |  | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 130 | function (callable) | Unblock_Mouse |  | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 139 | function (callable) | Block_Mouse | WWMouseClass | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 147 | function (callable) | Unblock_Mouse | WWMouseClass | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 158 | function (callable) | Set_Cursor_Clip | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 176 | function (callable) | Clear_Cursor_Clip | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 185 | function (callable) | Process_Mouse | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 261 | function (callable) | Set_Cursor | WWMouseClass | (int xhotspot,int yhotspot,void * cursor) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 308 | function (callable) | Low_Hide_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 325 | function (callable) | Hide_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 333 | function (callable) | Low_Show_Mouse | WWMouseClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 378 | function (callable) | Show_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 388 | function (callable) | Conditional_Hide_Mouse | WWMouseClass | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 450 | function (callable) | Conditional_Show_Mouse | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 475 | function (callable) | Draw_Mouse | WWMouseClass | (GraphicViewPortClass * scr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 534 | function (callable) | Erase_Mouse | WWMouseClass | (GraphicViewPortClass * scr,int forced) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 590 | function (callable) | Get_Mouse_State | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 604 | function (callable) | Get_Mouse_X | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 622 | function (callable) | Get_Mouse_Y | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 640 | function (callable) | Get_Mouse_XY | WWMouseClass | (int & x,int & y) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 651 | function (callable) | Process_Mouse |  | (UINT event_id,UINT res1,DWORD user,DWORD res2,DWORD res3) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 663 | function (callable) | Hide_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 669 | function (callable) | Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 675 | function (callable) | Conditional_Hide_Mouse |  | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 681 | function (callable) | Conditional_Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 687 | function (callable) | Get_Mouse_State |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 693 | function (callable) | Set_Mouse_Cursor |  | (int hotx,int hoty,void * cursor) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 703 | function (callable) | Get_Mouse_X |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/MOUSEWW.CPP | 712 | function (callable) | Get_Mouse_Y |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/NEWDEL.CPP | 60 | function (callable) | operator new |  | (size_t size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/NEWDEL.CPP | 80 | function (callable) | operator new[] |  | (size_t size) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/NEWDEL.CPP | 100 | function (callable) | operator delete |  | (void * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/NEWDEL.CPP | 119 | function (callable) | operator delete[] |  | (void * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | 86 | function (callable) | Set_Palette |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | 118 | function (callable) | Set_Palette_Color |  | (void * palette,int color,void * data) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | 157 | function (callable) | Fade_Palette_To |  | (void * palette1,unsigned int delay,void (* callback)()) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | 239 | function (callable) | Determine_Bump_Rate |  | (void * palette,int delay,short * ticks,short * rate) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PALETTE.CPP | 308 | function (callable) | Bump_Palette |  | (void * palette1,unsigned int step) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PLAYCD.H | 200 | function (callable) | Get_Number_Of_Drives | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PLAYCD.H | 220 | function (callable) | Get_Next_CD_Drive | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/PLAYCD.H | 247 | function (callable) | Get_First_CD_Drive | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/RAWFILE.H | 207 | function (callable) | File_Name | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/RAWFILE.H | 228 | function (callable) | RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/RAWFILE.H | 249 | function (callable) | Is_Open | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REGIONSZ.CPP | 54 | function (callable) | Size_Of_Region |  | (int width,int height) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 35 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 36 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 37 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 39 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 40 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/REMAP.ASM | 43 | label (assembly label candidate) | CODESEG |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SET_FONT.CPP | 59 | function (callable) | Set_Font |  | (void const * fontptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 50 | function (callable) | operator \| |  | (MakeShapeFlags_Type a,MakeShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 53 | function (callable) | operator & |  | (MakeShapeFlags_Type a,MakeShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 56 | function (callable) | operator ~ |  | (MakeShapeFlags_Type a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 86 | function (callable) | operator \| |  | (ShapeFlags_Type a,ShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 89 | function (callable) | operator & |  | (ShapeFlags_Type a,ShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/SHAPE.H | 92 | function (callable) | operator ~ |  | (ShapeFlags_Type a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 63 | function (callable) | TimerClass | TimerClass | (BaseTimerEnum timer,BOOL on) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 91 | function (callable) | Get_Ticks | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 124 | function (callable) | Time | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 149 | function (callable) | Stop | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 170 | function (callable) | Start | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.CPP | 194 | function (callable) | Set | TimerClass | (long value,BOOL start) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 71 | function (callable) | ~TimerClass | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 91 | function (callable) | Reset | TimerClass | (BOOL start) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 105 | function (callable) | ~CountDownTimerClass | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 118 | function (callable) | Stop | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 124 | function (callable) | Start | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMER.H | 130 | function (callable) | Reset | CountDownTimerClass | (BOOL start) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP | 63 | function (callable) | CountDownTimerClass | CountDownTimerClass | (BaseTimerEnum timer,long set,int on) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP | 69 | function (callable) | CountDownTimerClass | CountDownTimerClass | (BaseTimerEnum timer,int on) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP | 90 | function (callable) | Time | CountDownTimerClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERDWN.CPP | 115 | function (callable) | Set | CountDownTimerClass | (long value,BOOL start) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 98 | function (callable) | WinTimerClass | WinTimerClass | (UINT freq,BOOL partial) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 150 | function (callable) | ~WinTimerClass | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 186 | function (callable) | Timer_Callback |  | (UINT,UINT,DWORD,DWORD,DWORD) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 224 | function (callable) | Update_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 289 | function (callable) | Get_System_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TIMERINI.CPP | 309 | function (callable) | Get_User_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 45 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 73 | label (assembly label candidate) | Buffer_To_Buffer |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 149 | label (assembly label candidate) | scr_left_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 157 | label (assembly label candidate) | scr_bottom_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 162 | label (assembly label candidate) | scr_right_ok |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 168 | label (assembly label candidate) | do_blit |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 213 | label (assembly label candidate) | IF |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 216 | label (assembly label candidate) | ENDIF |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 224 | label (assembly label candidate) | forward_loop_dword |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 243 | label (assembly label candidate) | forward_loop_bytes |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 253 | label (assembly label candidate) | IF |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 255 | label (assembly label candidate) | forward_Blit_trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 264 | label (assembly label candidate) | forward_loop_trans |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 267 | label (assembly label candidate) | forward_trans_line |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 274 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 278 | label (assembly label candidate) | transp_reference |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 286 | label (assembly label candidate) | ENDIF |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/TOBUFF.ASM | 288 | label (assembly label candidate) | real_out |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 309 | function (callable) | Set_Modem_Dial_Type | WinNullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 310 | function (callable) | Get_Modem_Status | WinNullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 311 | function (callable) | Set_Serial_DTR | WinNullModemClass | (BOOL) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 312 | function (callable) | Get_Modem_Result | WinNullModemClass | (int,char *,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 313 | function (callable) | Dial_Modem | WinNullModemClass | (char *) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 314 | function (callable) | Send_Command_To_Modem | WinNullModemClass | (char *,char,char *,int,int,int) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 315 | function (callable) | Set_Echo_Function | WinNullModemClass | (void (*)(char)) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINCOMM.H | 316 | function (callable) | Set_Abort_Function | WinNullModemClass | (int (*)(void)) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 120 | function (callable) | Standard_More_Prompt |  | (char const * prompt,int space,int fcolor,int bcolor) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 173 | function (callable) | Set_More_Prompt |  | (char const * prompt,int space,int fcolor,int bcolor) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 206 | function (callable) | Set_More_On |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 228 | function (callable) | Set_More_Off |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 249 | function (callable) | Change_Window |  | (int windnum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 291 | function (callable) | Change_New_Window |  | (int windnum) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 317 | function (callable) | New_Window |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 349 | function (callable) | Window_Int_Print |  | (int num) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 411 | function (callable) | Window_Print |  | (char const string[],...) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 703 | function (callable) | Scroll_Window |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 748 | function (callable) | Flush_Line |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 891 | function (callable) | In_Char |  | (char * str) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINDOWS.CPP | 954 | function (callable) | Fetch_Char |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINHIDE.CPP | 60 | function (callable) | Window_Hide_Mouse |  | (int window) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WINHIDE.CPP | 87 | function (callable) | Window_Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP | 60 | function (callable) | Write_PCX_File |  | (char * name,GraphicViewPortClass & pic,unsigned char * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP | 124 | macro (function-like macro) | WRITE_CHAR |  | (x) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WRITEPCX.CPP | 132 | function (callable) | Write_Pcx_ScanLine |  | (int file_handle,int scansize,unsigned char * ptr) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 168 | function (callable) | Open_Animation |  | (char const * file_name,char * user_buffer,long user_buffer_size,WSAOpenType user_flags,unsigned char * palette) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 427 | function (callable) | Close_Animation |  | (void * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 461 | function (callable) | Animate_Frame |  | (void * handle,GraphicViewPortClass & view,int frame_number,int x_pixel,int y_pixel,WSAType flags_and_prio,void * magic_cols,void * magic) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 659 | function (callable) | Animate_Frame |  | (void * handle,GraphicViewPortClass & view,int frame_number,int x_pixel,int y_pixel,WSAType flags_and_prio,void * magic_cols,void * magic) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 851 | function (callable) | Get_Animation_Frame_Count |  | (void * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 876 | function (callable) | Get_Animation_X |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 900 | function (callable) | Get_Animation_Y |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 924 | function (callable) | Get_Animation_Width |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 945 | function (callable) | Get_Animation_Height |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 968 | function (callable) | Get_Animation_Palette |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 993 | function (callable) | Get_Animation_Size |  | (void const * handle) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 1020 | function (callable) | Get_Resident_Frame_Offset |  | (char * file_buffer,int frame) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 1056 | function (callable) | Get_File_Frame_Offset |  | (int file_handle,int frame,int palette_adjust) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.CPP | 1083 | function (callable) | Apply_Delta |  | (SysAnimHeaderType * sys_header,int curr_frame,char * dest_ptr,int dest_w) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.H | 118 | function (callable) | Open_Animation |  | (char * file_name,WSAOpenType user_flags,unsigned char * palette=NULL) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WSA.H | 141 | function (callable) | Open_Animation |  | (char * file_name,BufferClass & buffer,WSAOpenType user_flags,unsigned char * palette=NULL) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWFILE.H | 50 | function (callable) | ~FileClass | FileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWFILE.H | 65 | function (callable) | operator char const* | FileClass | () |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 62 | macro (function-like macro) | GET_SIZE |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 70 | function (callable) | ABS |  | (T a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 81 | function (callable) | MIN |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 93 | function (callable) | MAX |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 99 | macro (function-like macro) | LOW_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 102 | macro (function-like macro) | HIGH_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 105 | macro (function-like macro) | MAKE_LONG |  | (a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 115 | macro (function-like macro) | AssembleTo |  | (dest,fmt) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 143 | function (callable) | Min |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 148 | function (callable) | Max |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 153 | function (callable) | Abs |  | (T a) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 158 | function (callable) | minimize |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 164 | function (callable) | maximize |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 174 | macro (function-like macro) | Bit_Flags_On |  | (a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 175 | macro (function-like macro) | Bit_Flags_Off |  | (a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 176 | macro (function-like macro) | Bit_Flags_Value |  | (a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 177 | macro (function-like macro) | Bit_Flags_Flip |  | (a,b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 181 | function (callable) | BitFlagsOn |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 186 | function (callable) | BitFlagsOff |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 191 | function (callable) | BitFlagsValue |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/WWSTD.H | 196 | function (callable) | BitFlagsFlip |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 63 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 64 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 86 | label (assembly label candidate) | Apply_XOR_Delta |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 99 | label (assembly label candidate) | top_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 113 | label (assembly label candidate) | dump_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 126 | label (assembly label candidate) | short_run |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 130 | label (assembly label candidate) | do_run |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 134 | label (assembly label candidate) | run_loop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 146 | label (assembly label candidate) | check_others |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 158 | label (assembly label candidate) | do_skip |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 163 | label (assembly label candidate) | not_long_skip |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 181 | label (assembly label candidate) | long_dump |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 185 | label (assembly label candidate) | stop |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 219 | label (assembly label candidate) | Apply_XOR_Delta_To_Page_Or_Viewport |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 251 | label (assembly label candidate) | xorfunct |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 253 | label (assembly label candidate) | didcopy |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 281 | label (assembly label candidate) | XOR_Delta_Buffer |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 283 | label (assembly label candidate) | top_loop2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 297 | label (assembly label candidate) | dump_loop2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 309 | label (assembly label candidate) | end_col1 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 319 | label (assembly label candidate) | short_run2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 323 | label (assembly label candidate) | do_run2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 327 | label (assembly label candidate) | run_loop2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 338 | label (assembly label candidate) | end_col2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 349 | label (assembly label candidate) | check_others2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 361 | label (assembly label candidate) | do_skip2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 364 | label (assembly label candidate) | recheck2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 371 | label (assembly label candidate) | end_col3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 376 | label (assembly label candidate) | not_long_skip2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 394 | label (assembly label candidate) | long_dump2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 398 | label (assembly label candidate) | stop2 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 425 | label (assembly label candidate) | Copy_Delta_Buffer |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 429 | label (assembly label candidate) | top_loop3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 443 | label (assembly label candidate) | dump_loop3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 457 | label (assembly label candidate) | end_col1_3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 467 | label (assembly label candidate) | short_run3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 471 | label (assembly label candidate) | do_run3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 475 | label (assembly label candidate) | run_loop3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 486 | label (assembly label candidate) | end_col2_3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 497 | label (assembly label candidate) | check_others3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 509 | label (assembly label candidate) | do_skip3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 512 | label (assembly label candidate) | recheck3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 519 | label (assembly label candidate) | end_col3_3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 524 | label (assembly label candidate) | not_long_skip3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 542 | label (assembly label candidate) | long_dump3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/XORDELTA.ASM | 546 | label (assembly label candidate) | stop3 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 53 | macro (function-like macro) | FILEOPEN |  | (f,m) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 55 | macro (function-like macro) | FILECLOSE |  | (fd) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 56 | macro (function-like macro) | FILEREAD |  | (f,b,n) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 57 | macro (function-like macro) | FILEWRITE |  | (f,b,n) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 58 | macro (function-like macro) | FILESEEK |  | (f,b,n) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 59 | macro (function-like macro) | FILEDELETE |  | (f) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WIN32LIB/_FILE.H | 60 | macro (function-like macro) | CHANGEDIR |  | (p) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 36 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 37 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 38 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 102 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 121 | label (assembly label candidate) | SEQUENCER |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 122 | label (assembly label candidate) | MAP_MASK |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 200 | label (assembly label candidate) | ifdef |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 239 | label (assembly label candidate) | endif |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINASM.ASM | 312 | label (assembly label candidate) | set_dac_col |  |  |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 43 | function (callable) | output |  | (short,short) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 70 | function (callable) | Extract_Shape_Count |  | (VOID const * buffer) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 97 | function (callable) | Extract_Shape |  | (VOID const * buffer,int shape) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 141 | function (callable) | Focus_Loss |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 154 | function (callable) | Focus_Restore |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 181 | function (callable) | Check_For_Focus_Loss |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 232 | function (callable) | Windows_Procedure |  | (HWND hwnd,UINT message,UINT wParam,LONG lParam) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 389 | function (callable) | Create_Main_Window |  | (HANDLE instance,int command_show,int width,int height) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 448 | function (callable) | Window_Dialog_Box |  | (HANDLE hinst,LPCTSTR lpszTemplate,HWND hwndOwner,DLGPROC dlgprc) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 510 | function (callable) | Colour_Debug |  | (int call_number) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 533 | function (callable) | Any_Locked |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 565 | function (callable) | CCDebugString |  | (char * string) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 688 | function (callable) | Flag_To_Set_Palette |  | (unsigned char * palette,long numbytes,unsigned long slowpal) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 698 | function (callable) | Check_VQ_Palette_Set |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 710 | function (callable) | SetPalette |  | (unsigned char * palette,long,unsigned long) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 739 | function (callable) | Memory_Error_Handler |  | (void) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 784 | function (callable) | Load_Title_Screen |  | (char * name,GraphicViewPortClass * video_page,unsigned char * palette) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 830 | macro (function-like macro) | READ_CHAR |  | () | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WINSTUB.CPP | 845 | function (callable) | Read_PCX_File |  | (char * name,char * palette,void * Buff,long Size) | RUST/original-source-reference/TIBERIANDAWN/WINSTUB.md | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WWFILE.H | 57 | function (callable) | ~FileClass | FileClass | (void) |  | SIGNATURE_CATALOGUED |
| TIBERIANDAWN/WWFILE.H | 72 | function (callable) | operator char const* | FileClass | () |  | SIGNATURE_CATALOGUED |

**Extracted callable/candidate symbols:** 4329

A source-level signature is not a behavior description. Use PORTING_CONTRACT_TEMPLATE.md to document each contract, or group tightly coupled helpers when their behavior is inseparable.
