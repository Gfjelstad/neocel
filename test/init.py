from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from stub import API, api

print("Hello from python")
print(api.call)

def my_func(api: API, args):
    print("called from python")
    api.call("test")


def change_mode(api: API, args):
    api.call("doc.changeMode", {"mode":"insert"})

api.call("command.register_command", {
    "id": "say_hello",
    "function": my_func})
api.call("command.register_command", {
    "id": "change_mode",
    "function": change_mode})

api.call("command.register_keybind", {
    "keys": ["ctrl+h"],
    "command_id": "say_hello"
})

api.call("command.register_keybind", {
    "keys": ["ctrl+i"],
    "command_id": "change_mode"
})



