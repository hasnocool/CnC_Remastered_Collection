# PATH/FILE
original/TIBERIANDAWN/INFANTRY.CPP

## Purpose
Implementation of the InfantryClass for Tiberian Dawn. InfantryClass extends FootClass with infantry-specific behavior including prone/crawl states, fear/panic mechanics, hand-to-hand combat (boxing), capture/sabotage missions, specialized death animations per warhead type, and unique AI for civilian vs military infantry.

## Where it fits in the game
Entity hierarchy: `ObjectClass` → `TechnoClass` → `FootClass` → `InfantryClass`. All foot soldiers (riflemen, rocket soldiers, engineers, commandos, civilians, dogs) inherit from InfantryClass. Infantry are the only units that can capture buildings, go prone, engage in hand-to-hand combat, and exhibit fear behavior.

## Classes, structs, enums and important data

### InfantryClass
**Core instance class** - individual infantry soldier with prone state, fear, ammo, boxing, and specialized animations.

**Key data members:**
- `Class` (InfantryTypeClass*) - Static type data (weapon, speed, sight, animations, etc.)
- `Doing` (DoType) - Current animation sequence (DO_STAND_READY, DO_WALK, DO_FIRE_WEAPON, DO_PRONE, DO_CRAWL, DO_FIRE_PRONE, DO_LIE_DOWN, DO_GET_UP, DO_IDLE1, DO_IDLE2, DO_ON_GUARD, DO_FIGHT_READY, DO_PUNCH, DO_KICK, DO_PUNCH_HIT1, DO_PUNCH_HIT2, DO_PUNCH_DEATH, DO_KICK_HIT1, DO_KICK_HIT2, DO_KICK_DEATH, DO_READY_WEAPON, DO_GUN_DEATH, DO_EXPLOSION_DEATH, DO_EXPLOSION2_DEATH, DO_GRENADE_DEATH, DO_FIRE_DEATH, DO_GESTURE1, DO_SALUTE1, DO_GESTURE2, DO_SALUTE2, DO_PULL_GUN, DO_PLEAD, DO_PLEAD_DEATH)
- `Fear` (int, 0-FEAR_MAXIMUM) - Fear level: 0=brave, FEAR_ANXIOUS=anxious, FEAR_SCARED=scared, FEAR_PANIC=panic, FEAR_MAXIMUM=max
- `IsProne` (bool) - Currently lying down (takes half damage, moves at half speed)
- `IsStoked` (bool) - Victory dance state (plays gesture animation after kills)
- `IsBoxing` (bool) - Engaged in hand-to-hand combat with another infantry
- `IsTechnician` (bool) - Spawned from building destruction (shows "Technician" name)
- `IsSecondShot` (bool) - Two-shooter tracking (false=first shot pending, true=normal rearm)
- `Strength` (int) - Current hit points (0=dead, max=Class->MaxStrength)
- `Ammo` (int) - Current ammo count (0=empty, max=Class->MaxAmmo; civilians carry less)
- `StopDriverFrame` (int) - Frame when Stop_Driver last called (for walk pose hold)
- `HumanShape[32]` (static const int) - Frame mapping for 32 facings: {0,0,7,7,7,7,6,6,6,6,5,5,5,5,5,4,4,4,3,3,3,3,2,2,2,2,1,1,1,1,1,0}
- `Infantry_Kick_Damage[2]` = {10, 15} - Kick damage values (random pick)
- `Infantry_Punch_Damage[2]` = {4, 7} - Punch damage values (random pick)
- `VTable` (static void*) - Virtual function table pointer for RTTI
- `MasterDoControls[DO_COUNT]` (static const DoStruct) - Animation control flags per DoType: {interruptable, mobile, randomstart, rate}

**Constructor state:**
- `Doing = DO_NOTHING`
- `Fear = 0` (completely brave)
- `IsProne = false`
- `IsStoked = false`
- `IsBoxing = false`
- `IsTechnician = false`
- `Strength = Class->MaxStrength`
- `Ammo = Class->MaxAmmo` (civilians: Class->MaxAmmo typically lower)
- `IsSecondShot = Class->IsTwoShooter ? false : true`
- `StopDriverFrame = -1`
- House unit total incremented (if !Class->IsCivilian && GAME_INTERNET)
- RA AI tracking added (if USE_RA_AI)

### DoStruct (animation control)
```cpp
struct DoStruct {
    bool Interrupt;    // Can animation be interrupted
    bool Mobile;       // Unit moves during animation
    bool RandomStart;  // Start at random frame
    int Rate;          // Frame rate (0 = static)
};
```

