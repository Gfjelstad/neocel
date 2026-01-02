## keyinput -> Key -> Operator/Motion/Command/Digit -> Command -> CommandDispatcher -> Engine -> Render


### keyinput -> Key: Key::from(KeyEvent)


### Key -> Operator/Motion/Command/Digit
input_engine.parse_key(&engine, key)
  - keymap -> operator_map/motion_map/command_map
    - each map will look for document based node, then window, then global, based on the mode?

### Operator/Motion/Command/Digit -> Command
  - if the command is complete (TODO: WTF DOES THIS MEAN)
  create command from input_engine::pending

### Command -> CommandDispatcher dispatcher.dispatch(CommandContext, Command)

    
