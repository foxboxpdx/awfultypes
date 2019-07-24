# Awful Types
The worst data types you could ever hope (to never see in use anywhere ever)

## What? Why?
Because I spilled a big cup of salsa all over my jeans and it made me sad so I came up with these to make EVERYONE sad.

## Ok what are they?
Glad you asked!

### The Shambles
Imagine a typical Vec in Rust as a nice organized stack of data...like a Jenga tower.  Then imagine someone has lost the game of Jenga.  Everything's in
a big messy pile on the floor!  You think you see the data you want, so you reach for an index, only to find it's a completely random value!  And your 
rummaging around in the pile has knocked all the other data out of place!  Dang it!

The shambles operates like a normal vector, but every time you so much as look at it funny, it randomizes all the data it's holding.

### The MashHap
A HashMap is a very excellent data structure implemented in pretty much every language ever.  But what if instead of a set of nice, organized key-value 
pairs, it was more of a...two-dimensional array...where `var[x][0]` is the key and `var[x][1]` is the value?  That would be pretty awful if it existed. 
Good news! Now it does!

### The Foolean
What if you had a Bool with low self-confidence?  Or one that was just a little bit incompetent? That's the Foolean - a Bool with a hidden Float value 
representing the bool's confidence, randomly set at initialization.  When accessed, a random number is generated; if it's less than the confidence float, 
you get the bool's value.  If it's greater than, you get a different answer.  If it's exactly the same, well, who knows what you'll get?  Exciting!

## You should be ashamed of yourself.
Oh I am, buddy.  I absolutely am.


##### v0.0.1 23/Jul/19