**MasterDoControls per DoType:**
| DoType | Interrupt | Mobile | RandomStart | Rate | Notes |
|--------|-----------|--------|-------------|------|-------|
| DO_STAND_READY | true | false | false | 0 | Idle standing |
| DO_STAND_GUARD | true | false | false | 0 | Guard standing |
| DO_PRONE | true | false | false | 0 | Prone idle |
| DO_WALK | true | true | true | 2 | Walking |
| DO_FIRE_WEAPON | true | false | false | 1 | Fire standing |
| DO_LIE_DOWN | false | true | false | 2 | Go prone |
| DO_CRAWL | true | true | true | 2 | Crawl move |
| DO_GET_UP | false | false | false | 3 | Stand up |
| DO_FIRE_PRONE | true | false | false | 1 | Fire prone |
| DO_IDLE1 | true | false | false | 2 | Idle anim 1 |
| DO_IDLE2 | true | false | false | 2 | Idle anim 2 |
| DO_ON_GUARD | false | false | false | 2 | Boxing guard |
| DO_FIGHT_READY | true | false | false | 2 | Boxing ready |
| DO_PUNCH | false | false | false | 2 | Punch attack |
| DO_KICK | false | false | false | 2 | Kick attack |
| DO_PUNCH_HIT1 | false | false | false | 2 | Light punch hit |
| DO_PUNCH_HIT2 | false | false | false | 2 | Heavy punch hit |
| DO_PUNCH_DEATH | false | false | false | 1 | Punch death |
| DO_KICK_HIT1 | false | false | false | 2 | Light kick hit |
| DO_KICK_HIT2 | false | false | false | 2 | Heavy kick hit |
| DO_KICK_DEATH | false | false | false | 1 | Kick death |
| DO_READY_WEAPON | false | false | false | 2 | Draw weapon |
| DO_GUN_DEATH | false | false | false | 2 | Bullet death |
| DO_EXPLOSION_DEATH | false | false | false | 2 | HE explosion death |
| DO_EXPLOSION2_DEATH | false | false | false | 2 | Alt explosion death |
| DO_GRENADE_DEATH | false | false | false | 2 | Grenade death |
| DO_FIRE_DEATH | false | false | false | 2 | Fire/laser death |
| DO_GESTURE1 | false | false | false | 2 | Gesture 1 |
| DO_SALUTE1 | false | false | false | 2 | Salute 1 |
| DO_GESTURE2 | false | false | false | 2 | Gesture 2 |
| DO_SALUTE2 | false | false | false | 2 | Salute 2 |
| DO_PULL_GUN | true | false | false | 2 | Draw gun |
| DO_PLEAD | true | false | false | 2 | Plead |
| DO_PLEAD_DEATH | true | false | false | 2 | Plead death |

### Debug display (CHEAT_KEYS)
- Name (House:Type), Mission, TarCom, NavCom, Radio, Coord, HeadTo, State
- Health, Body, Turret, Speed, Path, Cargo
- Active, Limbo, Owned, Discovered, Selected, Teathered, Locked on Map
- Is Prone, Is A Loner, Deploying, Rotating, Firing, Driving, To Look, Recoiling, To Display
- Fear, Doing (numeric)
- FootClass debug fields

## Functions and methods

### Constructors/Destructor

**InfantryClass::InfantryClass()** - Default constructor
- Does nothing (Class = 0)
- Used for pool allocation

**InfantryClass::InfantryClass(classid, house)** - Normal constructor
- Base: `FootClass(house)`, `Class(&InfantryTypeClass::As_Reference(classid))`
- Initializes all infantry-specific state (see Constructor state above)
- Increments House->InfantryTotals for non-civilians in GAME_INTERNET
- Adds to RA AI tracking (if USE_RA_AI)

**InfantryClass::~InfantryClass()** - Destructor
- If GameActive: removes from RA AI tracking, calls Limbo()
- If GameActive && Team: Team->Remove(this)

**InfantryClass::operator new(size_t)** - Pool allocation
- Allocates from Infantry free pool via `Infantry.Allocate()`
- Calls Set_Active() on success
- Returns pointer or NULL

**InfantryClass::operator delete(void*)** - Pool deallocation
- Sets IsActive = false
- Returns to pool via `Infantry.Free()`

### Validation & Debug

