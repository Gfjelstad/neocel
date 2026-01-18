print("Hello from python")
print(api.call)

def my_func(api, args):
    print("called from python")
    api.call("test")

api.call("command.register_command", {
    "id": "say_hello",
    "function": my_func})
api.call("command.register_keybind", {
    "keys": ["ctrl+h"],
    "command_id": "say_hello"
})


