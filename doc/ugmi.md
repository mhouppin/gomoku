# UGMI Protocol

## Description of the Universal GoMoku Interface (UGMI)

- The specification is independent of the operating system. For Windows, the
engine is a normal exe file, either a console or "real" Windows application.

- All communication is done via standard input and output with text commands.

- The engine should boot and wait for input from the GUI. The engine should wait
for the `isready` or `setoption` commands to set up its internal parameters as
the boot process should be as quick as possible.

- The engine must always be able to process input from stdin, even while
thinking.

- All command strings the engine receives must end with a newline, all command
strings the GUI receives must end with a newline. It can be either `\r\n` (for
example on Windows) or `\n`.

- All time durations sent by the GUI or the engine are in milliseconds.

- Arbitrary whitespace counts before and after tokens are allowed, limited to
spaces and tabulations.
Example:
```
debug on\n
```
and
```
  debug \t  on  \t\n
```
all set the debug mode of the engine on.

- The engine should never start calculating or pondering without receiving a
`go` command first.

- Before the engine is asked to search on a position, there will always be a
`position` command to tell the engine about the current position.

- If the engine or the GUI receives an unknown or ill-formed command, it should
just ignore it. The engine can inform the GUI about the use of such commands
using a `info error` command. The same applies for commands that are not
supposed to come (for example a `stop` when the engine is not calculating).

## Move Format

- The move format is encoded as `n11` specifying the square of arrival of the
stone (with `n` being a letter between `a` and `s` indicating the file of the
board and `11` being a number between `01` and `19` indicating the rank of the
board). Nullmoves (aka passing moves) are encoded as `000`.

## GUI to Engine

- These are all the commands sent by the GUI to the engine.

### `ugmi`

- Tells the engine to use the UGMI protocol. This will be sent once as the first
command after program boot to tell the engine to switch to UGMI mode.
- After receiving the `ugmi` command, the engine must identify itself with the
`id` command and send the `option` commands to tell the GUI which engine
settings the engine supports, if any.
- After that the engine must send `ugmiok` to acknowledge the UGMI mode. If no
`uciok` is sent within a certain time period, the engine task should be killed
by the GUI.

### `debug (on|off)`

- Switches the debug mode of the engine on and off. In debug mode the engine
should send additional information to the GUI, for example with the `info
string` command, to help debugging (for example an engine could log the commands
it received and how it interpreted them).
- The debug mode must be switched off by default by the engine. The `debug`
command can be sent at any time, including when the engine is thinking.

### `isready`

- This command is used to synchronize the engine with the GUI. When the GUI has
sent a command or multiple commands that can take some time to complete (for
example initializing a new hashtable when the `Hash` option is set), this
command can be used to wait for the engine to be ready again or to ping the
engine to find out if it is still alive.
- This command must be sent at least once before the engine is asked to do any
search and after the position information has been sent, to wait for the engine
to finish initializing its internal board.
- This command must always be answered with `readyok` and can be sent at any
time, including when the engine is thinking. This command should not stop the
search.

### `setoption name <option_name> [value <option_value>]`

- This command is sent to the engine when the user wants to change the internal
parameters of the engine. For the button type the `value` token is not needed.
- One command will be sent for each parameter. This command must be sent only
when the engine is waiting. The name and value of the option are case-sensitive,
and can both include spaces.
- The substrings `name` and `value` must not be used for the `option_name` and
`option_value` fields to disambiguate parsing. This is also the case for the
`option` commands sent by the engine.
### `ugminewgame`

- This command is sent to the engine when the next requested search will be from
a different game. (This can be a new game the engine should play/analyze, or the
next position from a testsuite.)
- As the engine's reaction to `ugminewgame` can take some time the GUI must
always send `isready` after `ugminewgame` to finish its operation.

### `position (board <boardstring>|startpos) [moves [<move1> [<move2...moveN>]]]`

- This command asks the engine to set up the position described on its internal
board and play the moves listed from the given position.
- If the game was played from an empty board, the string `startpos` will be sent
in place of `board <boardstring>`.
- TODO: describe the syntax of the `<boardstring>` field

### `go [search_parameters]`

- This command asks the engine to start calculating on the current position set
up with the `position` command.
- There are a number of search parameters that can follow this command. If no
search parameters affecting the search limits (that is, any parameter except for
`searchmoves`) are specified the engine must run in infinite mode, which
means it must not send a `bestmove` command until a `stop` command is issued.

    - `searchmoves <move1> [<move2...moveN>]` restricts the search to the given
    moves only. For example, after `position startpos` and `go moves h08 j10`
    the engine should only search infinitely these two moves in the initial
    position.
    - `[wtime <time>] [btime <time>] [winc <time>] [binc <time>]`
    configures the remaining time and increment for white/black. If this
    parameter is sent by the GUI, it must contain the remaining time for the
    side to move.
    - `depth <x>` limits the search to `x` plies of depth.
    - `nodes <x>` limits the search to `x` nodes.
    - `mate <x>` asks the engine to search for a mate in `x` moves (that is,
    `(x * 2 - 1)` plies).
    - `movetime <x>` limits the search to `x` milliseconds.

### `stop`

- Asks the engine to stop calculating as soon as possible and return a bestmove.

### `quit`

- Asks the engine to stop its execution as soon as possible.

## Engine to GUI

### `id (name|author) <x>`

- This command must be sent twice after receiving the `ugmi` command to identify
the engine name and the engine author(s).

### `uciok`

- This command must be sent once after the `id` and `option` commands sent by
the engine in response to `ugmi`, to indicate that the engine has sent all infos
and is ready to operate in UGMI mode.

### `readyok`

- This command must be sent when the engine has received an `isready` command,
has processed all input and is ready to accept new commands now.

