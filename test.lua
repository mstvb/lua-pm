local class_module = require("class.class")

-- The module seems to return a table 'Class' containing the logic
local MyClass = class_module:new("MyClass")
MyClass:set("greeting", "Hello from mstvb/lua-libs!")

print("Class Name: " .. MyClass:__str__())
print("Greeting: " .. MyClass:get("greeting"))

print("Class successfully imported from mstvb/lua-libs!")
