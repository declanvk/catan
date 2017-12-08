@0xa2b08bf4e6ef41c4;

interface GameManagement {
    registerNewGame @0 (gameName :Text, options :GameOptions);
    openPlayerRegistration @1 (gameName :Text);
    startGame @2 (gameName :Text);
    endGame @3 (gameName :Text);
    cleanupGame @4 (gameName :Text);
}

struct GameOptions {
    maxPlayers @0 :Int32;
    publicListing @1 :Bool;
    turnTimeout @2 :Int32;
}
