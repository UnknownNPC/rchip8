pub trait ActionInterface {
    fn emulate_cycle(&self);
    fn set_keys();
}

pub trait Action {
    fn current_op(&self) -> u16;

    fn op_0NNN(&mut self);
    fn op_00E0(&mut self);
    fn op_00EE(&mut self);
    fn op_1NNN(&mut self);
    fn op_2NNN(&mut self);
    fn op_3XNN(&mut self);
    fn op_4XNN(&mut self);
    fn op_5XY0(&mut self);
    fn op_6XNN(&mut self);
    fn op_7XNN(&mut self);
    fn op_8XY0(&mut self);
    fn op_8XY1(&mut self);
    fn op_8XY2(&mut self);
    fn op_8XY3(&mut self);
    fn op_8XY4(&mut self);
    fn op_8XY5(&mut self);
    fn op_8XY6(&mut self);
    fn op_8XY7(&mut self);
    fn op_8XYE(&mut self);
    fn op_9XY0(&mut self);
    fn op_ANNN(&mut self);
    fn op_BNNN(&mut self);
    fn op_CXNN(&mut self);
    fn op_DXYN(&mut self);
    fn op_EX9E(&mut self);
    fn op_EXA1(&mut self);
    fn op_FX07(&mut self);
    fn op_FX0A(&mut self);
    fn op_FX15(&mut self);
    fn op_FX18(&mut self);
    fn op_FX1E(&mut self);
    fn op_FX29(&mut self);
    fn op_FX33(&mut self);
    fn op_FX55(&mut self);
    fn op_FX65(&mut self);
}
