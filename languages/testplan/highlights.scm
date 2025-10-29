(comment) @comment

(subtask_header 
  "[" @punctuation.bracket 
  "Subtask" @title 
  (number) @title 
  "]" @punctuation.bracket)

["copy" "echo"] @function

(script
  command: (_) @function)

(group_name) @variable.special

(directive) @type