**InfantryClass::Validate()** → int (CHEAT_KEYS only)
- Checks `Infantry.ID(this)` in range [0, INFANTRY_MAX)
- Returns 1=ok, 0=error (calls Validate_Error)

**InfantryClass::Debug_Dump(MonoClass*)** - Debug display (CHEAT_KEYS)
- Outputs formatted state to monochrome screen (see Debug display above)

### Core AI

**InfantryClass::AI()** - Main AI tick (called once per frame)
- Calls FootClass::AI()
- Clears IsFiring if animation rate 0
- Deletes if off-map in MISSION_GUARD with no Team
- Calls Commence() if not driving and animation interruptible
- **Fear decay**: Fear-- per frame (except INFANTRY_C10 Nikumba never loses fear)
  - When Fear reaches 0 and Ammo==0 && Primary!=WEAPON_NONE: reloads Ammo = MaxAmmo
- **Prone transitions**:
  - If prone and Fear < FEAR_ANXIOUS: Do_Action(DO_GET_UP)
  - If not prone and Fear >= FEAR_ANXIOUS and (no NavCom or !Special.IsDefenderAdvantage): Do_Action(DO_LIE_DOWN)
- **FraidyCat scatter**: If IsFraidyCat && Fear > FEAR_ANXIOUS && !IsDriving && !NavCom: Scatter(0)
- **Building scatter**: If !IsTethered && !IsFiring && !IsDriving && !IsRotating && (MISSION_GUARD/MISSION_GUARD_AREA) && no MissionQueue && on building: Scatter(0, true, true)
- **Victory dance**: If no NavCom && !IsProne && IsStoked && Comment.Expired(): IsStoked=false, random DO_GESTURE1/2; Rambo plays VOC_RAMBO_LEFTY/LAUGH/COMIN/TUFF
- **Firing logic**: Switch on Can_Fire(TarCom, 0):
  - FIRE_CLOAKED: Do_Uncloak()
  - FIRE_OK: 
    - If boxing: handle boxing attacks (punch/kick)
    - Else: Do_Action(IsProne?DO_FIRE_PRONE:DO_FIRE_WEAPON), IsFiring=true, face target
    - If TarCom==NavCom: clear NavCom, Path[0]=FACING_NONE
- **Fire launch**: At animation stage == Class->FireLaunch (or ProneLaunch, or 1/2 for boxing): Fire_At(TarCom, 0); grenade triggers Map.Incoming()
- **Animation completion**: When stage >= DoControls[Doing].Count:
  - Death anims: Delete_This()
  - Boxing anims: If In_Radio_Contact() continue DO_FIGHT_READY, else IsBoxing=false, DO_READY_WEAPON
  - Else: Resume DO_CRAWL/DO_WALK (driving) or DO_PRONE/DO_STAND_READY (idle)
- **Movement**: If !IsFiring:
  - Guard mode with NavCom: Assign_Destination(TARGET_NONE)
  - Pathfinding: Basic_Path() with retry logic (TryTryAgain = PATH_RETRY)
  - Obstacle handling: Attack blocking enemies/walls
  - Start_Driver() with speed adjustment (prone: half speed or double for fraidy cats)
  - Tethered units: Transmit_Message(RADIO_REDRAW)

### Damage & Combat

**InfantryClass::Take_Damage(damage, distance, warhead, source)** → ResultType
- Validate, IsFiring=false
- **Prone damage reduction**: If IsProne && damage>0: damage >>= 1 (half damage, no minimum clamp)
- Calls FootClass::Take_Damage()
- **Flame thrower (E4) extra damage**: If not destroyed && *this==INFANTRY_E4: extra 5 damage with WARHEAD_FIRE
- **Death handling** (RESULT_DESTROYED):
  - Death_Announcement(source), Stop_Driver(), Stun(), Mission=MISSION_NONE, Assign_Mission(MISSION_GUARD), Commence()
  - **E4 (flame)**: AnimClass(ANIM_NAPALM1), Explosion_Damage(80, WARHEAD_FIRE)
  - **E2**: AnimClass(ANIM_ART_EXP1), Explosion_Damage(30, WARHEAD_HE)
  - **E5 (chem)**: AnimClass(ANIM_CHEM_BALL), Explosion_Damage(80, WARHEAD_HE)
  - **Sound**: Rambo=VOC_RAMBO_YELL, else random VOC_SCREAM1-5 (alt VOC_YELL1)
  - **Death animation by warhead**:
    - WARHEAD_FEEDME: Heals source +30, falls to WARHEAD_SA
    - WARHEAD_HEADBUTT/SPORE/HOLLOW_POINT/SA: DO_GUN_DEATH
    - WARHEAD_HE: DO_EXPLOSION_DEATH
    - WARHEAD_AP: DO_GRENADE_DEATH
    - WARHEAD_PB/LASER/FIRE: DO_FIRE_DEATH (alt sound)
    - WARHEAD_FIST: DO_PUNCH_DEATH
    - WARHEAD_FOOT: DO_KICK_DEATH
