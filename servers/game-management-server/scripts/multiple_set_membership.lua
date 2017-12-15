-- KEYS[1] is the name of the temporary set that will store the results of the union
-- KEYS[2..n] are the sets that will be unioned into the temporary set
-- ARGV[1] is the member to check for in the resulting union

local result = nil;
local temp_set_key = table.remove(KEYS, 1)
local number_of_sets = table.getn(KEYS)

if redis.call("EXISTS", unpack(KEYS)) == number_of_sets then
    if redis.call("SUNIONSTORE", temp_set_key, unpack(KEYS)) > 0 then
        if redis.call("SISMEMBER", temp_set_key, ARGV[1]) == 1 then
            result = 1
        else
            result = 0
        end
    else
        result = 0
    end
else
    result = redis.error_reply("On of the sets passed does not exist")
end

redis.call("DEL", temp_set_key)
return result