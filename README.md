# Diamond Square
Small project for first time use of Rust language to implement the Diamond-Square Algorithm for terrain generation. Provides a library to create the terrain map dependent on the 'detail' level. Note that the image buffer generated will be of size _2^detail + 1_.

To generate terrain will need to use the `create` function. To write this to an image buffer will need to normalize it using `normalize_pixel_map`.

## Build
`cargo build` is all you will need

### TODO
* Documentation
* More tests
* Add function to draw the image 