- **Fear increase** (non-destroyed):
  - If source: Scatter(source->Coord)
  - If boxing: Play hit anim (PUNCH_HIT1/2 or KICK_HIT1/2 based on damage)
  - Else if source && Fear < FEAR_SCARED:
    - IsFraidyCat: Fear = FEAR_PANIC
    - Else: Fear = FEAR_SCARED
  - Else: Fear += FEAR_ANXIOUS (divided by 4 if Health_Ratio > 0x80), clamped to FEAR_MAXIMUM

**InfantryClass::Can_Fire(target, which)** → FireErrorType
- **Boxing checks**: If boxing and (hit anim playing || Arm>0): FIRE_BUSY
- If IsFiring: FIRE_REARM
- If !Target_Legal(target): FIRE_ILLEGAL
- If (IsDriving && Special.IsDefenderAdvantage) || (animation not interruptible): FIRE_MOVING
- If !Special.IsDefenderAdvantage && IsDriving && facing diff >= 32: FIRE_MOVING
- Returns FootClass::Can_Fire(target, which)

**InfantryClass::Fire_At(target, which)** → BulletClass*
- Validate, IsFiring=false
- **Boxing**: If boxing:
  - hitaction = DO_KICK ? RADIO_KICK : RADIO_PUNCH
  - If In_Radio_Contact && Target_Legal && Transmit_Message(hitaction)==ROGER: Arm = Rearm_Delay(true)
  - Else: IsBoxing=false, Do_Action(DO_READY_WEAPON)
- **Normal fire**: bullet = FootClass::Fire_At(target, which)
  - If bullet:
    - FraidyCat out of ammo: Fear = FEAR_MAXIMUM, if MISSION_ATTACK/HUNT → MISSION_GUARD
    - Sound_Effect(weapon->Sound, Coord)
- Returns bullet

**InfantryClass::Fire_Coord(which)** → COORDINATE
- INFANTRY_E4 (flame): returns Coord
- Others: returns Coord_Add(Coord, XYP_COORD(0, -5)) (5 leptons up)

**InfantryClass::Rearm_Delay(second)** → int
- If boxing: Random_Pick(5, 50)
- Else: FootClass::Rearm_Delay(second)

### Movement & Pathfinding

**InfantryClass::Assign_Destination(target)**
- If Target_Legal(target): Stop_Driver()
- Human double-click on same target while prone && !IsFraidyCat: Do_Action(DO_GET_UP)
- **Mission_Enter handling**: If MISSION_ENTER:
  - If !In_Radio_Contact(): Try RADIO_HELLO → RADIO_DOCKING handshake
  - Else: Path[0] = FACING_NONE
- Else: Path[0] = FACING_NONE
- Calls FootClass::Assign_Destination(target)

**InfantryClass::Assign_Target(target)**
- Path[0] = FACING_NONE
- FootClass::Assign_Target(target)
- If no NavCom && IsCapture && Primary==WEAPON_NONE && target is capturable building: Assign_Destination(target)

**InfantryClass::Can_Enter_Cell(cell, facing)** → MoveType
- Returns MOVE_NO if cell >= MAP_CELL_TOTAL
- If IsLocked && !IsALoaner && !ScenarioInit && !Map.In_Radar(cell): MOVE_NO
- **Wall check**: If overlay is wall and not fully damaged: MOVE_NO
- **Crate check**: If AI && overlay is crate: MOVE_NO
- **Occupant iteration**: For each object in cell:
  - Skip if capture/sabotage target or transport entry
  - **Allied**:
    - Unit: MOVE_MOVING_BLOCK if driving/has NavCom, else MOVE_TEMP
    - Aircraft/Building/Terrain: MOVE_NO
  - **Enemy** (not cloaked or IsFindPath):
    - No weapon: MOVE_NO
    - Terrain: MOVE_DESTROYABLE if flammable && weapon=FIRE, else MOVE_NO
    - Other: MOVE_DESTROYABLE
  - Cloaked enemy: MOVE_CLOAK
