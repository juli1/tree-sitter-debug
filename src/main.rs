use tree_sitter::QueryCursor;
use crate::file::FILE;

mod file;



fn get_tree_sitter_java() -> tree_sitter::Language {
    extern "C" {
        fn tree_sitter_java() -> tree_sitter::Language;
    }
    unsafe {tree_sitter_java()}
}

fn main() {


    let query_code: &str = r#"
(block
    (local_variable_declaration
        type: (type_identifier) @type
        declarator: (variable_declarator
            name: (identifier) @varname
        )

    ) @decl
    (_)*
    (expression_statement
        (method_invocation
            object: (identifier) @object1
            name: (identifier) @name1
        )

    )? @defaulttyping
    (_)*
    (expression_statement
        (method_invocation
            object: (identifier) @object2
            name: (identifier) @name2
        )

    )
    (#eq? @object1 @varname)
    (#eq? @name1 "enableDefaultTyping")
    (#eq? @type "ObjectMapper")
    (#eq? @object2 @varname)
    (#eq? @name2 "readValue")
)"#;
    println!("Hello, world!");
    let mut tree_sitter_parser = tree_sitter::Parser::new();
    let tree_sitter_language = get_tree_sitter_java();
    tree_sitter_parser
        .set_language(&tree_sitter_language)
        .unwrap();
    let ts_tree = tree_sitter_parser.parse(FILE, None).unwrap();
    let mut query_cursor = QueryCursor::new();
    let query = tree_sitter::Query::new(&tree_sitter_language, query_code).unwrap();
    let query_result = query_cursor.matches(&query, ts_tree.root_node(), FILE.as_bytes());
    for _query_match in query_result {
        println!("foo");
    }
    println!("done");

}
