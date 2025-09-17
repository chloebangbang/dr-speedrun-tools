# Ralsei Quiz Tracker 
---
A little program I made to display in OBS Ralsei's quiz choices in Deltarune chapter 3, how likely you were to get those answers or better, how much time that pattern lost you, and how much time Ralsei has lost you over the whole couse of using the program. Exactly how this program implements interfacing with OBS is currently extremely jank, but it is technically functional and that's what's important I think. It currently only works on Windows, but the only system that isn't cross-platform is the hotkey system. 

## How do I use the progam?
To use the program, simply run the executable and it will begin listening for input. When Ralsei answers a quiz question, press the number of the quiz option he selected, while holding control and shift (or, while you have them sticky keysed down, whatever floats your boat). Once you have three answers punched in, you can simply press any number 1-4 while holding control and shift to clear the display. The program will automatically save Ralsei's quiz answers as soon as you input the third one, so don't worry about having to clear the display before closing. 

## But I can't see anything! 
That's because this program is designed to be fed directly into an OBS scene! It outputs any values that are meant to be displayed to files in your temp directory, where you can display them through OBS by creating a text display that reads from file. 

## But chloe, that sounds like just about the worst way you could have done this! 
Yeah. But it was easy and it works, and more importantly, I will be in the cold hard ground before you get me to do GUI programming. 

## *Sigh.* So where are these files anyway?
- Current run timeloss: `%tmp%/chloebangbang/board_timeloss.txt`
- Lifetime timeloss: `%tmp%/chloebangbang/total_timeloss.txt`
- Ralsei's first answer: `%tmp%/chloebangbang/ralsei1.txt`
- Ralsei's second answer: `%tmp%/chloebangbang/ralsei2.txt`
- Ralsei's third answer: `%tmp%/chloebangbang/ralsei3.txt`
-Chance of getting these answers or better: `%tmp%/chloebangbang/board_chance.txt`

## What if I'm a turbo-nerd and want to do my own statistics on my Board 3 Ralsei quiz answers? 
I have good news! All saved board 3s are saved in JSON format over at `%appdata%/chloebangbang/ralsei-quiz-tracker.json`. Go wild. 
