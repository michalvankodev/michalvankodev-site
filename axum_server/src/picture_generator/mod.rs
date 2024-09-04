/*!
This is going to be an attempt for creating HTML markup for serving and generating images
for the most common PIXEL_DENSITIES.
It should create `<picture>` elements with following features:

- least amount of needed arguments
- for each pixel density it should have a definition in `srcset`
- create a `avif` type for the image for each pixel_density
- create an image in the original format for each pixel_density
- support case of `svg` therefore not doing any of the pixel_density logic

These features might be considered later:
- support case for art direction (different pictures for different screen sizes)


TODO: figure wether `height` or `width` have to be known ahead of time

## Usage

It can be used from the rust code
It should be used from the templates as well
*/

pub mod export_format;
pub mod picture_markup_generator;
