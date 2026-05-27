# hotkey-rs

Run a simple hotkey macro.

## Configuration

The program is configured using a YAML file.
The `active_keys` is a list of keys which must all be active for the macro to trigger. The macro will retrigger if the keys are held down and the previous run of the macro has finished.

The `actions` is a sequential list of actions which will be executed by the macro. The delays are listed in milliseconds by default. The valid syntax for an action is:

- A KeyName followed by either `Up` or `Down`. Example: `1 Down`
- `Delay` followed by a non-negative number, which can have decimals. Example: `Delay 500`

The `multiplier` is an optional field which will multiply the delays in the `actions` field by the specified multiplier. Can be useful to switch to a different timebase, such as using frames instead of milliseconds by using `multiplier: 16.666667`.

## Keys

The list of valid key names:
| KeyName | Resulting Key Enum (`Key::*`) |
| :--- | :--- |
| `"Alt"` | `Alt` |
| `"AltGr"` | `AltGr` |
| `"Backspace"` | `Backspace` |
| `"CapsLock"` | `CapsLock` |
| `"Ctrl"` , `"Control"` | `Control` |
| `"CtrlL"` , `"ControlLeft"` | `ControlLeft` |
| `"CtrlR"` , `"ControlRight"` | `ControlRight` |
| `"Del"` , `"Delete"` | `Delete` |
| `"DownArrow"` | `DownArrow` |
| `"End"` | `End` |
| `"Esc"` , `"Escape"` | `Escape` |
| `"F1"` | `F1` |
| `"F2"` | `F2` |
| `"F3"` | `F3` |
| `"F4"` | `F4` |
| `"F5"` | `F5` |
| `"F6"` | `F6` |
| `"F7"` | `F7` |
| `"F8"` | `F8` |
| `"F9"` | `F9` |
| `"F10"` | `F10` |
| `"F11"` | `F11` |
| `"F12"` | `F12` |
| `"Home"` | `Home` |
| `"LeftArrow"` | `LeftArrow` |
| `"MetaLeft"` | `MetaLeft` |
| `"MetaRight"` | `MetaRight` |
| `"PageDown"` | `PageDown` |
| `"PageUp"` | `PageUp` |
| `"Return"` | `Return` |
| `"RightArrow"` | `RightArrow` |
| `"Shift"` | `Shift` |
| `"ShiftLeft"` | `ShiftLeft` |
| `"ShiftRight"` | `ShiftRight` |
| `"Space"` | `Space` |
| `"Tab"` | `Tab` |
| `"UpArrow"` | `UpArrow` |
| `"PrintScreen"` | `PrintScreen` |
| `"ScrollLock"` | `ScrollLock` |
| `"Pause"` | `Pause` |
| `"NumLock"` | `NumLock` |
| `"BackQuote"` | `BackQuote` |
| `"1"` , `"Num1"` | `Num1` |
| `"2"` , `"Num2"` | `Num2` |
| `"3"` , `"Num3"` | `Num3` |
| `"4"` , `"Num4"` | `Num4` |
| `"5"` , `"Num5"` | `Num5` |
| `"6"` , `"Num6"` | `Num6` |
| `"7"` , `"Num7"` | `Num7` |
| `"8"` , `"Num8"` | `Num8` |
| `"9"` , `"Num9"` | `Num9` |
| `"0"` , `"Num0"` | `Num0` |
| `"-"` , `"Minus"` | `Minus` |
| `"="` , `"Equal"` | `Equal` |
| `"Q"` | `Q` |
| `"W"` | `W` |
| `"E"` | `E` |
| `"R"` | `R` |
| `"T"` | `T` |
| `"Y"` | `Y` |
| `"U"` | `U` |
| `"I"` | `I` |
| `"O"` | `O` |
| `"P"` | `P` |
| `"["` , `"LeftBracket"` | `LeftBracket` |
| `"]"` , `"RightBracket"` | `RightBracket` |
| `"A"` | `A` |
| `"S"` | `S` |
| `"D"` | `D` |
| `"F"` | `F` |
| `"G"` | `G` |
| `"H"` | `H` |
| `"J"` | `J` |
| `"K"` | `K` |
| `"L"` | `L` |
| `";"` , `"SemiColon"` | `SemiColon` |
| `"'"` , `"Quote"` | `Quote` |
| `"\\"` , `"BackSlash"` | `BackSlash` |
| `"IntlBackslash"` | `IntlBackslash` |
| `"Z"` | `Z` |
| `"X"` | `X` |
| `"C"` | `C` |
| `"V"` | `V` |
| `"B"` | `B` |
| `"N"` | `N` |
| `"M"` | `M` |
| `","` , `"Comma"` | `Comma` |
| `"."` , `"Dot"` | `Dot` |
| `"/"` , `"Slash"` | `Slash` |
