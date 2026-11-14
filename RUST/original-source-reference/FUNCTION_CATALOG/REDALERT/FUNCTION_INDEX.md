# Redalert — Function Index

**Pinned source:** f1f0d42bc2dcd06d5d1df943c6150ab34bf307ae  
**Interpretation:** extracted signatures only; behavior contracts remain to be reviewed.

| Source file | Line | Kind | Symbol | Scope | Signature | Documentation | Review state |
|---|---:|---|---|---|---|---|---|
| REDALERT/2KEYFRAM.CPP | 44 | macro (function-like macro) | Apply_Delta |  | (buffer,delta) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 99 | function (callable) | Get_Shape_Header_Data |  | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 111 | function (callable) | Get_Last_Frame_Length |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 117 | function (callable) | Reset_Theater_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 132 | function (callable) | Reallocate_Big_Shape_Buffer |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 169 | function (callable) | Disable_Uncompressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 190 | function (callable) | Enable_Uncompressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 196 | function (callable) | Check_Use_Compressed_Shapes |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 225 | function (callable) | Build_Frame |  | (void const * dataptr,unsigned short framenumber,void * buffptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 501 | function (callable) | Get_Build_Frame_Count |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 510 | function (callable) | Get_Build_Frame_X |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 519 | function (callable) | Get_Build_Frame_Y |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 543 | function (callable) | Get_Build_Frame_Width |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 567 | function (callable) | Get_Build_Frame_Height |  | (void const * dataptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/2KEYFRAM.CPP | 576 | function (callable) | Get_Build_Frame_Palette |  | (void const * dataptr,void * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 39 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 40 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 41 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 43 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 128 | label (assembly label candidate) | if |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2SUPPORT.ASM | 455 | label (assembly label candidate) | endif |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2TXTPRNT.ASM | 35 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2TXTPRNT.ASM | 36 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2TXTPRNT.ASM | 37 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2TXTPRNT.ASM | 44 | label (assembly label candidate) | STRUC |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/2TXTPRNT.ASM | 88 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 233 | function (callable) | AircraftTypeClass | AircraftTypeClass | (AircraftType airtype,int name,char const * ininame,int verticaloffset,int primaryoffset,int primarylateral,bool is_fixedwing,bool is_rotorequipped,bool is_rotorcustom,bool is_landable,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,StructType building,int landingspeed,int rotation,MissionType deforder) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 307 | function (callable) | operator new | AircraftTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 328 | function (callable) | operator delete | AircraftTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 349 | function (callable) | Init_Heap | AircraftTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 382 | function (callable) | From_Name | AircraftTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 410 | function (callable) | One_Time | AircraftTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 452 | function (callable) | Create_One_Of | AircraftTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 474 | function (callable) | Prep_For_Add | AircraftTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 503 | function (callable) | Display | AircraftTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 531 | function (callable) | Occupy_List | AircraftTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 552 | function (callable) | Overlap_List | AircraftTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 574 | function (callable) | Max_Pips | AircraftTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 601 | function (callable) | Create_And_Place | AircraftTypeClass | (CELL,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 624 | function (callable) | Dimensions | AircraftTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AADATA.CPP | 652 | function (callable) | As_Reference | AircraftTypeClass | (AircraftType aircraft) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 60 | function (callable) | Debug_Dump | AbstractClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 90 | function (callable) | Distance | AbstractClass | (TARGET target) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 131 | function (callable) | AbstractTypeClass | AbstractTypeClass | (RTTIType rtti,int id,int name,char const * ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 157 | function (callable) | Coord_Fixup | AbstractTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 178 | function (callable) | Full_Name | AbstractTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.CPP | 207 | function (callable) | Get_Ownable | AbstractTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 89 | function (callable) | AbstractClass | AbstractClass | (RTTIType rtti,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 90 | function (callable) | AbstractClass | AbstractClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 91 | function (callable) | ~AbstractClass | AbstractClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 96 | function (callable) | Name | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 97 | function (callable) | Owner | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 98 | function (callable) | As_Target | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 99 | function (callable) | What_Am_I | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 111 | function (callable) | Center_Coord | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 112 | function (callable) | Target_Coord | AbstractClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 118 | function (callable) | Direction | AbstractClass | (AbstractClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 119 | function (callable) | Direction | AbstractClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 120 | function (callable) | Direction | AbstractClass | (TARGET target) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 121 | function (callable) | Direction | AbstractClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 123 | function (callable) | Distance | AbstractClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 124 | function (callable) | Distance | AbstractClass | (AbstractClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 129 | function (callable) | Can_Enter_Cell | AbstractClass | (CELL,FacingType=FACING_NONE) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 134 | function (callable) | AI | AbstractClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ABSTRACT.H | 139 | function (callable) | Set_Active | AbstractClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2295 | function (callable) | AnimTypeClass | AnimTypeClass | (AnimType anim,char const * name,int size,int biggest,bool istheater,bool isnormal,bool iswhitetrans,bool isscorcher,bool iscrater,bool issticky,bool ground,bool istrans,bool isflame,fixed damage,int delaytime,int start,int loopstart,int loopend,int stages,int loops,VocType soundid,AnimType chainto,int virtualstages,int virtualscale,char const * virtualname,AnimType virtualanim) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2378 | function (callable) | operator new | AnimTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2399 | function (callable) | operator delete | AnimTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2420 | function (callable) | Init_Heap | AnimTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2535 | function (callable) | One_Time | AnimTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2576 | function (callable) | Init | AnimTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2610 | function (callable) | Anim_Name |  | (AnimType anim) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADATA.CPP | 2634 | function (callable) | As_Reference | AnimTypeClass | (AnimType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADPCM.CPP | 24 | function (callable) | sosCODECInitStream |  | (_SOS_COMPRESS_INFO * info) |  | SIGNATURE_CATALOGUED |
| REDALERT/ADPCM.CPP | 31 | function (callable) | sosCODECDecompressData |  | (_SOS_COMPRESS_INFO * info,unsigned long numbytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 113 | function (callable) | _Counts_As_Civ_Evac |  | (ObjectClass const * candidate) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 175 | function (callable) | operator new | AircraftClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 199 | function (callable) | operator delete | AircraftClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 225 | function (callable) | AircraftClass | AircraftClass | (AircraftType classid,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 283 | function (callable) | Unlimbo | AircraftClass | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 353 | function (callable) | Shape_Number | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 403 | function (callable) | Draw_It | AircraftClass | (int x,int y,WindowNumberType window) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 500 | function (callable) | Draw_Rotors | AircraftClass | (int x,int y,WindowNumberType window) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 561 | function (callable) | Read_INI | AircraftClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 645 | function (callable) | Mission_Hunt | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 882 | function (callable) | AI | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 989 | function (callable) | Overlap_List | AircraftClass | (bool redraw) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1064 | function (callable) | Init | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1086 | function (callable) | Mission_Unload | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1254 | function (callable) | Is_LZ_Clear | AircraftClass | (TARGET target) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1301 | function (callable) | Sort_Y | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1327 | function (callable) | Mission_Retreat | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1407 | function (callable) | Exit_Object | AircraftClass | (TechnoClass * unit) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1461 | function (callable) | Paradrop_Cargo | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1508 | function (callable) | Fire_At | AircraftClass | (TARGET target,int which) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1599 | function (callable) | Take_Damage | AircraftClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1674 | function (callable) | Mission_Move | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 1911 | function (callable) | Enter_Idle_Mode | AircraftClass | (bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2118 | function (callable) | Process_Fly_To | AircraftClass | (bool slowdown,TARGET dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2169 | function (callable) | Debug_Dump | AircraftClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2201 | function (callable) | Active_Click_With | AircraftClass | (ActionType action,ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2245 | function (callable) | Active_Click_With | AircraftClass | (ActionType action,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2274 | function (callable) | Player_Assign_Mission | AircraftClass | (MissionType mission,TARGET target,TARGET destination) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2307 | function (callable) | What_Action | AircraftClass | (ObjectClass const * target) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2370 | function (callable) | What_Action | AircraftClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2409 | function (callable) | Pose_Dir | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2441 | function (callable) | Mission_Attack | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2673 | function (callable) | New_LZ | AircraftClass | (TARGET oldlz) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2731 | function (callable) | Receive_Message | AircraftClass | (RadioClass * from,RadioMessageType message,long & param) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2900 | function (callable) | Desired_Load_Dir | AircraftClass | (ObjectClass * object,CELL & moveto) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2932 | function (callable) | Process_Take_Off | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 2997 | function (callable) | Process_Landing | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3065 | function (callable) | Can_Enter_Cell | AircraftClass | (CELL cell,FacingType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3111 | function (callable) | Good_Fire_Location | AircraftClass | (TARGET target) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3202 | function (callable) | Cell_Seems_Ok | AircraftClass | (CELL cell,bool strict) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3240 | function (callable) | Pip_Count | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3277 | function (callable) | Mission_Enter | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3536 | function (callable) | Good_LZ | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3592 | function (callable) | Set_Speed | AircraftClass | (int speed) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3619 | function (callable) | Fire_Direction | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3643 | function (callable) | ~AircraftClass | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3688 | function (callable) | Scatter | AircraftClass | (COORDINATE,bool,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3728 | function (callable) | Mission_Guard | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3868 | function (callable) | Mission_Guard_Area | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3906 | function (callable) | Response_Attack | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3936 | function (callable) | Response_Move | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 3966 | function (callable) | Response_Select | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4004 | function (callable) | Can_Fire | AircraftClass | (TARGET target,int which) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4073 | function (callable) | Landing_Takeoff_AI | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4205 | function (callable) | Edge_Of_World_AI | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4266 | function (callable) | Movement_AI | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4306 | function (callable) | Rotation_AI | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4341 | function (callable) | Per_Cell_Process | AircraftClass | (PCPType why) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4365 | function (callable) | Assign_Destination | AircraftClass | (TARGET dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4398 | function (callable) | In_Which_Layer | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4423 | function (callable) | Look | AircraftClass | (bool incremental) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.CPP | 4438 | function (callable) | Spied_By | AircraftClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 58 | function (callable) | operator new | AircraftClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 60 | function (callable) | operator AircraftType | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 62 | function (callable) | AircraftClass | AircraftClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 88 | function (callable) | Turret_Facing | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 91 | function (callable) | Class_Of | AircraftClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/AIRCRAFT.H | 172 | function (callable) | INI_Name | AircraftClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 74 | function (callable) | Anim_From_Name |  | (char const * name) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 108 | function (callable) | Shorten_Attached_Anims |  | (ObjectClass * obj) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 138 | function (callable) | Sort_Y | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 182 | function (callable) | Center_Coord | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 211 | function (callable) | Render | AnimClass | (bool forced) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 244 | function (callable) | Draw_It | AnimClass | (int x,int y,WindowNumberType window) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 347 | function (callable) | Mark | AnimClass | (MarkType mark) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 379 | function (callable) | Overlap_List | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 445 | function (callable) | Occupy_List | AnimClass | (bool) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 473 | function (callable) | Init | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 494 | function (callable) | operator new | AnimClass | (size_t) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 521 | function (callable) | operator delete | AnimClass | (void * ptr) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 552 | function (callable) | AnimClass | AnimClass | (AnimType animnum,COORDINATE coord,unsigned char timedelay,char loop) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 647 | function (callable) | ~AnimClass | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 717 | function (callable) | AI | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 920 | function (callable) | Attach_To | AnimClass | (ObjectClass * obj) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 957 | function (callable) | Sort_Above | AnimClass | (TARGET target) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 981 | function (callable) | In_Which_Layer | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1024 | function (callable) | Start | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1066 | function (callable) | Middle | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1139 | function (callable) | Chain | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1195 | function (callable) | Detach | AnimClass | (TARGET target,bool all) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1237 | function (callable) | Do_Atom_Damage | AnimClass | (HousesType ownerhouse,CELL cell) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1282 | function (callable) | Set_Owner | AnimClass | (HousesType owner) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.CPP | 1290 | function (callable) | Set_Visible_Flags | AnimClass | (unsigned flags) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 55 | function (callable) | AnimClass | AnimClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 58 | function (callable) | operator AnimType | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 61 | function (callable) | operator new | AnimClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 71 | function (callable) | Make_Invisible | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 72 | function (callable) | Make_Visible | AnimClass | (void) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 73 | function (callable) | Kill_At | AnimClass | (unsigned long long kill_time) | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 81 | function (callable) | Get_Visible_Flags | AnimClass | () const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 84 | function (callable) | Owner | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 85 | function (callable) | Can_Place_Here | AnimClass | (COORDINATE) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/ANIM.H | 91 | function (callable) | Class_Of | AnimClass | (void) const | RUST/original-source-reference/REDALERT/ANIM.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 58 | function (callable) | File_Stream_Sample |  | (char const * filename,BOOL real_time_start) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 59 | function (callable) | File_Stream_Sample_Vol |  | (char const * filename,int volume,BOOL real_time_start) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 60 | function (callable) | Sound_Callback |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 61 | function (callable) | maintenance_callback |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 62 | function (callable) | Load_Sample |  | (char const * filename) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 63 | function (callable) | Load_Sample_Into_Buffer |  | (char const * filename,void * buffer,long size) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 64 | function (callable) | Sample_Read |  | (int fh,void * buffer,long size) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 65 | function (callable) | Free_Sample |  | (void const * sample) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 66 | function (callable) | Audio_Init |  | (HWND window,int bits_per_sample,BOOL stereo,int rate,int reverse_channels) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 67 | function (callable) | Sound_End |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 68 | function (callable) | Stop_Sample |  | (int handle) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 69 | function (callable) | Sample_Status |  | (int handle) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 70 | function (callable) | Is_Sample_Playing |  | (void const * sample) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 71 | function (callable) | Stop_Sample_Playing |  | (void const * sample) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 72 | function (callable) | Play_Sample |  | (void const * sample,int priority,int volume,signed short panloc) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 73 | function (callable) | Play_Sample_Handle |  | (void const * sample,int priority,int volume,signed short panloc,int id) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 74 | function (callable) | Set_Sound_Vol |  | (int volume) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 75 | function (callable) | Set_Score_Vol |  | (int volume) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 76 | function (callable) | Fade_Sample |  | (int handle,int ticks) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 77 | function (callable) | Get_Free_Sample_Handle |  | (int priority) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 78 | function (callable) | Get_Digi_Handle |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 79 | function (callable) | Sample_Length |  | (void const * sample) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 80 | function (callable) | Restore_Sound_Buffers |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 81 | function (callable) | Set_Primary_Buffer_Format |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 82 | function (callable) | Start_Primary_Sound_Buffer |  | (BOOL forced) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 83 | function (callable) | Stop_Primary_Sound_Buffer |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 316 | function (callable) | Voc_From_Name |  | (char const * name) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 345 | function (callable) | Voc_Name |  | (VocType voc) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 380 | function (callable) | Sound_Effect |  | (VocType voc,COORDINATE coord,int variation,HousesType house) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 450 | function (callable) | Sound_Effect |  | (VocType voc,fixed volume,int variation,signed short pan_value,HousesType house) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 689 | function (callable) | Speech_Name |  | (VoxType speech) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 711 | function (callable) | Speak |  | (VoxType voice,HouseClass * house,COORDINATE coord) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 755 | function (callable) | Speak_AI |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 830 | function (callable) | Stop_Speaking |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.CPP | 853 | function (callable) | Is_Speaking |  | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.H | 64 | function (callable) | AudioClass | AudioClass | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.H | 72 | function (callable) | AudioClass | AudioClass | (char const * name,MemoryClass & mem) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/AUDIO.H | 84 | function (callable) | ~AudioClass | AudioClass | (void) | RUST/original-source-reference/REDALERT/AUDIO.md | SIGNATURE_CATALOGUED |
| REDALERT/B64PIPE.CPP | 62 | function (callable) | Put | Base64Pipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/B64PIPE.CPP | 143 | function (callable) | Flush | Base64Pipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/B64PIPE.H | 53 | function (callable) | Base64Pipe | Base64Pipe | (CodeControl control) |  | SIGNATURE_CATALOGUED |
| REDALERT/B64STRAW.CPP | 61 | function (callable) | Get | Base64Straw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/B64STRAW.H | 53 | function (callable) | Base64Straw | Base64Straw | (CodeControl control) |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.CPP | 70 | function (callable) | ProgressBarClass | ProgressBarClass | (int x,int y,int width,int height,int forecolor,int backcolor,int bordercolor) |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.CPP | 99 | function (callable) | Is_Horizontal | ProgressBarClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.CPP | 122 | function (callable) | Update | ProgressBarClass | (fixed value) |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.CPP | 149 | function (callable) | Outline | ProgressBarClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.CPP | 174 | function (callable) | Redraw | ProgressBarClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BAR.H | 71 | function (callable) | Is_Outlined | ProgressBarClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 66 | function (callable) | operator == | BaseNodeClass | (BaseNodeClass const & node) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 87 | function (callable) | operator != | BaseNodeClass | (BaseNodeClass const & node) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 108 | function (callable) | operator > | BaseNodeClass | (BaseNodeClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 130 | function (callable) | Load | BaseClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 188 | function (callable) | Save | BaseClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 235 | function (callable) | Is_Built | BaseClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 261 | function (callable) | Get_Building | BaseClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 310 | function (callable) | Is_Node | BaseClass | (BuildingClass const * obj) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 335 | function (callable) | Get_Node | BaseClass | (BuildingClass const * obj) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 361 | function (callable) | Get_Node | BaseClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 390 | function (callable) | Next_Buildable | BaseClass | (StructType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 447 | function (callable) | Read_INI | BaseClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.CPP | 509 | function (callable) | Write_INI | BaseClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 44 | function (callable) | BaseNodeClass | BaseNodeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 45 | function (callable) | BaseNodeClass | BaseNodeClass | (StructType building,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 66 | function (callable) | BaseClass | BaseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 67 | function (callable) | ~BaseClass | BaseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 72 | function (callable) | Init | BaseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 79 | function (callable) | INI_Name | BaseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 82 | function (callable) | Code_Pointers | BaseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 83 | function (callable) | Decode_Pointers | BaseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE.H | 104 | function (callable) | Get_Node | BaseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE64.CPP | 140 | function (callable) | Base64_Encode |  | (void const * source,int slen,void * dest,int dlen) |  | SIGNATURE_CATALOGUED |
| REDALERT/BASE64.CPP | 241 | function (callable) | Base64_Decode |  | (void const * source,int slen,void * dest,int dlen) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 63 | function (callable) | BulletTypeClass | BulletTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 118 | function (callable) | operator new | BulletTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 140 | function (callable) | operator delete | BulletTypeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 160 | function (callable) | Init_Heap | BulletTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 204 | function (callable) | One_Time | BulletTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 248 | function (callable) | As_Reference | BulletTypeClass | (BulletType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/BBDATA.CPP | 269 | function (callable) | Read_INI | BulletTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 67 | macro (function-like macro) | XYCELL |  | (x,y) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 2776 | function (callable) | BuildingTypeClass | BuildingTypeClass | (StructType type,int name,char const * ininame,FacingType foundation,COORDINATE exitpoint,RemapType remap,int verticaloffset,int primaryoffset,int primarylateral,bool is_fake,bool is_regulated,bool is_nominal,bool is_wall,bool is_simpledamage,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_theater,bool is_turret_equipped,bool is_remappable,RTTIType tobuild,DirType sframe,BSizeType size,short const * exitlist,short const * sizelist,short const * overlap) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 2889 | function (callable) | operator new | BuildingTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 2910 | function (callable) | operator delete | BuildingTypeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 2931 | function (callable) | Init_Heap | BuildingTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3049 | function (callable) | One_Time | BuildingTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3173 | function (callable) | From_Name | BuildingTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3207 | function (callable) | Display | BuildingTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3235 | function (callable) | Prep_For_Add | BuildingTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3262 | function (callable) | Create_And_Place | BuildingTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3292 | function (callable) | Create_One_Of | BuildingTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3324 | function (callable) | Init_Anim | BuildingTypeClass | (BStateType state,int start,int count,int rate) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3348 | function (callable) | Init | BuildingTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3397 | function (callable) | Dimensions | BuildingTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3422 | function (callable) | As_Reference | BuildingTypeClass | (StructType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3446 | function (callable) | Occupy_List | BuildingTypeClass | (bool placement) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3503 | function (callable) | Overlap_List | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3528 | function (callable) | Width | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3559 | function (callable) | Height | BuildingTypeClass | (bool bib) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3595 | function (callable) | Bib_And_Offset | BuildingTypeClass | (SmudgeType & bib,CELL & cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3645 | function (callable) | Max_Pips | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3670 | function (callable) | Raw_Cost | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3698 | function (callable) | Cost_Of | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3725 | function (callable) | Flush_For_Placement | BuildingTypeClass | (CELL cell,HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3765 | function (callable) | Read_INI | BuildingTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3804 | function (callable) | Coord_Fixup | BuildingTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BDATA.CPP | 3826 | function (callable) | Full_Name | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.CPP | 59 | function (callable) | Benchmark | Benchmark | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.CPP | 82 | function (callable) | Reset | Benchmark | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.CPP | 105 | function (callable) | Begin | Benchmark | (bool reset) |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.CPP | 128 | function (callable) | End | Benchmark | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.CPP | 158 | function (callable) | Value | Benchmark | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.H | 63 | function (callable) | operator ()  | PentiumTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.H | 64 | function (callable) | operator unsigned long | PentiumTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BENCH.H | 84 | function (callable) | Count | Benchmark | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 73 | function (callable) | BufferIOFileClass | BufferIOFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 109 | function (callable) | BufferIOFileClass | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 144 | function (callable) | ~BufferIOFileClass | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 163 | function (callable) | Cache | BufferIOFileClass | (long size,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 326 | function (callable) | Free | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 359 | function (callable) | Commit | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 410 | function (callable) | Set_Name | BufferIOFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 439 | function (callable) | Is_Available | BufferIOFileClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 464 | function (callable) | Is_Open | BufferIOFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 495 | function (callable) | Open | BufferIOFileClass | (char const * filename,int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 520 | function (callable) | Open | BufferIOFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 585 | function (callable) | Write | BufferIOFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 728 | function (callable) | Read | BufferIOFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 852 | function (callable) | Seek | BufferIOFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 936 | function (callable) | Size | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BFIOFILE.CPP | 961 | function (callable) | Close | BufferIOFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BIGCHECK.CPP | 25 | function (callable) | Draw_Me | BigCheckBoxClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/BIGCHECK.CPP | 67 | function (callable) | Action | BigCheckBoxClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/BIGCHECK.H | 33 | function (callable) | BigCheckBoxClass | BigCheckBoxClass | (unsigned id,int x,int y,int w,int h,const char * szCaptionIn,TextPrintType TextFlags,bool bInitiallyChecked=false) |  | SIGNATURE_CATALOGUED |
| REDALERT/BIGCHECK.H | 44 | function (callable) | ~BigCheckBoxClass | BigCheckBoxClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/BIGCHECK.H | 52 | function (callable) | Toggle | BigCheckBoxClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 79 | function (callable) | ~BlowfishEngine | BlowfishEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 111 | function (callable) | Submit_Key | BlowfishEngine | (void const * key,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 211 | function (callable) | Encrypt | BlowfishEngine | (void const * plaintext,int length,void * cyphertext) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 277 | function (callable) | Decrypt | BlowfishEngine | (void const * cyphertext,int length,void * plaintext) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 345 | function (callable) | Process_Block | BlowfishEngine | (void const * plaintext,void * cyphertext,unsigned long const * ptable) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.CPP | 428 | function (callable) | Sub_Key_Encrypt | BlowfishEngine | (unsigned long & left,unsigned long & right) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWFISH.H | 67 | function (callable) | BlowfishEngine | BlowfishEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWPIPE.CPP | 60 | function (callable) | Flush | BlowPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWPIPE.CPP | 91 | function (callable) | Put | BlowPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWPIPE.CPP | 182 | function (callable) | Key | BlowPipe | (void const * key,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWPIPE.H | 54 | function (callable) | BlowPipe | BlowPipe | (CryptControl control) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLOWPIPE.H | 55 | function (callable) | ~BlowPipe | BlowPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLWSTRAW.CPP | 63 | function (callable) | Get | BlowStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLWSTRAW.CPP | 144 | function (callable) | Key | BlowStraw | (void const * key,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLWSTRAW.H | 57 | function (callable) | BlowStraw | BlowStraw | (CryptControl control) |  | SIGNATURE_CATALOGUED |
| REDALERT/BLWSTRAW.H | 58 | function (callable) | ~BlowStraw | BlowStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BMP8.CPP | 20 | function (callable) | ~BMP8 | BMP8 | () |  | SIGNATURE_CATALOGUED |
| REDALERT/BMP8.CPP | 30 | function (callable) | Init | BMP8 | (const char * szFile,HWND hWnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/BMP8.CPP | 142 | function (callable) | drawBmp | BMP8 | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BMP8.H | 27 | function (callable) | BMP8 | BMP8 | () |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 65 | function (callable) | Buffer | Buffer | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 74 | function (callable) | Buffer | Buffer | (char * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 83 | function (callable) | Buffer | Buffer | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 108 | function (callable) | Buffer | Buffer | (long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 135 | function (callable) | Buffer | Buffer | (Buffer const & buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 158 | function (callable) | operator = | Buffer | (Buffer const & buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 186 | function (callable) | ~Buffer | Buffer | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.CPP | 209 | function (callable) | Reset | Buffer | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.H | 68 | function (callable) | operator void* | Buffer | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.H | 69 | function (callable) | operator char* | Buffer | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.H | 72 | function (callable) | Get_Buffer | Buffer | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.H | 73 | function (callable) | Get_Size | Buffer | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFF.H | 74 | function (callable) | Is_Valid | Buffer | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 48 | function (callable) | Transmuter | Transmuter | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 49 | function (callable) | ~Transmuter | Transmuter | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 57 | function (callable) | Attach | Transmuter | (Transmuter * transmuter) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 58 | function (callable) | Flush | Transmuter | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 59 | function (callable) | Put | Transmuter | (const void * input,unsigned length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 72 | function (callable) | FileTransmuter | FileTransmuter | (FileClass * file=NULL) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 74 | function (callable) | Attach | FileTransmuter | (FileClass * file) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 75 | function (callable) | Flush | FileTransmuter | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 76 | function (callable) | Put | FileTransmuter | (const void * input,unsigned length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 85 | function (callable) | BufferTransmuter | BufferTransmuter | (void * buffer=NULL) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 87 | function (callable) | Attach | BufferTransmuter | (void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 88 | function (callable) | Flush | BufferTransmuter | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUFFERX.H | 89 | function (callable) | Put | BufferTransmuter | (const void * input,unsigned length) |  | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 162 | function (callable) | Receive_Message | BuildingClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 409 | function (callable) | Debug_Dump | BuildingClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 453 | function (callable) | Draw_It | BuildingClass | (int x,int y,WindowNumberType window) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 565 | function (callable) | Shape_Number | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 722 | function (callable) | Mark | BuildingClass | (MarkType mark) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 834 | function (callable) | AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1074 | function (callable) | Unlimbo | BuildingClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1222 | function (callable) | Take_Damage | BuildingClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1544 | function (callable) | operator new | BuildingClass | (size_t) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1570 | function (callable) | operator delete | BuildingClass | (void * ptr) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1602 | function (callable) | BuildingClass | BuildingClass | (StructType type,HousesType house) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1666 | function (callable) | ~BuildingClass | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1699 | function (callable) | Drop_Debris | BuildingClass | (TARGET source) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1806 | function (callable) | Active_Click_With | BuildingClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1839 | function (callable) | Active_Click_With | BuildingClass | (ActionType action,CELL cell) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1874 | function (callable) | Assign_Target | BuildingClass | (TARGET target) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1901 | function (callable) | Init | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 1932 | function (callable) | Exit_Object | BuildingClass | (TechnoClass * base) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2187 | function (callable) | Update_Buildables | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2308 | function (callable) | Update_Buildables | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2393 | function (callable) | Fire_Out | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2416 | function (callable) | Limbo | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2473 | function (callable) | Turret_Facing | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2499 | function (callable) | Greatest_Threat | BuildingClass | (ThreatType threat) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2544 | function (callable) | Grand_Opening | BuildingClass | (bool captured) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2647 | function (callable) | Repair | BuildingClass | (int control) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2712 | function (callable) | Sell_Back | BuildingClass | (int control) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2775 | function (callable) | What_Action | BuildingClass | (ObjectClass const * object) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2884 | function (callable) | What_Action | BuildingClass | (CELL cell) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2925 | function (callable) | Begin_Mode | BuildingClass | (BStateType bstate) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2960 | function (callable) | Center_Coord | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 2986 | function (callable) | Docking_Coord | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3020 | function (callable) | Can_Fire | BuildingClass | (TARGET target,int which) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3083 | function (callable) | Toggle_Primary | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3150 | function (callable) | Captured | BuildingClass | (HouseClass * newowner) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3355 | function (callable) | Sort_Y | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3405 | function (callable) | Can_Enter_Cell | BuildingClass | (CELL cell,FacingType) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3439 | function (callable) | Can_Demolish | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3454 | function (callable) | Can_Demolish_Unit | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3460 | function (callable) | Can_Capture | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3492 | function (callable) | Mission_Guard | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3589 | function (callable) | Mission_Construction | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3649 | function (callable) | Mission_Deconstruction | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 3918 | function (callable) | Mission_Attack | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4047 | function (callable) | Mission_Harvest | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4129 | function (callable) | Mission_Repair | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4384 | function (callable) | Mission_Missile | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4604 | function (callable) | Revealed | BuildingClass | (HouseClass * house) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4651 | function (callable) | Enter_Idle_Mode | BuildingClass | (bool initial) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4691 | function (callable) | Pip_Count | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4716 | function (callable) | Death_Announcement | BuildingClass | (TechnoClass const * source) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4743 | function (callable) | Fire_Direction | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4771 | function (callable) | Remap_Table | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4797 | function (callable) | Mission_Unload | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4934 | function (callable) | Power_Output | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4963 | function (callable) | Detach | BuildingClass | (TARGET target,bool all) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 4994 | function (callable) | Crew_Type | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5046 | function (callable) | Detach_All | BuildingClass | (bool all) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5106 | function (callable) | Flush_For_Placement | BuildingClass | (TechnoClass * techno,CELL cell) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5136 | function (callable) | Find_Exit_Cell | BuildingClass | (TechnoClass const * techno) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5202 | function (callable) | Can_Player_Move | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5226 | function (callable) | Exit_Coord | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5255 | function (callable) | Check_Point | BuildingClass | (CheckPointType cp) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5302 | function (callable) | Update_Radar_Spied | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5336 | function (callable) | Read_INI | BuildingClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5453 | function (callable) | Write_INI | BuildingClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5502 | function (callable) | Target_Coord | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5529 | function (callable) | Factory_AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5676 | function (callable) | Rotation_AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5711 | function (callable) | Charging_AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5761 | function (callable) | Repair_AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5831 | function (callable) | Animation_AI | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5920 | function (callable) | How_Many_Survivors | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5948 | function (callable) | Get_Image_Data | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 5974 | function (callable) | Value | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 6013 | function (callable) | Remove_Gap_Effect | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 6050 | function (callable) | Overlap_List | BuildingClass | (bool redraw) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.CPP | 6069 | function (callable) | Spied_By | BuildingClass | () const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 215 | function (callable) | operator new | BuildingClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 219 | function (callable) | BuildingClass | BuildingClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 221 | function (callable) | BuildingClass | BuildingClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 224 | function (callable) | operator StructType | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 232 | function (callable) | Fetch_Anim_Control | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 250 | function (callable) | Class_Of | BuildingClass | (void) const | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BUILDING.H | 345 | function (callable) | INI_Name | BuildingClass | (void) | RUST/original-source-reference/REDALERT/BUILDING.md | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 76 | function (callable) | BulletClass | BulletClass | (BulletType id,TARGET target,TechnoClass * payback,int strength,WarheadType warhead,int speed) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 109 | function (callable) | ~BulletClass | BulletClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 191 | function (callable) | operator new | BulletClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 216 | function (callable) | operator delete | BulletClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 243 | function (callable) | Occupy_List | BulletClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 311 | function (callable) | Mark | BulletClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 342 | function (callable) | AI | BulletClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 515 | function (callable) | Shape_Number | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 552 | function (callable) | Draw_It | BulletClass | (int x,int y,WindowNumberType window) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 625 | function (callable) | Init | BulletClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 650 | function (callable) | Detach | BulletClass | (TARGET target,bool all) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 691 | function (callable) | Unlimbo | BulletClass | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 839 | function (callable) | Target_Coord | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 865 | function (callable) | Sort_Y | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 890 | function (callable) | In_Which_Layer | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 917 | function (callable) | Is_Forced_To_Explode | BulletClass | (COORDINATE & coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.CPP | 987 | function (callable) | Bullet_Explodes | BulletClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 73 | function (callable) | operator new | BulletClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 77 | function (callable) | BulletClass | BulletClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 79 | function (callable) | BulletClass | BulletClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 82 | function (callable) | operator BulletType | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 94 | function (callable) | Assign_Target | BulletClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 96 | function (callable) | Class_Of | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/BULLET.H | 102 | function (callable) | Overlap_List | BulletClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.CPP | 57 | function (callable) | Debug_Dump | CargoClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.CPP | 84 | function (callable) | Attach | CargoClass | (FootClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.CPP | 141 | function (callable) | Detach_Object | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.CPP | 172 | function (callable) | Attached_Object | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 49 | function (callable) | CargoClass | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 50 | function (callable) | CargoClass | CargoClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 51 | function (callable) | ~CargoClass | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 60 | function (callable) | AI | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 62 | function (callable) | How_Many | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARGO.H | 63 | function (callable) | Is_Something_Attached | CargoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARRY.CPP | 56 | function (callable) | CarryoverClass | CarryoverClass | (TechnoClass * techno) |  | SIGNATURE_CATALOGUED |
| REDALERT/CARRY.CPP | 109 | function (callable) | Create | CarryoverClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CARRY.H | 42 | function (callable) | CarryoverClass | CarryoverClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 90 | function (callable) | DDE_Callback |  | (unsigned char * data,long length) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 112 | function (callable) | DDEServerClass | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 145 | function (callable) | Enable | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 169 | function (callable) | Disable | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 196 | function (callable) | ~DDEServerClass | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 219 | function (callable) | Callback | DDEServerClass | (unsigned char * data,long length) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 327 | function (callable) | Get_MPlayer_Game_Info | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 348 | function (callable) | Delete_MPlayer_Game_Info | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 372 | function (callable) | Time_Since_Heartbeat | DDEServerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.CPP | 396 | function (callable) | Send_Data_To_DDE_Server |  | (char * data,int length,int packet_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCDDE.H | 53 | function (callable) | Get_MPlayer_Game_Info_Length | DDEServerClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 68 | function (callable) | CCFileClass | CCFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 89 | function (callable) | CCFileClass | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 118 | function (callable) | Error | CCFileClass | (int,int,char const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 145 | function (callable) | Write | CCFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 177 | function (callable) | Read | CCFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 242 | function (callable) | Seek | CCFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 289 | function (callable) | Size | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 326 | function (callable) | Is_Available | CCFileClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 363 | function (callable) | Is_Open | CCFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 392 | function (callable) | Close | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 416 | function (callable) | Open | CCFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 498 | function (callable) | Get_Date_Time | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 533 | function (callable) | Set_Date_Time | CCFileClass | (unsigned long datetime) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 562 | function (callable) | Open_File |  | (char const * file_name,int mode) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 575 | function (callable) | Close_File |  | (int handle) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 582 | function (callable) | Read_File |  | (int handle,void * buf,unsigned long bytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 590 | function (callable) | Write_File |  | (int handle,void const * buf,unsigned long bytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 598 | function (callable) | Find_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 605 | function (callable) | Delete_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 610 | function (callable) | Create_File |  | (char const * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 615 | function (callable) | Load_Data |  | (char const * name,void * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 621 | function (callable) | Load_Alloc_Data |  | (char const * name,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 628 | function (callable) | File_Size |  | (int handle) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 637 | function (callable) | Write_Data |  | (char const * name,void const * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 643 | function (callable) | Seek_File |  | (int handle,long offset,int starting) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 652 | function (callable) | Multi_Drive_Search |  | (bool on) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 658 | function (callable) | WWDOS_Init |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 662 | function (callable) | WWDOS_Shutdown |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 666 | function (callable) | Find_Disk_Number |  | (char const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.CPP | 688 | function (callable) | Unfragment_File_Cache |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.H | 55 | function (callable) | ~CCFileClass | CCFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.H | 59 | function (callable) | Is_Resident | CCFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCFILE.H | 62 | function (callable) | Open | CCFileClass | (char const * filename,int rights=READ) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 107 | function (callable) | Load | CCINIClass | (FileClass & file,bool withdigest) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 132 | function (callable) | Load | CCINIClass | (Straw & file,bool withdigest) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 183 | function (callable) | Save | CCINIClass | (FileClass & file,bool withdigest) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 209 | function (callable) | Save | CCINIClass | (Pipe & pipe,bool withdigest) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 248 | function (callable) | _Scale_To_256 |  | (int val) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 279 | function (callable) | Get_Lepton | CCINIClass | (char const * section,char const * entry,LEPTON defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 305 | function (callable) | Put_Lepton | CCINIClass | (char const * section,char const * entry,LEPTON value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 331 | function (callable) | Get_MPHType | CCINIClass | (char const * section,char const * entry,MPHType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 357 | function (callable) | Put_MPHType | CCINIClass | (char const * section,char const * entry,MPHType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 386 | function (callable) | Get_Owners | CCINIClass | (char const * section,char const * entry,long defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 425 | function (callable) | Put_Owners | CCINIClass | (char const * section,char const * entry,long value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 478 | function (callable) | Get_ArmorType | CCINIClass | (char const * section,char const * entry,ArmorType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 505 | function (callable) | Put_ArmorType | CCINIClass | (char const * section,char const * entry,ArmorType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 531 | function (callable) | Get_VocType | CCINIClass | (char const * section,char const * entry,VocType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 559 | function (callable) | Put_VocType | CCINIClass | (char const * section,char const * entry,VocType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 588 | function (callable) | Get_AnimType | CCINIClass | (char const * section,char const * entry,AnimType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 616 | function (callable) | Put_AnimType | CCINIClass | (char const * section,char const * entry,AnimType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 625 | function (callable) | Get_UnitType | CCINIClass | (char const * section,char const * entry,UnitType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 638 | function (callable) | Put_UnitType | CCINIClass | (char const * section,char const * entry,UnitType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 668 | function (callable) | Get_WeaponType | CCINIClass | (char const * section,char const * entry,WeaponType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 697 | function (callable) | Put_WeaponType | CCINIClass | (char const * section,char const * entry,WeaponType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 725 | function (callable) | Get_WarheadType | CCINIClass | (char const * section,char const * entry,WarheadType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 758 | function (callable) | Put_WarheadType | CCINIClass | (char const * section,char const * entry,WarheadType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 786 | function (callable) | Get_OverlayType | CCINIClass | (char const * section,char const * entry,OverlayType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 815 | function (callable) | Put_OverlayType | CCINIClass | (char const * section,char const * entry,OverlayType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 842 | function (callable) | Get_BulletType | CCINIClass | (char const * section,char const * entry,BulletType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 876 | function (callable) | Put_BulletType | CCINIClass | (char const * section,char const * entry,BulletType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 906 | function (callable) | Get_HousesType | CCINIClass | (char const * section,char const * entry,HousesType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 935 | function (callable) | Put_HousesType | CCINIClass | (char const * section,char const * entry,HousesType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 960 | function (callable) | Get_VQType | CCINIClass | (char const * section,char const * entry,VQType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 993 | function (callable) | Put_VQType | CCINIClass | (char const * section,char const * entry,VQType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1021 | function (callable) | Get_TheaterType | CCINIClass | (char const * section,char const * entry,TheaterType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1050 | function (callable) | Put_TheaterType | CCINIClass | (char const * section,char const * entry,TheaterType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1073 | function (callable) | Get_TriggerType | CCINIClass | (char const * section,char const * entry) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1102 | function (callable) | Put_TriggerType | CCINIClass | (char const * section,char const * entry,TriggerTypeClass * value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1127 | function (callable) | Get_ThemeType | CCINIClass | (char const * section,char const * entry,ThemeType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1156 | function (callable) | Put_ThemeType | CCINIClass | (char const * section,char const * entry,ThemeType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1182 | function (callable) | Get_SourceType | CCINIClass | (char const * section,char const * entry,SourceType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1211 | function (callable) | Put_SourceType | CCINIClass | (char const * section,char const * entry,SourceType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1236 | function (callable) | Get_CrateType | CCINIClass | (char const * section,char const * entry,CrateType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1265 | function (callable) | Put_CrateType | CCINIClass | (char const * section,char const * entry,CrateType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1290 | function (callable) | Get_TerrainType | CCINIClass | (char const * section,char const * entry,TerrainType defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1319 | function (callable) | Put_TerrainType | CCINIClass | (char const * section,char const * entry,TerrainType value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1347 | function (callable) | Get_Buildings | CCINIClass | (char const * section,char const * entry,long defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1393 | function (callable) | Put_Buildings | CCINIClass | (char const * section,char const * entry,long value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1428 | function (callable) | Get_Unique_ID | CCINIClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1454 | function (callable) | Calculate_Message_Digest | CCINIClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.CPP | 1481 | function (callable) | Invalidate_Message_Digest | CCINIClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCINI.H | 52 | function (callable) | CCINIClass | CCINIClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCMPATH.CPP | 53 | function (callable) | Init_MPATH |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCMPATH.CPP | 111 | function (callable) | Shutdown_MPATH |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCMPATH.CPP | 156 | function (callable) | Connect_MPATH |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCMPATH.CPP | 345 | function (callable) | Destroy_MPATH_Connection |  | (int id,int error) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.CPP | 84 | function (callable) | CCPtr | CCPtr | (T * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.CPP | 108 | function (callable) | operator > | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 48 | function (callable) | CCPtr | CCPtr | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 49 | function (callable) | CCPtr | CCPtr | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 52 | function (callable) | operator T* | CCPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 57 | function (callable) | operator * | CCPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 61 | function (callable) | operator -> | CCPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 67 | function (callable) | Is_Valid | CCPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 69 | function (callable) | operator == | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 70 | function (callable) | operator != | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 72 | function (callable) | operator <= | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 73 | function (callable) | operator < | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 74 | function (callable) | operator >= | CCPtr | (CCPtr<T> const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 76 | function (callable) | Raw | CCPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 77 | function (callable) | Set_Raw | CCPtr | (long value) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 79 | function (callable) | Set_Heap | CCPtr | (FixedIHeapClass * heap) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 103 | function (callable) | operator == |  | (CCPtr<T> & lvalue,T * rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCPTR.H | 109 | function (callable) | operator == |  | (T * lvalue,CCPtr<T> & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 67 | function (callable) | Init_TEN |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 125 | function (callable) | Shutdown_TEN |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 170 | function (callable) | Connect_TEN |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 358 | function (callable) | Destroy_TEN_Connection |  | (int id,int error) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 446 | function (callable) | Debug_Mono |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 481 | function (callable) | Send_TEN_Win_Packet |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 531 | function (callable) | Send_TEN_Alliance |  | (char * whom,int ally) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 567 | function (callable) | Send_TEN_Out_Of_Sync |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CCTEN.CPP | 591 | function (callable) | Send_TEN_Packet_Too_Late |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2484 | function (callable) | TemplateTypeClass | TemplateTypeClass | (TemplateType iconset,int theater,char const * ininame,int fullname) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2524 | function (callable) | operator new | TemplateTypeClass | (size_t) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2545 | function (callable) | operator delete | TemplateTypeClass | (void * ptr) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2551 | function (callable) | _Watcom_Ugh_Hack |  | (void) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2754 | function (callable) | Init_Heap | TemplateTypeClass | (void) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 2999 | function (callable) | Land_Type | TemplateTypeClass | (int icon) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3049 | function (callable) | From_Name | TemplateTypeClass | (char const * name) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3081 | function (callable) | Occupy_List | TemplateTypeClass | (bool) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3119 | function (callable) | Init | TemplateTypeClass | (TheaterType theater) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3162 | function (callable) | Display | TemplateTypeClass | (int x,int y,WindowNumberType window,HousesType) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3223 | function (callable) | Prep_For_Add | TemplateTypeClass | (void) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3249 | function (callable) | Create_And_Place | TemplateTypeClass | (CELL cell,HousesType) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3274 | function (callable) | Create_One_Of | TemplateTypeClass | (HouseClass *) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3292 | function (callable) | One_Time | TemplateTypeClass | (void) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3312 | function (callable) | As_Reference | TemplateTypeClass | (TemplateType type) | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDATA.CPP | 3318 | function (callable) | Coord_Fixup | TemplateTypeClass | (COORDINATE coord) const | RUST/original-source-reference/REDALERT/CDATA.md | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 61 | function (callable) | CDFileClass | CDFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 69 | function (callable) | CDFileClass | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 93 | function (callable) | harderr_handler |  | (unsigned int,unsigned int,unsigned int *) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 117 | function (callable) | Is_Disk_Inserted |  | (int disk) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 182 | function (callable) | Open | CDFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 200 | function (callable) | Refresh_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 371 | function (callable) | Set_Search_Drives | CDFileClass | (char * pathlist) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 469 | function (callable) | Add_Search_Drive | CDFileClass | (char * path) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 513 | function (callable) | Set_CD_Drive | CDFileClass | (int drive) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 535 | function (callable) | Clear_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 576 | function (callable) | Set_Name | CDFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 650 | function (callable) | Open | CDFileClass | (char const * filename,int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.CPP | 680 | function (callable) | Get_Search_Path | CDFileClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 56 | function (callable) | ~CDFileClass | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 62 | function (callable) | Searching | CDFileClass | (int on) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 64 | function (callable) | Is_There_Search_Drives | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 70 | function (callable) | Get_CD_Drive | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 71 | function (callable) | Get_Last_CD_Drive | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CDFILE.H | 74 | function (callable) | Reset_Raw_Path | CDFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 104 | function (callable) | CellClass | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 159 | function (callable) | Cell_Color | CellClass | (bool override) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 199 | function (callable) | Cell_Techno | CellClass | (int x,int y) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 245 | function (callable) | Cell_Find_Object | CellClass | (RTTIType rtti) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 280 | function (callable) | Cell_Building | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 305 | function (callable) | Cell_Terrain | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 334 | function (callable) | Cell_Object | CellClass | (int x,int y) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 378 | function (callable) | Redraw_Objects | CellClass | (bool forced) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 460 | function (callable) | Is_Clear_To_Build | CellClass | (SpeedType loco) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 539 | function (callable) | Recalc_Attributes | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 594 | function (callable) | Occupy_Down | CellClass | (ObjectClass * object) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 688 | function (callable) | Occupy_Up | CellClass | (ObjectClass * object) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 757 | function (callable) | Overlap_Down | CellClass | (ObjectClass * object) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 817 | function (callable) | Overlap_Up | CellClass | (ObjectClass * object) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 846 | function (callable) | Cell_Unit | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 869 | function (callable) | Cell_Vessel | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 893 | function (callable) | Cell_Infantry | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 902 | function (callable) | _Calc_Partial_Window |  | (int cellx,int celly,int & drawx,int & drawy) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 946 | function (callable) | _ocompare |  | (const void * left,const void * right) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 973 | function (callable) | Get_Template_Info | CellClass | (char * template_name,int & icon,void * & image_data) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1022 | function (callable) | Draw_It | CellClass | (int x,int y,bool objects) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1364 | function (callable) | Concrete_Calc | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1586 | function (callable) | Wall_Update | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1671 | function (callable) | Cell_Coord | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1694 | function (callable) | Reduce_Tiberium | CellClass | (int levels) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1732 | function (callable) | Reduce_Wall | CellClass | (int damage) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1812 | function (callable) | Spot_Index | CellClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1863 | function (callable) | Closest_Free_Spot | CellClass | (COORDINATE coord,bool any) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1955 | function (callable) | Clear_Icon | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 1987 | function (callable) | Incoming | CellClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2021 | function (callable) | Adjacent_Cell | CellClass | (FacingType face) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2054 | function (callable) | Adjust_Threat | CellClass | (HousesType house,int threat_value) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2094 | function (callable) | Tiberium_Adjust | CellClass | (bool pregame) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2180 | function (callable) | Goodie_Check | CellClass | (FootClass * object) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2752 | function (callable) | Flag_Place | CellClass | (HousesType house) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2781 | function (callable) | Flag_Remove | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2796 | function (callable) | Flag_Update | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2806 | function (callable) | Flag_Create | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2826 | function (callable) | Flag_Destroy | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2849 | function (callable) | Shimmer | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2889 | function (callable) | Is_Clear_To_Move | CellClass | (SpeedType loco,bool ignoreinfantry,bool ignorevehicles,int zone,MZoneType check) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 2981 | function (callable) | Is_Bridge_Here | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3022 | function (callable) | Can_Tiberium_Grow | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3057 | function (callable) | Can_Tiberium_Spread | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3089 | function (callable) | Grow_Tiberium | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3116 | function (callable) | Spread_Tiberium | CellClass | (bool forced) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3149 | function (callable) | Can_Tiberium_Germinate | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3195 | function (callable) | Set_Mapped | CellClass | (HousesType house,bool set) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3213 | function (callable) | Set_Mapped | CellClass | (HouseClass * player,bool set) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3230 | function (callable) | Is_Mapped | CellClass | (HousesType house) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3243 | function (callable) | Is_Mapped | CellClass | (HouseClass * player) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3258 | function (callable) | Set_Visible | CellClass | (HousesType house,bool set) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3276 | function (callable) | Set_Visible | CellClass | (HouseClass * player,bool set) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3293 | function (callable) | Is_Visible | CellClass | (HousesType house) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3306 | function (callable) | Is_Visible | CellClass | (HouseClass * player) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3314 | function (callable) | Is_Jamming | CellClass | (HousesType house) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3320 | function (callable) | Is_Jamming | CellClass | (HouseClass * player) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.CPP | 3328 | function (callable) | Override_Land_Type | CellClass | (LandType type) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 216 | function (callable) | CellClass | CellClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 217 | function (callable) | ~CellClass | CellClass | (void) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 219 | function (callable) | operator == | CellClass | (CellClass const & cell) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 228 | function (callable) | What_Am_I | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 230 | function (callable) | Cell_Number | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 233 | function (callable) | Free_Spot | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 234 | function (callable) | Adjacent_Cell | CellClass | (FacingType face) | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 237 | function (callable) | Land_Type | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 240 | function (callable) | Cell_Occupier | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 242 | function (callable) | As_Target | CellClass | (void) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 250 | function (callable) | Is_Spot_Free | CellClass | (int spot_index) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CELL.H | 300 | function (callable) | operator != | CellClass | (CellClass const &) const | RUST/original-source-reference/REDALERT/CELL.md | SIGNATURE_CATALOGUED |
| REDALERT/CHECKBOX.CPP | 55 | function (callable) | Draw_Me | CheckBoxClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHECKBOX.CPP | 89 | function (callable) | Action | CheckBoxClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHECKBOX.H | 43 | function (callable) | CheckBoxClass | CheckBoxClass | (unsigned id,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 70 | function (callable) | CheckListClass | CheckListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 92 | function (callable) | ~CheckListClass | CheckListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 117 | function (callable) | Add_Item | CheckListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 124 | function (callable) | Current_Item | CheckListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 149 | function (callable) | Get_Item | CheckListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 174 | function (callable) | Remove_Item | CheckListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 203 | function (callable) | Set_Selected_Index | CheckListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 232 | function (callable) | Check_Item | CheckListClass | (int index,bool checked) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 258 | function (callable) | Is_Checked | CheckListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 284 | function (callable) | Action | CheckListClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.CPP | 330 | function (callable) | Draw_Entry | CheckListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.H | 47 | function (callable) | CheckObject | CheckObject | (char const * text=0,bool checked=false) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.H | 67 | function (callable) | Add_Item | CheckListClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.H | 72 | function (callable) | Remove_Item | CheckListClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.H | 74 | function (callable) | Set_Selected_Index | CheckListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/CHEKLIST.H | 82 | function (callable) | Set_Read_Only | CheckListClass | (int rdonly) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 63 | function (callable) | ColorListClass | ColorListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 87 | function (callable) | ~ColorListClass | ColorListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 110 | function (callable) | Add_Item | ColorListClass | (char const * text,RemapControlType * color) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 133 | function (callable) | Add_Item | ColorListClass | (int text,RemapControlType * color) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 154 | function (callable) | Remove_Item | ColorListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 180 | function (callable) | Set_Selected_Style | ColorListClass | (SelectStyleType style,RemapControlType * color) |  | SIGNATURE_CATALOGUED |
| REDALERT/COLRLIST.CPP | 205 | function (callable) | Draw_Entry | ColorListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBAT.CPP | 69 | function (callable) | Modify_Damage |  | (int damage,WarheadType warhead,ArmorType armor,int distance) | RUST/original-source-reference/REDALERT/COMBAT.md | SIGNATURE_CATALOGUED |
| REDALERT/COMBAT.CPP | 159 | function (callable) | Explosion_Damage |  | (COORDINATE coord,int strength,TechnoClass * source,WarheadType warhead) | RUST/original-source-reference/REDALERT/COMBAT.md | SIGNATURE_CATALOGUED |
| REDALERT/COMBAT.CPP | 293 | function (callable) | Combat_Anim |  | (int damage,WarheadType warhead,LandType land) | RUST/original-source-reference/REDALERT/COMBAT.md | SIGNATURE_CATALOGUED |
| REDALERT/COMBAT.CPP | 391 | function (callable) | Wide_Area_Damage |  | (COORDINATE coord,LEPTON radius,int rawdamage,TechnoClass * source,WarheadType warhead) | RUST/original-source-reference/REDALERT/COMBAT.md | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 77 | function (callable) | CommBufferClass | CommBufferClass | (int numsend,int numreceive,int maxlen,int extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 142 | function (callable) | ~CommBufferClass | CommBufferClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 187 | function (callable) | Init | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 257 | function (callable) | Init_Send_Queue | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 302 | function (callable) | Queue_Send | CommBufferClass | (void * buf,int buflen,void * extrabuf,int extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 392 | function (callable) | UnQueue_Send | CommBufferClass | (void * buf,int * buflen,int index,void * extrabuf,int * extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 466 | function (callable) | Get_Send | CommBufferClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 496 | function (callable) | Queue_Receive | CommBufferClass | (void * buf,int buflen,void * extrabuf,int extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 585 | function (callable) | UnQueue_Receive | CommBufferClass | (void * buf,int * buflen,int index,void * extrabuf,int * extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 657 | function (callable) | Get_Receive | CommBufferClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 694 | function (callable) | Add_Delay | CommBufferClass | (unsigned long delay) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 733 | function (callable) | Avg_Response_Time | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 755 | function (callable) | Max_Response_Time | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 777 | function (callable) | Reset_Response_Time | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 811 | function (callable) | Configure_Debug | CommBufferClass | (int type_offset,int type_size,char ** names,int namestart,int namecount) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 840 | function (callable) | Mono_Debug_Print | CommBufferClass | (int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.CPP | 980 | function (callable) | Mono_Debug_Print2 | CommBufferClass | (int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 108 | function (callable) | Num_Send | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 109 | function (callable) | Max_Send | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 111 | function (callable) | Send_Total | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 120 | function (callable) | Num_Receive | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 121 | function (callable) | Max_Receive | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMBUF.H | 123 | function (callable) | Receive_Total | CommBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMINIT.CPP | 30 | function (callable) | ComInit | ComInit | () |  | SIGNATURE_CATALOGUED |
| REDALERT/COMINIT.CPP | 38 | function (callable) | ~ComInit | ComInit | () |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 48 | macro (function-like macro) | Size_Of_Region |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 53 | macro (function-like macro) | Set_Font_Palette |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 155 | function (callable) | Get_IconSet_MapWidth |  | (void const * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 163 | function (callable) | Get_IconSet_MapHeight |  | (void const * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 171 | function (callable) | Get_IconSet_ControlMap |  | (void const * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 185 | function (callable) | Map_Width | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 186 | function (callable) | Map_Height | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 187 | function (callable) | Control_Map | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 188 | function (callable) | Control_Map | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 189 | function (callable) | Icon_Count | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 190 | function (callable) | Pixel_Width | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 191 | function (callable) | Pixel_Height | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 192 | function (callable) | Total_Size | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 193 | function (callable) | Palette_Data | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 194 | function (callable) | Palette_Data | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 195 | function (callable) | Icon_Data | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 196 | function (callable) | Icon_Data | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 197 | function (callable) | Map_Data | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 198 | function (callable) | Map_Data | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 199 | function (callable) | Remap_Data | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 200 | function (callable) | Remap_Data | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 201 | function (callable) | Trans_Data | IconsetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/COMPAT.H | 202 | function (callable) | Trans_Data | IconsetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 74 | function (callable) | CommQueueClass | CommQueueClass | (int numsend,int numreceive,int maxlen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 121 | function (callable) | ~CommQueueClass | CommQueueClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 156 | function (callable) | Init | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 223 | function (callable) | Queue_Send | CommQueueClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 278 | function (callable) | UnQueue_Send | CommQueueClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 329 | function (callable) | Next_Send | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 358 | function (callable) | Get_Send | CommQueueClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 391 | function (callable) | Queue_Receive | CommQueueClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 444 | function (callable) | UnQueue_Receive | CommQueueClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 494 | function (callable) | Next_Receive | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 523 | function (callable) | Get_Receive | CommQueueClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 565 | function (callable) | Add_Delay | CommQueueClass | (unsigned long delay) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 603 | function (callable) | Avg_Response_Time | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 624 | function (callable) | Max_Response_Time | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 645 | function (callable) | Reset_Response_Time | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 678 | function (callable) | Configure_Debug | CommQueueClass | (int offset,int size,char ** names,int maxnames) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 705 | function (callable) | Mono_Debug_Print | CommQueueClass | (int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.CPP | 841 | function (callable) | Mono_Debug_Print2 | CommQueueClass | (int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 110 | function (callable) | Num_Send | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 111 | function (callable) | Max_Send | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 113 | function (callable) | Send_Total | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 121 | function (callable) | Num_Receive | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 122 | function (callable) | Max_Receive | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/COMQUEUE.H | 124 | function (callable) | Receive_Total | CommQueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONFDLG.CPP | 41 | function (callable) | Process | ConfirmationClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONFDLG.CPP | 62 | function (callable) | Process | ConfirmationClass | (char const * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONFDLG.H | 48 | function (callable) | ConfirmationClass | ConfirmationClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 89 | function (callable) | ConnectionClass | ConnectionClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout,int extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 151 | function (callable) | ~ConnectionClass | ConnectionClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 177 | function (callable) | Init | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 214 | function (callable) | Send_Packet | ConnectionClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 274 | function (callable) | Receive_Packet | ConnectionClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 477 | function (callable) | Get_Packet | ConnectionClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 554 | function (callable) | Service | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 591 | function (callable) | Service_Send_Queue | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 722 | function (callable) | Service_Receive_Queue | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 771 | function (callable) | Time | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.CPP | 822 | function (callable) | Command_Name | ConnectionClass | (int command) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 179 | function (callable) | Magic_Num | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 180 | function (callable) | Retry_Delta | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 181 | function (callable) | Set_Retry_Delta | ConnectionClass | (unsigned long delta) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 182 | function (callable) | Max_Retries | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 183 | function (callable) | Set_Max_Retries | ConnectionClass | (unsigned long retries) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 184 | function (callable) | Time_Out | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 185 | function (callable) | Set_TimeOut | ConnectionClass | (unsigned long t) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNECT.H | 186 | function (callable) | Max_Packet_Len | ConnectionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNMGR.H | 87 | function (callable) | ConnManClass | ConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONNMGR.H | 88 | function (callable) | ~ConnManClass | ConnManClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 212 | function (callable) | Main_Game |  | (int argc,char * argv[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 530 | function (callable) | Keyboard_Process |  | (KeyNumType & input) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 956 | function (callable) | Toggle_Formation |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1149 | function (callable) | Message_Input |  | (KeyNumType & input) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1533 | function (callable) | Color_Cycle |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1625 | function (callable) | Call_Back |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1701 | function (callable) | IPX_Call_Back |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1796 | function (callable) | TEN_Call_Back |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1856 | function (callable) | MPATH_Call_Back |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1933 | function (callable) | Language_Name |  | (char const * basename) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1961 | function (callable) | Source_From_Name |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 1989 | function (callable) | Name_From_Source |  | (SourceType source) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2013 | function (callable) | Theater_From_Name |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2045 | function (callable) | KN_To_Facing |  | (int input) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2097 | function (callable) | Sync_Delay |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2150 | function (callable) | Main_Loop |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2480 | function (callable) | Map_Edit_Loop |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2529 | function (callable) | Go_Editor |  | (bool flag) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2615 | function (callable) | MixFileHandler |  | (VQAHandle * vqa,long action,void * buffer,long nbytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2717 | function (callable) | Rebuild_Interpolated_Palette |  | (unsigned char * interpal) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2746 | function (callable) | Load_Interpolated_Palettes |  | (char const * filename,BOOL add) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2797 | function (callable) | Free_Interpolated_Palettes |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 2839 | function (callable) | Play_Movie |  | (char const * name,ThemeType theme,bool clrscrn,bool immediate) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3020 | function (callable) | Play_Movie |  | (VQType name,ThemeType theme,bool clrscrn,bool immediate) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3036 | function (callable) | PlayMpegMovie |  | (const char * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3136 | function (callable) | MpegCallback |  | (MPG_CMD cmd,LPVOID data,LPVOID user) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3225 | function (callable) | Unselect_All |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3247 | function (callable) | Unselect_All_Except |  | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3287 | function (callable) | Fading_Table_Name |  | (char const * base,TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3310 | function (callable) | Get_Radar_Icon |  | (void const * shapefile,int shapenum,int frames,int zoomfactor) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3433 | function (callable) | CC_Draw_Shape |  | (const ObjectClass * object,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,DirType rotation,long virtualscale,int width,int height) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3445 | function (callable) | CC_Draw_Shape |  | (const ObjectClass * object,const char * shape_file_name,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,DirType rotation,long virtualscale,char override_owner) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3458 | function (callable) | CC_Draw_Pip |  | (const ObjectClass * object,void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,DirType rotation) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3505 | function (callable) | CC_Draw_Shape |  | (void const * shapefile,int shapenum,int x,int y,WindowNumberType window,ShapeFlags_Type flags,void const * fadingdata,void const * ghostdata,DirType rotation) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3671 | function (callable) | Shape_Dimensions |  | (void const * shapedata,int shapenum) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3783 | function (callable) | Fetch_Techno_Type |  | (RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3831 | function (callable) | VQA_PauseAudio |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3836 | function (callable) | VQ_Call_Back |  | (unsigned char *,long) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3875 | function (callable) | VQ_Call_Back |  | (unsigned char *,long) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 3912 | function (callable) | Handle_Team |  | (int team,int action) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4201 | function (callable) | Handle_View |  | (int view,int action) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4261 | function (callable) | Get_CD_Index |  | (int cd_drive,int timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4333 | function (callable) | Get_CD_Index |  | (int cd_drive,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4430 | function (callable) | Force_CD_Available |  | (int cd) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4495 | function (callable) | Force_CD_Available |  | (int cd_desired) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 4806 | function (callable) | Force_CD_Available |  | (int cd) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5225 | function (callable) | Disk_Space_Available |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5255 | function (callable) | Do_Record_Playback |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5406 | function (callable) | Hires_Load |  | (char * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5447 | function (callable) | Crate_From_Name |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5475 | function (callable) | Owner_From_Name |  | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5509 | function (callable) | Shake_The_Screen |  | (int shakes,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5577 | function (callable) | List_Copy |  | (short const * source,int len,short * dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5614 | function (callable) | Game_Registry_Key |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5643 | function (callable) | Is_Counterstrike_Installed |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5677 | function (callable) | Is_Aftermath_Installed |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5710 | function (callable) | Enable_Secret_Units |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONQUER.CPP | 5724 | function (callable) | Force_Scenario_Available |  | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 71 | function (callable) | ControlClass | ControlClass | (unsigned id,int x,int y,int w,int h,unsigned flags,int sticky) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 94 | function (callable) | ControlClass | ControlClass | (ControlClass const & control) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 122 | function (callable) | Action | ControlClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 166 | function (callable) | Make_Peer | ControlClass | (GadgetClass & gadget) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 189 | function (callable) | Get_ID | ControlClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.CPP | 217 | function (callable) | Draw_Me | ControlClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/CONTROL.H | 58 | function (callable) | ControlClass | ControlClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 69 | function (callable) | Coord_Cell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 98 | function (callable) | Distance |  | (TARGET target1,TARGET target2) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 121 | function (callable) | Distance |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 160 | function (callable) | Coord_Spillage_List |  | (COORDINATE coord,int maxsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 266 | function (callable) | Coord_Spillage_List |  | (COORDINATE coord,Rect const & rect,bool nocenter) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 348 | function (callable) | Coord_Move |  | (COORDINATE start,register DirType dir,unsigned short distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 387 | function (callable) | Coord_Scatter |  | (COORDINATE coord,unsigned distance,bool lock) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 403 | function (callable) | calcx |  | (signed short param1,short distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 421 | function (callable) | calcy |  | (signed short param1,short distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 468 | function (callable) | Move_Point |  | (short & x,short & y,register DirType dir,unsigned short distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORD.CPP | 616 | function (callable) | Normal_Move_Point |  | (short & x,short & y,register DirType dir,unsigned short distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/COORDA.ASM | 36 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/COORDA.ASM | 37 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/COORDA.ASM | 38 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/CPUID.ASM | 73 | label (assembly label candidate) | Detect_MMX_Availability |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.CPP | 56 | function (callable) | Remove_It | CrateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.CPP | 83 | function (callable) | Create_Crate | CrateClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.CPP | 119 | function (callable) | Put_Crate | CrateClass | (CELL & cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.CPP | 166 | function (callable) | Get_Crate | CrateClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 57 | function (callable) | CrateClass | CrateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 58 | function (callable) | Init | CrateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 60 | function (callable) | Is_Here | CrateClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 62 | function (callable) | Is_Expired | CrateClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 63 | function (callable) | Is_Valid | CrateClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRATE.H | 69 | function (callable) | Make_Invalid | CrateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.CPP | 58 | function (callable) | operator ()  | CRCEngine | (char datum) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.CPP | 88 | function (callable) | operator ()  | CRCEngine | (void const * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.H | 65 | function (callable) | CRCEngine | CRCEngine | (long initial=0) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.H | 70 | function (callable) | operator ()  | CRCEngine | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.H | 79 | function (callable) | operator long | CRCEngine | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.H | 83 | function (callable) | Buffer_Needs_Data | CRCEngine | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRC.H | 87 | function (callable) | Value | CRCEngine | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCPIPE.CPP | 61 | function (callable) | Put | CRCPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCPIPE.CPP | 83 | function (callable) | Result | CRCPipe | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCPIPE.H | 49 | function (callable) | CRCPipe | CRCPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCSTRAW.CPP | 61 | function (callable) | Get | CRCStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCSTRAW.CPP | 88 | function (callable) | Result | CRCStraw | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/CRCSTRAW.H | 49 | function (callable) | CRCStraw | CRCStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREDITS.CPP | 56 | function (callable) | CreditClass | CreditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREDITS.CPP | 86 | function (callable) | Graphic_Logic | CreditClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREDITS.CPP | 192 | function (callable) | AI | CreditClass | (bool forced,HouseClass * player_ptr,bool logic_only) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREDITS.H | 52 | function (callable) | CreditClass | CreditClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREW.H | 55 | function (callable) | CrewClass | CrewClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREW.H | 56 | function (callable) | CrewClass | CrewClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREW.H | 57 | function (callable) | ~CrewClass | CrewClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CREW.H | 59 | function (callable) | Made_A_Kill | CrewClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/CSTRAW.CPP | 61 | function (callable) | Get | CacheStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/CSTRAW.H | 51 | function (callable) | CacheStraw | CacheStraw | (Buffer const & buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/CSTRAW.H | 52 | function (callable) | CacheStraw | CacheStraw | (int length=4096) |  | SIGNATURE_CATALOGUED |
| REDALERT/CSTRAW.H | 60 | function (callable) | Is_Valid | CacheStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 80 | function (callable) | Instance_Class | Instance_Class | (LPSTR name1,LPSTR name2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 146 | function (callable) | ~Instance_Class | Instance_Class | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 168 | function (callable) | Enable_Callback | Instance_Class | (BOOL flag) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 191 | function (callable) | Register_Server | Instance_Class | (BOOL CALLBACK (* callback_fnc)(LPBYTE,long)) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 222 | function (callable) | Test_Server_Running | Instance_Class | (HSZ name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 251 | function (callable) | Open_Poke_Connection | Instance_Class | (HSZ name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 283 | function (callable) | Close_Poke_Connection | Instance_Class | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 314 | function (callable) | Poke_Server | Instance_Class | (LPBYTE poke_data,DWORD poke_length) |  | SIGNATURE_CATALOGUED |
| REDALERT/DDE.CPP | 358 | function (callable) | dde_callback | Instance_Class | (UINT dde_event,UINT uFmt,HCONV,HSZ hsz1,HSZ hsz2,HDDEDATA hdata,DWORD,DWORD) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEBUG.CPP | 66 | function (callable) | Debug_Key |  | (unsigned input) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEBUG.CPP | 364 | function (callable) | Bench_Time |  | (BenchType btype) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEBUG.CPP | 402 | function (callable) | Benchmarks |  | (MonoClass * mono) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEBUG.CPP | 471 | function (callable) | Self_Regulate |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 605 | function (callable) | Build_Target |  | (RTTIType kind,int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 944 | function (callable) | operator \| |  | (ThreatType a,ThreatType b) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 947 | function (callable) | operator & |  | (ThreatType a,ThreatType b) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 950 | function (callable) | operator ~ |  | (ThreatType a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 1198 | function (callable) | operator ++ |  | (HousesType & ht) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 1309 | function (callable) | operator ++ |  | (ScenarioVarType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 1329 | function (callable) | operator ++ |  | (LayerType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 1363 | function (callable) | operator ++ |  | (BulletType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 1480 | function (callable) | operator ++ |  | (StructType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2156 | function (callable) | operator ++ |  | (TemplateType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2819 | macro (function-like macro) | BStart |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2820 | macro (function-like macro) | BEnd |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2822 | macro (function-like macro) | BStart |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2823 | macro (function-like macro) | BEnd |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2883 | function (callable) | operator \| |  | (TextPrintType a,TextPrintType b) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2886 | function (callable) | operator & |  | (TextPrintType a,TextPrintType b) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2889 | function (callable) | operator ~ |  | (TextPrintType a) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 2957 | function (callable) | operator ++ |  | (TheaterType & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3049 | function (callable) | operator + |  | (FacingType f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3053 | function (callable) | operator + |  | (FacingType f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3058 | function (callable) | operator - |  | (FacingType f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3062 | function (callable) | operator - |  | (FacingType f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3067 | function (callable) | operator += |  | (FacingType & f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3072 | function (callable) | operator += |  | (FacingType & f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3078 | function (callable) | operator * |  | (FacingType f1,FacingType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3099 | function (callable) | operator + |  | (DirType f1,DirType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3103 | function (callable) | operator + |  | (DirType f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DEFINES.H | 3631 | macro (function-like macro) | size_of |  | (typ,id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DESCDLG.CPP | 51 | function (callable) | Process | DescriptionClass | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/DESCDLG.H | 60 | function (callable) | DescriptionClass | DescriptionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIAL8.CPP | 61 | function (callable) | Dial8Class | Dial8Class | (int id,int x,int y,int w,int h,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIAL8.CPP | 149 | function (callable) | Action | Dial8Class | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIAL8.CPP | 230 | function (callable) | Draw_Me | Dial8Class | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIAL8.CPP | 289 | function (callable) | Get_Direction | Dial8Class | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DIAL8.CPP | 310 | function (callable) | Set_Direction | Dial8Class | (DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 71 | function (callable) | Dialog_Box |  | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 190 | function (callable) | Draw_Box |  | (int x,int y,int w,int h,BoxStyleEnum up,bool filled) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 255 | function (callable) | Format_Window_String |  | (char * string,int maxlinelen,int & width,int & height) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 334 | function (callable) | Window_Box |  | (WindowNumberType window,BoxStyleEnum style) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 392 | function (callable) | Simple_Text_Print |  | (char const * text,unsigned x,unsigned y,RemapControlType * fore,unsigned back,TextPrintType flag) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 662 | function (callable) | Fancy_Text_Print |  | (int text,unsigned x,unsigned y,RemapControlType * fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 719 | function (callable) | Fancy_Text_Print |  | (char const * text,unsigned x,unsigned y,RemapControlType * fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 779 | function (callable) | Conquer_Clip_Text_Print |  | (char const * text,unsigned x,unsigned y,RemapControlType * fore,unsigned back,TextPrintType flag,int width,int const * tabs) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 879 | function (callable) | Plain_Text_Print |  | (int text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 923 | function (callable) | Plain_Text_Print |  | (char const * text,unsigned x,unsigned y,unsigned fore,unsigned back,TextPrintType flag,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 946 | function (callable) | Font_Palette |  | (int color) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 975 | function (callable) | Draw_Caption |  | (int text,int x,int y,int w) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIALOG.CPP | 981 | function (callable) | Draw_Caption |  | (char const * text,int x,int y,int w) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBAPI.H | 82 | macro (function-like macro) | WIDTHBYTES |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 100 | function (callable) | LoadDIB |  | (LPSTR lpFileName) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 162 | function (callable) | SaveDIB |  | (HDIB hDib,LPSTR lpFileName) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 289 | function (callable) | DestroyDIB |  | (HDIB hDib) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 330 | function (callable) | ReadDIBFile |  | (int hFile) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 464 | function (callable) | MyRead |  | (int hFile,LPSTR lpBuffer,DWORD dwSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 503 | function (callable) | MyWrite |  | (int iFileHandle,VOID FAR * lpBuffer,DWORD dwBytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBFILE.CPP | 530 | function (callable) | LoadDIB_FromMemory |  | (const unsigned char * pData,DWORD dwBitsSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 98 | function (callable) | CreateDIB |  | (DWORD dwWidth,DWORD dwHeight,WORD wBitCount) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 190 | function (callable) | FindDIBBits |  | (LPCSTR lpDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 221 | function (callable) | DIBWidth |  | (LPCSTR lpDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 264 | function (callable) | DIBHeight |  | (LPCSTR lpDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 308 | function (callable) | PaletteSize |  | (LPCSTR lpDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 344 | function (callable) | DIBNumColors |  | (LPCSTR lpDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 417 | function (callable) | CreateDIBPalette |  | (HDIB hDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 559 | function (callable) | DIBToBitmap |  | (HDIB hDIB,HPALETTE hPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 635 | function (callable) | BitmapToDIB |  | (HBITMAP hBitmap,HPALETTE hPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 797 | function (callable) | PalEntriesOnDevice |  | (HDC hDC) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 845 | function (callable) | GetSystemPalette |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 923 | function (callable) | AllocRoomForDIB |  | (BITMAPINFOHEADER bi,HBITMAP hBitmap) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 1022 | function (callable) | ChangeDIBFormat |  | (HDIB hDIB,WORD wBitCount,DWORD dwCompression) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.CPP | 1195 | function (callable) | ChangeBitmapFormat |  | (HBITMAP hBitmap,WORD wBitCount,DWORD dwCompression,HPALETTE hPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.H | 33 | macro (function-like macro) | IS_WIN30_DIB |  | (lpbi) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.H | 34 | macro (function-like macro) | RECTWIDTH |  | (lpRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/DIBUTIL.H | 35 | macro (function-like macro) | RECTHEIGHT |  | (lpRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 162 | function (callable) | DisplayClass | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 213 | function (callable) | One_Time | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 263 | function (callable) | Init_Clear | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 304 | function (callable) | Init_IO | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 333 | function (callable) | Init_Theater | DisplayClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 490 | function (callable) | Text_Overlap_List | DisplayClass | (char const * text,int x,int y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 563 | function (callable) | Set_View_Dimensions | DisplayClass | (int x,int y,int width,int height) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 632 | function (callable) | Set_Cursor_Shape | DisplayClass | (short const * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 683 | function (callable) | Passes_Proximity_Check | DisplayClass | (ObjectTypeClass const * object,HousesType house,short const * list,CELL trycell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 852 | function (callable) | Set_Cursor_Pos | DisplayClass | (CELL pos) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 936 | function (callable) | Get_Occupy_Dimensions | DisplayClass | (int & w,int & h,short const * list) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 994 | function (callable) | Cursor_Mark | DisplayClass | (CELL pos,bool on) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1059 | function (callable) | AI | DisplayClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1097 | function (callable) | Submit | DisplayClass | (ObjectClass const * object,LayerType layer) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1123 | function (callable) | Remove | DisplayClass | (ObjectClass const * object,LayerType layer) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1152 | function (callable) | Click_Cell_Calc | DisplayClass | (int x,int y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1192 | function (callable) | Scroll_Map | DisplayClass | (DirType facing,int & distance,bool really) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1297 | function (callable) | Refresh_Cells | DisplayClass | (CELL cell,short const * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1340 | function (callable) | Cell_Shadow | DisplayClass | (CELL cell,HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1451 | function (callable) | Cell_Shadow | DisplayClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1564 | function (callable) | Map_Cell | DisplayClass | (CELL cell,HouseClass * house,bool check_radar_spied,bool and_for_allies) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1746 | function (callable) | Coord_To_Pixel | DisplayClass | (COORDINATE coord,int & x,int & y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1764 | function (callable) | Coord_To_Pixel | DisplayClass | (COORDINATE coord,int & x,int & y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1804 | function (callable) | Push_Onto_TacMap | DisplayClass | (COORDINATE & source,COORDINATE & dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1853 | function (callable) | Cell_Object | DisplayClass | (CELL cell,int x,int y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 1895 | function (callable) | Draw_It | DisplayClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2355 | function (callable) | Redraw_Icons | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2399 | function (callable) | Redraw_OIcons | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2449 | function (callable) | Redraw_Shadow | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2510 | function (callable) | Next_Object | DisplayClass | (ObjectClass * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2550 | function (callable) | Prev_Object | DisplayClass | (ObjectClass * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2592 | function (callable) | Pixel_To_Coord | DisplayClass | (int x,int y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2648 | function (callable) | Calculated_Cell | DisplayClass | (SourceType dir,WAYPOINT waypoint,CELL cell,SpeedType loco,bool zonecheck,MZoneType mzone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2811 | function (callable) | Good_Reinforcement_Cell | DisplayClass | (CELL outcell,CELL incell,SpeedType loco,int zone,MZoneType mzone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2866 | function (callable) | should_exclude_from_selection |  | (ObjectClass * obj) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 2873 | function (callable) | Select_These | DisplayClass | (COORDINATE coord1,COORDINATE coord2,bool additive) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3034 | function (callable) | Refresh_Band | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3108 | function (callable) | Action | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3341 | function (callable) | Selection_At_Mouse | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3419 | function (callable) | Command_Object | DisplayClass::TacticalClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3493 | function (callable) | Mouse_Right_Press | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3544 | function (callable) | Mouse_Left_Up | DisplayClass | (CELL cell,bool shadow,ObjectClass * object,ActionType action,bool wsmall) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 3874 | function (callable) | Mouse_Left_Release | DisplayClass | (CELL cell,int x,int y,ObjectClass * object,ActionType action,bool wsmall) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4147 | function (callable) | Mouse_Left_Press | DisplayClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4174 | function (callable) | Mouse_Left_Held | DisplayClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4243 | function (callable) | Set_Tactical_Position | DisplayClass | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4281 | function (callable) | Compute_Start_Pos | DisplayClass | (long & x,long & y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4362 | function (callable) | Sell_Mode_Control | DisplayClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4409 | function (callable) | Repair_Mode_Control | DisplayClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4456 | function (callable) | In_View | DisplayClass | (register CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4487 | function (callable) | Closest_Free_Spot | DisplayClass | (COORDINATE coord,bool any) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4512 | function (callable) | Is_Spot_Free | DisplayClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4540 | function (callable) | Center_Map | DisplayClass | (COORDINATE center) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4604 | function (callable) | Encroach_Shadow | DisplayClass | (HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4655 | function (callable) | Shroud_Cell | DisplayClass | (CELL cell,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4714 | function (callable) | Read_INI | DisplayClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4866 | function (callable) | Write_INI | DisplayClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4946 | function (callable) | All_To_Look | DisplayClass | (HouseClass * house,bool units_only) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 4975 | function (callable) | Constrained_Look | DisplayClass | (COORDINATE center,LEPTON distance,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5032 | function (callable) | Flag_Cell | DisplayClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5049 | function (callable) | get_action_priority |  | (ActionType action) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5060 | function (callable) | index_of |  | (const DynamicVectorClass<T * > & list,T * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5071 | function (callable) | Best_Object_With_ActionT |  | (DynamicVectorClass<ObjectClass * > & objects,T subject) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5100 | function (callable) | Best_Object_With_Action |  | (DynamicVectorClass<ObjectClass * > & objects,const ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5105 | function (callable) | Best_Object_With_Action |  | (DynamicVectorClass<ObjectClass * > & objects,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5110 | function (callable) | Best_Object_Action |  | (DynamicVectorClass<ObjectClass * > & objects,const ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5116 | function (callable) | Best_Object_Action |  | (DynamicVectorClass<ObjectClass * > & objects,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5122 | function (callable) | Best_Object_With_Action |  | (const ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5127 | function (callable) | Best_Object_With_Action |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5132 | function (callable) | Best_Object_Action |  | (const ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.CPP | 5137 | function (callable) | Best_Object_Action |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.H | 123 | function (callable) | DisplayClass | DisplayClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.H | 152 | function (callable) | Help_Text | DisplayClass | (int,int=-1,int=-1,int=YELLOW,bool=false) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.H | 161 | function (callable) | Put_Place_Back | DisplayClass | (TechnoClass *) |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.H | 192 | function (callable) | Is_Cell_Flagged | DisplayClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DISPLAY.H | 273 | function (callable) | TacticalClass | DisplayClass::TacticalClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 221 | function (callable) | Set_Event_Callback | DLLExportClass | (CNC_Event_Callback_Type event_callback) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 274 | function (callable) | Get_Shadow_Shapes | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 275 | function (callable) | Get_Shadow_Trans | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 289 | function (callable) | Get_Game_Over | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 395 | function (callable) | GetRandSeed |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 406 | function (callable) | Play_Movie_GlyphX |  | (const char * movie_name,ThemeType theme,bool immediate=false) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 415 | function (callable) | Display_Briefing_Text_GlyphX |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 421 | function (callable) | On_Sound_Effect |  | (int sound_index,int variation,COORDINATE coord,int house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 534 | function (callable) | On_Speech |  | (int speech_index,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 546 | function (callable) | On_Message |  | (const char * message,float timeout_seconds,int64 id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 551 | function (callable) | On_Update_Map_Cell |  | (int cell_x,int cell_y,const char * template_type_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 556 | function (callable) | On_Special_Weapon_Targetting |  | (const HouseClass * player_ptr,SpecialWeaponType weapon_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 561 | function (callable) | On_Ping |  | (const HouseClass * player_ptr,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 566 | function (callable) | On_Defeated_Message |  | (const char * message,float timeout_seconds) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 571 | function (callable) | GlyphX_Debug_Print |  | (const char * debug_text) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 576 | function (callable) | On_Achievement_Event |  | (const HouseClass * player_ptr,const char * achievement_type,const char * achievement_reason) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 596 | function (callable) | CNC_Version |  | (unsigned int version_in) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 617 | function (callable) | CNC_Init |  | (const char * command_line,CNC_Event_Callback_Type event_callback) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 641 | function (callable) | DLL_Shutdown |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 661 | function (callable) | CNC_Config |  | (const CNCRulesDataStruct & rules) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 680 | function (callable) | CNC_Add_Mod_Path |  | (const char * mod_path) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 700 | function (callable) | CNC_Get_Visible_Page |  | (unsigned char * buffer_in,unsigned int & width,unsigned int & height) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 742 | function (callable) | CNC_Get_Palette |  | (unsigned char (& palette_in)[256][3]) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 762 | function (callable) | CNC_Set_Multiplayer_Data |  | (int scenario_index,CNCMultiplayerOptionsStruct & game_options,int num_players,CNCPlayerInfoStruct * player_list,int max_players) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 861 | function (callable) | CNC_Clear_Object_Selection |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 872 | function (callable) | CNC_Select_Object |  | (uint64 player_id,int object_type_id,int object_to_select_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 996 | function (callable) | GlyphX_Assign_Houses |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1196 | function (callable) | CNC_Set_Home_Cell |  | (int x,int y,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1213 | function (callable) | CNC_Start_Instance |  | (int scenario_index,int build_level,const char * faction,const char * game_type,const char * content_directory,int sabotaged_structure,const char * override_map_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1231 | function (callable) | CNC_Start_Instance_Variation |  | (int scenario_index,int scenario_variation,int scenario_direction,int build_level,const char * faction,const char * game_type,const char * content_directory,int sabotaged_structure,const char * override_map_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1385 | function (callable) | CNC_Read_INI |  | (int scenario_index,int scenario_variation,int scenario_direction,const char * content_directory,const char * override_map_name,char * ini_buffer,int _ini_buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1461 | function (callable) | CNC_Start_Custom_Instance |  | (const char * content_directory,const char * directory_path,const char * scenario_name,int build_level,bool multiplayer) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1605 | function (callable) | Debug_Write_Shape_Type |  | (const ObjectTypeClass * type,int shapenum) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1623 | function (callable) | Debug_Write_Shape |  | (const char * file_name,void const * shapefile,int shapenum,int flags,void const * ghostdata) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1671 | function (callable) | CNC_Advance_Instance |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1913 | function (callable) | CNC_Save_Load |  | (bool save,const char * file_path_and_name,const char * game_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1974 | function (callable) | CNC_Set_Difficulty |  | (int difficulty) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 1987 | function (callable) | CNC_Restore_Carryover_Objects |  | (const CarryoverObjectStruct * objects) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2032 | function (callable) | CNC_Handle_Player_Switch_To_AI |  | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2121 | function (callable) | CNC_Handle_Human_Team_Wins |  | (uint64 quitting_player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2133 | function (callable) | CNC_Start_Mission_Timer |  | (int time) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2153 | function (callable) | CNC_Get_Start_Game_Info |  | (uint64 player_id,int & start_location_waypoint_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2176 | function (callable) | Is_Legacy_Render_Enabled |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2193 | function (callable) | Init | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2221 | function (callable) | Shutdown | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2247 | function (callable) | Add_Mod_Path | DLLExportClass | (const char * mod_path) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2267 | function (callable) | Set_Content_Directory | DLLExportClass | (const char * content_directory) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2304 | function (callable) | Config | DLLExportClass | (const CNCRulesDataStruct & rules) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2329 | function (callable) | Set_Home_Cell | DLLExportClass | (int x,int y,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2348 | function (callable) | On_Play_Movie | DLLExportClass | (const char * movie_name,ThemeType theme,bool immediate) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2370 | function (callable) | On_Display_Briefing_Text | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2392 | function (callable) | On_Sound_Effect | DLLExportClass | (const HouseClass * player_ptr,int sound_effect_index,const char * extension,int variation,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2456 | function (callable) | On_Speech | DLLExportClass | (const HouseClass * player_ptr,int speech_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2491 | function (callable) | RA_Calculate_Leadership | DLLExportClass | (HousesType player_house,int units_lost,int buildings_lost) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2520 | function (callable) | RA_Calculate_Economy | DLLExportClass | (long available_money,int stolen_buildings_credits,unsigned harvested_credits,long initial_credits) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2533 | function (callable) | RA_Calculate_Score | DLLExportClass | (int uspoints,int leadership,int economy) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2547 | function (callable) | Calculate_Single_Player_Score | DLLExportClass | (EventCallbackStruct & event) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2660 | function (callable) | On_Message | DLLExportClass | (const HouseClass * player_ptr,const char * message,float timeout_seconds,EventCallbackMessageEnum message_type,int64 message_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2721 | function (callable) | On_Update_Map_Cell | DLLExportClass | (int cell_x,int cell_y,const char * template_type_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2746 | function (callable) | On_Special_Weapon_Targetting | DLLExportClass | (const HouseClass * player_ptr,SpecialWeaponType weapon_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2776 | function (callable) | On_Ping | DLLExportClass | (const HouseClass * player_ptr,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2804 | function (callable) | On_Game_Over | DLLExportClass | (uint64 glyphx_Player_id,bool player_wins) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 2886 | function (callable) | On_Multiplayer_Game_Over | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3034 | function (callable) | On_Achievement | DLLExportClass | (const HouseClass * player_ptr,const char * achievement_type,const char * achievement_reason) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3054 | function (callable) | On_Center_Camera | DLLExportClass | (const HouseClass * player_ptr,int coord_x,int coord_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3085 | function (callable) | On_Debug_Output | DLLExportClass | (const char * debug_text) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3103 | function (callable) | Force_Human_Team_Wins | DLLExportClass | (uint64 quitting_player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3148 | function (callable) | CNC_Get_Game_State |  | (GameStateRequestEnum state_type,uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3284 | function (callable) | CNC_Handle_Game_Request |  | (GameRequestEnum request_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3293 | function (callable) | CNC_Handle_Game_Settings_Request |  | (int health_bar_display_mode,int resource_bar_display_mode) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3317 | function (callable) | DLL_Draw_Intercept |  | (int shape_number,int x,int y,int width,int height,int flags,const ObjectClass * object,DirType rotation,long scale,const char * shape_file_name,char override_owner) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3323 | function (callable) | DLL_Draw_Pip_Intercept |  | (const ObjectClass * object,int pip) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3329 | function (callable) | DLL_Draw_Intercept | DLLExportClass | (int shape_number,int x,int y,int width,int height,int flags,const ObjectClass * object,DirType rotation,long scale,const char * shape_file_name,char override_owner) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3680 | function (callable) | DLL_Draw_Pip_Intercept | DLLExportClass | (const ObjectClass * object,int pip) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3712 | function (callable) | Get_Layer_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3863 | function (callable) | Convert_Type | DLLExportClass | (const ObjectClass * object,CNCObjectStruct & object_out) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 3944 | function (callable) | CNC_Handle_Input |  | (InputRequestEnum input_event,unsigned char special_key_flags,uint64 player_id,int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4176 | function (callable) | CNC_Handle_Structure_Request |  | (StructureRequestEnum request_type,uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4218 | function (callable) | CNC_Handle_Unit_Request |  | (UnitRequestEnum request_type,uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4271 | function (callable) | CNC_Handle_Sidebar_Request |  | (SidebarRequestEnum request_type,uint64 player_id,int buildable_type,int buildable_id,short cell_x,short cell_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4324 | function (callable) | CNC_Handle_SuperWeapon_Request |  | (SuperWeaponRequestEnum request_type,uint64 player_id,int buildable_type,int buildable_id,int x1,int y1) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4350 | function (callable) | CNC_Handle_ControlGroup_Request |  | (ControlGroupRequestEnum request_type,uint64 player_id,unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4386 | function (callable) | Get_Sidebar_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4809 | function (callable) | Convert_Action_Type | DLLExportClass | (ActionType type,ObjectClass * object,TARGET target,DllActionTypeEnum & dll_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4875 | function (callable) | Convert_Special_Weapon_Type | DLLExportClass | (SpecialWeaponType weapon_type,DllSuperweaponTypeEnum & dll_weapon_type,char * weapon_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4953 | function (callable) | Fill_Sidebar_Entry_From_Special_Weapon | DLLExportClass | (CNCSidebarEntryStruct & sidebar_entry_out,SuperClass * & super_weapon_out,SpecialWeaponType weapon_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 4982 | function (callable) | Calculate_Placement_Distances | DLLExportClass | (BuildingTypeClass * placement_type,unsigned char * placement_distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5043 | function (callable) | Recalculate_Placement_Distances |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5048 | function (callable) | Recalculate_Placement_Distances | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5066 | function (callable) | Get_Placement_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5137 | function (callable) | Passes_Proximity_Check | DLLExportClass | (CELL cell_in,BuildingTypeClass * placement_type,unsigned char * placement_distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5175 | function (callable) | Start_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5201 | function (callable) | Hold_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5225 | function (callable) | Cancel_Construction | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5251 | function (callable) | Construction_Action | DLLExportClass | (SidebarRequestEnum construction_action,uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5436 | function (callable) | MP_Construction_Action | DLLExportClass | (SidebarRequestEnum construction_action,uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5659 | function (callable) | Start_Placement | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5702 | function (callable) | Cancel_Placement | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5735 | function (callable) | Place | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id,short cell_x,short cell_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 5794 | function (callable) | Get_Pending_Placement_Object | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6002 | function (callable) | Place_Super_Weapon | DLLExportClass | (uint64 player_id,int buildable_type,int buildable_id,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6024 | function (callable) | Create_Control_Group | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6036 | function (callable) | Add_To_Control_Group | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6048 | function (callable) | Toggle_Control_Group_Selection | DLLExportClass | (unsigned char control_group_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6067 | function (callable) | Get_Shroud_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6190 | function (callable) | Get_Occupier_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6273 | function (callable) | Get_Player_Info_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6413 | function (callable) | Get_Dynamic_Map_State | DLLExportClass | (uint64 player_id,unsigned char * buffer_in,unsigned int buffer_size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6539 | function (callable) | Cell_Class_Draw_It | DLLExportClass | (CNCDynamicMapStruct * dynamic_map,int & entry_index,CellClass * cell_ptr,int xpixel,int ypixel,bool debug_output) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6676 | function (callable) | Glyphx_Queue_AI | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6767 | function (callable) | Test_CC |  | (CarryoverObjectStruct * cptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6788 | function (callable) | Store_Carryover_Objects | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6935 | function (callable) | Reset_Sidebars | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 6963 | function (callable) | Set_Player_Context | DLLExportClass | (uint64 glyphx_player_id,bool force) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7016 | function (callable) | Reset_Player_Context | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7036 | function (callable) | Refresh_Player_Control_Flags | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7065 | function (callable) | Logic_Switch_Player_Context |  | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7082 | function (callable) | Logic_Switch_Player_Context | DLLExportClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7114 | function (callable) | Logic_Switch_Player_Context |  | (HouseClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7131 | function (callable) | Logic_Switch_Player_Context | DLLExportClass | (HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7182 | function (callable) | Calculate_Start_Positions | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7221 | function (callable) | Get_GlyphX_Player_ID | DLLExportClass | (const HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7267 | function (callable) | Adjust_Internal_View | DLLExportClass | (bool force_ignore_view_constraints) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7301 | function (callable) | Get_Current_Context_Sidebar | DLLExportClass | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7315 | function (callable) | Get_Current_Context_Sidebar |  | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7332 | function (callable) | Repair_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7355 | function (callable) | Repair | DLLExportClass | (uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7393 | function (callable) | Sell_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7416 | function (callable) | Sell | DLLExportClass | (uint64 player_id,int object_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7453 | function (callable) | Repair_Sell_Cancel | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7469 | function (callable) | Scatter_Selected | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7494 | function (callable) | Select_Next_Unit | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7521 | function (callable) | Select_Previous_Unit | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7548 | function (callable) | Selected_Guard_Mode | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7577 | function (callable) | Selected_Stop | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7603 | function (callable) | Units_Queued_Movement_Toggle | DLLExportClass | (uint64 player_id,bool toggle) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7627 | function (callable) | Team_Units_Formation_Toggle_On | DLLExportClass | (uint64 player_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7844 | function (callable) | CNC_Handle_Debug_Request |  | (DebugRequestEnum debug_request_type,uint64 player_id,const char * object_name,int x,int y,bool unshroud,bool enemy) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7933 | function (callable) | CNC_Handle_Beacon_Request |  | (BeaconRequestEnum beacon_request_type,uint64 player_id,int pixel_x,int pixel_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 7979 | function (callable) | Try_Debug_Spawn_Unlimbo | DLLExportClass | (TechnoClass * techno,int & cell_x,int & cell_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8035 | function (callable) | Debug_Spawn_All | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8189 | function (callable) | Debug_Spawn_Unit | DLLExportClass | (const char * object_name,int x,int y,bool enemy) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8325 | function (callable) | Debug_Kill_Unit | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8359 | function (callable) | Debug_Heal_Unit | DLLExportClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8455 | function (callable) | Legacy_Render_Enabled | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8484 | function (callable) | Computer_Message | DLLExportClass | (bool last_player_taunt) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8534 | function (callable) | Set_Special_Key_Flags | DLLExportClass | (unsigned char special_key_flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8550 | function (callable) | Clear_Special_Key_Flags | DLLExportClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8566 | function (callable) | Get_Input_Key_State | DLLExportClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8592 | function (callable) | DLL_Export_Get_Input_Key_State |  | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8600 | function (callable) | DLLSave |  | (Pipe & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8605 | function (callable) | DLLLoad |  | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8622 | function (callable) | Save | DLLExportClass | (Pipe & pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8689 | function (callable) | Load | DLLExportClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8790 | function (callable) | Code_Pointers | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8814 | function (callable) | Decode_Pointers | DLLExportClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8837 | function (callable) | DLL_Code_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.cpp | 8842 | function (callable) | DLL_Decode_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.h | 580 | function (callable) | GameOverMultiPlayerStatsStruct | GameOverMultiPlayerStatsStruct | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.h | 616 | function (callable) | EventCallbackStruct | EventCallbackStruct::EventCallbackStruct | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterface.h | 919 | function (callable) | CarryoverObjectStruct | CarryoverObjectStruct | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 60 | function (callable) | CNC_Editor_Startup |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 86 | function (callable) | CNC_Editor_Cleanup |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 115 | function (callable) | CNC_Editor_Load_Mix_Files |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 164 | function (callable) | CNC_Editor_Setup_Content_Directory |  | (char * cncdata_directory,int CD,char (& content_directory)[_MAX_PATH]) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 205 | function (callable) | CNC_Editor_Load_Map |  | (char * cncdata_directory,char * house_name,int scenario_index,char * east_west,char * variant) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 332 | function (callable) | CNC_Editor_Load_Map_By_Scenario_Name |  | (char * cncdata_directory,char * scenario_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 359 | function (callable) | LoadScenario |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 407 | function (callable) | CNC_Editor_Clear_Map |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 434 | function (callable) | CNC_Editor_Get_Map_Stats |  | (int & map_width,int & map_height,int & theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 461 | function (callable) | CNC_Editor_Get_Cell_Data_By_Index |  | (int cell_index,char * cell_name,unsigned long cell_name_size,int & template_type,int & template_icon_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 498 | function (callable) | CNC_Editor_Get_Cell_Data |  | (int x,int y,char * cell_name,unsigned long cell_name_size,int & template_type,int & template_icon_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 542 | function (callable) | CNC_Editor_Get_Cell_Texture_Buffer |  | (int x,int y,int & out_width,int & out_height,SAFEARRAY * & out_texture_array) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 646 | function (callable) | CNC_Editor_Get_Template_Data |  | (int template_type_index,SAFEARRAY * & template_points) |  | SIGNATURE_CATALOGUED |
| REDALERT/DLLInterfaceEditor.cpp | 699 | function (callable) | CNC_Editor_Get_Scenario_Names |  | (char * cncdata_directory) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.CPP | 58 | function (callable) | DoorClass | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.CPP | 81 | function (callable) | AI | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.CPP | 118 | function (callable) | Open_Door | DoorClass | (int rate,int stages) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.CPP | 150 | function (callable) | Close_Door | DoorClass | (int rate,int stages) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.CPP | 183 | function (callable) | Door_Stage | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 71 | function (callable) | DoorClass | DoorClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 73 | function (callable) | Time_To_Redraw | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 74 | function (callable) | Clear_Redraw_Flag | DoorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 77 | function (callable) | Is_Door_Opening | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 78 | function (callable) | Is_Door_Closing | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 81 | function (callable) | Is_Door_Open | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DOOR.H | 82 | function (callable) | Is_Door_Closed | DoorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.CPP | 44 | function (callable) | Swap | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.CPP | 81 | function (callable) | Swap | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.CPP | 100 | function (callable) | Copy | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.CPP | 122 | function (callable) | Copy_To | DOSSegmentClass | (void * source,int dest,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.CPP | 146 | function (callable) | Copy_From | DOSSegmentClass | (void * dest,int source,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 62 | function (callable) | Push_DS | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 63 | function (callable) | Pop_DS | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 104 | function (callable) | DOSSegmentClass | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 109 | function (callable) | ~DOSSegmentClass | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 113 | function (callable) | Copy_Word_To | DOSSegmentClass | (short data,int dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 118 | function (callable) | Copy_Byte_To | DOSSegmentClass | (char data,int dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 123 | function (callable) | Copy_DWord_To | DOSSegmentClass | (long data,int dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 128 | function (callable) | Assign | DOSSegmentClass | (unsigned short segment,long) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 133 | function (callable) | DOSSegmentClass | DOSSegmentClass | (unsigned short segment,long) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 138 | function (callable) | Copy_To | DOSSegmentClass | (void * source,int dest,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 143 | function (callable) | Copy_From | DOSSegmentClass | (void * dest,int source,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 148 | function (callable) | Copy | DOSSegmentClass | (DOSSegmentClass & src,int soffset,DOSSegmentClass & dest,int doffset,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 152 | function (callable) | Copy_Word_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 157 | function (callable) | Copy_Byte_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 162 | function (callable) | Copy_DWord_From | DOSSegmentClass | (int source) |  | SIGNATURE_CATALOGUED |
| REDALERT/DPMI.H | 167 | function (callable) | Get_Selector | DOSSegmentClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 62 | function (callable) | test |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 88 | function (callable) | Response_Select | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 121 | function (callable) | Response_Move | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 150 | function (callable) | Response_Attack | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 191 | function (callable) | Scatter | DriveClass | (COORDINATE threat,bool forced,bool nokidding) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 243 | function (callable) | Limbo | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 268 | function (callable) | Stop_Driver | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 325 | function (callable) | Do_Turn | DriveClass | (DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 379 | function (callable) | Teleport_To | DriveClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 435 | function (callable) | Force_Track | DriveClass | (int track,COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 462 | function (callable) | DriveClass | DriveClass | (RTTIType rtti,int id,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 493 | function (callable) | Debug_Dump | DriveClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 524 | function (callable) | Smooth_Turn | DriveClass | (COORDINATE adj,DirType & dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 576 | function (callable) | Assign_Destination | DriveClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 646 | function (callable) | While_Moving | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 843 | function (callable) | Per_Cell_Process | DriveClass | (PCPType why) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 891 | function (callable) | Start_Of_Move | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 1289 | function (callable) | AI | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 1417 | function (callable) | Fixup_Path | DriveClass | (PathType * path) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 1599 | function (callable) | Lay_Track | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 1639 | function (callable) | Mark_Track | DriveClass | (COORDINATE headto,MarkType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.CPP | 1692 | function (callable) | Ok_To_Move | DriveClass | (DirType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.H | 92 | function (callable) | DriveClass | DriveClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.H | 93 | function (callable) | ~DriveClass | DriveClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DRIVE.H | 105 | function (callable) | Overrun_Square | DriveClass | (CELL,bool=true) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 39 | function (callable) | DropListClass | DropListClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w,int h,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 53 | function (callable) | Zap | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 61 | function (callable) | Add | DropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 67 | function (callable) | Add_Tail | DropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 73 | function (callable) | Add_Head | DropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 79 | function (callable) | Remove | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 88 | function (callable) | Add_Item | DropListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 95 | function (callable) | Current_Item | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 100 | function (callable) | Current_Index | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 105 | function (callable) | Set_Selected_Index | DropListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 116 | function (callable) | Clear_Focus | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 121 | function (callable) | Peer_To_Peer | DropListClass | (unsigned flags,KeyNumType & key,ControlClass & whom) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 141 | function (callable) | Expand | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 154 | function (callable) | Collapse | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 163 | function (callable) | operator = | DropListClass | (DropListClass const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 177 | function (callable) | DropListClass | DropListClass | (DropListClass const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 189 | function (callable) | Set_Position | DropListClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 197 | function (callable) | Set_Selected_Index | DropListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.CPP | 210 | function (callable) | Flag_To_Redraw | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 44 | function (callable) | ~DropListClass | DropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 59 | function (callable) | Count | DropListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 60 | function (callable) | Get_Item | DropListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 103 | function (callable) | ~TDropListClass | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 105 | function (callable) | operator []  | TDropListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 106 | function (callable) | operator []  | TDropListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 121 | function (callable) | Count | TDropListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 122 | function (callable) | Get_Item | TDropListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 155 | function (callable) | TDropListClass | TDropListClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w,int h,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 168 | function (callable) | Zap | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 178 | function (callable) | Add | TDropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 186 | function (callable) | Add_Tail | TDropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 194 | function (callable) | Add_Head | TDropListClass | (LinkClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 202 | function (callable) | Remove | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 213 | function (callable) | Add_Item | TDropListClass | (T item) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 222 | function (callable) | Current_Item | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 229 | function (callable) | Current_Index | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 236 | function (callable) | Set_Selected_Index | TDropListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 248 | function (callable) | Clear_Focus | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 255 | function (callable) | Peer_To_Peer | TDropListClass | (unsigned flags,KeyNumType & key,ControlClass & whom) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 277 | function (callable) | Expand | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 292 | function (callable) | Collapse | TDropListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 302 | function (callable) | operator = | TDropListClass | (TDropListClass<T> const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 317 | function (callable) | TDropListClass | TDropListClass | (TDropListClass<T> const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 330 | function (callable) | Set_Position | TDropListClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/DROP.H | 339 | function (callable) | Set_Selected_Index | TDropListClass | (T text) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 74 | function (callable) | DynamicVectorClass | DynamicVectorClass | (unsigned size,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 102 | function (callable) | Resize | DynamicVectorClass | (unsigned newsize,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 131 | function (callable) | ID | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 158 | function (callable) | Add | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 204 | function (callable) | Add_Head | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 256 | function (callable) | Delete | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/DYNAVEC.CPP | 280 | function (callable) | Delete | DynamicVectorClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 80 | function (callable) | EditClass | EditClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w,int h,EditStyle style) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 122 | function (callable) | ~EditClass | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 144 | function (callable) | Set_Text | EditClass | (char * text,int max_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 168 | function (callable) | Draw_Me | EditClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 218 | function (callable) | Action | EditClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 319 | function (callable) | Draw_Background | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 341 | function (callable) | Draw_Text | EditClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 377 | function (callable) | Handle_Key | EditClass | (KeyASCIIType ascii) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.CPP | 465 | function (callable) | Set_Focus | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 55 | function (callable) | Get_Text | EditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 56 | function (callable) | Set_Color | EditClass | (RemapControlType * color) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 58 | function (callable) | Set_Read_Only | EditClass | (int rdonly) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 104 | function (callable) | operator \| |  | (EditClass::EditStyle a,EditClass::EditStyle b) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 109 | function (callable) | operator & |  | (EditClass::EditStyle a,EditClass::EditStyle b) |  | SIGNATURE_CATALOGUED |
| REDALERT/EDIT.H | 114 | function (callable) | operator ~ |  | (EditClass::EditStyle a) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 190 | function (callable) | EgoClass | EgoClass | (int x,int y,char * text,TextPrintType flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 214 | function (callable) | ~EgoClass | EgoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 234 | function (callable) | Scroll | EgoClass | (int distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 259 | function (callable) | Render | EgoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 282 | function (callable) | Wipe | EgoClass | (GraphicBufferClass * background) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 314 | function (callable) | Set_Pal |  | (char * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 347 | function (callable) | Slide_Show |  | (int slide,int frame) |  | SIGNATURE_CATALOGUED |
| REDALERT/EGOS.CPP | 453 | function (callable) | Show_Who_Was_Responsible |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ENDING.CPP | 37 | function (callable) | GDI_Ending |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ENDING.CPP | 70 | function (callable) | Nod_Ending |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 151 | function (callable) | EventClass | EventClass | (SpecialClass data) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 177 | function (callable) | EventClass | EventClass | (EventType type,TargetClass target) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 186 | function (callable) | EventClass | EventClass | (EventType type,CELL cell) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 210 | function (callable) | EventClass | EventClass | (EventType type) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 231 | function (callable) | EventClass | EventClass | (EventType type,int val) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 259 | function (callable) | EventClass | EventClass | (EventType type,TargetClass src,TargetClass dest) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 285 | function (callable) | EventClass | EventClass | (AnimType anim,HousesType owner,COORDINATE coord,int visible) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 318 | function (callable) | EventClass | EventClass | (TargetClass src,MissionType mission,TargetClass target,TargetClass destination) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 355 | function (callable) | EventClass | EventClass | (TargetClass src,MissionType mission,TargetClass target,TargetClass destination,SpeedType speed,MPHType maxspeed) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 388 | function (callable) | EventClass | EventClass | (EventType type,RTTIType object,int id) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 418 | function (callable) | EventClass | EventClass | (EventType type,RTTIType object,CELL cell) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 446 | function (callable) | EventClass | EventClass | (EventType type,int id,CELL cell) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 471 | function (callable) | EventClass | EventClass | (EventType type,void * ptr,unsigned long size) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.CPP | 496 | function (callable) | Execute | EventClass | (void) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.H | 229 | function (callable) | EventClass | EventClass | (void) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EVENT.H | 250 | function (callable) | operator == | EventClass | (EventClass & q) | RUST/original-source-reference/REDALERT/EVENT.md | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 61 | function (callable) | Expansion_CS_Present |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 83 | function (callable) | Expansion_AM_Present |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 289 | function (callable) | EListClass | EListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 292 | function (callable) | Add_Object | EListClass | (EObjectClass * obj) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 295 | function (callable) | Get_Object | EListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 298 | function (callable) | Current_Object | EListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 306 | function (callable) | Add_Item | EListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 307 | function (callable) | Add_Item | EListClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 308 | function (callable) | Current_Item | EListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 309 | function (callable) | Get_Item | EListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 334 | function (callable) | Draw_Entry | EListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/EXPAND.CPP | 366 | function (callable) | Expansion_Dialog |  | (bool bCounterstrike) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.CPP | 62 | function (callable) | Desired_Facing8 |  | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.CPP | 147 | function (callable) | Desired_Facing256 |  | (int srcx,int srcy,int dstx,int dsty) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.H | 53 | function (callable) | operator + |  | (DirType f1,DirType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.H | 57 | function (callable) | operator + |  | (DirType f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.H | 61 | function (callable) | operator - |  | (DirType f1,DirType f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACE.H | 65 | function (callable) | operator - |  | (DirType f1,int f2) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.CPP | 58 | function (callable) | FacingClass | FacingClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.CPP | 83 | function (callable) | Set_Desired | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.CPP | 109 | function (callable) | Set_Current | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.CPP | 139 | function (callable) | Rotation_Adjust | FacingClass | (int rate) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 48 | function (callable) | FacingClass | FacingClass | (DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 49 | function (callable) | FacingClass | FacingClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 51 | function (callable) | operator DirType | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 53 | function (callable) | Current | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 54 | function (callable) | Desired | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 59 | function (callable) | Set | FacingClass | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 64 | function (callable) | Get | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 66 | function (callable) | Is_Rotating | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 67 | function (callable) | Difference | FacingClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACING.H | 68 | function (callable) | Difference | FacingClass | (DirType facing) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 70 | function (callable) | FactoryClass | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 102 | function (callable) | ~FactoryClass | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 126 | function (callable) | Init | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 147 | function (callable) | operator new | FactoryClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 172 | function (callable) | operator delete | FactoryClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 199 | function (callable) | AI | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 256 | function (callable) | Force_Complete | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 286 | function (callable) | Has_Changed | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 318 | function (callable) | Set | FactoryClass | (TechnoTypeClass const & object,HouseClass & house) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 378 | function (callable) | Set | FactoryClass | (TechnoClass & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 410 | function (callable) | Suspend | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 439 | function (callable) | Start | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 481 | function (callable) | Abandon | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 536 | function (callable) | Completion | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 559 | function (callable) | Has_Completed | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 587 | function (callable) | Get_Object | FactoryClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 605 | function (callable) | Get_Special_Item | FactoryClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 627 | function (callable) | Cost_Per_Tick | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.CPP | 659 | function (callable) | Completed | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 48 | function (callable) | FactoryClass | FactoryClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 51 | function (callable) | operator new | FactoryClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 68 | function (callable) | Is_Building | FactoryClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 78 | function (callable) | Get_House | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 79 | function (callable) | Name | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 80 | function (callable) | Is_Blocked | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FACTORY.H | 81 | function (callable) | Set_Is_Blocked | FactoryClass | (bool set) |  | SIGNATURE_CATALOGUED |
| REDALERT/FAKESOCK.H | 45 | function (callable) | Get_Connected | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 35 | function (callable) | FieldClass | FieldClass | (char * id,char data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 45 | function (callable) | FieldClass | FieldClass | (char * id,unsigned char data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 55 | function (callable) | FieldClass | FieldClass | (char * id,short data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 65 | function (callable) | FieldClass | FieldClass | (char * id,unsigned short data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 75 | function (callable) | FieldClass | FieldClass | (char * id,long data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 85 | function (callable) | FieldClass | FieldClass | (char * id,unsigned long data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 95 | function (callable) | FieldClass | FieldClass | (char * id,char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 105 | function (callable) | FieldClass | FieldClass | (char * id,void * data,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 126 | function (callable) | Host_To_Net | FieldClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.CPP | 172 | function (callable) | Net_To_Host | FieldClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIELD.H | 61 | function (callable) | FieldClass | FieldClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 89 | macro (function-like macro) | CELL_FACING |  | (a,b) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 128 | function (callable) | Opposite |  | (FacingType face) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 134 | function (callable) | Next_Direction |  | (FacingType facing,FacingType dir) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 188 | function (callable) | Point_Relative_To_Line |  | (int x,int z,int x1,int z1,int x2,int z2) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 221 | function (callable) | Unravel_Loop | FootClass | (PathType * path,CELL & cell,FacingType & dir,int sx,int sy,int dx,int dy,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 308 | function (callable) | Register_Cell | FootClass | (PathType * path,CELL cell,FacingType dir,int cost,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 432 | function (callable) | Find_Path | FootClass | (CELL dest,FacingType * final_moves,int maxlen,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 792 | function (callable) | Follow_Edge | FootClass | (CELL start,CELL target,PathType * path,FacingType search,FacingType olddir,int threat,int,int max_cells,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 1051 | function (callable) | Optimize_Moves | FootClass | (PathType * path,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 1219 | function (callable) | Safety_Point | FootClass | (CELL src,CELL dst,int start,int max) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FINDPATH.CPP | 1279 | function (callable) | Passable_Cell | FootClass | (CELL cell,FacingType face,int threat,MoveType threshhold) | RUST/original-source-reference/REDALERT/FINDPATH.md | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.CPP | 56 | function (callable) | fixed | fixed | (int numerator,int denominator) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.CPP | 85 | function (callable) | fixed | fixed | (char const * ascii) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.CPP | 170 | function (callable) | To_ASCII | fixed | (char * buffer,int maxlen) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.CPP | 239 | function (callable) | As_ASCII | fixed | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 87 | function (callable) | fixed | fixed | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 90 | function (callable) | fixed | fixed | (fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 96 | function (callable) | fixed | fixed | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 97 | function (callable) | fixed | fixed | (unsigned int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 100 | function (callable) | fixed | fixed | (float value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 106 | function (callable) | operator unsigned | fixed | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 111 | function (callable) | operator *= | fixed | (fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 112 | function (callable) | operator *= | fixed | (int rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 113 | function (callable) | operator /= | fixed | (fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 114 | function (callable) | operator /= | fixed | (int rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 115 | function (callable) | operator += | fixed | (fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 116 | function (callable) | operator -= | fixed | (fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 122 | function (callable) | operator * | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 123 | function (callable) | operator * | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 124 | function (callable) | operator / | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 125 | function (callable) | operator / | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 126 | function (callable) | operator + | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 127 | function (callable) | operator + | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 128 | function (callable) | operator - | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 129 | function (callable) | operator - | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 134 | function (callable) | operator >>= | fixed | (unsigned rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 135 | function (callable) | operator <<= | fixed | (unsigned rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 136 | function (callable) | operator >> | fixed | (unsigned rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 137 | function (callable) | operator << | fixed | (unsigned rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 142 | function (callable) | operator == | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 143 | function (callable) | operator != | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 144 | function (callable) | operator < | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 145 | function (callable) | operator > | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 146 | function (callable) | operator <= | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 147 | function (callable) | operator >= | fixed | (fixed const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 148 | function (callable) | operator ! | fixed | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 153 | function (callable) | operator < | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 154 | function (callable) | operator > | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 155 | function (callable) | operator <= | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 156 | function (callable) | operator >= | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 157 | function (callable) | operator == | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 158 | function (callable) | operator != | fixed | (int rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 163 | function (callable) | operator * |  | (int lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 164 | function (callable) | operator / |  | (int lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 165 | function (callable) | operator + |  | (int lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 166 | function (callable) | operator - |  | (int lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 167 | function (callable) | operator < |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 168 | function (callable) | operator > |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 169 | function (callable) | operator <= |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 170 | function (callable) | operator >= |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 171 | function (callable) | operator == |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 172 | function (callable) | operator != |  | (unsigned lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 173 | function (callable) | operator *= |  | (int & lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 174 | function (callable) | operator /= |  | (int & lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 175 | function (callable) | operator += |  | (int & lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 176 | function (callable) | operator -= |  | (int & lvalue,fixed const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 181 | function (callable) | Round_Up | fixed | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 182 | function (callable) | Round_Down | fixed | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 183 | function (callable) | Round | fixed | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 184 | function (callable) | Saturate | fixed | (unsigned capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 185 | function (callable) | Saturate | fixed | (fixed const & capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 186 | function (callable) | Sub_Saturate | fixed | (unsigned capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 187 | function (callable) | Sub_Saturate | fixed | (fixed const & capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 188 | function (callable) | Inverse | fixed | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 194 | function (callable) | Round_Up |  | (fixed const & value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 195 | function (callable) | Round_Down |  | (fixed const & value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 196 | function (callable) | Round |  | (fixed const & value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 197 | function (callable) | Saturate |  | (fixed const & value,unsigned capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 198 | function (callable) | Saturate |  | (fixed const & value,fixed const & capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 199 | function (callable) | Sub_Saturate |  | (fixed const & value,unsigned capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 200 | function (callable) | Sub_Saturate |  | (fixed const & value,fixed const & capvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/FIXED.H | 201 | function (callable) | Inverse |  | (fixed const & value) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.CPP | 56 | function (callable) | Debug_Dump | FlasherClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.CPP | 80 | function (callable) | Process | FlasherClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.CPP | 118 | function (callable) | Get_Flashing_Flags | FlasherClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.H | 59 | function (callable) | FlasherClass | FlasherClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.H | 69 | function (callable) | FlasherClass | FlasherClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLASHER.H | 70 | function (callable) | ~FlasherClass | FlasherClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.CPP | 59 | function (callable) | Physics | FlyClass | (COORDINATE & coord,DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.CPP | 125 | function (callable) | Fly_Speed | FlyClass | (int speed,MPHType maximum) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.H | 53 | function (callable) | FlyClass | FlyClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.H | 54 | function (callable) | FlyClass | FlyClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.H | 55 | function (callable) | ~FlyClass | FlyClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FLY.H | 62 | function (callable) | Get_Speed | FlyClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 99 | function (callable) | FootClass | FootClass | (RTTIType rtti,int id,HousesType house) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 155 | function (callable) | Debug_Dump | FootClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 222 | function (callable) | Set_Speed | FootClass | (int speed) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 253 | function (callable) | Mark | FootClass | (MarkType mark) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 311 | function (callable) | Basic_Path | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 490 | function (callable) | Mission_Move | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 523 | function (callable) | Mission_Capture | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 560 | function (callable) | Mission_Attack | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 587 | function (callable) | Mission_Guard | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 663 | function (callable) | Mission_Hunt | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 731 | function (callable) | Stop_Driver | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 763 | function (callable) | Start_Driver | FootClass | (COORDINATE & headto) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 809 | function (callable) | Sort_Y | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 840 | function (callable) | Stun | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 870 | function (callable) | Approach_Target | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 985 | function (callable) | Mission_Guard_Area | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1085 | function (callable) | Unlimbo | FootClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1132 | function (callable) | Take_Damage | FootClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1219 | function (callable) | Active_Click_With | FootClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1319 | function (callable) | Active_Click_With | FootClass | (ActionType action,CELL cell) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1412 | function (callable) | Per_Cell_Process | FootClass | (PCPType why) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1564 | function (callable) | Override_Mission | FootClass | (MissionType mission,TARGET tarcom,TARGET navcom) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1587 | function (callable) | Restore_Mission | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1619 | function (callable) | Receive_Message | FootClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1728 | function (callable) | Mission_Enter | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1796 | function (callable) | Assign_Destination | FootClass | (TARGET target) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1827 | function (callable) | Detach_All | FootClass | (bool all) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1859 | function (callable) | Rescue_Mission | FootClass | (TARGET tarcom) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1938 | function (callable) | Death_Announcement | FootClass | (TechnoClass const *) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 1973 | function (callable) | Greatest_Threat | FootClass | (ThreatType method) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2039 | function (callable) | Detach | FootClass | (TARGET target,bool all) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2115 | function (callable) | Offload_Tiberium_Bail | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2145 | function (callable) | Can_Enter_Cell | FootClass | (CELL,FacingType) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2168 | function (callable) | Can_Demolish | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2211 | function (callable) | Sell_Back | FootClass | (int control) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2244 | function (callable) | Likely_Coord | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2273 | function (callable) | Adjust_Dest | FootClass | (CELL cell) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2307 | function (callable) | Handle_Navigation_List | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2363 | function (callable) | Queue_Navigation_List | FootClass | (TARGET target) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2415 | function (callable) | Clear_Navigation_List | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2440 | function (callable) | Is_Allowed_To_Leave_Map | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2472 | function (callable) | Is_Recruitable | FootClass | (HouseClass const * house) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2528 | function (callable) | AI | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2558 | function (callable) | Is_On_Priority_Mission | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.CPP | 2581 | function (callable) | Mission_Retreat | FootClass | (void) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.H | 264 | function (callable) | FootClass | FootClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.H | 288 | function (callable) | Head_To_Coord | FootClass | (void) const | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FOOT.H | 347 | function (callable) | Fixup_Path | FootClass | (PathType *) | RUST/original-source-reference/REDALERT/FOOT.md | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 86 | function (callable) | operator ()  | NoInitClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 125 | function (callable) | BasicTimerClass | BasicTimerClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 149 | function (callable) | BasicTimerClass | BasicTimerClass | (unsigned long set) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 170 | function (callable) | ~BasicTimerClass | BasicTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 176 | function (callable) | Value | BasicTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 199 | function (callable) | operator unsigned long | BasicTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 222 | function (callable) | operator ()  | BasicTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 241 | function (callable) | ~TTimerClass | TTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 267 | function (callable) | TTimerClass | TTimerClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 290 | function (callable) | TTimerClass | TTimerClass | (unsigned long set) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 314 | function (callable) | Value | TTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 342 | function (callable) | operator unsigned long | TTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 370 | function (callable) | operator ()  | TTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 397 | function (callable) | Stop | TTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 421 | function (callable) | Start | TTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 446 | function (callable) | Is_Active | TTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 486 | function (callable) | Was_Started | CDTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 495 | function (callable) | CDTimerClass | CDTimerClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 518 | function (callable) | CDTimerClass | CDTimerClass | (unsigned long set) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 541 | function (callable) | ~CDTimerClass | CDTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 563 | function (callable) | Value | CDTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 595 | function (callable) | operator unsigned long | CDTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 627 | function (callable) | operator ()  | CDTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 659 | function (callable) | Stop | CDTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 684 | function (callable) | Start | CDTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FTIMER.H | 711 | function (callable) | Is_Active | CDTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 157 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 168 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 169 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 190 | function (callable) | operator ()  | NoInitClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 264 | macro (function-like macro) | int386x |  | (a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 265 | macro (function-like macro) | int386 |  | (a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 439 | macro (function-like macro) | assert |  | (__ignore) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUNCTION.H | 441 | macro (function-like macro) | assert |  | (expr) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.CPP | 54 | function (callable) | FuseClass | FuseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.CPP | 91 | function (callable) | Arm_Fuse | FuseClass | (COORDINATE location,COORDINATE target,int timeto,int arming) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.CPP | 117 | function (callable) | Fuse_Checkup | FuseClass | (COORDINATE newlocation) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.CPP | 163 | function (callable) | Fuse_Write | FuseClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.CPP | 186 | function (callable) | Fuse_Read | FuseClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.H | 46 | function (callable) | FuseClass | FuseClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.H | 47 | function (callable) | ~FuseClass | FuseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/FUSE.H | 84 | function (callable) | Fuse_Target | FuseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 109 | function (callable) | GadgetClass | GadgetClass | (int x,int y,int w,int h,unsigned flags,int sticky) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 133 | function (callable) | GadgetClass | GadgetClass | (GadgetClass const & gadget) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 161 | function (callable) | ~GadgetClass | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 193 | function (callable) | Clicked_On | GadgetClass | (KeyNumType & key,unsigned flags,int mousex,int mousey) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 235 | function (callable) | Enable | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 258 | function (callable) | Disable | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 281 | function (callable) | Remove | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 303 | function (callable) | Get_Next | GadgetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 324 | function (callable) | Get_Prev | GadgetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 345 | function (callable) | Delete_List | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 391 | function (callable) | Action | GadgetClass | (unsigned flags,KeyNumType &) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 423 | function (callable) | Draw_Me | GadgetClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 448 | function (callable) | Draw_All | GadgetClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 470 | function (callable) | Input | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 681 | function (callable) | Extract_Gadget | GadgetClass | (unsigned id) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 713 | function (callable) | Flag_To_Redraw | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 736 | function (callable) | Sticky_Process | GadgetClass | (unsigned flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 763 | function (callable) | Set_Focus | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 790 | function (callable) | Clear_Focus | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 814 | function (callable) | Has_Focus | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 835 | function (callable) | Is_List_To_Redraw | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.CPP | 864 | function (callable) | Set_Position | GadgetClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 114 | function (callable) | GadgetClass | GadgetClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 115 | function (callable) | GadgetClass | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 126 | function (callable) | Flag_List_To_Redraw | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 136 | function (callable) | Get_ID | GadgetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 138 | function (callable) | Peer_To_Peer | GadgetClass | (unsigned,KeyNumType &,ControlClass &) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 143 | function (callable) | Is_To_Redraw | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 154 | function (callable) | Set_Color_Scheme | GadgetClass | (RemapControlType * scheme) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 157 | function (callable) | Get_Color_Scheme | GadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 257 | function (callable) | operator \| |  | (GadgetClass::FlagEnum a,GadgetClass::FlagEnum b) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 262 | function (callable) | operator & |  | (GadgetClass::FlagEnum a,GadgetClass::FlagEnum b) |  | SIGNATURE_CATALOGUED |
| REDALERT/GADGET.H | 267 | function (callable) | operator ~ |  | (GadgetClass::FlagEnum a) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAMEDLG.CPP | 62 | function (callable) | Process | GameControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAMEDLG.H | 43 | function (callable) | GameControlsClass | GameControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 59 | function (callable) | GaugeClass | GaugeClass | (unsigned id,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 90 | function (callable) | Set_Maximum | GaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 117 | function (callable) | Set_Value | GaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 144 | function (callable) | Pixel_To_Value | GaugeClass | (int pixel) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 178 | function (callable) | Value_To_Pixel | GaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 208 | function (callable) | Draw_Me | GaugeClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 271 | function (callable) | Action | GaugeClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 364 | function (callable) | Draw_Thumb | GaugeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 397 | function (callable) | TriColorGaugeClass | TriColorGaugeClass | (unsigned id,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 416 | function (callable) | Set_Red_Limit | TriColorGaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 444 | function (callable) | Set_Yellow_Limit | TriColorGaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.CPP | 472 | function (callable) | Draw_Me | TriColorGaugeClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.H | 47 | function (callable) | Get_Value | GaugeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.H | 48 | function (callable) | Use_Thumb | GaugeClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/GAUGE.H | 50 | function (callable) | Thumb_Pixels | GaugeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GETCPU.CPP | 80 | function (callable) | Get_CPU_Type |  | (int & cpu_type,bool & mmx,char * vendor_id,int vendor_id_length) |  | SIGNATURE_CATALOGUED |
| REDALERT/GOPTIONS.CPP | 68 | function (callable) | Process | GameOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GOPTIONS.CPP | 605 | function (callable) | Adjust_Variables_For_Resolution | GameOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GOPTIONS.H | 77 | function (callable) | GameOptionsClass | GameOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 70 | function (callable) | GScreenClass | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 101 | function (callable) | One_Time | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 135 | function (callable) | Init | GScreenClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 162 | function (callable) | Init_Clear | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 195 | function (callable) | Init_Theater | GScreenClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 215 | function (callable) | Init_IO | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 245 | function (callable) | Flag_To_Redraw | GScreenClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 271 | function (callable) | Input | GScreenClass | (KeyNumType & key,int & x,int & y) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 325 | function (callable) | Add_A_Button | GScreenClass | (GadgetClass & gadget) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 366 | function (callable) | Remove_A_Button | GScreenClass | (GadgetClass & gadget) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 390 | function (callable) | Render | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.CPP | 467 | function (callable) | Blit_Display | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.H | 46 | function (callable) | GScreenClass | GScreenClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.H | 61 | function (callable) | AI | GScreenClass | (KeyNumType &,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.H | 80 | function (callable) | Draw_It | GScreenClass | (bool=false) |  | SIGNATURE_CATALOGUED |
| REDALERT/GSCREEN.H | 98 | function (callable) | Shadow_Address | GScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 287 | function (callable) | HouseTypeClass | HouseTypeClass | (HousesType house,char const * ini,int fullname,char const * ext,int lemon,PlayerColorType remapcolor,char prefix) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 332 | function (callable) | operator new | HouseTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 353 | function (callable) | operator delete | HouseTypeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 373 | function (callable) | Init_Heap | HouseTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 421 | function (callable) | From_Name | HouseTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 448 | function (callable) | One_Time | HouseTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 468 | function (callable) | As_Reference | HouseTypeClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 489 | function (callable) | Remap_Table | HouseTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HDATA.CPP | 510 | function (callable) | Read_INI | HouseTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 115 | function (callable) | FixedHeapClass | FixedHeapClass | (int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 140 | function (callable) | ~FixedHeapClass | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 166 | function (callable) | Set_Heap | FixedHeapClass | (int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 222 | function (callable) | Allocate | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 253 | function (callable) | Free | FixedHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 287 | function (callable) | ID | FixedHeapClass | (void const * pointer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 312 | function (callable) | Clear | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 343 | function (callable) | Free_All | FixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 369 | function (callable) | Free_All | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 392 | function (callable) | Clear | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 417 | function (callable) | Set_Heap | FixedIHeapClass | (int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 443 | function (callable) | Allocate | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 470 | function (callable) | Free | FixedIHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 497 | function (callable) | Logical_ID | FixedIHeapClass | (void const * pointer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 524 | function (callable) | Save | TFixedIHeapClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 566 | function (callable) | Load | TFixedIHeapClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 633 | function (callable) | Code_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.CPP | 656 | function (callable) | Decode_Pointers | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 53 | function (callable) | Count | FixedHeapClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 54 | function (callable) | Length | FixedHeapClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 55 | function (callable) | Avail | FixedHeapClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 64 | function (callable) | operator [] | FixedHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 65 | function (callable) | operator [] | FixedHeapClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 118 | function (callable) | TFixedHeapClass | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 119 | function (callable) | ~TFixedHeapClass | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 122 | function (callable) | ID | TFixedHeapClass | (T const * pointer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 123 | function (callable) | Alloc | TFixedHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 124 | function (callable) | Free | TFixedHeapClass | (T * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 126 | function (callable) | operator [] | TFixedHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 127 | function (callable) | operator [] | TFixedHeapClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 140 | function (callable) | FixedIHeapClass | FixedIHeapClass | (int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 141 | function (callable) | ~FixedIHeapClass | FixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 149 | function (callable) | Logical_ID | FixedIHeapClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 151 | function (callable) | Active_Ptr | FixedIHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 152 | function (callable) | Active_Ptr | FixedIHeapClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 174 | function (callable) | TFixedIHeapClass | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 175 | function (callable) | ~TFixedIHeapClass | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 177 | function (callable) | ID | TFixedIHeapClass | (T const * pointer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 178 | function (callable) | Logical_ID | TFixedIHeapClass | (T const * pointer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 179 | function (callable) | Logical_ID | TFixedIHeapClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 180 | function (callable) | Alloc | TFixedIHeapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 181 | function (callable) | Free | TFixedIHeapClass | (T * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 182 | function (callable) | Free | TFixedIHeapClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 188 | function (callable) | Ptr | TFixedIHeapClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HEAP.H | 189 | function (callable) | Raw_Ptr | TFixedIHeapClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 77 | function (callable) | HelpClass | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 107 | function (callable) | Init_Clear | HelpClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 131 | function (callable) | Overlap_List | HelpClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 161 | function (callable) | AI | HelpClass | (KeyNumType & key,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 212 | function (callable) | Help_Text | HelpClass | (int text,int x,int y,int color,bool quick) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 268 | function (callable) | Draw_It | HelpClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 310 | function (callable) | Set_Text | HelpClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 360 | function (callable) | Scroll_Map | HelpClass | (DirType facing,int & distance,bool really) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 386 | function (callable) | Set_Cost | HelpClass | (int cost) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.CPP | 408 | function (callable) | Set_Tactical_Position | HelpClass | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/HELP.H | 46 | function (callable) | HelpClass | HelpClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 145 | function (callable) | Save | TFixedIHeapClass::BuildChoiceClass | (Pipe &) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 150 | function (callable) | Load | TFixedIHeapClass::BuildChoiceClass | (Straw &) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 155 | function (callable) | Code_Pointers | TFixedIHeapClass::BuildChoiceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 159 | function (callable) | Decode_Pointers | TFixedIHeapClass::BuildChoiceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 180 | function (callable) | operator HousesType | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 203 | function (callable) | Tiberium_Fraction | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 229 | function (callable) | As_Pointer | HouseClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 257 | function (callable) | One_Time | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 280 | function (callable) | Assign_Handicap | HouseClass | (DiffType handicap) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 315 | function (callable) | Print_Zone_Stats | HouseClass | (int x,int y,ZoneType zone,MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 337 | function (callable) | Debug_Dump | HouseClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 461 | function (callable) | operator new | HouseClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 486 | function (callable) | operator delete | HouseClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 511 | function (callable) | HouseClass | HouseClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 702 | function (callable) | ~HouseClass | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 725 | function (callable) | HouseStaticClass | HouseStaticClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 762 | function (callable) | Can_Build | HouseClass | (ObjectTypeClass const * type,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 880 | function (callable) | Init | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 912 | function (callable) | AI | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1378 | function (callable) | Super_Weapon_Handler | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1820 | function (callable) | Attacked | HouseClass | (BuildingClass * source) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1868 | function (callable) | Harvested | HouseClass | (unsigned tiberium) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1900 | function (callable) | Stole | HouseClass | (unsigned worth) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1923 | function (callable) | Available_Money | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1948 | function (callable) | Spend_Money | HouseClass | (unsigned money) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 1983 | function (callable) | Refund_Money | HouseClass | (unsigned money) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2008 | function (callable) | Adjust_Capacity | HouseClass | (int adjust,bool inanger) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2051 | function (callable) | Silo_Redraw_Check | HouseClass | (long oldtib,long oldcap) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2085 | function (callable) | Is_Ally | HouseClass | (HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2110 | function (callable) | Is_Ally | HouseClass | (HouseClass const * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2135 | function (callable) | Is_Ally | HouseClass | (ObjectClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2163 | function (callable) | Make_Ally | HouseClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2276 | function (callable) | Make_Enemy | HouseClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2354 | function (callable) | Remap_Table | HouseClass | (bool blushing,RemapType remap) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2383 | function (callable) | Suggested_New_Team | HouseClass | (bool alertcheck) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2408 | function (callable) | Adjust_Threat | HouseClass | (int region,int threat) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2460 | function (callable) | Begin_Production | HouseClass | (RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2538 | function (callable) | Suspend_Production | HouseClass | (RTTIType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2585 | function (callable) | Abandon_Production | HouseClass | (RTTIType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2651 | function (callable) | Special_Weapon_AI | HouseClass | (SpecialWeaponType id) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 2707 | function (callable) | Place_Special_Blast | HouseClass | (SpecialWeaponType id,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3068 | function (callable) | Place_Object | HouseClass | (RTTIType type,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3239 | function (callable) | Manual_Place | HouseClass | (BuildingClass * builder,BuildingClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3283 | function (callable) | Clobber_All | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3359 | function (callable) | Detach | HouseClass | (TARGET target,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3388 | function (callable) | Does_Enemy_Building_Exist | HouseClass | (StructType btype) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3424 | function (callable) | Suggest_New_Object | HouseClass | (RTTIType objecttype,bool kennel) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3501 | function (callable) | Flag_Remove | HouseClass | (TARGET target,bool set_home) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3568 | function (callable) | Flag_Attach | HouseClass | (CELL cell,bool set_home) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3695 | function (callable) | Flag_Attach | HouseClass | (UnitClass * object,bool set_home) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3729 | function (callable) | MPlayer_Defeated | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 3958 | function (callable) | Tally_Score | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4086 | function (callable) | Blowup_All | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4254 | function (callable) | Flag_To_Die | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4281 | function (callable) | Flag_To_Win | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4309 | function (callable) | Flag_To_Lose | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4343 | function (callable) | Init_Data | HouseClass | (PlayerColorType color,HousesType house,int credits) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4380 | function (callable) | Power_Fraction | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4409 | function (callable) | Sell_Wall | HouseClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4501 | function (callable) | Suggest_New_Building | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4533 | function (callable) | Find_Building | HouseClass | (StructType type,ZoneType zone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4575 | function (callable) | Find_Build_Location | HouseClass | (BuildingClass * building) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4693 | function (callable) | Recalc_Center | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 4823 | function (callable) | Expert_AI | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5119 | function (callable) | Check_Build_Power | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5149 | function (callable) | Check_Build_Defense | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5162 | function (callable) | Check_Build_Offense | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5178 | function (callable) | Check_Attack | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5192 | function (callable) | Check_Build_Income | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5206 | function (callable) | Check_Fire_Sale | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5221 | function (callable) | Check_Build_Engineer | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5238 | function (callable) | Check_Raise_Money | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5257 | function (callable) | Check_Lower_Power | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5274 | function (callable) | Check_Raise_Power | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5291 | function (callable) | AI_Attack | HouseClass | (UrgencyType) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5352 | function (callable) | AI_Build_Power | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5364 | function (callable) | AI_Build_Defense | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5375 | function (callable) | AI_Build_Offense | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5386 | function (callable) | AI_Build_Income | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5394 | function (callable) | AI_Fire_Sale | HouseClass | (UrgencyType urgency) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5409 | function (callable) | AI_Build_Engineer | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5420 | function (callable) | AI_Lower_Power | HouseClass | (UrgencyType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5456 | function (callable) | AI_Raise_Power | HouseClass | (UrgencyType urgency) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5512 | function (callable) | AI_Raise_Money | HouseClass | (UrgencyType urgency) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5575 | function (callable) | AI_Base_Defense | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 5664 | function (callable) | AI_Building | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6020 | function (callable) | AI_Unit | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6153 | function (callable) | AI_Vessel | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6276 | function (callable) | AI_Infantry | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6473 | function (callable) | AI_Aircraft | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6530 | function (callable) | Production_Begun | HouseClass | (TechnoClass const * product) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6589 | function (callable) | Tracking_Remove | HouseClass | (TechnoClass const * techno) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6657 | function (callable) | Tracking_Add | HouseClass | (TechnoClass const * techno) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6762 | function (callable) | Factory_Counter | HouseClass | (RTTIType rtti) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6807 | function (callable) | Active_Remove | HouseClass | (TechnoClass const * techno) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6835 | function (callable) | Active_Add | HouseClass | (TechnoClass const * techno) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6864 | function (callable) | Which_Zone | HouseClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6897 | function (callable) | Which_Zone | HouseClass | (ObjectClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6921 | function (callable) | Which_Zone | HouseClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 6945 | function (callable) | Recalc_Attributes | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7044 | function (callable) | Zone_Cell | HouseClass | (ZoneType zone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7088 | function (callable) | Where_To_Go | HouseClass | (FootClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7123 | function (callable) | Find_Juicy_Target | HouseClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7169 | function (callable) | Get_Quantity | HouseClass | (AircraftType aircraft) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7192 | function (callable) | Fetch_Factory | HouseClass | (RTTIType rtti) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7257 | function (callable) | Set_Factory | HouseClass | (RTTIType rtti,FactoryClass * factory) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7319 | function (callable) | Factory_Count | HouseClass | (RTTIType rtti) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7343 | function (callable) | Get_Quantity | HouseClass | (StructType building) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7366 | function (callable) | Read_INI | HouseClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7419 | function (callable) | Write_INI | HouseClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7493 | function (callable) | Is_No_YakMig | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7534 | function (callable) | Is_Hack_Prevented | HouseClass | (RTTIType rtti,int value) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7557 | function (callable) | Fire_Sale | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7589 | function (callable) | Do_All_To_Hunt | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7648 | function (callable) | Is_Allowed_To_Ally | HouseClass | (HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7746 | function (callable) | Computer_Paranoid | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7794 | function (callable) | Adjust_Power | HouseClass | (int adjust) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7817 | function (callable) | Adjust_Drain | HouseClass | (int adjust) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7836 | function (callable) | Update_Spied_Power_Plants | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7874 | function (callable) | Find_Cell_In_Zone | HouseClass | (TechnoClass const * techno,ZoneType zone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 7941 | function (callable) | Random_Cell_In_Zone | HouseClass | (ZoneType zone) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 8028 | function (callable) | Get_Ally_Flags | HouseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 8047 | function (callable) | Check_Pertinent_Structures | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 8121 | function (callable) | Init_Unit_Trackers | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.CPP | 8171 | function (callable) | Free_Unit_Trackers | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 60 | function (callable) | HouseStaticClass | HouseStaticClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 646 | function (callable) | operator new | HouseClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 649 | function (callable) | HouseClass | HouseClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 685 | function (callable) | Name | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 700 | function (callable) | Make_Ally | HouseClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 702 | function (callable) | Make_Enemy | HouseClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 706 | function (callable) | Get_Allies | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 739 | function (callable) | Begin_Production | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 769 | function (callable) | Can_Make_Money | HouseClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 895 | function (callable) | operator new | HouseClass::BuildChoiceClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 899 | function (callable) | BuildChoiceClass | HouseClass::BuildChoiceClass | (UrgencyType u,StructType s) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 900 | function (callable) | BuildChoiceClass | HouseClass::BuildChoiceClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 901 | function (callable) | Save | HouseClass::BuildChoiceClass | (Pipe &) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 902 | function (callable) | Load | HouseClass::BuildChoiceClass | (Straw &) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 903 | function (callable) | Code_Pointers | HouseClass::BuildChoiceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 904 | function (callable) | Decode_Pointers | HouseClass::BuildChoiceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 932 | function (callable) | QuantityB | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 934 | function (callable) | QuantityU | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 938 | function (callable) | QuantityI | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 942 | function (callable) | QuantityA | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 943 | function (callable) | QuantityV | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 948 | function (callable) | QuantityU | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 949 | function (callable) | QuantityI | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 950 | function (callable) | QuantityA | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HOUSE.H | 951 | function (callable) | QuantityV | HouseClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.CPP | 66 | function (callable) | Adjust | HSVClass | (int ratio,HSVClass const & hsv) |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.CPP | 105 | function (callable) | Difference | HSVClass | (HSVClass const & hsv) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.CPP | 135 | function (callable) | operator RGBClass | HSVClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.CPP | 204 | function (callable) | Set | HSVClass | (int color) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.H | 53 | function (callable) | HSVClass | HSVClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.H | 54 | function (callable) | HSVClass | HSVClass | (unsigned char hue,unsigned char saturation,unsigned char value) |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.H | 67 | function (callable) | Hue_Component | HSVClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.H | 68 | function (callable) | Saturation_Component | HSVClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/HSV.H | 69 | function (callable) | Value_Component | HSVClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 40 | function (callable) | IconListClass | IconListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down,bool bResponsibleForStringAlloc,int iSelectionType,int iMaxItemsSaved) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 58 | function (callable) | ~IconListClass | IconListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 89 | function (callable) | Add_Item | IconListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 94 | function (callable) | Add_Item | IconListClass | (const char * text,const char * szHelp,void * pIcon0,ICONKIND IconKind0,const char * szExtraDataString,void * pvExtraDataPtr,RemapControlType * pColorRemap,void * pIcon1,ICONKIND IconKind1,void * pIcon2,ICONKIND IconKind2,void * pFixedIcon,ICONKIND FixedIconKind,int iXFixedIcon,int iYFixedIcon,int iFixedIconWidth) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 255 | function (callable) | Add_Item_Detail | IconListClass | (const char * szToken,const char * szHelp,void * pIcon0,ICONKIND IconKind0,const char * szExtraDataString,void * pvExtraData,RemapControlType * pColorRemap,void * pIcon1,ICONKIND IconKind1,void * pIcon2,ICONKIND IconKind2,void * pFixedIcon,ICONKIND FixedIconKind,int iXFixedIcon,int iYFixedIcon,int iFixedIconWidth) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 308 | function (callable) | Add_Item | IconListClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 314 | function (callable) | Add_Item | IconListClass | (int text,const char * szHelp,void * pIcon0,ICONKIND IconKind0,const char * szExtraDataString,void * pvExtraDataPtr,RemapControlType * pColorRemap,void * pIcon1,ICONKIND IconKind1,void * pIcon2,ICONKIND IconKind2,void * pFixedIcon,ICONKIND FixedIconKind,int iXFixedIcon,int iYFixedIcon,int iFixedIconWidth) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 326 | function (callable) | Remove_Item | IconListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 333 | function (callable) | Remove_Item | IconListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 381 | function (callable) | Draw_Entry | IconListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 509 | function (callable) | Action | IconListClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 534 | function (callable) | Show_Last_Item | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 545 | function (callable) | bItemIsMultiSelected | IconListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 554 | function (callable) | MultiSelect | IconListClass | (int index,bool bSelect) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 561 | function (callable) | Get_Item_ExtraDataString | IconListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 573 | function (callable) | Set_Item_ExtraDataString | IconListClass | (int index,const char * szNewString) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 595 | function (callable) | Get_Item_ExtraDataPtr | IconListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 606 | function (callable) | Set_Item_ExtraDataPtr | IconListClass | (int index,void * pNewValue) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 614 | function (callable) | Get_ItemExtras | IconListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 623 | function (callable) | Get_Item_Help | IconListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 633 | function (callable) | Clear | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 655 | function (callable) | Get_Item_Color | IconListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 664 | function (callable) | Set_Item_Color | IconListClass | (int index,RemapControlType * pColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 671 | function (callable) | Find | IconListClass | (const char * szItemToFind) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 684 | function (callable) | FindColor | IconListClass | (RemapControlType * pColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 696 | function (callable) | Set_Item | IconListClass | (unsigned int index,const char * szText) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 716 | function (callable) | Set_Icon | IconListClass | (unsigned int index,unsigned int iIconNumber,void * pIcon,ICONKIND IconKind) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 728 | function (callable) | GetRealWidth | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 736 | function (callable) | Resize | IconListClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 758 | function (callable) | IndexUnderMouse | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 770 | function (callable) | OffsetToIndex | IconListClass | (int iIndex,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 786 | function (callable) | Format_Window_String_New |  | (const char * string,int maxlinelen,int & width,int & height,char * szReturn,int iExtraChars) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.CPP | 860 | function (callable) | CC_Draw_DIB |  | (const char * pDIB,int xDest,int yDest,int iWidth,WindowNumberType window) |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.H | 52 | function (callable) | IconList_ItemExtras | IconList_ItemExtras | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.H | 58 | function (callable) | ~IconList_ItemExtras | IconList_ItemExtras | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.H | 139 | function (callable) | Get_View_Index | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.H | 140 | function (callable) | bScrollBeingDragged | IconListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ICONLIST.H | 157 | function (callable) | SetSelectType | IconListClass | (int iSelectTypeNew) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1047 | function (callable) | InfantryTypeClass | InfantryTypeClass | (InfantryType type,int name,char const * ininame,int verticaloffset,int primaryoffset,bool is_female,bool is_crawling,bool is_civilian,bool is_remap_override,bool is_nominal,bool is_theater,PipEnum pip,DoInfoStruct const * control,DoInfoStruct const * virtual_control,int firelaunch,int pronelaunch,unsigned char const * override_remap,int horizontaloffset) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1132 | function (callable) | operator new | InfantryTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1153 | function (callable) | operator delete | InfantryTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1173 | function (callable) | Init_Heap | InfantryTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1228 | function (callable) | Create_One_Of | InfantryTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1252 | function (callable) | Create_And_Place | InfantryTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1285 | function (callable) | Occupy_List | InfantryTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1314 | function (callable) | Display | InfantryTypeClass | (int x,int y,WindowNumberType window,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1345 | function (callable) | Prep_For_Add | InfantryTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1371 | function (callable) | From_Name | InfantryTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1399 | function (callable) | One_Time | InfantryTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1461 | function (callable) | Full_Name | InfantryTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1485 | function (callable) | As_Reference | InfantryTypeClass | (InfantryType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1506 | function (callable) | Read_INI | InfantryTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/IDATA.CPP | 1521 | function (callable) | Dimensions | InfantryTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 141 | function (callable) | Debug_Dump | InfantryClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 177 | function (callable) | InfantryClass | InfantryClass | (InfantryType classid,HousesType house) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 230 | function (callable) | ~InfantryClass | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 266 | function (callable) | operator new | InfantryClass | (size_t) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 290 | function (callable) | operator delete | InfantryClass | (void * ptr) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 323 | function (callable) | Take_Damage | InfantryClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 484 | function (callable) | Shape_Number | InfantryClass | (WindowNumberType window) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 565 | function (callable) | Draw_It | InfantryClass | (int x,int y,WindowNumberType window) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 609 | function (callable) | Per_Cell_Process | InfantryClass | (PCPType why) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1065 | function (callable) | Detach | InfantryClass | (TARGET target,bool all) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1094 | function (callable) | Init | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1115 | function (callable) | Assign_Destination | InfantryClass | (TARGET target) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1200 | function (callable) | Assign_Target | InfantryClass | (TARGET target) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1242 | function (callable) | AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1343 | function (callable) | Can_Enter_Cell | InfantryClass | (CELL cell,FacingType) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1624 | function (callable) | Overlap_List | InfantryClass | (bool redraw) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1688 | function (callable) | Can_Fire | InfantryClass | (TARGET target,int which) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1754 | function (callable) | Fire_Coord | InfantryClass | (int which) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1790 | function (callable) | Enter_Idle_Mode | InfantryClass | (bool) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1869 | function (callable) | Random_Animate | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 1979 | function (callable) | Scatter | InfantryClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2077 | function (callable) | Look | InfantryClass | (bool incremental) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2103 | function (callable) | Do_Action | InfantryClass | (DoType todo,bool force) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2183 | function (callable) | Stop_Driver | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2244 | function (callable) | Start_Driver | InfantryClass | (COORDINATE & headto) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2296 | function (callable) | Limbo | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2329 | function (callable) | Fire_At | InfantryClass | (TARGET target,int which) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2377 | function (callable) | Unlimbo | InfantryClass | (COORDINATE coord,DirType facing) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2432 | function (callable) | Greatest_Threat | InfantryClass | (ThreatType threat) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2521 | function (callable) | Response_Select | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2645 | function (callable) | Response_Move | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2776 | function (callable) | Response_Attack | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 2905 | function (callable) | What_Action | InfantryClass | (ObjectClass const * object) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3133 | function (callable) | Active_Click_With | InfantryClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3183 | function (callable) | Set_Occupy_Bit | InfantryClass | (CELL cell,int spot_index) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3212 | function (callable) | Clear_Occupy_Bit | InfantryClass | (CELL cell,int spot_index) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3250 | function (callable) | Full_Name | InfantryClass | (void) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3285 | function (callable) | Mission_Attack | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3325 | function (callable) | What_Action | InfantryClass | (CELL cell) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3435 | function (callable) | Class_Of | InfantryClass | (void) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3464 | function (callable) | Read_INI | InfantryClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3584 | function (callable) | Write_INI | InfantryClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3634 | function (callable) | Fear_AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3696 | function (callable) | Edge_Of_World_AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3728 | function (callable) | Firing_AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3863 | function (callable) | Doing_AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 3952 | function (callable) | Movement_AI | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 4246 | function (callable) | Get_Image_Data | InfantryClass | (void) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 4271 | function (callable) | Is_Ready_To_Random_Animate | InfantryClass | (void) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.CPP | 4343 | function (callable) | Paradrop | InfantryClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 114 | function (callable) | operator new | InfantryClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 117 | function (callable) | InfantryClass | InfantryClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 119 | function (callable) | operator InfantryType | InfantryClass | (void) const | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 159 | function (callable) | Active_Click_With | InfantryClass | (ActionType action,CELL cell) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 171 | function (callable) | Set_Occupy_Bit | InfantryClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 173 | function (callable) | Clear_Occupy_Bit | InfantryClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INFANTRY.H | 207 | function (callable) | INI_Name | InfantryClass | (void) | RUST/original-source-reference/REDALERT/INFANTRY.md | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 102 | function (callable) | ~INIClass | INIClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 130 | function (callable) | Clear | INIClass | (char const * section,char const * entry) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 176 | function (callable) | Load | INIClass | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 197 | function (callable) | Load | INIClass | (Straw & ffile) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 312 | function (callable) | Save | INIClass | (FileClass & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 332 | function (callable) | Save | INIClass | (Pipe & pipe) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 392 | function (callable) | Find_Section | INIClass | (char const * section) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 422 | function (callable) | Section_Count | INIClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 444 | function (callable) | Entry_Count | INIClass | (char const * section) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 472 | function (callable) | Find_Entry | INIClass | (char const * section,char const * entry) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 500 | function (callable) | Get_Entry | INIClass | (char const * section,int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 538 | function (callable) | Put_UUBlock | INIClass | (char const * section,void const * block,int len) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 590 | function (callable) | Get_UUBlock | INIClass | (char const * section,void * block,int len) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 634 | function (callable) | Put_TextBlock | INIClass | (char const * section,char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 706 | function (callable) | Get_TextBlock | INIClass | (char const * section,char * buffer,int len) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 765 | function (callable) | Put_Int | INIClass | (char const * section,char const * entry,int number,int format) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 808 | function (callable) | Get_Int | INIClass | (char const * section,char const * entry,int defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 852 | function (callable) | Put_Hex | INIClass | (char const * section,char const * entry,int number) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 882 | function (callable) | Get_Hex | INIClass | (char const * section,char const * entry,int defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 917 | function (callable) | Put_String | INIClass | (char const * section,char const * entry,char const * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 981 | function (callable) | Get_String | INIClass | (char const * section,char const * entry,char const * defvalue,char * buffer,int size) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1042 | function (callable) | Put_Bool | INIClass | (char const * section,char const * entry,bool value) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1075 | function (callable) | Get_Bool | INIClass | (char const * section,char const * entry,bool defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1117 | function (callable) | Find_Entry | INIClass::INISection | (char const * entry) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1149 | function (callable) | Put_PKey | INIClass | (PKey const & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1178 | function (callable) | Get_PKey | INIClass | (bool fast) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1225 | function (callable) | Get_Fixed | INIClass | (char const * section,char const * entry,fixed defvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1257 | function (callable) | Put_Fixed | INIClass | (char const * section,char const * entry,fixed value) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.CPP | 1278 | function (callable) | Strip_Comments | INIClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 54 | function (callable) | INIClass | INIClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 71 | function (callable) | Is_Loaded | INIClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 73 | function (callable) | Is_Present | INIClass | (char const * section,char const * entry=0) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 80 | function (callable) | Section_Present | INIClass | (char const * section) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 120 | function (callable) | INIEntry | INIClass::INIEntry | (char * entry=0,char * value=0) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 121 | function (callable) | ~INIEntry | INIClass::INIEntry | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 122 | function (callable) | Index_ID | INIClass::INIEntry | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 133 | function (callable) | INISection | INIClass::INISection | (char * section) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 134 | function (callable) | ~INISection | INIClass::INISection | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INI.H | 136 | function (callable) | Index_ID | INIClass::INISection | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INICODE.CPP | 40 | function (callable) | Get_Scenario_Digest |  | (char * digest,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/INICODE.CPP | 101 | function (callable) | Read_Scenario_INI_Write_INB |  | (char * root,bool fresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 161 | function (callable) | Load_Prolog_Page |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 194 | function (callable) | Init_Game |  | (int,char * []) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 469 | function (callable) | Select_Game |  | (bool fade) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 1663 | function (callable) | Play_Intro |  | (bool sequenced) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 1713 | function (callable) | Anim_Init |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 1803 | function (callable) | Parse_Command_Line |  | (int argc,char * argv[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2275 | function (callable) | Obfuscate |  | (char const * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2454 | function (callable) | Calculate_CRC |  | (void * buffer,long len) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2476 | function (callable) | Init_Random |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2589 | function (callable) | Load_Title_Page |  | (bool visible) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2622 | function (callable) | Init_Color_Remaps |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2755 | function (callable) | Init_Heaps |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2812 | function (callable) | Init_Expansion_Files |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2906 | function (callable) | Init_One_Time_Systems |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2947 | function (callable) | Init_Fonts |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 2982 | function (callable) | Init_CDROM_Access |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3076 | function (callable) | Init_Bootstrap_Mixfiles |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3167 | function (callable) | Init_Secondary_Mixfiles |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3252 | function (callable) | Bootstrap |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3385 | function (callable) | Init_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3434 | function (callable) | Init_Authorization |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3537 | function (callable) | Init_Bulk_Data |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3588 | function (callable) | Init_Keys |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3624 | function (callable) | Save_Recording_Values |  | (CCFileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3655 | function (callable) | Load_Recording_Values |  | (CCFileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3670 | function (callable) | __PRO |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3676 | function (callable) | Extract |  | (char * filename,char * outname) |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3711 | function (callable) | Is_DVD_Installed |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3730 | function (callable) | Determine_If_Using_DVD |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3754 | function (callable) | Using_DVD |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/INIT.CPP | 3777 | function (callable) | Free_Heaps |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 96 | function (callable) | Lepton_To_Pixel |  | (LEPTON lepton) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 116 | function (callable) | Pixel_To_Lepton |  | (int pixel) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 137 | function (callable) | XY_Coord |  | (LEPTON x,LEPTON y) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 162 | function (callable) | XYP_COORD |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 183 | function (callable) | Coord_XCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 203 | function (callable) | Coord_YCell |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 224 | function (callable) | XY_Cell |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 248 | function (callable) | Cell_To_Lepton |  | (int cell_distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 273 | function (callable) | Lepton_To_Cell |  | (LEPTON lepton_distance) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 296 | function (callable) | Coord_X |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 316 | function (callable) | Coord_Y |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 336 | function (callable) | Cell_X |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 356 | function (callable) | Cell_Y |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 379 | function (callable) | Coord_XLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 400 | function (callable) | Coord_YLepton |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 424 | function (callable) | XYP_Coord |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 449 | function (callable) | Cell_Coord |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 477 | function (callable) | Coord_Snap |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 500 | function (callable) | Coord_Fraction |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 523 | function (callable) | Coord_Whole |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 548 | function (callable) | Coord_Add |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 576 | function (callable) | Coord_Sub |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 602 | function (callable) | Coord_Mid |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 627 | function (callable) | Dir_Facing |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 648 | function (callable) | Facing_Dir |  | (FacingType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 669 | function (callable) | Dir_To_16 |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 691 | function (callable) | Dir_To_32 |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 713 | function (callable) | Direction256 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 736 | function (callable) | Direction |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 759 | function (callable) | Direction8 |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 783 | function (callable) | Direction |  | (CELL cell1,CELL cell2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 807 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,FacingType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 831 | function (callable) | Adjacent_Cell |  | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 855 | function (callable) | Adjacent_Cell |  | (CELL cell,FacingType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 879 | function (callable) | Adjacent_Cell |  | (CELL cell,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 900 | function (callable) | Dir_To_8 |  | (DirType facing) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 921 | function (callable) | Text_String |  | (int string) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 952 | function (callable) | Random_Pick |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 976 | function (callable) | Percent_Chance |  | (int percent) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 1003 | function (callable) | Sim_Random_Pick |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 1027 | function (callable) | Sim_Percent_Chance |  | (int percent) |  | SIGNATURE_CATALOGUED |
| REDALERT/INLINE.H | 1052 | function (callable) | Distance |  | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 54 | function (callable) | Int | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 55 | function (callable) | Int | Int | (unsigned long value) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 57 | function (callable) | Randomize | Int | (Straw & rng,int bitcount) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 58 | function (callable) | Randomize | Int | (Straw & rng,const Int & minval,const Int & maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 65 | function (callable) | operator digit* | Int | () |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 66 | function (callable) | operator const digit* | Int | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 71 | function (callable) | operator [] | Int | (unsigned bit) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 76 | function (callable) | operator ++ | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 77 | function (callable) | operator -- | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 78 | function (callable) | operator ! | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 79 | function (callable) | operator ~ | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 80 | function (callable) | operator - | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 85 | function (callable) | ByteCount | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 86 | function (callable) | BitCount | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 87 | function (callable) | Is_Negative | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 88 | function (callable) | MaxBitPrecision | Int | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 89 | function (callable) | IsSmallPrime | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 90 | function (callable) | SmallDivisorsTest | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 91 | function (callable) | FermatTest | Int | (unsigned rounds) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 92 | function (callable) | IsPrime | Int | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 93 | function (callable) | RabinMillerTest | Int | (Straw & rng,unsigned int rounds) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 98 | function (callable) | operator += | Int | (const Int & number) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 99 | function (callable) | operator -= | Int | (const Int & number) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 100 | function (callable) | operator *= | Int | (const Int & multiplier) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 101 | function (callable) | operator /= | Int | (const Int & t) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 102 | function (callable) | operator %= | Int | (const Int & t) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 103 | function (callable) | operator <<= | Int | (int bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 104 | function (callable) | operator >>= | Int | (int bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 109 | function (callable) | operator + | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 110 | function (callable) | operator + | Int | (unsigned short b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 112 | function (callable) | operator - | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 113 | function (callable) | operator - | Int | (unsigned short b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 115 | function (callable) | operator * | Int | (const Int & multiplier) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 116 | function (callable) | operator * | Int | (unsigned short b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 118 | function (callable) | operator / | Int | (const Int & divisor) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 119 | function (callable) | operator / | Int | (unsigned long b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 120 | function (callable) | operator / | Int | (unsigned short divisor) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 122 | function (callable) | operator % | Int | (const Int & divisor) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 123 | function (callable) | operator % | Int | (unsigned long b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 124 | function (callable) | operator % | Int | (unsigned short divisor) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 130 | function (callable) | operator >> | Int | (int bits) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 131 | function (callable) | operator << | Int | (int bits) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 136 | function (callable) | operator == | Int | (const Int & b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 137 | function (callable) | operator != | Int | (const Int & b) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 138 | function (callable) | operator > | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 139 | function (callable) | operator >= | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 140 | function (callable) | operator < | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 141 | function (callable) | operator <= | Int | (const Int & number) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 146 | function (callable) | Negate | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 147 | function (callable) | Abs | Int | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 148 | function (callable) | times_b_mod_c | Int | (Int const & multiplier,Int const & modulus) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 156 | function (callable) | exp_b_mod_c | Int | (const Int & e,const Int & m) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 163 | function (callable) | Unsigned_Mult | Int | (Int const & multiplicand,Int const & multiplier) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 164 | function (callable) | Unsigned_Divide | Int | (Int & remainder,Int & quotient,const Int & dividend,const Int & divisor) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 165 | function (callable) | Signed_Divide | Int | (Int & remainder,Int & quotient,const Int & dividend,const Int & divisor) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 166 | function (callable) | Inverse | Int | (const Int & modulus) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 168 | function (callable) | Decode_ASCII | Int | (char const * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 171 | function (callable) | Encode | Int | (unsigned char * output) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 172 | function (callable) | Encode | Int | (unsigned char * output,unsigned length) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 173 | function (callable) | Signed_Decode | Int | (const unsigned char * from,int frombytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 174 | function (callable) | Unsigned_Decode | Int | (const unsigned char * from,int frombytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 177 | function (callable) | DEREncode | Int | (unsigned char * output) const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 178 | function (callable) | DERDecode | Int | (const unsigned char * input) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 204 | function (callable) | RemainderTable | Int::RemainderTable | (const Int<PRECISION> & p) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 210 | function (callable) | HasZero | Int::RemainderTable | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 211 | function (callable) | Increment | Int::RemainderTable | (unsigned short increment=1) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 222 | function (callable) | Increment | Int::RemainderTable | (const RemainderTable & rtQ) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 242 | function (callable) | Gcd |  | (const T & a,const T & n) |  | SIGNATURE_CATALOGUED |
| REDALERT/INT.H | 259 | function (callable) | Generate_Prime |  | (Straw & rng,int pbits,T const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 109 | function (callable) | Check_From_WChat |  | (char * wchat_name) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 243 | function (callable) | Read_Game_Options |  | (char * name) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 444 | function (callable) | Just_Path |  | (char * path,char * destpath) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 473 | function (callable) | Is_User_WChat_Registered |  | (char * buffer,int buffer_len) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 568 | function (callable) | Spawn_WChat |  | (bool can_launch) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 716 | function (callable) | Spawn_Registration_App |  | (void) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERNET.CPP | 780 | function (callable) | Do_The_Internet_Menu_Thang |  | (void) | RUST/original-source-reference/REDALERT/INTERNET.md | SIGNATURE_CATALOGUED |
| REDALERT/INTERPAL.CPP | 89 | function (callable) | Read_Interpolation_Palette |  | (char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTERPAL.CPP | 117 | function (callable) | Write_Interpolation_Palette |  | (char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTERPAL.CPP | 144 | function (callable) | Create_Palette_Interpolation_Table |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTERPAL.CPP | 268 | function (callable) | Increase_Palette_Luminance |  | (unsigned char * palette,int red_percentage,int green_percentage,int blue_percentage,int cap) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTERPAL.CPP | 322 | function (callable) | Interpolate_2X_Scale |  | (GraphicBufferClass * source,GraphicViewPortClass * dest,char const * palette_file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTRO.CPP | 40 | function (callable) | Open_Movie |  | (char * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/INTRO.CPP | 73 | function (callable) | Choose_Side |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 66 | function (callable) | Should_Save | CellClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 86 | function (callable) | Load | CellClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 105 | function (callable) | Save | CellClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 132 | function (callable) | Code_Pointers | CellClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 169 | function (callable) | Decode_Pointers | CellClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 212 | function (callable) | Load | MouseClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 323 | function (callable) | Save | MouseClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 389 | function (callable) | Code_Pointers | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 440 | function (callable) | Decode_Pointers | DisplayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 485 | function (callable) | Code_Pointers | MapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOMAP.CPP | 519 | function (callable) | Decode_Pointers | MapClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 88 | function (callable) | Code_Pointers | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 118 | function (callable) | Decode_Pointers | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 151 | function (callable) | Code_Pointers | TeamClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 181 | function (callable) | Decode_Pointers | TeamClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 213 | function (callable) | Code_Pointers | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 238 | function (callable) | Decode_Pointers | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 265 | function (callable) | Code_Pointers | BulletClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 299 | function (callable) | Decode_Pointers | BulletClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 336 | function (callable) | Code_Pointers | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 364 | function (callable) | Decode_Pointers | FactoryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 390 | function (callable) | Load | LayerClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 432 | function (callable) | Save | LayerClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 472 | function (callable) | Code_Pointers | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 500 | function (callable) | Decode_Pointers | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 530 | function (callable) | Code_Pointers | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 553 | function (callable) | Decode_Pointers | HouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 580 | function (callable) | Code_Pointers | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 601 | function (callable) | Decode_Pointers | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 627 | function (callable) | Code_Pointers | FootClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 657 | function (callable) | Decode_Pointers | FootClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 688 | function (callable) | Code_Pointers | RadioClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 719 | function (callable) | Decode_Pointers | RadioClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 753 | function (callable) | Code_Pointers | TechnoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 778 | function (callable) | Decode_Pointers | TechnoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 805 | function (callable) | Code_Pointers | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 834 | function (callable) | Decode_Pointers | CargoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 866 | function (callable) | Code_Pointers | ObjectClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IOOBJ.CPP | 892 | function (callable) | Decode_Pointers | ObjectClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 89 | function (callable) | IPX_SPX_Installed |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 212 | function (callable) | IPX_Open_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 271 | function (callable) | IPX_Close_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 330 | function (callable) | IPX_Get_Connection_Number |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 389 | function (callable) | IPX_Get_1st_Connection_Num |  | (char * username) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 539 | function (callable) | IPX_Get_Internet_Address |  | (int connection_number,unsigned char * network_number,unsigned char * physical_node) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 670 | function (callable) | IPX_Get_User_ID |  | (int connection_number,char * user_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 820 | function (callable) | IPX_Listen_For_Packet |  | (struct ECB * ecb_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 899 | function (callable) | IPX_Send_Packet |  | (struct ECB * ecb_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 960 | function (callable) | IPX_Get_Local_Target |  | (unsigned char * dest_network,unsigned char * dest_node,unsigned short dest_socket,unsigned char * bridge_address) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 1076 | function (callable) | IPX_Cancel_Event |  | (struct ECB * ecb_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX.CPP | 1135 | function (callable) | Let_IPX_Breath |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 53 | function (callable) | IPX_Initialise |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 54 | function (callable) | IPX_Get_Outstanding_Buffer95 |  | (unsigned char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 55 | function (callable) | IPX_Shut_Down95 |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 56 | function (callable) | IPX_Send_Packet95 |  | (unsigned char *,unsigned char *,int,unsigned char *,unsigned char *) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 57 | function (callable) | IPX_Broadcast_Packet95 |  | (unsigned char *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 58 | function (callable) | IPX_Start_Listening95 |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 59 | function (callable) | IPX_Open_Socket95 |  | (int socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 60 | function (callable) | IPX_Close_Socket95 |  | (int socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 61 | function (callable) | IPX_Get_Connection_Number95 |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 62 | function (callable) | IPX_Get_Local_Target95 |  | (unsigned char *,unsigned char *,unsigned short,unsigned char *) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 121 | function (callable) | Load_IPX_Dll |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 175 | function (callable) | Unload_IPX_Dll |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 188 | function (callable) | IPX_Open_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 194 | function (callable) | IPX_Close_Socket |  | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 201 | function (callable) | IPX_Get_Connection_Number |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 209 | function (callable) | IPX_Broadcast_Packet |  | (unsigned char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPX95.CPP | 221 | function (callable) | IPX_Get_Local_Target |  | (unsigned char * dest_network,unsigned char * dest_node,unsigned short dest_socket,unsigned char * bridge_address) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 74 | function (callable) | IPXAddressClass | IPXAddressClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 106 | function (callable) | IPXAddressClass | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 132 | function (callable) | IPXAddressClass | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 156 | function (callable) | Set_Address | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 182 | function (callable) | Set_Address | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 236 | function (callable) | Get_Address | IPXAddressClass | (NetNumType net,NetNodeType node) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 260 | function (callable) | Get_Address | IPXAddressClass | (IPXHeaderType * header) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 283 | function (callable) | Is_Broadcast | IPXAddressClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 323 | function (callable) | operator == | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 381 | function (callable) | operator != | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 434 | function (callable) | operator > | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 456 | function (callable) | operator < | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 478 | function (callable) | operator >= | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXADDR.CPP | 500 | function (callable) | operator <= | IPXAddressClass | (IPXAddressClass & addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 108 | function (callable) | IPXConnClass | IPXConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,IPXAddressClass * address,int id,char * name,int extralen) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 186 | function (callable) | Init | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 226 | function (callable) | Configure | IPXConnClass | (unsigned short socket,int conn_num,ECBType * listen_ecb,ECBType * send_ecb,IPXHeaderType * listen_header,IPXHeaderType * send_header,char * listen_buf,char * send_buf,long handler_rm_ptr,int maxpacketlen) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 271 | function (callable) | Start_Listening | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 393 | function (callable) | Stop_Listening | IPXConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 458 | function (callable) | Send | IPXConnClass | (char * buf,int buflen,void *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 488 | function (callable) | Open_Socket | IPXConnClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 555 | function (callable) | Close_Socket | IPXConnClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 610 | function (callable) | Send_To | IPXConnClass | (char * buf,int buflen,IPXAddressClass * address,NetNodeType immed) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.CPP | 777 | function (callable) | Broadcast | IPXConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXCONN.H | 74 | function (callable) | ~IPXConnClass | IPXConnClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 71 | function (callable) | IPXGlobalConnClass | IPXGlobalConnClass | (int numsend,int numreceive,int maxlen,unsigned short product_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 117 | function (callable) | Send_Packet | IPXGlobalConnClass | (void * buf,int buflen,IPXAddressClass * address,int ack_req) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 190 | function (callable) | Receive_Packet | IPXGlobalConnClass | (void * buf,int buflen,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 341 | function (callable) | Get_Packet | IPXGlobalConnClass | (void * buf,int * buflen,IPXAddressClass * address,unsigned short * product_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 412 | function (callable) | Send | IPXGlobalConnClass | (char * buf,int buflen,void * extrabuf,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 464 | function (callable) | Service_Receive_Queue | IPXGlobalConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.CPP | 505 | function (callable) | Set_Bridge | IPXGlobalConnClass | (NetNumType bridge) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXGCONN.H | 136 | function (callable) | ~IPXGlobalConnClass | IPXGlobalConnClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 114 | function (callable) | IPXManagerClass | IPXManagerClass | (int glb_maxlen,int pvt_maxlen,int glb_num_packets,int pvt_num_packets,unsigned short socket,unsigned short product_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 232 | function (callable) | ~IPXManagerClass | IPXManagerClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 291 | function (callable) | Init | IPXManagerClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 395 | function (callable) | Is_IPX | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 423 | function (callable) | Set_Timing | IPXManagerClass | (unsigned long retrydelta,unsigned long maxretries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 464 | function (callable) | Create_Connection | IPXManagerClass | (int id,char * name,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 517 | function (callable) | Delete_Connection | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 585 | function (callable) | Num_Connections | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 607 | function (callable) | Connection_ID | IPXManagerClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 633 | function (callable) | Connection_Name | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 663 | function (callable) | Connection_Address | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 693 | function (callable) | Connection_Index | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 725 | function (callable) | Set_Connection_Parms | IPXManagerClass | (int index,int id,char * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 754 | function (callable) | Send_Global_Message | IPXManagerClass | (void * buf,int buflen,int ack_req,IPXAddressClass * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 792 | function (callable) | Get_Global_Message | IPXManagerClass | (void * buf,int * buflen,IPXAddressClass * address,unsigned short * product_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 823 | function (callable) | Send_Private_Message | IPXManagerClass | (void * buf,int buflen,int ack_req,int conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 907 | function (callable) | Get_Private_Message | IPXManagerClass | (void * buf,int * buflen,int * conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 975 | function (callable) | Service | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1309 | function (callable) | Get_Bad_Connection | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1331 | function (callable) | Global_Num_Send | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1360 | function (callable) | Global_Num_Receive | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1389 | function (callable) | Private_Num_Send | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1446 | function (callable) | Private_Num_Receive | IPXManagerClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1505 | function (callable) | Set_Socket | IPXManagerClass | (unsigned short socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1528 | function (callable) | Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1561 | function (callable) | Global_Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1588 | function (callable) | Reset_Response_Time | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1617 | function (callable) | Oldest_Send | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1679 | function (callable) | Set_Bridge | IPXManagerClass | (NetNumType bridge) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1714 | function (callable) | Configure_Debug | IPXManagerClass | (int index,int type_offset,int type_size,char ** names,int namestart,int namecount) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1745 | function (callable) | Mono_Debug_Print | IPXManagerClass | (int index,int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 1824 | function (callable) | Alloc_RealMode_Mem | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXMGR.CPP | 2020 | function (callable) | Free_RealMode_Mem | IPXManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXPROT.ASM | 38 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXPROT.ASM | 39 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXPROT.ASM | 40 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXPROT.ASM | 41 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXPROT.ASM | 58 | label (assembly label candidate) | include |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXREAL.ASM | 35 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXREAL.ASM | 36 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXREAL.ASM | 37 | label (assembly label candidate) | P386N |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/IPXREAL.ASM | 38 | label (assembly label candidate) | LOCALS |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 65 | function (callable) | Small_Icon |  | (void const * iconptr,int iconnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 118 | function (callable) | Set_Window |  | (int window,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 144 | function (callable) | Fatal |  | (char const * message,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 159 | function (callable) | File_Fatal |  | (char const * message) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 192 | function (callable) | Load_Uncompress |  | (FileClass & file,BuffType & uncomp_buff,BuffType & dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 269 | function (callable) | Load_Picture |  | (char const * filename,BufferClass & scratchbuf,BufferClass & destbuf,unsigned char * palette,PicturePlaneType) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 292 | function (callable) | Load_Alloc_Data |  | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 321 | function (callable) | Translucent_Table_Size |  | (int count) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 354 | function (callable) | Build_Translucent_Table |  | (PaletteClass const & palette,TLucentType const * control,int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 408 | function (callable) | Conquer_Build_Translucent_Table |  | (PaletteClass const & palette,TLucentType const * control,int count,void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 435 | function (callable) | Make_Fading_Table |  | (PaletteClass const & palette,void * dest,int color,int frac) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 466 | function (callable) | Conquer_Build_Fading_Table |  | (PaletteClass const & palette,void * dest,int color,int frac) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 518 | function (callable) | xDesired_Facing8 |  | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.CPP | 582 | function (callable) | xDesired_Facing256 |  | (int srcx,int srcy,int dstx,int dsty) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 77 | function (callable) | KeyboardClass | KeyboardClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 82 | function (callable) | Get | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 83 | function (callable) | Check | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 84 | function (callable) | To_ASCII | KeyboardClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 85 | function (callable) | Clear | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 86 | function (callable) | Down | KeyboardClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 88 | function (callable) | KeyboardClass | KeyboardClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 89 | function (callable) | Get | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 90 | function (callable) | Check | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 91 | function (callable) | To_ASCII | KeyboardClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 92 | function (callable) | Clear | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 93 | function (callable) | Down | KeyboardClass | (KeyNumType key) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 96 | function (callable) | Mouse_X | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 97 | function (callable) | Mouse_Y | KeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 106 | function (callable) | operator ++ |  | (T & a) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 111 | function (callable) | operator ++ |  | (T & a,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 117 | function (callable) | operator -- |  | (T & a) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 122 | function (callable) | operator -- |  | (T & a,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 128 | function (callable) | operator \| |  | (T t1,T t2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 132 | function (callable) | operator & |  | (T t1,T t2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 136 | function (callable) | operator ~ |  | (T t1) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 142 | function (callable) | min |  | (T value1,T value2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 152 | function (callable) | max |  | (T value1,T value2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 163 | function (callable) | swap |  | (T & value1,T & value2) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 173 | function (callable) | Bound |  | (T original,T minval,T maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 184 | function (callable) | _rotl |  | (T X,int n) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 194 | macro (function-like macro) | ARRAY_LENGTH |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 195 | macro (function-like macro) | ARRAY_SIZE |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 345 | function (callable) | operator ()  | FrameTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 346 | function (callable) | operator long | FrameTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 368 | function (callable) | operator ()  | SystemTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 369 | function (callable) | operator long | SystemTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 382 | function (callable) | operator ()  | UserTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 383 | function (callable) | operator long | UserTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 392 | function (callable) | Bubble_Sort |  | (T * array,int count) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 412 | function (callable) | PBubble_Sort |  | (T * array,int count) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 432 | function (callable) | PNBubble_Sort |  | (T * array,int count) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 455 | function (callable) | SmartPtr | SmartPtr | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 456 | function (callable) | SmartPtr | SmartPtr | (T * realptr=0) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 457 | function (callable) | SmartPtr | SmartPtr | (SmartPtr const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 458 | function (callable) | ~SmartPtr | SmartPtr | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 460 | function (callable) | operator T* | SmartPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 462 | function (callable) | operator long | SmartPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 464 | function (callable) | operator ++ | SmartPtr | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 465 | function (callable) | operator ++ | SmartPtr | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 466 | function (callable) | operator -- | SmartPtr | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 467 | function (callable) | operator -- | SmartPtr | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 469 | function (callable) | operator = | SmartPtr | (SmartPtr const & rvalue) |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 470 | function (callable) | operator -> | SmartPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/JSHELL.H | 471 | function (callable) | operator * | SmartPtr | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 63 | macro (function-like macro) | ARRAY_SIZE |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 76 | function (callable) | WWKeyboardClass | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 100 | function (callable) | Buff_Get | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 127 | function (callable) | Is_Mouse_Key | WWKeyboardClass | (unsigned short key) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 147 | function (callable) | Check | WWKeyboardClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 167 | function (callable) | Get | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 186 | function (callable) | Put | WWKeyboardClass | (unsigned short key) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 208 | function (callable) | Put_Key_Message | WWKeyboardClass | (unsigned short vk_key,bool release) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 263 | function (callable) | Put_Mouse_Message | WWKeyboardClass | (unsigned short vk_key,int x,int y,bool release) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 291 | function (callable) | To_ASCII | WWKeyboardClass | (unsigned short key) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 362 | function (callable) | Down | WWKeyboardClass | (unsigned short key) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 389 | function (callable) | Fetch_Element | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 418 | function (callable) | Peek_Element | WWKeyboardClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 443 | function (callable) | Put_Element | WWKeyboardClass | (unsigned short val) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 470 | function (callable) | Is_Buffer_Full | WWKeyboardClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 493 | function (callable) | Is_Buffer_Empty | WWKeyboardClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 518 | function (callable) | Fill_Buffer_From_System | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 547 | function (callable) | Clear | WWKeyboardClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 588 | function (callable) | Message_Handler | WWKeyboardClass | (HWND window,UINT message,UINT wParam,LONG lParam) |  | SIGNATURE_CATALOGUED |
| REDALERT/KEY.CPP | 745 | function (callable) | Available_Buffer_Room | WWKeyboardClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 51 | macro (function-like macro) | ARRAY_SIZE |  | (x) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 64 | function (callable) | WWKeyboardClass | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 91 | function (callable) | Buff_Get | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 104 | function (callable) | Is_Mouse_Key | WWKeyboardClass | (unsigned short key) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 124 | function (callable) | Check | WWKeyboardClass | (void) const | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 144 | function (callable) | Get | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 163 | function (callable) | Put | WWKeyboardClass | (unsigned short key) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 185 | function (callable) | Put_Key_Message | WWKeyboardClass | (unsigned short vk_key,bool release) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 223 | function (callable) | To_ASCII | WWKeyboardClass | (unsigned short key) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 297 | function (callable) | Down | WWKeyboardClass | (unsigned short key) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 309 | function (callable) | Fetch_Element | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 321 | function (callable) | Peek_Element | WWKeyboardClass | (void) const | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 330 | function (callable) | Put_Element | WWKeyboardClass | (unsigned short val) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 342 | function (callable) | Is_Buffer_Full | WWKeyboardClass | (void) const | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 351 | function (callable) | Is_Buffer_Empty | WWKeyboardClass | (void) const | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 360 | function (callable) | Fill_Buffer_From_System | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 375 | function (callable) | Clear | WWKeyboardClass | (void) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYBOARD.CPP | 381 | function (callable) | Message_Handler | WWKeyboardClass | (HWND,UINT message,UINT wParam,LONG lParam) | RUST/original-source-reference/REDALERT/KEYBOARD.md | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 149 | macro (function-like macro) | USE |  | func varname |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 229 | label (assembly label candidate) | BLIT_TRANSPARENT |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 230 | label (assembly label candidate) | BLIT_GHOST |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 231 | label (assembly label candidate) | BLIT_FADING |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 232 | label (assembly label candidate) | BLIT_PREDATOR |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 233 | label (assembly label candidate) | BLIT_SKIP |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 234 | label (assembly label candidate) | BLIT_ALL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 273 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 274 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 275 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 298 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 299 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 300 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 315 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 318 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 321 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 324 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 327 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 330 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 333 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 336 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 339 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 342 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 345 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 348 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 351 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 354 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 357 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 360 | label (assembly label candidate) | USE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 957 | label (assembly label candidate) | AllFlagsJumpTable |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1022 | label (assembly label candidate) | Setup_Shape_Header |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1044 | label (assembly label candidate) | outer_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1051 | label (assembly label candidate) | inner_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1069 | label (assembly label candidate) | not_transp |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1076 | label (assembly label candidate) | not_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1088 | label (assembly label candidate) | not_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1092 | label (assembly label candidate) | end_lp |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1113 | label (assembly label candidate) | not_all_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1114 | label (assembly label candidate) | no_transparencies |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1191 | label (assembly label candidate) | Buffer_Frame_To_Page |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1253 | label (assembly label candidate) | is_ordinary_shape |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1267 | label (assembly label candidate) | do_args |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1271 | label (assembly label candidate) | check_centering |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1285 | label (assembly label candidate) | check_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1294 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1307 | label (assembly label candidate) | check_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1324 | label (assembly label candidate) | new_shape |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1328 | label (assembly label candidate) | setup_headers |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1340 | label (assembly label candidate) | no_header_setup |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1347 | label (assembly label candidate) | not_shape_predator |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1352 | label (assembly label candidate) | not_shape_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1358 | label (assembly label candidate) | not_shape_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1364 | label (assembly label candidate) | not_shape_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1373 | label (assembly label candidate) | headers_set |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1393 | label (assembly label candidate) | set_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1404 | label (assembly label candidate) | single_fade |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1409 | label (assembly label candidate) | check_predator |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1426 | label (assembly label candidate) | check_range |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1429 | label (assembly label candidate) | pred_cont |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1435 | label (assembly label candidate) | pred_neg_init |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1439 | label (assembly label candidate) | pred_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1452 | label (assembly label candidate) | check_partial |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1461 | label (assembly label candidate) | setupfunc |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1520 | label (assembly label candidate) | dest_left_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1529 | label (assembly label candidate) | dest_bottom_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1536 | label (assembly label candidate) | dest_right_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1543 | label (assembly label candidate) | do_blit |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1590 | label (assembly label candidate) | use_old_stuff |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1602 | label (assembly label candidate) | real_out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1608 | label (assembly label candidate) | no_mmx_cleanup |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1622 | label (assembly label candidate) | BF_Copy |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1628 | label (assembly label candidate) | forward_loop_dword |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1649 | label (assembly label candidate) | forward_loop_bytes |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1690 | label (assembly label candidate) | slt_mask_map_lp |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1705 | label (assembly label candidate) | slt_end_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1710 | label (assembly label candidate) | slt_skip |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1728 | label (assembly label candidate) | slt_skip2 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1736 | label (assembly label candidate) | slt_round_again |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1751 | label (assembly label candidate) | slt_skip_end_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1766 | label (assembly label candidate) | Long_Single_Line_Copy |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1776 | label (assembly label candidate) | LSLC_aligned |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1790 | label (assembly label candidate) | proc_out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1801 | label (assembly label candidate) | Short_Single_Line_Copy |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1819 | label (assembly label candidate) | Single_Line_Skip |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1834 | label (assembly label candidate) | Single_Line_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1838 | label (assembly label candidate) | slg_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1850 | label (assembly label candidate) | slg_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1867 | label (assembly label candidate) | Single_Line_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1873 | label (assembly label candidate) | slgt_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1878 | label (assembly label candidate) | slgt_not_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1889 | label (assembly label candidate) | slgt_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1900 | label (assembly label candidate) | slgt_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1905 | label (assembly label candidate) | slgt_round_again |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1918 | label (assembly label candidate) | slgt_out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1928 | label (assembly label candidate) | slgt4 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1949 | label (assembly label candidate) | slgt4_store1 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1951 | label (assembly label candidate) | slgt4_trans1 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1981 | label (assembly label candidate) | Single_Line_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 1989 | label (assembly label candidate) | slf_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2016 | label (assembly label candidate) | Single_Line_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2024 | label (assembly label candidate) | slft_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2031 | label (assembly label candidate) | slft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2037 | label (assembly label candidate) | slft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2058 | label (assembly label candidate) | Single_Line_Single_Fade |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2063 | label (assembly label candidate) | slsf_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2083 | label (assembly label candidate) | Single_Line_Single_Fade_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2088 | label (assembly label candidate) | slsft_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2102 | label (assembly label candidate) | slsft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2117 | label (assembly label candidate) | slsft_next_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2132 | label (assembly label candidate) | Single_Line_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2137 | label (assembly label candidate) | SLGF_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2150 | label (assembly label candidate) | slgf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2154 | label (assembly label candidate) | slgf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2176 | label (assembly label candidate) | Single_Line_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2184 | label (assembly label candidate) | SLGFT_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2199 | label (assembly label candidate) | slgft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2203 | label (assembly label candidate) | slgft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2209 | label (assembly label candidate) | slgft_trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2220 | label (assembly label candidate) | slgft4 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2242 | label (assembly label candidate) | slgft4_fade |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2245 | label (assembly label candidate) | slgft4_fade_lp |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2251 | label (assembly label candidate) | slgft4_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2262 | label (assembly label candidate) | slgft4_out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2273 | label (assembly label candidate) | Single_Line_Predator |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2277 | label (assembly label candidate) | slp_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2287 | label (assembly label candidate) | slp_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2298 | label (assembly label candidate) | slp_skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2318 | label (assembly label candidate) | Single_Line_Predator_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2322 | label (assembly label candidate) | slpt_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2346 | label (assembly label candidate) | slpt_skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2363 | label (assembly label candidate) | Single_Line_Predator_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2367 | label (assembly label candidate) | slpg_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2376 | label (assembly label candidate) | slpg_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2386 | label (assembly label candidate) | slpg_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2400 | label (assembly label candidate) | slpg_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2420 | label (assembly label candidate) | Single_Line_Predator_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2423 | label (assembly label candidate) | slpgt_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2436 | label (assembly label candidate) | slpgt_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2446 | label (assembly label candidate) | slpgt_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2460 | label (assembly label candidate) | slpgt_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2462 | label (assembly label candidate) | slpgt_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2480 | label (assembly label candidate) | Single_Line_Predator_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2485 | label (assembly label candidate) | slpf_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2505 | label (assembly label candidate) | slpf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2510 | label (assembly label candidate) | slpf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2533 | label (assembly label candidate) | Single_Line_Predator_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2537 | label (assembly label candidate) | slpft_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2549 | label (assembly label candidate) | slpft_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2560 | label (assembly label candidate) | slpft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2565 | label (assembly label candidate) | slpft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2571 | label (assembly label candidate) | slpft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2589 | label (assembly label candidate) | Single_Line_Predator_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2594 | label (assembly label candidate) | slpgf_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2604 | label (assembly label candidate) | slpgf_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2615 | label (assembly label candidate) | slpgf_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2629 | label (assembly label candidate) | slpgf_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2634 | label (assembly label candidate) | slpgf_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2639 | label (assembly label candidate) | slpgf_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2658 | label (assembly label candidate) | Single_Line_Predator_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2663 | label (assembly label candidate) | slpgft_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2676 | label (assembly label candidate) | slpgft_get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2687 | label (assembly label candidate) | slpgft_check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2701 | label (assembly label candidate) | slpgft_do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2706 | label (assembly label candidate) | slpgft_fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2711 | label (assembly label candidate) | slpgft_store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2713 | label (assembly label candidate) | slpgft_transparent |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2733 | label (assembly label candidate) | BF_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2746 | label (assembly label candidate) | trans_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2753 | label (assembly label candidate) | trans_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2764 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2768 | label (assembly label candidate) | trans_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2784 | label (assembly label candidate) | BF_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2807 | label (assembly label candidate) | ghost_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2811 | label (assembly label candidate) | ghost_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2830 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2836 | label (assembly label candidate) | ghost_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2853 | label (assembly label candidate) | BF_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2876 | label (assembly label candidate) | ghost_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2880 | label (assembly label candidate) | ghost_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2903 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2906 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2911 | label (assembly label candidate) | ghost_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2928 | label (assembly label candidate) | BF_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2951 | label (assembly label candidate) | fading_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2957 | label (assembly label candidate) | fading_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2960 | label (assembly label candidate) | fading_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2968 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2978 | label (assembly label candidate) | fading_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 2996 | label (assembly label candidate) | BF_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3019 | label (assembly label candidate) | fading_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3025 | label (assembly label candidate) | fading_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3028 | label (assembly label candidate) | fading_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3040 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3047 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3052 | label (assembly label candidate) | fading_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3070 | label (assembly label candidate) | BF_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3093 | label (assembly label candidate) | ghost_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3098 | label (assembly label candidate) | ghost_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3101 | label (assembly label candidate) | ghost_f_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3123 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3127 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3132 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3138 | label (assembly label candidate) | ghost_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3156 | label (assembly label candidate) | BF_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3179 | label (assembly label candidate) | ghost_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3184 | label (assembly label candidate) | ghost_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3187 | label (assembly label candidate) | ghost_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3213 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3217 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3222 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3225 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3230 | label (assembly label candidate) | ghost_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3248 | label (assembly label candidate) | BF_Predator |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3271 | label (assembly label candidate) | predator_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3275 | label (assembly label candidate) | predator_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3290 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3309 | label (assembly label candidate) | skip_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3314 | label (assembly label candidate) | predator_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3331 | label (assembly label candidate) | BF_Predator_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3354 | label (assembly label candidate) | predator_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3358 | label (assembly label candidate) | predator_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3377 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3388 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3391 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3396 | label (assembly label candidate) | predator_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3413 | label (assembly label candidate) | BF_Predator_Ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3436 | label (assembly label candidate) | predator_g_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3440 | label (assembly label candidate) | predator_g_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3456 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3467 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3481 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3487 | label (assembly label candidate) | predator_g_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3504 | label (assembly label candidate) | BF_Predator_Ghost_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3527 | label (assembly label candidate) | predator_g_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3531 | label (assembly label candidate) | predator_g_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3551 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3562 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3576 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3579 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3584 | label (assembly label candidate) | predator_g_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3601 | label (assembly label candidate) | BF_Predator_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3624 | label (assembly label candidate) | predator_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3629 | label (assembly label candidate) | predator_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3632 | label (assembly label candidate) | predator_f_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3648 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3659 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3663 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3673 | label (assembly label candidate) | predator_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3691 | label (assembly label candidate) | BF_Predator_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3714 | label (assembly label candidate) | predator_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3719 | label (assembly label candidate) | predator_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3722 | label (assembly label candidate) | predator_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3742 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3753 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3757 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3764 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3769 | label (assembly label candidate) | predator_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3787 | label (assembly label candidate) | BF_Predator_Ghost_Fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3810 | label (assembly label candidate) | predator_g_f_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3815 | label (assembly label candidate) | predator_g_f_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3818 | label (assembly label candidate) | predator_g_f_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3836 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3847 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3862 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3866 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3871 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3877 | label (assembly label candidate) | predator_g_f_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3895 | label (assembly label candidate) | BF_Predator_Ghost_Fading_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3918 | label (assembly label candidate) | predator_g_f_t_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3923 | label (assembly label candidate) | predator_g_f_t_line_begin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3926 | label (assembly label candidate) | predator_g_f_t_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3948 | label (assembly label candidate) | get_pred |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3959 | label (assembly label candidate) | check_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3974 | label (assembly label candidate) | do_fading |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3978 | label (assembly label candidate) | fade_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3983 | label (assembly label candidate) | store_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3986 | label (assembly label candidate) | trans_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 3991 | label (assembly label candidate) | predator_g_f_t_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4008 | label (assembly label candidate) | Not_Supported |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4016 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4036 | label (assembly label candidate) | Detect_MMX_Availability |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4153 | label (assembly label candidate) | Init_MMX |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4193 | label (assembly label candidate) | MMX_Done |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4228 | label (assembly label candidate) | MMX_Single_Line_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4473 | label (assembly label candidate) | ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4492 | label (assembly label candidate) | no_ghost |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4509 | label (assembly label candidate) | no_centering |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4521 | label (assembly label candidate) | test_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4527 | label (assembly label candidate) | test_priority |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4536 | label (assembly label candidate) | priority |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4547 | label (assembly label candidate) | no_priority |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4604 | label (assembly label candidate) | get_buffer |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4616 | label (assembly label candidate) | jump_exit |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4619 | label (assembly label candidate) | clip_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4629 | label (assembly label candidate) | check_bottom |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4639 | label (assembly label candidate) | clip_x_left |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4649 | label (assembly label candidate) | clip_x_right |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4664 | label (assembly label candidate) | draw_prep |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4705 | label (assembly label candidate) | top_of_loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4721 | label (assembly label candidate) | proc_exit |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4792 | label (assembly label candidate) | loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4862 | label (assembly label candidate) | fini |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4876 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4882 | label (assembly label candidate) | skip |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4918 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4931 | label (assembly label candidate) | out_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4965 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4979 | label (assembly label candidate) | write_back |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 4981 | label (assembly label candidate) | out_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5007 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5036 | label (assembly label candidate) | normal_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5044 | label (assembly label candidate) | skip |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5070 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5101 | label (assembly label candidate) | normal_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5109 | label (assembly label candidate) | skip |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5143 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5154 | label (assembly label candidate) | out_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5189 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5199 | label (assembly label candidate) | write_back |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/KEYFBUFF.ASM | 5201 | label (assembly label candidate) | out_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LAYER.CPP | 59 | function (callable) | Submit | LayerClass | (ObjectClass const * object,bool sort) |  | SIGNATURE_CATALOGUED |
| REDALERT/LAYER.CPP | 89 | function (callable) | Sort | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LAYER.CPP | 118 | function (callable) | Sorted_Add | LayerClass | (ObjectClass const * const object) |  | SIGNATURE_CATALOGUED |
| REDALERT/LAYER.H | 52 | function (callable) | Init | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LAYER.H | 53 | function (callable) | One_Time | LayerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCW.CPP | 69 | function (callable) | LCW_Uncomp |  | (void const * source,void * dest,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 41 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 63 | label (assembly label candidate) | LCW_Comp |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 113 | label (assembly label candidate) | _loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 117 | label (assembly label candidate) | searchloop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 150 | label (assembly label candidate) | notlongenough |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 152 | label (assembly label candidate) | notrunlength |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 154 | label (assembly label candidate) | oploop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 181 | label (assembly label candidate) | notend |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 196 | label (assembly label candidate) | searchdone |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 211 | label (assembly label candidate) | shortrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 220 | label (assembly label candidate) | medrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 230 | label (assembly label candidate) | lenin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 234 | label (assembly label candidate) | lenin1 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 239 | label (assembly label candidate) | len |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 244 | label (assembly label candidate) | stolen |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 251 | label (assembly label candidate) | longrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 257 | label (assembly label candidate) | medrunnxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 260 | label (assembly label candidate) | srunnxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 268 | label (assembly label candidate) | nxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWCOMP.ASM | 274 | label (assembly label candidate) | _out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWPIPE.CPP | 63 | function (callable) | LCWPipe | LCWPipe | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWPIPE.CPP | 91 | function (callable) | ~LCWPipe | LCWPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWPIPE.CPP | 120 | function (callable) | Put | LCWPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWPIPE.CPP | 256 | function (callable) | Flush | LCWPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWSTRAW.CPP | 63 | function (callable) | LCWStraw | LCWStraw | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWSTRAW.CPP | 92 | function (callable) | ~LCWStraw | LCWStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWSTRAW.CPP | 123 | function (callable) | Get | LCWStraw | (void * destbuf,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LCWUNCMP.CPP | 70 | function (callable) | LCW_Uncompress |  | (void * source,void * dest,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/LED.H | 35 | function (callable) | Scroll_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| REDALERT/LED.H | 36 | function (callable) | Caps_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| REDALERT/LED.H | 37 | function (callable) | Num_Lock | LEDClass | (ControlType control=LED_TOGGLE) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 67 | function (callable) | LinkClass | LinkClass | (LinkClass const & link) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 92 | function (callable) | ~LinkClass | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 115 | function (callable) | Zap | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 144 | function (callable) | operator = | LinkClass | (LinkClass const & link) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 170 | function (callable) | Get_Next | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 191 | function (callable) | Get_Prev | LinkClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 212 | function (callable) | Head_Of_List | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 238 | function (callable) | Tail_Of_List | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 265 | function (callable) | Add | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 305 | function (callable) | Add_Head | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 338 | function (callable) | Add_Tail | LinkClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.CPP | 375 | function (callable) | Remove | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.H | 47 | function (callable) | LinkClass | LinkClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/LINK.H | 48 | function (callable) | LinkClass | LinkClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 79 | function (callable) | ListClass | ListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 112 | function (callable) | ListClass | ListClass | (ListClass const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 132 | function (callable) | Set_Position | ListClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 156 | function (callable) | ~ListClass | ListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 173 | function (callable) | Add_Item | ListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 211 | function (callable) | Add_Item | ListClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 220 | function (callable) | Remove_Item | ListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 279 | function (callable) | Remove_Item | ListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 299 | function (callable) | Action | ListClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 356 | function (callable) | Draw_Me | ListClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 413 | function (callable) | Bump | ListClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 438 | function (callable) | Step | ListClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 464 | function (callable) | Get_Item | ListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 488 | function (callable) | Current_Item | ListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 512 | function (callable) | Current_Index | ListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 538 | function (callable) | Peer_To_Peer | ListClass | (unsigned flags,KeyNumType &,ControlClass & whom) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 575 | function (callable) | Set_View_Index | ListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 605 | function (callable) | Add_Scroll_Bar | ListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 673 | function (callable) | Remove_Scroll_Bar | ListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 705 | function (callable) | Set_Tabs | ListClass | (int const * tabs) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 732 | function (callable) | Draw_Entry | ListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 769 | function (callable) | Add | ListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 800 | function (callable) | Add_Head | ListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 837 | function (callable) | Add_Tail | ListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 874 | function (callable) | Remove | ListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 907 | function (callable) | Set_Selected_Index | ListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 939 | function (callable) | Step_Selected_Index | ListClass | (int step) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 948 | function (callable) | Flag_To_Redraw | ListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.CPP | 959 | function (callable) | Set_Selected_Index | ListClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 67 | function (callable) | Count | ListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 157 | function (callable) | operator []  | TListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 158 | function (callable) | operator []  | TListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 164 | function (callable) | Count | TListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 170 | function (callable) | Get_Item | TListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 245 | function (callable) | TListClass | TListClass | (int id,int x,int y,int w,int h,TextPrintType flags,void const * up,void const * down) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 277 | function (callable) | TListClass | TListClass | (TListClass<T> const & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 297 | function (callable) | Set_Position | TListClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 310 | function (callable) | ~TListClass | TListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 316 | function (callable) | Insert_Item | TListClass | (T item) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 339 | function (callable) | Add_Item | TListClass | (T text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 364 | function (callable) | Remove_Index | TListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 408 | function (callable) | Remove_Item | TListClass | (T text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 414 | function (callable) | Action | TListClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 454 | function (callable) | Draw_Me | TListClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 498 | function (callable) | Bump | TListClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 509 | function (callable) | Step | TListClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 521 | function (callable) | Get_Item | TListClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 529 | function (callable) | Current_Item | TListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 539 | function (callable) | Current_Index | TListClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 545 | function (callable) | Peer_To_Peer | TListClass | (unsigned flags,KeyNumType &,ControlClass & whom) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 566 | function (callable) | Set_View_Index | TListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 581 | function (callable) | Add_Scroll_Bar | TListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 634 | function (callable) | Remove_Scroll_Bar | TListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 649 | function (callable) | Set_Tabs | TListClass | (int const * tabs) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 656 | function (callable) | Draw_Entry | TListClass | (int index,int x,int y,int width,int selected) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 679 | function (callable) | Add | TListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 697 | function (callable) | Add_Head | TListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 715 | function (callable) | Add_Tail | TListClass | (LinkClass & list) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 735 | function (callable) | Remove | TListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 753 | function (callable) | Set_Selected_Index | TListClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 768 | function (callable) | Step_Selected_Index | TListClass | (int step) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 777 | function (callable) | Flag_To_Redraw | TListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LIST.H | 788 | function (callable) | Set_Selected_Index | TListClass | (T text) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 66 | function (callable) | GenericNode | GenericNode | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 67 | function (callable) | ~GenericNode | GenericNode | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 68 | function (callable) | GenericNode | GenericNode | (GenericNode & node) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 69 | function (callable) | operator = | GenericNode | (GenericNode & node) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 76 | function (callable) | Unlink | GenericNode | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 85 | function (callable) | Main_List | GenericNode | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 92 | function (callable) | Link | GenericNode | (GenericNode * node) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 101 | function (callable) | Next | GenericNode | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 102 | function (callable) | Prev | GenericNode | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 103 | function (callable) | Is_Valid | GenericNode | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 117 | function (callable) | GenericList | GenericList | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 121 | function (callable) | First | GenericList | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 122 | function (callable) | Last | GenericList | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 123 | function (callable) | Is_Empty | GenericList | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 124 | function (callable) | Add_Head | GenericList | (GenericNode * node) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 125 | function (callable) | Add_Tail | GenericList | (GenericNode * node) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 126 | function (callable) | Delete | GenericList | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 151 | function (callable) | Main_List | Node | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 152 | function (callable) | Next | Node | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 153 | function (callable) | Prev | Node | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 154 | function (callable) | Is_Valid | Node | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 165 | function (callable) | First | List | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LISTNODE.H | 166 | function (callable) | Last | List | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 61 | function (callable) | LoadOptionsClass | LoadOptionsClass | (LoadStyleType style) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 83 | function (callable) | ~LoadOptionsClass | LoadOptionsClass | () | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 107 | function (callable) | Process | LoadOptionsClass | (void) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 578 | function (callable) | Clear_List | LoadOptionsClass | (ListClass * list) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 614 | function (callable) | Fill_List | LoadOptionsClass | (ListClass * list) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 747 | function (callable) | Num_From_Ext | LoadOptionsClass | (char * fname) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOADDLG.CPP | 772 | function (callable) | Compare | LoadOptionsClass | (const void * p1,const void * p2) | RUST/original-source-reference/REDALERT/LOADDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/LOGIC.CPP | 62 | function (callable) | Debug_Dump | LogicClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/LOGIC.md | SIGNATURE_CATALOGUED |
| REDALERT/LOGIC.CPP | 195 | function (callable) | AI | LogicClass | (void) | RUST/original-source-reference/REDALERT/LOGIC.md | SIGNATURE_CATALOGUED |
| REDALERT/LOGIC.CPP | 441 | function (callable) | Detach | LogicClass | (TARGET target,bool) | RUST/original-source-reference/REDALERT/LOGIC.md | SIGNATURE_CATALOGUED |
| REDALERT/LOGIC.CPP | 470 | function (callable) | Clear_Recently_Created_Bits | LogicClass | (void) | RUST/original-source-reference/REDALERT/LOGIC.md | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 75 | macro (function-like macro) | _DV2 |  | (p,shift1,shift2) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 77 | macro (function-like macro) | DVAL_NEXT |  | (dv,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 79 | macro (function-like macro) | _DV |  | (p,shift) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 80 | macro (function-like macro) | DVAL_FIRST |  | (dv,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 81 | macro (function-like macro) | _DINDEX |  | (dv,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 82 | macro (function-like macro) | DINDEX |  | (dv,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 83 | macro (function-like macro) | UPDATE_D |  | (dict,cycle,dv,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 84 | macro (function-like macro) | UPDATE_I |  | (dict,cycle,index,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 91 | function (callable) | do_compress |  | (const lzo_byte * in,lzo_uint in_len,lzo_byte * out,lzo_uint * out_len,lzo_voidp wrkmem) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_C.CPP | 315 | function (callable) | lzo1x_1_compress |  | (const lzo_byte * in,lzo_uint in_len,lzo_byte * out,lzo_uint * out_len,lzo_voidp wrkmem) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO1X_D.CPP | 63 | function (callable) | lzo1x_decompress |  | (const lzo_byte * in,lzo_uint in_len,lzo_byte * out,lzo_uint * out_len,lzo_voidp) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOCONF.H | 119 | macro (function-like macro) | LZO_ALIGN |  | (ptr,size) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOCONF.H | 142 | macro (function-like macro) | LZO_EXTERN |  | (_rettype) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOPIPE.CPP | 64 | function (callable) | LZOPipe | LZOPipe | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOPIPE.CPP | 92 | function (callable) | ~LZOPipe | LZOPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOPIPE.CPP | 121 | function (callable) | Put | LZOPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOPIPE.CPP | 262 | function (callable) | Flush | LZOPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOSTRAW.CPP | 63 | function (callable) | LZOStraw | LZOStraw | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOSTRAW.CPP | 92 | function (callable) | ~LZOStraw | LZOStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZOSTRAW.CPP | 123 | function (callable) | Get | LZOStraw | (void * destbuf,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 88 | macro (function-like macro) | BOUNDS_CHECKING_OFF_DURING |  | (stmt) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 89 | macro (function-like macro) | BOUNDS_CHECKING_OFF_IN_EXPR |  | (expr) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 101 | macro (function-like macro) | LZO_UNUSED |  | (parm) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 103 | macro (function-like macro) | LZO_UNUSED |  | (parm) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 197 | macro (function-like macro) | LZO_BYTE |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 199 | macro (function-like macro) | LZO_MAX |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 200 | macro (function-like macro) | LZO_MIN |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 202 | macro (function-like macro) | lzo_sizeof |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 204 | macro (function-like macro) | LZO_HIGH |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 207 | macro (function-like macro) | LZO_SIZE |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 208 | macro (function-like macro) | LZO_MASK |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 210 | macro (function-like macro) | LZO_LSIZE |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 211 | macro (function-like macro) | LZO_LMASK |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 213 | macro (function-like macro) | LZO_USIZE |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 214 | macro (function-like macro) | LZO_UMASK |  | (bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 221 | macro (function-like macro) | _LZO_STRINGIZE |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 222 | macro (function-like macro) | _LZO_MEXPAND |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 225 | macro (function-like macro) | _LZO_CONCAT2 |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 226 | macro (function-like macro) | _LZO_CONCAT3 |  | (a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 227 | macro (function-like macro) | _LZO_CONCAT4 |  | (a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 228 | macro (function-like macro) | _LZO_CONCAT5 |  | (a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 231 | macro (function-like macro) | _LZO_ECONCAT2 |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 232 | macro (function-like macro) | _LZO_ECONCAT3 |  | (a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 233 | macro (function-like macro) | _LZO_ECONCAT4 |  | (a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 234 | macro (function-like macro) | _LZO_ECONCAT5 |  | (a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 263 | macro (function-like macro) | LZO_CHECK_MPOS_DET |  | (m_pos,m_off,in,ip,max_offset) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZO_CONF.H | 272 | macro (function-like macro) | LZO_CHECK_MPOS_NON_DET |  | (m_pos,m_off,in,ip,max_offset) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 46 | function (callable) | LZWEngine | LZWEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 52 | function (callable) | Reset | LZWEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 59 | function (callable) | Compress | LZWEngine | (Buffer const & input,Buffer const & output) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 133 | function (callable) | Uncompress | LZWEngine | (Buffer const & input,Buffer const & output) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 189 | function (callable) | Make_LZW_Hash | LZWEngine | (CodeType code,char character) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 195 | function (callable) | Find_Child_Node | LZWEngine | (CodeType parent_code,char child_character) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 248 | function (callable) | Decode_String | LZWEngine | (char * ptr,CodeType code) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 262 | function (callable) | LZW_Uncompress |  | (Buffer const & inbuff,Buffer const & outbuff) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 269 | function (callable) | LZW_Compress |  | (Buffer const & inbuff,Buffer const & outbuff) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 309 | function (callable) | CodeClass | CodeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 310 | function (callable) | CodeClass | CodeClass | (CodeType code,CodeType parent,char c) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 312 | function (callable) | Make_Unused | CodeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 313 | function (callable) | Is_Unused | CodeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 314 | function (callable) | Is_Matching | CodeClass | (CodeType code,char c) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 320 | function (callable) | Make_LZW_Hash |  | (CodeType code,char character) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 348 | function (callable) | Find_Child_Node |  | (CodeType parent_code,char child_character) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 406 | function (callable) | Decode_String |  | (char * ptr,CodeType code) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 430 | function (callable) | LZW_Compress |  | (Buffer & inbuff,Buffer & outbuff) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.CPP | 483 | function (callable) | LZW_Uncompress |  | (Buffer & inbuff,Buffer & outbuff) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.H | 57 | function (callable) | CodeClass | LZWEngine::CodeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.H | 58 | function (callable) | CodeClass | LZWEngine::CodeClass | (CodeType code,CodeType parent,char c) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.H | 61 | function (callable) | Make_Unused | LZWEngine::CodeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.H | 62 | function (callable) | Is_Unused | LZWEngine::CodeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LZW.H | 63 | function (callable) | Is_Matching | LZWEngine::CodeClass | (CodeType code,char c) const |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWOTRAW.CPP | 64 | function (callable) | LZWStraw | LZWStraw | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWOTRAW.CPP | 94 | function (callable) | ~LZWStraw | LZWStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWOTRAW.CPP | 125 | function (callable) | Get | LZWStraw | (void * destbuf,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWPIPE.CPP | 64 | function (callable) | LZWPipe | LZWPipe | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWPIPE.CPP | 93 | function (callable) | ~LZWPipe | LZWPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWPIPE.CPP | 122 | function (callable) | Put | LZWPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWPIPE.CPP | 258 | function (callable) | Flush | LZWPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWSTRAW.CPP | 63 | function (callable) | LZWStraw | LZWStraw | (CompControl control,int blocksize) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWSTRAW.CPP | 93 | function (callable) | ~LZWStraw | LZWStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/LZWSTRAW.CPP | 124 | function (callable) | Get | LZWStraw | (void * destbuf,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 102 | function (callable) | One_Time | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 132 | function (callable) | Init_Clear | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 165 | function (callable) | Alloc_Cells | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 195 | function (callable) | Free_Cells | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 220 | function (callable) | Init_Cells | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 248 | function (callable) | Set_Map_Dimensions | MapClass | (int x,int y,int w,int h) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 283 | function (callable) | Sight_From | MapClass | (CELL cell,int sightrange,HouseClass * house,bool incremental) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 363 | function (callable) | Shroud_From | MapClass | (CELL cell,int sightrange,HouseClass * house) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.CPP | 434 | function (callable) | Jam_From | MapClass | (CELL cell,int jamrange,HouseClass * house) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 45 | function (callable) | MapClass | MapClass | (void) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 46 | function (callable) | MapClass | MapClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 66 | function (callable) | Detach | MapClass | (ObjectClass *) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 120 | function (callable) | operator []  | MapClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 121 | function (callable) | operator []  | MapClass | (CELL cell) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 122 | function (callable) | operator []  | MapClass | (COORDINATE coord) const | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 123 | function (callable) | operator []  | MapClass | (CELL cell) const | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 124 | function (callable) | ID | MapClass | (CellClass const * ptr) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAP.H | 125 | function (callable) | ID | MapClass | (CellClass const & ptr) | RUST/original-source-reference/REDALERT/MAP.md | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDDLG.CPP | 71 | function (callable) | New_Scenario | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDDLG.CPP | 207 | function (callable) | Load_Scenario | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 102 | function (callable) | MapEditClass | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 142 | function (callable) | One_Time | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 234 | function (callable) | Init_IO | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 271 | function (callable) | Clear_List | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 305 | function (callable) | Add_To_List | MapEditClass | (ObjectTypeClass const * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 427 | function (callable) | AI | MapEditClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1381 | function (callable) | Draw_It | MapEditClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1457 | function (callable) | Mouse_Moved | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1526 | function (callable) | Main_Menu | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1718 | function (callable) | AI_Menu | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1808 | function (callable) | Verify_House | MapEditClass | (HousesType house,ObjectTypeClass const * objtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1833 | function (callable) | Cycle_House | MapEditClass | (HousesType curhouse,ObjectTypeClass const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1889 | function (callable) | Fatal | MapEditClass | (int txt) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1897 | function (callable) | Scroll_Map | MapEditClass | (DirType facing,int & distance,bool really) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1912 | function (callable) | Flag_To_Redraw | MapEditClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1919 | function (callable) | Detach | MapEditClass | (ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 1926 | function (callable) | Get_Waypoint_Name | MapEditClass | (char wayptname[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 2105 | function (callable) | Update_Waypoint | MapEditClass | (int waypt_idx) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 2144 | function (callable) | Read_INI | MapEditClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.CPP | 2161 | function (callable) | Write_INI | MapEditClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.H | 180 | function (callable) | MapEditClass | MapEditClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDIT.H | 192 | function (callable) | Detach | MapEditClass | (TARGET target,bool all=true) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 100 | function (callable) | Placement_Dialog | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 667 | function (callable) | Start_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 773 | function (callable) | Place_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1004 | function (callable) | Cancel_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1048 | function (callable) | Place_Next | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1122 | function (callable) | Place_Prev | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1194 | function (callable) | Place_Next_Category | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1284 | function (callable) | Place_Prev_Category | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1395 | function (callable) | Place_Home | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1469 | function (callable) | Toggle_House | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1528 | function (callable) | Set_House_Buttons | MapEditClass | (HousesType house,GadgetClass *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1577 | function (callable) | Start_Trigger_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1599 | function (callable) | Stop_Trigger_Placement | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1622 | function (callable) | Place_Trigger | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1698 | function (callable) | Start_Base_Building | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1734 | function (callable) | Cancel_Base_Building | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDPLC.CPP | 1770 | function (callable) | Build_Base_To | MapEditClass | (int percent) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 63 | function (callable) | Select_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 160 | function (callable) | Select_Next | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 269 | function (callable) | Popup_Controls | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 405 | function (callable) | Grab_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 436 | function (callable) | Move_Grabbed_Object | MapEditClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDSEL.CPP | 531 | function (callable) | Change_House | MapEditClass | (HousesType newhouse) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDTM.CPP | 59 | function (callable) | Handle_Teams | MapEditClass | (char const * caption) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDTM.CPP | 183 | function (callable) | Select_Team | MapEditClass | (char const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDTM.CPP | 458 | function (callable) | Team_Members | MapEditClass | (HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPEDTM.CPP | 912 | function (callable) | Draw_Member | MapEditClass | (TechnoTypeClass const * ptr,int index,int quant,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPSEL.CPP | 108 | function (callable) | Map_Selection |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPSEL.CPP | 323 | function (callable) | Mouse_Over_Spot |  | (int house,int scenario) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPSEL.CPP | 340 | function (callable) | Cycle_Call_Back_Delay |  | (int time,PaletteClass & pal) |  | SIGNATURE_CATALOGUED |
| REDALERT/MAPSEL.CPP | 381 | function (callable) | Set_Mouse |  | (MouseType shape,int & start,int & count,int & delay,int & xspot,int & yspot) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 51 | function (callable) | GetDeviceCount | MCI | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 87 | function (callable) | GetDeviceName | MCI | (unsigned int item,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 120 | function (callable) | GetProductName | MCI | (MCIDEVICEID id,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 155 | function (callable) | OpenDevice | MCI | (const char * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 172 | function (callable) | CloseDevice | MCI | (MCIDEVICEID id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 196 | function (callable) | GetDeviceDescription | MCI | (const char * name,MCIDevice * caps) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 256 | function (callable) | GetCapability | MCI | (MCIDEVICEID id,unsigned long capItem,unsigned long * result) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 287 | function (callable) | GetDeviceTypeName | MCI | (unsigned long type) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCI.CPP | 336 | function (callable) | EnumerateDevices | MCI | (MCIEnumCB * callback,void * context) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 37 | function (callable) | MCIMovie | MCIMovie | (HWND mainWindow) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 57 | function (callable) | ~MCIMovie | MCIMovie | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 84 | function (callable) | Open | MCIMovie | (const char * name,const char * device) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 169 | function (callable) | Play | MCIMovie | (HWND window) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 217 | function (callable) | Pause | MCIMovie | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 243 | function (callable) | Close | MCIMovie | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 277 | function (callable) | SizeDestination | MCIMovie | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.CPP | 323 | function (callable) | AttachWindow | MCIMovie | (HWND window) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.H | 39 | function (callable) | GetWidth | MCIMovie | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MCIMOVIE.H | 42 | function (callable) | GetHeight | MCIMovie | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 140 | macro (function-like macro) | mc_register_near |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 141 | macro (function-like macro) | mc_register_far |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 142 | macro (function-like macro) | mc_unregister_near |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 143 | macro (function-like macro) | mc_unregister_far |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 144 | macro (function-like macro) | mc_check_near |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 145 | macro (function-like macro) | mc_check_far |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 181 | macro (function-like macro) | MC_EFLAG |  | (e) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 595 | macro (function-like macro) | mc_version |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 596 | macro (function-like macro) | mc_revision |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 609 | macro (function-like macro) | _MCID |  | (f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 794 | macro (function-like macro) | TOUCH |  | (var) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 796 | macro (function-like macro) | TOUCH |  | (var) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 798 | macro (function-like macro) | TOUCH |  | (var) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1011 | macro (function-like macro) | NO_OVERLAP |  | (mcid) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1145 | macro (function-like macro) | MCCS_ACTION |  | (pMCCS) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1146 | macro (function-like macro) | MCCS_ENTER |  | (pMCCS) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1147 | macro (function-like macro) | MCCS_LEAVE |  | (pMCCS) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1303 | macro (function-like macro) | mc_alloc_count |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1304 | macro (function-like macro) | mc_blocks_allocated |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1305 | macro (function-like macro) | mc_blocks_freed |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1306 | macro (function-like macro) | mc_breakpoint |  | (fi) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1307 | macro (function-like macro) | mc_bytes_allocated |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1308 | macro (function-like macro) | mc_bytes_freed |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1309 | macro (function-like macro) | mc_check |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1310 | macro (function-like macro) | mc_check_buffers |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1311 | macro (function-like macro) | mc_cur_file |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1312 | macro (function-like macro) | mc_cur_line |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1313 | macro (function-like macro) | mc_debug |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1314 | macro (function-like macro) | mc_debugf |  | (_args) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1315 | macro (function-like macro) | mc_debug_on |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1316 | macro (function-like macro) | mc_debug_off |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1317 | macro (function-like macro) | mc_endcheck |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1318 | macro (function-like macro) | mc_errno |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1319 | macro (function-like macro) | mc_error_flags |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1320 | macro (function-like macro) | mc_error_text |  | (e) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1321 | macro (function-like macro) | mc_except_text |  | (e) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1322 | macro (function-like macro) | mc_file |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1323 | macro (function-like macro) | mc_find_buffer |  | (p,mr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1324 | macro (function-like macro) | mc_func |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1325 | macro (function-like macro) | mc_func_name |  | (mcid) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1326 | macro (function-like macro) | mc_get_erf |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1327 | macro (function-like macro) | mc_get_mode |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1328 | macro (function-like macro) | mc_get_speed |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1329 | macro (function-like macro) | mc_in_source |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1330 | macro (function-like macro) | mc_is_active |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1331 | macro (function-like macro) | mc_line |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1332 | macro (function-like macro) | mc_load_debugger |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1333 | macro (function-like macro) | mc_location_text |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1334 | macro (function-like macro) | mc_memory_leaked |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1335 | macro (function-like macro) | mc_memrec |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1336 | macro (function-like macro) | mc_memrec_text |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1337 | macro (function-like macro) | mc_msg_continued |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1338 | macro (function-like macro) | mc_nullcheck |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1339 | macro (function-like macro) | mc_null_snapshot |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1340 | macro (function-like macro) | mc_register |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1341 | macro (function-like macro) | mc_report |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1342 | macro (function-like macro) | mc_set_alignsize |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1343 | macro (function-like macro) | mc_set_breakfile |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1344 | macro (function-like macro) | mc_set_checkbytes |  | (_cb) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1345 | macro (function-like macro) | mc_set_checkf |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1346 | macro (function-like macro) | mc_set_critf |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1347 | macro (function-like macro) | mc_set_endf |  | (erf) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1348 | macro (function-like macro) | mc_set_erf |  | (erf) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1349 | macro (function-like macro) | mc_set_globalf |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1350 | macro (function-like macro) | mc_set_globalexitf |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1351 | macro (function-like macro) | mc_set_speed |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1352 | macro (function-like macro) | mc_set_location |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1353 | macro (function-like macro) | mc_set_trackf |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1354 | macro (function-like macro) | mc_set_tracefile |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1355 | macro (function-like macro) | mc_set_tree_order |  | (_q) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1356 | macro (function-like macro) | mc_set_track_dir |  | (_dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1357 | macro (function-like macro) | mc_source_ptr |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1358 | macro (function-like macro) | mc_stack_trace |  | (_memo) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1359 | macro (function-like macro) | mc_startcheck |  | (_erf) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1360 | macro (function-like macro) | mc_unregister |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1361 | macro (function-like macro) | mc_set_exceptf |  | (f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1362 | macro (function-like macro) | mc_get_exceptf |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1365 | macro (function-like macro) | erf_default |  | (_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1366 | macro (function-like macro) | erf_standard |  | (_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1367 | macro (function-like macro) | erf_logfile |  | (_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1368 | macro (function-like macro) | erf_log_only |  | (_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1369 | macro (function-like macro) | erf_trace |  | (_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1372 | macro (function-like macro) | _direct_output |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1373 | macro (function-like macro) | _mcsl |  | (_f,_l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1374 | macro (function-like macro) | _mcsl_delete |  | (_f,_l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1375 | macro (function-like macro) | _mcsl_new |  | (_f,_l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1376 | macro (function-like macro) | _mcslx |  | (_f,_l,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1377 | macro (function-like macro) | _mc_set_delflag |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1378 | macro (function-like macro) | _mc_set_location |  | (_f,_l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1379 | macro (function-like macro) | _mc_set_newflag |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1382 | macro (function-like macro) | MC_SET_AUTOINIT |  | (_toggle) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1383 | macro (function-like macro) | MC_SET_CHECK_FREQ |  | (_freq) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1384 | macro (function-like macro) | MC_SET_CHECKF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1385 | macro (function-like macro) | MC_SET_CRITF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1386 | macro (function-like macro) | MC_SET_ENDF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1387 | macro (function-like macro) | MC_SET_ENV_VAR |  | (_envvarname) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1388 | macro (function-like macro) | MC_SET_ERF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1389 | macro (function-like macro) | MC_SET_GLOBALF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1390 | macro (function-like macro) | MC_SET_GLOBALEXITF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1391 | macro (function-like macro) | MC_SET_LOGFILE |  | (_logfilename) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1392 | macro (function-like macro) | MC_SET_PANIC_BUFFERS |  | (_q) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1393 | macro (function-like macro) | MC_SET_SSFH |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1394 | macro (function-like macro) | MC_SET_STARTF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1395 | macro (function-like macro) | MC_SET_TRACKF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1396 | macro (function-like macro) | MC_SET_VERIFYF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1399 | macro (function-like macro) | RTL |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1404 | macro (function-like macro) | NEW |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1405 | macro (function-like macro) | DELETE |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1406 | macro (function-like macro) | DELETE_ARR |  | (_arr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1408 | macro (function-like macro) | cpp_malloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1409 | macro (function-like macro) | cpp_calloc |  | (_n,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1410 | macro (function-like macro) | cpp_free |  | (_p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1413 | macro (function-like macro) | cpp_farmalloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1414 | macro (function-like macro) | cpp_farfree |  | (_fp) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1417 | macro (function-like macro) | cpp__fmalloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1418 | macro (function-like macro) | cpp__ffree |  | (_fp) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1564 | macro (function-like macro) | mc_load_debugger |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1583 | macro (function-like macro) | mc_set_location |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1613 | macro (function-like macro) | _direct_output |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1621 | macro (function-like macro) | mc_breakpoint |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1623 | macro (function-like macro) | mc_breakpoint_trace |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1836 | macro (function-like macro) | MC_SET_AUTOINIT |  | (_toggle) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1838 | macro (function-like macro) | MC_SET_CHECKF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1840 | macro (function-like macro) | MC_SET_CHECK_FREQ |  | (_freq) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1842 | macro (function-like macro) | MC_SET_CRITF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1844 | macro (function-like macro) | MC_SET_ENDF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1846 | macro (function-like macro) | MC_SET_ENV_VAR |  | (_envvarname) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1848 | macro (function-like macro) | MC_SET_ERF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1850 | macro (function-like macro) | MC_SET_EXCEPTF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1852 | macro (function-like macro) | MC_SET_GLOBALF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1854 | macro (function-like macro) | MC_SET_GLOBALEXITF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1856 | macro (function-like macro) | MC_SET_LOGFILE |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1858 | macro (function-like macro) | MC_SET_PANIC_BUFFERS |  | (_q) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1860 | macro (function-like macro) | MC_SET_SSFH |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1862 | macro (function-like macro) | MC_SET_STARTF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1864 | macro (function-like macro) | MC_SET_TRACKF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1866 | macro (function-like macro) | MC_SET_VERIFYF |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1890 | macro (function-like macro) | MC_HANDLE_EXCEPTION |  | (e) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1904 | macro (function-like macro) | _VA_DEF |  | (f,r,p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1922 | macro (function-like macro) | __paste |  | (x,y) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1923 | macro (function-like macro) | RTL |  | (func) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1928 | macro (function-like macro) | _RTLDECL |  | (f,rctype,params) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 1943 | macro (function-like macro) | C7ANSI |  | (func) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2012 | macro (function-like macro) | _INTERCEPT |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2013 | macro (function-like macro) | _VA_INTERCEPT |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2014 | macro (function-like macro) | _SETLOC |  | (_f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2040 | macro (function-like macro) | _INTERCEPTX |  | (_f,_d) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2042 | macro (function-like macro) | _INTERCEPTX |  | (_f,_d) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2140 | macro (function-like macro) | calloc |  | (n,_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2141 | macro (function-like macro) | malloc |  | (_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2142 | macro (function-like macro) | realloc |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2143 | macro (function-like macro) | free |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2145 | macro (function-like macro) | memcpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2146 | macro (function-like macro) | memmove |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2147 | macro (function-like macro) | memset |  | (p,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2148 | macro (function-like macro) | strcat |  | (s1,s2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2151 | macro (function-like macro) | strcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2154 | macro (function-like macro) | strcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2156 | macro (function-like macro) | strncat |  | (s1,s2,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2157 | macro (function-like macro) | strncpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2158 | macro (function-like macro) | vsprintf |  | (s,f,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2167 | macro (function-like macro) | _expand |  | (_p,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2168 | macro (function-like macro) | _fcalloc |  | (n,_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2169 | macro (function-like macro) | _fexpand |  | (_p,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2170 | macro (function-like macro) | _ffree |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2171 | macro (function-like macro) | _fmalloc |  | (_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2172 | macro (function-like macro) | _frealloc |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2173 | macro (function-like macro) | _fmsize |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2174 | macro (function-like macro) | _msize |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2175 | macro (function-like macro) | _nfree |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2176 | macro (function-like macro) | _nmalloc |  | (_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2177 | macro (function-like macro) | _nrealloc |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2178 | macro (function-like macro) | _ncalloc |  | (n,_sz) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2179 | macro (function-like macro) | _nexpand |  | (_p,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2180 | macro (function-like macro) | _nmsize |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2181 | macro (function-like macro) | _nstrdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2187 | macro (function-like macro) | cgets |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2188 | macro (function-like macro) | _cgets |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2189 | macro (function-like macro) | memccpy |  | (d,s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2190 | macro (function-like macro) | _memccpy |  | (d,s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2191 | macro (function-like macro) | strdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2192 | macro (function-like macro) | _strdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2193 | macro (function-like macro) | _strnset |  | (s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2194 | macro (function-like macro) | strnset |  | (s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2195 | macro (function-like macro) | strset |  | (s,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2196 | macro (function-like macro) | _strset |  | (s,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2197 | macro (function-like macro) | swab |  | (s,d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2198 | macro (function-like macro) | _swab |  | (s,d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2199 | macro (function-like macro) | tempnam |  | (d,pfx) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2200 | macro (function-like macro) | _tempnam |  | (d,pfx) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2202 | macro (function-like macro) | _fmemcpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2203 | macro (function-like macro) | _fmemmove |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2204 | macro (function-like macro) | _fmemset |  | (d,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2205 | macro (function-like macro) | _fmemccpy |  | (d,s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2206 | macro (function-like macro) | _fstrcat |  | (s1,s2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2207 | macro (function-like macro) | _fstrcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2208 | macro (function-like macro) | _fstrncat |  | (s1,s2,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2209 | macro (function-like macro) | _fstrncpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2210 | macro (function-like macro) | _fstrdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2211 | macro (function-like macro) | _fstrnset |  | (d,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2212 | macro (function-like macro) | _fstrset |  | (d,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2214 | macro (function-like macro) | getcwd |  | (d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2215 | macro (function-like macro) | _getcwd |  | (d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2216 | macro (function-like macro) | _getdcwd |  | (r,d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2217 | macro (function-like macro) | _dos_setvect |  | (_i,_h) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2218 | macro (function-like macro) | _fullpath |  | (b,p,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2225 | macro (function-like macro) | farfree |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2226 | macro (function-like macro) | farmalloc |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2227 | macro (function-like macro) | farcalloc |  | (n,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2228 | macro (function-like macro) | farrealloc |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2231 | macro (function-like macro) | cgets |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2232 | macro (function-like macro) | memccpy |  | (d,s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2234 | macro (function-like macro) | movmem |  | (s,d,l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2237 | macro (function-like macro) | setmem |  | (d,c,v) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2239 | macro (function-like macro) | setvect |  | (i,v) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2240 | macro (function-like macro) | stpcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2241 | macro (function-like macro) | _stpcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2242 | macro (function-like macro) | strdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2243 | macro (function-like macro) | strnset |  | (s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2244 | macro (function-like macro) | strset |  | (s,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2245 | macro (function-like macro) | swab |  | (s,d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2246 | macro (function-like macro) | tempnam |  | (d,pfx) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2248 | macro (function-like macro) | getcwd |  | (d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2249 | macro (function-like macro) | _getdcwd |  | (r,d,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2250 | macro (function-like macro) | _dos_setvect |  | (_i,_h) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2253 | macro (function-like macro) | _fmemcpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2254 | macro (function-like macro) | _fmemmove |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2255 | macro (function-like macro) | _fmemset |  | (d,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2256 | macro (function-like macro) | _fmemccpy |  | (d,s,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2257 | macro (function-like macro) | _fmovmem |  | (s,d,l) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2258 | macro (function-like macro) | _fsetmem |  | (d,c,v) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2259 | macro (function-like macro) | _fstrcat |  | (s1,s2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2260 | macro (function-like macro) | _fstrcpy |  | (d,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2261 | macro (function-like macro) | _fstrncat |  | (s1,s2,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2262 | macro (function-like macro) | _fstrncpy |  | (d,s,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2263 | macro (function-like macro) | _fstrdup |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2264 | macro (function-like macro) | _fstrnset |  | (d,c,n) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2265 | macro (function-like macro) | _fstrset |  | (d,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2354 | macro (function-like macro) | mem_get |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2355 | macro (function-like macro) | mem_free |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2380 | macro (function-like macro) | mc_startcheck |  | (erf) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2381 | macro (function-like macro) | mc_stack_trace |  | (_memo) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2382 | macro (function-like macro) | mc_debug |  | (s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2383 | macro (function-like macro) | mc_debugf |  | (arglist) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2385 | macro (function-like macro) | mc_endcheck |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2386 | macro (function-like macro) | mc_check_buffers |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2387 | macro (function-like macro) | mc_check |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2388 | macro (function-like macro) | mc_register |  | (p,s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2389 | macro (function-like macro) | mc_unregister |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2390 | macro (function-like macro) | mc_nullcheck |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2391 | macro (function-like macro) | mc_report |  | (f) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2431 | function (callable) | MemCheck | MemCheck | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2432 | function (callable) | ~MemCheck | MemCheck | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2455 | macro (function-like macro) | cpp_malloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2456 | macro (function-like macro) | cpp_calloc |  | (_n,_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2457 | macro (function-like macro) | cpp_free |  | (_p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2460 | macro (function-like macro) | cpp_farmalloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2461 | macro (function-like macro) | cpp_farfree |  | (_fp) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2464 | macro (function-like macro) | cpp__fmalloc |  | (_s) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2465 | macro (function-like macro) | cpp__ffree |  | (_fp) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2535 | macro (function-like macro) | NEW |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2538 | macro (function-like macro) | DELETE |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2540 | macro (function-like macro) | DELETE_ARR |  | (_arr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2560 | macro (function-like macro) | NEW |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2563 | macro (function-like macro) | DELETE |  | (_object) |  | SIGNATURE_CATALOGUED |
| REDALERT/MEMCHECK.H | 2565 | macro (function-like macro) | DELETE_ARR |  | (_arr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 71 | function (callable) | Select_To_Entry |  | (int select,unsigned long bitfield,int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 103 | function (callable) | Flash_Line |  | (char const * text,int xpix,int ypix,unsigned nfgc,unsigned hfgc,unsigned bgc) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 127 | function (callable) | Coordinates_In_Region |  | (int x,int y,int inx1,int iny1,int inx2,int iny2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 146 | function (callable) | Find_Menu_Items |  | (int maxitems,unsigned long field,char index) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 173 | function (callable) | Setup_Menu |  | (int menu,char const * text[],unsigned long field,int index,int skip) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 207 | function (callable) | Check_Menu |  | (int menu,char const * text[],char *,long field,int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 374 | function (callable) | Do_Menu |  | (char const ** strings,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/MENUS.CPP | 460 | function (callable) | Main_Menu |  | (unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 67 | function (callable) | MissionClass | MissionClass | (RTTIType rtti,int id) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 94 | function (callable) | Mission_Sleep | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 95 | function (callable) | Mission_Ambush | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 96 | function (callable) | Mission_Attack | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 97 | function (callable) | Mission_Capture | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 98 | function (callable) | Mission_Guard | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 99 | function (callable) | Mission_Guard_Area | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 100 | function (callable) | Mission_Harvest | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 101 | function (callable) | Mission_Hunt | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 102 | function (callable) | Mission_Move | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 103 | function (callable) | Mission_Retreat | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 104 | function (callable) | Mission_Return | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 105 | function (callable) | Mission_Stop | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 106 | function (callable) | Mission_Unload | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 107 | function (callable) | Mission_Enter | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 108 | function (callable) | Mission_Construction | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 109 | function (callable) | Mission_Deconstruction | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 110 | function (callable) | Mission_Repair | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 111 | function (callable) | Mission_Missile | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 130 | function (callable) | Set_Mission | MissionClass | (MissionType mission) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 155 | function (callable) | Get_Mission | MissionClass | (void) const | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 179 | function (callable) | Debug_Dump | MissionClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 210 | function (callable) | AI | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 340 | function (callable) | Commence | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 376 | function (callable) | Assign_Mission | MissionClass | (MissionType order) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 409 | function (callable) | Mission_From_Name | MissionClass | (char const * name) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 439 | function (callable) | Mission_Name | MissionClass | (MissionType mission) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 465 | function (callable) | Override_Mission | MissionClass | (MissionType mission,TARGET,TARGET) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 491 | function (callable) | Restore_Mission | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 519 | function (callable) | Is_Recruitable_Mission | MissionClass | (MissionType mission) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 529 | function (callable) | MissionControlClass | MissionControlClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 543 | function (callable) | Name | MissionControlClass | (void) const | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.CPP | 553 | function (callable) | Read_INI | MissionControlClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.H | 69 | function (callable) | MissionClass | MissionClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.H | 70 | function (callable) | ~MissionClass | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.H | 79 | function (callable) | Shorten_Mission_Timer | MissionClass | (void) | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.H | 138 | function (callable) | Normal_Delay | MissionControlClass | (void) const | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MISSION.H | 139 | function (callable) | AA_Delay | MissionControlClass | (void) const | RUST/original-source-reference/REDALERT/MISSION.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 91 | function (callable) | Free | MixFileClass | (char const * filename) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 121 | function (callable) | ~MixFileClass | MixFileClass | (void) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 165 | function (callable) | MixFileClass | MixFileClass | (char const * filename,PKey const * key) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 279 | function (callable) | Retrieve | MixFileClass | (char const * filename) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 305 | function (callable) | Finder | MixFileClass | (char const * filename) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 347 | function (callable) | Cache | MixFileClass | (char const * filename,Buffer const * buffer) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 376 | function (callable) | Cache | MixFileClass | (Buffer const * buffer) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 478 | function (callable) | Free | MixFileClass | (void) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 488 | function (callable) | compfunc |  | (void const * ptr1,void const * ptr2) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 527 | function (callable) | Offset | MixFileClass | (char const * filename,void ** realptr,MixFileClass ** mixfile,long * offset,long * size) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.CPP | 591 | function (callable) | Free_All | MixFileClass | (void) | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.H | 63 | function (callable) | operator < | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.H | 64 | function (callable) | operator > | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MIXFILE.H | 65 | function (callable) | operator == | MixFileClass::SubBlock | (SubBlock & two) const | RUST/original-source-reference/REDALERT/MIXFILE.md | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 57 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 58 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 78 | label (assembly label candidate) | Detect_MMX_Availability |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 201 | label (assembly label candidate) | Init_MMX |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 241 | label (assembly label candidate) | MMX_Done |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MMX.ASM | 276 | label (assembly label candidate) | MMX_Single_Line_Trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 99 | function (callable) | min |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 105 | function (callable) | max |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 129 | function (callable) | MonoClass | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 169 | function (callable) | ~MonoClass | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 191 | function (callable) | Pan | MonoClass | (int cols) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 241 | function (callable) | Sub_Window | MonoClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 303 | function (callable) | Draw_Box | MonoClass | (int x,int y,int w,int h,MonoAttribute attrib,BoxStyleType thick) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 379 | function (callable) | Set_Cursor | MonoClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 420 | function (callable) | Clear | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 458 | function (callable) | Fill_Attrib | MonoClass | (int x,int y,int w,int h,MonoAttribute attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 487 | function (callable) | Scroll | MonoClass | (int lines) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 541 | function (callable) | Printf | MonoClass | (char const * text,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 580 | function (callable) | Printf | MonoClass | (int text,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 618 | function (callable) | Print | MonoClass | (char const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 710 | function (callable) | Text_Print | MonoClass | (char const * text,int x,int y,MonoAttribute attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 744 | function (callable) | Text_Print | MonoClass | (int text,int x,int y,MonoAttribute attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 775 | function (callable) | Print | MonoClass | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 796 | function (callable) | operator = | MonoClass | (MonoClass const & src) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 823 | function (callable) | View | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 867 | function (callable) | Mono_Set_Cursor |  | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 897 | function (callable) | Mono_Printf |  | (char const * string,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 935 | function (callable) | Mono_Clear_Screen |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 966 | function (callable) | Mono_Text_Print |  | (void const * text,int x,int y,int attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1000 | function (callable) | Mono_Draw_Rect |  | (int x,int y,int w,int h,int attrib,int thick) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1027 | function (callable) | Mono_Print |  | (void const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1055 | function (callable) | Mono_X |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1083 | function (callable) | Mono_Y |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1097 | function (callable) | Mono_Put_Char |  | (char,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1101 | function (callable) | Mono_Scroll |  | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1105 | function (callable) | Mono_View_Page |  | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.CPP | 1109 | function (callable) | Mono_Printf |  | (int string,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 84 | function (callable) | Enable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 85 | function (callable) | Disable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 86 | function (callable) | Is_Enabled | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 87 | function (callable) | Get_Current | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 92 | function (callable) | Set_Default_Attribute | MonoClass | (MonoAttribute attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 104 | function (callable) | Get_X | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 105 | function (callable) | Get_Y | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 106 | function (callable) | Get_Width | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 107 | function (callable) | Get_Height | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 208 | function (callable) | Raw_Ptr | MonoClass | (int page) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MONOC.H | 211 | function (callable) | Page_Ptr | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 73 | function (callable) | MouseClass | MouseClass | (void) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 96 | function (callable) | Set_Default_Mouse | MouseClass | (MouseType mouse,bool size) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 120 | function (callable) | Revert_Mouse_Shape | MouseClass | (void) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 142 | function (callable) | Mouse_Small | MouseClass | (bool wsmall) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 189 | function (callable) | Override_Mouse_Shape | MouseClass | (MouseType mouse,bool wsmall) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 250 | function (callable) | AI | MouseClass | (KeyNumType & input,int x,int y) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 286 | function (callable) | One_Time | MouseClass | (void) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.CPP | 322 | function (callable) | Init_Clear | MouseClass | (void) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.H | 45 | function (callable) | MouseClass | MouseClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MOUSE.H | 56 | function (callable) | Get_Mouse_Shape | MouseClass | (void) const | RUST/original-source-reference/REDALERT/MOUSE.md | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 93 | macro (function-like macro) | min |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 117 | function (callable) | _Byte_Precision |  | (unsigned long value) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 147 | function (callable) | XMP_DER_Length_Encode |  | (unsigned long length,unsigned char * output) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 186 | function (callable) | XMP_DER_Encode |  | (digit const * from,unsigned char * output,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 225 | function (callable) | XMP_DER_Decode |  | (digit * result,unsigned char const * input,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 270 | function (callable) | XMP_Encode |  | (unsigned char * to,unsigned tobytes,digit const * from,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 316 | function (callable) | XMP_Encode |  | (unsigned char * to,digit const * from,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 367 | function (callable) | XMP_Signed_Decode |  | (digit * result,const unsigned char * from,int frombytes,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 417 | function (callable) | XMP_Unsigned_Decode |  | (digit * result,const unsigned char * from,int frombytes,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 462 | function (callable) | XMP_Significance |  | (const digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 491 | function (callable) | XMP_Inc |  | (digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 519 | function (callable) | XMP_Dec |  | (digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 548 | function (callable) | XMP_Neg |  | (digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 576 | function (callable) | XMP_Abs |  | (digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 606 | function (callable) | XMP_Shift_Right_Bits |  | (digit * number,int bits,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 692 | function (callable) | XMP_Shift_Left_Bits |  | (digit * number,int bits,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 779 | function (callable) | XMP_Rotate_Left |  | (digit * number,bool carry,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 811 | function (callable) | XMP_Not |  | (digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 842 | function (callable) | XMP_Init |  | (digit * number,digit value,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 869 | function (callable) | XMP_Count_Bits |  | (const digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 906 | function (callable) | XMP_Count_Bytes |  | (const digit * number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 937 | function (callable) | XMP_Move |  | (digit * dest,digit const * source,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 964 | function (callable) | XMP_Compare |  | (const digit * left_number,const digit * right_number,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1003 | function (callable) | XMP_Add |  | (digit * result,const digit * left_number,const digit * right_number,bool carry,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1045 | function (callable) | XMP_Add_Int |  | (digit * result,const digit * left_number,digit right_number,bool carry,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1086 | function (callable) | XMP_Sub |  | (digit * result,const digit * left_number,const digit * right_number,bool borrow,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1129 | function (callable) | XMP_Sub_Int |  | (digit * result,const digit * left_number,unsigned short right_number,bool borrow,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1168 | function (callable) | XMP_Unsigned_Mult |  | (digit * prod,const digit * multiplicand,const digit * multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1223 | function (callable) | XMP_Unsigned_Mult_Int |  | (digit * prod,const digit * multiplicand,short multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1263 | function (callable) | XMP_Signed_Mult_Int |  | (digit * prod,const digit * multiplicand,signed short multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1312 | function (callable) | XMP_Signed_Mult |  | (digit * prod,const digit * multiplicand,const digit * multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1366 | function (callable) | XMP_Unsigned_Div_Int |  | (digit * quotient,digit const * dividend,unsigned short divisor,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1429 | function (callable) | XMP_Unsigned_Div |  | (digit * remainder,digit * quotient,digit const * dividend,digit const * divisor,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1493 | function (callable) | XMP_Signed_Div |  | (digit * remainder,digit * quotient,digit const * dividend,digit const * divisor,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1548 | function (callable) | XMP_Inverse_A_Mod_B |  | (digit * result,digit const * number,digit const * modulus,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1595 | function (callable) | XMP_Reciprocal |  | (digit * quotient,const digit * divisor,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1651 | function (callable) | XMP_Decode_ASCII |  | (char const * str,digit * mpn,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1767 | function (callable) | XMP_Hybrid_Mul |  | (unsigned short * prod,unsigned short * multiplicand,unsigned short multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1804 | function (callable) | XMP_Double_Mul |  | (digit * prod,const digit * multiplicand,const digit * multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1861 | function (callable) | XMP_Prepare_Modulus |  | (const digit * n_modulus,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 1919 | function (callable) | XMP_Mod_Mult |  | (digit * prod,const digit * multiplicand,const digit * multiplier,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2010 | function (callable) | XMP_Mod_Mult_Clear |  | (int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2048 | function (callable) | mp_quo_digit |  | (unsigned short * dividend) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2081 | function (callable) | xmp_exponent_mod |  | (digit * expout,const digit * expin,const digit * exponent_ptr,const digit * modulus,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2180 | function (callable) | memrev |  | (char * buffer,size_t length) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2191 | function (callable) | pfunc |  | (const void * pkey,const void * base) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2216 | function (callable) | XMP_Is_Small_Prime |  | (const digit * candidate,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2249 | function (callable) | XMP_Small_Divisors_Test |  | (const digit * candidate,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2282 | function (callable) | XMP_Fermat_Test |  | (const digit * candidate_prime,unsigned rounds,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2327 | function (callable) | XMP_Rabin_Miller_Test |  | (Straw & rng,digit const * w,int rounds,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2399 | function (callable) | XMP_Randomize |  | (digit * result,Straw & rng,int total_bits,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2437 | function (callable) | XMP_Randomize |  | (digit * result,Straw & rng,digit const * minval,digit const * maxval,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.CPP | 2470 | function (callable) | XMP_Is_Prime |  | (digit const * prime,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 71 | macro (function-like macro) | ARRAY_SIZE |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 127 | function (callable) | XMP_Digits_To_Bits |  | (int digits) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 133 | function (callable) | XMP_Bits_To_Digits |  | (int bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 139 | function (callable) | XMP_Bits_To_Mask |  | (int bits) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 146 | function (callable) | XMP_Is_Negative |  | (const digit * r,int precision) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 152 | function (callable) | XMP_Test_Eq_Int |  | (digit const * r,int i,int p) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 158 | function (callable) | XMP_Set_Bit |  | (digit * r,unsigned bit) |  | SIGNATURE_CATALOGUED |
| REDALERT/MP.H | 163 | function (callable) | XMP_Test_Bit |  | (const digit * r,unsigned bit) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 57 | function (callable) | MPGSettings | MPGSettings | (const char * deviceName) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 81 | function (callable) | MPGSettings | MPGSettings | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 151 | function (callable) | ~MPGSettings | MPGSettings | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 179 | function (callable) | SetDeviceName | MPGSettings | (const char * deviceName) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 191 | function (callable) | Save | MPGSettings | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 232 | function (callable) | Dialog | MPGSettings | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.CPP | 566 | function (callable) | EnumMCI |  | (MCIDevice * desc,void * context) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPGSET.H | 33 | function (callable) | GetDeviceName | MPGSettings | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 65 | function (callable) | Select_MPlayer_Game |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 460 | function (callable) | Clear_Listbox |  | (ListClass * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 492 | function (callable) | Clear_Vector |  | (DynamicVectorClass<NodeNameType * > * vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 522 | function (callable) | Computer_Message |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 594 | function (callable) | Garble_Message |  | (char * buf) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 683 | function (callable) | Surrender_Dialog |  | (int text) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 690 | function (callable) | Surrender_Dialog |  | (const char * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 891 | function (callable) | Abort_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 1086 | function (callable) | Read_TEN_Game_Options |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLAYER.CPP | 1179 | function (callable) | Read_MPATH_Game_Options |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 30 | function (callable) | Yield |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 39 | function (callable) | PostWindowsMessage |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 49 | function (callable) | MGenGetQueueCtr |  | (int qNo) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 60 | function (callable) | MGenMoveTo |  | (int qFrom,int qTo) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 72 | function (callable) | MGenGetNode |  | (int q) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 83 | function (callable) | MGenGetMasterNode |  | (unsigned * size) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 94 | function (callable) | MGenFlushNodes |  | (int qFrom,int qTo) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 106 | function (callable) | MGenMCount |  | (unsigned lowerOrderBits,unsigned upperOrderBits) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPLIB.CPP | 118 | function (callable) | MGenSanityCheck |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 41 | macro (function-like macro) | min |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 58 | function (callable) | MPlayerManClass | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 79 | function (callable) | Send_Private_Message | MPlayerManClass | (void * buf,int buflen,int,int conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 112 | function (callable) | Get_Private_Message | MPlayerManClass | (void * buf,int * buflen,int * conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 145 | function (callable) | Send_Global_Message | MPlayerManClass | (void * buf,int buflen,int,int address) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 171 | function (callable) | Get_Global_Message | MPlayerManClass | (void * buf,int * buflen,int * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 202 | function (callable) | Service | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 208 | function (callable) | Create_Connection | MPlayerManClass | (int id,char * name,int address) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 218 | function (callable) | Delete_Connection | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 235 | function (callable) | Connection_Name | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 246 | function (callable) | Connection_Address | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 257 | function (callable) | Num_Connections | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 263 | function (callable) | Connection_ID | MPlayerManClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 269 | function (callable) | Connection_Index | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 283 | function (callable) | Global_Num_Send | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 289 | function (callable) | Global_Num_Receive | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 295 | function (callable) | Private_Num_Send | MPlayerManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 301 | function (callable) | Private_Num_Receive | MPlayerManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 307 | function (callable) | Reset_Response_Time | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 313 | function (callable) | Response_Time | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 319 | function (callable) | Set_Timing | MPlayerManClass | (unsigned long,unsigned long,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 327 | function (callable) | Configure_Debug | MPlayerManClass | (int,int,int,char **,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 335 | function (callable) | Mono_Debug_Print | MPlayerManClass | (int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 341 | function (callable) | Init | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 346 | function (callable) | Find_Num_Connections | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRD.CPP | 358 | function (callable) | Flush_All | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 40 | macro (function-like macro) | min |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 80 | function (callable) | MPGetProcAddress |  | (HMODULE hModule,LPCSTR lpProcName) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 91 | function (callable) | MPlayerManClass | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 137 | function (callable) | ~MPlayerManClass | MPlayerManClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 146 | function (callable) | Send_Private_Message | MPlayerManClass | (void * buf,int buflen,int ack_req,int conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 155 | function (callable) | Get_Private_Message | MPlayerManClass | (void * buf,int * buflen,int * conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 162 | function (callable) | Send_Global_Message | MPlayerManClass | (void * buf,int buflen,int ack_req,int addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 169 | function (callable) | Get_Global_Message | MPlayerManClass | (void * buf,int * buflen,int * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 175 | function (callable) | Service | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 181 | function (callable) | Create_Connection | MPlayerManClass | (int id,char * name,int address) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 187 | function (callable) | Delete_Connection | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 193 | function (callable) | Connection_Name | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 199 | function (callable) | Connection_Address | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 205 | function (callable) | Num_Connections | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 211 | function (callable) | Connection_ID | MPlayerManClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 217 | function (callable) | Connection_Index | MPlayerManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 223 | function (callable) | Global_Num_Send | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 229 | function (callable) | Global_Num_Receive | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 236 | function (callable) | Private_Num_Send | MPlayerManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 242 | function (callable) | Private_Num_Receive | MPlayerManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 248 | function (callable) | Reset_Response_Time | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 254 | function (callable) | Response_Time | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 260 | function (callable) | Set_Timing | MPlayerManClass | (unsigned long,unsigned long,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 268 | function (callable) | Configure_Debug | MPlayerManClass | (int,int,int,char **,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 276 | function (callable) | Mono_Debug_Print | MPlayerManClass | (int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 282 | function (callable) | Init | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 287 | function (callable) | Find_Num_Connections | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPMGRW.CPP | 293 | function (callable) | Flush_All | MPlayerManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MPU.CPP | 39 | function (callable) | Get_CPU_Clock |  | (unsigned long & high) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGBOX.CPP | 74 | function (callable) | Process | WWMessageBox | (const char * msg,const char * b1txt,const char * b2txt,const char * b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGBOX.CPP | 488 | function (callable) | Process | WWMessageBox | (int msg,int b1txt,int b2txt,int b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGBOX.CPP | 514 | function (callable) | Process | WWMessageBox | (char const * msg,int b1txt,int b2txt,int b3txt,bool preserve) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGBOX.H | 45 | function (callable) | WWMessageBox | WWMessageBox | (int caption=TXT_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 75 | function (callable) | MessageListClass | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 126 | function (callable) | ~MessageListClass | MessageListClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 159 | function (callable) | Init | MessageListClass | (int x,int y,int max_msg,int maxchars,int height,int edit_x,int edit_y,int overflow_on,int over_start,int over_end,int width) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 264 | function (callable) | Reset | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 322 | function (callable) | Add_Message | MessageListClass | (char const * name,int id,char const * txt,PlayerColorType color,TextPrintType style,int timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 492 | function (callable) | Get_Message | MessageListClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 529 | function (callable) | Get_Label | MessageListClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 569 | function (callable) | Concat_Message | MessageListClass | (char const * name,int id,char const * txt,int timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 697 | function (callable) | Set_Edit_Focus | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 717 | function (callable) | Has_Edit_Focus | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 746 | function (callable) | Add_Edit | MessageListClass | (PlayerColorType color,TextPrintType style,char * to,char cursor,int width) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 835 | function (callable) | Remove_Edit | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 873 | function (callable) | Get_Edit_Buf | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 895 | function (callable) | Set_Edit_Color | MessageListClass | (PlayerColorType color) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 921 | function (callable) | Manage | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 990 | function (callable) | Input | MessageListClass | (KeyNumType & input) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 1164 | function (callable) | Draw | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 1216 | function (callable) | Num_Messages | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 1255 | function (callable) | Set_Width | MessageListClass | (int width) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 1293 | function (callable) | Trim_Message | MessageListClass | (char * dest,char * src,int min_chars,int max_chars,int scandir) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.CPP | 1380 | function (callable) | Compute_Y | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.H | 141 | function (callable) | Get_Overflow_Buf | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.H | 142 | function (callable) | Clear_Overflow_Buf | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MSGLIST.H | 143 | function (callable) | Is_Edit | MessageListClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 31 | function (callable) | Mem_Copy |  | (void const * source,void * dest,unsigned long bytes_to_copy) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 54 | function (callable) | Distance_Coord |  | (COORDINATE coord1,COORDINATE coord2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 104 | function (callable) | Desired_Facing16 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 204 | function (callable) | Desired_Facing256 |  | (LONG srcx,LONG srcy,LONG dstx,LONG dsty) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 371 | function (callable) | Desired_Facing8 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 496 | function (callable) | Desired_Facing16 |  | (long x1,long y1,long x2,long y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 778 | function (callable) | Set_Bit |  | (void * array,int bit,int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 796 | function (callable) | Get_Bit |  | (void const * array,int bit) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 809 | function (callable) | First_True_Bit |  | (void const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 825 | function (callable) | First_False_Bit |  | (void const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 842 | function (callable) | Bound |  | (int original,int min,int max) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 902 | function (callable) | Shake_Screen |  | (int shakes) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1072 | function (callable) | Conquer_Build_Fading_Table |  | (void const * palette,void * dest,int color,int frac) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1262 | function (callable) | Reverse_Long |  | (long number) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1273 | function (callable) | Reverse_Short |  | (short number) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1283 | function (callable) | Swap_Long |  | (long number) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1325 | function (callable) | strtrim |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1404 | function (callable) | Fat_Put_Pixel |  | (int x,int y,int color,int siz,GraphicViewPortClass & gpage) |  | SIGNATURE_CATALOGUED |
| REDALERT/MiscAsm.cpp | 1564 | function (callable) | Set_Palette_Range |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 109 | function (callable) | Destroy_Connection |  | (int,int) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 110 | function (callable) | Process_Global_Packet |  | (GlobalPacketType *,IPXAddressClass *) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 111 | function (callable) | Client_Remote_Connect |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 112 | function (callable) | Server_Remote_Connect |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 113 | function (callable) | Init_Network |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 114 | function (callable) | Remote_Connect |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 963 | function (callable) | Init_Network |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 1009 | function (callable) | Shutdown_Network |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 1090 | function (callable) | Process_Global_Packet |  | (GlobalPacketType * packet,IPXAddressClass * address) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 1171 | function (callable) | Destroy_Connection |  | (int id,int error) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 1257 | function (callable) | Remote_Connect |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NETDLG.CPP | 1394 | function (callable) | Net_Join_Dialog |  | (void) | RUST/original-source-reference/REDALERT/NETDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 70 | function (callable) | NonSequencedConnClass | NonSequencedConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 99 | function (callable) | ~NonSequencedConnClass | NonSequencedConnClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 120 | function (callable) | Init | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 156 | function (callable) | Send_Packet | NonSequencedConnClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 215 | function (callable) | Receive_Packet | NonSequencedConnClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 432 | function (callable) | Get_Packet | NonSequencedConnClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 507 | function (callable) | Service_Send_Queue | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NOSEQCON.CPP | 655 | function (callable) | Service_Receive_Queue | NonSequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 68 | function (callable) | NullModemConnClass | NullModemConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 106 | function (callable) | ~NullModemConnClass | NullModemConnClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 129 | function (callable) | Init | NullModemConnClass | (HANDLE port_handle) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 135 | function (callable) | Init | NullModemConnClass | (PORT * port) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 160 | function (callable) | Send | NullModemConnClass | (char * buf,int buflen,void *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 235 | function (callable) | Compute_CRC | NullModemConnClass | (char * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.CPP | 272 | function (callable) | Packet_Overhead_Size | NullModemConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLCONN.H | 98 | function (callable) | Actual_Max_Packet | NullModemConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 52 | function (callable) | Select_Serial_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 145 | function (callable) | Init_Null_Modem |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 188 | function (callable) | Shutdown_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 220 | function (callable) | Modem_Signoff |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 258 | function (callable) | Test_Null_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 543 | function (callable) | Reconnect_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 593 | function (callable) | Reconnect_Null_Modem |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 790 | function (callable) | Destroy_Null_Connection |  | (int id,int error) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 885 | function (callable) | Select_Serial_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 1339 | function (callable) | Advanced_Modem_Settings |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 1626 | function (callable) | Com_Settings_Dialog |  | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 2818 | function (callable) | Build_Init_String_Listbox |  | (ListClass * list,EditClass * edit,char * buf,int * index) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 2887 | function (callable) | Init_String_Compare |  | (const void * p1,const void * p2) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 2910 | function (callable) | Com_Scenario_Dialog |  | (bool skirmish) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 4667 | function (callable) | Find_Local_Scenario |  | (char * description,char * filename,unsigned int length,char * digest,bool official) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 4790 | function (callable) | Com_Show_Scenario_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 6331 | function (callable) | Phone_Dialog |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 6774 | function (callable) | Build_Phone_Listbox |  | (ListClass * list,EditClass * edit,char * buf) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 6867 | function (callable) | Phone_Compare |  | (const void * p1,const void * p2) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 6902 | function (callable) | Edit_Phone_Dialog |  | (PhoneEntryClass * phone) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7180 | function (callable) | Dial_Modem |  | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7369 | function (callable) | Answer_Modem |  | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7538 | function (callable) | Modem_Echo |  | (char c) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7549 | function (callable) | Smart_Printf |  | (char * format,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7576 | function (callable) | Hex_Dump_Data |  | (char * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7660 | function (callable) | itoh |  | (int i,char * s) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7686 | function (callable) | Log_Start_Time |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7697 | function (callable) | Log_End_Time |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7728 | function (callable) | Log_Time |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7759 | function (callable) | Log_Start_Nest_Time |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLDLG.CPP | 7796 | function (callable) | Log_End_Nest_Time |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 114 | function (callable) | NullModemClass | NullModemClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 181 | function (callable) | ~NullModemClass | NullModemClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 211 | function (callable) | Init | NullModemClass | (int port,int irq,char * dev_name,int baud,char parity,int wordlen,int stopbits,int flowcontrol) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 465 | function (callable) | Num_Connections | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 485 | function (callable) | Delete_Connection | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 528 | function (callable) | Init_Send_Queue | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 558 | function (callable) | Detect_Port | NullModemClass | (SerialSettingsType * settings) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 785 | function (callable) | Shutdown | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 830 | function (callable) | Set_Timing | NullModemClass | (unsigned long retrydelta,unsigned long maxretries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 876 | function (callable) | Send_Message | NullModemClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 909 | function (callable) | Get_Message | NullModemClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 934 | function (callable) | Service | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1182 | function (callable) | Num_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1204 | function (callable) | Num_Receive | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1226 | function (callable) | Response_Time | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1248 | function (callable) | Reset_Response_Time | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1268 | function (callable) | Oldest_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1317 | function (callable) | Configure_Debug | NullModemClass | (int,int type_offset,int type_size,char ** names,int namestart,int namecount) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1342 | function (callable) | Mono_Debug_Print | NullModemClass | (int,int refresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1406 | function (callable) | Detect_Modem | NullModemClass | (SerialSettingsType * settings,bool reconnect) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1671 | function (callable) | Dial_Modem | NullModemClass | (char * string,DialMethodType method,bool reconnect) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 1863 | function (callable) | Answer_Modem | NullModemClass | (bool reconnect) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2118 | function (callable) | Hangup_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2236 | function (callable) | Setup_Modem_Echo | NullModemClass | (void (* func)(char c)) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2261 | function (callable) | Remove_Modem_Echo | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2284 | function (callable) | Print_EchoBuf | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2314 | function (callable) | Reset_EchoBuf | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2335 | function (callable) | Abort_Modem | NullModemClass | (PORT *) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2379 | function (callable) | Setup_Abort_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2412 | function (callable) | Remove_Abort_Modem | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2447 | function (callable) | Change_IRQ_Priority | NullModemClass | (int irq) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2483 | function (callable) | Get_Modem_Status | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2525 | function (callable) | Send_Modem_Command | NullModemClass | (char * command,char terminator,char * buffer,int buflen,int delay,int retries) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.CPP | 2595 | function (callable) | Verify_And_Convert_To_Int | NullModemClass | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 98 | function (callable) | Connection_ID | NullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 99 | function (callable) | Connection_Index | NullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 115 | function (callable) | Send_Private_Message | NullModemClass | (void * buf,int buflen,int ack_req=1,int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 118 | function (callable) | Get_Private_Message | NullModemClass | (void * buf,int * buflen,int *) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 145 | function (callable) | Global_Num_Send | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 146 | function (callable) | Global_Num_Receive | NullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 147 | function (callable) | Private_Num_Send | NullModemClass | (int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/NULLMGR.H | 149 | function (callable) | Private_Num_Receive | NullModemClass | (int=CONNECTION_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 129 | function (callable) | ObjectClass | ObjectClass | (RTTIType rtti,int id) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 162 | function (callable) | Get_Image_Data | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 183 | function (callable) | Name | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 206 | function (callable) | Exit_Coord | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 228 | function (callable) | AI | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 290 | function (callable) | What_Action | ObjectClass | (ObjectClass const *) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 318 | function (callable) | What_Action | ObjectClass | (CELL) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 342 | function (callable) | In_Which_Layer | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 369 | function (callable) | Get_Ownable | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 393 | function (callable) | Can_Repair | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 417 | function (callable) | Can_Demolish | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 426 | function (callable) | Can_Demolish_Unit | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 435 | function (callable) | Can_Capture | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 460 | function (callable) | Can_Player_Fire | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 485 | function (callable) | Can_Player_Move | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 510 | function (callable) | Target_Coord | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 536 | function (callable) | Center_Coord | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 561 | function (callable) | Render_Coord | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 586 | function (callable) | Docking_Coord | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 612 | function (callable) | Sort_Y | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 639 | function (callable) | Fire_Data | ObjectClass | (int which) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 647 | function (callable) | Fire_Coord | ObjectClass | (int) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 671 | function (callable) | Record_The_Kill | ObjectClass | (TechnoClass *) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 695 | function (callable) | Do_Shimmer | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 721 | function (callable) | Exit_Object | ObjectClass | (TechnoClass *) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 744 | function (callable) | Hidden | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 769 | function (callable) | Look | ObjectClass | (bool) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 795 | function (callable) | Active_Click_With | ObjectClass | (ActionType,ObjectClass *) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 819 | function (callable) | Active_Click_With | ObjectClass | (ActionType,CELL) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 842 | function (callable) | Clicked_As_Target | ObjectClass | (HousesType house,int) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 868 | function (callable) | In_Range | ObjectClass | (COORDINATE,int) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 893 | function (callable) | Weapon_Range | ObjectClass | (int) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 921 | function (callable) | Scatter | ObjectClass | (COORDINATE,bool,bool) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 946 | function (callable) | Catch_Fire | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 970 | function (callable) | Fire_Out | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 996 | function (callable) | Value | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1021 | function (callable) | Get_Mission | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1049 | function (callable) | Repair | ObjectClass | (int) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1075 | function (callable) | Sell_Back | ObjectClass | (int) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1098 | function (callable) | Move | ObjectClass | (FacingType facing) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1132 | function (callable) | Unselect | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1169 | function (callable) | Unselect_All_Players | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1201 | function (callable) | Unselect_All_Players_Except_Owner | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1244 | function (callable) | Select | ObjectClass | (bool allow_mixed) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1320 | function (callable) | Render | ObjectClass | (bool forced) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1375 | function (callable) | Debug_Dump | ObjectClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1422 | function (callable) | Mark_For_Redraw | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1456 | function (callable) | Limbo | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1510 | function (callable) | Unlimbo | ObjectClass | (COORDINATE coord,DirType) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1559 | function (callable) | Detach | ObjectClass | (TARGET target,bool) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1585 | function (callable) | Detach_All | ObjectClass | (bool all) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1635 | function (callable) | Receive_Message | ObjectClass | (RadioClass *,RadioMessageType message,long &) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1690 | function (callable) | Take_Damage | ObjectClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1836 | function (callable) | Mark | ObjectClass | (MarkType mark) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1943 | function (callable) | Init | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1964 | function (callable) | Revealed | ObjectClass | (HouseClass * house) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 1988 | function (callable) | Set_Selected_By_Player | ObjectClass | (HouseClass * player) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2023 | function (callable) | Set_Unselected_By_Player | ObjectClass | (HouseClass * player) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2052 | function (callable) | Is_Selected_By_Player | ObjectClass | (HouseClass * player) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2084 | function (callable) | Paradrop | ObjectClass | (COORDINATE coord) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2127 | function (callable) | Attach_Trigger | ObjectClass | (TriggerClass * trigger) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2145 | function (callable) | Occupy_List | ObjectClass | (bool placement) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2146 | function (callable) | Overlap_List | ObjectClass | (bool) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2147 | function (callable) | Who_Can_Build_Me | ObjectClass | (bool intheory,bool legal) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2148 | function (callable) | Health_Ratio | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2149 | function (callable) | Full_Name | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2173 | function (callable) | ObjectTypeClass | ObjectTypeClass | (RTTIType rtti,int id,bool is_sentient,bool is_stealthy,bool is_selectable,bool is_legal_target,bool is_insignificant,bool is_immune,bool is_footprint,int name,char const * ini) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2222 | function (callable) | Max_Pips | ObjectTypeClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2246 | function (callable) | Dimensions | ObjectTypeClass | (int & width,int & height) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2269 | function (callable) | Cost_Of | ObjectTypeClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2290 | function (callable) | Time_To_Build | ObjectTypeClass | (HousesType) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2312 | function (callable) | Get_Cameo_Data | ObjectTypeClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2335 | function (callable) | Occupy_List | ObjectTypeClass | (bool) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2359 | function (callable) | Overlap_List | ObjectTypeClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2381 | function (callable) | One_Time | ObjectTypeClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.CPP | 2423 | function (callable) | Who_Can_Build_Me | ObjectTypeClass | (bool intheory,bool legal,HousesType house) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 140 | function (callable) | ObjectClass | ObjectClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 141 | function (callable) | ~ObjectClass | ObjectClass | (void) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 142 | function (callable) | operator < | ObjectClass | (ObjectClass const & object) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 143 | function (callable) | operator > | ObjectClass | (ObjectClass const & object) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 153 | function (callable) | Is_Players_Army | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 158 | function (callable) | Is_Infantry | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 159 | function (callable) | Is_Foot | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 160 | function (callable) | Is_Techno | ObjectClass | (void) const | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OBJECT.H | 248 | function (callable) | Per_Cell_Process | ObjectClass | (PCPType) | RUST/original-source-reference/REDALERT/OBJECT.md | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 509 | macro (function-like macro) | IEnumConnections_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 512 | macro (function-like macro) | IEnumConnections_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 515 | macro (function-like macro) | IEnumConnections_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 519 | macro (function-like macro) | IEnumConnections_Next |  | (This,cConnections,rgcd,pcFetched) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 522 | macro (function-like macro) | IEnumConnections_Skip |  | (This,cConnections) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 525 | macro (function-like macro) | IEnumConnections_Reset |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 528 | macro (function-like macro) | IEnumConnections_Clone |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 683 | macro (function-like macro) | IConnectionPoint_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 686 | macro (function-like macro) | IConnectionPoint_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 689 | macro (function-like macro) | IConnectionPoint_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 693 | macro (function-like macro) | IConnectionPoint_GetConnectionInterface |  | (This,pIID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 696 | macro (function-like macro) | IConnectionPoint_GetConnectionPointContainer |  | (This,ppCPC) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 699 | macro (function-like macro) | IConnectionPoint_Advise |  | (This,pUnkSink,pdwCookie) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 702 | macro (function-like macro) | IConnectionPoint_Unadvise |  | (This,dwCookie) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 705 | macro (function-like macro) | IConnectionPoint_EnumConnections |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 865 | macro (function-like macro) | IEnumConnectionPoints_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 868 | macro (function-like macro) | IEnumConnectionPoints_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 871 | macro (function-like macro) | IEnumConnectionPoints_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 875 | macro (function-like macro) | IEnumConnectionPoints_Next |  | (This,cConnections,ppCP,pcFetched) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 878 | macro (function-like macro) | IEnumConnectionPoints_Skip |  | (This,cConnections) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 881 | macro (function-like macro) | IEnumConnectionPoints_Reset |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 884 | macro (function-like macro) | IEnumConnectionPoints_Clone |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1018 | macro (function-like macro) | IConnectionPointContainer_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1021 | macro (function-like macro) | IConnectionPointContainer_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1024 | macro (function-like macro) | IConnectionPointContainer_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1028 | macro (function-like macro) | IConnectionPointContainer_EnumConnectionPoints |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1031 | macro (function-like macro) | IConnectionPointContainer_FindConnectionPoint |  | (This,riid,ppCP) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1173 | macro (function-like macro) | IClassFactory2_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1176 | macro (function-like macro) | IClassFactory2_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1179 | macro (function-like macro) | IClassFactory2_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1183 | macro (function-like macro) | IClassFactory2_CreateInstance |  | (This,pUnkOuter,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1186 | macro (function-like macro) | IClassFactory2_LockServer |  | (This,fLock) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1190 | macro (function-like macro) | IClassFactory2_GetLicInfo |  | (This,pLicInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1193 | macro (function-like macro) | IClassFactory2_RequestLicKey |  | (This,dwReserved,pBstrKey) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1196 | macro (function-like macro) | IClassFactory2_CreateInstanceLic |  | (This,pUnkOuter,pUnkReserved,riid,bstrKey,ppvObj) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1309 | macro (function-like macro) | IProvideClassInfo_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1312 | macro (function-like macro) | IProvideClassInfo_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1315 | macro (function-like macro) | IProvideClassInfo_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1319 | macro (function-like macro) | IProvideClassInfo_GetClassInfo |  | (This,ppTI) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1416 | macro (function-like macro) | IProvideClassInfo2_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1419 | macro (function-like macro) | IProvideClassInfo2_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1422 | macro (function-like macro) | IProvideClassInfo2_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1426 | macro (function-like macro) | IProvideClassInfo2_GetClassInfo |  | (This,ppTI) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1430 | macro (function-like macro) | IProvideClassInfo2_GetGUID |  | (This,dwGuidKind,pGUID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1554 | macro (function-like macro) | IOleControl_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1557 | macro (function-like macro) | IOleControl_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1560 | macro (function-like macro) | IOleControl_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1564 | macro (function-like macro) | IOleControl_GetControlInfo |  | (This,pCI) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1567 | macro (function-like macro) | IOleControl_OnMnemonic |  | (This,pMsg) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1570 | macro (function-like macro) | IOleControl_OnAmbientPropertyChange |  | (This,dispID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1573 | macro (function-like macro) | IOleControl_FreezeEvents |  | (This,bFreeze) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1755 | macro (function-like macro) | IOleControlSite_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1758 | macro (function-like macro) | IOleControlSite_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1761 | macro (function-like macro) | IOleControlSite_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1765 | macro (function-like macro) | IOleControlSite_OnControlInfoChanged |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1768 | macro (function-like macro) | IOleControlSite_LockInPlaceActive |  | (This,fLock) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1771 | macro (function-like macro) | IOleControlSite_GetExtendedControl |  | (This,ppDisp) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1774 | macro (function-like macro) | IOleControlSite_TransformCoords |  | (This,pPtlHimetric,pPtfContainer,dwFlags) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1777 | macro (function-like macro) | IOleControlSite_TranslateAccelerator |  | (This,pMsg,grfModifiers) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1780 | macro (function-like macro) | IOleControlSite_OnFocus |  | (This,fGotFocus) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 1783 | macro (function-like macro) | IOleControlSite_ShowPropertyFrame |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2024 | macro (function-like macro) | IPropertyPage_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2027 | macro (function-like macro) | IPropertyPage_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2030 | macro (function-like macro) | IPropertyPage_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2034 | macro (function-like macro) | IPropertyPage_SetPageSite |  | (This,pPageSite) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2037 | macro (function-like macro) | IPropertyPage_Activate |  | (This,hWndParent,pRect,bModal) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2040 | macro (function-like macro) | IPropertyPage_Deactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2043 | macro (function-like macro) | IPropertyPage_GetPageInfo |  | (This,pPageInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2046 | macro (function-like macro) | IPropertyPage_SetObjects |  | (This,cObjects,ppUnk) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2049 | macro (function-like macro) | IPropertyPage_Show |  | (This,nCmdShow) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2052 | macro (function-like macro) | IPropertyPage_Move |  | (This,pRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2055 | macro (function-like macro) | IPropertyPage_IsPageDirty |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2058 | macro (function-like macro) | IPropertyPage_Apply |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2061 | macro (function-like macro) | IPropertyPage_Help |  | (This,pszHelpDir) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2064 | macro (function-like macro) | IPropertyPage_TranslateAccelerator |  | (This,pMsg) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2314 | macro (function-like macro) | IPropertyPage2_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2317 | macro (function-like macro) | IPropertyPage2_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2320 | macro (function-like macro) | IPropertyPage2_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2324 | macro (function-like macro) | IPropertyPage2_SetPageSite |  | (This,pPageSite) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2327 | macro (function-like macro) | IPropertyPage2_Activate |  | (This,hWndParent,pRect,bModal) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2330 | macro (function-like macro) | IPropertyPage2_Deactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2333 | macro (function-like macro) | IPropertyPage2_GetPageInfo |  | (This,pPageInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2336 | macro (function-like macro) | IPropertyPage2_SetObjects |  | (This,cObjects,ppUnk) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2339 | macro (function-like macro) | IPropertyPage2_Show |  | (This,nCmdShow) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2342 | macro (function-like macro) | IPropertyPage2_Move |  | (This,pRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2345 | macro (function-like macro) | IPropertyPage2_IsPageDirty |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2348 | macro (function-like macro) | IPropertyPage2_Apply |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2351 | macro (function-like macro) | IPropertyPage2_Help |  | (This,pszHelpDir) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2354 | macro (function-like macro) | IPropertyPage2_TranslateAccelerator |  | (This,pMsg) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2358 | macro (function-like macro) | IPropertyPage2_EditProperty |  | (This,dispID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2472 | macro (function-like macro) | IPropertyPageSite_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2475 | macro (function-like macro) | IPropertyPageSite_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2478 | macro (function-like macro) | IPropertyPageSite_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2482 | macro (function-like macro) | IPropertyPageSite_OnStatusChange |  | (This,dwFlags) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2485 | macro (function-like macro) | IPropertyPageSite_GetLocaleID |  | (This,pLocaleID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2488 | macro (function-like macro) | IPropertyPageSite_GetPageContainer |  | (This,ppUnk) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2491 | macro (function-like macro) | IPropertyPageSite_TranslateAccelerator |  | (This,pMsg) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2620 | macro (function-like macro) | IPropertyNotifySink_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2623 | macro (function-like macro) | IPropertyNotifySink_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2626 | macro (function-like macro) | IPropertyNotifySink_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2630 | macro (function-like macro) | IPropertyNotifySink_OnChanged |  | (This,dispID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2633 | macro (function-like macro) | IPropertyNotifySink_OnRequestEdit |  | (This,dispID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2739 | macro (function-like macro) | ISpecifyPropertyPages_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2742 | macro (function-like macro) | ISpecifyPropertyPages_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2745 | macro (function-like macro) | ISpecifyPropertyPages_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2749 | macro (function-like macro) | ISpecifyPropertyPages_GetPages |  | (This,pPages) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2869 | macro (function-like macro) | IPersistMemory_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2872 | macro (function-like macro) | IPersistMemory_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2875 | macro (function-like macro) | IPersistMemory_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2879 | macro (function-like macro) | IPersistMemory_GetClassID |  | (This,pClassID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2883 | macro (function-like macro) | IPersistMemory_IsDirty |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2886 | macro (function-like macro) | IPersistMemory_Load |  | (This,pMem,cbSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2889 | macro (function-like macro) | IPersistMemory_Save |  | (This,pMem,fClearDirty,cbSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2892 | macro (function-like macro) | IPersistMemory_GetSizeMax |  | (This,pCbSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 2895 | macro (function-like macro) | IPersistMemory_InitNew |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3060 | macro (function-like macro) | IPersistStreamInit_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3063 | macro (function-like macro) | IPersistStreamInit_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3066 | macro (function-like macro) | IPersistStreamInit_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3070 | macro (function-like macro) | IPersistStreamInit_GetClassID |  | (This,pClassID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3074 | macro (function-like macro) | IPersistStreamInit_IsDirty |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3077 | macro (function-like macro) | IPersistStreamInit_Load |  | (This,pStm) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3080 | macro (function-like macro) | IPersistStreamInit_Save |  | (This,pStm,fClearDirty) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3083 | macro (function-like macro) | IPersistStreamInit_GetSizeMax |  | (This,pCbSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3086 | macro (function-like macro) | IPersistStreamInit_InitNew |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3241 | macro (function-like macro) | IPersistPropertyBag_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3244 | macro (function-like macro) | IPersistPropertyBag_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3247 | macro (function-like macro) | IPersistPropertyBag_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3251 | macro (function-like macro) | IPersistPropertyBag_GetClassID |  | (This,pClassID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3255 | macro (function-like macro) | IPersistPropertyBag_InitNew |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3258 | macro (function-like macro) | IPersistPropertyBag_Load |  | (This,pPropBag,pErrorLog) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3261 | macro (function-like macro) | IPersistPropertyBag_Save |  | (This,pPropBag,fClearDirty,fSaveAllProperties) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3400 | macro (function-like macro) | ISimpleFrameSite_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3403 | macro (function-like macro) | ISimpleFrameSite_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3406 | macro (function-like macro) | ISimpleFrameSite_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3410 | macro (function-like macro) | ISimpleFrameSite_PreMessageFilter |  | (This,hWnd,msg,wp,lp,plResult,pdwCookie) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3413 | macro (function-like macro) | ISimpleFrameSite_PostMessageFilter |  | (This,hWnd,msg,wp,lp,plResult,dwCookie) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3692 | macro (function-like macro) | IFont_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3695 | macro (function-like macro) | IFont_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3698 | macro (function-like macro) | IFont_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3702 | macro (function-like macro) | IFont_get_Name |  | (This,pName) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3705 | macro (function-like macro) | IFont_put_Name |  | (This,name) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3708 | macro (function-like macro) | IFont_get_Size |  | (This,pSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3711 | macro (function-like macro) | IFont_put_Size |  | (This,size) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3714 | macro (function-like macro) | IFont_get_Bold |  | (This,pBold) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3717 | macro (function-like macro) | IFont_put_Bold |  | (This,bold) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3720 | macro (function-like macro) | IFont_get_Italic |  | (This,pItalic) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3723 | macro (function-like macro) | IFont_put_Italic |  | (This,italic) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3726 | macro (function-like macro) | IFont_get_Underline |  | (This,pUnderline) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3729 | macro (function-like macro) | IFont_put_Underline |  | (This,underline) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3732 | macro (function-like macro) | IFont_get_Strikethrough |  | (This,pStrikethrough) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3735 | macro (function-like macro) | IFont_put_Strikethrough |  | (This,strikethrough) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3738 | macro (function-like macro) | IFont_get_Weight |  | (This,pWeight) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3741 | macro (function-like macro) | IFont_put_Weight |  | (This,weight) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3744 | macro (function-like macro) | IFont_get_Charset |  | (This,pCharset) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3747 | macro (function-like macro) | IFont_put_Charset |  | (This,charset) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3750 | macro (function-like macro) | IFont_get_hFont |  | (This,phFont) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3753 | macro (function-like macro) | IFont_Clone |  | (This,ppFont) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3756 | macro (function-like macro) | IFont_IsEqual |  | (This,pFontOther) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3759 | macro (function-like macro) | IFont_SetRatio |  | (This,cyLogical,cyHimetric) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3762 | macro (function-like macro) | IFont_QueryTextMetrics |  | (This,pTM) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3765 | macro (function-like macro) | IFont_AddRefHfont |  | (This,hFont) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3768 | macro (function-like macro) | IFont_ReleaseHfont |  | (This,hFont) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 3771 | macro (function-like macro) | IFont_SetHdc |  | (This,hDC) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4265 | macro (function-like macro) | IPicture_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4268 | macro (function-like macro) | IPicture_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4271 | macro (function-like macro) | IPicture_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4275 | macro (function-like macro) | IPicture_get_Handle |  | (This,pHandle) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4278 | macro (function-like macro) | IPicture_get_hPal |  | (This,phPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4281 | macro (function-like macro) | IPicture_get_Type |  | (This,pType) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4284 | macro (function-like macro) | IPicture_get_Width |  | (This,pWidth) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4287 | macro (function-like macro) | IPicture_get_Height |  | (This,pHeight) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4290 | macro (function-like macro) | IPicture_Render |  | (This,hDC,x,y,cx,cy,xSrc,ySrc,cxSrc,cySrc,pRcWBounds) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4293 | macro (function-like macro) | IPicture_set_hPal |  | (This,hPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4296 | macro (function-like macro) | IPicture_get_CurDC |  | (This,phDC) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4299 | macro (function-like macro) | IPicture_SelectPicture |  | (This,hDCIn,phDCOut,phBmpOut) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4302 | macro (function-like macro) | IPicture_get_KeepOriginalFormat |  | (This,pKeep) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4305 | macro (function-like macro) | IPicture_put_KeepOriginalFormat |  | (This,keep) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4308 | macro (function-like macro) | IPicture_PictureChanged |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4311 | macro (function-like macro) | IPicture_SaveAsFile |  | (This,pStream,fSaveMemCopy,pCbSize) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4314 | macro (function-like macro) | IPicture_get_Attributes |  | (This,pDwAttr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4590 | macro (function-like macro) | IFontDisp_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4593 | macro (function-like macro) | IFontDisp_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4596 | macro (function-like macro) | IFontDisp_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4600 | macro (function-like macro) | IFontDisp_GetTypeInfoCount |  | (This,pctinfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4603 | macro (function-like macro) | IFontDisp_GetTypeInfo |  | (This,iTInfo,lcid,ppTInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4606 | macro (function-like macro) | IFontDisp_GetIDsOfNames |  | (This,riid,rgszNames,cNames,lcid,rgDispId) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4609 | macro (function-like macro) | IFontDisp_Invoke |  | (This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4706 | macro (function-like macro) | IPictureDisp_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4709 | macro (function-like macro) | IPictureDisp_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4712 | macro (function-like macro) | IPictureDisp_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4716 | macro (function-like macro) | IPictureDisp_GetTypeInfoCount |  | (This,pctinfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4719 | macro (function-like macro) | IPictureDisp_GetTypeInfo |  | (This,iTInfo,lcid,ppTInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4722 | macro (function-like macro) | IPictureDisp_GetIDsOfNames |  | (This,riid,rgszNames,cNames,lcid,rgDispId) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4725 | macro (function-like macro) | IPictureDisp_Invoke |  | (This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4820 | macro (function-like macro) | IAdviseSinkEx_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4823 | macro (function-like macro) | IAdviseSinkEx_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4826 | macro (function-like macro) | IAdviseSinkEx_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4830 | macro (function-like macro) | IAdviseSinkEx_OnDataChange |  | (This,pFormatetc,pStgmed) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4833 | macro (function-like macro) | IAdviseSinkEx_OnViewChange |  | (This,dwAspect,lindex) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4836 | macro (function-like macro) | IAdviseSinkEx_OnRename |  | (This,pmk) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4839 | macro (function-like macro) | IAdviseSinkEx_OnSave |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4842 | macro (function-like macro) | IAdviseSinkEx_OnClose |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4846 | macro (function-like macro) | IAdviseSinkEx_OnViewStatusChange |  | (This,dwViewStatus) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4967 | macro (function-like macro) | IOleInPlaceObjectWindowless_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4970 | macro (function-like macro) | IOleInPlaceObjectWindowless_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4973 | macro (function-like macro) | IOleInPlaceObjectWindowless_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4977 | macro (function-like macro) | IOleInPlaceObjectWindowless_GetWindow |  | (This,phwnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4980 | macro (function-like macro) | IOleInPlaceObjectWindowless_ContextSensitiveHelp |  | (This,fEnterMode) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4984 | macro (function-like macro) | IOleInPlaceObjectWindowless_InPlaceDeactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4987 | macro (function-like macro) | IOleInPlaceObjectWindowless_UIDeactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4990 | macro (function-like macro) | IOleInPlaceObjectWindowless_SetObjectRects |  | (This,lprcPosRect,lprcClipRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4993 | macro (function-like macro) | IOleInPlaceObjectWindowless_ReactivateAndUndo |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 4997 | macro (function-like macro) | IOleInPlaceObjectWindowless_OnWindowMessage |  | (This,msg,wParam,lParam,plResult) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5000 | macro (function-like macro) | IOleInPlaceObjectWindowless_GetDropTarget |  | (This,ppDropTarget) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5166 | macro (function-like macro) | IOleInPlaceSiteEx_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5169 | macro (function-like macro) | IOleInPlaceSiteEx_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5172 | macro (function-like macro) | IOleInPlaceSiteEx_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5176 | macro (function-like macro) | IOleInPlaceSiteEx_GetWindow |  | (This,phwnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5179 | macro (function-like macro) | IOleInPlaceSiteEx_ContextSensitiveHelp |  | (This,fEnterMode) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5183 | macro (function-like macro) | IOleInPlaceSiteEx_CanInPlaceActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5186 | macro (function-like macro) | IOleInPlaceSiteEx_OnInPlaceActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5189 | macro (function-like macro) | IOleInPlaceSiteEx_OnUIActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5192 | macro (function-like macro) | IOleInPlaceSiteEx_GetWindowContext |  | (This,ppFrame,ppDoc,lprcPosRect,lprcClipRect,lpFrameInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5195 | macro (function-like macro) | IOleInPlaceSiteEx_Scroll |  | (This,scrollExtant) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5198 | macro (function-like macro) | IOleInPlaceSiteEx_OnUIDeactivate |  | (This,fUndoable) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5201 | macro (function-like macro) | IOleInPlaceSiteEx_OnInPlaceDeactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5204 | macro (function-like macro) | IOleInPlaceSiteEx_DiscardUndoState |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5207 | macro (function-like macro) | IOleInPlaceSiteEx_DeactivateAndUndo |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5210 | macro (function-like macro) | IOleInPlaceSiteEx_OnPosRectChange |  | (This,lprcPosRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5214 | macro (function-like macro) | IOleInPlaceSiteEx_OnInPlaceActivateEx |  | (This,pfNoRedraw,dwFlags) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5217 | macro (function-like macro) | IOleInPlaceSiteEx_OnInPlaceDeactivateEx |  | (This,fNoRedraw) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5220 | macro (function-like macro) | IOleInPlaceSiteEx_RequestUIActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5486 | macro (function-like macro) | IOleInPlaceSiteWindowless_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5489 | macro (function-like macro) | IOleInPlaceSiteWindowless_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5492 | macro (function-like macro) | IOleInPlaceSiteWindowless_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5496 | macro (function-like macro) | IOleInPlaceSiteWindowless_GetWindow |  | (This,phwnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5499 | macro (function-like macro) | IOleInPlaceSiteWindowless_ContextSensitiveHelp |  | (This,fEnterMode) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5503 | macro (function-like macro) | IOleInPlaceSiteWindowless_CanInPlaceActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5506 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnInPlaceActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5509 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnUIActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5512 | macro (function-like macro) | IOleInPlaceSiteWindowless_GetWindowContext |  | (This,ppFrame,ppDoc,lprcPosRect,lprcClipRect,lpFrameInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5515 | macro (function-like macro) | IOleInPlaceSiteWindowless_Scroll |  | (This,scrollExtant) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5518 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnUIDeactivate |  | (This,fUndoable) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5521 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnInPlaceDeactivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5524 | macro (function-like macro) | IOleInPlaceSiteWindowless_DiscardUndoState |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5527 | macro (function-like macro) | IOleInPlaceSiteWindowless_DeactivateAndUndo |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5530 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnPosRectChange |  | (This,lprcPosRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5534 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnInPlaceActivateEx |  | (This,pfNoRedraw,dwFlags) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5537 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnInPlaceDeactivateEx |  | (This,fNoRedraw) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5540 | macro (function-like macro) | IOleInPlaceSiteWindowless_RequestUIActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5544 | macro (function-like macro) | IOleInPlaceSiteWindowless_CanWindowlessActivate |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5547 | macro (function-like macro) | IOleInPlaceSiteWindowless_GetCapture |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5550 | macro (function-like macro) | IOleInPlaceSiteWindowless_SetCapture |  | (This,fCapture) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5553 | macro (function-like macro) | IOleInPlaceSiteWindowless_GetFocus |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5556 | macro (function-like macro) | IOleInPlaceSiteWindowless_SetFocus |  | (This,fFocus) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5559 | macro (function-like macro) | IOleInPlaceSiteWindowless_GetDC |  | (This,pRect,grfFlags,phDC) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5562 | macro (function-like macro) | IOleInPlaceSiteWindowless_ReleaseDC |  | (This,hDC) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5565 | macro (function-like macro) | IOleInPlaceSiteWindowless_InvalidateRect |  | (This,pRect,fErase) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5568 | macro (function-like macro) | IOleInPlaceSiteWindowless_InvalidateRgn |  | (This,hRGN,fErase) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5571 | macro (function-like macro) | IOleInPlaceSiteWindowless_ScrollRect |  | (This,dx,dy,pRectScroll,pRectClip) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5574 | macro (function-like macro) | IOleInPlaceSiteWindowless_AdjustRect |  | (This,prc) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5577 | macro (function-like macro) | IOleInPlaceSiteWindowless_OnDefWindowMessage |  | (This,msg,wParam,lParam,plResult) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5957 | macro (function-like macro) | IViewObjectEx_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5960 | macro (function-like macro) | IViewObjectEx_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5963 | macro (function-like macro) | IViewObjectEx_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5967 | macro (function-like macro) | IViewObjectEx_Draw |  | (This,dwDrawAspect,lindex,pvAspect,ptd,hdcTargetDev,hdcDraw,lprcBounds,lprcWBounds,pfnContinue,dwContinue) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5970 | macro (function-like macro) | IViewObjectEx_GetColorSet |  | (This,dwDrawAspect,lindex,pvAspect,ptd,hicTargetDev,ppColorSet) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5973 | macro (function-like macro) | IViewObjectEx_Freeze |  | (This,dwDrawAspect,lindex,pvAspect,pdwFreeze) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5976 | macro (function-like macro) | IViewObjectEx_Unfreeze |  | (This,dwFreeze) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5979 | macro (function-like macro) | IViewObjectEx_SetAdvise |  | (This,aspects,advf,pAdvSink) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5982 | macro (function-like macro) | IViewObjectEx_GetAdvise |  | (This,pAspects,pAdvf,ppAdvSink) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5986 | macro (function-like macro) | IViewObjectEx_GetExtent |  | (This,dwDrawAspect,lindex,ptd,lpsizel) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5990 | macro (function-like macro) | IViewObjectEx_GetRect |  | (This,dwAspect,pRect) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5993 | macro (function-like macro) | IViewObjectEx_GetViewStatus |  | (This,pdwStatus) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5996 | macro (function-like macro) | IViewObjectEx_QueryHitPoint |  | (This,dwAspect,pRectBounds,ptlLoc,lCloseHint,pHitResult) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 5999 | macro (function-like macro) | IViewObjectEx_QueryHitRect |  | (This,dwAspect,pRectBounds,pRectLoc,lCloseHint,pHitResult) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6002 | macro (function-like macro) | IViewObjectEx_GetNaturalExtent |  | (This,dwAspect,lindex,ptd,hicTargetDev,pExtentInfo,pSizel) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6171 | macro (function-like macro) | IOleUndoUnit_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6174 | macro (function-like macro) | IOleUndoUnit_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6177 | macro (function-like macro) | IOleUndoUnit_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6181 | macro (function-like macro) | IOleUndoUnit_Do |  | (This,pUndoManager) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6184 | macro (function-like macro) | IOleUndoUnit_GetDescription |  | (This,pBstr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6187 | macro (function-like macro) | IOleUndoUnit_GetUnitType |  | (This,pClsid,plID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6190 | macro (function-like macro) | IOleUndoUnit_OnNextAdd |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6358 | macro (function-like macro) | IOleParentUndoUnit_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6361 | macro (function-like macro) | IOleParentUndoUnit_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6364 | macro (function-like macro) | IOleParentUndoUnit_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6368 | macro (function-like macro) | IOleParentUndoUnit_Do |  | (This,pUndoManager) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6371 | macro (function-like macro) | IOleParentUndoUnit_GetDescription |  | (This,pBstr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6374 | macro (function-like macro) | IOleParentUndoUnit_GetUnitType |  | (This,pClsid,plID) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6377 | macro (function-like macro) | IOleParentUndoUnit_OnNextAdd |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6381 | macro (function-like macro) | IOleParentUndoUnit_Open |  | (This,pPUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6384 | macro (function-like macro) | IOleParentUndoUnit_Close |  | (This,pPUU,fCommit) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6387 | macro (function-like macro) | IOleParentUndoUnit_Add |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6390 | macro (function-like macro) | IOleParentUndoUnit_FindUnit |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6393 | macro (function-like macro) | IOleParentUndoUnit_GetParentState |  | (This,pdwState) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6551 | macro (function-like macro) | IEnumOleUndoUnits_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6554 | macro (function-like macro) | IEnumOleUndoUnits_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6557 | macro (function-like macro) | IEnumOleUndoUnits_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6561 | macro (function-like macro) | IEnumOleUndoUnits_Next |  | (This,cElt,rgElt,pcEltFetched) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6564 | macro (function-like macro) | IEnumOleUndoUnits_Skip |  | (This,cElt) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6567 | macro (function-like macro) | IEnumOleUndoUnits_Reset |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6570 | macro (function-like macro) | IEnumOleUndoUnits_Clone |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6773 | macro (function-like macro) | IOleUndoManager_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6776 | macro (function-like macro) | IOleUndoManager_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6779 | macro (function-like macro) | IOleUndoManager_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6783 | macro (function-like macro) | IOleUndoManager_Open |  | (This,pPUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6786 | macro (function-like macro) | IOleUndoManager_Close |  | (This,pPUU,fCommit) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6789 | macro (function-like macro) | IOleUndoManager_Add |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6792 | macro (function-like macro) | IOleUndoManager_GetOpenParentState |  | (This,pdwState) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6795 | macro (function-like macro) | IOleUndoManager_DiscardFrom |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6798 | macro (function-like macro) | IOleUndoManager_UndoTo |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6801 | macro (function-like macro) | IOleUndoManager_RedoTo |  | (This,pUU) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6804 | macro (function-like macro) | IOleUndoManager_EnumUndoable |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6807 | macro (function-like macro) | IOleUndoManager_EnumRedoable |  | (This,ppEnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6810 | macro (function-like macro) | IOleUndoManager_GetLastUndoDescription |  | (This,pBstr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6813 | macro (function-like macro) | IOleUndoManager_GetLastRedoDescription |  | (This,pBstr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 6816 | macro (function-like macro) | IOleUndoManager_Enable |  | (This,fEnable) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7093 | macro (function-like macro) | IQuickActivate_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7096 | macro (function-like macro) | IQuickActivate_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7099 | macro (function-like macro) | IQuickActivate_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7103 | macro (function-like macro) | IQuickActivate_QuickActivate |  | (This,pQaContainer,pQaControl) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7106 | macro (function-like macro) | IQuickActivate_SetContentExtent |  | (This,pSizel) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7109 | macro (function-like macro) | IQuickActivate_GetContentExtent |  | (This,pSizel) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7255 | macro (function-like macro) | IPointerInactive_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7258 | macro (function-like macro) | IPointerInactive_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7261 | macro (function-like macro) | IPointerInactive_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7265 | macro (function-like macro) | IPointerInactive_GetActivationPolicy |  | (This,pdwPolicy) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7268 | macro (function-like macro) | IPointerInactive_OnInactiveMouseMove |  | (This,pRectBounds,x,y,grfKeyState) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7271 | macro (function-like macro) | IPointerInactive_OnInactiveSetCursor |  | (This,pRectBounds,x,y,dwMouseMsg,fSetAlways) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7397 | macro (function-like macro) | IObjectWithSite_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7400 | macro (function-like macro) | IObjectWithSite_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7403 | macro (function-like macro) | IObjectWithSite_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7407 | macro (function-like macro) | IObjectWithSite_SetSite |  | (This,pUnkSite) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7410 | macro (function-like macro) | IObjectWithSite_GetSite |  | (This,riid,ppvSite) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7511 | macro (function-like macro) | IErrorLog_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7514 | macro (function-like macro) | IErrorLog_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7517 | macro (function-like macro) | IErrorLog_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7521 | macro (function-like macro) | IErrorLog_AddError |  | (This,pszPropName,pExcepInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7621 | macro (function-like macro) | IPropertyBag_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7624 | macro (function-like macro) | IPropertyBag_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7627 | macro (function-like macro) | IPropertyBag_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7631 | macro (function-like macro) | IPropertyBag_Read |  | (This,pszPropName,pVar,pErrorLog) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7634 | macro (function-like macro) | IPropertyBag_Write |  | (This,pszPropName,pVar) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7786 | macro (function-like macro) | IPerPropertyBrowsing_QueryInterface |  | (This,riid,ppvObject) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7789 | macro (function-like macro) | IPerPropertyBrowsing_AddRef |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7792 | macro (function-like macro) | IPerPropertyBrowsing_Release |  | (This) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7796 | macro (function-like macro) | IPerPropertyBrowsing_GetDisplayString |  | (This,dispID,pBstr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7799 | macro (function-like macro) | IPerPropertyBrowsing_MapPropertyToPage |  | (This,dispID,pClsid) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7802 | macro (function-like macro) | IPerPropertyBrowsing_GetPredefinedStrings |  | (This,dispID,pCaStringsOut,pCaCookiesOut) |  | SIGNATURE_CATALOGUED |
| REDALERT/OCIDL.H | 7805 | macro (function-like macro) | IPerPropertyBrowsing_GetPredefinedValue |  | (This,dispID,dwCookie,pVarOut) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 498 | function (callable) | OverlayTypeClass | OverlayTypeClass | (OverlayType iconset,char const * ininame,int fullname,LandType ground,int damagelevels,int damagepoints,bool isradarvisible,bool iswooden,bool istarget,bool iscrushable,bool istiberium,bool high,bool theater,bool walltype,bool iscrate) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 557 | function (callable) | operator new | OverlayTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 578 | function (callable) | operator delete | OverlayTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 599 | function (callable) | Init_Heap | OverlayTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 649 | function (callable) | One_Time | OverlayTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 671 | function (callable) | From_Name | OverlayTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 702 | function (callable) | Occupy_List | OverlayTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 723 | function (callable) | Radar_Icon | OverlayTypeClass | (int data) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 749 | function (callable) | Display | OverlayTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 785 | function (callable) | Prep_For_Add | OverlayTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 815 | function (callable) | Create_And_Place | OverlayTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 840 | function (callable) | Create_One_Of | OverlayTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 864 | function (callable) | Draw_It | OverlayTypeClass | (int x,int y,int data) const |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 887 | function (callable) | Init | OverlayTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 926 | function (callable) | As_Reference | OverlayTypeClass | (OverlayType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/ODATA.CPP | 948 | function (callable) | Coord_Fixup | OverlayTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 87 | function (callable) | OptionsClass | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 178 | function (callable) | One_Time | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 199 | function (callable) | Process | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 219 | function (callable) | Set_Shuffle | OptionsClass | (int on) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 240 | function (callable) | Set_Repeat | OptionsClass | (int on) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 263 | function (callable) | Set_Score_Volume | OptionsClass | (fixed volume,bool feedback) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 291 | function (callable) | Set_Sound_Volume | OptionsClass | (fixed volume,bool feedback) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 315 | function (callable) | Set_Brightness | OptionsClass | (fixed brightness) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 338 | function (callable) | Get_Brightness | OptionsClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 359 | function (callable) | Set_Saturation | OptionsClass | (fixed color) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 383 | function (callable) | Get_Saturation | OptionsClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 404 | function (callable) | Set_Contrast | OptionsClass | (fixed contrast) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 427 | function (callable) | Get_Contrast | OptionsClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 447 | function (callable) | Set_Tint | OptionsClass | (fixed tint) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 470 | function (callable) | Get_Tint | OptionsClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 502 | function (callable) | Adjust_Palette | OptionsClass | (PaletteClass const & oldpal,PaletteClass & newpal,fixed brightness,fixed color,fixed tint,fixed contrast) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 564 | function (callable) | Load_Settings | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 716 | function (callable) | Save_Settings | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 827 | function (callable) | Set | OptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 861 | function (callable) | Normalize_Delay | OptionsClass | (int delay) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 895 | function (callable) | Fixup_Palette | OptionsClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OPTIONS.CPP | 917 | function (callable) | Normalize_Volume | OptionsClass | (int volume) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 64 | function (callable) | Init | OverlayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 85 | function (callable) | operator new | OverlayClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 110 | function (callable) | operator delete | OverlayClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 135 | function (callable) | OverlayClass | OverlayClass | (OverlayType type,CELL pos,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 164 | function (callable) | Mark | OverlayClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 263 | function (callable) | Read_INI | OverlayClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.CPP | 350 | function (callable) | Write_INI | OverlayClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 58 | function (callable) | operator new | OverlayClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 61 | function (callable) | OverlayClass | OverlayClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 62 | function (callable) | ~OverlayClass | OverlayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 63 | function (callable) | operator OverlayType | OverlayClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 72 | function (callable) | INI_Name | OverlayClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 80 | function (callable) | Class_Of | OverlayClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/OVERLAY.H | 81 | function (callable) | Draw_It | OverlayClass | (int,int,WindowNumberType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 57 | function (callable) | ~PacketClass | PacketClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 81 | function (callable) | Add_Field | PacketClass | (FieldClass * field) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 99 | function (callable) | PacketClass | PacketClass | (char * curbuf) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 175 | function (callable) | Create_Comms_Packet | PacketClass | (int & size) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 262 | function (callable) | Find_Field | PacketClass | (char * id) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 286 | function (callable) | Get_Field | PacketClass | (char * id,char & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 310 | function (callable) | Get_Field | PacketClass | (char * id,unsigned char & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 334 | function (callable) | Get_Field | PacketClass | (char * id,short & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 358 | function (callable) | Get_Field | PacketClass | (char * id,unsigned short & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 382 | function (callable) | Get_Field | PacketClass | (char * id,long & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 406 | function (callable) | Get_Field | PacketClass | (char * id,char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 429 | function (callable) | Get_Field | PacketClass | (char * id,unsigned long & data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.CPP | 454 | function (callable) | Get_Field | PacketClass | (char * id,void * data,int & length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 46 | function (callable) | PacketClass | PacketClass | (short id=0) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 65 | function (callable) | Add_Field | PacketClass | (char * field,char data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 66 | function (callable) | Add_Field | PacketClass | (char * field,unsigned char data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 67 | function (callable) | Add_Field | PacketClass | (char * field,short data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 68 | function (callable) | Add_Field | PacketClass | (char * field,unsigned short data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 69 | function (callable) | Add_Field | PacketClass | (char * field,long data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 70 | function (callable) | Add_Field | PacketClass | (char * field,unsigned long data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 71 | function (callable) | Add_Field | PacketClass | (char * field,char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PACKET.H | 72 | function (callable) | Add_Field | PacketClass | (char * field,void * data,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 46 | function (callable) | operator ()  | NoInitClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 58 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 69 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 70 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 98 | function (callable) | operator ()  | SystemTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 99 | function (callable) | operator long | SystemTimerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 128 | function (callable) | PaletteClass | PaletteClass | (RGBClass const & rgb) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 151 | function (callable) | operator == | PaletteClass | (PaletteClass const & palette) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 174 | function (callable) | operator = | PaletteClass | (PaletteClass const & palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 200 | function (callable) | Adjust | PaletteClass | (int ratio) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 227 | function (callable) | Adjust | PaletteClass | (int ratio,PaletteClass const & palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 265 | function (callable) | Partial_Adjust | PaletteClass | (int ratio,char * lut) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 299 | function (callable) | Partial_Adjust | PaletteClass | (int ratio,PaletteClass const & palette,char * lut) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 325 | function (callable) | Closest_Color | PaletteClass | (RGBClass const & rgb) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.CPP | 383 | function (callable) | Set | PaletteClass | (int time,void (* callback)(void)) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 52 | function (callable) | PaletteClass | PaletteClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 55 | function (callable) | operator []  | PaletteClass | (unsigned index) |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 56 | function (callable) | operator []  | PaletteClass | (unsigned index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 58 | function (callable) | operator != | PaletteClass | (PaletteClass const & palette) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 64 | function (callable) | Get_Data | PaletteClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PALETTEC.H | 65 | function (callable) | Get_Data | PaletteClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.CPP | 60 | function (callable) | ~Pipe | Pipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.CPP | 89 | function (callable) | Put_To | Pipe | (Pipe * pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.CPP | 128 | function (callable) | Put | Pipe | (void const * source,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.CPP | 155 | function (callable) | Flush | Pipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.H | 65 | function (callable) | Pipe | Pipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.H | 69 | function (callable) | End | Pipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PIPE.H | 71 | function (callable) | Put_To | Pipe | (Pipe & pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 63 | function (callable) | PKey | PKey | (void const * exponent,void const * modulus) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 87 | function (callable) | Encode_Modulus | PKey | (void * buffer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 112 | function (callable) | Encode_Exponent | PKey | (void * buffer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 136 | function (callable) | Decode_Modulus | PKey | (void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 158 | function (callable) | Decode_Exponent | PKey | (void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 192 | function (callable) | Generate | PKey | (Straw & random,int bits,PKey & fastkey,PKey & slowkey) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 280 | function (callable) | Encrypt | PKey | (void const * source,int slen,void * dest) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.CPP | 332 | function (callable) | Decrypt | PKey | (void const * source,int slen,void * dest) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.H | 56 | function (callable) | PKey | PKey | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.H | 64 | function (callable) | Plain_Block_Size | PKey | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.H | 65 | function (callable) | Crypt_Block_Size | PKey | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.H | 66 | function (callable) | Block_Count | PKey | (int plaintext_length) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PK.H | 74 | function (callable) | Fast_Exponent | PKey | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 62 | function (callable) | PKPipe | PKPipe | (CryptControl control,RandomStraw & rnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 90 | function (callable) | Put_To | PKPipe | (Pipe * pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 129 | function (callable) | Key | PKPipe | (PKey const * key) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 164 | function (callable) | Put | PKPipe | (void const * source,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 256 | function (callable) | Encrypted_Key_Length | PKPipe | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.CPP | 280 | function (callable) | Plain_Key_Length | PKPipe | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PKPIPE.H | 60 | function (callable) | Put_To | PKPipe | (Pipe & pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 67 | function (callable) | PKStraw | PKStraw | (CryptControl control,RandomStraw & rnd) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 95 | function (callable) | Get_From | PKStraw | (Straw * straw) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 138 | function (callable) | Key | PKStraw | (PKey const * key) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 168 | function (callable) | Get | PKStraw | (void * source,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 273 | function (callable) | Encrypted_Key_Length | PKStraw | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.CPP | 297 | function (callable) | Plain_Key_Length | PKStraw | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/PKSTRAW.H | 55 | function (callable) | Get_From | PKStraw | (Straw & straw) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 70 | function (callable) | PowerClass | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 104 | function (callable) | Init_Clear | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 135 | function (callable) | One_Time | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 163 | function (callable) | Draw_It | PowerClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 265 | function (callable) | AI | PowerClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 369 | function (callable) | Refresh_Cells | PowerClass | (CELL cell,short const * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 391 | function (callable) | Power_Height | PowerClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 434 | function (callable) | Action | PowerClass::PowerButtonClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.CPP | 470 | function (callable) | Flash_Power | PowerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.H | 44 | function (callable) | PowerClass | PowerClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/POWER.H | 65 | function (callable) | PowerButtonClass | PowerClass::PowerButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 61 | function (callable) | Read_Private_Config_Struct |  | (FileClass & file,NewConfigType * config) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 100 | function (callable) | WWGetPrivateProfileHex |  | (char const * section,char const * entry,char * profile) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 141 | function (callable) | WWGetPrivateProfileInt |  | (char const * section,char const * entry,int def,char * profile) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 183 | function (callable) | WWWritePrivateProfileInt |  | (char const * section,char const * entry,int value,char * profile) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 233 | function (callable) | WWGetPrivateProfileString |  | (char const * section,char const * entry,char const * def,char * retbuffer,int retlen,char const * profile) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 547 | function (callable) | WWWritePrivateProfileString |  | (char const * section,char const * entry,char const * string,char * profile) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 684 | function (callable) | Read_Bin_Buffer |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 690 | function (callable) | Read_Bin_Init |  | (char * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 700 | function (callable) | Read_Bin_Length |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 710 | function (callable) | Read_Bin_Num |  | (void * num,int length,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 731 | function (callable) | Read_Bin_Pos |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 741 | function (callable) | Read_Bin_PosSet |  | (unsigned int pos,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 752 | function (callable) | Read_Bin_String |  | (char * string,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 779 | function (callable) | Write_Bin_Buffer |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 785 | function (callable) | Write_Bin_Init |  | (char * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 795 | function (callable) | Write_Bin_Length |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 805 | function (callable) | Write_Bin_Num |  | (void * num,int length,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 826 | function (callable) | Write_Bin_Pos |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 836 | function (callable) | Write_Bin_PosSet |  | (unsigned int pos,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/PROFILE.CPP | 847 | function (callable) | Write_Bin_String |  | (char * string,int length,char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 238 | function (callable) | Queue_Mission |  | (TargetClass whom,MissionType mission,TARGET target,TARGET destination) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 273 | function (callable) | Queue_Mission |  | (TargetClass whom,MissionType mission,TARGET target,TARGET destination,SpeedType speed,MPHType maxspeed) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 298 | function (callable) | Queue_Options |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 325 | function (callable) | Queue_Exit |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 352 | function (callable) | Queue_AI |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 403 | function (callable) | Queue_AI_Normal |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 551 | function (callable) | Queue_AI_Multiplayer |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 915 | function (callable) | Wait_For_Players |  | (int first_time,ConnManClass * net,int resend_delta,int dialog_time,int timeout,char * multi_packet_buf,int my_sent,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1294 | function (callable) | Generate_Timing_Event |  | (ConnManClass * net,int my_sent) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1378 | function (callable) | Generate_Real_Timing_Event |  | (ConnManClass * net,int my_sent) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1496 | function (callable) | Generate_Process_Time_Event |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1558 | function (callable) | Process_Send_Period |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1608 | function (callable) | Send_Packets |  | (ConnManClass * net,char * multi_packet_buf,int multi_packet_max,int max_ahead,int my_sent) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1736 | function (callable) | Send_FrameSync |  | (ConnManClass * net,int cmd_count) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1806 | function (callable) | Process_Receive_Packet |  | (ConnManClass * net,char * multi_packet_buf,int id,int packetlen,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 1955 | function (callable) | Process_Serial_Packet |  | (char * multi_packet_buf,int first_time) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2095 | function (callable) | Can_Advance |  | (ConnManClass * net,int max_ahead,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2164 | function (callable) | Process_Reconnect_Dialog |  | (CDTimerClass<SystemTimerClass> * timeout_timer,long * their_frame,int num_conn,int reconn,int fresh) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2243 | function (callable) | Handle_Timeout |  | (ConnManClass * net,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2347 | function (callable) | Stop_Game |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2395 | function (callable) | Build_Send_Packet |  | (void * buf,int bufsize,int frame_delay,int num_cmds,int cap) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2496 | function (callable) | Add_Uncompressed_Events |  | (void * buf,int bufsize,int frame_delay,int size,int cap) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2592 | function (callable) | Add_Compressed_Events |  | (void * buf,int bufsize,int frame_delay,int size,int cap) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2906 | function (callable) | Breakup_Receive_Packet |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 2944 | function (callable) | Extract_Uncompressed_Events |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3024 | function (callable) | Extract_Compressed_Events |  | (void * buf,int bufsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3245 | function (callable) | Execute_DoList |  | (int max_houses,HousesType base_house,ConnManClass * net,CDTimerClass<FrameTimerClass> * skip_crc,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3560 | function (callable) | Clean_DoList |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3608 | function (callable) | Queue_Record |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3658 | function (callable) | Queue_Playback |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3799 | function (callable) | Compute_Game_CRC |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3901 | function (callable) | Add_CRC |  | (unsigned long * crc,unsigned long val) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 3933 | function (callable) | Print_CRCs |  | (EventClass * ev) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 4250 | function (callable) | Init_Queue_Mono |  | (ConnManClass * net) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 4329 | function (callable) | Update_Queue_Mono |  | (ConnManClass * net,int flow_index) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 4379 | function (callable) | Print_Framesync_Values |  | (long curframe,unsigned long max_ahead,int num_connections,unsigned short * their_recv,unsigned short * their_sent,unsigned short my_sent) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 4432 | function (callable) | Dump_Packet_Too_Late_Stuff |  | (EventClass * event,ConnManClass * net,long * their_frame,unsigned short * their_sent,unsigned short * their_recv) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.CPP | 4491 | function (callable) | Check_Mirror |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 138 | function (callable) | QueueClass | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 159 | function (callable) | Init | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 183 | function (callable) | Add | QueueClass | (T const & q) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 213 | function (callable) | Next | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 244 | function (callable) | operator [] | QueueClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 269 | function (callable) | First | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 275 | function (callable) | Get_Head | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 281 | function (callable) | Get_Tail | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/QUEUE.H | 287 | function (callable) | Get_Array | QueueClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 100 | function (callable) | RadarClass | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 148 | function (callable) | One_Time | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 198 | function (callable) | Init_Clear | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 244 | function (callable) | Radar_Activate | RadarClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 363 | function (callable) | Draw_It | RadarClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 646 | function (callable) | Render_Terrain | RadarClass | (CELL cell,int x,int y,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 743 | function (callable) | Render_Infantry | RadarClass | (CELL cell,int x,int y,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 803 | function (callable) | Render_Overlay | RadarClass | (CELL cell,int x,int y,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 870 | function (callable) | Zoom_Mode | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 986 | function (callable) | Is_Zoomable | RadarClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1022 | function (callable) | Plot_Radar_Pixel | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1152 | function (callable) | Radar_Pixel | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1185 | function (callable) | Click_In_Radar | RadarClass | (int & ptr_x,int & ptr_y,bool change) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1234 | function (callable) | Click_Cell_Calc | RadarClass | (int x,int y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1267 | function (callable) | Map_Cell | RadarClass | (CELL cell,HouseClass * house,bool check_radar_spied,bool and_for_allies) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1278 | function (callable) | Cursor_Cell | RadarClass | (CELL cell,int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1305 | function (callable) | Mark_Radar | RadarClass | (int x1,int y1,int x2,int y2,int value,int barlen) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1374 | function (callable) | Cell_XY_To_Radar_Pixel | RadarClass | (int cellx,int celly,int & x,int & y) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1397 | function (callable) | Jam_Cell | RadarClass | (CELL cell,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1439 | function (callable) | UnJam_Cell | RadarClass | (CELL cell,HouseClass * house) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1469 | function (callable) | Radar_Cursor | RadarClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1592 | function (callable) | Radar_Anim | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1649 | function (callable) | AI | RadarClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1744 | function (callable) | Action | RadarClass::RTacticalClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1947 | function (callable) | Refresh_Cells | RadarClass | (CELL cell,short const * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 1971 | function (callable) | Set_Radar_Position | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2172 | function (callable) | Radar_Position | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2195 | function (callable) | Set_Map_Dimensions | RadarClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2218 | function (callable) | Set_Tactical_Position | RadarClass | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2240 | function (callable) | Cell_On_Radar | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2267 | function (callable) | Player_Names | RadarClass | (bool on) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2294 | function (callable) | Spy_Next_House | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2356 | function (callable) | Draw_House_Info | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2462 | function (callable) | Draw_Names | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2557 | function (callable) | Activate_Pulse | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2580 | function (callable) | Is_Radar_Active | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2601 | function (callable) | Is_Radar_Activating | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2622 | function (callable) | Is_Radar_Existing | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2642 | function (callable) | Get_Jammed | RadarClass | (HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2653 | function (callable) | Get_Jammed | RadarClass | (HouseClass * player) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2661 | function (callable) | Set_Jammed | RadarClass | (HousesType house,bool jam) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2672 | function (callable) | Set_Jammed | RadarClass | (HouseClass * player,bool jam) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.CPP | 2679 | function (callable) | Flag_Cell | RadarClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.H | 44 | function (callable) | RadarClass | RadarClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.H | 103 | function (callable) | Is_Player_Names | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.H | 104 | function (callable) | Spying_On_House | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.H | 107 | function (callable) | Is_Zoomed | RadarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADAR.H | 171 | function (callable) | RTacticalClass | RadarClass::RTacticalClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.CPP | 99 | function (callable) | Debug_Dump | RadioClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.CPP | 139 | function (callable) | Receive_Message | RadioClass | (RadioClass * from,RadioMessageType message,long & param) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.CPP | 201 | function (callable) | Transmit_Message | RadioClass | (RadioMessageType message,long & param,RadioClass * to) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.CPP | 255 | function (callable) | Limbo | RadioClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.CPP | 285 | function (callable) | Transmit_Message | RadioClass | (RadioMessageType message,RadioClass * to) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 77 | function (callable) | RadioClass | RadioClass | (RTTIType rtti,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 78 | function (callable) | RadioClass | RadioClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 79 | function (callable) | ~RadioClass | RadioClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 84 | function (callable) | In_Radio_Contact | RadioClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 85 | function (callable) | Radio_Off | RadioClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RADIO.H | 86 | function (callable) | Contact_With_Whom | RadioClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 78 | function (callable) | RAMFileClass | RAMFileClass | (void * buffer,int len) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 108 | function (callable) | ~RAMFileClass | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 134 | function (callable) | Create | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 159 | function (callable) | Delete | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 183 | function (callable) | Is_Available | RAMFileClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 203 | function (callable) | Is_Open | RAMFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 228 | function (callable) | Open | RAMFileClass | (char const *,int access) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 250 | function (callable) | Open | RAMFileClass | (int access) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 297 | function (callable) | Read | RAMFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 346 | function (callable) | Seek | RAMFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 398 | function (callable) | Size | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 422 | function (callable) | Write | RAMFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.CPP | 469 | function (callable) | Close | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 49 | function (callable) | File_Name | RAMFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 50 | function (callable) | Set_Name | RAMFileClass | (char const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 62 | function (callable) | Get_Date_Time | RAMFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 63 | function (callable) | Set_Date_Time | RAMFileClass | (unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 64 | function (callable) | Error | RAMFileClass | (int,int=false,char const * =NULL) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAMFILE.H | 66 | function (callable) | operator char const* | RAMFileClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAND.CPP | 55 | function (callable) | Sim_Random |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAND.CPP | 104 | function (callable) | Sim_IRandom |  | (int minval,int maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/RANDOM.CPP | 60 | function (callable) | RandomClass | RandomClass | (unsigned seed) |  | SIGNATURE_CATALOGUED |
| REDALERT/RANDOM.CPP | 86 | function (callable) | operator () | RandomClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RANDOM.CPP | 125 | function (callable) | operator ()  | RandomClass | (int minval,int maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/RANDOM.H | 56 | function (callable) | operator int | RandomClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 93 | function (callable) | Error | RawFileClass | (int,int,char const *) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 115 | function (callable) | RawFileClass | RawFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 148 | function (callable) | Set_Name | RawFileClass | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 190 | function (callable) | Open | RawFileClass | (char const * filename,int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 215 | function (callable) | Open | RawFileClass | (int rights) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 341 | function (callable) | Is_Available | RawFileClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 453 | function (callable) | Close | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 528 | function (callable) | Read | RawFileClass | (void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 659 | function (callable) | Write | RawFileClass | (void const * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 779 | function (callable) | Seek | RawFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 848 | function (callable) | Size | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 937 | function (callable) | Create | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 975 | function (callable) | Delete | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 1065 | function (callable) | Get_Date_Time | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 1143 | function (callable) | Set_Date_Time | RawFileClass | (unsigned long datetime) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 1231 | function (callable) | Bias | RawFileClass | (int start,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.CPP | 1274 | function (callable) | Raw_Seek | RawFileClass | (long pos,int dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.H | 242 | function (callable) | File_Name | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.H | 264 | function (callable) | RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.H | 297 | function (callable) | ~RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWFILE.H | 323 | function (callable) | Is_Open | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 41 | function (callable) | RAChatEventSink | RAChatEventSink | (WolapiObject * pOwnerIn) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 64 | function (callable) | ~RAChatEventSink | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 79 | function (callable) | QueryInterface | RAChatEventSink | (const IID & iid,void ** ppv) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 99 | function (callable) | AddRef | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 109 | function (callable) | Release | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 122 | function (callable) | OnServerList | RAChatEventSink | (HRESULT hRes,Server * pServerHead) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 189 | function (callable) | OnPageSend | RAChatEventSink | (HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 204 | function (callable) | OnPaged | RAChatEventSink | (HRESULT,User * pUser,LPCSTR szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 228 | function (callable) | OnFind | RAChatEventSink | (HRESULT hRes,Channel * pChannel) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 246 | function (callable) | OnLogout | RAChatEventSink | (HRESULT hRes,User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 263 | function (callable) | OnBusy | RAChatEventSink | (HRESULT) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 269 | function (callable) | OnIdle | RAChatEventSink | (HRESULT) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 275 | function (callable) | OnConnection | RAChatEventSink | (HRESULT hRes,LPCSTR motd) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 340 | function (callable) | OnChannelCreate | RAChatEventSink | (HRESULT hRes,Channel *) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 360 | function (callable) | OnChannelModify | RAChatEventSink | (HRESULT,Channel *) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 366 | function (callable) | OnChannelJoin | RAChatEventSink | (HRESULT hRes,Channel *,User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 456 | function (callable) | InsertUserSorted | RAChatEventSink | (User * pUserNew) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 500 | function (callable) | operator < |  | (const User & u1,const User & u2) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 514 | function (callable) | OnChannelLeave | RAChatEventSink | (HRESULT hRes,Channel *,User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 613 | function (callable) | OnChannelTopic | RAChatEventSink | (HRESULT,Channel *,LPCSTR) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 619 | function (callable) | OnGroupList | RAChatEventSink | (HRESULT,Group *) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 625 | function (callable) | OnPublicMessage | RAChatEventSink | (HRESULT,Channel *,User * pUserSender,LPCSTR szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 654 | function (callable) | OnPrivateMessage | RAChatEventSink | (HRESULT,User * pUserSender,LPCSTR szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 718 | function (callable) | bSpecialMessage | RAChatEventSink | (const char * szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 734 | function (callable) | OnSystemMessage | RAChatEventSink | (HRESULT,LPCSTR) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 740 | function (callable) | OnNetStatus | RAChatEventSink | (HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 783 | function (callable) | OnChannelList | RAChatEventSink | (HRESULT,Channel * pChannelListIn) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 886 | function (callable) | DeleteChannelList | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 900 | function (callable) | OnUserList | RAChatEventSink | (HRESULT,Channel *,User * pUserListIn) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 932 | function (callable) | DeleteUserList | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 950 | function (callable) | OnUpdateList | RAChatEventSink | (HRESULT hRes,Update * pUpdateList) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 987 | function (callable) | DownloadUpdates | RAChatEventSink | (Update * pUpdateList,int iUpdates) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1069 | function (callable) | OnServerError | RAChatEventSink | (HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1077 | function (callable) | OnMessageOfTheDay | RAChatEventSink | (HRESULT,LPCSTR) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1083 | function (callable) | ActionEggSound | RAChatEventSink | (const char * szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1191 | function (callable) | OnPrivateAction | RAChatEventSink | (HRESULT,User * pUserSender,LPCSTR szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1211 | function (callable) | OnPublicAction | RAChatEventSink | (HRESULT,Channel *,User * pUserSender,LPCSTR szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1227 | function (callable) | OnPrivateGameOptions | RAChatEventSink | (HRESULT,User * pUser,LPCSTR szRequest) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1249 | function (callable) | OnPublicGameOptions | RAChatEventSink | (HRESULT,Channel *,User *,LPCSTR szInform) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1267 | function (callable) | OnGameStart | RAChatEventSink | (HRESULT hRes,Channel *,User * pUserIn,int iGameID) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1323 | function (callable) | GetPlayerGameIP | RAChatEventSink | (const char * szPlayerName) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1337 | function (callable) | OnUserKick | RAChatEventSink | (HRESULT hRes,Channel *,User * pUserKicked,User * pUserKicker) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1393 | function (callable) | OnUserIP | RAChatEventSink | (HRESULT hRes,User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1436 | function (callable) | DeleteUserIPList | RAChatEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1450 | function (callable) | GetUserIP | RAChatEventSink | (const char * szName) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1471 | function (callable) | OnServerError | RAChatEventSink | (HRESULT,LPCSTR) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1477 | function (callable) | OnServerBannedYou | RAChatEventSink | (HRESULT,time_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1483 | function (callable) | OnUserFlags | RAChatEventSink | (HRESULT hRes,LPCSTR name,unsigned int flags,unsigned int) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1545 | function (callable) | OnChannelBan | RAChatEventSink | (HRESULT,LPCSTR name,int banned) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1561 | function (callable) | RADownloadEventSink | RADownloadEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1576 | function (callable) | QueryInterface | RADownloadEventSink | (const IID & iid,void ** ppv) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1595 | function (callable) | AddRef | RADownloadEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1604 | function (callable) | Release | RADownloadEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1616 | function (callable) | OnEnd | RADownloadEventSink | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1624 | function (callable) | OnError | RADownloadEventSink | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1640 | function (callable) | OnProgressUpdate | RADownloadEventSink | (int bytesread,int totalsize,int timetaken,int timeleft) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1654 | function (callable) | OnStatusUpdate | RADownloadEventSink | (int status) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1667 | function (callable) | OnQueryResume | RADownloadEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1680 | function (callable) | RANetUtilEventSink | RANetUtilEventSink | (WolapiObject * pOwnerIn) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1691 | function (callable) | ~RANetUtilEventSink | RANetUtilEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1702 | function (callable) | QueryInterface | RANetUtilEventSink | (const IID & iid,void ** ppv) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1722 | function (callable) | AddRef | RANetUtilEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1732 | function (callable) | Release | RANetUtilEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1744 | function (callable) | OnGameresSent | RANetUtilEventSink | (HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1752 | function (callable) | OnLadderList | RANetUtilEventSink | (HRESULT hRes,Ladder * pLadderListIn,int,long,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1824 | function (callable) | OnPing | RANetUtilEventSink | (HRESULT hRes,int time,unsigned long ip,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1854 | function (callable) | DeleteLadderList | RANetUtilEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1868 | function (callable) | GetUserRank | RANetUtilEventSink | (const char * szName,bool bRankRA) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 1897 | function (callable) | ChatDefAsText |  | (char * szDesc,HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 2029 | function (callable) | DebugChatDef |  | (HRESULT hRes) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 2037 | function (callable) | iChannelLobbyNumber |  | (const unsigned char * szChannelName) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.CPP | 2052 | function (callable) | InterpretLobbyNumber |  | (char * szLobbyNameToSet,int iLobby) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.H | 39 | macro (function-like macro) | DECLSPEC_UUID |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/RAWOLAPI.H | 239 | function (callable) | ~RADownloadEventSink | RADownloadEventSink | () |  | SIGNATURE_CATALOGUED |
| REDALERT/READLINE.CPP | 29 | function (callable) | strtrimcpp |  | (char * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/READLINE.CPP | 59 | function (callable) | Read_Line |  | (FileClass & file,char * buffer,int len,bool & eof) |  | SIGNATURE_CATALOGUED |
| REDALERT/READLINE.CPP | 65 | function (callable) | Read_Line |  | (Straw & file,char * buffer,int len,bool & eof) |  | SIGNATURE_CATALOGUED |
| REDALERT/RECT.CPP | 58 | function (callable) | Rect | Rect | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/RECT.CPP | 83 | function (callable) | Is_Valid | Rect | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RECT.CPP | 112 | function (callable) | Intersect | Rect | (Rect const & rectangle,int * x,int * y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RECT.CPP | 172 | function (callable) | Union |  | (Rect const & rect1,Rect const & rect2) |  | SIGNATURE_CATALOGUED |
| REDALERT/RECT.H | 63 | function (callable) | Size | Rect | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 41 | function (callable) | RegionClass | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 42 | function (callable) | ~RegionClass | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 43 | function (callable) | operator != | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 44 | function (callable) | operator == | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 45 | function (callable) | operator > | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 46 | function (callable) | operator < | RegionClass | (RegionClass const & region) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 48 | function (callable) | Reset_Threat | RegionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 49 | function (callable) | Adjust_Threat | RegionClass | (int threat,int neg) |  | SIGNATURE_CATALOGUED |
| REDALERT/REGION.H | 50 | function (callable) | Threat_Value | RegionClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 62 | function (callable) | _Pop_Group_Out_Of_Object |  | (FootClass * group,TechnoClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 126 | function (callable) | _Need_To_Take |  | (AircraftClass const * air) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 164 | function (callable) | _Create_Group |  | (TeamTypeClass const * teamtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 294 | function (callable) | _Consists_Only_Of_Infantry |  | (FootClass const * first) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 324 | function (callable) | _Who_Can_Pop_Out_Of |  | (CELL origin) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 370 | function (callable) | Do_Reinforcements |  | (TeamTypeClass const * teamtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 558 | function (callable) | Create_Special_Reinforcement |  | (HouseClass * house,TechnoTypeClass const * type,TechnoTypeClass const * another,TeamMissionType mission,int argument) |  | SIGNATURE_CATALOGUED |
| REDALERT/REINF.CPP | 638 | function (callable) | Create_Air_Reinforcement |  | (HouseClass * house,AircraftType air,int number,MissionType mission,TARGET tarcom,TARGET navcom,InfantryType passenger) |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.CPP | 67 | function (callable) | Adjust | RGBClass | (int ratio,RGBClass const & rgb) |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.CPP | 109 | function (callable) | Difference | RGBClass | (RGBClass const & rgb) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.CPP | 139 | function (callable) | operator HSVClass | RGBClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.CPP | 239 | function (callable) | Set | RGBClass | (int color) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 85 | function (callable) | RGBClass | RGBClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 86 | function (callable) | RGBClass | RGBClass | (unsigned char red,unsigned char green,unsigned char blue) |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 92 | function (callable) | operator = | RGBClass | (RGBClass const & rgb) |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 108 | function (callable) | Red_Component | RGBClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 109 | function (callable) | Green_Component | RGBClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 110 | function (callable) | Blue_Component | RGBClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 117 | function (callable) | Raw_Set | RGBClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RGB.H | 125 | function (callable) | Raw_Color_Prep | RGBClass | (unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 66 | function (callable) | RandomStraw | RandomStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 89 | function (callable) | ~RandomStraw | RandomStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 110 | function (callable) | Reset | RandomStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 139 | function (callable) | Seed_Bits_Needed | RandomStraw | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 165 | function (callable) | Seed_Bit | RandomStraw | (int seed) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 195 | function (callable) | Seed_Byte | RandomStraw | (char seed) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 218 | function (callable) | Seed_Short | RandomStraw | (short seed) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 241 | function (callable) | Seed_Long | RandomStraw | (long seed) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 268 | function (callable) | Scramble_Seed | RandomStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNDSTRAW.CPP | 302 | function (callable) | Get | RandomStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/RNG.H | 44 | function (callable) | ~RandomNumberGenerator | RandomNumberGenerator | () |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.CPP | 66 | function (callable) | Rotate_Bitmap |  | (GraphicViewPortClass * srcvp,GraphicViewPortClass * destvp,int a) |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.H | 18 | macro (function-like macro) | MAKE_PTR |  | (vp,x,y) |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.H | 19 | macro (function-like macro) | MAKE_PTR2 |  | (vp,x,y) |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.H | 25 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.H | 36 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| REDALERT/ROTBMP.H | 37 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 71 | function (callable) | _Scale_To_256 |  | (int val) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 95 | function (callable) | RulesClass | RulesClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 311 | function (callable) | Difficulty_Get |  | (CCINIClass & ini,DifficultyClass & diff,char const * section) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 344 | function (callable) | Process | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 382 | function (callable) | General | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 537 | function (callable) | MPlayer | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 568 | function (callable) | Recharge | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 602 | function (callable) | Heap_Maximums | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 724 | function (callable) | AI | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 780 | function (callable) | Powerups | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 840 | function (callable) | Land_Types | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 889 | function (callable) | Themes | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 937 | function (callable) | IQ | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 974 | function (callable) | Objects | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 1045 | function (callable) | Difficulty | RulesClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/RULES.CPP | 1071 | function (callable) | Is_MCV_Deploy |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SAVEDLG.H | 63 | function (callable) | SaveOptionsClass | SaveOptionsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 128 | function (callable) | Put_All |  | (Pipe & pipe,int save_net) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 320 | function (callable) | Save_Game |  | (int id,char const * descr,bool) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 344 | function (callable) | Save_Game |  | (const char * file_name,const char * descr) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 492 | function (callable) | Load_Game |  | (int id) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 513 | function (callable) | Load_Game |  | (const char * file_name) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1066 | function (callable) | Save_Misc_Values |  | (Pipe & file) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1129 | function (callable) | Load_Misc_Values |  | (Straw & file) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1202 | function (callable) | Save_MPlayer_Values |  | (Pipe & file) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1231 | function (callable) | Load_MPlayer_Values |  | (Straw & file) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1262 | function (callable) | Code_All_Pointers |  | (void) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1349 | function (callable) | Decode_All_Pointers |  | (void) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1458 | function (callable) | Get_Savefile_Info |  | (int id,char * buf,unsigned * scenp,HousesType * housep) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1540 | function (callable) | Reconcile_Players |  | (void) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SAVELOAD.CPP | 1651 | function (callable) | MPlayer_Save_Message |  | (void) | RUST/original-source-reference/REDALERT/SAVELOAD.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 130 | function (callable) | ScenarioClass | ScenarioClass | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 200 | function (callable) | Do_BW_Fade | ScenarioClass | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 223 | function (callable) | Do_Fade_AI | ScenarioClass | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 265 | function (callable) | Set_Global_To | ScenarioClass | (int global,bool value) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 313 | function (callable) | Start_Scenario |  | (char * name,bool briefing) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 430 | function (callable) | Set_Scenario_Difficulty |  | (int difficulty) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 470 | function (callable) | Read_Scenario |  | (char * name) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 588 | function (callable) | Fill_In_Data |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 731 | function (callable) | Post_Load_Game |  | (int load_multi) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 765 | function (callable) | Clear_Scenario |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 872 | function (callable) | Do_Win |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1152 | function (callable) | Do_Lose |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1241 | function (callable) | Do_Draw |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1305 | function (callable) | Do_Restart |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1358 | function (callable) | Restate_Mission |  | (char const * name,int button1,int button2) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1395 | function (callable) | BGMessageBox |  | (char const * msg,int btn1,int btn2) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1859 | function (callable) | Set_Scenario_Name | ScenarioClass | (int scenario,ScenarioPlayerType player,ScenarioDirType dir,ScenarioVarType var) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 1990 | function (callable) | Set_Scenario_Name | ScenarioClass | (char const * name) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 2046 | function (callable) | Read_Scenario_INI |  | (char * fname,bool) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 2695 | function (callable) | Write_Scenario_INI |  | (char * fname) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 2787 | function (callable) | Assign_Houses |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 2949 | function (callable) | Remove_AI_Players |  | (void) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3010 | function (callable) | Reserve_Infantry |  | () | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3019 | function (callable) | Reserve_Unit |  | () | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3027 | function (callable) | Create_Units |  | (bool official) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3470 | function (callable) | Scan_Place_Object |  | (ObjectClass * obj,CELL cell) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3583 | function (callable) | Clip_Scatter |  | (CELL cell,int maxdist) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3658 | function (callable) | Clip_Move |  | (CELL cell,FacingType facing,int dist) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCENARIO.CPP | 3736 | function (callable) | Disect_Scenario_Name |  | (char const * name,int & scenario,ScenarioPlayerType & player,ScenarioDirType & dir,ScenarioVarType & var) | RUST/original-source-reference/REDALERT/SCENARIO.md | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 125 | function (callable) | ScoreAnimClass | ScoreAnimClass | (int x,int y,void const * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 134 | function (callable) | ScoreTimeClass | ScoreTimeClass | (int xpos,int ypos,void const * data,int maxval,int xtimer) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 142 | function (callable) | Update | ScoreTimeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 163 | function (callable) | ScoreCredsClass | ScoreCredsClass | (int xpos,int ypos,void const * data,int maxval,int xtimer) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 174 | function (callable) | Update | ScoreCredsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 201 | function (callable) | ScorePrintClass | ScorePrintClass | (int string,int xpos,int ypos,void const * palette,int background) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 210 | function (callable) | ScorePrintClass | ScorePrintClass | (void const * string,int xpos,int ypos,void const * palette,int background) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 219 | function (callable) | Update | ScorePrintClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 269 | function (callable) | ScoreScaleClass | ScoreScaleClass | (void const * string,int xpos,int ypos,char const palette[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 281 | function (callable) | Update | ScoreScaleClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 320 | function (callable) | Alloc_Object |  | (ScoreAnimClass * obj) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 356 | function (callable) | Presentation | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 874 | function (callable) | Cycle_Wait_Click |  | (bool cycle) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 932 | function (callable) | Do_Nod_Buildings_Graph | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1046 | function (callable) | Do_GDI_Graph | ScoreClass | (void const * yellowptr,void const * redptr,int gkilled,int nkilled,int ypos) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1170 | function (callable) | Do_Nod_Casualties_Graph | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1269 | function (callable) | Show_Credits | ScoreClass | (int house,char const pal[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1343 | function (callable) | Print_Minutes | ScoreClass | (int minutes) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1379 | function (callable) | Count_Up_Print | ScoreClass | (char * str,int percent,int maxval,int xpos,int ypos) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1408 | function (callable) | Input_Name | ScoreClass | (char str[],int xpos,int ypos,char const pal[]) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1489 | function (callable) | Animate_Cursor |  | (int pos,int ypos) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1535 | function (callable) | Draw_InfantryMen |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1571 | function (callable) | Draw_InfantryMan |  | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1622 | function (callable) | New_Infantry_Anim |  | (int index,int anim) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1650 | function (callable) | Draw_Bar_Graphs |  | (int i,int gkilled,int nkilled) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1702 | function (callable) | Call_Back_Delay |  | (int time) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1733 | function (callable) | Animate_Score_Objs |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1753 | function (callable) | Int_Print |  | (int a) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1777 | function (callable) | Multi_Score_Presentation |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.CPP | 1927 | function (callable) | Init | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 43 | function (callable) | ScoreClass | ScoreClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 44 | function (callable) | ScoreClass | ScoreClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 92 | function (callable) | Update | ScoreAnimClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 93 | function (callable) | ~ScoreAnimClass | ScoreAnimClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 106 | function (callable) | ~ScoreCredsClass | ScoreCredsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 116 | function (callable) | ~ScoreTimeClass | ScoreTimeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 127 | function (callable) | ~ScorePrintClass | ScorePrintClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCORE.H | 136 | function (callable) | ~ScoreScaleClass | ScoreScaleClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCREEN.H | 51 | function (callable) | ScreenClass | ScreenClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCROLL.CPP | 63 | function (callable) | ScrollClass | ScrollClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCROLL.CPP | 90 | function (callable) | AI | ScrollClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCROLL.CPP | 251 | function (callable) | Set_Autoscroll | ScrollClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCROLL.H | 63 | function (callable) | ScrollClass | ScrollClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/SCROLL.H | 68 | function (callable) | Init_IO | ScrollClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 191 | function (callable) | SmudgeTypeClass | SmudgeTypeClass | (SmudgeType smudge,char const * ininame,int fullname,int width,int height,bool isbib,bool iscrater) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 236 | function (callable) | operator new | SmudgeTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 257 | function (callable) | operator delete | SmudgeTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 278 | function (callable) | Init_Heap | SmudgeTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 319 | function (callable) | From_Name | SmudgeTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 349 | function (callable) | Occupy_List | SmudgeTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 379 | function (callable) | Init | SmudgeTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 411 | function (callable) | Display | SmudgeTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 445 | function (callable) | Prep_For_Add | SmudgeTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 473 | function (callable) | Create_And_Place | SmudgeTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 500 | function (callable) | Create_One_Of | SmudgeTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 521 | function (callable) | Draw_It | SmudgeTypeClass | (int x,int y,int data) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 544 | function (callable) | One_Time | SmudgeTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SDATA.CPP | 565 | function (callable) | As_Reference | SmudgeTypeClass | (SmudgeType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 209 | function (callable) | IndexClass | IndexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 235 | function (callable) | ~IndexClass | IndexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 257 | function (callable) | Clear | IndexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 285 | function (callable) | Increase_Table_Size | IndexClass | (int amount) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 340 | function (callable) | Count | IndexClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 362 | function (callable) | Is_Present | IndexClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 420 | function (callable) | Fetch_Index | IndexClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 450 | function (callable) | Is_Archive_Same | IndexClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 476 | function (callable) | Invalidate_Archive | IndexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 498 | function (callable) | Set_Archive | IndexClass | (NodeElement const * node) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 525 | function (callable) | Add_Index | IndexClass | (int id,T data) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 570 | function (callable) | Remove_Index | IndexClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 627 | function (callable) | search_compfunc | IndexClass | (void const * ptr1,void const * ptr2) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEARCH.H | 656 | function (callable) | Search_For_Node | IndexClass | (int id) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SEDITDLG.CPP | 30 | function (callable) | SimpleEditDlgClass | SimpleEditDlgClass | (int iDialogWidth,const char * szTitle,const char * szPrompt,int iEditCharsAccept,const char * szPrompt2,int iEditCharsAccept2) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEDITDLG.CPP | 68 | function (callable) | ~SimpleEditDlgClass | SimpleEditDlgClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SEDITDLG.CPP | 76 | function (callable) | SetButtons | SimpleEditDlgClass | (const char * szOk,const char * szCancel,const char * szMiddle) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEDITDLG.CPP | 84 | function (callable) | Show | SimpleEditDlgClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SENDFILE.CPP | 89 | function (callable) | Get_Scenario_File_From_Host |  | (char * return_name,int gametype) |  | SIGNATURE_CATALOGUED |
| REDALERT/SENDFILE.CPP | 212 | function (callable) | Receive_Remote_File |  | (char * file_name,unsigned int file_length,int gametype) |  | SIGNATURE_CATALOGUED |
| REDALERT/SENDFILE.CPP | 519 | function (callable) | Send_Remote_File |  | (char * file_name,int gametype) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 70 | function (callable) | SequencedConnClass | SequencedConnClass | (int numsend,int numreceive,int maxlen,unsigned short magicnum,unsigned long retry_delta,unsigned long max_retries,unsigned long timeout) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 105 | function (callable) | ~SequencedConnClass | SequencedConnClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 126 | function (callable) | Init | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 155 | function (callable) | Send_Packet | SequencedConnClass | (void * buf,int buflen,int ack_req) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 212 | function (callable) | Receive_Packet | SequencedConnClass | (void * buf,int buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 343 | function (callable) | Get_Packet | SequencedConnClass | (void * buf,int * buflen) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 392 | function (callable) | Service_Send_Queue | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SEQCONN.CPP | 504 | function (callable) | Service_Receive_Queue | SequencedConnClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 150 | function (callable) | SessionClass | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 253 | function (callable) | ~SessionClass | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 273 | function (callable) | One_Time | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 301 | function (callable) | Init | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 327 | function (callable) | Create_Connections | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 381 | function (callable) | Create_TEN_Connections | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 436 | function (callable) | Create_MPATH_Connections | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 485 | function (callable) | Am_I_Master | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 531 | function (callable) | Save | SessionClass | (Pipe & file) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 573 | function (callable) | Load | SessionClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 620 | function (callable) | Save | SessionClass | (CCFileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 667 | function (callable) | Load | SessionClass | (CCFileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 717 | function (callable) | Read_MultiPlayer_Settings | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1022 | function (callable) | Write_MultiPlayer_Settings | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1195 | function (callable) | Is_Mission_Counterstrike |  | (char * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1213 | function (callable) | Is_Mission_Aftermath |  | (char * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1239 | function (callable) | Is_Mission_126x126 |  | (char * file_name) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1274 | function (callable) | Read_Scenario_Descriptions | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1565 | function (callable) | Free_Scenario_Descriptions | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1612 | function (callable) | Trap_Object | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1760 | function (callable) | Compute_Unique_ID | SessionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1803 | function (callable) | MultiMission | MultiMission | (char const * filename,char const * description,char const * digest,bool official,bool expansion) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1813 | function (callable) | Draw_It | MultiMission | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1835 | function (callable) | Set_Description | MultiMission | (char const * description) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1844 | function (callable) | Set_Filename | MultiMission | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1852 | function (callable) | Set_Digest | MultiMission | (char const * digest) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1864 | function (callable) | Set_Official | MultiMission | (bool official) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.CPP | 1869 | function (callable) | Set_Expansion | MultiMission | (bool expansion) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.H | 429 | function (callable) | Description | MultiMission | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.H | 430 | function (callable) | Get_Filename | MultiMission | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.H | 431 | function (callable) | Get_Digest | MultiMission | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.H | 432 | function (callable) | Get_Official | MultiMission | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SESSION.H | 433 | function (callable) | Get_Expansion | MultiMission | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 46 | macro (function-like macro) | min |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 72 | function (callable) | Process_Partial | SHAEngine | (void const * & data,long & length) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 123 | function (callable) | Hash | SHAEngine | (void const * data,long length) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 160 | macro (function-like macro) | Reverse_LONG |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 178 | function (callable) | Result | SHAEngine | (void * result) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 243 | function (callable) | _rotl |  | (T X,int n) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.CPP | 272 | function (callable) | Process_Block | SHAEngine | (void const * source,SHADigest & acc) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 68 | function (callable) | SHAEngine | SHAEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 76 | function (callable) | Init | SHAEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 85 | function (callable) | Digest_Size | SHAEngine | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 123 | function (callable) | Get_Constant | SHAEngine | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 131 | function (callable) | Function1 | SHAEngine | (long X,long Y,long Z) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 136 | function (callable) | Function2 | SHAEngine | (long X,long Y,long Z) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 141 | function (callable) | Function3 | SHAEngine | (long X,long Y,long Z) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 146 | function (callable) | Function4 | SHAEngine | (long X,long Y,long Z) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHA.H | 150 | function (callable) | Do_Function | SHAEngine | (int index,long X,long Y,long Z) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPEBTN.CPP | 55 | function (callable) | ShapeButtonClass | ShapeButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPEBTN.CPP | 85 | function (callable) | ShapeButtonClass | ShapeButtonClass | (unsigned id,void const * shape,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPEBTN.CPP | 109 | function (callable) | Set_Shape | ShapeButtonClass | (void const * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPEBTN.CPP | 135 | function (callable) | Draw_Me | ShapeButtonClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPEBTN.H | 47 | function (callable) | Get_Shape_Data | ShapeButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPIPE.CPP | 59 | function (callable) | Put | SHAPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPIPE.CPP | 82 | function (callable) | Result | SHAPipe | (void * result) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHAPIPE.H | 49 | function (callable) | SHAPipe | SHAPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHASTRAW.CPP | 61 | function (callable) | Get | SHAStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/SHASTRAW.CPP | 90 | function (callable) | Result | SHAStraw | (void * result) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SHASTRAW.H | 50 | function (callable) | SHAStraw | SHAStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 142 | function (callable) | SidebarClass | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 191 | function (callable) | SidebarClass | SidebarClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 219 | function (callable) | One_Time | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 271 | function (callable) | Init_Clear | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 300 | function (callable) | Init_IO | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 372 | function (callable) | Init_Theater | SidebarClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 394 | function (callable) | Reload_Sidebar | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 457 | function (callable) | Which_Column | SidebarClass | (RTTIType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 486 | function (callable) | Factory_Link | SidebarClass | (int factory,RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 514 | function (callable) | Refresh_Cells | SidebarClass | (CELL cell,short const * list) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 544 | function (callable) | Activate_Repair | SidebarClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 592 | function (callable) | Activate_Upgrade | SidebarClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 637 | function (callable) | Activate_Demolish | SidebarClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 682 | function (callable) | Add | SidebarClass | (RTTIType type,int id,bool via_capture) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 723 | function (callable) | Scroll | SidebarClass | (bool up,int column) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 765 | function (callable) | Draw_It | SidebarClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 835 | function (callable) | AI | SidebarClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 955 | function (callable) | Recalc | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 992 | function (callable) | Activate | SidebarClass | (int control) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1090 | function (callable) | StripClass | SidebarClass::StripClass | (InitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1128 | function (callable) | One_Time | SidebarClass::StripClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1161 | function (callable) | Get_Special_Cameo | SidebarClass::StripClass | (SpecialWeaponType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1182 | function (callable) | Init_Clear | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1218 | function (callable) | Init_IO | SidebarClass::StripClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1275 | function (callable) | Init_Theater | SidebarClass::StripClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1307 | function (callable) | Reload_LogoShapes | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1351 | function (callable) | Activate | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1381 | function (callable) | Deactivate | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1409 | function (callable) | Add | SidebarClass::StripClass | (RTTIType type,int id,bool via_capture) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1449 | function (callable) | Scroll | SidebarClass::StripClass | (bool up) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1477 | function (callable) | Flag_To_Redraw | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1506 | function (callable) | AI | SidebarClass::StripClass | (KeyNumType & input,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1680 | function (callable) | Draw_It | SidebarClass::StripClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1913 | function (callable) | Recalc | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 1983 | function (callable) | SelectClass | SidebarClass::StripClass::SelectClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2010 | function (callable) | Set_Owner | SidebarClass::StripClass::SelectClass | (StripClass & strip,int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2037 | function (callable) | Action | SidebarClass::StripClass::SelectClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2284 | function (callable) | Action | SidebarClass::SBGadgetClass | (unsigned,KeyNumType &) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2314 | function (callable) | Factory_Link | SidebarClass::StripClass | (int factory,RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2350 | function (callable) | Abandon_Production | SidebarClass | (RTTIType type,int factory) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2372 | function (callable) | Abandon_Production | SidebarClass::StripClass | (int factory) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.CPP | 2420 | function (callable) | Zoom_Mode_Control | SidebarClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.H | 144 | function (callable) | SelectClass | SidebarClass::StripClass::SelectClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.H | 156 | function (callable) | StripClass | SidebarClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.H | 158 | function (callable) | StripClass | SidebarClass::StripClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBAR.H | 367 | function (callable) | SBGadgetClass | SidebarClass::SBGadgetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 69 | function (callable) | SidebarGlyphxClass | SidebarGlyphxClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 91 | function (callable) | Init_Clear | SidebarGlyphxClass | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 122 | function (callable) | Init_IO | SidebarGlyphxClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 150 | function (callable) | Which_Column | SidebarGlyphxClass | (RTTIType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 179 | function (callable) | Factory_Link | SidebarGlyphxClass | (int factory,RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 202 | function (callable) | Add | SidebarGlyphxClass | (RTTIType type,int id,bool via_capture) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 246 | function (callable) | AI | SidebarGlyphxClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 274 | function (callable) | Recalc | SidebarGlyphxClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 296 | function (callable) | StripClass | SidebarGlyphxClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 323 | function (callable) | Init_Clear | SidebarGlyphxClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 359 | function (callable) | Add | SidebarGlyphxClass::StripClass | (RTTIType type,int id,bool via_capture) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 403 | function (callable) | AI | SidebarGlyphxClass::StripClass | (KeyNumType & input,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 484 | function (callable) | Recalc | SidebarGlyphxClass::StripClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 555 | function (callable) | Factory_Link | SidebarGlyphxClass::StripClass | (int factory,RTTIType type,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 588 | function (callable) | Abandon_Production | SidebarGlyphxClass | (RTTIType type,int factory) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 610 | function (callable) | Abandon_Production | SidebarGlyphxClass::StripClass | (int factory) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 647 | function (callable) | Code_Pointers | SidebarGlyphxClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 669 | function (callable) | Decode_Pointers | SidebarGlyphxClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 691 | function (callable) | Load | SidebarGlyphxClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 719 | function (callable) | Save | SidebarGlyphxClass | (Pipe & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 731 | function (callable) | Sidebar_Glyphx_Init_Clear |  | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 739 | function (callable) | Sidebar_Glyphx_Init_IO |  | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 747 | function (callable) | Sidebar_Glyphx_Abandon_Production |  | (RTTIType type,int factory,HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 757 | function (callable) | Sidebar_Glyphx_Add |  | (RTTIType type,int id,HouseClass * player_ptr,bool via_capture) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 767 | function (callable) | Sidebar_Glyphx_Recalc |  | (HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 775 | function (callable) | Sidebar_Glyphx_AI |  | (HouseClass * player_ptr,KeyNumType & input) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 783 | function (callable) | Sidebar_Glyphx_Factory_Link |  | (int factory,RTTIType type,int id,HouseClass * player_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 793 | function (callable) | Sidebar_Glyphx_Save |  | (Pipe & file,SidebarGlyphxClass * sidebar) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 801 | function (callable) | Sidebar_Glyphx_Load |  | (Straw & file,SidebarGlyphxClass * sidebar) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 809 | function (callable) | Sidebar_Glyphx_Code_Pointers |  | (SidebarGlyphxClass * sidebar) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.CPP | 816 | function (callable) | Sidebar_Glyphx_Decode_Pointers |  | (SidebarGlyphxClass * sidebar) |  | SIGNATURE_CATALOGUED |
| REDALERT/SIDEBARGlyphx.H | 126 | function (callable) | Set_Parent_Sidebar | SidebarGlyphxClass::StripClass | (SidebarGlyphxClass * parent) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 63 | function (callable) | SliderClass | SliderClass | (unsigned id,int x,int y,int w,int h,int belong_to_list) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 109 | function (callable) | ~SliderClass | SliderClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 134 | function (callable) | Set_Maximum | SliderClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 157 | function (callable) | Set_Thumb_Size | SliderClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 178 | function (callable) | Set_Value | SliderClass | (int value) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 202 | function (callable) | Recalc_Thumb | SliderClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 228 | function (callable) | Action | SliderClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 292 | function (callable) | Bump | SliderClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 312 | function (callable) | Step | SliderClass | (int up) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 332 | function (callable) | Draw_Thumb | SliderClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 352 | function (callable) | Draw_Me | SliderClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.CPP | 401 | function (callable) | Peer_To_Peer | SliderClass | (unsigned flags,KeyNumType &,ControlClass & whom) |  | SIGNATURE_CATALOGUED |
| REDALERT/SLIDER.H | 70 | function (callable) | Thumb_Pixels | SliderClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 66 | function (callable) | operator new | SmudgeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 90 | function (callable) | operator delete | SmudgeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 119 | function (callable) | SmudgeClass | SmudgeClass | (SmudgeType type,COORDINATE pos,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 149 | function (callable) | Init | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 171 | function (callable) | Mark | SmudgeClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 253 | function (callable) | Disown | SmudgeClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 293 | function (callable) | Read_INI | SmudgeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.CPP | 335 | function (callable) | Write_INI | SmudgeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 60 | function (callable) | operator new | SmudgeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 63 | function (callable) | SmudgeClass | SmudgeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 64 | function (callable) | operator SmudgeType | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 65 | function (callable) | ~SmudgeClass | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 74 | function (callable) | INI_Name | SmudgeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 78 | function (callable) | Class_Of | SmudgeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SMUDGE.H | 80 | function (callable) | Draw_It | SmudgeClass | (int,int,WindowNumberType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SOUNDDLG.CPP | 43 | function (callable) | MusicListClass | MusicListClass | (int id,int x,int y,int w,int h) | RUST/original-source-reference/REDALERT/SOUNDDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/SOUNDDLG.CPP | 46 | function (callable) | ~MusicListClass | MusicListClass | (void) | RUST/original-source-reference/REDALERT/SOUNDDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/SOUNDDLG.CPP | 67 | function (callable) | Process | SoundControlsClass | (void) | RUST/original-source-reference/REDALERT/SOUNDDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/SOUNDDLG.CPP | 432 | function (callable) | Draw_Entry | MusicListClass | (int index,int x,int y,int width,int selected) | RUST/original-source-reference/REDALERT/SOUNDDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/SOUNDDLG.H | 108 | function (callable) | SoundControlsClass | SoundControlsClass | (void) | RUST/original-source-reference/REDALERT/SOUNDDLG.md | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 72 | function (callable) | Init | SpecialClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 104 | function (callable) | Special_Dialog |  | (bool simple) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 236 | function (callable) | PWEditClass | PWEditClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w=-1,int h=-1) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 261 | function (callable) | Draw_Text | PWEditClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 315 | function (callable) | Fetch_Password |  | (int caption,int message,int btext) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPECIAL.CPP | 474 | function (callable) | Fetch_Difficulty |  | (bool amath) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPRITE.CPP | 32 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPRITE.CPP | 43 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPRITE.CPP | 44 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SPRITE.CPP | 134 | function (callable) | Scale_Rotate | GraphicBufferClass | (BitmapClass & bmp,TPoint2D const & pt,long scale,unsigned char angle) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 60 | function (callable) | StageClass | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 61 | function (callable) | StageClass | StageClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 63 | function (callable) | Fetch_Stage | StageClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 64 | function (callable) | Fetch_Rate | StageClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 65 | function (callable) | Set_Stage | StageClass | (int stage) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 66 | function (callable) | Set_Rate | StageClass | (int rate) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 67 | function (callable) | AI | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 68 | function (callable) | About_To_Change | StageClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/STAGE.H | 69 | function (callable) | Graphic_Logic | StageClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 100 | function (callable) | DllMain |  | (HINSTANCE instance,unsigned int fdwReason,void * lpvReserved) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 170 | function (callable) | DLL_Startup |  | (const char * command_line_in) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 819 | function (callable) | InitDDraw |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 969 | function (callable) | Prog_End |  | (const char * why,bool fatal) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 998 | function (callable) | Prog_End |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 1013 | function (callable) | Print_Error_End_Exit |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 1023 | function (callable) | Print_Error_Exit |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 1053 | function (callable) | Emergency_Exit |  | (int code) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 1127 | function (callable) | Read_Setup_Options |  | (RawFileClass * config_file) |  | SIGNATURE_CATALOGUED |
| REDALERT/STARTUP.CPP | 1195 | function (callable) | Get_OS_Version |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 68 | function (callable) | StaticButtonClass | StaticButtonClass | (unsigned,char const * text,TextPrintType style,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 102 | function (callable) | StaticButtonClass | StaticButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 126 | function (callable) | Draw_Me | StaticButtonClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 172 | function (callable) | Set_Text | StaticButtonClass | (char const * text,bool resize) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 214 | function (callable) | Draw_Background | StaticButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATBTN.CPP | 253 | function (callable) | Draw_Text | StaticButtonClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATS.CPP | 190 | function (callable) | Send_Statistics_Packet |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATS.CPP | 897 | function (callable) | Register_Game_Start_Time |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STATS.CPP | 904 | function (callable) | Register_Game_End_Time |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STRAW.CPP | 60 | function (callable) | ~Straw | Straw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STRAW.CPP | 90 | function (callable) | Get_From | Straw | (Straw * straw) |  | SIGNATURE_CATALOGUED |
| REDALERT/STRAW.CPP | 130 | function (callable) | Get | Straw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/STRAW.H | 64 | function (callable) | Straw | Straw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/STRAW.H | 68 | function (callable) | Get_From | Straw | (Straw & pipe) |  | SIGNATURE_CATALOGUED |
| REDALERT/STUB.CPP | 19 | function (callable) | main |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 68 | function (callable) | SuperClass | SuperClass | (int recharge,bool powered,VoxType charging,VoxType ready,VoxType impatient,VoxType suspend) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 99 | function (callable) | Suspend | SuperClass | (bool on) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 142 | function (callable) | Enable | SuperClass | (bool onetime,bool player,bool quiet) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 171 | function (callable) | Remove | SuperClass | (bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 198 | function (callable) | Recharge | SuperClass | (bool player) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 238 | function (callable) | Discharged | SuperClass | (bool player) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 271 | function (callable) | AI | SuperClass | (bool player) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 315 | function (callable) | Anim_Stage | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 350 | function (callable) | Impatient_Click | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.CPP | 377 | function (callable) | Forced_Charge | SuperClass | (bool player) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 42 | function (callable) | SuperClass | SuperClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 43 | function (callable) | SuperClass | SuperClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 54 | function (callable) | Is_Ready | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 55 | function (callable) | Is_Present | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 56 | function (callable) | Is_One_Time | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 57 | function (callable) | Is_Powered | SuperClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPER.H | 60 | function (callable) | Get_Recharge_Time | SuperClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 39 | label (assembly label candidate) | IDEAL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 40 | label (assembly label candidate) | P386 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 41 | label (assembly label candidate) | MODEL |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 43 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 123 | label (assembly label candidate) | if |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SUPPORT.ASM | 450 | label (assembly label candidate) | endif |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.CPP | 39 | function (callable) | Surface | Surface | (int w,int h,Buffer const * buffer,int pitch) |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.CPP | 73 | function (callable) | Surface | Surface | (Surface const & surface,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.CPP | 82 | function (callable) | Copy_To | Surface | (Buffer & buffer,int x,int y,int w,int h) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.CPP | 106 | function (callable) | Copy_To | Surface | (Rect const & fromrect,Buffer & tobuffer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 53 | function (callable) | Surface | Surface | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 56 | function (callable) | ~Surface | Surface | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 77 | function (callable) | Get_Buffer | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 78 | function (callable) | Get_Size | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 79 | function (callable) | Get_Width | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 80 | function (callable) | Get_Height | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 81 | function (callable) | Get_Pitch | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/SURFACE.H | 84 | function (callable) | Bytes_Per_Line | Surface | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/Shape.cpp | 52 | function (callable) | Set_Shape_Buffer |  | (void const * buffer,int size) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 60 | function (callable) | TabClass | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 88 | function (callable) | Draw_It | TabClass | (bool complete) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 144 | function (callable) | Draw_Credits_Tab | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 166 | function (callable) | Hilite_Tab | TabClass | (int tab) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 214 | function (callable) | AI | TabClass | (KeyNumType & input,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 273 | function (callable) | Set_Active | TabClass | (int select) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 305 | function (callable) | One_Time | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.CPP | 313 | function (callable) | Flash_Money | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.H | 45 | function (callable) | TabClass | TabClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TAB.H | 54 | function (callable) | Redraw_Tab | TabClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 159 | function (callable) | Draw_It | ActionChoiceClass | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 197 | function (callable) | Detach | TActionClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 225 | function (callable) | Build_INI_Entry | TActionClass | (char * ptr) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 247 | function (callable) | Read_INI | TActionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 293 | function (callable) | Code_Pointers | TActionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 314 | function (callable) | Decode_Pointers | TActionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 345 | function (callable) | operator ()  | TActionClass | (HousesType house,ObjectClass * object,int id,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 787 | function (callable) | Action_From_Name |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 818 | function (callable) | Name_From_Action |  | (TActionType action) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.CPP | 839 | function (callable) | Action_Needs |  | (TActionType action) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 118 | function (callable) | TActionClass | TActionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 122 | function (callable) | TActionClass | TActionClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 136 | function (callable) | ActionChoiceClass | ActionChoiceClass | (TActionType event=TACTION_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 138 | function (callable) | operator TActionType | ActionChoiceClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 139 | function (callable) | operator == | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 140 | function (callable) | operator != | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 141 | function (callable) | operator > | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 142 | function (callable) | operator < | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 143 | function (callable) | operator <= | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 144 | function (callable) | operator >= | ActionChoiceClass | (ActionChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TACTION.H | 145 | function (callable) | Description | ActionChoiceClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 62 | function (callable) | TargetClass | TargetClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 68 | function (callable) | TargetClass | TargetClass | (AbstractClass const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 77 | function (callable) | TargetClass | TargetClass | (AbstractTypeClass const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 86 | function (callable) | TargetClass | TargetClass | (CellClass const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 95 | function (callable) | As_Cell | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 119 | function (callable) | As_Trigger |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 144 | function (callable) | As_Team |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 169 | function (callable) | As_TeamType |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 190 | function (callable) | As_Animation |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 215 | function (callable) | As_Bullet |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 240 | function (callable) | As_Aircraft |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 266 | function (callable) | As_Techno |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 293 | function (callable) | As_Object |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 365 | function (callable) | As_Unit |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 391 | function (callable) | As_Vessel |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 417 | function (callable) | As_Infantry |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 442 | function (callable) | As_Building |  | (TARGET target,bool check_active) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 469 | function (callable) | Target_Legal |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 496 | function (callable) | As_Cell |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 518 | function (callable) | As_Coord |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 570 | function (callable) | As_Movement_Coord |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 614 | function (callable) | As_Abstract | xTargetClass | (bool check_active) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 672 | function (callable) | As_TypeClass | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 734 | function (callable) | As_Techno | xTargetClass | (bool check_active) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 768 | function (callable) | As_Object | xTargetClass | (bool check_active) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 838 | function (callable) | As_Target |  | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 867 | function (callable) | As_Target |  | (COORDINATE coord) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 895 | function (callable) | As_TechnoType |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.CPP | 935 | function (callable) | As_TriggerType |  | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 39 | function (callable) | Target_Kind |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 44 | function (callable) | Target_Value |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 49 | function (callable) | Is_Target_Team |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 50 | function (callable) | Is_Target_TeamType |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 51 | function (callable) | Is_Target_Trigger |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 52 | function (callable) | Is_Target_TriggerType |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 53 | function (callable) | Is_Target_Infantry |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 54 | function (callable) | Is_Target_Bullet |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 55 | function (callable) | Is_Target_Terrain |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 56 | function (callable) | Is_Target_Cell |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 57 | function (callable) | Is_Target_Unit |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 58 | function (callable) | Is_Target_Vessel |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 59 | function (callable) | Is_Target_Building |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 60 | function (callable) | Is_Target_Template |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 61 | function (callable) | Is_Target_Aircraft |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 62 | function (callable) | Is_Target_Animation |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 63 | function (callable) | Is_Target_Object |  | (TARGET a) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 108 | function (callable) | operator RTTIType | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 111 | function (callable) | operator == | xTargetClass | (xTargetClass & tgt) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 114 | function (callable) | As_TARGET | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 116 | function (callable) | Value | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 118 | function (callable) | Invalidate | xTargetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 119 | function (callable) | Is_Valid | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 121 | function (callable) | As_Target | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 132 | function (callable) | As_TriggerType | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 133 | function (callable) | As_TeamType | xTargetClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 134 | function (callable) | As_Terrain | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 135 | function (callable) | As_Bullet | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 136 | function (callable) | As_Anim | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 137 | function (callable) | As_Team | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 138 | function (callable) | As_Infantry | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 139 | function (callable) | As_Unit | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 140 | function (callable) | As_Building | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 141 | function (callable) | As_Aircraft | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 142 | function (callable) | As_Vessel | xTargetClass | (bool check_active=true) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 157 | function (callable) | TargetClass | TargetClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 158 | function (callable) | TargetClass | TargetClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 159 | function (callable) | TargetClass | TargetClass | (RTTIType rtti,int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TARGET.H | 163 | function (callable) | TargetClass | TargetClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 86 | function (callable) | TcpipManagerClass | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 111 | function (callable) | ~TcpipManagerClass | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 132 | function (callable) | Close | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 190 | function (callable) | Init | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 250 | function (callable) | Start_Server | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 301 | function (callable) | Read | TcpipManagerClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 346 | function (callable) | Write | TcpipManagerClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 397 | function (callable) | Add_Client | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 505 | function (callable) | Message_Handler | TcpipManagerClass | (HWND,UINT message,UINT,LONG lParam) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 716 | function (callable) | Copy_To_In_Buffer | TcpipManagerClass | (int bytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 742 | function (callable) | Set_Host_Address | TcpipManagerClass | (char * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 764 | function (callable) | Start_Client | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 875 | function (callable) | Close_Socket | TcpipManagerClass | (SOCKET s) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 886 | function (callable) | Set_Protocol_UDP | TcpipManagerClass | (BOOL state) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.CPP | 893 | function (callable) | Clear_Socket_Error | TcpipManagerClass | (SOCKET socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.H | 107 | function (callable) | Get_Connected | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TCPIP.H | 122 | function (callable) | Get_Connection_Status | TcpipManagerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 499 | function (callable) | TerrainTypeClass | TerrainTypeClass | (TerrainType terrain,int theater,COORDINATE centerbase,bool is_immune,bool is_water,char const * ininame,int fullname,short const * occupy,short const * overlap) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 548 | function (callable) | operator new | TerrainTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 569 | function (callable) | operator delete | TerrainTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 590 | function (callable) | Init_Heap | TerrainTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 651 | function (callable) | One_Time | TerrainTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 670 | function (callable) | Init | TerrainTypeClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 719 | function (callable) | From_Name | TerrainTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 752 | function (callable) | Display | TerrainTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 776 | function (callable) | Prep_For_Add | TerrainTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 801 | function (callable) | Create_And_Place | TerrainTypeClass | (CELL cell,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 827 | function (callable) | Create_One_Of | TerrainTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 850 | function (callable) | Occupy_List | TerrainTypeClass | (bool) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 879 | function (callable) | Overlap_List | TerrainTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 907 | function (callable) | As_Reference | TerrainTypeClass | (TerrainType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/TDATA.CPP | 913 | function (callable) | Coord_Fixup | TerrainTypeClass | (COORDINATE coord) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 96 | function (callable) | _Is_It_Breathing |  | (FootClass const * object) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 138 | function (callable) | _Is_It_Playing |  | (FootClass const * object) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 180 | function (callable) | Debug_Dump | TeamClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 219 | function (callable) | Init | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 240 | function (callable) | operator new | TeamClass | (size_t) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 264 | function (callable) | operator delete | TeamClass | (void * ptr) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 289 | function (callable) | ~TeamClass | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 328 | function (callable) | TeamClass | TeamClass | (TeamTypeClass const * type,HouseClass * owner) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 393 | function (callable) | Assign_Mission_Target | TeamClass | (TARGET new_target) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 467 | function (callable) | AI | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 888 | function (callable) | Add | TeamClass | (FootClass * obj) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 958 | function (callable) | Can_Add | TeamClass | (FootClass * obj,int & typeindex) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1050 | function (callable) | Remove | TeamClass | (FootClass * obj,int typeindex) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1177 | function (callable) | Recruit | TeamClass | (int typeindex) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1345 | function (callable) | Detach | TeamClass | (TARGET target,bool) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1387 | function (callable) | Calc_Center | TeamClass | (TARGET & center,TARGET & close_member) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1571 | function (callable) | Took_Damage | TeamClass | (FootClass *,ResultType result,TechnoClass * source) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1633 | function (callable) | Coordinate_Attack | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1737 | function (callable) | Coordinate_Regroup | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1806 | function (callable) | Coordinate_Do | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 1871 | function (callable) | Coordinate_Move | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2026 | function (callable) | Lagging_Units | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2107 | function (callable) | TMission_Unload | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2191 | function (callable) | TMission_Load | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2276 | function (callable) | Coordinate_Conscript | TeamClass | (FootClass * unit) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2317 | function (callable) | Is_A_Member | TeamClass | (void const * who) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2345 | function (callable) | Suspend_Teams | TeamClass | (int priority,HouseClass const * house) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2382 | function (callable) | Is_Leaving_Map | TeamClass | (void) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2415 | function (callable) | Has_Entered_Map | TeamClass | (void) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2449 | function (callable) | Scan_Limit | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2479 | function (callable) | TMission_Formation | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2686 | function (callable) | TMission_Attack | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2769 | function (callable) | TMission_Spy | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2841 | function (callable) | TMission_Follow | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2865 | function (callable) | TMission_Loop | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2889 | function (callable) | TMission_Invulnerable | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2918 | function (callable) | TMission_Set_Global | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2944 | function (callable) | TMission_Patrol | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 2986 | function (callable) | TMission_Deploy | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.CPP | 3052 | function (callable) | Fetch_A_Leader | TeamClass | (void) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.H | 199 | function (callable) | TeamClass | TeamClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.H | 203 | function (callable) | operator new | TeamClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.H | 216 | function (callable) | Is_Empty | TeamClass | (void) const | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAM.H | 218 | function (callable) | Force_Active | TeamClass | (void) | RUST/original-source-reference/REDALERT/TEAM.md | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 102 | function (callable) | Draw_It | TeamTypeClass | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 162 | function (callable) | TeamTypeClass | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 200 | function (callable) | Init | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 221 | function (callable) | As_Pointer | TeamTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 249 | function (callable) | Mission_From_Name | TeamTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 278 | function (callable) | Name_From_Mission | TeamTypeClass | (TeamMissionType order) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 301 | function (callable) | operator new | TeamTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 326 | function (callable) | operator delete | TeamTypeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 350 | function (callable) | Create_One_Of | TeamTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 375 | function (callable) | Destroy_All_Of | TeamTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 416 | function (callable) | Suggested_New_Team | TeamTypeClass | (HouseClass * house,long,long,long,long,bool alerted) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 513 | function (callable) | From_Name | TeamTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 541 | function (callable) | TeamMission_Needs |  | (TeamMissionType tmtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 616 | function (callable) | Draw_It | TeamMissionClass | (int index,int x,int y,int width,int height,bool selected,TextPrintType flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 654 | function (callable) | Edit | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1392 | function (callable) | atoh |  | (char * str) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1435 | function (callable) | Member_Description | TeamTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1481 | function (callable) | Description | TeamTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1520 | function (callable) | Description | TeamMissionClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1578 | function (callable) | Detach | TeamTypeClass | (TARGET target,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1608 | function (callable) | Read_INI | TeamTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1657 | function (callable) | Fill_In | TeamTypeClass | (char * name,char * entry) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1798 | function (callable) | Write_INI | TeamTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.CPP | 1835 | function (callable) | Build_INI_Entry | TeamTypeClass | (char * buf) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 80 | function (callable) | operator const char* | TeamMissionClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 121 | function (callable) | TeamTypeClass | TeamTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 122 | function (callable) | ~TeamTypeClass | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 125 | function (callable) | operator new | TeamTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 140 | function (callable) | INI_Name | TeamTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEAMTYPE.H | 170 | function (callable) | operator const char* | TeamTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 217 | function (callable) | Is_Players_Army | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 284 | function (callable) | Can_Teleport_Here | TechnoClass | (CELL cell) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 339 | function (callable) | What_Weapon_Should_I_Use | TechnoClass | (TARGET target) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 400 | function (callable) | How_Many_Survivors | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 425 | function (callable) | Combat_Damage | TechnoClass | (int which) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 468 | function (callable) | Is_Allowed_To_Recloak | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 492 | function (callable) | Fire_Data | TechnoClass | (int which) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 511 | function (callable) | Fire_Coord | TechnoClass | (int which) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 557 | function (callable) | Debug_Dump | TechnoClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 615 | function (callable) | TechnoClass | TechnoClass | (RTTIType rtti,int id,HousesType house) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 677 | function (callable) | Is_Visible_On_Radar | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 713 | function (callable) | Revealed | TechnoClass | (HouseClass * house) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 822 | function (callable) | Hidden | TechnoClass | (void) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 851 | function (callable) | Mark | TechnoClass | (MarkType mark) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 892 | function (callable) | Receive_Message | TechnoClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1048 | function (callable) | Per_Cell_Process | TechnoClass | (PCPType why) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1096 | function (callable) | Draw_It | TechnoClass | (int x,int y,WindowNumberType window) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1237 | function (callable) | Unlimbo | TechnoClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1270 | function (callable) | In_Range | TechnoClass | (TARGET target,int which,bool reciprocal_check) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1326 | function (callable) | In_Range | TechnoClass | (ObjectClass const * target,int which,bool reciprocal_check) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1379 | function (callable) | In_Range | TechnoClass | (COORDINATE coord,int which) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1408 | function (callable) | Area_Modify | TechnoClass | (CELL cell) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.CPP | 1515 | function (callable) | Evaluate_Object | TechnoClass | (ThreatType method,int mask,int range,TechnoClass const * object,int & value,int zone) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 257 | function (callable) | TechnoClass | TechnoClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 259 | function (callable) | TechnoClass | TechnoClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 261 | function (callable) | ~TechnoClass | TechnoClass | (void) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 273 | function (callable) | Turret_Facing | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 275 | function (callable) | Techno_Type_Class | TechnoClass | (void) const | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 319 | function (callable) | Made_A_Kill | TechnoClass | (void) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TECHNO.H | 392 | function (callable) | Random_Animate | TechnoClass | (void) | RUST/original-source-reference/REDALERT/TECHNO.md | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.CPP | 61 | function (callable) | Init | TemplateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.CPP | 84 | function (callable) | Mark | TemplateClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.CPP | 181 | function (callable) | operator new | TemplateClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.CPP | 206 | function (callable) | operator delete | TemplateClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.CPP | 231 | function (callable) | TemplateClass | TemplateClass | (TemplateType type,CELL pos) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 59 | function (callable) | operator new | TemplateClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 62 | function (callable) | TemplateClass | TemplateClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 63 | function (callable) | ~TemplateClass | TemplateClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 64 | function (callable) | operator TemplateType | TemplateClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 71 | function (callable) | Class_Of | TemplateClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEMPLATE.H | 78 | function (callable) | Draw_It | TemplateClass | (int,int,WindowNumberType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 109 | function (callable) | TenConnManClass | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 143 | function (callable) | ~TenConnManClass | TenConnManClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 168 | function (callable) | Init | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 209 | function (callable) | Service | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 245 | function (callable) | Send_Private_Message | TenConnManClass | (void * buf,int buflen,int reliable,int conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 305 | function (callable) | Get_Private_Message | TenConnManClass | (void * buf,int * buflen,int * conn_id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 348 | function (callable) | Send_Global_Message | TenConnManClass | (void * buf,int buflen,int reliable,int address) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 417 | function (callable) | Get_Global_Message | TenConnManClass | (void * buf,int * buflen,int * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 448 | function (callable) | Num_Connections | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 470 | function (callable) | Connection_ID | TenConnManClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 496 | function (callable) | Connection_Index | TenConnManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 528 | function (callable) | Create_Connection | TenConnManClass | (int id,char * name,int address) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 555 | function (callable) | Delete_Connection | TenConnManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 591 | function (callable) | Connection_Name | TenConnManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 621 | function (callable) | Connection_Address | TenConnManClass | (int id) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 651 | function (callable) | Global_Num_Send | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 672 | function (callable) | Global_Num_Receive | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 693 | function (callable) | Private_Num_Send | TenConnManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 714 | function (callable) | Private_Num_Receive | TenConnManClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 736 | function (callable) | Flush_All | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 790 | function (callable) | Reset_Response_Time | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 814 | function (callable) | Response_Time | TenConnManClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 836 | function (callable) | Set_Timing | TenConnManClass | (unsigned long,unsigned long,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 861 | function (callable) | Configure_Debug | TenConnManClass | (int,int,int,char **,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 885 | function (callable) | Mono_Debug_Print | TenConnManClass | (int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 908 | function (callable) | terminateApp |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 933 | function (callable) | debugMessage |  | (int,char * msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 979 | function (callable) | doAlert |  | (int,int,char * msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 1025 | function (callable) | doPregameHook |  | (char * joinType,char *,char *,char *,char *,char *) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 1059 | function (callable) | doIncomingPacket |  | (int addr,void * buf,size_t size) |  | SIGNATURE_CATALOGUED |
| REDALERT/TENMGR.CPP | 1106 | function (callable) | doPlayerEntered |  | (int pid,int isYou,char *,char *,char *,long,char *) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 76 | function (callable) | ~TerrainClass | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 105 | function (callable) | Take_Damage | TerrainClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 166 | function (callable) | operator new | TerrainClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 191 | function (callable) | operator delete | TerrainClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 218 | function (callable) | TerrainClass | TerrainClass | (TerrainType type,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 252 | function (callable) | Mark | TerrainClass | (MarkType mark) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 299 | function (callable) | Draw_It | TerrainClass | (int x,int y,WindowNumberType window) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 351 | function (callable) | Init | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 374 | function (callable) | Can_Enter_Cell | TerrainClass | (CELL cell,FacingType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 416 | function (callable) | Catch_Fire | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 453 | function (callable) | Fire_Out | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 488 | function (callable) | AI | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 530 | function (callable) | Debug_Dump | TerrainClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 562 | function (callable) | Unlimbo | TerrainClass | (COORDINATE coord,DirType dir) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 589 | function (callable) | Start_To_Crumble | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 617 | function (callable) | Limbo | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 644 | function (callable) | Center_Coord | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 669 | function (callable) | Radar_Icon | TerrainClass | (CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 711 | function (callable) | Target_Coord | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 736 | function (callable) | Read_INI | TerrainClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.CPP | 769 | function (callable) | Write_INI | TerrainClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 59 | function (callable) | operator new | TerrainClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 62 | function (callable) | TerrainClass | TerrainClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 64 | function (callable) | operator TerrainType | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 76 | function (callable) | Class_Of | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 83 | function (callable) | Render_Coord | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 84 | function (callable) | Sort_Y | TerrainClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 105 | function (callable) | Clicked_As_Target | TerrainClass | (HousesType house,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/TERRAIN.H | 131 | function (callable) | INI_Name | TerrainClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 143 | function (callable) | Draw_It | EventChoiceClass | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 180 | function (callable) | Reset | TEventClass | (TDEventClass & td) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 217 | function (callable) | operator ()  | TEventClass | (TDEventClass & td,TEventType event,HousesType house,ObjectClass const * object,bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 486 | function (callable) | Build_INI_Entry | TEventClass | (char * ptr) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 506 | function (callable) | Read_INI | TEventClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 563 | function (callable) | Event_Needs |  | (TEventType event) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 624 | function (callable) | Event_From_Name |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 653 | function (callable) | Name_From_Event |  | (TEventType event) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.CPP | 677 | function (callable) | Attaches_To |  | (TEventType event) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 104 | function (callable) | TDEventClass | TDEventClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 105 | function (callable) | TDEventClass | TDEventClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 138 | function (callable) | TEventClass | TEventClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 139 | function (callable) | TEventClass | TEventClass | (TEventType event) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 140 | function (callable) | TEventClass | TEventClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 167 | function (callable) | EventChoiceClass | EventChoiceClass | (TEventType event=TEVENT_NONE) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 169 | function (callable) | operator TEventType | EventChoiceClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 170 | function (callable) | operator == | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 171 | function (callable) | operator != | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 172 | function (callable) | operator > | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 173 | function (callable) | operator < | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 174 | function (callable) | operator <= | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 175 | function (callable) | operator >= | EventChoiceClass | (EventChoiceClass const & rvalue) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEVENT.H | 176 | function (callable) | Description | EventChoiceClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 72 | function (callable) | TextButtonClass | TextButtonClass | (unsigned id,char const * text,TextPrintType style,int x,int y,int w,int h,int blackborder) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 103 | function (callable) | TextButtonClass | TextButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 140 | function (callable) | TextButtonClass | TextButtonClass | (unsigned id,int text,TextPrintType style,int x,int y,int w,int h,int blackborder) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 177 | function (callable) | Draw_Me | TextButtonClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 223 | function (callable) | Set_Text | TextButtonClass | (char const * text,bool resize) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 254 | function (callable) | Set_Text | TextButtonClass | (int text,bool resize) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 278 | function (callable) | Draw_Background | TextButtonClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.CPP | 326 | function (callable) | Draw_Text | TextButtonClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/TEXTBTN.H | 50 | function (callable) | Set_Style | TextButtonClass | (TextPrintType style) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 122 | function (callable) | Base_Name | ThemeClass | (ThemeType theme) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 145 | function (callable) | ThemeClass | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 169 | function (callable) | Full_Name | ThemeClass | (ThemeType theme) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 194 | function (callable) | AI | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 235 | function (callable) | Next_Song | ThemeClass | (ThemeType theme) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 283 | function (callable) | Queue_Song | ThemeClass | (ThemeType theme) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 331 | function (callable) | Play_Song | ThemeClass | (ThemeType theme) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 363 | function (callable) | Theme_File_Name | ThemeClass | (ThemeType theme) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 392 | function (callable) | Track_Length | ThemeClass | (ThemeType theme) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 416 | function (callable) | Stop | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 427 | function (callable) | Suspend | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 452 | function (callable) | Still_Playing | ThemeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 478 | function (callable) | Is_Allowed | ThemeClass | (ThemeType index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 531 | function (callable) | From_Name | ThemeClass | (char const * name) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 577 | function (callable) | Scan | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.CPP | 606 | function (callable) | Set_Theme_Data | ThemeClass | (ThemeType theme,int scenario,int owners) |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.H | 69 | function (callable) | What_Is_Playing | ThemeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.H | 71 | function (callable) | Is_Regular | ThemeClass | (ThemeType theme) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.H | 74 | function (callable) | Max_Themes | ThemeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/THEME.H | 80 | function (callable) | Fade_Out | ThemeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOGGLE.CPP | 62 | function (callable) | ToggleClass | ToggleClass | (unsigned id,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOGGLE.CPP | 86 | function (callable) | Turn_On | ToggleClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOGGLE.CPP | 108 | function (callable) | Turn_Off | ToggleClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOGGLE.CPP | 137 | function (callable) | Action | ToggleClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 28 | function (callable) | ToolTipClass | ToolTipClass | (GadgetClass * pGadget,const char * szText,int xShow,int yShow,bool bRightAlign,bool bIconList) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 64 | function (callable) | GetToolTipHit | ToolTipClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 77 | function (callable) | bGadgetHit | ToolTipClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 86 | function (callable) | Move | ToolTipClass | (int xShow,int yShow) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 106 | function (callable) | Show | ToolTipClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 165 | function (callable) | Unshow | ToolTipClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 210 | function (callable) | bOverDifferentLine | ToolTipClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 218 | function (callable) | SaveSurfaceRect |  | (int xRect,int yRect,int wRect,int hRect,char * pBits,WindowNumberType window) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.CPP | 260 | function (callable) | RestoreSurfaceRect |  | (int xRect,int yRect,int wRect,int hRect,const char * pBits,WindowNumberType window) |  | SIGNATURE_CATALOGUED |
| REDALERT/TOOLTIP.H | 33 | function (callable) | ~ToolTipClass | ToolTipClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/TRACKER.CPP | 61 | function (callable) | Detach_This_From_All |  | (TARGET target,bool all) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 66 | function (callable) | Description | TriggerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 87 | function (callable) | Draw_It | TriggerClass | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 125 | function (callable) | TriggerClass | TriggerClass | (TriggerTypeClass * trigtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 152 | function (callable) | ~TriggerClass | TriggerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 195 | function (callable) | Init | TriggerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 224 | function (callable) | Spring | TriggerClass | (TEventType event,ObjectClass * obj,CELL cell,bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 373 | function (callable) | operator new | TriggerClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 399 | function (callable) | operator delete | TriggerClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 422 | function (callable) | As_Target | TriggerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 450 | function (callable) | Find_Or_Make |  | (TriggerTypeClass * trigtype) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.CPP | 486 | function (callable) | Detach | TriggerClass | (TARGET target,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.H | 55 | function (callable) | TriggerClass | TriggerClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.H | 74 | function (callable) | Code_Pointers | TriggerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.H | 75 | function (callable) | Decode_Pointers | TriggerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.H | 83 | function (callable) | Name | TriggerClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGGER.H | 89 | function (callable) | operator new | TriggerClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 71 | function (callable) | TriggerTypeClass | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 96 | function (callable) | ~TriggerTypeClass | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 117 | function (callable) | operator new | TriggerTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 143 | function (callable) | operator delete | TriggerTypeClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 167 | function (callable) | As_Target | TriggerTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 189 | function (callable) | Detach | TriggerTypeClass | (TARGET target,bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 213 | function (callable) | Edit | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 1709 | function (callable) | Description | TriggerTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 1812 | function (callable) | Attaches_To | TriggerTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 1860 | function (callable) | Read_INI | TriggerTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 1932 | function (callable) | Fill_In | TriggerTypeClass | (char * name,char * entry) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 1968 | function (callable) | Write_INI | TriggerTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 2003 | function (callable) | Build_INI_Entry | TriggerTypeClass | (char * buffer) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 2054 | function (callable) | Draw_It | TriggerTypeClass | (int,int x,int y,int width,int height,bool selected,TextPrintType flags) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 2092 | function (callable) | Init | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.CPP | 2116 | function (callable) | From_Name | TriggerTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.H | 101 | function (callable) | TriggerTypeClass | TriggerTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.H | 105 | function (callable) | operator new | TriggerTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.H | 121 | function (callable) | INI_Name | TriggerTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TRIGTYPE.H | 143 | function (callable) | operator const char* | TriggerTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TURRET.CPP | 64 | function (callable) | ~TurretClass | TurretClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TURRET.CPP | 83 | function (callable) | TurretClass | TurretClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TURRET.CPP | 105 | function (callable) | TurretClass | TurretClass | (UnitType classid,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTLABEL.CPP | 59 | function (callable) | TextLabelClass | TextLabelClass | (char * txt,int x,int y,RemapControlType * color,TextPrintType style) |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTLABEL.CPP | 86 | function (callable) | Draw_Me | TextLabelClass | (int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTLABEL.H | 55 | function (callable) | Set_Text | TextLabelClass | (char * txt) |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 44 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 61 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 62 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 63 | label (assembly label candidate) | extern |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 64 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 164 | label (assembly label candidate) | Buffer_Print |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 270 | label (assembly label candidate) | next_char |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 356 | label (assembly label candidate) | loop_top |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 370 | label (assembly label candidate) | draw_char |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 375 | label (assembly label candidate) | while_data |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 384 | label (assembly label candidate) | skiplo |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 398 | label (assembly label candidate) | skiphi |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 404 | label (assembly label candidate) | nextrow |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 423 | label (assembly label candidate) | loop_bottom |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 441 | label (assembly label candidate) | force_line_feed |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 450 | label (assembly label candidate) | line_feed |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 476 | label (assembly label candidate) | lfeed |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 485 | label (assembly label candidate) | overflow |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 487 | label (assembly label candidate) | done |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TXTPRNT.ASM | 508 | label (assembly label candidate) | Get_Font_Palette_Ptr |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 78 | function (callable) | AbstractTypeClass | AbstractTypeClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 79 | function (callable) | ~AbstractTypeClass | AbstractTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 81 | function (callable) | What_Am_I | AbstractTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 82 | function (callable) | As_Target | AbstractTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 86 | function (callable) | Name | AbstractTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 87 | function (callable) | Set_Name | AbstractTypeClass | (char const * buf) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 93 | function (callable) | Code_Pointers | AbstractTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 94 | function (callable) | Decode_Pointers | AbstractTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 151 | function (callable) | HouseTypeClass | HouseTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 163 | function (callable) | operator new | HouseTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 273 | function (callable) | ObjectTypeClass | ObjectTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 289 | function (callable) | Is_Foot | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 290 | function (callable) | Graphic_Name | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 301 | function (callable) | Get_Image_Data | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 302 | function (callable) | Get_Radar_Data | ObjectTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 305 | function (callable) | Display | ObjectTypeClass | (int,int,WindowNumberType,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 540 | function (callable) | TechnoTypeClass | TechnoTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 570 | function (callable) | Max_Passengers | TechnoTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 743 | function (callable) | BuildingTypeClass | BuildingTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 773 | function (callable) | operator StructType | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 776 | function (callable) | operator new | BuildingTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 800 | function (callable) | Get_Buildup_Data | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 802 | function (callable) | Is_Factory | BuildingTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 948 | function (callable) | UnitTypeClass | UnitTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 980 | function (callable) | operator new | UnitTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 986 | function (callable) | Init | UnitTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1065 | function (callable) | VesselTypeClass | VesselTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1084 | function (callable) | operator new | VesselTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1090 | function (callable) | Init | VesselTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1214 | function (callable) | InfantryTypeClass | InfantryTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1236 | function (callable) | operator new | InfantryTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1242 | function (callable) | Init | InfantryTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1313 | function (callable) | AircraftTypeClass | AircraftTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1336 | function (callable) | operator new | AircraftTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1342 | function (callable) | Init | AircraftTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1512 | function (callable) | BulletTypeClass | BulletTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1516 | function (callable) | operator new | BulletTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1521 | function (callable) | Init | BulletTypeClass | (TheaterType) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1525 | function (callable) | Create_And_Place | BulletTypeClass | (CELL,HousesType=HOUSE_NONE) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1526 | function (callable) | Create_One_Of | BulletTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1562 | function (callable) | TerrainTypeClass | TerrainTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1575 | function (callable) | operator new | TerrainTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1628 | function (callable) | TemplateTypeClass | TemplateTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1636 | function (callable) | operator new | TemplateTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1828 | function (callable) | AnimTypeClass | AnimTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1858 | function (callable) | operator new | AnimTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1865 | function (callable) | Create_And_Place | AnimTypeClass | (CELL,HousesType=HOUSE_NONE) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1866 | function (callable) | Create_One_Of | AnimTypeClass | (HouseClass *) const |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1946 | function (callable) | OverlayTypeClass | OverlayTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 1965 | function (callable) | operator new | OverlayTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 2023 | function (callable) | SmudgeTypeClass | SmudgeTypeClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 2035 | function (callable) | operator new | SmudgeTypeClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/TYPE.H | 2048 | function (callable) | Overlap_List | SmudgeTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 792 | function (callable) | UnitTypeClass | UnitTypeClass | (UnitType type,int name,char const * ininame,AnimType exp,RemapType remap,int verticaloffset,int primaryoffset,int primarylateral,int secondaryoffset,int secondarylateral,bool is_goodie,bool is_nominal,bool is_crusher,bool is_harvest,bool is_stealthy,bool is_insignificant,bool is_turret_equipped,bool is_radar_equipped,bool is_fire_anim,bool is_lock_turret,bool is_gigundo,bool is_animating,bool is_jammer,bool is_gapper,int rotation,int toffset,MissionType order) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 883 | function (callable) | operator new | UnitTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 903 | function (callable) | operator delete | UnitTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 923 | function (callable) | Init_Heap | UnitTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 980 | function (callable) | From_Name | UnitTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1014 | function (callable) | Display | UnitTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1042 | function (callable) | Prep_For_Add | UnitTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1068 | function (callable) | One_Time | UnitTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1159 | function (callable) | Create_And_Place | UnitTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1186 | function (callable) | Create_One_Of | UnitTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1207 | function (callable) | As_Reference | UnitTypeClass | (UnitType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1231 | function (callable) | Dimensions | UnitTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1256 | function (callable) | Max_Pips | UnitTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1287 | function (callable) | Turret_Adjust | UnitTypeClass | (DirType dir,int & x,int & y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/UDATA.CPP | 1359 | function (callable) | Read_INI | UnitTypeClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/UDPADDR.CPP | 23 | function (callable) | Get_Broadcast_Addresses |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 148 | function (callable) | Recoil_Adjust |  | (DirType dir,int & x,int & y) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 211 | function (callable) | operator new | UnitClass | (size_t) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 237 | function (callable) | operator delete | UnitClass | (void * ptr) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 261 | function (callable) | ~UnitClass | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 307 | function (callable) | UnitClass | UnitClass | (UnitType classid,HousesType house) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 362 | function (callable) | Debug_Dump | UnitClass | (MonoClass * mono) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 397 | function (callable) | Sort_Y | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 421 | function (callable) | AI | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 538 | function (callable) | Rotation_AI | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 615 | function (callable) | Edge_Of_World_AI | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 642 | function (callable) | Reload_AI | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 674 | function (callable) | Firing_AI | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 743 | function (callable) | Receive_Message | UnitClass | (RadioClass * from,RadioMessageType message,long & param) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 966 | function (callable) | Unlimbo | UnitClass | (COORDINATE coord,DirType dir) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1035 | function (callable) | Take_Damage | UnitClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1251 | function (callable) | Active_Click_With | UnitClass | (ActionType action,ObjectClass * object) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1313 | function (callable) | Active_Click_With | UnitClass | (ActionType action,CELL cell) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1329 | function (callable) | Player_Assign_Mission | UnitClass | (MissionType mission,TARGET target,TARGET destination) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1366 | function (callable) | Enter_Idle_Mode | UnitClass | (bool initial) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1454 | function (callable) | Goto_Clear_Spot | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1552 | function (callable) | Try_To_Deploy | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1685 | function (callable) | Per_Cell_Process | UnitClass | (PCPType why) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 1980 | function (callable) | Shape_Number | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2107 | function (callable) | Draw_It | UnitClass | (int x,int y,WindowNumberType window) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2243 | function (callable) | Tiberium_Check | UnitClass | (CELL & center,int x,int y) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2301 | function (callable) | Goto_Tiberium | UnitClass | (int rad) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2394 | function (callable) | Harvesting | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2475 | function (callable) | Mission_Unload | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 2886 | function (callable) | Mission_Harvest | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3081 | function (callable) | Mission_Hunt | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3143 | function (callable) | Overlap_List | UnitClass | (bool redraw) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3208 | function (callable) | Can_Enter_Cell | UnitClass | (CELL cell,FacingType) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3477 | function (callable) | Init | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3500 | function (callable) | Start_Driver | UnitClass | (COORDINATE & headto) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3530 | function (callable) | What_Action | UnitClass | (ObjectClass const * object) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3694 | function (callable) | What_Action | UnitClass | (CELL cell) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3724 | macro (function-like macro) | XYCELL |  | (x,y) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3725 | function (callable) | Exit_Repair | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3779 | function (callable) | Mission_Guard | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3814 | function (callable) | Mission_Move | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3832 | function (callable) | Mission_Enter | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3873 | function (callable) | Desired_Load_Dir | UnitClass | (ObjectClass * passenger,CELL & moveto) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3967 | function (callable) | Flag_Attach | UnitClass | (HousesType house) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 3996 | function (callable) | Flag_Remove | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4026 | function (callable) | Pip_Count | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4072 | function (callable) | APC_Close_Door | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4095 | function (callable) | APC_Open_Door | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4126 | function (callable) | Crew_Type | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4158 | function (callable) | Mission_Repair | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4214 | function (callable) | Fire_Direction | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4259 | function (callable) | Ok_To_Move | UnitClass | (DirType dir) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4297 | function (callable) | Can_Fire | UnitClass | (TARGET target,int which) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4367 | function (callable) | Fire_At | UnitClass | (TARGET target,int which) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4410 | function (callable) | Class_Of | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4433 | function (callable) | Tiberium_Load | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4444 | function (callable) | Find_Best_Refinery | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4486 | function (callable) | Offload_Tiberium_Bail | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4525 | function (callable) | Approach_Target | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4575 | function (callable) | Overrun_Square | UnitClass | (CELL cell,bool threaten) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4662 | function (callable) | Assign_Destination | UnitClass | (TARGET target) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4814 | function (callable) | Greatest_Threat | UnitClass | (ThreatType threat) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4853 | function (callable) | Read_INI | UnitClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4939 | function (callable) | Write_INI | UnitClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 4986 | function (callable) | Credit_Load | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5009 | function (callable) | Should_Crush_It | UnitClass | (TechnoClass const * it) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5077 | function (callable) | Scatter | UnitClass | (COORDINATE threat,bool forced,bool nokidding) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5117 | function (callable) | Limbo | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5146 | function (callable) | Apply_Temporary_Jamming_Shroud | UnitClass | (HouseClass * house_to_apply_for) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5196 | function (callable) | Unapply_Temporary_Jamming_Shroud | UnitClass | (HouseClass * house_to_unapply_for,unsigned int shroud_bits_applied) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5226 | function (callable) | Shroud_Regen | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.CPP | 5309 | function (callable) | Mission_Guard_Area | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.H | 128 | function (callable) | operator new | UnitClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.H | 131 | function (callable) | UnitClass | UnitClass | (NoInitClass const & x) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.H | 132 | function (callable) | operator UnitType | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.H | 161 | function (callable) | Turret_Facing | UnitClass | (void) const | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UNIT.H | 256 | function (callable) | INI_Name | UnitClass | (void) | RUST/original-source-reference/REDALERT/UNIT.md | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 53 | macro (function-like macro) | htonl |  | (val) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 54 | macro (function-like macro) | ntohl |  | (val) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 73 | function (callable) | UnitTrackerClass | UnitTrackerClass | (int unit_count) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 96 | function (callable) | ~UnitTrackerClass | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 117 | function (callable) | Increment_Unit_Total | UnitTrackerClass | (int unit_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 137 | function (callable) | Decrement_Unit_Total | UnitTrackerClass | (int unit_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 157 | function (callable) | Get_All_Totals | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 177 | function (callable) | Clear_Unit_Total | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 198 | function (callable) | To_Network_Format | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 223 | function (callable) | To_PC_Format | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.CPP | 235 | function (callable) | Get_Unit_Total | UnitTrackerClass | (int unit_type) |  | SIGNATURE_CATALOGUED |
| REDALERT/UTRACKER.H | 56 | function (callable) | Get_Unit_Count | UnitTrackerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 200 | function (callable) | VesselTypeClass | VesselTypeClass | (VesselType type,int name,char const * ininame,AnimType exp,int verticaloffset,int primaryoffset,int primarylateral,int secondaryoffset,int secondarylateral,bool is_eight,bool is_nominal,bool is_turret_equipped,int rotation,int toffset) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 271 | function (callable) | operator new | VesselTypeClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 292 | function (callable) | operator delete | VesselTypeClass | (void * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 312 | function (callable) | Init_Heap | VesselTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 349 | function (callable) | As_Reference | VesselTypeClass | (VesselType type) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 380 | function (callable) | Who_Can_Build_Me | VesselTypeClass | (bool intheory,bool legal,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 426 | function (callable) | Display | VesselTypeClass | (int x,int y,WindowNumberType window,HousesType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 454 | function (callable) | Prep_For_Add | VesselTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 482 | function (callable) | Create_One_Of | VesselTypeClass | (HouseClass * house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 505 | function (callable) | Create_And_Place | VesselTypeClass | (CELL cell,HousesType house) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 533 | function (callable) | Dimensions | VesselTypeClass | (int & width,int & height) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 555 | function (callable) | One_Time | VesselTypeClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 606 | function (callable) | Turret_Adjust | VesselTypeClass | (DirType dir,int & x,int & y) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 648 | function (callable) | Overlap_List | VesselTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 678 | function (callable) | From_Name | VesselTypeClass | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/VDATA.CPP | 706 | function (callable) | Max_Pips | VesselTypeClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 89 | function (callable) | VectorClass | VectorClass | (unsigned size,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 125 | function (callable) | ~VectorClass | VectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 147 | function (callable) | VectorClass | VectorClass | (VectorClass<T> const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 172 | function (callable) | operator = | VectorClass | (VectorClass<T> const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 211 | function (callable) | operator == | VectorClass | (VectorClass<T> const & vector) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 244 | function (callable) | ID | VectorClass | (T const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 267 | function (callable) | ID | VectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 295 | function (callable) | Clear | VectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 326 | function (callable) | Resize | VectorClass | (unsigned newsize,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 409 | function (callable) | BooleanVectorClass | BooleanVectorClass | (unsigned size,unsigned char * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 432 | function (callable) | BooleanVectorClass | BooleanVectorClass | (BooleanVectorClass const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 455 | function (callable) | operator = | BooleanVectorClass | (BooleanVectorClass const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 481 | function (callable) | operator == | BooleanVectorClass | (const BooleanVectorClass & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 503 | function (callable) | Resize | BooleanVectorClass | (unsigned size) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 561 | function (callable) | Clear | BooleanVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 583 | function (callable) | Reset | BooleanVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 606 | function (callable) | Set | BooleanVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.CPP | 636 | function (callable) | Fixup | BooleanVectorClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 84 | function (callable) | VectorClass | VectorClass | (NoInitClass const &) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 89 | function (callable) | operator [] | VectorClass | (unsigned index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 90 | function (callable) | operator [] | VectorClass | (unsigned index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 95 | function (callable) | Length | VectorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 138 | function (callable) | Clear | DynamicVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 141 | function (callable) | Count | DynamicVectorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 154 | function (callable) | Delete_All | DynamicVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 157 | function (callable) | Set_Growth_Step | DynamicVectorClass | (int step) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 160 | function (callable) | Growth_Step | DynamicVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 162 | function (callable) | ID | DynamicVectorClass | (T const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 193 | function (callable) | DynamicVectorArrayClass | DynamicVectorArrayClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 195 | function (callable) | Set_Active_Context | DynamicVectorArrayClass | (int active) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 200 | function (callable) | Clear_All | DynamicVectorArrayClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 208 | function (callable) | Clear | DynamicVectorArrayClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 213 | function (callable) | Count | DynamicVectorArrayClass | () const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 218 | function (callable) | Add | DynamicVectorArrayClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 223 | function (callable) | Add_Head | DynamicVectorArrayClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 228 | function (callable) | Delete | DynamicVectorArrayClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 233 | function (callable) | Delete_All | DynamicVectorArrayClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 243 | function (callable) | Delete_All_Except | DynamicVectorArrayClass | (T const & object,int except) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 256 | function (callable) | Delete | DynamicVectorArrayClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 261 | function (callable) | operator [] | DynamicVectorArrayClass | (unsigned index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 266 | function (callable) | operator [] | DynamicVectorArrayClass | (unsigned index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 271 | function (callable) | Clear | DynamicVectorArrayClass | (int context) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 276 | function (callable) | Count | DynamicVectorArrayClass | (int context) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 281 | function (callable) | Add | DynamicVectorArrayClass | (int context,T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 286 | function (callable) | Add_Head | DynamicVectorArrayClass | (int context,T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 291 | function (callable) | Delete | DynamicVectorArrayClass | (int context,T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 296 | function (callable) | Delete | DynamicVectorArrayClass | (int context,int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 301 | function (callable) | Raw | DynamicVectorArrayClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 306 | function (callable) | Raw | DynamicVectorArrayClass | (int context) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 340 | function (callable) | Length | BooleanVectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 355 | function (callable) | operator [] | BooleanVectorClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 359 | function (callable) | operator [] | BooleanVectorClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 365 | function (callable) | Is_True | BooleanVectorClass | (int index) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 371 | function (callable) | First_False | BooleanVectorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 385 | function (callable) | First_True | BooleanVectorClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 455 | function (callable) | DynamicVectorClass | DynamicVectorClass | (unsigned size,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 483 | function (callable) | Resize | DynamicVectorClass | (unsigned newsize,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 512 | function (callable) | ID | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 539 | function (callable) | Add | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 586 | function (callable) | Add_Head | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 639 | function (callable) | Delete | DynamicVectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 663 | function (callable) | Delete | DynamicVectorClass | (int index) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 708 | function (callable) | VectorClass | VectorClass | (unsigned size,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 745 | function (callable) | ~VectorClass | VectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 767 | function (callable) | VectorClass | VectorClass | (VectorClass<T> const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 792 | function (callable) | operator = | VectorClass | (VectorClass<T> const & vector) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 832 | function (callable) | operator == | VectorClass | (VectorClass<T> const & vector) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 865 | function (callable) | ID | VectorClass | (T const * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 888 | function (callable) | ID | VectorClass | (T const & object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 916 | function (callable) | Clear | VectorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VECTOR.H | 947 | function (callable) | Resize | VectorClass | (unsigned newsize,T const * array) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 87 | function (callable) | VersionClass | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 150 | function (callable) | Version_Number | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 193 | function (callable) | Major_Version | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 291 | function (callable) | Minor_Version | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 413 | function (callable) | Version_Name | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 459 | function (callable) | Read_Text_String | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 491 | function (callable) | Version_Protocol | VersionClass | (unsigned long version) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 541 | function (callable) | Init_Clipping | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 580 | function (callable) | Clip_Version | VersionClass | (unsigned long minver,unsigned long maxver) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 636 | function (callable) | Min_Version | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 691 | function (callable) | Max_Version | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.CPP | 729 | function (callable) | Version_Name |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.H | 84 | function (callable) | ~VersionClass | VersionClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.H | 104 | function (callable) | Version_Text | VersionClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/VERSION.H | 116 | function (callable) | Get_Clipped_Version | VersionClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 85 | function (callable) | VesselClass | VesselClass | (VesselType classid,HousesType house) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 141 | function (callable) | ~VesselClass | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 184 | function (callable) | operator new | VesselClass | (size_t) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 209 | function (callable) | operator delete | VesselClass | (void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 233 | function (callable) | Class_Of | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 256 | function (callable) | Can_Enter_Cell | VesselClass | (CELL cell,FacingType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 335 | function (callable) | Shape_Number | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 384 | function (callable) | Draw_It | VesselClass | (int x,int y,WindowNumberType window) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 512 | function (callable) | Debug_Dump | VesselClass | (MonoClass * mono) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 543 | function (callable) | Overlap_List | VesselClass | (bool redraw) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 589 | function (callable) | AI | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 702 | function (callable) | Per_Cell_Process | VesselClass | (PCPType why) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 773 | function (callable) | What_Action | VesselClass | (ObjectClass const * object) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 851 | function (callable) | Active_Click_With | VesselClass | (ActionType action,ObjectClass * object) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 902 | function (callable) | Active_Click_With | VesselClass | (ActionType action,CELL cell) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 943 | function (callable) | Take_Damage | VesselClass | (int & damage,int distance,WarheadType warhead,TechnoClass * source,bool forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1034 | function (callable) | Can_Fire | VesselClass | (TARGET target,int which) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1186 | function (callable) | Fire_Data | VesselClass | (int which) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1212 | function (callable) | Fire_Coord | VesselClass | (int which) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1256 | function (callable) | Init | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1277 | function (callable) | Greatest_Threat | VesselClass | (ThreatType threat) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1330 | function (callable) | Enter_Idle_Mode | VesselClass | (bool) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1401 | function (callable) | Receive_Message | VesselClass | (RadioClass * from,RadioMessageType message,long & param) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1610 | function (callable) | Desired_Load_Dir | VesselClass | (ObjectClass * passenger,CELL & moveto) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1707 | function (callable) | LST_Open_Door | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1732 | function (callable) | LST_Close_Door | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1755 | function (callable) | Mission_Unload | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1909 | function (callable) | Assign_Destination | VesselClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1955 | function (callable) | Pip_Count | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 1989 | function (callable) | Mission_Retreat | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2041 | function (callable) | Is_Allowed_To_Recloak | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2062 | function (callable) | Read_INI | VesselClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2149 | function (callable) | Write_INI | VesselClass | (CCINIClass & ini) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2196 | function (callable) | Start_Driver | VesselClass | (COORDINATE & headto) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2225 | function (callable) | What_Action | VesselClass | (CELL cell) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2256 | function (callable) | Rotation_AI | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2300 | function (callable) | Combat_AI | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2355 | function (callable) | Edge_Of_World_AI | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2383 | function (callable) | Repair_AI | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.CPP | 2427 | function (callable) | Fire_At | VesselClass | (TARGET target,int which) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 77 | function (callable) | VesselClass | VesselClass | (NoInitClass const & x) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 79 | function (callable) | operator new | VesselClass | (size_t,void * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 81 | function (callable) | operator VesselType | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 88 | function (callable) | Zone_Check_Type | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 94 | function (callable) | Turret_Facing | VesselClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/VESSEL.H | 130 | function (callable) | INI_Name | VesselClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VISUDLG.CPP | 55 | function (callable) | Process | VisualControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VISUDLG.H | 69 | function (callable) | VisualControlsClass | VisualControlsClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 85 | function (callable) | ChronalVortexClass | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 111 | function (callable) | ~ChronalVortexClass | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 133 | function (callable) | Appear | ChronalVortexClass | (COORDINATE coordinate) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 191 | function (callable) | Disappear | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 216 | function (callable) | Hide | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 238 | function (callable) | Show | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 284 | function (callable) | Stop | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 306 | function (callable) | Load | ChronalVortexClass | (Straw & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 333 | function (callable) | Save | ChronalVortexClass | (Pipe & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 371 | function (callable) | AI | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 490 | function (callable) | Movement | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 569 | function (callable) | Set_Target | ChronalVortexClass | (ObjectClass * target) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 595 | function (callable) | Attack | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 686 | function (callable) | Zap_Target | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 764 | function (callable) | Coordinate_Remap | ChronalVortexClass | (GraphicViewPortClass * inbuffer,int x,int y,int width,int height,unsigned char * remap_table) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 842 | function (callable) | Render | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 1086 | function (callable) | Set_Redraw | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 1124 | function (callable) | Setup_Remap_Tables | ChronalVortexClass | (TheaterType theater) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 1195 | function (callable) | Build_Fading_Table | ChronalVortexClass | (PaletteClass const & palette,void * dest,int color,int frac) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.CPP | 1252 | function (callable) | Detach | ChronalVortexClass | (TARGET target) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 97 | function (callable) | Is_Active | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 112 | function (callable) | Get_Position | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 113 | function (callable) | Get_Range | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 114 | function (callable) | Get_Speed | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 115 | function (callable) | Get_Damage | ChronalVortexClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 120 | function (callable) | Set_Range | ChronalVortexClass | (int range) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 121 | function (callable) | Set_Speed | ChronalVortexClass | (int speed) |  | SIGNATURE_CATALOGUED |
| REDALERT/VORTEX.H | 122 | function (callable) | Set_Damage | ChronalVortexClass | (int damage) |  | SIGNATURE_CATALOGUED |
| REDALERT/W95TRACE.CPP | 28 | function (callable) | OutputDebugStringW95 |  | (LPCTSTR,...) |  | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.CPP | 66 | function (callable) | WarheadTypeClass | WarheadTypeClass | (char const * name) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.CPP | 99 | function (callable) | operator new | WarheadTypeClass | (size_t) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.CPP | 119 | function (callable) | operator delete | WarheadTypeClass | (void * pointer) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.CPP | 140 | function (callable) | As_Pointer | WarheadTypeClass | (WarheadType warhead) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.CPP | 165 | function (callable) | Read_INI | WarheadTypeClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.H | 47 | function (callable) | WarheadTypeClass | WarheadTypeClass | (NoInitClass const &) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.H | 50 | function (callable) | operator new | WarheadTypeClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.H | 53 | function (callable) | Code_Pointers | WarheadTypeClass | (void) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.H | 54 | function (callable) | Decode_Pointers | WarheadTypeClass | (void) | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WARHEAD.H | 55 | function (callable) | Name | WarheadTypeClass | (void) const | RUST/original-source-reference/REDALERT/WARHEAD.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 69 | function (callable) | WeaponTypeClass | WeaponTypeClass | (char const * name) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 103 | function (callable) | ~WeaponTypeClass | WeaponTypeClass | (void) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 127 | function (callable) | operator new | WeaponTypeClass | (size_t) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 148 | function (callable) | operator delete | WeaponTypeClass | (void * pointer) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 170 | function (callable) | As_Pointer | WeaponTypeClass | (WeaponType weapon) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 201 | function (callable) | Read_INI | WeaponTypeClass | (CCINIClass & ini) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 255 | function (callable) | Weapon_From_Name |  | (char const * name) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 285 | function (callable) | Armor_From_Name |  | (char const * name) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 314 | function (callable) | Allowed_Threats | WeaponTypeClass | (void) const | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.CPP | 327 | function (callable) | Is_Wall_Destroyer | WeaponTypeClass | (void) const | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.H | 49 | function (callable) | WeaponTypeClass | WeaponTypeClass | (NoInitClass const &) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.H | 53 | function (callable) | operator new | WeaponTypeClass | (size_t,void * ptr) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.H | 56 | function (callable) | Name | WeaponTypeClass | (void) const | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.H | 59 | function (callable) | Code_Pointers | WeaponTypeClass | (void) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WEAPON.H | 60 | function (callable) | Decode_Pointers | WeaponTypeClass | (void) | RUST/original-source-reference/REDALERT/WEAPON.md | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 98 | function (callable) | DPMI_Lock |  | (VOID const *,long const) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 114 | function (callable) | DPMI_Unlock |  | (void const *,long const) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 147 | function (callable) | Alloc |  | (unsigned long bytes_to_alloc,MemoryFlagType flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 319 | function (callable) | Free |  | (void const * pointer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 412 | function (callable) | Resize_Alloc |  | (void * original_ptr,unsigned long new_size_in_bytes) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 447 | function (callable) | Ram_Free |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 474 | function (callable) | Heap_Size |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ALLOC.CPP | 498 | function (callable) | Total_Ram_Free |  | (MemoryFlagType) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.CPP | 62 | function (callable) | BufferClass | BufferClass | (VOID * buffer,LONG size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.CPP | 85 | function (callable) | BufferClass | BufferClass | (LONG size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.CPP | 106 | function (callable) | BufferClass | BufferClass | (VOID) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.CPP | 123 | function (callable) | ~BufferClass | BufferClass | (VOID) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.H | 103 | function (callable) | Get_Size | BufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/BUFFER.H | 117 | function (callable) | Get_Buffer | BufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 85 | function (callable) | Process_DD_Result |  | (HRESULT result,int display_ok_msg) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 408 | function (callable) | Check_Overlapped_Blit_Capability |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 451 | function (callable) | Set_Video_Mode |  | (HWND hwnd,int w,int h,int bits_per_pixel) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 549 | function (callable) | Reset_Video_Mode |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 584 | function (callable) | Get_Free_Video_Memory |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 623 | function (callable) | Get_Video_Hardware_Capabilities |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 686 | function (callable) | Wait_Vert_Blank |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 719 | function (callable) | Set_DD_Palette |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 777 | function (callable) | Wait_Blit |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 804 | function (callable) | SurfaceMonitorClass | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 829 | function (callable) | Add_DD_Surface | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE new_surface) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 858 | function (callable) | Remove_DD_Surface | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE old_surface) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 885 | function (callable) | Got_Surface_Already | SurfaceMonitorClass | (LPDIRECTDRAWSURFACE test_surface) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 911 | function (callable) | Restore_Surfaces | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 966 | function (callable) | Set_Surface_Focus | SurfaceMonitorClass | (BOOL in_focus) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.CPP | 989 | function (callable) | Release | SurfaceMonitorClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 36 | macro (function-like macro) | MAKE_DDHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 368 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDraw,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 398 | macro (function-like macro) | IDirectDraw_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 399 | macro (function-like macro) | IDirectDraw_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 400 | macro (function-like macro) | IDirectDraw_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 401 | macro (function-like macro) | IDirectDraw_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 402 | macro (function-like macro) | IDirectDraw_CreateClipper |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 403 | macro (function-like macro) | IDirectDraw_CreatePalette |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 404 | macro (function-like macro) | IDirectDraw_CreateSurface |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 405 | macro (function-like macro) | IDirectDraw_DuplicateSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 406 | macro (function-like macro) | IDirectDraw_EnumDisplayModes |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 407 | macro (function-like macro) | IDirectDraw_EnumSurfaces |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 408 | macro (function-like macro) | IDirectDraw_FlipToGDISurface |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 409 | macro (function-like macro) | IDirectDraw_GetCaps |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 410 | macro (function-like macro) | IDirectDraw_GetDisplayMode |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 411 | macro (function-like macro) | IDirectDraw_GetFourCCCodes |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 412 | macro (function-like macro) | IDirectDraw_GetGDISurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 413 | macro (function-like macro) | IDirectDraw_GetMonitorFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 414 | macro (function-like macro) | IDirectDraw_GetScanLine |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 415 | macro (function-like macro) | IDirectDraw_GetVerticalBlankStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 416 | macro (function-like macro) | IDirectDraw_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 417 | macro (function-like macro) | IDirectDraw_RestoreDisplayMode |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 418 | macro (function-like macro) | IDirectDraw_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 419 | macro (function-like macro) | IDirectDraw_SetDisplayMode |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 420 | macro (function-like macro) | IDirectDraw_WaitForVerticalBlank |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 428 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDraw2,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 459 | macro (function-like macro) | IDirectDraw2_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 460 | macro (function-like macro) | IDirectDraw2_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 461 | macro (function-like macro) | IDirectDraw2_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 462 | macro (function-like macro) | IDirectDraw2_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 463 | macro (function-like macro) | IDirectDraw2_CreateClipper |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 464 | macro (function-like macro) | IDirectDraw2_CreatePalette |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 465 | macro (function-like macro) | IDirectDraw2_CreateSurface |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 466 | macro (function-like macro) | IDirectDraw2_DuplicateSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 467 | macro (function-like macro) | IDirectDraw2_EnumDisplayModes |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 468 | macro (function-like macro) | IDirectDraw2_EnumSurfaces |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 469 | macro (function-like macro) | IDirectDraw2_FlipToGDISurface |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 470 | macro (function-like macro) | IDirectDraw2_GetCaps |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 471 | macro (function-like macro) | IDirectDraw2_GetDisplayMode |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 472 | macro (function-like macro) | IDirectDraw2_GetFourCCCodes |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 473 | macro (function-like macro) | IDirectDraw2_GetGDISurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 474 | macro (function-like macro) | IDirectDraw2_GetMonitorFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 475 | macro (function-like macro) | IDirectDraw2_GetScanLine |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 476 | macro (function-like macro) | IDirectDraw2_GetVerticalBlankStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 477 | macro (function-like macro) | IDirectDraw2_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 478 | macro (function-like macro) | IDirectDraw2_RestoreDisplayMode |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 479 | macro (function-like macro) | IDirectDraw2_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 480 | macro (function-like macro) | IDirectDraw2_SetDisplayMode |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 481 | macro (function-like macro) | IDirectDraw2_WaitForVerticalBlank |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 482 | macro (function-like macro) | IDirectDraw2_GetAvailableVidMem |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 493 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawPalette,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 507 | macro (function-like macro) | IDirectDrawPalette_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 508 | macro (function-like macro) | IDirectDrawPalette_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 509 | macro (function-like macro) | IDirectDrawPalette_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 510 | macro (function-like macro) | IDirectDrawPalette_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 511 | macro (function-like macro) | IDirectDrawPalette_GetEntries |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 512 | macro (function-like macro) | IDirectDrawPalette_Initialize |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 513 | macro (function-like macro) | IDirectDrawPalette_SetEntries |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 524 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawClipper,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 540 | macro (function-like macro) | IDirectDrawClipper_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 541 | macro (function-like macro) | IDirectDrawClipper_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 542 | macro (function-like macro) | IDirectDrawClipper_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 543 | macro (function-like macro) | IDirectDrawClipper_GetClipList |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 544 | macro (function-like macro) | IDirectDrawClipper_GetHWnd |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 545 | macro (function-like macro) | IDirectDrawClipper_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 546 | macro (function-like macro) | IDirectDrawClipper_IsClipListChanged |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 547 | macro (function-like macro) | IDirectDrawClipper_SetClipList |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 548 | macro (function-like macro) | IDirectDrawClipper_SetHWnd |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 559 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawSurface,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 602 | macro (function-like macro) | IDirectDrawSurface_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 603 | macro (function-like macro) | IDirectDrawSurface_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 604 | macro (function-like macro) | IDirectDrawSurface_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 605 | macro (function-like macro) | IDirectDrawSurface_AddAttachedSurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 606 | macro (function-like macro) | IDirectDrawSurface_AddOverlayDirtyRect |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 607 | macro (function-like macro) | IDirectDrawSurface_Blt |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 608 | macro (function-like macro) | IDirectDrawSurface_BltBatch |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 609 | macro (function-like macro) | IDirectDrawSurface_BltFast |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 610 | macro (function-like macro) | IDirectDrawSurface_DeleteAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 611 | macro (function-like macro) | IDirectDrawSurface_EnumAttachedSurfaces |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 612 | macro (function-like macro) | IDirectDrawSurface_EnumOverlayZOrders |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 613 | macro (function-like macro) | IDirectDrawSurface_Flip |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 614 | macro (function-like macro) | IDirectDrawSurface_GetAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 615 | macro (function-like macro) | IDirectDrawSurface_GetBltStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 616 | macro (function-like macro) | IDirectDrawSurface_GetCaps |  | (p,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 617 | macro (function-like macro) | IDirectDrawSurface_GetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 618 | macro (function-like macro) | IDirectDrawSurface_GetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 619 | macro (function-like macro) | IDirectDrawSurface_GetDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 620 | macro (function-like macro) | IDirectDrawSurface_GetFlipStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 621 | macro (function-like macro) | IDirectDrawSurface_GetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 622 | macro (function-like macro) | IDirectDrawSurface_GetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 623 | macro (function-like macro) | IDirectDrawSurface_GetPixelFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 624 | macro (function-like macro) | IDirectDrawSurface_GetSurfaceDesc |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 625 | macro (function-like macro) | IDirectDrawSurface_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 626 | macro (function-like macro) | IDirectDrawSurface_IsLost |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 627 | macro (function-like macro) | IDirectDrawSurface_Lock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 628 | macro (function-like macro) | IDirectDrawSurface_ReleaseDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 629 | macro (function-like macro) | IDirectDrawSurface_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 630 | macro (function-like macro) | IDirectDrawSurface_SetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 631 | macro (function-like macro) | IDirectDrawSurface_SetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 632 | macro (function-like macro) | IDirectDrawSurface_SetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 633 | macro (function-like macro) | IDirectDrawSurface_SetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 634 | macro (function-like macro) | IDirectDrawSurface_Unlock |  | (p,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 635 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlay |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 636 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlayDisplay |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 637 | macro (function-like macro) | IDirectDrawSurface_UpdateOverlayZOrder |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 645 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectDrawSurface2,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 692 | macro (function-like macro) | IDirectDrawSurface2_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 693 | macro (function-like macro) | IDirectDrawSurface2_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 694 | macro (function-like macro) | IDirectDrawSurface2_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 695 | macro (function-like macro) | IDirectDrawSurface2_AddAttachedSurface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 696 | macro (function-like macro) | IDirectDrawSurface2_AddOverlayDirtyRect |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 697 | macro (function-like macro) | IDirectDrawSurface2_Blt |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 698 | macro (function-like macro) | IDirectDrawSurface2_BltBatch |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 699 | macro (function-like macro) | IDirectDrawSurface2_BltFast |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 700 | macro (function-like macro) | IDirectDrawSurface2_DeleteAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 701 | macro (function-like macro) | IDirectDrawSurface2_EnumAttachedSurfaces |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 702 | macro (function-like macro) | IDirectDrawSurface2_EnumOverlayZOrders |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 703 | macro (function-like macro) | IDirectDrawSurface2_Flip |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 704 | macro (function-like macro) | IDirectDrawSurface2_GetAttachedSurface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 705 | macro (function-like macro) | IDirectDrawSurface2_GetBltStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 706 | macro (function-like macro) | IDirectDrawSurface2_GetCaps |  | (p,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 707 | macro (function-like macro) | IDirectDrawSurface2_GetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 708 | macro (function-like macro) | IDirectDrawSurface2_GetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 709 | macro (function-like macro) | IDirectDrawSurface2_GetDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 710 | macro (function-like macro) | IDirectDrawSurface2_GetFlipStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 711 | macro (function-like macro) | IDirectDrawSurface2_GetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 712 | macro (function-like macro) | IDirectDrawSurface2_GetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 713 | macro (function-like macro) | IDirectDrawSurface2_GetPixelFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 714 | macro (function-like macro) | IDirectDrawSurface2_GetSurfaceDesc |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 715 | macro (function-like macro) | IDirectDrawSurface2_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 716 | macro (function-like macro) | IDirectDrawSurface2_IsLost |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 717 | macro (function-like macro) | IDirectDrawSurface2_Lock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 718 | macro (function-like macro) | IDirectDrawSurface2_ReleaseDC |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 719 | macro (function-like macro) | IDirectDrawSurface2_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 720 | macro (function-like macro) | IDirectDrawSurface2_SetClipper |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 721 | macro (function-like macro) | IDirectDrawSurface2_SetColorKey |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 722 | macro (function-like macro) | IDirectDrawSurface2_SetOverlayPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 723 | macro (function-like macro) | IDirectDrawSurface2_SetPalette |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 724 | macro (function-like macro) | IDirectDrawSurface2_Unlock |  | (p,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 725 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlay |  | (p,a,b,c,d,e) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 726 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlayDisplay |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 727 | macro (function-like macro) | IDirectDrawSurface2_UpdateOverlayZOrder |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 728 | macro (function-like macro) | IDirectDrawSurface2_GetDDInterface |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 729 | macro (function-like macro) | IDirectDrawSurface2_PageLock |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DDRAW.H | 730 | macro (function-like macro) | IDirectDrawSurface2_PageUnlock |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DELAY.CPP | 35 | function (callable) | Delay |  | (int duration) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DELAY.CPP | 54 | function (callable) | Vsync |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DIPTHONG.CPP | 72 | function (callable) | Fixup_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DIPTHONG.CPP | 114 | function (callable) | Dip_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DIPTHONG.CPP | 200 | function (callable) | UnDip_Text |  | (char const * source,char * dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DIPTHONG.CPP | 315 | function (callable) | Extract_String |  | (void const * data,int string) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DPLAY.H | 33 | macro (function-like macro) | MAKE_DPHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DPLAY.H | 136 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectPlay,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DRAWRECT.CPP | 60 | function (callable) | Draw_Rect | GraphicViewPortClass | (int x1_pixel,int y1_pixel,int x2_pixel,int y2_pixel,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 34 | macro (function-like macro) | MAKE_DSHRESULT |  | (code) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 140 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectSound,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 159 | macro (function-like macro) | IDirectSound_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 160 | macro (function-like macro) | IDirectSound_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 161 | macro (function-like macro) | IDirectSound_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 162 | macro (function-like macro) | IDirectSound_CreateSoundBuffer |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 163 | macro (function-like macro) | IDirectSound_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 164 | macro (function-like macro) | IDirectSound_DuplicateSoundBuffer |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 165 | macro (function-like macro) | IDirectSound_SetCooperativeLevel |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 166 | macro (function-like macro) | IDirectSound_Compact |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 167 | macro (function-like macro) | IDirectSound_GetSpeakerConfig |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 168 | macro (function-like macro) | IDirectSound_SetSpeakerConfig |  | (p,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 169 | macro (function-like macro) | IDirectSound_Initialize |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 180 | function (callable) | DECLARE_INTERFACE_ |  | (IDirectSoundBuffer,IUnknown) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 209 | macro (function-like macro) | IDirectSoundBuffer_QueryInterface |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 210 | macro (function-like macro) | IDirectSoundBuffer_AddRef |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 211 | macro (function-like macro) | IDirectSoundBuffer_Release |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 212 | macro (function-like macro) | IDirectSoundBuffer_GetCaps |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 213 | macro (function-like macro) | IDirectSoundBuffer_GetCurrentPosition |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 214 | macro (function-like macro) | IDirectSoundBuffer_GetFormat |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 215 | macro (function-like macro) | IDirectSoundBuffer_GetVolume |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 216 | macro (function-like macro) | IDirectSoundBuffer_GetPan |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 217 | macro (function-like macro) | IDirectSoundBuffer_GetFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 218 | macro (function-like macro) | IDirectSoundBuffer_GetStatus |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 219 | macro (function-like macro) | IDirectSoundBuffer_Initialize |  | (p,a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 220 | macro (function-like macro) | IDirectSoundBuffer_Lock |  | (p,a,b,c,d,e,f,g) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 221 | macro (function-like macro) | IDirectSoundBuffer_Play |  | (p,a,b,c) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 222 | macro (function-like macro) | IDirectSoundBuffer_SetCurrentPosition |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 223 | macro (function-like macro) | IDirectSoundBuffer_SetFormat |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 224 | macro (function-like macro) | IDirectSoundBuffer_SetVolume |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 225 | macro (function-like macro) | IDirectSoundBuffer_SetPan |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 226 | macro (function-like macro) | IDirectSoundBuffer_SetFrequency |  | (p,a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 227 | macro (function-like macro) | IDirectSoundBuffer_Stop |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 228 | macro (function-like macro) | IDirectSoundBuffer_Unlock |  | (p,a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DSOUND.H | 229 | macro (function-like macro) | IDirectSoundBuffer_Restore |  | (p) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 38 | function (callable) | IconCacheClass | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 47 | function (callable) | ~IconCacheClass | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 63 | function (callable) | Invalidate_Cached_Icons |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 64 | function (callable) | Restore_Cached_Icons |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 65 | function (callable) | Register_Icon_Set |  | (void * icon_data,BOOL pre_cache) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 70 | function (callable) | Clear_Icon_Pointers |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 71 | function (callable) | Cache_Copy_Icon |  | (void const * icon_ptr,void *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 72 | function (callable) | Is_Icon_Cached |  | (void const * icon_data,int icon) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 73 | function (callable) | Get_Icon_Index |  | (void * icon_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 74 | function (callable) | Get_Free_Index |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 75 | function (callable) | Cache_New_Icon |  | (int icon_index,void * icon_ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 76 | function (callable) | Get_Free_Cache_Slot |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 78 | function (callable) | Draw_It | IconCacheClass | (LPDIRECTDRAWSURFACE dest_surface,int x_pixel,int y_pixel,int window_left,int window_top,int window_width,int window_height) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 86 | function (callable) | Set_Font_Palette_Range |  | (void const * palette,INT start_idx,INT end_idx) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 112 | function (callable) | Buffer_Draw_Line |  | (void * this_object,int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 1061 | function (callable) | Buffer_Fill_Rect |  | (void * thisptr,int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 1308 | function (callable) | Buffer_Clear |  | (void * this_object,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 1395 | function (callable) | Linear_Blit_To_Linear |  | (void * this_object,void * dest,int x_pixel,int y_pixel,int dest_x0,int dest_y0,int pixel_width,int pixel_height,BOOL trans) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 1877 | function (callable) | Linear_Scale_To_Linear |  | (void * this_object,void * dest,int src_x,int src_y,int dst_x,int dst_y,int src_width,int src_height,int dst_width,int dst_height,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 2688 | function (callable) | Init_Stamps |  | (unsigned int icondata) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 2760 | function (callable) | Buffer_Draw_Stamp |  | (void const * this_object,void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 2963 | function (callable) | Buffer_Draw_Stamp_Clip |  | (void const * this_object,void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap,int min_x,int min_y,int max_x,int max_y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 3293 | function (callable) | Buffer_Remap |  | (void * this_object,int sx,int sy,int width,int height,void * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 3526 | function (callable) | Apply_XOR_Delta |  | (char * target,char * delta) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 3662 | function (callable) | Apply_XOR_Delta_To_Page_Or_Viewport |  | (void * target,void * delta,int width,int nextrow,int copy) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 3728 | function (callable) | XOR_Delta_Buffer |  | (int nextrow) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 3879 | function (callable) | Copy_Delta_Buffer |  | (int nextrow) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4072 | function (callable) | Build_Fading_Table |  | (void const * palette,void const * dest,long int color,long int frac) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4344 | function (callable) | Set_Palette_Range |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4391 | function (callable) | Bump_Color |  | (void * pal,int color,int desired) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4506 | function (callable) | Buffer_Put_Pixel |  | (void * this_object,int x_pixel,int y_pixel,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4615 | function (callable) | Clip_Rect |  | (int * x,int * y,int * w,int * h,int width,int height) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4768 | function (callable) | Confine_Rect |  | (int * x,int * y,int w,int h,int width,int height) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 4882 | function (callable) | LCW_Uncompress |  | (void * source,void * dest,unsigned long length_) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 5192 | function (callable) | Buffer_To_Page |  | (int x_pixel,int y_pixel,int pixel_width,int pixel_height,void * src,void * dest) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/DrawMisc.cpp | 5465 | function (callable) | Buffer_Get_Pixel |  | (void * this_object,int x_pixel,int y_pixel) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/FONT.CPP | 60 | function (callable) | Char_Pixel_Width |  | (char chr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/FONT.CPP | 87 | function (callable) | String_Pixel_Width |  | (char const * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/FONT.CPP | 126 | function (callable) | Get_Next_Text_Print_XY |  | (GraphicViewPortClass & gp,unsigned long offset,INT * x,INT * y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 81 | function (callable) | GraphicViewPortClass | GraphicViewPortClass | (GraphicBufferClass * gbuffer,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 98 | function (callable) | GraphicViewPortClass | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 112 | function (callable) | ~GraphicViewPortClass | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 140 | function (callable) | Attach | GraphicViewPortClass | (GraphicBufferClass * gbuffer,int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 208 | function (callable) | Change | GraphicViewPortClass | (int x,int y,int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 238 | function (callable) | DD_Init | GraphicBufferClass | (GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 282 | function (callable) | Attach_DD_Surface | GraphicBufferClass | (GraphicBufferClass * attach_buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 304 | function (callable) | Init | GraphicBufferClass | (int w,int h,void * buffer,long size,GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 353 | function (callable) | Un_Init | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 387 | function (callable) | GraphicBufferClass | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 410 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,void * buffer,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 426 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,void * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 446 | function (callable) | GraphicBufferClass | GraphicBufferClass | (int w,int h,GBC_Enum flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 461 | function (callable) | ~GraphicBufferClass | GraphicBufferClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 484 | function (callable) | Set_Logic_Page |  | (GraphicViewPortClass * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 503 | function (callable) | Set_Logic_Page |  | (GraphicViewPortClass & ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 528 | function (callable) | Lock | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 616 | function (callable) | Unlock | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.CPP | 669 | function (callable) | DD_Linear_Blit_To_Linear | GraphicViewPortClass | (GraphicViewPortClass & dest,int source_x,int source_y,int dest_x,int dest_y,int width,int height,BOOL mask) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 151 | function (callable) | BitmapClass | BitmapClass | (int w,int h,unsigned char * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 162 | function (callable) | TPoint2D | TPoint2D | (int xx,int yy) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 163 | function (callable) | TPoint2D | TPoint2D | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 386 | function (callable) | Get_LockCount | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 407 | function (callable) | Get_IsDirectDraw | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 428 | function (callable) | Get_DD_Surface | GraphicBufferClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 451 | function (callable) | Lock | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 477 | function (callable) | Unlock | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 498 | function (callable) | Get_Offset | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 513 | function (callable) | Get_Height | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 528 | function (callable) | Get_Width | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 544 | function (callable) | Get_XAdd | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 558 | function (callable) | Get_XPos | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 576 | function (callable) | Get_YPos | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 591 | function (callable) | Get_Graphic_Buffer | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 608 | function (callable) | Size_Of_Region | GraphicViewPortClass | (int w,int h) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 626 | function (callable) | Put_Pixel | GraphicViewPortClass | (int x,int y,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 649 | function (callable) | Get_Pixel | GraphicViewPortClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 673 | function (callable) | Clear | GraphicViewPortClass | (unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 694 | function (callable) | To_Buffer | GraphicViewPortClass | (int x,int y,int w,int h,void * buff,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 716 | function (callable) | To_Buffer | GraphicViewPortClass | (int x,int y,int w,int h,BufferClass * buff) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 738 | function (callable) | To_Buffer | GraphicViewPortClass | (BufferClass * buff) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 760 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,int x_pixel,int y_pixel,int dx_pixel,int dy_pixel,int pixel_width,int pixel_height,BOOL trans) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 797 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,int dx,int dy,BOOL trans) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 835 | function (callable) | Blit | GraphicViewPortClass | (GraphicViewPortClass & dest,BOOL trans) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 876 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,int src_x,int src_y,int dst_x,int dst_y,int src_w,int src_h,int dst_w,int dst_h,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 902 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,int src_x,int src_y,int dst_x,int dst_y,int src_w,int src_h,int dst_w,int dst_h,char * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 928 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,BOOL trans,char * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 953 | function (callable) | Scale | GraphicViewPortClass | (GraphicViewPortClass & dest,char * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 977 | function (callable) | Print | GraphicViewPortClass | (char const * str,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 998 | function (callable) | Print | GraphicViewPortClass | (int num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1021 | function (callable) | Print | GraphicViewPortClass | (short num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1044 | function (callable) | Print | GraphicViewPortClass | (long num,int x,int y,int fcol,int bcol) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1067 | function (callable) | Draw_Stamp | GraphicViewPortClass | (void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1091 | function (callable) | Draw_Stamp | GraphicViewPortClass | (void const * icondata,int icon,int x_pixel,int y_pixel,void const * remap,int clip_window) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1139 | function (callable) | Draw_Line | GraphicViewPortClass | (int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1159 | function (callable) | Fill_Rect | GraphicViewPortClass | (int sx,int sy,int dx,int dy,unsigned char color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1223 | function (callable) | Remap | GraphicViewPortClass | (int sx,int sy,int width,int height,VOID * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1232 | function (callable) | Fill_Quad | GraphicViewPortClass | (VOID * span_buff,int x0,int y0,int x1,int y1,int x2,int y2,int x3,int y3,int color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1251 | function (callable) | Remap | GraphicViewPortClass | (VOID * remap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1259 | function (callable) | Get_Pitch | GraphicViewPortClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1281 | function (callable) | Buffer_To_Page |  | (int x,int y,int w,int h,void * Buffer,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1306 | function (callable) | To_Page | BufferClass | (int w,int h,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1329 | function (callable) | To_Page | BufferClass | (GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GBUFFER.H | 1352 | function (callable) | To_Page | BufferClass | (int x,int y,int w,int h,GraphicViewPortClass & view) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 69 | function (callable) | Get_Shape_Size |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 104 | function (callable) | Get_Shape_Uncomp_Size |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 137 | function (callable) | Get_Shape_Data |  | (VOID const * shape,WORD data) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 169 | function (callable) | Extract_Shape_Count |  | (VOID const * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 196 | function (callable) | Extract_Shape |  | (VOID const * buffer,int shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 233 | function (callable) | Get_Shape_Width |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 259 | function (callable) | Get_Shape_Height |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 290 | function (callable) | Set_Shape_Height |  | (VOID const * shape,WORD newheight) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 319 | function (callable) | Restore_Shape_Height |  | (VOID * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/GETSHAPE.CPP | 349 | function (callable) | Get_Shape_Original_Height |  | (VOID const * shape) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONCACH.H | 140 | function (callable) | Get_Is_Cached | IconCacheClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 112 | function (callable) | Load_Icon_Set |  | (char const * filename,void * iconsetptr,long buffsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 247 | function (callable) | Free_Icon_Set |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 260 | function (callable) | Get_Icon_Set_Size |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 273 | function (callable) | Get_Icon_Set_Width |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 286 | function (callable) | Get_Icon_Set_Height |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 299 | function (callable) | Get_Icon_Set_Icondata |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 308 | function (callable) | Get_Icon_Set_Trans |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 321 | function (callable) | Get_Icon_Set_Count |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/ICONSET.CPP | 334 | function (callable) | Get_Icon_Set_Map |  | (void const * iconset) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.CPP | 71 | function (callable) | Open_Iff_File |  | (char const * filename) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.CPP | 125 | function (callable) | Close_Iff_File |  | (int fh) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.CPP | 144 | function (callable) | Get_Iff_Chunk_Size |  | (int fh,long id) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.CPP | 212 | function (callable) | Read_Iff_Chunk |  | (int fh,long id,void * buffer,unsigned long maxsize) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.CPP | 272 | function (callable) | Write_Iff_Chunk |  | (int file,long id,void * buffer,long length) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.H | 55 | macro (function-like macro) | MAKE_ID |  | (a,b,c,d) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.H | 56 | macro (function-like macro) | IFFize_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IFF.H | 57 | macro (function-like macro) | IFFize_LONG |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IRANDOM.CPP | 51 | function (callable) | IRandom |  | (int minval,int maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IRANDOM.CPP | 70 | function (callable) | Random |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/IRANDOM.CPP | 91 | function (callable) | Get_Random_Mask |  | (int maxval) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 43 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 65 | label (assembly label candidate) | LCW_Compress |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 115 | label (assembly label candidate) | _loop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 119 | label (assembly label candidate) | searchloop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 152 | label (assembly label candidate) | notlongenough |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 154 | label (assembly label candidate) | notrunlength |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 156 | label (assembly label candidate) | oploop |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 183 | label (assembly label candidate) | notend |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 198 | label (assembly label candidate) | searchdone |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 213 | label (assembly label candidate) | shortrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 222 | label (assembly label candidate) | medrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 232 | label (assembly label candidate) | lenin |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 236 | label (assembly label candidate) | lenin1 |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 241 | label (assembly label candidate) | len |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 246 | label (assembly label candidate) | stolen |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 253 | label (assembly label candidate) | longrun |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 259 | label (assembly label candidate) | medrunnxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 262 | label (assembly label candidate) | srunnxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 270 | label (assembly label candidate) | nxt |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWCOMP.ASM | 276 | label (assembly label candidate) | _out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWUNCMP.ASM | 42 | label (assembly label candidate) | externdef |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LCWUNCMP.ASM | 57 | label (assembly label candidate) | LCW_Uncompress |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 74 | function (callable) | Load_Data |  | (char const * name,void * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 104 | function (callable) | Write_Data |  | (char const * name,void * ptr,unsigned long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 136 | function (callable) | Load_Alloc_Data |  | (char const * name,MemoryFlagType flags) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 169 | function (callable) | Load_Uncompress |  | (char const * file,BufferClass & uncomp_buff,BufferClass & dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 244 | function (callable) | Load_Uncompress |  | (char const * file,BuffType uncomp_buff,BuffType dest_buff,void * reserved_data) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOAD.CPP | 330 | function (callable) | Uncompress_Data |  | (void const * src,void * dst) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/LOADFONT.CPP | 82 | function (callable) | Load_Font |  | (char const * name) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MEMFLAG.H | 74 | function (callable) | operator new |  | (size_t size,MemoryFlagType flag) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MEMFLAG.H | 78 | function (callable) | operator new[]  |  | (size_t size,MemoryFlagType flag) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MEMFLAG.H | 94 | function (callable) | Add_Long_To_Pointer |  | (void const * ptr,long size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 35 | function (callable) | Get_Modem_Name | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 37 | function (callable) | Get_Modem_Device_Name | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 39 | function (callable) | Get_Modem_Error_Correction_Enable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 41 | function (callable) | Get_Modem_Error_Correction_Disable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 43 | function (callable) | Get_Modem_Compression_Enable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 45 | function (callable) | Get_Modem_Compression_Disable | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 47 | function (callable) | Get_Modem_Hardware_Flow_Control | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MODEMREG.H | 49 | function (callable) | Get_Modem_No_Flow_Control | ModemRegistryEntryClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 101 | function (callable) | Enable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 102 | function (callable) | Disable | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 103 | function (callable) | Is_Enabled | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 104 | function (callable) | Get_Current | MonoClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 107 | function (callable) | Set_Default_Attribute | MonoClass | (char attrib) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 117 | function (callable) | Get_X | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 118 | function (callable) | Get_Y | MonoClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 141 | function (callable) | Offset | MonoClass | (int x=0,int y=0) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MONO.H | 143 | function (callable) | Store_Cell | MonoClass | (CellType & cell,int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 51 | function (callable) | WWMouseClass | WWMouseClass | (GraphicViewPortClass * scr,int mouse_max_width,int mouse_max_height) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 102 | function (callable) | ~WWMouseClass | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 122 | function (callable) | Block_Mouse |  | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 130 | function (callable) | Unblock_Mouse |  | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 139 | function (callable) | Block_Mouse | WWMouseClass | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 147 | function (callable) | Unblock_Mouse | WWMouseClass | (GraphicBufferClass * buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 158 | function (callable) | Set_Cursor_Clip | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 176 | function (callable) | Clear_Cursor_Clip | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 185 | function (callable) | Process_Mouse | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 261 | function (callable) | Set_Cursor | WWMouseClass | (int xhotspot,int yhotspot,void * cursor) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 308 | function (callable) | Low_Hide_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 325 | function (callable) | Hide_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 333 | function (callable) | Low_Show_Mouse | WWMouseClass | (int x,int y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 378 | function (callable) | Show_Mouse | WWMouseClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 388 | function (callable) | Conditional_Hide_Mouse | WWMouseClass | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 450 | function (callable) | Conditional_Show_Mouse | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 475 | function (callable) | Draw_Mouse | WWMouseClass | (GraphicViewPortClass * scr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 534 | function (callable) | Erase_Mouse | WWMouseClass | (GraphicViewPortClass * scr,int forced) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 590 | function (callable) | Get_Mouse_State | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 604 | function (callable) | Get_Mouse_X | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 622 | function (callable) | Get_Mouse_Y | WWMouseClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 640 | function (callable) | Get_Mouse_XY | WWMouseClass | (int & x,int & y) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 651 | function (callable) | Process_Mouse |  | (UINT event_id,UINT res1,DWORD user,DWORD res2,DWORD res3) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 663 | function (callable) | Hide_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 669 | function (callable) | Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 675 | function (callable) | Conditional_Hide_Mouse |  | (int x1,int y1,int x2,int y2) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 681 | function (callable) | Conditional_Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 687 | function (callable) | Get_Mouse_State |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 693 | function (callable) | Set_Mouse_Cursor |  | (int hotx,int hoty,void * cursor) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 703 | function (callable) | Get_Mouse_X |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/MOUSEWW.CPP | 712 | function (callable) | Get_Mouse_Y |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PALETTE.CPP | 86 | function (callable) | Set_Palette |  | (void * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PALETTE.CPP | 118 | function (callable) | Set_Palette_Color |  | (void * palette,int color,void * data) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PALETTE.CPP | 157 | function (callable) | Fade_Palette_To |  | (void * palette1,unsigned int delay,void (* callback)()) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PALETTE.CPP | 239 | function (callable) | Determine_Bump_Rate |  | (void * palette,int delay,short * ticks,short * rate) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PALETTE.CPP | 308 | function (callable) | Bump_Palette |  | (void * palette1,unsigned int step) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PLAYCD.H | 200 | function (callable) | Get_Number_Of_Drives | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PLAYCD.H | 220 | function (callable) | Get_Next_CD_Drive | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/PLAYCD.H | 247 | function (callable) | Get_First_CD_Drive | GetCDClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/RAWFILE.H | 207 | function (callable) | File_Name | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/RAWFILE.H | 228 | function (callable) | RawFileClass | RawFileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/RAWFILE.H | 249 | function (callable) | Is_Open | RawFileClass | (void) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SET_FONT.CPP | 59 | function (callable) | Set_Font |  | (void const * fontptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 50 | function (callable) | operator \| |  | (MakeShapeFlags_Type a,MakeShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 53 | function (callable) | operator & |  | (MakeShapeFlags_Type a,MakeShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 56 | function (callable) | operator ~ |  | (MakeShapeFlags_Type a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 85 | function (callable) | operator \| |  | (ShapeFlags_Type a,ShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 88 | function (callable) | operator & |  | (ShapeFlags_Type a,ShapeFlags_Type b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/SHAPE.H | 91 | function (callable) | operator ~ |  | (ShapeFlags_Type a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 63 | function (callable) | TimerClass | TimerClass | (BaseTimerEnum timer,BOOL on) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 91 | function (callable) | Get_Ticks | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 124 | function (callable) | Time | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 149 | function (callable) | Stop | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 170 | function (callable) | Start | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.CPP | 194 | function (callable) | Set | TimerClass | (long value,BOOL start) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 71 | function (callable) | ~TimerClass | TimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 91 | function (callable) | Reset | TimerClass | (BOOL start) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 105 | function (callable) | ~CountDownTimerClass | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 118 | function (callable) | Stop | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 124 | function (callable) | Start | CountDownTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMER.H | 130 | function (callable) | Reset | CountDownTimerClass | (BOOL start) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERDWN.CPP | 63 | function (callable) | CountDownTimerClass | CountDownTimerClass | (BaseTimerEnum timer,long set,int on) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERDWN.CPP | 69 | function (callable) | CountDownTimerClass | CountDownTimerClass | (BaseTimerEnum timer,int on) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERDWN.CPP | 90 | function (callable) | Time | CountDownTimerClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERDWN.CPP | 115 | function (callable) | Set | CountDownTimerClass | (long value,BOOL start) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 98 | function (callable) | WinTimerClass | WinTimerClass | (UINT freq,BOOL partial) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 152 | function (callable) | ~WinTimerClass | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 188 | function (callable) | Timer_Callback |  | (UINT,UINT,DWORD,DWORD,DWORD) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 226 | function (callable) | Update_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 291 | function (callable) | Get_System_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TIMERINI.CPP | 311 | function (callable) | Get_User_Tick_Count | WinTimerClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 45 | label (assembly label candidate) | INCLUDE |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 73 | label (assembly label candidate) | Buffer_To_Buffer |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 149 | label (assembly label candidate) | scr_left_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 157 | label (assembly label candidate) | scr_bottom_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 162 | label (assembly label candidate) | scr_right_ok |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 168 | label (assembly label candidate) | do_blit |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 213 | label (assembly label candidate) | IF |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 216 | label (assembly label candidate) | ENDIF |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 224 | label (assembly label candidate) | forward_loop_dword |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 243 | label (assembly label candidate) | forward_loop_bytes |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 253 | label (assembly label candidate) | IF |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 255 | label (assembly label candidate) | forward_Blit_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 264 | label (assembly label candidate) | forward_loop_trans |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 267 | label (assembly label candidate) | forward_trans_line |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 274 | label (assembly label candidate) | transp_pixel |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 278 | label (assembly label candidate) | transp_reference |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 286 | label (assembly label candidate) | ENDIF |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/TOBUFF.ASM | 288 | label (assembly label candidate) | real_out |  |  |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 309 | function (callable) | Set_Modem_Dial_Type | WinNullModemClass | (int) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 310 | function (callable) | Get_Modem_Status | WinNullModemClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 311 | function (callable) | Set_Serial_DTR | WinNullModemClass | (BOOL) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 312 | function (callable) | Get_Modem_Result | WinNullModemClass | (int,char *,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 313 | function (callable) | Dial_Modem | WinNullModemClass | (char *) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 314 | function (callable) | Send_Command_To_Modem | WinNullModemClass | (char *,char,char *,int,int,int) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 315 | function (callable) | Set_Echo_Function | WinNullModemClass | (void (*)(char)) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINCOMM.H | 316 | function (callable) | Set_Abort_Function | WinNullModemClass | (int (*)(void)) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 120 | function (callable) | Standard_More_Prompt |  | (char const * prompt,int space,int fcolor,int bcolor) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 173 | function (callable) | Set_More_Prompt |  | (char const * prompt,int space,int fcolor,int bcolor) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 206 | function (callable) | Set_More_On |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 228 | function (callable) | Set_More_Off |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 249 | function (callable) | Change_Window |  | (int windnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 291 | function (callable) | Change_New_Window |  | (int windnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 317 | function (callable) | New_Window |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 349 | function (callable) | Window_Int_Print |  | (int num) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 411 | function (callable) | Window_Print |  | (char const string[],...) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 703 | function (callable) | Scroll_Window |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 748 | function (callable) | Flush_Line |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 891 | function (callable) | In_Char |  | (char * str) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINDOWS.CPP | 954 | function (callable) | Fetch_Char |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINHIDE.CPP | 60 | function (callable) | Window_Hide_Mouse |  | (int window) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WINHIDE.CPP | 87 | function (callable) | Window_Show_Mouse |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WRITEPCX.CPP | 60 | function (callable) | Write_PCX_File |  | (char * name,GraphicViewPortClass & pic,unsigned char * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WRITEPCX.CPP | 124 | macro (function-like macro) | WRITE_CHAR |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WRITEPCX.CPP | 132 | function (callable) | Write_Pcx_ScanLine |  | (int file_handle,int scansize,unsigned char * ptr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WSA.H | 118 | function (callable) | Open_Animation |  | (char * file_name,WSAOpenType user_flags,unsigned char * palette=NULL) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WSA.H | 141 | function (callable) | Open_Animation |  | (char * file_name,BufferClass & buffer,WSAOpenType user_flags,unsigned char * palette=NULL) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WWFILE.H | 50 | function (callable) | ~FileClass | FileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/WWFILE.H | 65 | function (callable) | operator char const* | FileClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 114 | macro (function-like macro) | GET_SIZE |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 121 | function (callable) | ABS |  | (T a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 132 | function (callable) | MIN |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 145 | function (callable) | MAX |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 155 | macro (function-like macro) | LOW_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 158 | macro (function-like macro) | HIGH_WORD |  | (a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 161 | macro (function-like macro) | MAKE_LONG |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 171 | macro (function-like macro) | AssembleTo |  | (dest,fmt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 262 | function (callable) | Min |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 267 | function (callable) | Max |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 272 | function (callable) | Abs |  | (T a) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 277 | function (callable) | minimize |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 283 | function (callable) | maximize |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 293 | macro (function-like macro) | Bit_Flags_On |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 294 | macro (function-like macro) | Bit_Flags_Off |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 295 | macro (function-like macro) | Bit_Flags_Value |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 296 | macro (function-like macro) | Bit_Flags_Flip |  | (a,b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 300 | function (callable) | BitFlagsOn |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 305 | function (callable) | BitFlagsOff |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 310 | function (callable) | BitFlagsValue |  | (T a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WIN32LIB/wwstd.h | 315 | function (callable) | BitFlagsFlip |  | (T & a,T b) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 54 | function (callable) | output |  | (short,short) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 80 | function (callable) | Focus_Loss |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 87 | function (callable) | Focus_Restore |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 114 | function (callable) | Check_For_Focus_Loss |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 164 | function (callable) | Windows_Procedure |  | (HWND hwnd,UINT message,UINT wParam,LONG lParam) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 363 | function (callable) | WWDebugString |  | (char * string) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 433 | function (callable) | Create_Main_Window |  | (HANDLE instance,int command_show,int width,int height) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 499 | function (callable) | Window_Dialog_Box |  | (HANDLE hinst,LPCTSTR lpszTemplate,HWND hwndOwner,DLGPROC dlgprc) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 561 | function (callable) | Colour_Debug |  | (int call_number) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 585 | function (callable) | Any_Locked |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 679 | function (callable) | Flag_To_Set_Palette |  | (unsigned char * palette,long numbytes,unsigned long slowpal) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 689 | function (callable) | Check_VQ_Palette_Set |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 701 | function (callable) | SetPalette |  | (unsigned char * palette,long,unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 731 | function (callable) | Assert_Failure |  | (char * expression,int line,char * file) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 797 | function (callable) | Memory_Error_Handler |  | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 823 | function (callable) | Load_Title_Screen |  | (char * name,GraphicViewPortClass * video_page,unsigned char * palette) |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 868 | macro (function-like macro) | READ_CHAR |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WINSTUB.CPP | 875 | function (callable) | Read_PCX_File |  | (char * name,char * palette,void * Buff,long Size) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 41 | function (callable) | WolapiObject | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 142 | function (callable) | ~WolapiObject | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 198 | function (callable) | bSetupCOMStuff | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 268 | function (callable) | UnsetupCOMStuff | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 324 | function (callable) | LinkToChatDlg | WolapiObject | (IconListClass * pILChat,IconListClass * pILChannels,IconListClass * pILUsers,StaticButtonClass * pStaticUsers) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 337 | function (callable) | ClearListPtrs | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 350 | function (callable) | LinkToGameDlg | WolapiObject | (IconListClass * pILDisc,IconListClass * pILPlayers) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 360 | function (callable) | PrepareButtonsAndIcons | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 496 | function (callable) | GetGameTypeInfo | WolapiObject | (int iGameType,WOL_GAMETYPEINFO & GameTypeInfo,HPALETTE hPal) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 544 | function (callable) | IconForGameType | WolapiObject | (int iGameType) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 557 | function (callable) | NameOfGameType | WolapiObject | (int iGameType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 570 | function (callable) | URLForGameType | WolapiObject | (int iGameType) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 582 | function (callable) | PrintMessage | WolapiObject | (const char * szText,PlayerColorType iColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 589 | function (callable) | PrintMessage | WolapiObject | (const char * szText,RemapControlType * pColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 596 | function (callable) | GetChatServer | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 679 | function (callable) | AttemptLogin | WolapiObject | (const char * szName,const char * szPass,bool bPassIsMangled) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 748 | function (callable) | bLoggedIn | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 754 | function (callable) | Logout | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 790 | function (callable) | UpdateChannels | WolapiObject | (int iChannelType,CHANNELFILTER ChannelFilter,bool bAutoping) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 827 | function (callable) | OnChannelList | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 840 | function (callable) | ListChannels | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 981 | function (callable) | ChannelJoin | WolapiObject | (const char * szChannelName,const char * szKey) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 992 | function (callable) | ChannelJoin | WolapiObject | (Channel * pChannelToJoin) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1040 | function (callable) | ChannelLeave | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1124 | function (callable) | ListChannelUsers | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1309 | function (callable) | bItemMarkedAccepted | WolapiObject | (int iIndex) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1319 | function (callable) | MarkItemAccepted | WolapiObject | (int iIndex,bool bAccept) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1330 | function (callable) | bItemMarkedReadyToGo | WolapiObject | (int iIndex) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1342 | function (callable) | MarkItemReadyToGo | WolapiObject | (int iIndex,const char * szReadyState) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1351 | function (callable) | bItemMarkedNeedScenario | WolapiObject | (int iIndex) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1361 | function (callable) | PullPlayerName_Into_From | WolapiObject | (char * szDest,const char * szSource) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1383 | function (callable) | PullPlayerHouse_From | WolapiObject | (const char * szSource) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1423 | function (callable) | WritePlayerListItem | WolapiObject | (char * szDest,const char * szName,HousesType House) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1438 | function (callable) | RequestPlayerPings | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1461 | function (callable) | SendMessage | WolapiObject | (const char * szMessage,IconListClass & ILUsers,bool bAction) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1620 | function (callable) | ChannelCreate | WolapiObject | (const char * szChannelName,const char * szKey,bool bGame,int iMaxPlayers,bool bTournament,int iLobby,CREATEGAMEINFO::GAMEKIND GameKind) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1697 | function (callable) | DoFindPage | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1807 | function (callable) | Locate | WolapiObject | (const char * szUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1845 | function (callable) | Page | WolapiObject | (const char * szUser,const char * szSend,bool bWaitForResult) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1888 | function (callable) | DoKick | WolapiObject | (IconListClass * pILUsersOrPlayers,bool bAndBan) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1930 | function (callable) | Kick | WolapiObject | (User * pUserToKick) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1943 | function (callable) | Ban | WolapiObject | (User * pUserToKick) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1956 | function (callable) | DoSquelch | WolapiObject | (IconListClass * pILUsersOrPlayers) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 1993 | function (callable) | Squelch | WolapiObject | (User * pUserToSquelch) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2011 | function (callable) | DoOptions | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2022 | function (callable) | DoLadder | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2047 | function (callable) | DoHelp | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2064 | function (callable) | DoWebRegistration | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2096 | function (callable) | DoGameAdvertising | WolapiObject | (const Channel * pChannel) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2119 | function (callable) | SpawnBrowser | WolapiObject | (const char * szURL) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2209 | function (callable) | ChannelListTitle | WolapiObject | (const char * szTitle) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2216 | function (callable) | EnterLevel_Top | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2244 | function (callable) | EnterLevel_OfficialChat | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2274 | function (callable) | EnterLevel_UserChat | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2304 | function (callable) | EnterLevel_Games | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2363 | function (callable) | EnterLevel_GamesOfType | WolapiObject | (WOL_GAMETYPEINFO * pGameTypeInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2392 | function (callable) | EnterLevel_Lobbies | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2421 | function (callable) | OnEnteringChatChannel | WolapiObject | (const char * szChannelName,bool bICreatedChannel,int iLobby) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2520 | function (callable) | OnExitingChatChannel | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2555 | function (callable) | ExitChatChannelForGameChannel | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2571 | function (callable) | OnEnteringGameChannel | WolapiObject | (const char * szChannelName,bool bICreatedChannel,const CREATEGAMEINFO & CreateGameInfo) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2640 | function (callable) | OnEnteringGameSetup | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2680 | function (callable) | OnFailedToEnterGameChannel | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2707 | function (callable) | OnExitingGameChannel | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2728 | function (callable) | RejoinLobbyAfterGame | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2761 | function (callable) | RequestLadders | WolapiObject | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2829 | function (callable) | RequestIPs | WolapiObject | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2871 | function (callable) | SaveChat | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2899 | function (callable) | RestoreChat | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2911 | function (callable) | AddHostLeftMessageToSavedChat | WolapiObject | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2926 | function (callable) | AddMessageToSavedChat | WolapiObject | (const char * szMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2941 | function (callable) | DeleteSavedChat | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2954 | function (callable) | GenericErrorMessage | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 2963 | function (callable) | GetNameOfBeginningLobby | WolapiObject | (char * szNameToSet) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3000 | function (callable) | GetLobbyChannels | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3034 | function (callable) | pGameHostName | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3051 | function (callable) | pGameHost | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3068 | function (callable) | SendGameOpt | WolapiObject | (const char * szSend,User * pUserPriv) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3094 | function (callable) | RequestGameStart | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3136 | function (callable) | SendGo | WolapiObject | (const char * szSend) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3162 | function (callable) | Init_DisconnectPinging | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3179 | function (callable) | Pump_DisconnectPinging | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3247 | function (callable) | DisconnectPingResultsString | WolapiObject | (char * szStringToSet) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3261 | function (callable) | SetOptionDefaults | WolapiObject | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3293 | function (callable) | SetOptions | WolapiObject | (bool bEnableFind,bool bEnablePage,bool bLangFilterOn,bool bShowAllGames) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3321 | function (callable) | GetCurrentScreenPalette |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3363 | function (callable) | RemapDIBToPalette |  | (HPALETTE hPal,const char * pDIB) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLAPIOB.CPP | 3425 | function (callable) | HostNameFromGameChannelName |  | (char * szNameToSet,const char * szChannelName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLDEBUG.H | 30 | macro (function-like macro) | _ASSERTE |  | (x) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLEDIT.CPP | 32 | function (callable) | Draw_Text | WOLEditClass | (char const * text) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLEDIT.CPP | 58 | function (callable) | Action | WOLEditClass | (unsigned flags,KeyNumType & key) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOLEDIT.H | 30 | function (callable) | WOLEditClass | WOLEditClass | (int id,char * text,int max_len,TextPrintType flags,int x,int y,int w,int h,EditStyle style) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CGAM.CPP | 37 | function (callable) | WOL_CreateGame_Dialog |  | (WolapiObject * pWO) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CGAM.CPP | 363 | function (callable) | SetPlayerCountList |  | (IconListClass & PlayerCountList,int iPlayerMax,char * pShpBoxCheck,char * pShpBoxEmpty) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 70 | function (callable) | WOL_Chat_Dialog |  | (WolapiObject * pWO) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1015 | function (callable) | WOL_PrintMessage |  | (IconListClass & ILTarget,const char * szText,PlayerColorType iColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1022 | function (callable) | WOL_PrintMessage |  | (IconListClass & ILTarget,const char * szText,RemapControlType * pColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1030 | function (callable) | OnExpandChannelList |  | (IconListClass & chanlist,IconListClass & userlist) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1054 | function (callable) | OnExpandUserList |  | (IconListClass & chanlist,IconListClass & userlist) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1078 | function (callable) | ResizeChannelList |  | (IconListClass & chanlist,bool bExpand) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1088 | function (callable) | ResizeUserList |  | (IconListClass & userlist,bool bExpand) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1098 | function (callable) | EnterChannel |  | (WolapiObject * pWO,IconListClass & chatlist,IconListClass & chanlist,int iIndex,bool bGame) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1110 | function (callable) | EnterChannel |  | (WolapiObject * pWO,IconListClass & chatlist,Channel * pChannel,char * szChannelName,bool bGame) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1281 | function (callable) | ExitChatChannel |  | (WolapiObject * pWO) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1299 | function (callable) | CreateChatChannel |  | (WolapiObject * pWO) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1340 | function (callable) | CreateGameChannel |  | (WolapiObject * pWO,const CREATEGAMEINFO & cgi) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1374 | function (callable) | ProcessChannelListSelection |  | (WolapiObject * pWO,IconListClass & chatlist,IconListClass & chanlist,int iIndex) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_CHAT.CPP | 1566 | function (callable) | bLinkInList |  | (const LinkClass * pListHead,const LinkClass * pLinkToFind) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_DNLD.CPP | 26 | function (callable) | WOL_Download_Dialog |  | (IDownload * pDownload,RADownloadEventSink * pDownloadSink,const char * szTitle) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 108 | function (callable) | WOL_GameSetupDialog | WOL_GameSetupDialog | (WolapiObject * pWO,bool bHost) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 157 | function (callable) | ~WOL_GameSetupDialog | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 189 | function (callable) | Run | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 196 | function (callable) | Initialize | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 449 | function (callable) | Show | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1576 | function (callable) | SetSpecialControlStates | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1619 | function (callable) | BindControls | WOL_GameSetupDialog | (bool bBind) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1777 | function (callable) | ScenarioDisplayMode | WOL_GameSetupDialog | (SCENARIO_GAMEKIND ScenKind) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1829 | function (callable) | ExitGameChannel | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1849 | function (callable) | DrawScenarioDescripIcon | WOL_GameSetupDialog | (const char * pDIB) const |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1855 | function (callable) | SetPlayerColor | WOL_GameSetupDialog | (const char * szName,PlayerColorType Color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1883 | function (callable) | GetPlayerColor | WOL_GameSetupDialog | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1897 | function (callable) | SetPlayerHouse | WOL_GameSetupDialog | (const char * szName,HousesType House) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1922 | function (callable) | GetPlayerHouse | WOL_GameSetupDialog | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1936 | function (callable) | SetPlayerAccepted | WOL_GameSetupDialog | (const char * szName,bool bAccepted) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1952 | function (callable) | IveAccepted | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1962 | function (callable) | SetPlayerReadyToGo | WOL_GameSetupDialog | (const char * szName,const char * szReadyState) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1982 | function (callable) | ResetReadyToGo | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 1992 | function (callable) | bPlayerReadyToGo | WOL_GameSetupDialog | (const char * szName) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2002 | function (callable) | bAllPlayersReadyToGo | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2020 | function (callable) | ProcessGuestRequest | WOL_GameSetupDialog | (User * pUser,const char * szRequest) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2124 | function (callable) | ProcessInform | WOL_GameSetupDialog | (char * szInform) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2257 | function (callable) | bParamsUnfresh | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2277 | function (callable) | SendParams | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2353 | function (callable) | AcceptParams | WOL_GameSetupDialog | (char * szParams) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2515 | function (callable) | SetGParamsToCurrent | WOL_GameSetupDialog | (GAMEPARAMS & GParams) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2543 | function (callable) | operator == |  | (const GAMEPARAMS & gp1,const GAMEPARAMS & gp2) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2550 | function (callable) | operator == |  | (const GlobalPacketType & gp1,const GlobalPacketType & gp2) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2644 | function (callable) | PlayerColorTypeOf |  | (RemapControlType * pColorRemap) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2655 | function (callable) | RequestPlayerColor | WOL_GameSetupDialog | (PlayerColorType Color) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2672 | function (callable) | InformAboutPlayerColor | WOL_GameSetupDialog | (const char * szName,PlayerColorType Color,User * pUserPriv) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2697 | function (callable) | InformAboutPlayerHouse | WOL_GameSetupDialog | (const char * szName,HousesType House,User * pUserPriv) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2723 | function (callable) | InformAboutPlayerAccept | WOL_GameSetupDialog | (const char * szName,User * pUserPriv) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2734 | function (callable) | InformAboutStart | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2747 | function (callable) | InformAboutCancelStart | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2762 | function (callable) | OnGuestJoin | WOL_GameSetupDialog | (User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2862 | function (callable) | AcceptNewGuestPlayerInfo | WOL_GameSetupDialog | (char * szMsg) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2933 | function (callable) | OnGuestLeave | WOL_GameSetupDialog | (User * pUser) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2948 | function (callable) | ClearAllAccepts | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2986 | function (callable) | bAllGuestsAccept | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 2997 | function (callable) | ColorNextAvailable | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3015 | function (callable) | GuestIsReadyToPlay | WOL_GameSetupDialog | (const char * szName,const char * szReadyState) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3029 | function (callable) | bNeedScenarioDownload | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3068 | function (callable) | HostSaysGo | WOL_GameSetupDialog | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3125 | function (callable) | TriggerGameStart | WOL_GameSetupDialog | (char * szGoMessage) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3473 | function (callable) | bSpecialAftermathScenario |  | (const char * szScenarioDescription) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_GSUP.CPP | 3489 | function (callable) | ScenarioIndex_From_Filename |  | (const char * szScenarioFilename) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_LOGN.CPP | 41 | function (callable) | WOL_Login_Dialog |  | (WolapiObject * pWO) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_LOGN.CPP | 501 | function (callable) | ReadSavedNicks |  | (WolapiObject * pWO,IconListClass & NickList,char * szNameBuffer,char * szPassBuffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_LOGN.CPP | 533 | function (callable) | bSaveNick |  | (WolapiObject * pWO,const char * szNickToSave,const char * szPassToSave,bool bPassIsMangled) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_LOGN.CPP | 611 | function (callable) | DeleteNick |  | (WolapiObject * pWO,int iOneBasedEntryToDelete) |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_MAIN.CPP | 45 | function (callable) | WOL_Main |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_MAIN.CPP | 194 | function (callable) | ReregisterWolapiDLL |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_MAIN.CPP | 256 | function (callable) | HandleDLLFail |  | () |  | SIGNATURE_CATALOGUED |
| REDALERT/WOL_OPT.CPP | 32 | function (callable) | WOL_Options_Dialog |  | (WolapiObject * pWO,bool bCalledFromGame) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.CPP | 61 | function (callable) | IPXInterfaceClass | IPXInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.CPP | 87 | function (callable) | Get_Network_Card_Address | IPXInterfaceClass | (int card_number,SOCKADDR_IPX * addr) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.CPP | 169 | function (callable) | Open_Socket | IPXInterfaceClass | (SOCKET socketnum) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.CPP | 299 | function (callable) | Message_Handler | IPXInterfaceClass | (HWND,UINT message,UINT,LONG lParam) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.H | 65 | function (callable) | Get_Protocol | IPXInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPIPX.H | 69 | function (callable) | Protocol_Event_Message | IPXInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 75 | function (callable) | WinsockInterfaceClass | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 97 | function (callable) | ~WinsockInterfaceClass | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 117 | function (callable) | Close | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 158 | function (callable) | Close_Socket | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 182 | function (callable) | Start_Listening | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 212 | function (callable) | Stop_Listening | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 237 | function (callable) | Discard_In_Buffers | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 263 | function (callable) | Discard_Out_Buffers | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 289 | function (callable) | Init | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 368 | function (callable) | Read | WinsockInterfaceClass | (void * buffer,int & buffer_len,void * address,int & address_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 429 | function (callable) | WriteTo | WinsockInterfaceClass | (void * buffer,int buffer_len,void * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 479 | function (callable) | Broadcast | WinsockInterfaceClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 531 | function (callable) | Clear_Socket_Error | WinsockInterfaceClass | (SOCKET socket) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.CPP | 561 | function (callable) | Set_Socket_Options | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 103 | function (callable) | Set_Broadcast_Address | WinsockInterfaceClass | (void *) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 105 | function (callable) | Get_Protocol | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 109 | function (callable) | Protocol_Event_Message | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 113 | function (callable) | Open_Socket | WinsockInterfaceClass | (SOCKET) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 117 | function (callable) | Message_Handler | WinsockInterfaceClass | (HWND,UINT,UINT,LONG) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPROTO.H | 135 | function (callable) | Get_Connection_Status | WinsockInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 67 | function (callable) | UDPInterfaceClass | UDPInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 86 | function (callable) | ~UDPInterfaceClass | UDPInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 116 | function (callable) | Set_Broadcast_Address | UDPInterfaceClass | (void * address) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 143 | function (callable) | Open_Socket | UDPInterfaceClass | (SOCKET) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 246 | function (callable) | Broadcast | UDPInterfaceClass | (void * buffer,int buffer_len) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.CPP | 307 | function (callable) | Message_Handler | UDPInterfaceClass | (HWND,UINT message,UINT,LONG lParam) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.H | 59 | function (callable) | Get_Protocol | UDPInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WSPUDP.H | 63 | function (callable) | Protocol_Event_Message | UDPInterfaceClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 38 | macro (function-like macro) | YEAR |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 39 | macro (function-like macro) | MONTH |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 40 | macro (function-like macro) | DAY |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 41 | macro (function-like macro) | HOUR |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 42 | macro (function-like macro) | MINUTE |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 43 | macro (function-like macro) | SECOND |  | (dt) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 72 | function (callable) | ~FileClass | FileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 86 | function (callable) | Get_Date_Time | FileClass | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 87 | function (callable) | Set_Date_Time | FileClass | (unsigned long) |  | SIGNATURE_CATALOGUED |
| REDALERT/WWFILE.H | 90 | function (callable) | operator char const* | FileClass | () |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.CPP | 67 | function (callable) | Put | BufferPipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.CPP | 95 | function (callable) | ~FilePipe | FilePipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.CPP | 124 | function (callable) | End | FilePipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.CPP | 152 | function (callable) | Put | FilePipe | (void const * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 50 | function (callable) | BufferPipe | BufferPipe | (Buffer const & buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 51 | function (callable) | BufferPipe | BufferPipe | (void * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 61 | function (callable) | Is_Valid | BufferPipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 75 | function (callable) | FilePipe | FilePipe | (FileClass * file) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 76 | function (callable) | FilePipe | FilePipe | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/XPIPE.H | 86 | function (callable) | Valid_File | FilePipe | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.CPP | 65 | function (callable) | Get | BufferStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.CPP | 112 | function (callable) | Get | FileStraw | (void * source,int slen) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.CPP | 141 | function (callable) | ~FileStraw | FileStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 51 | function (callable) | BufferStraw | BufferStraw | (Buffer const & buffer) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 52 | function (callable) | BufferStraw | BufferStraw | (void const * buffer,int length) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 61 | function (callable) | Is_Valid | BufferStraw | (void) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 73 | function (callable) | FileStraw | FileStraw | (FileClass * file) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 74 | function (callable) | FileStraw | FileStraw | (FileClass & file) |  | SIGNATURE_CATALOGUED |
| REDALERT/XSTRAW.H | 82 | function (callable) | Valid_File | FileStraw | (void) |  | SIGNATURE_CATALOGUED |

**Extracted callable/candidate symbols:** 6713

A source-level signature is not a behavior description. Use PORTING_CONTRACT_TEMPLATE.md to document each contract, or group tightly coupled helpers when their behavior is inseparable.
