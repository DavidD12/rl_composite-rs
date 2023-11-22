//// programming language version grammar

Include
    "include" "{" skillset_path+ "}"

Type
    "type" "{" input_type+ "}"

Function
    "def" function_id "(" function_input+ ")"
    "-> (" (function_return ",")+ ")"
    "{"
        expression
    "}"

Function_return
    | "success" return_id return_output
    | "failure" return_id
    | "_"

Return_output
    | "_"
    | output_id

Function_input
    | robot_name ": skillset " skillset_name
    | input_name ": " input_type

Expression
    | function_call
    | skill_call
    | sequence_expression
    | choice_expression
    | match_expression
    | parallel_expression
    | race_expression
    | if_else_expression

Function_call
    function_id "(" (input_name ",")+ ")"

skill_call
    skillset_name "." skill_name "(" (input_name ",")+ ")"

Match_expression
    "match " function_call " {"
        ("|" function_return "->" function_return )+
    "}"

sequence_expression
    (expression ">>")+

choice_expression
    (expression "+")+

parallel_expression
    (expression "|")+

race_expression
    "race(" (expression ", ")+ ")"
race_failure_expression
    "race_failure(" (expression ", ")+ ")"

if_else_expression
    "if" "(" condition_expression ")" "{"
        expression
    "}"
    ["else if" "(" condition_expression ")" "{"
        expression
    "}"]
    ["else" "{"
        expression
    "}"]

condition_expression
    | expression
    | resource_guard
    | value_guard

Resource_guard = logical expr on resource states
Value_guard = logical expr on data/output/input values