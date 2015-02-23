use std::hash::{Hash, Hasher};
use std::num::ToPrimitive;

use sys::keycode as ll;

#[derive(PartialEq, Eq, FromPrimitive, Debug, Copy)]
pub enum KeyCode {
    Unknown            = ll::SDLK_UNKNOWN as isize,
    Backspace          = ll::SDLK_BACKSPACE as isize,
    Tab                = ll::SDLK_TAB as isize,
    Return             = ll::SDLK_RETURN as isize,
    Escape             = ll::SDLK_ESCAPE as isize,
    Space              = ll::SDLK_SPACE as isize,
    Exclaim            = ll::SDLK_EXCLAIM as isize,
    Quotedbl           = ll::SDLK_QUOTEDBL as isize,
    Hash               = ll::SDLK_HASH as isize,
    Dollar             = ll::SDLK_DOLLAR as isize,
    Percent            = ll::SDLK_PERCENT as isize,
    Ampersand          = ll::SDLK_AMPERSAND as isize,
    Quote              = ll::SDLK_QUOTE as isize,
    LeftParen          = ll::SDLK_LEFTPAREN as isize,
    RightParen         = ll::SDLK_RIGHTPAREN as isize,
    Asterisk           = ll::SDLK_ASTERISK as isize,
    Plus               = ll::SDLK_PLUS as isize,
    Comma              = ll::SDLK_COMMA as isize,
    Minus              = ll::SDLK_MINUS as isize,
    Period             = ll::SDLK_PERIOD as isize,
    Slash              = ll::SDLK_SLASH as isize,
    Num0               = ll::SDLK_0 as isize,
    Num1               = ll::SDLK_1 as isize,
    Num2               = ll::SDLK_2 as isize,
    Num3               = ll::SDLK_3 as isize,
    Num4               = ll::SDLK_4 as isize,
    Num5               = ll::SDLK_5 as isize,
    Num6               = ll::SDLK_6 as isize,
    Num7               = ll::SDLK_7 as isize,
    Num8               = ll::SDLK_8 as isize,
    Num9               = ll::SDLK_9 as isize,
    Colon              = ll::SDLK_COLON as isize,
    Semicolon          = ll::SDLK_SEMICOLON as isize,
    Less               = ll::SDLK_LESS as isize,
    Equals             = ll::SDLK_EQUALS as isize,
    Greater            = ll::SDLK_GREATER as isize,
    Question           = ll::SDLK_QUESTION as isize,
    At                 = ll::SDLK_AT as isize,
    LeftBracket        = ll::SDLK_LEFTBRACKET as isize,
    Backslash          = ll::SDLK_BACKSLASH as isize,
    RightBracket       = ll::SDLK_RIGHTBRACKET as isize,
    Caret              = ll::SDLK_CARET as isize,
    Underscore         = ll::SDLK_UNDERSCORE as isize,
    Backquote          = ll::SDLK_BACKQUOTE as isize,
    A                  = ll::SDLK_a as isize,
    B                  = ll::SDLK_b as isize,
    C                  = ll::SDLK_c as isize,
    D                  = ll::SDLK_d as isize,
    E                  = ll::SDLK_e as isize,
    F                  = ll::SDLK_f as isize,
    G                  = ll::SDLK_g as isize,
    H                  = ll::SDLK_h as isize,
    I                  = ll::SDLK_i as isize,
    J                  = ll::SDLK_j as isize,
    K                  = ll::SDLK_k as isize,
    L                  = ll::SDLK_l as isize,
    M                  = ll::SDLK_m as isize,
    N                  = ll::SDLK_n as isize,
    O                  = ll::SDLK_o as isize,
    P                  = ll::SDLK_p as isize,
    Q                  = ll::SDLK_q as isize,
    R                  = ll::SDLK_r as isize,
    S                  = ll::SDLK_s as isize,
    T                  = ll::SDLK_t as isize,
    U                  = ll::SDLK_u as isize,
    V                  = ll::SDLK_v as isize,
    W                  = ll::SDLK_w as isize,
    X                  = ll::SDLK_x as isize,
    Y                  = ll::SDLK_y as isize,
    Z                  = ll::SDLK_z as isize,
    Delete             = ll::SDLK_DELETE as isize,
    CapsLock           = ll::SDLK_CAPSLOCK as isize,
    F1                 = ll::SDLK_F1 as isize,
    F2                 = ll::SDLK_F2 as isize,
    F3                 = ll::SDLK_F3 as isize,
    F4                 = ll::SDLK_F4 as isize,
    F5                 = ll::SDLK_F5 as isize,
    F6                 = ll::SDLK_F6 as isize,
    F7                 = ll::SDLK_F7 as isize,
    F8                 = ll::SDLK_F8 as isize,
    F9                 = ll::SDLK_F9 as isize,
    F10                = ll::SDLK_F10 as isize,
    F11                = ll::SDLK_F11 as isize,
    F12                = ll::SDLK_F12 as isize,
    PrintScreen        = ll::SDLK_PRINTSCREEN as isize,
    ScrollLock         = ll::SDLK_SCROLLLOCK as isize,
    Pause              = ll::SDLK_PAUSE as isize,
    Insert             = ll::SDLK_INSERT as isize,
    Home               = ll::SDLK_HOME as isize,
    PageUp             = ll::SDLK_PAGEUP as isize,
    End                = ll::SDLK_END as isize,
    PageDown           = ll::SDLK_PAGEDOWN as isize,
    Right              = ll::SDLK_RIGHT as isize,
    Left               = ll::SDLK_LEFT as isize,
    Down               = ll::SDLK_DOWN as isize,
    Up                 = ll::SDLK_UP as isize,
    NumLockClear       = ll::SDLK_NUMLOCKCLEAR as isize,
    KpDivide           = ll::SDLK_KPDIVIDE as isize,
    KpMultiply         = ll::SDLK_KPMULTIPLY as isize,
    KpMinus            = ll::SDLK_KPMINUS as isize,
    KpPlus             = ll::SDLK_KPPLUS as isize,
    KpEnter            = ll::SDLK_KPENTER as isize,
    Kp1                = ll::SDLK_KP1 as isize,
    Kp2                = ll::SDLK_KP2 as isize,
    Kp3                = ll::SDLK_KP3 as isize,
    Kp4                = ll::SDLK_KP4 as isize,
    Kp5                = ll::SDLK_KP5 as isize,
    Kp6                = ll::SDLK_KP6 as isize,
    Kp7                = ll::SDLK_KP7 as isize,
    Kp8                = ll::SDLK_KP8 as isize,
    Kp9                = ll::SDLK_KP9 as isize,
    Kp0                = ll::SDLK_KP0 as isize,
    KpPeriod           = ll::SDLK_KPPERIOD as isize,
    Application        = ll::SDLK_APPLICATION as isize,
    Power              = ll::SDLK_POWER as isize,
    KpEquals           = ll::SDLK_KPEQUALS as isize,
    F13                = ll::SDLK_F13 as isize,
    F14                = ll::SDLK_F14 as isize,
    F15                = ll::SDLK_F15 as isize,
    F16                = ll::SDLK_F16 as isize,
    F17                = ll::SDLK_F17 as isize,
    F18                = ll::SDLK_F18 as isize,
    F19                = ll::SDLK_F19 as isize,
    F20                = ll::SDLK_F20 as isize,
    F21                = ll::SDLK_F21 as isize,
    F22                = ll::SDLK_F22 as isize,
    F23                = ll::SDLK_F23 as isize,
    F24                = ll::SDLK_F24 as isize,
    Execute            = ll::SDLK_EXECUTE as isize,
    Help               = ll::SDLK_HELP as isize,
    Menu               = ll::SDLK_MENU as isize,
    Select             = ll::SDLK_SELECT as isize,
    Stop               = ll::SDLK_STOP as isize,
    Again              = ll::SDLK_AGAIN as isize,
    Undo               = ll::SDLK_UNDO as isize,
    Cut                = ll::SDLK_CUT as isize,
    Copy               = ll::SDLK_COPY as isize,
    Paste              = ll::SDLK_PASTE as isize,
    Find               = ll::SDLK_FIND as isize,
    Mute               = ll::SDLK_MUTE as isize,
    VolumeUp           = ll::SDLK_VOLUMEUP as isize,
    VolumeDown         = ll::SDLK_VOLUMEDOWN as isize,
    KpComma            = ll::SDLK_KPCOMMA as isize,
    KpEqualsAS400      = ll::SDLK_KPEQUALSAS400 as isize,
    AltErase           = ll::SDLK_ALTERASE as isize,
    Sysreq             = ll::SDLK_SYSREQ as isize,
    Cancel             = ll::SDLK_CANCEL as isize,
    Clear              = ll::SDLK_CLEAR as isize,
    Prior              = ll::SDLK_PRIOR as isize,
    Return2            = ll::SDLK_RETURN2 as isize,
    Separator          = ll::SDLK_SEPARATOR as isize,
    Out                = ll::SDLK_OUT as isize,
    Oper               = ll::SDLK_OPER as isize,
    ClearAgain         = ll::SDLK_CLEARAGAIN as isize,
    CrSel              = ll::SDLK_CRSEL as isize,
    ExSel              = ll::SDLK_EXSEL as isize,
    Kp00               = ll::SDLK_KP00 as isize,
    Kp000              = ll::SDLK_KP000 as isize,
    ThousandsSeparator = ll::SDLK_THOUSANDSSEPARATOR as isize,
    DecimalSeparator   = ll::SDLK_DECIMALSEPARATOR as isize,
    CurrencyUnit       = ll::SDLK_CURRENCYUNIT as isize,
    CurrencySubUnit    = ll::SDLK_CURRENCYSUBUNIT as isize,
    KpLeftParen        = ll::SDLK_KPLEFTPAREN as isize,
    KpRightParen       = ll::SDLK_KPRIGHTPAREN as isize,
    KpLeftBrace        = ll::SDLK_KPLEFTBRACE as isize,
    KpRightBrace       = ll::SDLK_KPRIGHTBRACE as isize,
    KpTab              = ll::SDLK_KPTAB as isize,
    KpBackspace        = ll::SDLK_KPBACKSPACE as isize,
    KpA                = ll::SDLK_KPA as isize,
    KpB                = ll::SDLK_KPB as isize,
    KpC                = ll::SDLK_KPC as isize,
    KpD                = ll::SDLK_KPD as isize,
    KpE                = ll::SDLK_KPE as isize,
    KpF                = ll::SDLK_KPF as isize,
    KpXor              = ll::SDLK_KPXOR as isize,
    KpPower            = ll::SDLK_KPPOWER as isize,
    KpPercent          = ll::SDLK_KPPERCENT as isize,
    KpLess             = ll::SDLK_KPLESS as isize,
    KpGreater          = ll::SDLK_KPGREATER as isize,
    KpAmpersand        = ll::SDLK_KPAMPERSAND as isize,
    KpDblAmpersand     = ll::SDLK_KPDBLAMPERSAND as isize,
    KpVerticalBar      = ll::SDLK_KPVERTICALBAR as isize,
    KpDblVerticalBar   = ll::SDLK_KPDBLVERTICALBAR as isize,
    KpColon            = ll::SDLK_KPCOLON as isize,
    KpHash             = ll::SDLK_KPHASH as isize,
    KpSpace            = ll::SDLK_KPSPACE as isize,
    KpAt               = ll::SDLK_KPAT as isize,
    KpExclam           = ll::SDLK_KPEXCLAM as isize,
    KpMemStore         = ll::SDLK_KPMEMSTORE as isize,
    KpMemRecall        = ll::SDLK_KPMEMRECALL as isize,
    KpMemClear         = ll::SDLK_KPMEMCLEAR as isize,
    KpMemAdd           = ll::SDLK_KPMEMADD as isize,
    KpMemSubtract      = ll::SDLK_KPMEMSUBTRACT as isize,
    KpMemMultiply      = ll::SDLK_KPMEMMULTIPLY as isize,
    KpMemDivide        = ll::SDLK_KPMEMDIVIDE as isize,
    KpPlusMinus        = ll::SDLK_KPPLUSMINUS as isize,
    KpCear             = ll::SDLK_KPCEAR as isize,
    KpClearEntry       = ll::SDLK_KPCLEARENTRY as isize,
    KpBinary           = ll::SDLK_KPBINARY as isize,
    KpOctal            = ll::SDLK_KPOCTAL as isize,
    KpDecimal          = ll::SDLK_KPDECIMAL as isize,
    KpHexadecimal      = ll::SDLK_KPHEXADECIMAL as isize,
    LCtrl              = ll::SDLK_LCTRL as isize,
    LShift             = ll::SDLK_LSHIFT as isize,
    LAlt               = ll::SDLK_LALT as isize,
    LGui               = ll::SDLK_LGUI as isize,
    RCtrl              = ll::SDLK_RCTRL as isize,
    RShift             = ll::SDLK_RSHIFT as isize,
    RAlt               = ll::SDLK_RALT as isize,
    RGui               = ll::SDLK_RGUI as isize,
    Mode               = ll::SDLK_MODE as isize,
    AudioNext          = ll::SDLK_AUDIONEXT as isize,
    AudioPrev          = ll::SDLK_AUDIOPREV as isize,
    AudioStop          = ll::SDLK_AUDIOSTOP as isize,
    AudioPlay          = ll::SDLK_AUDIOPLAY as isize,
    AudioMute          = ll::SDLK_AUDIOMUTE as isize,
    MediaSelect        = ll::SDLK_MEDIASELECT as isize,
    Www                = ll::SDLK_WWW as isize,
    Mail               = ll::SDLK_MAIL as isize,
    Calculator         = ll::SDLK_CALCULATOR as isize,
    Computer           = ll::SDLK_COMPUTER as isize,
    AcSearch           = ll::SDLK_ACSEARCH as isize,
    AcHome             = ll::SDLK_ACHOME as isize,
    AcBack             = ll::SDLK_ACBACK as isize,
    AcForward          = ll::SDLK_ACFORWARD as isize,
    AcStop             = ll::SDLK_ACSTOP as isize,
    AcRefresh          = ll::SDLK_ACREFRESH as isize,
    AcBookmarks        = ll::SDLK_ACBOOKMARKS as isize,
    BrightnessDown     = ll::SDLK_BRIGHTNESSDOWN as isize,
    BrightnessUp       = ll::SDLK_BRIGHTNESSUP as isize,
    DisplaySwitch      = ll::SDLK_DISPLAYSWITCH as isize,
    KbdIllumToggle     = ll::SDLK_KBDILLUMTOGGLE as isize,
    KbdIllumDown       = ll::SDLK_KBDILLUMDOWN as isize,
    KbdIllumUp         = ll::SDLK_KBDILLUMUP as isize,
    Eject              = ll::SDLK_EJECT as isize,
    Sleep              = ll::SDLK_SLEEP as isize,
}

impl Hash for KeyCode {
    #[inline]
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        (*self as i32).hash(state);
    }
}

impl ToPrimitive for KeyCode {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        Some(*self as i64)
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        Some(*self as u64)
    }

    #[inline]
    fn to_int(&self) -> Option<isize> {
        Some(*self as isize)
    }
}
