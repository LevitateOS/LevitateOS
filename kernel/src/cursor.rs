// TEAM_086: Import GpuState for new Display API
use crate::gpu::GpuState;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use levitate_utils::Spinlock;

// TEAM_058: Track both current and previous position to erase trails
struct CursorState {
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
}

static CURSOR: Spinlock<CursorState> = Spinlock::new(CursorState { 
    x: 500, 
    y: 500,
    prev_x: 500,
    prev_y: 500,
});

#[allow(dead_code)]
pub fn update(x: i32, y: i32) {
    let mut state = CURSOR.lock();
    state.prev_x = state.x;
    state.prev_y = state.y;
    state.x = x;
    state.y = y;
}

#[allow(dead_code)]
pub fn set_x(x: i32) {
    let mut state = CURSOR.lock();
    state.prev_x = state.x;
    state.x = x;
}

#[allow(dead_code)]
pub fn set_y(y: i32) {
    let mut state = CURSOR.lock();
    state.prev_y = state.y;
    state.y = y;
}

/// TEAM_086: Changed to accept &mut GpuState instead of &mut Display
/// This eliminates 4 separate lock acquisitions and fixes deadlock potential
#[allow(dead_code)]
pub fn draw(gpu_state: &mut GpuState) {
    let state = CURSOR.lock();
    
    // Draw new cursor position (white)
    let _ = Rectangle::new(Point::new(state.x, state.y), Size::new(10, 10))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::WHITE))
        .draw(gpu_state);

    // TEAM_086: Flush is now caller's responsibility - or we can do it here
    // since this is a complete operation
    let _ = gpu_state.flush();
}