- **Terrain cost**: If MOVE_OK && !IsTethered && Ground[land].Cost[SPEED_FOOT]==0: MOVE_NO
- **Vehicle occupy**: If MOVE_OK && cellptr->Flag.Occupy.Vehicle: MOVE_NO
- **Infantry occupy**: If allied && cell full (0x1F): MOVE_MOVING_BLOCK; If enemy: MOVE_DESTROYABLE if armed else MOVE_NO
- **Cell full**: If MOVE_OK && (Composite & 0x1F) == 0x1F: MOVE_NO
- Returns worst blockage found

**InfantryClass::Overlap_List()** → short const*
- Returns Coord_Spillage_List(Coord, 24 + ((Doing > DO_WALK || Is_Selected_By_Player()) ? 12 : 0))

**InfantryClass::Per_Cell_Process(center)**
- **Capture**: If center && MISSION_CAPTURE && cell has target building that Can_Capture(): Captured(House), Delete_This()
- **Enter transport**: If center && MISSION_ENTER && at transport cell && In_Radio_Contact && Transmit_Message(RADIO_IM_IN)==RADIO_ATTACH: Limbo(), techno->Attach(this)
- **Sabotage**: If center && MISSION_SABOTAGE && building at cell == NavCom: Building->IsGoingToBlow=true, Clicked_As_Target, CountDown=20, WhomToRepay=this, Scatter(building->Center_Coord(), true)
- **Tether release**: If center && IsTethered: Transmit_Message(RADIO_UNLOADED), DO_GESTURE1/2 (Good/Bad), Rambo: VOC_RAMBO_ROCK; If cell full of infantry: cellptr->Incoming(0, true)
- **Commence**: If center: Commence()
- Look(true)
- FootClass::Per_Cell_Process(center)
- **Tiberium damage**: If active && !limbo && center && land==LAND_TIBERIUM && not INFANTRY_E5: Take_Damage(2, 0, WARHEAD_FIRE)

**InfantryClass::Scatter(threat, forced, nokidding)**
- No scatter if IsDriving || NavCom legal (unless forced)
- No scatter if in combat (Target_Legal(TarCom) && !IsFraidyCat && !forced)
- No scatter if animation not interruptible
- Human players: no scatter unless Special.IsScatter || nokidding || forced || Team
- If forced || IsFraidyCat:
  - Calculate toface from threat or map center
  - Try 8 directions from toface: if cell in radar && Can_Enter_Cell==MOVE_OK && (not avoiding tiberium or no tiberium): Assign_Mission(MISSION_MOVE), Assign_Destination(cell)

**InfantryClass::Start_Driver(headto)** → bool
- headto = Map[cell].Closest_Free_Spot(headto moved south 0x7C)
- If fails && Can_Enter_Cell ok: retry with 0x80 offset
- If success: Clear_Occupy_Bit(Coord), Set_Occupy_Bit(headto), FootClass::Start_Driver(headto)

**InfantryClass::Stop_Driver()** → bool
- Clear_Occupy_Bit(Head_To_Coord())
- Set_Occupy_Bit(Coord)
- StopDriverFrame = Frame (if not DO_STAND_READY)
- Do_Action(IsProne?DO_PRONE:DO_STAND_READY)
- FootClass::Stop_Driver()

**InfantryClass::Set_Occupy_Bit(cell, spot_index)**
- Sets Map[cell].Flag.Composite |= (1<<spot_index)
- Map[cell].InfType = Owner()

**InfantryClass::Clear_Occupy_Bit(cell, spot_index)**
- Clears bit in Composite
- If no infantry bits left: Map[cell].InfType = HOUSE_NONE

### Animation & Actions

**InfantryClass::Do_Action(todo, force)** → bool
- If todo!=Doing && (DO_NOTHING || force || interruptible):
  - Mark(MARK_CHANGE), Doing=todo
  - Rate = Options.Normalize_Delay(MasterDoControls[Rate]) for IDLE1/2, else MasterDoControls[Rate]
  - Stage = 0
  - If !Strength: Stop_Driver()
  - **State flags by action**:
    - DO_LIE_DOWN: IsProne=true
    - DO_GET_UP: IsProne=false
    - DO_READY_WEAPON: IsBoxing=false
    - DO_ON_GUARD: IsBoxing=true, face target, Path[0]=FACING_NONE
  - Returns true
