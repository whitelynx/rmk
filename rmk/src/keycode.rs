use num_enum::FromPrimitive;

/// Base keycodes are 8-bit keycodes, which contains:
/// 1. Keycodes defined in HID specification.
/// 2. Extra keycodes that are commonly used but not included in HID spec, such as media control keys, FNs, mouse keys, etc.
///    Keycode range 0xA5-DF and 0xE8-0xFF are used for extra keycodes, they are not used (or depreciated) by HID spec.
///    In current implementatin, 0xC3-0xCC and 0xF8-0xFD are not used.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u8)]
pub enum BaseKeyCode {
    /// Reserved, no-key.
    #[num_enum(default)]
    No = 0x00,
    /// Keyboard roll over error, too many keys are pressed simultaneously, not a physical key.
    /// NKRO: n-key rollover.
    ErrorRollover,
    /// Keyboard post fail error, not a physical key.
    PostFail,
    /// An undefined error, not a physical key.
    ErrorUndefined,

    // 0x04 - 0x1D: Letters
    /// `a` and `A`.
    A,
    /// `b` and `B`.
    B,
    /// `c` and `C`.
    C,
    /// `d` and `D`.
    D,
    /// `e` and `E`.
    E,
    /// `f` and `F`.
    F,
    /// `g` and `G`.
    G,
    /// `h` and `H`.
    H,
    /// `i` and `I`.
    I,
    /// `j` and `J`.
    J,
    /// `k` and `K`.
    K,
    /// `l` and `L`.
    L,
    /// `m` and `M`.
    M,
    /// `n` and `N`.
    N,
    /// `o` and `O`.
    O,
    /// `p` and `P`.
    P,
    /// `q` and `Q`.
    Q,
    /// `r` and `R`.
    R,
    /// `s` and `S`.
    S,
    /// `t` and `T`.
    T,
    /// `u` and `U`.
    U,
    /// `v` and `V`.
    V,
    /// `w` and `W`.
    W,
    /// `x` and `X`.
    X,
    /// `y` and `Y`.
    Y,
    /// `z` and `Z`.
    Z,

    // 0x1E - 0x27: Keyboard numbers
    /// `1` and `!`.
    Kc1,
    /// `2` and `@`.
    Kc2,
    /// `3` and `#`.
    Kc3,
    /// `4` and `$`.
    Kc4,
    /// `5` and `%`.
    Kc5,
    /// `6` and `^`.
    Kc6,
    /// `7` and `&`.
    Kc7,
    /// `8` and `*`.
    Kc8,
    /// `9` and `(`.
    Kc9,
    /// `0` and `)`.
    Kc0,

    /// Return (Enter).
    /// Note: Keyboard Enter and Keypad Enter generate different Usage codes.
    Enter,
    /// Escape.
    Escape,
    /// Delete (Backspace).
    BackSpace,
    /// Tab.
    Tab,
    /// Spacebar
    Space,
    /// `-` and `_`.
    Minus,
    /// `=` and `+`.
    Equal,
    /// `[` and `{`.
    LBracket,
    /// `]` and `}`.
    RBracket,
    /// `\` and `|`.
    BSlash,
    /// Non-US `#` and `~` (Typically near the Enter key).
    NonUsHash,
    /// `;` and `:`.
    SColon,
    /// `'` and `"`.
    Quote,
    /// \` and `~`.
    Grave,
    /// `,` and `<`.
    Comma,
    /// `.` and `>`.
    Dot,
    /// `/` and `?`.
    Slash,
    /// Caps lock.
    CapsLock,
    /// F1.
    F1,
    /// F2.
    F2,
    /// F3.
    F3,
    /// F4.
    F4,
    /// F5.
    F5,
    /// F6.
    F6,
    /// F7.
    F7,
    /// F8.
    F8,
    /// F9.
    F9,
    /// F10.
    F10,
    /// F11.
    F11,
    /// F12.
    F12,
    /// Print screen.
    PrintScreen,
    /// Scroll lock, Brightness down(macOS).
    ScrollLock,
    /// Pause, Brightness up(macOS).
    Pause,
    /// Insert.
    Insert,
    /// Home.
    Home,
    /// Page up.
    PgUp,
    /// Forward delete.
    Delete,
    /// End.
    End,
    /// Page down.
    PgDown,
    /// Right arrow.
    Right,
    /// Left arrow.
    Left,
    /// Down arrow.
    Down,
    /// Up arrow.
    Up,
    /// Keypad num lock.
    NumLock,
    /// Keypad `/`.
    KpSlash,
    /// Keypad `*`.
    KpAsterisk,
    /// Keypad `-`.
    KpMinus,
    /// Keypad `+`.
    KpPlus,
    /// Keypad Enter.
    KpEnter,
    /// Keypad `1` and End.
    Kp1,
    /// Keypad `2` and down arrow.
    Kp2,
    /// Keypad `3` and page down.
    Kp3,
    /// Keypad `4` and left arrow.
    Kp4,
    /// Keypad `5`.
    Kp5,
    /// Keypad `6` and right arrow.
    Kp6,
    /// Keypad `7` and home.
    Kp7,
    /// Keypad `8` and up arrow.
    Kp8,
    /// Keypad `9` and page up.
    Kp9,
    /// Keypad `0` and insert.
    Kp0,
    /// Keypad `.` and delete.
    KpDot,
    /// Non-US `\` and `|`.
    NonUsBSlash,
    /// Application (windows context menu key).
    Application,
    /// Reserved for keyboard power status, not a physical key.
    Power,
    /// Keypad `=`.
    KpEqual,
    /// F13.
    F13,
    /// F14.
    F14,
    /// F15.
    F15,
    /// F16.
    F16,
    /// F17.
    F17,
    /// F18.
    F18,
    /// F19.
    F19,
    /// F10.
    F20,
    /// F21.
    F21,
    /// F22.
    F22,
    /// F23.
    F23,
    /// F24.
    F24,
    /// Execute.
    Execute,
    /// Help.
    Help,
    /// Menu.
    Menu,
    /// Select.
    Select,
    /// Stop.
    Stop,
    /// Again.
    Again,
    /// Undo.
    Undo,
    /// Cut.
    Cut,
    /// Copy.
    Copy,
    /// Paste.
    Paste,
    /// Find.
    Find,
    /// Mute.
    KbMute,
    /// Volume up.
    KbVolUp,
    /// Volume down.
    KbVolDown,
    /// Deprecated.
    LockingCapsLock,
    /// Deprecated.
    LockingNumLock,
    /// Deprecated.
    LockingScrollLock,
    /// Keypad `,`.
    KpComma,
    /// Keypad `=` on AS/400 keyboards.
    KpEqualSign,
    /// JIS `\` and `_`.
    International1,
    /// JIS Katakana/Hiragana.
    International2,
    /// JIS `¥` and `|`.
    International3,
    /// JIS Henkan.
    International4,
    /// JIS Muhenkan.
    International5,
    /// JIS numpad `,`.
    International6,
    /// International 7.
    International7,
    /// International 8.
    International8,
    /// International 9.
    International9,
    /// Hangul/English.
    Language1,
    /// Hanja.
    Language2,
    /// JIS Katakana.
    Language3,
    /// JIS Hiragana.
    Language4,
    /// JIS Zenkaku/Hankaku.
    Language5,
    /// Language 6.
    Language6,
    /// Language 7.
    Language7,
    /// Language 8.
    Language8,
    /// Language 9.
    Language9,
    /// Alternate erase.
    AltErase,
    /// SysReq/Attention.
    SysReq,
    /// Cancel.
    Cancel,
    /// Clear.
    Clear,
    /// Prior.
    Prior,
    /// Return.
    Return,
    /// Separator.
    Separator,
    /// Out.
    Out,
    /// Oper.
    Oper,
    /// Clear/Again.
    ClearAgain,
    /// CrSel/Props.
    CrSel,
    /// ExSel.
    ExSel,

