-- KEYS[1] the name of the sorted set that will contain the game uuid scored 
--          by number of players in the game
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 1"
-- KEYS[2] the name of the sorted set that will contain the game uuid scored
--          by the seconds since epoch formatted a flat
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 0.1513222545"
-- KEYS[3] the name of the sorted set that will contain the game uuid scored
--          by the number of players desired for a game
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 4"
-- ARGV[1] the uuid of the game that will be tracked 
-- ARGV[2] the system time in milliseconds
-- ARGV[3] the max number of players allowed in the game

redis.call(ZADD, KEYS[1], 0, ARGV[1]) -- add to current player count index
redis.call(ZADD, KEYS[2], ARGV[2], ARGV[1]) -- add to time index
redis.call(ZADD, KEYS[3], ARGV[3], ARGV[1]) -- add to max players count index