- Returns false

**InfantryClass::Draw_It(x, y, window)**
- Validate
- facing = Facing_To_32(PrimaryFacing.Current())
- shapefile = Class->Get_Image_Data()
- y+=4, x-=2
- doit = Doing==DO_NOTHING ? DO_STAND_READY : Doing
- **Walk pose hold**: If DO_STAND_READY && window==WINDOW_VIRTUAL && StopDriverFrame!=-1 && Frame-StopDriverFrame<=2 && Path[0]!=FACING_NONE: doit=DO_WALK
- shapenum = (Stage % Count) + facenum*Jump + Frame (from Class->DoControls[doit])
- **Boxing hack**: If IsBoxing && BodyFacing<128: shapenum+=47
- Techno_Draw_Object(shapefile, shapenum, x, y, window)
- FootClass::Draw_It(x, y, window)

**InfantryClass::Random_Animate()**
- If !IsDriving && !IsProne && (DO_STAND_GUARD || DO_STAND_READY) && !IsFiring:
  - **FraidyCat**: If !House->IsHuman && Fear>FEAR_ANXIOUS: Scatter(NULL, true)
  - **Nikumba (C10)**: Random_Pick(0,3): case 0: DO_IDLE2
  - **Random pick 0-55**:
    - 10: DO_SALUTE1
    - 11: DO_SALUTE2
    - 12: DO_GESTURE1
    - 13: DO_GESTURE2
    - 0,3,4: Random facing, Mark(MARK_CHANGE)
    - 1: DO_IDLE1 + random facing; Moebius discovered: VOC_EXCELLENT3/QUIP1
    - 2: DO_IDLE2 + random facing; Rambo owned: VOC_RAMBO_CMON
    - 5-9: FraidyCat civilians: Scatter(NULL, true)

### Missions & Orders

**InfantryClass::Enter_Idle_Mode(initial)**
- Priority: TarCom legal → MISSION_ATTACK; NavCom legal → MISSION_MOVE; else:
  - Normal game or human: MISSION_GUARD
  - AI (non-RA): MISSION_HUNT
  - RA AI: MISSION_GUARD (or GUARD_AREA if IQ>=Rule.IQGuardArea && armed && no Team)
- Assign_Mission(order)

**InfantryClass::Assign_Mission(order)**
- If MISSION_SABOTAGE: Sound_Effect(VOC_RAMBO_PRESENT, Coord)
- IsBoxing = false
- FootClass::Assign_Mission(order)

**InfantryClass::What_Action(object)** → ActionType
- action = FootClass::What_Action(object)
- **Commando sabotage**: If INFANTRY_RAMBO && action==ACTION_ATTACK && object is building: ACTION_SABOTAGE
- **No self-select**: If ACTION_SELF: ACTION_NONE
- **Enter transport**: If allied && owned by player && Transmit_Message(RADIO_CAN_LOAD)==ROGER: ACTION_ENTER
- **Capture**: If IsCapture && action==ACTION_ATTACK && enemy && (empty transport aircraft OR capturable building): ACTION_CAPTURE; Else if Primary==WEAPON_NONE: ACTION_NONE

**InfantryClass::Active_Click_With(action, object)**
- If What_Action(object)!=action:
  - SABOTAGE/CAPTURE → ACTION_ATTACK
  - ENTER → ACTION_MOVE
  - Default → ACTION_NONE
- FootClass::Active_Click_With(action, object)

**InfantryClass::Mission_Attack()** → int
- If INFANTRY_E7 (engineer) && target is building: Assign_Destination(TarCom), Assign_Mission(MISSION_CAPTURE), return 1
- Else: FootClass::Mission_Attack()

**InfantryClass::Greatest_Threat(threat)** → TARGET
- **Engineer capture-only**: If IsCapture && Primary==WEAPON_NONE && not E7: threat |= THREAT_CAPTURE
- **Anti-air**: If Primary weapon bullet IsAntiAircraft: threat |= THREAT_AIR
- **Sniper (RIFLE)**: If human && (GUARD/GUARD_AREA): TARGET_NONE; Else: TechnoClass::Greatest_Threat(threat | THREAT_INFANTRY|THREAT_BUILDINGS)
- Default: FootClass::Greatest_Threat(threat)

**InfantryClass::Made_A_Kill()** → int
- If INFANTRY_RAMBO || Random_Pick(0,5) < Kills: IsStoked=true, Comment=2*TICKS_PER_SECOND
- FootClass::Made_A_Kill()

