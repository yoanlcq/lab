# color_stds

A crate providing numerous named color constants from various existing
standards, including (but not limited to) CSS and XKCD.  
It also allows, given an arbitrary color value and a threshold, to retrieve the names
of the closest constants (and, given a name, to retrieve the matching constant).  
You are free to choose how you represent colors : as long as your color types implement
`From<color_stds::Rgb24>`, then they can implement any number of `color_stds`'s traits, which
would automatically give them color-name superpowers.

# Example
FIXME : See the docs
# Use cases

