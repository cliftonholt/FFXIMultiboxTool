# FFXI Multitool

Command line tool for use with multi-boxing in Final Fantasy XI. Designed to facilitate swapping login_w.bin when you have more than 4 characters when using Jaku's autopol tool: https://github.com/jaku/FFXI-autoPOL.

# Usage

FFXI Multitool can be used by running the application normally or by calling it with a numerical argument.

    ./FFXIMultitool.exe

or

    ./FFXIMultitool.exe 1

# Config
Paths and file names are controlled by the config.toml file.

Default:

    [config]
    polpath = "G:\\PlayOnline\\SquareEnix\\PlayOnlineViewer"
    binpath = "G:\\Windower\\charbin\\"
    firstbin = "char1.bin"
    secondbin = "char2.bin"

polpath is the path the PlayOnlineViewer directory, where pol.exe resides.
binpath is the path to where you store your the bin files to swap,
firstbin and secondbin contain the file names you have given the bin files