### Audio Responses

**InfantryClass::Response_Select()**
- Rambo: VOC_RAMBO_YEA/YES/YO
- Moebius: VOC_YES/COMMANDER/HELLO/HMMM
- Civilian female: VOC_GIRL_YEAH
- Civilian male: VOC_GUY_YEAH
- Military: VOC_ACKNOWL/REPORT/REPORT/YESSIR/YESSIR/READY/AWAIT
- Sound_Effect(response, 0, Infantry.ID(this)+1) if AllowVoice

**InfantryClass::Response_Move()**
- Rambo: VOC_RAMBO_UGOTIT/ONIT/NOPROB
- Moebius: VOC_OF_COURSE/YESYES
- Civilian female: VOC_GIRL_OKAY
- Civilian male: VOC_GUY_OKAY
- Military: VOC_MOVEOUT/MOVEOUT/MOVEOUT/ROGER/RIGHT_AWAY/UGOTIT/AFFIRM/AFFIRM
- Sound_Effect if AllowVoice

**InfantryClass::Response_Attack()**
- Rambo: VOC_RAMBO_NOPROB/UGOTIT/NOPROB/ONIT
- Civilian female: VOC_GIRL_OKAY
- Civilian male: VOC_GUY_OKAY
- Military: VOC_RIGHT_AWAY/AFFIRM/AFFIRM/UGOTIT/NO_PROB/YESSIR/YESSIR/YESSIR
- Sound_Effect if AllowVoice

### Radio Messages

**InfantryClass::Receive_Message(from, message, param)** → RadioMessageType
- **RADIO_OVER_OUT**: If boxing: Do_Action(DO_READY_WEAPON)
- **RADIO_PREPARE_TO_BOX**: If not boxing && Contact_With_Whom()==from: Do_Action(DO_ON_GUARD), Assign_Target(from), RADIO_ROGER; else RADIO_NEGATIVE
- **RADIO_KICK**: damage=random Kick_Damage, Take_Damage(WARHEAD_FOOT), ROGER or STATIC if destroyed
- **RADIO_PUNCH**: damage=random Punch_Damage, Take_Damage(WARHEAD_FIST), ROGER or STATIC if destroyed
- Else: FootClass::Receive_Message()

### Limbo/Unlimbo

**InfantryClass::Limbo()** → bool
- If !IsInLimbo: Stop_Driver(), Clear_Occupy_Bit(Coord)
- FootClass::Limbo()

**InfantryClass::Unlimbo(coord, facing)** → bool
- coord = Map[cell].Closest_Free_Spot(coord, ScenarioInit)
- If FootClass::Unlimbo(coord, facing):
  - House->IScan/ActiveIScan |= (1<<Class->Type)
  - If SightRange==0 && normal game && !human && !visible to player: IsDiscoveredByPlayer=false
  - Set_Occupy_Bit(coord)
  - Returns true
- Returns false

### Serialization

**InfantryClass::Read_INI(buffer)**
- Reads [INFANTRY] section entries: House,Type,Strength,Cell,SubPos,Mission,Facing,Trigger
- Special case: C7→C5 on scg08eb (Good, mission 8B)
- Special case: Delete pre-placed Chan on scb10ea (Bad, mission 10A)
- Creates infantry, sets strength, unlimbos at coord, assigns mission or Enter_Idle_Mode

**InfantryClass::Write_INI(buffer)**
- Clears existing [INFANTRY] section
- For each active infantry: writes "House,Type,HealthRatio,Cell,SpotIndex,MissionName,Facing,TriggerName"

### Utility

**InfantryClass::Full_Name()** → int
- If IsTechnician: TXT_TECHNICIAN
- Else: Class->Full_Name()

**InfantryClass::What_Am_I()** → RTTIType
- Returns RTTI_INFANTRY

**InfantryClass::What_Action(cell)** → ActionType
- FootClass::What_Action(cell)

**InfantryClass::Class_Of()** → ObjectTypeClass const&
- Returns *Class

**InfantryClass::Is_Infantry()** → bool
- Returns true

## Inputs, outputs and side effects

**State mutations:**
- AI: Fear, IsProne, IsBoxing, IsStoked, Doing, IsFiring, NavCom, Path, Mission, Ammo, Strength
- Take_Damage: Strength, Fear, IsFiring, Mission, IsProne (via Do_Action), plays sounds/animations
- Movement: Coord, Path, HeadToCoord, IsDriving, IsProne, occupy bits (Set/Clear_Occupy_Bit)
- Animation: Doing, Stage, Rate, PrimaryFacing, IsProne, IsBoxing
- Limbo/Unlimbo: IsInLimbo, occupy bits, House IScan, IsDiscoveredByPlayer
- Boxing: IsBoxing, Arm, TarCom, radio messages

