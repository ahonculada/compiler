# compiler

# The Calc Language
| Function | Description |
|----------|-------------|
|parse_program| This parses the whole *Calc* program.|
|parse_declaration|This parses a *Calc* declaration statment, such as @total.|
|parse_input_statement|This parses a *Calc* input statment, such as >addend.|
|parse_output_statement|This parses a *Calc* output statment.|
|parse_assignment|This parses a *Calc* such as total := addend * 2.|
|parse_expr|This parses a *Calc* expression, such as addend * 2 + val / (incr + 1).|
|parse_term|This parses a *Calc* term, such as val / (incr + 1).|
|parse_factor|This parses a *Calc* factor, such as incr, or 4.56e12, or (incr + 1).|
|parse_subexpr|This parses a *Calc* parenthesized expression, such as (incr + 1).|
|parse_identifier|This parses a *Calc* identifier, such as addend.|
|skip_spaces|This parses a sequence of zero or more white spaces.|

