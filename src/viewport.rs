pub type PixelCoord = (i32, i32);
pub type ViewportCoord = (f32, f32);
pub type ViewportSize = (u32, u32);


pub fn pixel_to_viewport(pixel_pos: PixelCoord, viewport_size: ViewportSize) -> ViewportCoord
{
    let (x, inv_y) = pixel_pos;
    let (w, h) = viewport_size;
    let y = (h as i32) - inv_y;
    (
        2.0 * x as f32 / w as f32 - 1.0,
        2.0 * y as f32 / h as f32 - 1.0
    )
}


pub fn viewport_to_pixel(viewport_pos: ViewportCoord, viewport_size: ViewportSize) -> PixelCoord
{
    let (x, inv_y) = viewport_pos;
    let (w, h) = viewport_size;
    let y = -inv_y;
    (
        (((x + 1.0) / 2.0) * (w as f32)) as i32,
        (((y + 1.0) / 2.0) * (h as f32)) as i32
    )
}


#[cfg(test)]
mod tests
{
    use super::*;


    fn test_pixel_to_viewport(pixel_pos: PixelCoord, viewport_pos: ViewportCoord)
    {
        assert_eq!(pixel_to_viewport(pixel_pos, (1600, 900)), viewport_pos)
    }


    fn test_viewport_to_pixel(viewport_pos: ViewportCoord, pixel_pos: PixelCoord)
    {
        assert_eq!(viewport_to_pixel(viewport_pos, (1600, 900)), pixel_pos)
    }


    #[test]
    fn pixel_to_viewport_is_accurate()
    {
        test_pixel_to_viewport((0, 0), (-1.0, 1.0));
        test_pixel_to_viewport((1600, 0), (1.0, 1.0));
        test_pixel_to_viewport((1600, 900), (1.0, -1.0));
        test_pixel_to_viewport((0, 900), (-1.0, -1.0));
        test_pixel_to_viewport((800, 450), (0.0, 0.0));
    }

    #[test]
    fn viewport_to_pixel_is_accurate()
    {
        test_viewport_to_pixel((-1.0, 1.0), (0, 0));
        test_viewport_to_pixel((1.0, 1.0), (1600, 0));
        test_viewport_to_pixel((1.0, -1.0), (1600, 900));
        test_viewport_to_pixel((-1.0, -1.0), (0, 900));
        test_viewport_to_pixel((0.0, 0.0), (800, 450));
    }
}