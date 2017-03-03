pub trait KeyboardController
{
    type Key;

    fn clear_changes(&mut self);

    fn _set_key_down(&mut self, key: &Self::Key, key_down: bool);
    fn _set_key_state_changed(&mut self, key: &Self::Key);

    fn key_down(&self, key: &Self::Key) -> bool;
    fn key_state_changed(&self, key: &Self::Key) -> bool;


    fn set_key_state(&mut self, key: &Self::Key, key_down: bool)
    {
        self._set_key_down(key, key_down);
        self._set_key_state_changed(key);
    }

    fn set_key_down(&mut self, key: &Self::Key)
    {
        self.set_key_state(key, true);
    }

    fn set_key_up(&mut self, key: &Self::Key)
    {
        self.set_key_state(key, false);
    }

    fn key_pressed(&self, key: &Self::Key) -> bool
    {
        self.key_down(key) && self.key_state_changed(key)
    }

    fn key_released(&self, key: &Self::Key) -> bool
    {
        !self.key_down(key) && self.key_state_changed(key)
    }
}


pub trait AnyKeyController
{
    fn any_key_pressed(&self) -> bool;
    fn any_key_released(&self) -> bool;
}


pub struct AnyKeyKeyboardState<T>
{
    keyboard_state: T,
    any_key_pressed: bool,
    any_key_released: bool
}

impl<T> AnyKeyKeyboardState<T>
    where T: KeyboardController
{
    pub fn new(keyboard_state: T) -> Self
    {
        AnyKeyKeyboardState
        {
            keyboard_state: keyboard_state,
            any_key_pressed: false,
            any_key_released: false
        }
    }
}

impl<T> KeyboardController for AnyKeyKeyboardState<T>
    where T: KeyboardController
{
    type Key = T::Key;

    fn clear_changes(&mut self)
    {
        self.keyboard_state.clear_changes();
        self.any_key_pressed = false;
        self.any_key_released = false;
    }

    fn _set_key_down(&mut self, key: &Self::Key, key_down: bool)
    {
        self.keyboard_state._set_key_down(key, key_down);
    }

    fn _set_key_state_changed(&mut self, key: &Self::Key)
    {
        self.keyboard_state._set_key_state_changed(key);
        match self.key_down(key)
        {
            true => self.any_key_pressed = true,
            false => self.any_key_released = true
        }
    }

    fn key_down(&self, key: &Self::Key) -> bool
    {
        self.keyboard_state.key_down(key)
    }

    fn key_state_changed(&self, key: &Self::Key) -> bool
    {
        self.keyboard_state.key_state_changed(key)
    }
}

impl<T> AnyKeyController for AnyKeyKeyboardState<T>
    where T: KeyboardController
{
    fn any_key_pressed(&self) -> bool
    {
        self.any_key_pressed
    }

    fn any_key_released(&self) -> bool
    {
        self.any_key_released
    }
}


pub struct GenericKeyboardState
{
    key_down: [bool; 256],
    key_state_changed: [bool; 256],
    any_key_pressed: bool,
    any_key_released: bool
}

impl GenericKeyboardState
{
    pub fn new() -> Self
    {
        GenericKeyboardState
        {
            key_down: [false; 256],
            key_state_changed: [false; 256],
            any_key_pressed: false,
            any_key_released: false
        }
    }
}

impl KeyboardController for GenericKeyboardState
{
    type Key = u8;

    fn clear_changes(&mut self)
    {
        self.key_state_changed = [false; 256];
        self.any_key_pressed = false;
        self.any_key_released = false;
    }

    fn _set_key_down(&mut self, key: &Self::Key, key_down: bool)
    {
        self.key_down[*key as usize] = key_down;
    }

    fn _set_key_state_changed(&mut self, key: &Self::Key)
    {
        self.key_state_changed[*key as usize] = true;
    }

    fn key_down(&self, key: &Self::Key) -> bool
    {
        self.key_down[*key as usize]
    }

    fn key_state_changed(&self, key: &Self::Key) -> bool
    {
        self.key_state_changed[*key as usize]
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn keys_can_be_set_down()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_down(&0);
        assert_eq!(keyboard.key_down(&0), true);
        assert_eq!(keyboard.key_down(&1), false);
    }

    #[test]
    fn keys_can_be_set_up()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_down(&0);
        keyboard.set_key_down(&1);
        keyboard.set_key_up(&0);
        assert_eq!(keyboard.key_down(&0), false);
        assert_eq!(keyboard.key_down(&1), true);
    }

    #[test]
    fn keys_are_pressed_when_set_down()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_down(&0);
        assert_eq!(keyboard.key_pressed(&0), true);
        assert_eq!(keyboard.key_pressed(&1), false);
    }

    #[test]
    fn keys_are_released_when_set_up()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_up(&0);
        assert_eq!(keyboard.key_released(&0), true);
        assert_eq!(keyboard.key_released(&1), false);
    }

    #[test]
    fn pressed_keys_are_reset_when_changes_cleared()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_down(&0);
        keyboard.clear_changes();
        assert_eq!(keyboard.key_down(&0), true);
        assert_eq!(keyboard.key_pressed(&0), false);
    }

    #[test]
    fn released_keys_are_reset_when_changes_cleared()
    {
        let mut keyboard = GenericKeyboardState::new();
        keyboard.set_key_up(&0);
        keyboard.clear_changes();
        assert_eq!(keyboard.key_down(&0), false);
        assert_eq!(keyboard.key_released(&0), false);
    }

    #[test]
    fn can_check_if_any_key_pressed()
    {
        let mut keyboard = AnyKeyKeyboardState::new(GenericKeyboardState::new());
        keyboard.set_key_down(&137);
        assert_eq!(keyboard.any_key_pressed(), true);
    }

    #[test]
    fn can_check_if_any_key_released()
    {
        let mut keyboard = AnyKeyKeyboardState::new(GenericKeyboardState::new());
        keyboard.set_key_up(&137);
        assert_eq!(keyboard.any_key_released(), true);
    }

    #[test]
    fn any_key_pressed_cleared_when_changes_cleared()
    {
        let mut keyboard = AnyKeyKeyboardState::new(GenericKeyboardState::new());
        keyboard.set_key_down(&137);
        keyboard.clear_changes();
        assert_eq!(keyboard.any_key_pressed(), false);
    }

    #[test]
    fn any_key_released_cleared_when_changes_cleared()
    {
        let mut keyboard = AnyKeyKeyboardState::new(GenericKeyboardState::new());
        keyboard.set_key_up(&137);
        keyboard.clear_changes();
        assert_eq!(keyboard.any_key_released(), false);
    }
}