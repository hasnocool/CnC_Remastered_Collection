# PATH/FILE
original/TIBERIANDAWN/WINSTUB.CPP

## Purpose
This file contains stubs for undefined externals when linked under Watcom for Win 95. It provides minimal function bodies and variable declarations to satisfy linker requirements for the Command & Conquer Windows port. Most functions are empty stubs; some are conditional (#if(0)) and compiled out. The file bridges the gap between the game's external references and the actual implementation in other translation units.

## Where it fits in the game
Win32/DLL entry point support module. WINSTUB.CPP is linked into the DLL build to provide placeholder definitions for functions and variables declared extern across the codebase. It runs very early in the startup sequence (before main game initialization) and serves as a compatibility layer for the Watcom C compiler targetting Windows 95. Undefined externals resolved by this file include palette management, focus loss handling, window procedures, and VQA audio control.

## Classes, structs, enums and important data

### ColourList struct
```c
typedef struct tColourList {
	char	Red;
	char	Green;
	char	Blue;
} ColourList;
```
ColourLookup[9] - Global array of 9 colour entries used for debug/palette debugging:
- Entry 0: 0, 0, 0 (black)
- Entry 1: 63, 0, 0 (red)
- Entry 2: 0, 63, 0 (green)
- Entry 3: 0, 0, 63 (blue)
- Entry 4: 63, 0, 63 (magenta)
- Entry 5: 63, 63, 0 (yellow)
- Entry 6: 0, 63, 63 (cyan)
- Entry 7: 32, 32, 32 (dark gray)
- Entry 8: 63, 63, 63 (white)

### Global variables
- `InDebugger` (bool) - Flag indicating if running under debugger
- `ReadyToQuit` (bool) - Flag set when game should quit
- `OldTheme` (ThemeType) - Tracks currently playing theme before focus loss
- `CCFocusMessage` (unsigned long) - Private window message ID (WM_USER+50) for focus events
- `InMovie` (BOOL) - Flag indicating if playing a movie/ cutscene
- `DebugFile` (HANDLE) - Handle to debug log file (debug.txt)
- `DebugColour` (int) - Current debug colour index
- `VQPalette` (unsigned char*) - VQ palette data pointer
- `VQNumBytes` (long) - VQ palette byte count
- `VQSlowpal` (unsigned long) - Slow palette factor
- `VQPaletteChange` (bool) - Flag indicating VQ palette needs updating

### Extern declarations (referenced elsewhere)
- `VQA_PauseAudio()` / `VQA_ResumeAudio()` - VQ audio control
- `Set_Palette_Register()` - Palette register setter
- `GamePalette`, `GameInFocus`, `MainWindow`, `hInstance`, `SoundOn`, `Theme`, `WWMouse`, `VisiblePage`, `HiddenPage`, `AllSurfaces` - global game state

## Functions and methods

### output(short, short)
```c
void output(short, short) {}
```
Empty stub function. No implementation body; exists solely to satisfy linker expectations for this external symbol.

### Focus_Loss(void)
```c
void Focus_Loss(void)
{
	if (SoundOn){
		if (OldTheme == THEME_NONE){
			OldTheme = Theme.What_Is_Playing();
		}
	}
	Theme.Stop();
	Stop_Primary_Sound_Buffer();
	if (WWMouse) WWMouse->Clear_Cursor_Clip();
}
```
Called when the application loses focus. Saves the current theme, stops primary sound buffer, and clears the mouse cursor clip. References external: `SoundOn`, `OldTheme`, `Theme`, `Stop_Primary_Sound_Buffer()`, `WWMouse`.

### Focus_Restore(void)
```c
void Focus_Restore(void)
{
	Restore_Cached_Icons();
	Map.Flag_To_Redraw(true);
	Start_Primary_Sound_Buffer(TRUE);
	if (WWMouse) WWMouse->Set_Cursor_Clip();
	VisiblePage.Clear();
	HiddenPage.Clear();
}
```
Restores application focus. Restores cached icons, flags redraw, restores primary sound buffer, sets mouse cursor clip, and clears visible/hidden pages. References external: `Restore_Cached_Icons()`, `Map`, `Start_Primary_Sound_Buffer()`, `WWMouse`, `VisiblePage`, `HiddenPage`.

### Check_For_Focus_Loss(void)
```c
void Check_For_Focus_Loss(void)
{
#if (0)
	static BOOL	focus_last_time = 1;
			 MSG	msg;

	if ( !GameInFocus ){
		Focus_Loss();
		while( PeekMessage( &msg, NULL, 0, 0, PM_NOREMOVE | PM_NOYIELD ) ){
	  		if( !GetMessage( &msg, NULL, 0, 0 ) ){
				return;
			}
			TranslateMessage(&msg);
			DispatchMessage(&msg);
		}
	}

	if (!focus_last_time && GameInFocus){
		VQA_PauseAudio();
		CountDownTimerClass cd;
		cd.Set(60*1);
		do {
			while (PeekMessage( &msg, NULL, 0, 0, PM_NOREMOVE )) {
	  			if( !GetMessage( &msg, NULL, 0, 0 ) ){
					return;
				}
				TranslateMessage(&msg);
				DispatchMessage(&msg);
			}
		} while(cd.Time());
		VQA_ResumeAudio();
		//AllSurfaces.Restore_Surfaces();
		//VisiblePage.Clear();
		//HiddenPage.Clear();
		//Map.Flag_To_Redraw(true);
		PostMessage (MainWindow, CCFocusMessage,0,0);
	}
	focus_last_time = GameInFocus;
#endif
}
```
Focus loss checking routine. Entire body is wrapped in `#if(0)` — the active stub does nothing. When active (0 disabled), it would pause/resume VQA audio, restore surfaces on focus return, and post the CCFocusMessage to MainWindow. References external: `GameInFocus`, `Focus_Loss`, `PeekMessage`, `GetMessage`, `TranslateMessage`, `DispatchMessage`, `VQA_PauseAudio`, `VQA_ResumeAudio`, `CountDownTimerClass`, `AllSurfaces`, `MainWindow`, `CCFocusMessage`.

### Create_Main_Window(HANDLE instance, int command_show, int width, int height)
```c
void Create_Main_Window ( HANDLE instance ,int command_show , int width , int height )
{
	MainWindow = NULL;
	return;
#if (0)
	HWND        	hwnd ;
	WNDCLASS    	wndclass ;

	// Register the window class
	wndclass.style         = CS_HREDRAW | CS_VREDRAW ;
	wndclass.lpfnWndProc   = Windows_Procedure ;
	wndclass.cbClsExtra    = 0 ;
	wndclass.cbWndExtra    = 0 ;
	wndclass.hInstance     = instance ;
	wndclass.hIcon         = LoadIcon (instance, MAKEINTRESOURCE(CC_ICON)) ;
	wndclass.hCursor       = NULL;
	wndclass.hbrBackground = NULL;
	wndclass.lpszMenuName  = "Command & Conquer";	//NULL
	wndclass.lpszClassName = "Command & Conquer";

	RegisterClass (&wndclass) ;

	// Create our main window
	hwnd = CreateWindowEx (
							WS_EX_TOPMOST,
							"Command & Conquer",
							"Command & Conquer",
							WS_POPUP | WS_MAXIMIZE,
							0,
							0,
							width,
							height,
							NULL,
							NULL,
							instance,
							NULL );

	ShowWindow (hwnd, command_show );
	ShowCommand = command_show;
	UpdateWindow (hwnd);
	SetFocus (hwnd);
	MainWindow=hwnd;			//Save the handle to our main window
	hInstance = instance;
	CCFocusMessage = RegisterWindowMessage ("CC_GOT_FOCUS");
	Audio_Focus_Loss_Function = &Focus_Loss;
	Misc_Focus_Loss_Function = &Focus_Loss;
	Misc_Focus_Restore_Function = &Focus_Restore;
	Gbuffer_Focus_Loss_Function = &Focus_Loss;
#endif
}
```
Creates the main game window. The active stub is a no-op (`MainWindow = NULL; return;`). The full implementation (behind `#if(0)`) would register a WNDCLASS, create a topmost popup window sized to width/height, and save the window handle globally. References external: `CC_ICON`, `Windows_Procedure`, `WS_EX_TOPMOST`, `WS_POPUP`, `WS_MAXIMIZE`, `ShowWindow`, `UpdateWindow`, `SetFocus`, `hInstance`, `CCFocusMessage`, `Audio_Focus_Loss_Function`, `Misc_Focus_Loss_Function`, `Misc_Focus_Restore_Function`, `Gbuffer_Focus_Loss_Function`.

### Window_Dialog_Box(HANDLE hinst, LPCTSTR lpszTemplate, HWND hwndOwner, DLGPROC dlgprc)
```c
void Window_Dialog_Box(HANDLE  hinst, LPCTSTR  lpszTemplate, HWND  hwndOwner, DLGPROC  dlgprc)
{
#if (0)
	MSG					msg;
	/*
	** Get rid of the Westwood mouse cursor because we are showing a
	** dialog box and we want to have the right windows cursor showing
	** for it.
	*/
	Hide_Mouse();
	ShowCursor(TRUE);

	/*
	** Pop up the dialog box and then run a standard message handler
	** until the dialog box is closed.
	*/
	DialogBox(hinst, lpszTemplate, hwndOwner, dlgprc);
	while (GetMessage(&msg, NULL, 0, 0) && !AllDone) {
		TranslateMessage(&msg);
		DispatchMessage(&msg);
	}

	/*
	** Restore the westwood mouse cursor and get rid of the windows one
	** because it is now time to restore back to the westwood way of
	** doing things.
	*/
	ShowCursor(FALSE);
	Show_Mouse();
#endif
}
```
Dialog box display routine. Entire body is `#if(0)` — compiled out. Would hide WW mouse, show cursor, call DialogBox, pump messages, then restore cursor. References external: `Hide_Mouse`, `ShowCursor`, `DialogBox`, `GetMessage`, `TranslateMessage`, `DispatchMessage`, `ShowCursor`, `Show_Mouse`, `AllDone`.

### Windows_Procedure(HWND hwnd, UINT message, UINT wParam, LONG lParam)
```c
long FAR PASCAL _export Windows_Procedure (HWND hwnd, UINT message, UINT wParam, LONG lParam)
{
	int	low_param = LOWORD(wParam);

	if (message == CCFocusMessage){
		Start_Primary_Sound_Buffer(TRUE);
		if (!InMovie){
			Theme.Queue_Song(OldTheme);
			OldTheme = THEME_NONE;
		}
		return(0);
	}

	switch ( message ){
		case WM_MOUSEMOVE:
		case WM_SYSKEYDOWN:
		case WM_SYSKEYUP:
		case WM_KEYDOWN:
		case WM_KEYUP:
		case WM_LBUTTONDOWN:
		case WM_LBUTTONUP:
		case WM_LBUTTONDBLCLK:
		case WM_MBUTTONDOWN:
		case WM_MBUTTONUP:
		case WM_MBUTTONDBLCLK:
		case WM_RBUTTONDOWN:
		case WM_RBUTTONUP:
		case WM_RBUTTONDBLCLK:
		 	Kbd.Message_Handler(hwnd, message, wParam, lParam);
			return(0);
		case WM_DESTROY:
			CCDebugString ("C&C95 - WM_DESTROY message received.\n");
			CCDebugString ("C&C95 - About to call Prog_End.\n");
			Prog_End();
			CCDebugString ("C&C95 - About to Invalidate_Cached_Icons.\n");
			Invalidate_Cached_Icons();
			CCDebugString ("C&C95 - About to release the video surfaces.\n");
			VisiblePage.Un_Init();
			HiddenPage.Un_Init();
			AllSurfaces.Release();
			if (!InDebugger){
				CCDebugString ("C&C95 - About to reset the video mode.\n");
				Reset_Video_Mode();
			}
			CCDebugString ("C&C95 - About to stop the profiler.\n");
			Stop_Profiler();
			CCDebugString ("C&C95 - Posting the quit message.\n");
			PostQuitMessage( 0 );
			if (ReadyToQuit){
				CCDebugString ("C&C95 - We are now ready to quit.\n");
				ReadyToQuit = 2;
			}else{
				CCDebugString ("C&C95 - Emergency shutdown.\n");
				CCDebugString ("C&C95 - Shut down the network stuff.\n");
#ifndef DEMO
				Shutdown_Network();
#endif
				if (Winsock.Get_Connected()) Winsock.Close();
				CCDebugString ("C&C95 - Call ExitProcess.\n");
				ExitProcess( 0 );
			}
	}
	return (DefWindowProc (hwnd, message, wParam, lParam));
}
```
Window procedure for the main C&C window. Handles focus messages, keyboard/mouse input, and WM_DESTROY shutdown sequence. Wrapped in `#if(0)` — the active stub would be a minimal message handler. References external: `CCFocusMessage`, `Start_Primary_Sound_Buffer`, `InMovie`, `Theme`, `OldTheme`, `Kbd`, `CCDebugString`, `Prog_End`, `Invalidate_Cached_Icons`, `VisiblePage`, `HiddenPage`, `AllSurfaces`, `InDebugger`, `Reset_Video_Mode`, `Stop_Profiler`, `PostQuitMessage`, `ReadyToQuit`, `Winsock`, `ExitProcess`, `DefWindowProc`.

### SetPalette(unsigned char *palette, long numbytes, unsigned long slowpal)
```c
void __cdecl SetPalette(unsigned char *palette,long numbytes,unsigned long slowpal)
{
	for (int i=0 ; i<256*3 ; i++){
		*(palette+i)&=63;
	}
	Increase_Palette_Luminance(palette , 15 , 15 , 15 ,63);
	if (PalettesRead){
		memcpy (&PaletteInterpolationTable[0][0] , InterpolatedPalettes[PaletteCounter++] , 65536);
	}
	Set_Palette(palette);
}
```
Palette setting function. Active implementation modifies palette entries, increases luminance, copies interpolation table, and calls Set_Palette. References external: `Increase_Palette_Luminance`, `PalettesRead`, `PaletteInterpolationTable`, `InterpolatedPalettes`, `PaletteCounter`, `Set_Palette`.

### Flag_To_Set_Palette(unsigned char *palette, long numbytes, unsigned long slowpal)
```c
void Flag_To_Set_Palette(unsigned char *palette,long numbytes,unsigned long slowpal)
{
	VQPalette = palette;
	VQNumBytes = numbytes;
	VQSlowpal = slowpal;
	VQPaletteChange = true;
}
```
Sets flags to indicate palette needs updating. Stores the palette pointer, byte count, and slowpal factor; sets the change flag. References external: `VQPalette`, `VQNumBytes`, `VQSlowpal`, `VQPaletteChange`.

### Check_VQ_Palette_Set(void)
```c
void Check_VQ_Palette_Set(void)
{
	if (VQPaletteChange){
		SetPalette(VQPalette, VQNumBytes, VQSlowpal);
		VQPaletteChange = false;
	}
}
```
Checks and applies pending VQ palette change. Calls SetPalette if flag is set, then clears the flag. References external: `VQPaletteChange`, `SetPalette`, `VQPalette`, `VQNumBytes`, `VQSlowpal`.

### Memory_Error_Handler(void)
```c
void Memory_Error_Handler(void)
{
	GlyphX_Debug_Print("Error - out of memory.");
	VisiblePage.Clear();
	Set_Palette(GamePalette);
	while (Get_Mouse_State()){Show_Mouse();}
	CCMessageBox().Process("Error - out of memory.", "Abort", false);
	Invalidate_Cached_Icons();

	// Nope. ST - 1/10/2019 10:38AM
	//PostQuitMessage( 0 );
	//ExitProcess(0);
}
```
Memory error handler. Active stub prints error, clears page, sets palette, waits for mouse release, shows message box, and invalidates icons. Full implementation would also post quit message and exit. References external: `GlyphX_Debug_Print`, `VisiblePage`, `GamePalette`, `Get_Mouse_State`, `Show_Mouse`, `CCMessageBox`, `Invalidate_Cached_Icons`, `PostQuitMessage`, `ExitProcess`.

### Read_PCX_File(char* name, char* palette, void *Buff, long Size) → GraphicBufferClass*
```c
GraphicBufferClass* Read_PCX_File(char* name, char* palette, void *Buff, long Size)
```
Full PCX file reader implementation. Reads PCX headers, decodes RLE-compressed pixel data, extracts optional palette, and returns a GraphicBufferClass. The function handles both buffered and unbuffered modes. Returns NULL if file unavailable or header validation fails. This is one of the few non-stub functions in the file.

### Load_Title_Screen(char *name, GraphicViewPortClass *video_page, unsigned char *palette)
```c
void Load_Title_Screen(char *name, GraphicViewPortClass *video_page, unsigned char *palette)
{
	GraphicBufferClass *load_buffer;

	load_buffer = Read_PCX_File (name, (char*)palette, NULL, 0);
	if (load_buffer){
		load_buffer->Blit(*video_page);
		delete load_buffer;
	}
}
```
Loads and displays a title screen PCX file. Delegates to Read_PCX_File, blits the result to the video page, and cleans up the buffer. References external: `Read_PCX_File`, `GraphicBufferClass::Blit`.

### SetPalette (lowercase, the actual implementation)
```c
void __cdecl SetPalette(unsigned char *palette,long,unsigned long)
{
	for (int i=0 ; i<256*3 ; i++){
		*(palette+i)&=63;
	}
	Increase_Palette_Luminance(palette , 15 , 15 , 15 ,63);
	if (PalettesRead){
		memcpy (&PaletteInterpolationTable[0][0] , InterpolatedPalettes[PaletteCounter++] , 65536);
	}
	Set_Palette(palette);
}
```
The lowercase `SetPalette` is the actual implementation (the uppercase `SetPalette` at file top is just the `extern "C"` declaration). Modifies palette entries, increases luminance, copies interpolation table if needed, and calls Set_Palette. References external: `Increase_Palette_Luminance`, `PalettesRead`, `PaletteInterpolationTable`, `InterpolatedPalettes`, `PaletteCounter`, `Set_Palette`.

### Colour_Debug(int call_number)
```c
void Colour_Debug(int call_number)
{
#if 0
	//if (call_number){
		//	Wait_Vert_Blank();
	//}
	// ST - 1/3/2019 10:43AM
	//if (call_number){
		//	Wait_Vert_Blank();
	//}
	//#endif
	string = string;
#endif
}
```
Debug colour function. Body is `#if 0` — a no-op `string = string;`. Would optionally wait for vertical blank and handle debug colour display. References external: `Wait_Vert_Blank`.

## Inputs, outputs and side effects

### State mutations
- `DebugFile` opened/closed for debug.txt writes
- `VQPalette`, `VQNumBytes`, `VQSlowpal`, `VQPaletteChange` set/cleared
- `OldTheme` updated on focus loss
- `ColourLookup` array read for debug colour lookup
- `MainWindow` handle set during window creation
- Palette entries modified via bitwise AND with 63
- `PaletteInterpolationTable` updated via memcpy

### External systems affected
- Window system (window creation, message pumping, focus handling)
- Palette system (colour register updates, luminance adjustment)
- Audio system (primary sound buffer start/stop, VQA audio pause/resume)
- Mouse cursor management
- Debug logging (debug.txt file I/O)
- Video surfaces (initialization, release, reset)
- Network subsystem (shutdown on WM_DESTROY)
- Keyboard input (Message_Handler routing)

## Calls into / called by

### Called by:
- DLL startup/linker — provides stub definitions for extern declarations
- Main window message loop — Windows_Procedure handles messages
- Focus management code — Focus_Loss/Focus_Restore pair
- Palette management code — SetPalette/Flag_To_Set_Palette/Check_VQ_Palette_Set
- Memory error handling code — Memory_Error_Handler
- Title screen loading — Load_Title_Screen

### Calls into:
- `Read_PCX_File` — PCX file decoding (the only full implementation in this file)
- `Set_Palette` — final palette application
- `Increase_Palette_Luminance` — luminance adjustment
- `GlyphX_Debug_Print` — debug output
- `CCMessageBox` — message box display
- `Prog_End` — program termination
- `Reset_Video_Mode` — video mode reset
- `Stop_Profiler` — profiler stop
- `Shutdown_Network` — network cleanup
- `Winsock.Close` — Winsock cleanup
- `Wait_Vert_Blank` — vertical blank synchronization
- `Get_Mouse_State` / `Show_Mouse` — mouse state management
- `Set_Palette` (global) — palette register write
- `GamePalette` — global palette reference
- `VisiblePage`, `HiddenPage` — video page management
- `AllSurfaces` — surface management
- `Winsock` — network socket management
- `Kbd` — keyboard input handler
- `CCDebugString` — debug string output
- `CCFocusMessage` — custom focus message

## Related original files
- **WINSTUB.H** - Function declarations and extern definitions
- **FUNCTION.H** - Global externs and function prototypes
- **TCPIP.H** - TCP/IP related declarations (included via `#include "tcpip.h"`)
- **THEME.H** - ThemeClass and theme management
- **PALETTE.H** - Palette functions and globals
- **MOUSE.H** - Mouse cursor and state functions
- **KEYBOARD.H** - Keyboard input handler (Kbd)
- **WINDOW.H** - Window procedure and window management
- **DEBUG.H** - Debug output functions
- **MCIBOX.H** - Message box class (CCMessageBox)
- **VQA.H** - VQ audio functions

## Rust port mapping

**Crates:** `cnc-launcher`, `cnc-tiberian-dawn`, `cnc-core`

**Mapped modules:**
- `cnc-launcher/src/winstub.rs` - Winstub stub definitions and linker bridge
- `cnc-core/src/palette.rs` - SetPalette, PaletteInterpolationTable management
- `cnc-core/src/debug.rs` - GlyphX_Debug_Print, CCDebugString equivalents
- `cnc-core/src/mouse.rs` - Mouse cursor state and clip management
- `cnc-tiberian-dawn/src/windows/proc.rs` - Windows procedure equivalent
- `cnc-tiberian-dawn/src/windows/focus.rs` - Focus loss/restore logic

**Not yet ported:**
- Watcom C linker stub semantics (empty bodies for extern satisfaction)
- `#if(0)` conditional compilation patterns
- ColourLookup array and Colour_Debug debug scaffolding
- VQPalette change flag pipeline (Flag_To_Set_Palette / Check_VQ_Palette_Set)
- Memory_Error_Handler full shutdown sequence
- Read_PCX_File PCX decoding port (handled by cnc-core graphics)
- Load_Title_Screen title screen blit pipeline
- Windows_Procedure full message dispatch and WM_DESTROY shutdown
- Create_Main_Window window creation and WNDCLASS registration
- Window_Dialog_Box dialog box modal loop
- Colour_Debug no-op debug scaffolding
- DebugFile / debug.txt I/O
- COLOURLIST struct and global ColourLookup array
- Increase_Palette_Luminance luminance function
- PaletteInterpolationTable / PaletteCounter globals
- GameInFocus, MainWindow, hInstance, SoundOn, WWMouse global state wiring
- Kbd.Message_Handler keyboard routing
- Prog_End, Reset_Video_Mode, Stop_Profiler, Shutdown_Network
- ExitProcess / PostQuitMessage Win32 API calls
- WM_MOUSEMOVE, WM_KEYDOWN, etc. message routing
- WM_DESTROY full shutdown sequence (network, Winsock, ExitProcess)
- tColourList struct definition
- 9-entry ColourLookup colour table

## Evidence and review status
Status: Reviewed
Source: original/TIBERIANDAWN/WINSTUB.CPP at f1f0d42