    // 0xA5 - 0xDF: not used on modern keyboards

    // 0xE0 - 0xE7: Modifiers
    /// Left control.
    LCtrl = 0xE0,
    /// Left shift.
    LShift,
    /// Left alt(option).
    LAlt,
    /// Left gui(widnows/command/meta key).
    LGui,
    /// Right control.
    RCtrl,
    /// Right shift.
    RShift,
    /// Right alt(option/AltGr).
    RAlt,
    /// Right gui(windows/command/meta key).
    RGui,

    // Keycodes below are extra keycodes, which are not included in HID spec

    // System control
    /// System power down.
    SystemPower = 0xA5,
    /// System sleep.
    SystemSleep,
    /// System wake.
    SystemWake,

    // Audio & media control
    /// Audio mute.
    AudioMute,
    /// Audio volume up.
    AudioVolUp,
    /// Audio volume down.
    AudioVolDown,
    /// Media next track.
    MediaNextTrack,
    /// Media previous track.
    MediaPrevTrack,
    /// Media stop.
    MediaStop,
    /// Media play/pause.
    MediaPlayPause,
    /// Eject.
    MediaEject,
    /// Launch media player.
    MediaSelect,
    /// Launch mail.
    Mail,
    /// Launch calculator.
    Calculator,
    /// Launch my computer.
    MyComputer,
    /// Browser search.
    WWWSearch,
    /// Browser home.
    WWWHome,
    /// Browser back.
    WWWBack,
    /// Browser forward.
    WWWForward,
    /// Browser stop.
    WWWStop,
    /// Browser refresh.
    WWWRefresh,
    /// Browser favorites.
    WWWFavorites,
    /// Media fast forward/next track.
    MediaFastForward,
    /// Media rewind/previous track.
    MediaRewind,
    /// Brightness up.
    BrightnessUp,
    /// Brightness down.
    BrightnessDown,
    /// Control panel
    ControlPanel,
    /// Assistant
    Assistant,
    /// Mission control
    MissionControl,
    /// Launchpad
    Launchpad,

    // 0xCD - 0xDF: Mouse keys
    /// Mouse up.
    MouseUp = 0xCD,
    /// Mouse down.
    MouseDown,
    /// Mouse left.
    MouseLeft,
    /// Mouse right.
    MouseRight,
    /// Mouse button1.
    MouseButton1,
    /// Mouse button2.
    MouseButton2,
    /// Mouse button3.
    MouseButton3,
    /// Mouse button4.
    MouseButton4,
    /// Mouse button5.
    MouseButton5,
    /// Mouse button6.
    MouseButton6,
    /// Mouse button7.
    MouseButton7,
    /// Mouse button8.
    MouseButton8,
    /// Mouse wheel up.
    MouseWheeleelUp,
    /// Mouse wheel down.
    MouseWheeleelDown,
    /// Mouse wheel left.
    MouseWheeleelLeft,
    /// Mouse wheel right.
    MouseWheeleelRight,
    /// Mouse acceleration 0.
    MouseAcceleration0,
    /// Mouse acceleration 1.
    MouseAcceleration1,
    /// Mouse acceleration 2.
    MouseAcceleration2,

    // 0xE0-E7 are Modifiers. DO NOT USE.
    /// Fn0.
    Fn0 = 0xE8,
    /// Fn1.
    Fn1,
    /// Fn2.
    Fn2,
    /// Fn3.
    Fn3,
    /// Fn4.
    Fn4,
    /// Fn5.
    Fn5,
    /// Fn6.
    Fn6,
    /// Fn7.
    Fn7,
    /// Fn8.
    Fn8,
    /// Fn9.
    Fn9,
    /// Fn10.
    Fn10,
    /// Fn11.
    Fn11,
    /// Fn12.
    Fn12,
    /// Fn13.
    Fn13,
    /// Fn14.
    Fn14,
    /// Fn15.
    Fn15,