### `bestmove <move>`

- This command indicates that the engine has stopped searching and selected the
move `<move>` as best in this position.
- This command must always be sent if the engine stops searching, either
voluntarily or after a `stop` command is issued by the GUI.
- Prior to this command the engine should send a final `info` command with the
final search information available, so that the GUI has complete statistics
about the last search.

### `info [info_parameters]`

- This command indicates that the engine wants to send information to the GUI.
- The engine can choose to send multiple information with a single `info`
command, for example with `info depth 12 nodes 123456 nps 250000`.
- These default info parameters are supported:

    - `depth <x>` indicates the search depth in plies.
    - `seldepth <x>` indicates the selective search depth in plies. While depth
    should never go down as search progresses, the selective depth can fluctuate
    up and down. If `seldepth` is indicated in an `info` command, `depth` must
    be indicated in the same command as well.
    - `time <x>` indicates the search time in milliseconds.
    - `nodes <x>` indicates the number of nodes searched.
    - `pv <move1 move2...moveN>` indicates the theoretical best line of play
    found by the engine (PV = Principal Variation). This parameter must be sent
    as the last parameter of the `info` command.
    - `score <cp|mate> <x> [lowerbound|upperbound]` indicates the estimated
    score of the position from the engine's POV (theoretical (dis)advantage).
    `cp` denotes that the score is in centipoints (1/100th of a point), `mate`
    denotes that the engine found a mate in the position (if `x` is negative, it
    means that the engine thinks it's getting mated). `lowerbound` and
    `upperbound` indicate that the search score only has partial information,
    that is, the given score is only a lowerbound/upperbound.
    - `hashfull` indicates the filling rate of the engine's hashtable, and is in
    permill (by units of 1/1000th).
    - `nps` indicates the number of nodes per second searched.
    - `string <str>` indicates information that should be sent in form of a
    plain string that can be displayed if wanted by the GUI. This parameter must
    be sent as the last parameter of the `info` command.

### `option name <option_name> type <option_type> [...]`

- This command tells the GUI which parameters can be changed in the engine.
- This should be sent once at engine startup when `ugmi` is issued, after the
`id` commands and before `ugmiok`.
- The GUI should parse this and build a dialog for the user to change the
settings.
- If the user wants to change settings, the GUI will send a `setoption` command
to the engine. Note that the GUI does not need to send the `setoption` command
when starting the engine for every option if it does not want to change the
default value.
- `name <option_name>` denotes the option name.
- `type <option_type>` denotes an option type. There are 5 types of options:

    - `check`: a checkbox that can be either true or false.
    - `spin`: a spin wheel that can be an integer in a certain range.
    - `combo`: a combo box that can have different predefined strings as a
    value.
    - `button`: a button that can be pressed to trigger an event in the engine.
    It does not have any value associated, and therefore does not need a `value`
    field when used in a `setoption` command.
    - `string`: a text field that has a string as a value. `<empty>` denotes an
    empty string.

- `default <x>` indicates the default value of the parameter.
- `min <x>` indicates the minimum value of the parameter. It must only be used
for `spin` options.
- `max <x>` indicates the maximum value of the parameter.It must only be used
for `spin` options.
- `var <x>` indicates a predefined value for the parameter. It must only be used
for `combo` options.
- Here are 5 examples of `option` commands:
    - `option name Enable Nullmove type check default true`
    - `option name HashSizeMB type spin default 16 min 1 max 4096`
    - `option name Ruleset type combo default standard var standard var pro var swap var swap2`
    - `option name Logfile Path type string default /tmp/gomoku_engine.log`
    - `option name Clear Hash type button`

### Example of a exchange between a GUI and an engine

- In the following block, `<- ` denotes lines sent by the GUI to the engine,
while `->` denotes lines sent by the engine to the GUI.

```
<- ugmi
-> id name Some Engine v1.12
-> id author Corentin Scialpi
-> option name Enable Nullmove type check default true
-> option name HashSizeMB type spin default 16 min 1 max 4096
-> option name Ruleset type combo default standard var standard var pro var swap var swap2
-> option name Logfile Path type string default /tmp/gomoku_engine.log
-> option name Clear Hash type button
-> ugmiok
<- setoption name Hash value 32
<- setoption name Enable NullMove value false
<- setoption name Ruleset value pro
<- isready
-> readyok
<- ugminewgame
<- isready
-> readyok
<- position startpos
<- isready
-> readyok
<- go movetime 6000 nodes 600000 depth 15
-> info depth 1 seldepth 2 score cp -15 nodes 23 nps 23000 time 1 pv i10
-> info depth 2 seldepth 4 score cp 17 nodes 214 nps 107000 time 2 pv i10 h08
-> info depth 3 seldepth 3 score cp 21 nodes 355 nps 118333 time 3 pv i10 h08 j10
-> info depth 4 seldepth 7 score cp -14 nodes 1485 nps 148500 time 10 pv i10 h08 j10 g07
-> info depth 5 seldepth 9 score cp -7 nodes 5600 nps 140000 time 40 pv i10 h10 g11 j08
<- stop
-> info depth 6 seldepth 6 score cp -7 nodes 14500 nps 145000 time 100 pv i10 h10 g11 j08
-> bestmove i10
<- ugminewgame
<- debug on
<- isready
-> readyok
<- position board 19/19/19/19/19/19/19/19/19/9xo8/19/19/19/19/19/19/19/19/19 moves h08
-> info string Successfully parsed board
<- isready
-> readyok
<- go wtime 60000 btime 60000 winc 600 binc 600
-> info string Search time control: Standard 60+0.6
-> info depth 1 seldepth 4 score cp -16 nodes 27 nps 27000 time 1 pv j10
-> info string Consumed alloted time
<- bestmove j10
<- quit
```