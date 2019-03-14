use crate::controller::Controller;
use crate::mem::Memory;

pub fn halt(mem: &mut Memory, controller: &mut Controller) {
    controller.stop();
}
