# Gobu
A library for creating Visual Novels.

You can create Visual Novels using pure Rust (hard) or by using TOML config files to import assets like Characters,
Background Images, Input, and Scripts; and writing scripts using a syntax that is much simpler than how it would be in Rust.

<b>NB: Only support a linear story at this point however support for branching plotlines is planned.</b>

Feedback is welcome! Feel free to create an issue!

[Documentation](https://hirunya.github.io/docs/gobu/)

## Currently Supports:
* Drawing Backgrounds
* Drawing Characters
* Drawing TextBoxes
* A scripting parser that can turn a script into directions.
* Playing Music (Background Music Only for now) via [Rodio](https://github.com/tomaka/rodio)
(Supports MP3, WAV, Vorbis, and Flac.)
* Character Transitions (FadeIn and FadeOut are currently the only ones supported, however you can make your own one)

## To Do:
* Add the ability to create VNs with branching plotlines.
Unfortunately this is the hardest one as it requires me trying to figure out how to make buttons work and how I would
implement it in the script. This is unfortunately probably the furthest away from being done.
* Play Sound Effects and Character Monologues
* Adding more Transitions like SlideFromLeft
* Adding Transitions to the Background.

## Contents:
* [Examples](#examples)
* [Script Syntax](#script-syntax)
* [TOML Files](#toml-files)

## Examples
See the [Example VN](https://github.com/HiruNya/example_vn) for an example as to how to make a game with minimal Rust knowledge.

![SCREENSHOT](https://user-images.githubusercontent.com/20044495/42315772-5e253856-809c-11e8-8eb0-cb2d26a6620c.PNG)

## Script Syntax
Example:
```
"Speaker": "Dialogue"
```
Set the text of a textbox with the text in ``Dialogue`` and changed the name of the speaker to ``Speaker``
```
"More Dialogue"
```
Only sets the text of a textbox.
```
SPAWN 'Character'
```
Spawns a ``CharacterEntity`` using a ``Character``. A ``CharacterEntity`` is the object which is drawn and moved across stage.
The entity would be called the same name as the character.
```
SPAWN 'Character' as 'Character2' at (3.0, 2.0) with 'Transition'
```
Spawn a ``CharacterEntity`` calling the entity "Character2" instead of "Character" by using the ``as`` syntax.
The ``at`` part defines the position of where it should be spawned.
The ``with`` part defines a Transition to be used when the Character spawns. This Transition must be defined in a TOML file.
``as``, ``at`` and ``with`` are all optional.
```
KILL 'Character1'
```
Remove the entity from the stage.
```
MOVE 'Character1' (1.0, 2.0)
```
Move an entity to the position specified.
```
HIDE 'Character'
```
Hides an entity, turning it invisible.
```
SHOW 'Character'
```
Shows an entity, turning it visible.
```
SHOW 'Character' ~ 'Sad'
```
Show an entity and changes it's state to 'Sad' which means the image on screen also changes.
```
PLAY 'Music'
```
Plays the music 'Music'.
```
STAGE 'BackgroundImage'
```
Sets the background image to `BackgroundImage`.

## TOML files
An example TOML file that creates Characters. Most of these keys are optional.
```TOML
[CharacterName]
default = "happy"
happy = "./path/to/happy.png"
sad = "./path/to/sad.png"
offset = { x = 0.5, y = 0.5 }  # Centres the image
size = { w = 32, h = 32 } # 32 pixels wide and high
```
