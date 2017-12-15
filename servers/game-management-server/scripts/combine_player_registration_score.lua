-- KEYS[1] the name of the sorted set that will contain the game uuid scored
--          by number of players in the game
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 1"
-- KEYS[2] the name of the sorted set that will contain the game uuid scored
--          by the seconds since epoch formatted a flat
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 0.1513222545"
-- KEYS[3] the name of the sorted set that will contain the game uuid scored
--          by the number of players desired for a game
--          example "f16ccb53-7871-5fee-8dcf-eddc4f70ac47 4"
-- KEYS[4] is the set that will contain the accumulated score result

redis.call("ZUNIONSTORE", KEYS[4], 3, KEYS[1], KEYS[2], KEYS[3], "WEIGHT", "1", "-1", "-1", "AGGREGATE", "SUM")
