# RustBattleship

Battleship made in rust, made for [#arcade](https://hackclub.com/arcade/).

This game has 4 ships:  
```
██
 ██

█
█
█
█

█
█
█

and

█
█
```

When starting you will be asked to positon you ships, you can use `wasd` for moving `r` for rotating, `q` for changing the selected ship and `e` when you're done. Player 2 the does the same

Then comes the main game loop, Players will take turns shooting, if they hit a ship they may go again, if they dont the other player gets to go.

When shooting you can select a single row by using a letter like `d` or a single collum by using a number `5` or both by using letternumber `d5`

Thr game goes on until one player hits all the ships
