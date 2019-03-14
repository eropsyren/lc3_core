use crate::mem::Memory;
use crate::controller::Controller;

pub fn halt(mem: &mut Memory, controller: &mut Controller) {
    controller.stop();    
}