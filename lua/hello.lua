
function Len(...)
    local arg=select(6,...)
    print("arg: "..arg)
end

Len(9,4,5,1,2,3)


function square(iteratorMaxCount,currentNumber)
    if currentNumber<iteratorMaxCount
    then
       currentNumber = currentNumber+1
    return currentNumber, currentNumber*currentNumber
    end
 end

 for i,n in square,4,0
 do
    print(i,n)
 end

 print('-------分割线-------')

 function iter (a, i)
    print("a=",a)
    i = i + 1
    local v = a[i]
    if v then
       return i, v
    end
end

function ipairs (a)
    return iter, a, 0
end


tab1 = { "val4",key1 = "val1", key2 = "val2", "val3" }
for k, v in pairs(tab1) do
    print("k= "..k .. "   v= " .. v)
end

tab1.key1 = nil
for k, v in ipairs(tab1) do
    print(k .. " - " .. v)
end