    /// Jump to bootloader.
    Bootloader = 0xFE,
    /// Transparent key.
    Transparent = 0xFF,
}

/// KeyCode is the internal representation of all keycodes, keyboard operations, etc.
/// To be compatible with Via/Vial, most of them are same with [QMK](https://github.com/qmk/qmk_firmware/blob/master/quantum/keycodes.h)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
#[repr(u16)]
pub enum KeyCode {
    #[num_enum(default)]
    No = 0x0000,
    Transparent = 0x0001,
    A = 0x0004,
    B = 0x0005,
    C = 0x0006,
    D = 0x0007,
    E = 0x0008,
    F = 0x0009,
    G = 0x000A,
    H = 0x000B,
    I = 0x000C,
    J = 0x000D,
    K = 0x000E,
    L = 0x000F,
    M = 0x0010,
    N = 0x0011,
    O = 0x0012,
    P = 0x0013,
    Q = 0x0014,
    R = 0x0015,
    S = 0x0016,
    T = 0x0017,
    U = 0x0018,
    V = 0x0019,
    W = 0x001A,
    X = 0x001B,
    Y = 0x001C,
    Z = 0x001D,
    Kc1 = 0x001E,
    Kc2 = 0x001F,
    Kc3 = 0x0020,
    Kc4 = 0x0021,
    Kc5 = 0x0022,
    Kc6 = 0x0023,
    Kc7 = 0x0024,
    Kc8 = 0x0025,
    Kc9 = 0x0026,
    Kc0 = 0x0027,
    Enter = 0x0028,
    Escape = 0x0029,
    Backspace = 0x002A,
    Tab = 0x002B,
    Space = 0x002C,
    Minus = 0x002D,
    Equal = 0x002E,
    LeftBracket = 0x002F,
    RightBracket = 0x0030,
    Backslash = 0x0031,
    NonusHash = 0x0032,
    Semicolon = 0x0033,
    Quote = 0x0034,
    Grave = 0x0035,
    Comma = 0x0036,
    Dot = 0x0037,
    Slash = 0x0038,
    CapsLock = 0x0039,
    F1 = 0x003A,
    F2 = 0x003B,
    F3 = 0x003C,
    F4 = 0x003D,
    F5 = 0x003E,
    F6 = 0x003F,
    F7 = 0x0040,
    F8 = 0x0041,
    F9 = 0x0042,
    F10 = 0x0043,
    F11 = 0x0044,
    F12 = 0x0045,
    PrintScreen = 0x0046,
    ScrollLock = 0x0047,
    Pause = 0x0048,
    Insert = 0x0049,
    Home = 0x004A,
    PageUp = 0x004B,
    Delete = 0x004C,
    End = 0x004D,
    PageDown = 0x004E,
    Right = 0x004F,
    Left = 0x0050,
    Down = 0x0051,
    UP = 0x0052,
    NumLock = 0x0053,
    KpSlash = 0x0054,
    KpAsterisk = 0x0055,
    KpMinus = 0x0056,
    KpPlus = 0x0057,
    KpEnter = 0x0058,
    Kp1 = 0x0059,
    Kp2 = 0x005A,
    Kp3 = 0x005B,
    Kp4 = 0x005C,
    Kp5 = 0x005D,
    Kp6 = 0x005E,
    Kp7 = 0x005F,
    Kp8 = 0x0060,
    Kp9 = 0x0061,
    Kp0 = 0x0062,
    KpDot = 0x0063,
    NonusBackslash = 0x0064,
    Application = 0x0065,
    KbPower = 0x0066,
    KpEqual = 0x0067,
    F13 = 0x0068,
    F14 = 0x0069,
    F15 = 0x006A,
    F16 = 0x006B,
    F17 = 0x006C,
    F18 = 0x006D,
    F19 = 0x006E,
    F20 = 0x006F,
    F21 = 0x0070,
    F22 = 0x0071,
    F23 = 0x0072,
    F24 = 0x0073,
    Execute = 0x0074,
    Help = 0x0075,
    Menu = 0x0076,
    Select = 0x0077,
    Stop = 0x0078,
    Again = 0x0079,
    Undo = 0x007A,
    Cut = 0x007B,
    Copy = 0x007C,
    Paste = 0x007D,
    Find = 0x007E,
    KbMute = 0x007F,
    KbVolumeUp = 0x0080,
    KbVolumeDown = 0x0081,
    LockingCapsLock = 0x0082,
    LockingNumLock = 0x0083,
    LockingScrollLock = 0x0084,
    KpComma = 0x0085,
    KpEqualAs400 = 0x0086,
    International1 = 0x0087,
    International2 = 0x0088,
    International3 = 0x0089,
    International4 = 0x008A,
    International5 = 0x008B,
    International6 = 0x008C,
    International7 = 0x008D,
    International8 = 0x008E,
    International9 = 0x008F,
    Language1 = 0x0090,
    Language2 = 0x0091,
    Language3 = 0x0092,
    Language4 = 0x0093,
    Language5 = 0x0094,
    Language6 = 0x0095,
    Language7 = 0x0096,
    Language8 = 0x0097,
    Language9 = 0x0098,
    AlternateErase = 0x0099,
    SystemRequest = 0x009A,
    Cancel = 0x009B,
    Clear = 0x009C,
    Prior = 0x009D,
    Return = 0x009E,
    Separator = 0x009F,
    Out = 0x00A0,
    Oper = 0x00A1,
    ClearAgain = 0x00A2,
    Crsel = 0x00A3,
    Exsel = 0x00A4,
    SystemPower = 0x00A5,
    SystemSleep = 0x00A6,
    SystemWake = 0x00A7,
    AudioMute = 0x00A8,
    AudioVolUp = 0x00A9,
    AudioVolDown = 0x00AA,
    MediaNextTrack = 0x00AB,
    MediaPrevTrack = 0x00AC,
    MediaStop = 0x00AD,
    MediaPlayPause = 0x00AE,
    MediaSelect = 0x00AF,
    MediaEject = 0x00B0,
    Mail = 0x00B1,
    Calculator = 0x00B2,
    MyComputer = 0x00B3,
    WwwSearch = 0x00B4,
    WwwHome = 0x00B5,
    WwwBack = 0x00B6,
    WwwForward = 0x00B7,
    WwwStop = 0x00B8,
    WwwRefresh = 0x00B9,
    WwwFavorites = 0x00BA,
    MediaFastForward = 0x00BB,
    MediaRewind = 0x00BC,
    BrightnessUp = 0x00BD,
    BrightnessDown = 0x00BE,
    ControlPanel = 0x00BF,
    Assistant = 0x00C0,
    MissionControl = 0x00C1,
    Launchpad = 0x00C2,
    MouseUp = 0x00CD,
    MouseDown = 0x00CE,
    MouseLeft = 0x00CF,
    MouseRight = 0x00D0,
    MouseBtn1 = 0x00D1,
    MouseBtn2 = 0x00D2,
    MouseBtn3 = 0x00D3,
    MouseBtn4 = 0x00D4,
    MouseBtn5 = 0x00D5,
    MouseBtn6 = 0x00D6,
    MouseBtn7 = 0x00D7,
    MouseBtn8 = 0x00D8,
    MouseWheelUp = 0x00D9,
    MouseWheelDown = 0x00DA,
    MouseWheelLeft = 0x00DB,
    MouseWheelRight = 0x00DC,
    MouseAccel0 = 0x00DD,
    MouseAccel1 = 0x00DE,
    MouseAccel2 = 0x00DF,
    LCtrl = 0x00E0,
    LShift = 0x00E1,
    LAlt = 0x00E2,
    LGui = 0x00E3,
    RCtrl = 0x00E4,
    RShift = 0x00E5,
    RAlt = 0x00E6,
    RGui = 0x00E7,
    SwapHandsToggle = 0x56F0,
    SwapHandsTapToggle = 0x56F1,
    SwapHandsMomentaryOn = 0x56F2,
    SwapHandsMomentaryOff = 0x56F3,
    SwapHandsOff = 0x56F4,
    SwapHandsOn = 0x56F5,
    SwapHandsOneShot = 0x56F6,
    MagicSwapControlCapsLock = 0x7000,
    MagicUnswapControlCapsLock = 0x7001,
    MagicToggleControlCapsLock = 0x7002,
    MagicCapsLockAsControlOff = 0x7003,
    MagicCapsLockAsControlOn = 0x7004,
    MagicSwapLaltLGui = 0x7005,
    MagicUnswapLaltLGui = 0x7006,
    MagicSwapRaltRGui = 0x7007,
    MagicUnswapRaltRGui = 0x7008,
    MagicGuiOn = 0x7009,
    MagicGuiOff = 0x700A,
    MagicToggleGui = 0x700B,
    MagicSwapGraveEsc = 0x700C,
    MagicUnswapGraveEsc = 0x700D,
    MagicSwapBackslashBackspace = 0x700E,
    MagicUnswapBackslashBackspace = 0x700F,
    MagicToggleBackslashBackspace = 0x7010,
    MagicNkroOn = 0x7011,
    MagicNkroOff = 0x7012,
    MagicToggleNkro = 0x7013,
    MagicSwapAltGui = 0x7014,
    MagicUnswapAltGui = 0x7015,
    MagicToggleAltGui = 0x7016,
    MagicSwapLctlLGui = 0x7017,
    MagicUnswapLctlLGui = 0x7018,
    MagicSwapRctlRGui = 0x7019,
    MagicUnswapRctlRGui = 0x701A,
    MagicSwapCtlGui = 0x701B,
    MagicUnswapCtlGui = 0x701C,
    MagicToggleCtlGui = 0x701D,
    MagicEeHandsLeft = 0x701E,
    MagicEeHandsRight = 0x701F,
    MagicSwapEscapeCapsLock = 0x7020,
    MagicUnswapEscapeCapsLock = 0x7021,
    MagicToggleEscapeCapsLock = 0x7022,
    MidiOn = 0x7100,
    MidiOff = 0x7101,
    MidiToggle = 0x7102,
    MidiNoteC0 = 0x7103,
    MidiNoteCSharp0 = 0x7104,
    MidiNoteD0 = 0x7105,
    MidiNoteDSharp0 = 0x7106,
    MidiNoteE0 = 0x7107,
    MidiNoteF0 = 0x7108,
    MidiNoteFSharp0 = 0x7109,
    MidiNoteG0 = 0x710A,
    MidiNoteGSharp0 = 0x710B,
    MidiNoteA0 = 0x710C,
    MidiNoteASharp0 = 0x710D,
    MidiNoteB0 = 0x710E,
    MidiNoteC1 = 0x710F,
    MidiNoteCSharp1 = 0x7110,
    MidiNoteD1 = 0x7111,
    MidiNoteDSharp1 = 0x7112,
    MidiNoteE1 = 0x7113,
    MidiNoteF1 = 0x7114,
    MidiNoteFSharp1 = 0x7115,
    MidiNoteG1 = 0x7116,
    MidiNoteGSharp1 = 0x7117,
    MidiNoteA1 = 0x7118,
    MidiNoteASharp1 = 0x7119,
    MidiNoteB1 = 0x711A,
    MidiNoteC2 = 0x711B,
    MidiNoteCSharp2 = 0x711C,
    MidiNoteD2 = 0x711D,
    MidiNoteDSharp2 = 0x711E,
    MidiNoteE2 = 0x711F,
    MidiNoteF2 = 0x7120,
    MidiNoteFSharp2 = 0x7121,
    MidiNoteG2 = 0x7122,
    MidiNoteGSharp2 = 0x7123,
    MidiNoteA2 = 0x7124,
    MidiNoteASharp2 = 0x7125,
    MidiNoteB2 = 0x7126,
    MidiNoteC3 = 0x7127,
    MidiNoteCSharp3 = 0x7128,
    MidiNoteD3 = 0x7129,
    MidiNoteDSharp3 = 0x712A,
    MidiNoteE3 = 0x712B,
    MidiNoteF3 = 0x712C,
    MidiNoteFSharp3 = 0x712D,
    MidiNoteG3 = 0x712E,
    MidiNoteGSharp3 = 0x712F,
    MidiNoteA3 = 0x7130,
    MidiNoteASharp3 = 0x7131,
    MidiNoteB3 = 0x7132,
    MidiNoteC4 = 0x7133,
    MidiNoteCSharp4 = 0x7134,
    MidiNoteD4 = 0x7135,
    MidiNoteDSharp4 = 0x7136,
    MidiNoteE4 = 0x7137,
    MidiNoteF4 = 0x7138,
    MidiNoteFSharp4 = 0x7139,
    MidiNoteG4 = 0x713A,
    MidiNoteGSharp4 = 0x713B,
    MidiNoteA4 = 0x713C,
    MidiNoteASharp4 = 0x713D,
    MidiNoteB4 = 0x713E,
    MidiNoteC5 = 0x713F,
    MidiNoteCSharp5 = 0x7140,
    MidiNoteD5 = 0x7141,
    MidiNoteDSharp5 = 0x7142,
    MidiNoteE5 = 0x7143,
    MidiNoteF5 = 0x7144,
    MidiNoteFSharp5 = 0x7145,
    MidiNoteG5 = 0x7146,
    MidiNoteGSharp5 = 0x7147,
    MidiNoteA5 = 0x7148,
    MidiNoteASharp5 = 0x7149,
    MidiNoteB5 = 0x714A,
    MidiOctaveN2 = 0x714B,
    MidiOctaveN1 = 0x714C,
    MidiOctave0 = 0x714D,
    MidiOctave1 = 0x714E,
    MidiOctave2 = 0x714F,
    MidiOctave3 = 0x7150,
    MidiOctave4 = 0x7151,
    MidiOctave5 = 0x7152,
    MidiOctave6 = 0x7153,
    MidiOctave7 = 0x7154,
    MidiOctaveDOWN = 0x7155,
    MidiOctaveUP = 0x7156,
    MidiTransposeN6 = 0x7157,
    MidiTransposeN5 = 0x7158,
    MidiTransposeN4 = 0x7159,
    MidiTransposeN3 = 0x715A,
    MidiTransposeN2 = 0x715B,
    MidiTransposeN1 = 0x715C,
    MidiTranspose0 = 0x715D,
    MidiTranspose1 = 0x715E,
    MidiTranspose2 = 0x715F,
    MidiTranspose3 = 0x7160,
    MidiTranspose4 = 0x7161,
    MidiTranspose5 = 0x7162,
    MidiTranspose6 = 0x7163,
    MidiTransposeDown = 0x7164,
    MidiTransposeUp = 0x7165,
    MidiVelocity0 = 0x7166,
    MidiVelocity1 = 0x7167,
    MidiVelocity2 = 0x7168,
    MidiVelocity3 = 0x7169,
    MidiVelocity4 = 0x716A,
    MidiVelocity5 = 0x716B,
    MidiVelocity6 = 0x716C,
    MidiVelocity7 = 0x716D,
    MidiVelocity8 = 0x716E,
    MidiVelocity9 = 0x716F,
    MidiVelocity10 = 0x7170,
    MidiVelocityDOWN = 0x7171,
    MidiVelocityUP = 0x7172,
    MidiChannel1 = 0x7173,
    MidiChannel2 = 0x7174,
    MidiChannel3 = 0x7175,
    MidiChannel4 = 0x7176,
    MidiChannel5 = 0x7177,
    MidiChannel6 = 0x7178,
    MidiChannel7 = 0x7179,
    MidiChannel8 = 0x717A,
    MidiChannel9 = 0x717B,
    MidiChannel10 = 0x717C,
    MidiChannel11 = 0x717D,
    MidiChannel12 = 0x717E,
    MidiChannel13 = 0x717F,
    MidiChannel14 = 0x7180,
    MidiChannel15 = 0x7181,
    MidiChannel16 = 0x7182,
    MidiChannelDOWN = 0x7183,
    MidiChannelUP = 0x7184,
    MidiAllNotesOff = 0x7185,
    MidiSustain = 0x7186,
    MidiPortamento = 0x7187,
    MidiSostenuto = 0x7188,
    MidiSoft = 0x7189,
    MidiLegato = 0x718A,
    MidiModulation = 0x718B,
    MidiModulationSpeedDown = 0x718C,
    MidiModulationSpeedUp = 0x718D,
    MidiPitchBendDown = 0x718E,
    MidiPitchBendUp = 0x718F,
    SequencerOn = 0x7200,
    SequencerOff = 0x7201,
    SequencerToggle = 0x7202,
    SequencerTempoDown = 0x7203,
    SequencerTempoUp = 0x7204,
    SequencerResolutionDown = 0x7205,
    SequencerResolutionUp = 0x7206,
    SequencerStepsAll = 0x7207,
    SequencerStepsClear = 0x7208,
    JoystickButton0 = 0x7400,
    JoystickButton1 = 0x7401,
    JoystickButton2 = 0x7402,
    JoystickButton3 = 0x7403,
    JoystickButton4 = 0x7404,
    JoystickButton5 = 0x7405,
    JoystickButton6 = 0x7406,
    JoystickButton7 = 0x7407,
    JoystickButton8 = 0x7408,
    JoystickButton9 = 0x7409,
    JoystickButton10 = 0x740A,
    JoystickButton11 = 0x740B,
    JoystickButton12 = 0x740C,
    JoystickButton13 = 0x740D,
    JoystickButton14 = 0x740E,
    JoystickButton15 = 0x740F,
    JoystickButton16 = 0x7410,
    JoystickButton17 = 0x7411,
    JoystickButton18 = 0x7412,
    JoystickButton19 = 0x7413,
    JoystickButton20 = 0x7414,
    JoystickButton21 = 0x7415,
    JoystickButton22 = 0x7416,
    JoystickButton23 = 0x7417,
    JoystickButton24 = 0x7418,
    JoystickButton25 = 0x7419,
    JoystickButton26 = 0x741A,
    JoystickButton27 = 0x741B,
    JoystickButton28 = 0x741C,
    JoystickButton29 = 0x741D,
    JoystickButton30 = 0x741E,
    JoystickButton31 = 0x741F,
    ProgrammableButton1 = 0x7440,
    ProgrammableButton2 = 0x7441,
    ProgrammableButton3 = 0x7442,
    ProgrammableButton4 = 0x7443,
    ProgrammableButton5 = 0x7444,
    ProgrammableButton6 = 0x7445,
    ProgrammableButton7 = 0x7446,
    ProgrammableButton8 = 0x7447,
    ProgrammableButton9 = 0x7448,
    ProgrammableButton10 = 0x7449,
    ProgrammableButton11 = 0x744A,
    ProgrammableButton12 = 0x744B,
    ProgrammableButton13 = 0x744C,
    ProgrammableButton14 = 0x744D,
    ProgrammableButton15 = 0x744E,
    ProgrammableButton16 = 0x744F,
    ProgrammableButton17 = 0x7450,
    ProgrammableButton18 = 0x7451,
    ProgrammableButton19 = 0x7452,
    ProgrammableButton20 = 0x7453,
    ProgrammableButton21 = 0x7454,
    ProgrammableButton22 = 0x7455,
    ProgrammableButton23 = 0x7456,
    ProgrammableButton24 = 0x7457,
    ProgrammableButton25 = 0x7458,
    ProgrammableButton26 = 0x7459,
    ProgrammableButton27 = 0x745A,
    ProgrammableButton28 = 0x745B,
    ProgrammableButton29 = 0x745C,
    ProgrammableButton30 = 0x745D,
    ProgrammableButton31 = 0x745E,
    ProgrammableButton32 = 0x745F,
    AudioOn = 0x7480,
    AudioOff = 0x7481,
    AudioToggle = 0x7482,
    AudioClickyToggle = 0x748A,
    AudioClickyOn = 0x748B,
    AudioClickyOff = 0x748C,
    AudioClickyUp = 0x748D,
    AudioClickyDown = 0x748E,
    AudioClickyReset = 0x748F,
    MusicOn = 0x7490,
    MusicOff = 0x7491,
    MusicToggle = 0x7492,
    MusicModeNext = 0x7493,
    AudioVoiceNext = 0x7494,
    AudioVoicePrevious = 0x7495,
    StenoBolt = 0x74F0,
    StenoGemini = 0x74F1,
    StenoComb = 0x74F2,
    StenoCombMax = 0x74FC,
    Macro0 = 0x7700,
    Macro1 = 0x7701,
    Macro2 = 0x7702,
    Macro3 = 0x7703,
    Macro4 = 0x7704,
    Macro5 = 0x7705,
    Macro6 = 0x7706,
    Macro7 = 0x7707,
    Macro8 = 0x7708,
    Macro9 = 0x7709,
    Macro10 = 0x770A,
    Macro11 = 0x770B,
    Macro12 = 0x770C,
    Macro13 = 0x770D,
    Macro14 = 0x770E,
    Macro15 = 0x770F,
    Macro16 = 0x7710,
    Macro17 = 0x7711,
    Macro18 = 0x7712,
    Macro19 = 0x7713,
    Macro20 = 0x7714,
    Macro21 = 0x7715,
    Macro22 = 0x7716,
    Macro23 = 0x7717,
    Macro24 = 0x7718,
    Macro25 = 0x7719,
    Macro26 = 0x771A,
    Macro27 = 0x771B,
    Macro28 = 0x771C,
    Macro29 = 0x771D,
    Macro30 = 0x771E,
    Macro31 = 0x771F,
    BacklightOn = 0x7800,
    BacklightOff = 0x7801,
    BacklightToggle = 0x7802,
    BacklightDown = 0x7803,
    BacklightUp = 0x7804,
    BacklightStep = 0x7805,
    BacklightToggleBreathing = 0x7806,
    RgbTog = 0x7820,
    RgbModeForward = 0x7821,
    RgbModeReverse = 0x7822,
    RgbHui = 0x7823,
    RgbHud = 0x7824,
    RgbSai = 0x7825,
    RgbSad = 0x7826,
    RgbVai = 0x7827,
    RgbVad = 0x7828,
    RgbSpi = 0x7829,
    RgbSpd = 0x782A,
    RgbModePlain = 0x782B,
    RgbModeBreathe = 0x782C,
    RgbModeRainbow = 0x782D,
    RgbModeSwirl = 0x782E,
    RgbModeSnake = 0x782F,
    RgbModeKnight = 0x7830,
    RgbModeXmas = 0x7831,
    RgbModeGradient = 0x7832,
    RgbModeRgbtest = 0x7833,
    RgbModeTwinkle = 0x7834,
    Bootloader = 0x7C00,
    Reboot = 0x7C01,
    DebugToggle = 0x7C02,
    ClearEeprom = 0x7C03,
    Make = 0x7C04,
    AutoShiftDown = 0x7C10,
    AutoShiftUp = 0x7C11,
    AutoShiftReport = 0x7C12,
    AutoShiftOn = 0x7C13,
    AutoShiftOff = 0x7C14,
    AutoShiftToggle = 0x7C15,
    GraveEscape = 0x7C16,
    VelocikeyToggle = 0x7C17,
    SpaceCadetLCtrlParenthesisOpen = 0x7C18,
    SpaceCadetRCtrlParenthesisClose = 0x7C19,
    SpaceCadetLShiftParenthesisOpen = 0x7C1A,
    SpaceCadetRShiftParenthesisClose = 0x7C1B,
    SpaceCadetLAltParenthesisOpen = 0x7C1C,
    SpaceCadetRAltParenthesisClose = 0x7C1D,
    SpaceCadetRShiftEnter = 0x7C1E,
    OutputAuto = 0x7C20,
    OutputUsb = 0x7C21,
    OutputBluetooth = 0x7C22,
    UnicodeModeNext = 0x7C30,
    UnicodeModePrevious = 0x7C31,
    UnicodeModeMacos = 0x7C32,
    UnicodeModeLinux = 0x7C33,
    UnicodeModeWindows = 0x7C34,
    UnicodeModeBsd = 0x7C35,
    UnicodeModeWincompose = 0x7C36,
    UnicodeModeEmacs = 0x7C37,
    HapticOn = 0x7C40,
    HapticOff = 0x7C41,
    HapticToggle = 0x7C42,
    HapticReset = 0x7C43,
    HapticFeedbackToggle = 0x7C44,
    HapticBuzzToggle = 0x7C45,
    HapticModeNext = 0x7C46,
    HapticModePrevious = 0x7C47,
    HapticContinuousToggle = 0x7C48,
    HapticContinuousUp = 0x7C49,
    HapticContinuousDown = 0x7C4A,
    HapticDwellUp = 0x7C4B,
    HapticDwellDown = 0x7C4C,
    ComboOn = 0x7C50,
    ComboOff = 0x7C51,
    ComboToggle = 0x7C52,
    DynamicMacroRecordStart1 = 0x7C53,
    DynamicMacroRecordStart2 = 0x7C54,
    DynamicMacroRecordStop = 0x7C55,
    DynamicMacroPlay1 = 0x7C56,
    DynamicMacroPlay2 = 0x7C57,
    Leader = 0x7C58,
    Lock = 0x7C59,
    OneShotOn = 0x7C5A,
    OneShotOff = 0x7C5B,
    OneShotToggle = 0x7C5C,
    KeyOverrideToggle = 0x7C5D,
    KeyOverrideOn = 0x7C5E,
    KeyOverrideOff = 0x7C5F,
    SecureLock = 0x7C60,
    SecureUnlock = 0x7C61,
    SecureToggle = 0x7C62,
    SecureRequest = 0x7C63,
    DynamicTappingTermPrint = 0x7C70,
    DynamicTappingTermUp = 0x7C71,
    DynamicTappingTermDown = 0x7C72,
    CapsWordToggle = 0x7C73,
    AutocorrectOn = 0x7C74,
    AutocorrectOff = 0x7C75,
    AutocorrectToggle = 0x7C76,
    TriLayerLower = 0x7C77,
    TriLayerUpper = 0x7C78,
    RepeatKey = 0x7C79,
    AltRepeatKey = 0x7C7A,
    Kb0 = 0x7E00,
    Kb1 = 0x7E01,
    Kb2 = 0x7E02,
    Kb3 = 0x7E03,
    Kb4 = 0x7E04,
    Kb5 = 0x7E05,
    Kb6 = 0x7E06,
    Kb7 = 0x7E07,
    Kb8 = 0x7E08,
    Kb9 = 0x7E09,
    Kb10 = 0x7E0A,
    Kb11 = 0x7E0B,
    Kb12 = 0x7E0C,
    Kb13 = 0x7E0D,
    Kb14 = 0x7E0E,
    Kb15 = 0x7E0F,
    Kb16 = 0x7E10,
    Kb17 = 0x7E11,
    Kb18 = 0x7E12,
    Kb19 = 0x7E13,
    Kb20 = 0x7E14,
    Kb21 = 0x7E15,
    Kb22 = 0x7E16,
    Kb23 = 0x7E17,
    Kb24 = 0x7E18,
    Kb25 = 0x7E19,
    Kb26 = 0x7E1A,
    Kb27 = 0x7E1B,
    Kb28 = 0x7E1C,
    Kb29 = 0x7E1D,
    Kb30 = 0x7E1E,
    Kb31 = 0x7E1F,
    User0 = 0x7E40,
    User1 = 0x7E41,
    User2 = 0x7E42,
    User3 = 0x7E43,
    User4 = 0x7E44,
    User5 = 0x7E45,
    User6 = 0x7E46,
    User7 = 0x7E47,
    User8 = 0x7E48,
    User9 = 0x7E49,
    User10 = 0x7E4A,
    User11 = 0x7E4B,
    User12 = 0x7E4C,
    User13 = 0x7E4D,
    User14 = 0x7E4E,
    User15 = 0x7E4F,
    User16 = 0x7E50,
    User17 = 0x7E51,
    User18 = 0x7E52,
    User19 = 0x7E53,
    User20 = 0x7E54,
    User21 = 0x7E55,
    User22 = 0x7E56,
    User23 = 0x7E57,
    User24 = 0x7E58,
    User25 = 0x7E59,
    User26 = 0x7E5A,
    User27 = 0x7E5B,
    User28 = 0x7E5C,
    User29 = 0x7E5D,
    User30 = 0x7E5E,
    User31 = 0x7E5F,
}

