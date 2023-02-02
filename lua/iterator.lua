local function iterator(array)
    local index = 0
    return function()
        index=index+1
        local value=array[index];
        if value then
        return index,value
        end
    end
end

local function iterator(array)
    local index = 0
    return function()
        index=index+1
        local value=array[index];
        if value then
        return index,value
        end
    end
end

local arr={0,1,2,3}
for k,v in iterator(arr)
do
    print(k,v)
end