# Gobu
A library for creating Visual Novels.

You can create Visual Novels using pure Rust (hard) or by using TOML config files to import assets like Characters,
Background Images, Input, and Scripts; and writing scripts using a syntax that is much simpler than how it would be in Rust.

Contents:

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
SPAWN 'Character' as 'Character2' at (3.0, 2.0)
```
Spawn a ``CharacterEntity`` calling the entity "Character2" instead of "Character" by using the ``as`` syntax.
The ``at`` part defines the position of where it should be spawned. Both ``as`` and ``at`` are optional.
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