impl BaseKeyCode {
    /// Returns `true` if the keycode corresponds to a modifier (sent
    /// separately on the USB HID report).
    pub fn is_modifier(self) -> bool {
        BaseKeyCode::LCtrl <= self && self <= BaseKeyCode::RGui
    }

    /// Returns the byte with the bit corresponding to the USB HID
    /// modifier bitfield set.
    pub fn as_modifier_bit(self) -> u8 {
        if self.is_modifier() {
            1 << (self as u8 - BaseKeyCode::LCtrl as u8)
        } else {
            0
        }
    }

    /// Returns `true` if the keycode is a keycode in consumer page
    /// Note: For windows, the last keycode in consumer page is BrightnessDown. We use Launchpad to be compatible with macos.
    pub fn is_consumer(self) -> bool {
        BaseKeyCode::AudioMute <= self && self <= BaseKeyCode::Launchpad
    }

    /// Returns `true` if the keycode is a mouse key which sent by
    /// separate mouse report
    pub fn is_mouse_key(self) -> bool {
        BaseKeyCode::MouseUp <= self && self <= BaseKeyCode::MouseAcceleration2
    }
}

impl KeyCode {
    /// Returns `true` if the keycode is basic keycode
    pub fn is_basic(self) -> bool {
        KeyCode::No <= self && self <= KeyCode::RGui
    }

