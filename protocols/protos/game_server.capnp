@0x9871a19c07269466;

struct MessageMetadata {
    origin @0 :Identity;
    messageAuthentication @1 :Data;

    union {
        broadcast @2 :Void;
        direct @3 :DirectMessage;
    }
}

struct DirectMessage {
    recipient @0 :Identity;
}

struct Identity {
    enum Role {
        player @0;
        gameServer @1;
        utility @2;
    }

    name @0 :Text;
    publicKey @1 :Data;
    role @2 :Role;
}

interface GameServerCommands {
    constructBuilding @0 (player :Identity, options :ConstructionOptions);
    endTurn @1 (player :Identity);
    buyDevelopmentCard @2 (player :Identity);
    playDevelopmentCard @3 (player :Identity, developmentCard :DevelopmentCard);
    exchangeResource @4 (player :Identity, source :DevelopmentCard);
    postTradeOffer @5 (player :Identity, offered :ResourceCollection, requested :ResourceCollection);
    acceptTradeOffer @6 (player :Identity);
}

struct ResourceCollection {
    ore @0 :Int32;
    brick @1 :Int32;
    grain @2 :Int32;
    wool @3 :Int32;
    lumber @4 :Int32;
}

enum ExchangeSource {
    bank @0;
    harbor @1;
}

enum DevelopmentCard {
    knight @0;
    roadBuilding @1;
    yearOfPlenty @2;
    victoryPoint @3;
}

struct ConstructionOptions {
    enum BuildingType {
        road @0;
        settlement @1;
        city @2;
    }

    type @0 :BuildingType;
    location @1 :BoardLocation;
}

struct BoardLocation {
    p @0 :Int32;
    q @1 :Int32;
    r @2 :Int32;
}

struct Map(Key, Value) {
    entries @0 :List(Entry);

    struct Entry {
        key @0 :Key;
        value @1 :Value;
    }
}
