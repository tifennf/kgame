# Kaing's Game (Kgame)
Reimplementation of my Golang Kgame but in Rust and with many more features.

## Description
Kaing's Game is a turn-based board game.

It is about aligning 5 dots in order to form a "Kaing" and to win.

You can play it on paper like that:

    you take a squared paper sheet
    2 pens of differents colors ( player 1 and player 2 )
    each in turn, a player will put a dot/cross on a intersection
    first to make a Kaing win !

Of course, you can play it with more players.

## HTTP Web server
There is a web server running on another thread inside the application, you can use it to play the game with HTTP request.

## TODO
- Implement a reinforcement learning algorithm using game HTTP endpoint
