# HiddenGems
This project is open-source. Feel free to contribute!

## What it is
This project aims to create a steganography utility written in rust. It encodes data to images in the png format (jpg has weird compression I don't understand that breaks pixels, I stuck to png)

The idea is that you encode text in an image's pixels. Consider the following simplification :

An image is a matrix of pixels, each pixel has a number representing the color of the pixel. consider our 10x10 canvas :
```
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
```
And now with color : 
```
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][ ][X][X][ ][ ][X][X][ ][ ]
[ ][ ][X][X][ ][ ][X][X][ ][ ]
[ ][ ][X][X][ ][ ][X][X][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
[ ][X][X][ ][ ][ ][ ][X][X][ ]
[ ][ ][X][X][ ][ ][X][X][ ][ ]
[ ][ ][ ][X][X][X][X][ ][ ][ ]
[ ][ ][ ][ ][X][X][ ][ ][ ][ ]
[ ][ ][ ][ ][ ][ ][ ][ ][ ][ ]
```

Each pixel in this canvas will be simplified as black or white. Each color in a computer is represented by a set of numbers called RGB (Red, Green, Blue). a RGB value would