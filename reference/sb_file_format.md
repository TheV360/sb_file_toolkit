# SmileBASIC File Format

## Note from Record
> Quick note: I didn't figure out most of the format of SB files. Trinitro21 did most of the work figuring out the file structure, and the footer was thanks to code plutooo had for smilehax. I just wrote this document, which sums everything we know.  
DISCLAIMER: This is not 100% correct and may change at any time.  
**By the way, me and Trinitro21 (triangle) wrote an API to download programs from the SmileBASIC servers. It can also display GRPs as PNGs. It's available at [http://sbapi.me/](http://sbapi.me/)**

## Note from V360
> I made the table formatting worse and probably added some incorrect info; lots of love

SmileBASIC has its own format for storing its files. There are 2 main types of files, TXT and DAT. Both include the common header and footer, but DAT files have a secondary header after the common header to store metadata like how many dimensions it has, how large each dimension is, and what type of data it stores.

All files are stored in SmileBASIC's ExtData archive stored on the SD card. The default folder is stored as ### in the ExtData, and filenames are prefixed with T or B, for Text (TXT) and Binary (DAT), respectively.

Prefix | File type
------:|----------
   `T` | Text (TXT, PRG)
   `B` | Binary (DAT, GRP)

## Common Header
The shared part of every SB file is the common header. This contains information such as the username of who wrote it, how large the data stored is, and when it was last modified. The common header is 80 bytes long on the 3DS and 112 bytes on Switch. All values are little-endian.

(Modified some of this to be nice and Markdown; `Offset` is abbreviated to `Ofs.`; `Size (in bytes` is abbreviated to `Bytes`)