    /// Returns `true` if the keycode is a modifier keycode
    pub fn is_modifier(self) -> bool {
        KeyCode::LCtrl <= self && self <= KeyCode::RGui
    }

    /// Returns `true` if the keycode is a system keycode
    pub fn is_system(self) -> bool {
        KeyCode::SystemPower <= self && self <= KeyCode::SystemWake
    }

    /// Returns `true` if the keycode is a keycode in consumer page
    pub fn is_consumer(self) -> bool {
        KeyCode::AudioMute <= self && self <= KeyCode::Launchpad
    }

    /// Returns `true` if the keycode is a mouse keycode
    pub fn is_mouse_key(self) -> bool {
        KeyCode::MouseUp <= self && self <= KeyCode::MouseAccel2
    }

    /// Returns `true` if the keycode is a swap hands keycode
    pub fn is_swap_hands(self) -> bool {
        KeyCode::SwapHandsToggle <= self && self <= KeyCode::SwapHandsOneShot
    }

    /// Returns `true` if the keycode is a magic keycode
    pub fn is_magic(self) -> bool {
        KeyCode::MagicSwapControlCapsLock <= self && self <= KeyCode::MagicToggleEscapeCapsLock
    }

    /// Returns `true` if the keycode is a midi keycode
    pub fn is_midi(self) -> bool {
        KeyCode::MidiOn <= self && self <= KeyCode::MidiPitchBendUp
    }

