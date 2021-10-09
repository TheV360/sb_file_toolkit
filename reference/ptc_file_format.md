# VERY MUCH NOT COMPLETE
<span style="color: red">UNFINISHED</span>

# Petit Computer File Format

This is a comprehensive list of the of the information contained in each file format used by Petit computer.

Converted to markdown now :)

## Common Header

The common header is 48 bytes long.

Offset | Bytes | Description
-------|------:|------------
`&h00` | 4     | Always the ASCII string "PX01"
`&h04` | 4     | Number of bytes after MD5 Hash (do they mean Length of MD5 Hash in bytes?)
`&h08` | 4     | Always 4 bytes of zeros. "Possibly continuation of bytes after header."
`&h0C` | 8     | File Name, (in ASCII?)
`&h14` | 16    | MD5 Hash string
`&h24` | 12    | File Type (see table below)
`&h30` |       | End of Header (48 bytes total)

The MD5 hash string is made from the following information, in order:
* `PETITCOM`
* The file type string
* Contents of the file

This common header is -- I'm assuming

### PRG Subheader
> `PETC0300RPRG`

'''Total Size(Without Heading): 12+ Bytes'''




{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;"|Byte(s)
! scope="col" style="text-align: center;"|Contained information
|-
|48-51
|Unused(needs to be tested)
|-
|52-55
|Package string, only useful if saved as package.
|-
|56-59
|The number of characters in file(Including spaces, carriage returns, etc.). See notes
|-
|60-(size+59)
|Petit computer character data: See [[Character Table]] for reference
|-
|(size+60)-EOF
|All of the packaged file, in package string bit order. Only exists if saved as package.
|}
===='''Notes:'''====

The absolute maximum number of characters that can be in a program is 524,228(0x07FFC4), which for whatever reason is less than the maximum value that 3 bytes can store. This is probably because the header takes up 60 bytes (0x00003C) which when added to the max character count gets 524,288, the maximum value Petit Computer allows.

Packaged programs just append the content of the files (without the PX01 header) at the end of the file.

=== <nowiki/>CHR File ===
'''File Type: PETC0100RCHR'''

'''Total Size(Without Heading): 8193'''
{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;" |Byte(s)
! scope="col" style="text-align: center;" |Contained information
|-
|48-8239
|Each byte makes up two pixels units which can have a value pointing to one of the 16(0-15) palette indexes.
|}
===='''Format:'''====
Each character block is made up of 64 pixel elements which is stored into 32 bytes. The data for the character to the right(or first position next column) after this block. The following image will help to visualize how each block is stored

![work in progress lol](ptc_file_format_fig1.svg)
[[File:Chr-format.png|thumb|none|178px|How a CHR file is stored]]

===COL File===
'''File Type: PETC0100RCOL'''

'''Total Size(Without Heading): 512 Bytes'''

{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;" |Byte(s)
! scope="col" style="text-align: center;" |Contained information
|-
|48-559
|Palette data: See below for format
|}
===='''Format:'''====
Each color consists of 2 bytes(16 bits) and the color is read in the following format.

<span style="color:#0f0;">GGG</span><span style="color:#f00;">RRRRR</span> <span style="color:#0f0;">G</span><span style="color:#00f;">BBBBB</span><span style="color:#0f0;">GG</span>

Each color has 5 bits except for green which has 6.

The G channel must be right-rotated 2 bits in order to be parsed correctly.

===MEM File===
'''File Type: PETC0200RMEM'''

'''Total Size(Without Heading): 516 Bytes'''

{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;" |Byte(s)
! scope="col" style="text-align: center;" |Contained information
|-
|48-559
|UCS-2 encoded string. Unused characters are filled with 0x00 for strings less than 256 characters.
|-
|560-563
|Length of string (in characters - not bytes.)
|}
===='''Format:'''====
Strings are encoded as [[wikipedia:UCS-2|UCS-2 characters]], and all printed characters are encoded as their full width equivalents (eg, 'H' (ASCII 72) is encoded as 'Ｈ' (Unicode 65320/0xFF28))

=== GRP File ===
'''File type: PETC0100RGRP'''

'''Total Size(Without heading): 49152'''
{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;" |Byte(s)
! scope="col" style="text-align: center;" |Contained information
|-
|48-49199
|GRP data: See below, or linked page.
|}
===='''Format:'''====

GRP data is actually stored essentially as blocks of character data: A 4x3 page of 8x8 blocks of 8x8 characters. The difference is that the characters have 256 colors instead of 16. See [[GRP File Format (External)|here]] for more information.

=== SCR File ===
'''File type: PETC0100RSCR'''

'''Total Size(Without heading): 8192'''
{| class="article-table" border="1" cellpadding="4" cellspacing="4" style="width: 500px; height: 0px;"
|-
! scope="col" style="text-align: center; width: 20%;" |Byte(s)
! scope="col" style="text-align: center;" |Contained information
|-
|48-8239
|SCR data: See below.
|}
===='''Format:'''====

Each BG tile is represented as a 16 bit (2 byte) value. The 64x64 BG layer is broken into four 32x32 chunks, which are stored in the file as tile values, going left to right and then top to bottom through each chunk. The chunks are similarly stored left to right and top to bottom.

# Credits
* Petrified Lasagna: Most of the above article
* Randomouscrap98: Creation of [http://petitcomputer.wikia.com/wiki/GRP_File_Format_%28External%29 this] that contains information about the header
*'''Discostew: '''Knowing about the information in the header in the first place!
*'''MasterR3C0RD:''' MEM file and PRG package information
*'''Minxrod:''' Added SCR file info, minor edits
[[Category:Resources]]
[[Category:System Guides]]