(Also added a "type" section which is mostly redundant. It uses the same types as like... rust or something so `i16` is signed int 16 bits, `u16` is unsigned int 16 bits etc. Stolen from `sbfile.js` from 12's sbtools)

<!-- lol sorry -->

 Ofs. | Bytes | Type | Description | Values
------|------:|-----:|---|-
`&h00`| 2     | i16  | File Version | <table><tr><th>Value</th><th>Description</th></tr><tr><td>0</td><td>SmileBASIC 3 System Files (?)</td></tr><tr><td>1</td><td>SmileBASIC 3</td></tr><tr><td>4</td><td>SmileBASIC 4</td></tr></table>
`&h02`| 2     | i16  | File Type | <table><tr><th>Value</th><th>Type</th></tr><tr><td>0</td><td>TXT</td></tr><tr><td>1</td><td>DAT (includes SB3 GRPs)</td></tr><tr><td>2</td><td>GRP (SB4 only)</td></tr><tr><td>4</td><td>META (SB4 only)</td></tr></table>
`&h04`| 2     | i16  | Zlib Compression | <table><tr><th>Value</th><th>well, it's a boolean</th></tr><tr><td>0</td><td>No Compression</td></tr><tr><td>1</td><td>Compression</td></tr></table>
`&h06`| 2     | i16  | Project Browser Icon | For TXT files: 0 = TXT and 1 = PRG <br /> For DAT files: 0 = DAT and 2 = GRP
`&h08`| 4     | i32  | File Size | 32-bit value storing the size of the file contents. (No header/footer)
`&h0C`| 2     | i16  | Last mod. date: year
`&h0E`| 1     | i8   | Last mod. date: month
`&h0F`| 1     | i8   | Last mod. date: day
`&h10`| 1     | i8   | Last mod. date: hour
`&h11`| 1     | i8   | Last mod. date: minute
`&h12`| 1     | i8   | Last mod. date: second
`&h13`| 1     | i8   | Unknown, might be part of mod. date

In SB4, uploader information is slightly longer, to accomodate longer NNIDs. Due to this, the header lengths

### 3DS Uploader Informaton

 Ofs. | Bytes | Description
------|------:|---------------------
`&h14`| 18    | The first author's (the original uploader)  NNID. This one isn't shown in the project browser
`&h26`| 18    | The second author's (the last editor) NNID. This one is displayed in the project browser
`&h38`| 4     | The first author's user ID. Used for controlling the blacklist editable in the project download area
`&h3C`| 4     | The second author's user ID
`&h50`|       | End of Header

### Switch Uploader Information

 Ofs. | Bytes | Description
------|------:|---------------------
`&h14`| 32    | The first author's (the original uploader)  NNID. This one isn't shown in the project browser
`&h34`| 32    | The second author's (the last editor) NNID. This one is displayed in the project browser
`&h54`| 4     | The first author's user ID. Used for controlling the blacklist editable in the project download area
`&h58`| 4     | The second author's user ID
`&h70`|       | End of Header

The header ends at `&h50` on 3DS and `&h70` on Switch. Note that there's empty space.

## DAT Secondary Header (Petit Computer BiNary)

TXT and PRG files just place the UTF-8 text after the footer. However, the DAT and GRP files need *more* information -- the data type, the dimensions, etc.. To store this, they employ a secondary header, stored immediately after the end of the first header. This secondary header stores information for SB to parse the file properly.

(offset is the offset after the header, which changes depending on the version of SmileBASIC in use.)
(the offset is relative to the end of the common header)

(03 = 16 bit unsigned (colors as RGBA5551, used for GRPs), 04 = Signed 32 bit integers (int%), 05 = 64-bit double (real#)). GRPs in SB4 are stored as integer (data type &h04) DAT files since they use RGBA8888 encoding.


 Ofs. | Bytes | Description
------|------:|--------------
`&h00`| 8     | Always the ASCII string "PCBN000n", where n is the device type (offset &h00)
`&h08`| 1     | Data type<br /><table><tr><th>Value</th><th>Data Type</th></tr><tr><td>3</td><td>Unsigned 16-bit Integer - SB3 GRPs, as RGBA5551</td></tr><tr><td>4</td><td>Signed 32-bit Integer - VAR% arrays; SB4 GRPs, as RGBA8888</td></tr><tr><td>5</td><td>64-bit Double - VAR# arrays</td></tr></table>
`&h0A`| 1     | Number of dimensions (1-4)
`&h0C`| 4     | 32-bit value storing the size of the first dimension
`&h10`| 4     | 32-bit value storing the size of the second dimension if applicable
`&h14`| 4     | 32-bit value storing the size of the third dimension if applicable
`&h18`| 4     | 32-bit value storing the size of the fourth dimension if applicable

Afterward, the data is stored in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order) (left-to-right top-to-bottom, instead of top-to-bottom left-to-right).

```
(Example: Say we have a 9x9 grid of numbers)
┌─┬─┬─┐
│A│D│G│
├─┼─┼─┤
│H│B│E│
├─┼─┼─┤
│F│I│C│
└─┴─┴─┘

Row-Major:    A D G H B E F I C
Column-Major: A H F D B I G E C
```

## META Project file (Petit Computer Project Metadata)
META files are used in projects to store metadata about a project, including icon, name, and description.

(offset is the offset after the header, which changes depending on the version of SmileBASIC in use.)

 Offset | Bytes | Description
--------|------:|-------------------
`&h0000`| 8     | Always the ASCII string "PCPM0005"
`&h0008`| 48    | The project name (UCS-2)
`&h0038`| 4576  | The project description (UCS-2)
`&h1218`| 4     | The width of the project icon

The data following is the icon's pixel data, encoded in BGRA8888. Icons are always square, so you can find the length of the icon data by squaring the icon width and multiplying by 4.

## Projects
File type 2 in SB3 and file type 3 on SB4 are reserved for project files that are used for uploading/downloading projects from the server. They are unpacked by SB when downloaded into a proper file structure.
As such, this file format should never be encountered unless you're talking to the SB servers yourself. If you're doing that, figuring out project file format is left as an exercise for you :)

## Common Footer
The footer is a 20 byte HMAC-SHA1 hash of the **header *and* data** using this HMAC key:

> `nqmby+e9S?{%U*-V]51n%^xZMk8>b{?x]&?(NmmV[,g85:%6Sqd"'U")/8u77UL2`

The footer must be valid in order to download or upload a program/project, not having a valid footer will cause an error when doing either of these.
