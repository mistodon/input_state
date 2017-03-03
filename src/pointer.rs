use viewport::{self, PixelCoord, ViewportCoord, ViewportSize};


pub trait PointerController
{
    fn set_screen_resolution(&mut self, resolution: ViewportSize);
    fn set_pointer_position(&mut self, position: PixelCoord);
    fn screen_resolution(&self) -> ViewportSize;
    fn pointer_pixel_position(&self) -> PixelCoord;

    fn pointer_viewport_position(&self) -> ViewportCoord
    {
        viewport::pixel_to_viewport(self.pointer_pixel_position(), self.screen_resolution())
    }
}


pub struct PointerState
{
    pixel_position: (i32, i32),
    screen_resolution: (u32, u32)
}

impl PointerState
{
    pub fn new(screen_resolution: (u32, u32), pixel_position: (i32, i32)) -> Self
    {
        PointerState
        {
            pixel_position: pixel_position,
            screen_resolution: screen_resolution
        }
    }
}

impl PointerController for PointerState
{
    fn set_screen_resolution(&mut self, resolution: (u32, u32))
    {
        self.screen_resolution = resolution;
    }

    fn set_pointer_position(&mut self, position: (i32, i32))
    {
        self.pixel_position = position;
    }

    fn screen_resolution(&self) -> (u32, u32)
    {
        self.screen_resolution
    }

    fn pointer_pixel_position(&self) -> (i32, i32)
    {
        self.pixel_position
    }
}


#[cfg(test)]
mod tests
{
    use super::*;


    #[test]
    pub fn pointer_viewport_position_is_accurate()
    {
        let pointer = PointerState::new((800, 600), (400, 300));
        assert_eq!(pointer.pointer_viewport_position(), (0.0, 0.0));
    }

    #[test]
    pub fn pointer_viewport_position_updates_with_resize()
    {
        let pointer = PointerState::new((800, 600), (400, 300));
        assert_eq!(pointer.pointer_viewport_position(), (0.0, 0.0));
        let pointer = PointerState::new((1600, 1200), (400, 300));
        assert_eq!(pointer.pointer_viewport_position(), (-0.5, 0.5));
    }
}