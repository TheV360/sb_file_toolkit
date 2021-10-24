(another document i will write directly before abandoning the project)

# High level structure or whatever


so from the console
i wanna pick one of two modes :

read a file or write a file.
sub commands?

and i guess when i try to automatically determine the filetype from the ext, i might need to have like `(FileType, Option<FileVersion>)` or some shit. if file version is none then it tries to figure out. prints out `use --sb-version 3/4 please` (and it does support `0` for internal sb3 file for whatever reason)

# How will metadata survive a round-trip?

Maybe like... at some point in the file there should be something like
```
'～ MD fileVersion 3
'～ MD creator 42069 V360
'～ MD editor 42069 V360
'using U+FF5E to make it difficult to type by mistake
```
There'll be a special substitution for invalid characters

|     Character | Replacement | Candidates |
|--------------:|-------------|------------|
|   `Null` U+00 | `０` U+FF10 |`␀`|
| `Return` U+10 | `．` U+FF0E |`⏎`, `␍`|

Characters
Unicode
Name
⏎
U+23CE
Return Symbol
␍
U+240

stop actualy. 
metadata will only be present in SB->TXT files, and will be removed and used
if found while converting from TXT->SB

```vb
' :: sb_tools metadata :: '
' version:  sb3       (ptc|sb3s|sb3|sb4)
' type:     txt       (txt|dat|grp|meta; validation: sb4-only variants)
' compress:           (false|true|omitted; validation: only on dat)
' icon:     prg       (txt|prg|dat|grp; validation: overlapping variants)
' size:     auto      (auto|i32; validation: overflow)
' mod: 2021/04/20     (yyyy/mm/dd hh:mm:ss (all i8, year i16); validation: overflow)
' unknown:            (i8|omitted)
' creator: 42069 V360 ()
' editor:             (same as creator|omitted to duplicate; validation: same as creator)
' :: sb_tools metadata :: '
```

compact exmaple
```vb
' :: metadata :: '
' version:  sb3
' type:     txt
' icon:     prg
' size:     auto
' mod: 2021/04/20 13:37:00
' creator: 42069 V360
' :: metadata :: '
```

the metadata representation is mostly the same as sb_file_format stuff. hell this references that too. use that.

name|values|needs validation for
-|-|-
`version`|`ptc`\|`sb3s`\|`sb3`\|`sb4`
`type`|`txt`\|`dat`\|`grp`\|`meta`|sb4-only variants
`compress`|`false`\|`true`\|omitted|only allowed on dat files
`icon`|`txt`\|`prg`\|`dat`\|`grp`|there are overlapping variants
`size`|`auto`/unset\|`<i32>`|overflow
`modified`|`auto`/unset\|`<i16>/<i8>/<i8> <i8>:<i8>:<i8>`|overflow
`unknown`|unset\|`<i8>`|overflow
`creator`|`<i32> <string>`|overflow, string length
`editor`|`auto`/unset\|`<i32> <string>`|overflow, string length

strings need to be quoted, to preserve whitespace. Sorry, it looked good too...

the plan is to support some kinda pseudo-csv for dat files and have this same header at the top. maybe it could be a buncha `DATA` statements?? that'd be real cool, and it means you could paste them into your program / paste them into a new file!!!

if the metadata is not present, it'll be generated from sensible presets. i don't think i'll do any hiding of those presets when they go `brandNewTXT -> SB -> TXT`, so you'll have to deal with em

the goal is to round trip a file and have it be byte-for-byte equal


What the hell is going on with unknown_1 and unknown_2. is unk_1 1 byte or 8? why is it also set to 3?
* unk1 is 1 byte.
* unk2 is 16B in sb3 & 20B in sb4

# How about subcommands and a simple input-output default command?

as in

## Simple conversion

### To SmileBASIC Formats
```
$ sb_tools for.sb3 FOR
Plain Text -> SmileBASIC 3 Text.
Added prefix. Filename is now "TFOR".
Converted.
$ sb_tools hi.dat
$ 
```

## Print out info

```
$ sb_tools info TFOR
FOR
SmileBASIC 3 Text File
Creator: V360 (42069)
Editor:  V360 (42069)
Last Mod: 2019-12-23 11:19 PM
```

# what to do with bit-for-bit stuff

i might still try to do that human-readable header thing, becuase it's cool.

so, when load a file, i'll borrow the first 2 bytes to check the version, then branch based off that.


