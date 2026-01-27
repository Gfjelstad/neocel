import api
from theme import set_theme

set_theme(api)

def my_function():
    print("Printing from inside my python function")

api.commands.register("test_command", function=my_function)
api.commands.run("test_command")

content = "WHAT"
times: int = 0

api.keybinds.bind("n", ["ctrl+q"], function=lambda: api.engine.kill())
api.keybinds.bind("n", ["i"], function=lambda: api.document.change_mode("input"))

def move(drow: int, dcol: int):
     (row, col) = api.document.get_cursor()
     api.document.set_cursor(row + drow, col + dcol)

api.keybinds.bind("n", ["h"], function=lambda: move(-1,0)) #up
api.keybinds.bind("n", ["j"], function=lambda: move(0, -1)) #left
api.keybinds.bind("n", ["k"], function=lambda: move(0,1)) #right
api.keybinds.bind("n", ["l"], function=lambda: move(1,0)) #down


def createForm():
    global times
    times += 1
    doc_id = api.text.create(content=content + str(times));
    print("doc", doc_id)
    win_id = api.engine.create_window(options={"doc": doc_id, "enter": True, "relative": "editor", "position": "center", "width":50, "height":10})
    print("win_id", win_id)
    def closeForm():
            api.engine.close_window(win_id)
    # def submitForm():
    #         lines = api.text.content(doc_id)
    #         print(lines)
    #         api.engine.close_window(win_id)
    api.keybinds.bind("n", ["enter"], function=closeForm, options={"doc": doc_id})
    # api.keybinds.bind("n", ["enter"], function=submitForm, options={"doc": doc_id})

api.commands.register("create_form", function=createForm)
api.keybinds.bind("n", ["ctrl+k"], command={"id":"create_form"})

print(api.commands.list_current())