**External systems affected:**
- Map: Occupy bits, threat maps, Tiberium damage, cell Overlap/Refresh
- AnimClass: Death animations (NAPALM1, ART_EXP1, CHEM_BALL)
- Explosion_Damage: Area damage on death (E2, E4, E5)
- Sound: Vocal responses, death screams, weapon sounds
- Radio: Boxing handshake (PREPARE_TO_BOX, KICK, PUNCH, OVER_OUT), transport (HELLO, DOCKING, IM_IN, OVER_OUT), capture/sabotage
- Trigger: EVENT_ATTACKED (via FootClass::Take_Damage)
- Selection: Health bar updates
- House: Unit totals (GAME_INTERNET), IScan/ActiveIScan
- Team: Remove on destruction

## Calls into / called by

**Called by:**
- Scenario load: Read_INI()
- Game loop: AI() per frame
- Player input: Active_Click_With(), Assign_Destination(), Assign_Target(), Response_Select/Move/Attack()
- Damage system: Take_Damage()
- Radio: Receive_Message()
- Rendering: Draw_It()
- Movement: Per_Cell_Process(), Start_Driver(), Stop_Driver()
- Serialization: Write_INI()
- Object creation: Unlimbo() after new InfantryClass()
- Object deletion: Limbo() in destructor

**Calls:**
- FootClass: AI(), Take_Damage(), Assign_Destination(), Assign_Target(), Assign_Mission(), Per_Cell_Process(), Draw_It(), Start_Driver(), Stop_Driver(), Fire_At(), Greatest_Threat(), Made_A_Kill(), Receive_Message(), Limbo(), Unlimbo(), What_Action(), Can_Fire(), Rearm_Delay()
- TechnoClass: Greatest_Threat(), Death_Announcement(), Set_Primary_Facing()
- Map: In_Radar(), Cell_Techno(), Cell_Building(), Overlay checks, Ground costs, Closest_Free_Spot(), Sight_From(), Adjust_Threat, Overlap/Refresh
- AnimClass: Death animations
- Explosion_Damage: Area damage
- Sound_Effect: All vocal and weapon sounds
- Radio: Transmit_Message() for all radio handshakes
- House: IScan, InfantryTotals, Tracking_Add/Remove
- Team: Remove()
- BulletTypeClass: Warhead lookups
- Class (InfantryTypeClass): DoControls, FireLaunch, ProneLaunch, MaxStrength, MaxAmmo, IsTwoShooter, IsCivilian, IsCapture, Primary, Secondary, SightRange, IsFraidyCat, IsCrawling, IsDefenderAdvantage, IsAvoidingTiberium, Full_Name(), Get_Image_Data()

## Special infantry types referenced

- **INFANTRY_E1-E7**: Special infantry (E1=minigunner?, E2=?, E3=?, E4=flame thrower, E5=chem warrior, E6=?, E7=engineer)
- **INFANTRY_RAMBO**: Commando (special sounds, sabotage, victory dance)
- **INFANTRY_C10**: Nikumba (never loses fear, special idle anims)
- **INFANTRY_MOEBIUS**: Civilian scientist (special select/move responses)
- **INFANTRY_CHAN**: Special character (deleted on scb10ea)
- **INFANTRY_C5/C7**: Civilian variants (mission-specific swap)

## Configuration constants

- `INFANTRY_MAX` - Maximum infantry pool size
- `DO_COUNT` - Number of DoType animations (34)
- `PATH_RETRY` - Pathfinding retry count (3)
- `FEAR_ANXIOUS`, `FEAR_SCARED`, `FEAR_PANIC`, `FEAR_MAXIMUM` - Fear thresholds
- `TICKS_PER_SECOND` - Game ticks per second (60)
- `WARHEAD_*`: FEEDME, HEADBUTT, SPORE, HOLLOW_POINT, SA, HE, AP, PB, LASER, FIRE, FIST, FOOT
- `VOC_*`: All vocal response constants
- `ANIM_*`: NAPALM1, ART_EXP1, CHEM_BALL

## Source
original/TIBERIANDAWN/INFANTRY.CPP at f1f0d42