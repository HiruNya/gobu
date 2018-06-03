use piston_window;
pub struct GameInput {
    pub cont_inp: Vec<Input>
}

pub struct Input {
    pub key: bool,
    pub pressed: bool,
}