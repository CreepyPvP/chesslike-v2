# Notes

This document contains notes, meant for myself, on how to approach the rewrite of this project

- Data orientation: Think first about which data should represent a gamestate, then think about functions. Functions are only a "glue", mere transformation of the data, nothing more.
- Solve the problem at hand the simplest way possible. Don't overgeneralize, Don't overengineer. Just. Solve. The. Problem.
- Ecs are awesome
- OOP sucks
- Consider using cpp
- Use debugger more often
- Look into managing cache hits / misses

## Gdb cheatsheet

- Press Enter to repeat previous command
- (n)ext, to jump to next line, without stepping into it
- (s)tep to step into current function
- (p)rint, to print a value
- layout src, to show the source code
- Ctrl + L, to repair a broken layout
- (l)ist, to print the current line
- (k)ill and (r)un, to start / exit program
- (b)reak, to set a breakpoint at e. g. "chessgame::game::map::create_map", also accepts a line (#)
- (c)ontinue, to continue execution as normalrestart
- to use linenumbers or print correct source code compiler optimizaiton has to be disabled

