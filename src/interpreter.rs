use crate::ast;

enum Value {
    Str(String),
    Void,
}

fn evaluate(term: ast::Term) -> Value {
    match term {
        ast::Term::Str(str) => return Value::Str(str.value),
        ast::Term::Print(print) => {
            let value = evaluate(*print.value);
            match value {
                Value::Str(str) => print!("{str}"),
                _ => todo!(),
            };
            Value::Void
        }
    }
}

pub fn interpret_ast_file(ast_file: ast::File) {
    // todo: create context

    evaluate(ast_file.expression);
}
