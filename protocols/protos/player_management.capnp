@0x9debcf687b86d643;

interface PlayerManagement {
    loginPlayer @0 (username :Text);
    requestJoinGame @1 (username :Text, gameName :Text);
    logoutPlayer @2 (username :Text);
}
