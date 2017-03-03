use keyboard::*;
use pointer::*;
use viewport::{PixelCoord, ViewportSize};


pub enum MouseButton
{
    Left,
    Middle,
    Right,
    Button(u8)
}

impl<'a> From<&'a MouseButton> for u8
{
    fn from(button: &'a MouseButton) -> Self
    {
        match button
        {
            &MouseButton::Left => 0,
            &MouseButton::Middle => 1,
            &MouseButton::Right => 2,
            &MouseButton::Button(n) => n
        }
    }
}


pub struct GenericMouseState
{
    buttons: AnyKeyKeyboardState<GenericKeyboardState>,
    pointer: PointerState
}

impl GenericMouseState
{
    pub fn new(screen_resolution: ViewportSize, mouse_position: PixelCoord) -> GenericMouseState
    {
        GenericMouseState
        {
            buttons: AnyKeyKeyboardState::new(GenericKeyboardState::new()),
            pointer: PointerState::new(screen_resolution, mouse_position)
        }
    }
}


impl KeyboardController for GenericMouseState
{
    type Key = MouseButton;

    fn clear_changes(&mut self)
    {
        self.buttons.clear_changes();
    }

    fn _set_key_down(&mut self, key: &Self::Key, key_down: bool)
    {
        self.buttons._set_key_down(&u8::from(key), key_down);
    }

    fn _set_key_state_changed(&mut self, key: &Self::Key)
    {
        self.buttons._set_key_state_changed(&u8::from(key));
    }

    fn key_down(&self, key: &Self::Key) -> bool
    {
        self.buttons.key_down(&u8::from(key))
    }

    fn key_state_changed(&self, key: &Self::Key) -> bool
    {
        self.buttons.key_state_changed(&u8::from(key))
    }
}


impl AnyKeyController for GenericMouseState
{
    fn any_key_pressed(&self) -> bool
    {
        self.buttons.any_key_pressed()
    }

    fn any_key_released(&self) -> bool
    {
        self.buttons.any_key_released()
    }
}


impl PointerController for GenericMouseState
{

    fn set_screen_resolution(&mut self, resolution: ViewportSize)
    {
        self.pointer.set_screen_resolution(resolution);
    }

    fn set_pointer_position(&mut self, position: PixelCoord)
    {
        self.pointer.set_pointer_position(position);
    }

    fn screen_resolution(&self) -> ViewportSize
    {
        self.pointer.screen_resolution()
    }

    fn pointer_pixel_position(&self) -> PixelCoord
    {
        self.pointer.pointer_pixel_position()
    }

}