local msg="总计@1,伤害@2"
local i=1
local sum=100

local result = string.gsub(msg,"@1",i)
local temp = string.gsub(result,"@2",sum)
if temp then
  result = temp;
end
print("result",result)
