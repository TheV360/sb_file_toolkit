# SmileBASIC File Format

## Overview

Name | Size
-----|-----
[Common Header](#common_header) | `&h50` or `&h70` ()
[DAT Secondary Header](#dat_header) | `&h1C` ()
[Common Footer](#common_footer) | `&h14`

<!-- TODO: remember what i was doing in those above parentheses -->

## Note from Record
> Quick note: I didn't figure out most of the format of SB files. Trinitro21 did most of the work figuring out the file structure, and the footer was thanks to code plutooo had for smilehax. I just wrote this document, which sums everything we know.  
DISCLAIMER: This is not 100% correct and may change at any time.  
**By the way, me (Record) and Trinitro21 (triangle) wrote an API to download programs from the SmileBASIC servers. It can also display GRPs as PNGs. It's available at [http://sbapi.me/](http://sbapi.me/)**

## Note from V360
> I made the table formatting worse and probably added some incorrect info; lots of love
>
> hmmmmmmm the tables aren't 100% right -- the data offsets aren't right maybe? I also need to add `End of Header` to a lot more things.
>
> (Modified some of this to be nice and Markdown; `Offset` is no longer abbreviated to `Ofs.`; `Size (in bytes` is abbreviated to `Bytes`)
>
> (Also added a "type" section which is mostly redundant. It uses the same types as like... rust or something so `i16` is signed int 16 bits, `u16` is unsigned int 16 bits etc. Stolen from `sbfile.js` from 12's sbtools)

SmileBASIC, being a complete technical overhaul of Petit Computer, has its own format for storing files. There are 2 main types of files: TXT and DAT. Both include the [common header](#common_header) and footer, but DAT files have a secondary header after the common header to store metadata.

All files are stored in SmileBASIC's ExtData archive stored on the SD card. The default folder is stored as ### in the ExtData, and filenames are prefixed with T or B, for Text (TXT) and Binary (DAT), respectively.

Prefix | File type
------:|----------
`T`    | Text (TXT, PRG)
`B`    | Binary (DAT, GRP)

## <span id="common_header">Common Header</span>
The shared part of every SB file is the common header. This contains information such as the username of who wrote it, how large the data stored is, and when it was last modified. The common header is 80 bytes long on the 3DS and 112 bytes on Switch. All values are little-endian.

<!-- lol, sorry about this. nested tables when. -->

Offset | Bytes | Type | Description
------:|------:|-----:|------------
`&h00` | 2     | i16  | File Version<br /><table><thead><tr><th>Value<th>Description<tbody><tr><td>≤3<td>SmileBASIC 3 (Some system files may mistakenly have 0)<tr><td>4<td>SmileBASIC 4</table>
`&h02` | 2     | i16  | File Type<br /><table><thead><tr><th>Value<th>Type<tbody><tr><td>0<td>TXT<tr><td>1<td>DAT (includes SB3 GRPs)<tr><td>2<td>GRP (SB4 only)<tr><td>4<td>META (SB4 only)</table>
`&h04` | 2     | i16  | Zlib Compression<br /><table><thead><tr><th>Value<th>well, it's a boolean<tbody><tr><td>0<td>No Compression<tr><td>1<td>Compression</table>
`&h06` | 2     | i16  | Project Browser Icon<br />For TXT files: 0 = TXT and 1 = PRG<br />For DAT files: 0 = DAT and 2 = GRP
`&h08` | 4     | i32  | File Size<br />stores the size of the file contents. (Header/footer not included)
`&h0C` | 2     | i16  | Last mod. date: year
`&h0E` | 1     | i8   | Last mod. date: month
`&h0F` | 1     | i8   | Last mod. date: day
`&h10` | 1     | i8   | Last mod. date: hour
`&h11` | 1     | i8   | Last mod. date: minute
`&h12` | 1     | i8   | Last mod. date: second
`&h13` | 1     | i8   | Padding(?), may be part of mod. date

Any files with a file version of 0 loaded will become files with a file version of 1 when saved. This is obvious -- it's a round trip from SYS to an array to a user project, of course it's the same.

In SB4, uploader information is slightly longer, to accomodate longer NNIDs. Due to this, the header lengths vary between platforms.

### <span id="uploader_3ds">3DS Uploader Information

Offset | Bytes | Type | Description
------:|------:|-----:|------------
`&h14` | 18    | string | The first author's (the original uploader)  NNID. This one isn't shown in the project browser
`&h26` | 18    | string | The second author's (the last editor) NNID. This one is displayed in the project browser
`&h38` | 4     | i32  | The first author's user ID. Used for controlling the blacklist editable in the project download area
`&h3C` | 4     | i32  | The second author's user ID
`&h40` | 16    | ?    | Unknown
`&h50` |       |      | End of Header

### <span id="uploader_switch">Switch Uploader Information</span>

Offset | Bytes | Type | Description
------:|------:|-----:|------------
`&h14` | 32    | string | The first author's (the original uploader)  NNID. This one isn't shown in the project browser
`&h34` | 32    | string | The second author's (the last editor) NNID. This one is displayed in the project browser
`&h54` | 4     | i32  | The first author's user ID. Used for controlling the blacklist editable in the project download area
`&h58` | 4     | i32  | The second author's user ID
`&h5C` | 20    | ?    | Unknown
`&h70` |       |      | End of Header

The header ends at `&h50` on 3DS and `&h70` on Switch. Note that there's empty space.

## <span id="dat_header">DAT Secondary Header</span>

> Fun Fact! The constant string "`PCBN`" at the beginning stands for Petit Computer BiNary!

TXT and PRG files just place the UTF-8 text after the footer. However, the DAT and GRP files need *more* information -- the data type, the dimensions, etc.. To store this, they employ a secondary header, stored immediately after the end of the first header. This secondary header stores information for SB to parse the file properly.

The offset is relative to the end of the common header, so it will change position depending on the version of SmileBASIC in use.

<!--(03 = 16 bit unsigned (colors as RGBA5551, used for GRPs), 04 = Signed 32 bit integers (int%), 05 = 64-bit double (real#)). GRPs in SB4 are stored as integer (data type &h04) DAT files since they use RGBA8888 encoding.-->

<!--### Example

This example shows the DAT header of a file that is a 4 dimensional array with the size 2x5x10x4.

```
50 43 42 4E ; Magic number
30 30 30 31 ; Device Type
04 00       ; Data Type
04 00       ; Number of Dimensions
02 00 00 00 ; First Dimension Size
05 00 00 00 ; Second Dimension Size
0A 00 00 00 ; Third Dimension Size
04 00 00 00 ; Fourth Dimension Size
```

Of course, the integers are stored in little-endian. The device type, it being a string, is stored in big-endian, because it has to.-->

Offset | Bytes | Type | Description
------:|------:|-----:|------------
`&h00` | 8     | string | Always the ASCII string "PCBN000n", where n is the device type<br />This is similar to the [common header](#common_header)'s File Version field.<table><thead><tr><th>Value<th>Data Type<tbody><tr><td>1<td>SmileBASIC 3<tr><td>4<td>SmileBASIC 4</table>
`&h08` | 2     | i16 | Data type<br /><table><thead><tr><th>Value<th>Data Type<tbody><tr><td>3<td>Unsigned 16-bit Integer - SB3 GRPs, as RGBA5551<tr><td>4<td>Signed 32-bit Integer - VAR% arrays; SB4 GRPs, as RGBA8888<tr><td>5<td>64-bit Double - VAR# arrays</table>
`&h0A` | 2     | i16 | Number of dimensions (1-4)
`&h0C` | 4     | i32 | size of the first dimension
`&h10` | 4     | i32 | size of the second dimension (if applicable)
`&h14` | 4     | i32 | size of the third dimension (if applicable)
`&h18` | 4     | i32 | size of the fourth dimension (if applicable)
`&h1C` |       |     | End of Header

Afterward, the data is stored in [row-major order](https://en.wikipedia.org/wiki/Row-_and_column-major_order) (left-to-right wrapping from top-to-bottom, instead of top-to-bottom wrapping from left-to-right).

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

 Offset | Bytes | Type | Description
--------|------:|-----:|------------
`&h0000`| 8     | string | Always the ASCII string "PCPM0005"
`&h0008`| 48    | string | The project name (UCS-2)
`&h0038`| 4576  | string | The project description (UCS-2)
`&h1218`| 4     | i32 | The width of the project icon
`&h121C`|       |     | End of Header (gfx data continues)

The data following is the icon's pixel data, encoded in BGRA8888. Icons are always square, so you can find the length of the icon data by squaring the icon width and multiplying by 4, the number of bytes per pixel.

## Projects

File type 2 in SB3 and file type 3 on SB4 are reserved for project files that are used for uploading/downloading projects from the server. When downloaded, SmileBASIC automatically unpacks them into the proper file structure.
As such, this file format should never be encountered unless you're talking to the SB servers yourself. If you're doing that, figuring out project file format is left as an exercise for you :)

## <span id="common_footer">Common Footer</span>

The footer is a 20 byte HMAC-SHA1 hash of the **header *and* data** using this HMAC key:

> `nqmby+e9S?{%U*-V]51n%^xZMk8>b{?x]&?(NmmV[,g85:%6Sqd"'U")/8u77UL2`

The footer must be valid in order to download/upload a file. The absence of a valid footer will cause an error when doing either of these.
