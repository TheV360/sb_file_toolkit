# SmileBASIC File Toolkit

Tool to read/write SmileBASIC files in the command line. Currently only supports writing SB3 TXT/PRG files, which is enough for me.

i think this project (barebones cli app) will take me several years

## Future Plans

- SB3 DAT files
- BMP/PNG to GRP
- SB4 files maybe

## Documentation

Use `sb_tools --help` at the command line to see the full command reference. Also, check the `reference` folder for the materials I referenced while making this.

Invocation should look kinda like this.

`sb_tools [INPUT] [OUTPUT]`

TODO: there will be flags

### Converting a File

Let's say you have a SmileBASIC 3 program named `hi.sb3`.

```smilebasic
' This is my program
PRINT "Hi!"
WAIT 398
```

To convert it to a file your 3DS can accept, just run...

```shell
$ ./sb_tools hi.sb3 HI
```

(TODO: should actually have it work like this) If you don't supply the prefix, (the `T` in the resulting `THI`) the tool will add it for you!

## Contribution

## License

Well uh... hmm... I guess MIT / "do whatever just credit me" double-license is fine

## Credits

I loosely based this off their stuff. Hell, if you look in the `reference` folder, you can see some of the utilities & cheatsheets I've hacked together.

* [SmileBASIC file format](https://old.smilebasicsource.com/page?pid=652)
* [12Me21](https://github.com/12Me21/) ([& co.](https://github.com/12Me21/sbtools/blob/4e4ccaa5181120a6d0f9920c7c3a9e62338eea65/sbfile.js#L169)) - [JavaScript file parser / writer](https://github.com/12Me21/sbtools) ([online](https://12me21.github.io/sbtools/))
* [SmileBASIC API Team](https://github.com/SBAPI-Team) - [TypeScript file parser / writer](https://github.com/SBAPI-Team/SmileBASIC-FileParser)
* third item
