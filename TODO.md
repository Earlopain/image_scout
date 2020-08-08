# Rework compare ui
- not enough space to show everything at once
- Comapre slider should take up 50% of the screen on the right side
- Hotkeys to change currently selected image (1-9 and left/right arrow key)

# Write own image comparator
All kinds of weird things happen when you try to resize it because everything is done in absolutes.  
You also create a new instance if you want to switch images, instead of just switching them out.

# Implement post fetching
- Twitter
- Pixiv
- Deviantart
- Artstation
- Furaffinity
- Sofurry
- Inkbunny
- Furrynetwork
- Weasly

# Image hotlinking
Some pages might allow hotlinking their images, which would reduce our own bandwidht usage.  
It would also make sense to save a hotlinkable thumbnail in the db.

# Missing UI compontents
- Creating artists
- Adding pages to artists
- Adding aliases to artists
- Artist overview (all posts, links, aliases, etc.)
- Image info template (widht, height, source, etc.)