    /// Returns `true` if the keycode is a sequencer keycode
    pub fn is_sequencer(self) -> bool {
        KeyCode::SequencerOn <= self && self <= KeyCode::SequencerStepsClear
    }

    /// Returns `true` if the keycode is a joystick keycode
    pub fn is_joystick(self) -> bool {
        KeyCode::JoystickButton0 <= self && self <= KeyCode::JoystickButton31
    }

    /// Returns `true` if the keycode is a programmable button keycode
    pub fn is_programmable_button(self) -> bool {
        KeyCode::ProgrammableButton1 <= self && self <= KeyCode::ProgrammableButton32
    }

    /// Returns `true` if the keycode is a audio keycode
    /// Note: Basic audio keycodes are not included
    pub fn is_audio(self) -> bool {
        KeyCode::AudioOn <= self && self <= KeyCode::AudioVoicePrevious
    }

    /// Returns `true` if the keycode is a steno keycode
    pub fn is_steno(self) -> bool {
        KeyCode::StenoBolt <= self && self <= KeyCode::StenoCombMax
    }

    /// Returns `true` if the keycode is a macro keycode
    pub fn is_macro(self) -> bool {
        KeyCode::Macro0 <= self && self <= KeyCode::Macro31
    }

    /// Returns `true` if the keycode is a backlight keycode
    pub fn is_backlight(self) -> bool {
        KeyCode::BacklightOn <= self && self <= KeyCode::BacklightToggleBreathing
    }

    /// Returns `true` if the keycode is a rgb keycode
    pub fn is_rgb(self) -> bool {
        KeyCode::RgbTog <= self && self <= KeyCode::RgbModeTwinkle
    }

    /// Returns `true` if the keycode is defined by rmk to achieve special functionalities, such as reboot keyboard, goto bootloader, etc.
    pub fn is_rmk(self) -> bool {
        KeyCode::Bootloader <= self && self <= KeyCode::AltRepeatKey
    }

    /// Returns `true` if the keycode is a kb keycode
    pub fn is_kb(self) -> bool {
        KeyCode::Kb0 <= self && self <= KeyCode::Kb31
    }

    /// Returns `true` if the keycode is a user keycode
    pub fn is_user(self) -> bool {
        KeyCode::User0 <= self && self <= KeyCode::User31
    }

    pub fn to_base_keycode(self) -> BaseKeyCode {
        if self.is_basic() {
            BaseKeyCode::from_primitive(self as u8)
        } else {
            BaseKeyCode::No
        }
    }

    pub fn from_base_keycode(keycode: BaseKeyCode) -> Self {
        KeyCode::from_primitive(keycode as u8 as u16)
    }
}
