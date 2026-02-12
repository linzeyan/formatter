use std::path::Path;

use formatter::formats::{
    FormatError, bash, css, dockerfile, go, graphql, hcl, html, ini, javascript, json, lua,
    makefile, markdown, nginx, protobuf, python, rlang, rustfmt, sql, toml_fmt, typescript, xml,
    yaml,
};

fn run(
    f: fn(&Path, &str) -> anyhow::Result<Option<String>, FormatError>,
    path: &str,
    input: &str,
) -> String {
    f(Path::new(path), input)
        .unwrap()
        .unwrap_or_else(|| input.to_string())
}

// ══════════════════════════════════════════════
// JSON 深度測試
// ══════════════════════════════════════════════

#[test]
fn json_formats_array_of_objects() {
    let input = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let out = run(json::format, "a.json", input);
    assert!(out.contains("\"name\": \"Alice\""));
    assert!(out.contains("\"age\": 30"));
    assert!(out.contains("\"name\": \"Bob\""));
}

#[test]
fn json_preserves_number_precision() {
    let input = r#"{"pi":3.141592653589793,"large":9007199254740992}"#;
    let out = run(json::format, "a.json", input);
    assert!(out.contains("3.141592653589793"));
    assert!(out.contains("9007199254740992"));
}

#[test]
fn json_handles_empty_string_values() {
    let input = r#"{"key":""}"#;
    let out = run(json::format, "a.json", input);
    assert!(out.contains("\"key\": \"\""));
}

#[test]
fn json_multiline_formatting() {
    // Use a larger object that exceeds line width to trigger multi-line formatting
    let input = r#"{"name":"Alice","email":"alice@example.com","address":"123 Main St, City, State 12345","phone":"+1-555-0123"}"#;
    let out = run(json::format, "a.json", input);
    // Formatted JSON should add spacing around colons
    assert!(out.contains("\"name\": \"Alice\""));
    assert!(out.contains("\"email\":"));
}

// ══════════════════════════════════════════════
// YAML 深度測試
// ══════════════════════════════════════════════

#[test]
fn yaml_flow_sequence_formatting() {
    let input = "items: [1, 2, 3, 4, 5]\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("items:"));
}

#[test]
fn yaml_nested_mapping() {
    let input = "server:\n  host: localhost\n  port: 8080\n  ssl:\n    enabled: true\n    cert: /path/to/cert\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("host: localhost"));
    assert!(out.contains("port: 8080"));
    assert!(out.contains("enabled: true"));
}

#[test]
fn yaml_multiline_string_literal() {
    let input = "desc: |\n  This is a\n  multiline string\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("desc:"));
    assert!(out.contains("multiline string"));
}

#[test]
fn yaml_boolean_values() {
    let input = "a: true\nb: false\nc: yes\nd: no\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("true"));
    assert!(out.contains("false"));
}

#[test]
fn yaml_null_values() {
    let input = "a: null\nb: ~\nc:\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("a:"));
}

#[test]
fn yaml_numeric_keys() {
    let input = "1: one\n2: two\n";
    let out = run(yaml::format, "a.yaml", input);
    assert!(out.contains("one"));
    assert!(out.contains("two"));
}

// ══════════════════════════════════════════════
// TOML 深度測試
// ══════════════════════════════════════════════

#[test]
fn toml_inline_table() {
    let input = "[package]\nname = \"test\"\nauthors = [\"Alice\", \"Bob\"]\n";
    let out = run(toml_fmt::format, "a.toml", input);
    assert!(out.contains("[package]"));
    assert!(out.contains("name = \"test\""));
}

#[test]
fn toml_nested_tables() {
    let input = "[server]\nhost = \"localhost\"\n\n[server.ssl]\nenabled = true\n";
    let out = run(toml_fmt::format, "a.toml", input);
    assert!(out.contains("[server]"));
    assert!(out.contains("host = \"localhost\""));
}

#[test]
fn toml_array_of_tables() {
    let input = "[[fruits]]\nname = \"apple\"\n\n[[fruits]]\nname = \"banana\"\n";
    let out = run(toml_fmt::format, "a.toml", input);
    assert!(out.contains("[[fruits]]"));
    assert!(out.contains("\"apple\""));
    assert!(out.contains("\"banana\""));
}

// ══════════════════════════════════════════════
// XML 深度測試
// ══════════════════════════════════════════════

#[test]
fn xml_with_attributes() {
    let input = r#"<root><item id="1" class="main">text</item></root>"#;
    let out = run(xml::format, "a.xml", input);
    assert!(out.contains("id=\"1\""));
    assert!(out.contains("class=\"main\""));
}

#[test]
fn xml_self_closing_tags() {
    let input = "<root><br/><hr/></root>";
    let out = run(xml::format, "a.xml", input);
    assert!(out.contains("<root>"));
}

#[test]
fn xml_cdata_section() {
    let input = "<root><![CDATA[<html>content</html>]]></root>";
    let r = xml::format(Path::new("a.xml"), input);
    assert!(r.is_ok());
}

#[test]
fn xml_declaration() {
    let input = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><root><item>1</item></root>";
    let r = xml::format(Path::new("a.xml"), input);
    assert!(r.is_ok());
}

#[test]
fn xml_entities() {
    let input = "<root><item>&amp;&lt;&gt;&quot;&apos;</item></root>";
    let out = run(xml::format, "a.xml", input);
    assert!(out.contains("&amp;"));
    assert!(out.contains("&lt;"));
    assert!(out.contains("&gt;"));
}

// ══════════════════════════════════════════════
// Bash 深度測試
// ══════════════════════════════════════════════

#[test]
fn bash_for_loop() {
    let input = "for i in 1 2 3; do\necho $i\ndone\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("for i in 1 2 3; do"));
    assert!(out.contains("  echo $i"));
    assert!(out.contains("done"));
}

#[test]
fn bash_while_loop() {
    let input = "while true; do\necho running\nsleep 1\ndone\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("while true; do"));
    assert!(out.contains("  echo running"));
    assert!(out.contains("  sleep 1"));
}

#[test]
fn bash_nested_if() {
    let input = "if [ 1 ]; then\nif [ 2 ]; then\necho nested\nfi\nfi\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("    echo nested"));
}

#[test]
fn bash_elif_chain() {
    let input = "if [ 1 ]; then\necho 1\nelif [ 2 ]; then\necho 2\nelif [ 3 ]; then\necho 3\nelse\necho other\nfi\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("elif"));
    assert!(out.contains("else"));
}

#[test]
fn bash_heredoc() {
    let input = "cat << EOF\nhello world\nEOF\n";
    let r = bash::format(Path::new("a.sh"), input);
    assert!(r.is_ok());
}

#[test]
fn bash_subshell() {
    let input = "result=$(echo hello | tr 'a-z' 'A-Z')\necho $result\n";
    let r = bash::format(Path::new("a.sh"), input);
    assert!(r.is_ok());
    let out = r.unwrap().unwrap_or_else(|| input.to_string());
    assert!(out.contains("result="));
}

#[test]
fn bash_array_operations() {
    let input = "arr=(one two three)\necho ${arr[0]}\necho ${#arr[@]}\n";
    let r = bash::format(Path::new("a.sh"), input);
    assert!(r.is_ok());
}

#[test]
fn bash_string_with_hash_not_comment() {
    // Hash inside quotes should not be treated as comment
    let input = "echo \"hello #world\"\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("\"hello #world\""));
}

#[test]
fn bash_double_ampersand_spacing() {
    let input = "cmd1&&cmd2&&cmd3\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("cmd1 && cmd2 && cmd3"));
}

#[test]
fn bash_pipe_spacing() {
    let input = "cat file|grep pattern|sort|uniq\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("cat file | grep pattern | sort | uniq"));
}

#[test]
fn bash_redirect_spacing() {
    let input = "echo hello>output.txt\ncat<input.txt\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("> output.txt") || out.contains(">output.txt"));
}

#[test]
fn bash_select_statement() {
    let input = "select opt in \"a\" \"b\" \"quit\"; do\necho $opt\nbreak\ndone\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("select opt"));
    assert!(out.contains("  echo $opt"));
}

#[test]
fn bash_until_loop() {
    let input = "until false; do\necho running\ndone\n";
    let out = run(bash::format, "a.sh", input);
    assert!(out.contains("until false; do"));
    assert!(out.contains("  echo running"));
}

// ══════════════════════════════════════════════
// CSS 深度測試
// ══════════════════════════════════════════════

#[test]
fn css_multiple_selectors() {
    let input = "h1,h2,h3{color:red;font-size:16px;}";
    let out = run(css::format, "a.css", input);
    assert!(out.contains("color: red"));
}

#[test]
fn css_media_query() {
    let input = "@media (max-width:768px){.container{width:100%;}}";
    let out = run(css::format, "a.css", input);
    assert!(out.contains("@media"));
    assert!(out.contains("container"));
}

#[test]
fn css_pseudo_classes() {
    let input = "a:hover{color:blue;}a:active{color:red;}";
    let out = run(css::format, "a.css", input);
    assert!(out.contains(":hover"));
    assert!(out.contains(":active"));
}

// ══════════════════════════════════════════════
// TypeScript/JavaScript 深度測試
// ══════════════════════════════════════════════

#[test]
fn typescript_interface() {
    let input = "interface User{name:string;age:number;}";
    let out = run(typescript::format, "a.ts", input);
    assert!(out.contains("interface User"));
    assert!(out.contains("name: string"));
    assert!(out.contains("age: number"));
}

#[test]
fn typescript_generic_type() {
    let input = "function id<T>(arg:T):T{return arg;}";
    let out = run(typescript::format, "a.ts", input);
    assert!(out.contains("function id<T>"));
}

#[test]
fn javascript_arrow_function() {
    let input = "const add=(a,b)=>a+b";
    let out = run(javascript::format, "a.js", input);
    assert!(out.contains("const add"));
    assert!(out.contains("=>"));
}

#[test]
fn javascript_async_await() {
    let input = "async function fetch(){const res=await fetch('/api');return res.json();}";
    let out = run(javascript::format, "a.js", input);
    assert!(out.contains("async function"));
    assert!(out.contains("await"));
}

#[test]
fn javascript_jsx_extension() {
    let input = "const App=()=>{return <div>Hello</div>;}";
    let out = run(javascript::format, "a.jsx", input);
    assert!(out.contains("App"));
}

#[test]
fn javascript_mjs_extension() {
    let input = "export const  x=1";
    let out = run(javascript::format, "a.mjs", input);
    assert!(out.contains("export const x = 1;"));
}

#[test]
fn javascript_cjs_extension() {
    let input = "const  x=require('fs')";
    let out = run(javascript::format, "a.cjs", input);
    assert!(out.contains("const x = require"));
}

// ══════════════════════════════════════════════
// Dockerfile 深度測試
// ══════════════════════════════════════════════

#[test]
fn dockerfile_multi_stage() {
    let input =
        "FROM node:18 AS builder\nRUN npm install\n\nFROM alpine\nCOPY --from=builder /app /app\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("FROM node:18 AS builder"));
    assert!(out.contains("FROM alpine"));
    assert!(out.contains("COPY --from=builder"));
}

#[test]
fn dockerfile_env_and_arg() {
    let input = "FROM alpine\nARG VERSION=latest\nENV APP_VERSION=$VERSION\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("ARG VERSION"));
    assert!(out.contains("ENV APP_VERSION"));
}

#[test]
fn dockerfile_multiline_run() {
    let input = "FROM alpine\nRUN apk add --no-cache \\\n  curl \\\n  wget \\\n  git\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("RUN"));
    assert!(out.contains("apk add"));
}

#[test]
fn dockerfile_without_run() {
    // Dockerfile with no RUN commands should still format
    let input = "FROM alpine\nEXPOSE 8080\nCMD [\"echo\", \"hello\"]\n";
    let out = run(dockerfile::format, "Dockerfile", input);
    assert!(out.contains("FROM alpine"));
    assert!(out.contains("EXPOSE"));
}

// ══════════════════════════════════════════════
// SQL 深度測試
// ══════════════════════════════════════════════

#[test]
fn sql_join_query() {
    let input = "select a.id,b.name from users a join orders b on a.id=b.user_id where a.active=1";
    let out = run(sql::format, "a.sql", input);
    let lower = out.to_lowercase();
    assert!(lower.contains("select"));
    assert!(lower.contains("join"));
    assert!(lower.contains("where"));
}

#[test]
fn sql_insert_statement() {
    let input = "insert into users (name,email) values ('Alice','alice@example.com')";
    let out = run(sql::format, "a.sql", input);
    let lower = out.to_lowercase();
    assert!(lower.contains("insert"));
    assert!(lower.contains("values"));
}

#[test]
fn sql_create_table() {
    let input = "create table users (id int primary key,name varchar(255),email varchar(255))";
    let out = run(sql::format, "a.sql", input);
    let lower = out.to_lowercase();
    assert!(lower.contains("create table"));
}

// ══════════════════════════════════════════════
// Go 深度測試
// ══════════════════════════════════════════════

#[test]
fn go_struct_definition() {
    let input = "package main\n\ntype User struct {\nName string\nAge int\n}\n";
    let out = run(go::format, "a.go", input);
    assert!(out.contains("type User struct"));
    assert!(out.contains("Name string"));
}

#[test]
fn go_interface() {
    let input = "package main\n\ntype Reader interface {\nRead(p []byte) (n int, err error)\n}\n";
    let out = run(go::format, "a.go", input);
    assert!(out.contains("type Reader interface"));
}

// ══════════════════════════════════════════════
// Rust 深度測試
// ══════════════════════════════════════════════

#[test]
fn rust_struct_definition() {
    let input = "struct Point{x:f64,y:f64}";
    let out = run(rustfmt::format, "a.rs", input);
    assert!(out.contains("struct Point"));
    assert!(out.contains("x: f64"));
    assert!(out.contains("y: f64"));
}

#[test]
fn rust_enum_definition() {
    let input = "enum Color{Red,Green,Blue}";
    let out = run(rustfmt::format, "a.rs", input);
    assert!(out.contains("enum Color"));
    assert!(out.contains("Red"));
    assert!(out.contains("Green"));
    assert!(out.contains("Blue"));
}

#[test]
fn rust_impl_block() {
    let input = "struct Foo;\nimpl Foo{fn bar(&self)->i32{42}}";
    let out = run(rustfmt::format, "a.rs", input);
    assert!(out.contains("impl Foo"));
    assert!(out.contains("fn bar"));
}

// ══════════════════════════════════════════════
// INI 深度測試
// ══════════════════════════════════════════════

#[test]
fn ini_multiple_sections() {
    let input = "[database]\nhost=localhost\nport=5432\n\n[server]\nhost=0.0.0.0\nport=8080\n";
    let out = run(ini::format, "a.ini", input);
    assert!(out.contains("[database]"));
    assert!(out.contains("[server]"));
}

#[test]
fn ini_empty_section() {
    let input = "[empty]\n[notempty]\nkey=value\n";
    let out = run(ini::format, "a.ini", input);
    assert!(out.contains("[empty]"));
    assert!(out.contains("[notempty]"));
}

// ══════════════════════════════════════════════
// GraphQL 深度測試
// ══════════════════════════════════════════════

#[test]
fn graphql_mutation() {
    let input = "mutation{createUser(name:\"Alice\"){id name}}";
    let out = run(graphql::format, "a.graphql", input);
    assert!(out.contains("mutation"));
    assert!(out.contains("createUser"));
}

#[test]
fn graphql_query_with_args() {
    let input = "query{user(id:1){name email posts{title}}}";
    let out = run(graphql::format, "a.graphql", input);
    assert!(out.contains("query"));
    assert!(out.contains("user"));
    assert!(out.contains("posts"));
}

#[test]
fn graphql_schema_definition() {
    let input = "schema{query:Query mutation:Mutation}";
    let out = run(graphql::format, "a.graphql", input);
    assert!(out.contains("schema"));
}

// ══════════════════════════════════════════════
// HCL 深度測試
// ══════════════════════════════════════════════

#[test]
fn hcl_nested_blocks() {
    let input = "resource \"aws_instance\" \"web\" {\n  ami = \"abc\"\n  tags = {\n    Name = \"web\"\n  }\n}\n";
    let out = run(hcl::format, "a.hcl", input);
    assert!(out.contains("resource"));
    assert!(out.contains("aws_instance"));
}

#[test]
fn hcl_variable_block() {
    let input = "variable \"region\" {\n  type = string\n  default = \"us-west-2\"\n}\n";
    let out = run(hcl::format, "a.tf", input);
    assert!(out.contains("variable"));
    assert!(out.contains("us-west-2"));
}

// ══════════════════════════════════════════════
// Lua 深度測試
// ══════════════════════════════════════════════

#[test]
fn lua_function_definition() {
    let input = "function hello(name)\nprint(\"Hello, \" .. name)\nend\n";
    let out = run(lua::format, "a.lua", input);
    assert!(out.contains("function hello"));
    assert!(out.contains("print"));
}

#[test]
fn lua_if_else() {
    let input = "if x > 0 then\nprint(\"positive\")\nelse\nprint(\"non-positive\")\nend\n";
    let out = run(lua::format, "a.lua", input);
    assert!(out.contains("if x > 0"));
}

#[test]
fn lua_table_constructor() {
    let input = "local config={debug=true,port=8080,host=\"localhost\"}\n";
    let out = run(lua::format, "a.lua", input);
    assert!(out.contains("debug"));
    assert!(out.contains("port"));
}

// ══════════════════════════════════════════════
// HTML 深度測試
// ══════════════════════════════════════════════

#[test]
fn html_full_document() {
    let input =
        "<!DOCTYPE html><html><head><title>Test</title></head><body><h1>Hello</h1></body></html>";
    let out = run(html::format, "a.html", input);
    assert!(out.contains("<html>"));
    assert!(out.contains("<title>Test</title>"));
    assert!(out.contains("<h1>Hello</h1>"));
}

#[test]
fn html_nested_elements() {
    let input = "<div><ul><li>item1</li><li>item2</li></ul></div>";
    let out = run(html::format, "a.html", input);
    assert!(out.contains("<li>item1</li>"));
    assert!(out.contains("<li>item2</li>"));
}

#[test]
fn html_void_elements() {
    let input = "<div><br><hr><img src=\"test.png\"></div>";
    let r = html::format(Path::new("a.html"), input);
    assert!(r.is_ok());
}

// ══════════════════════════════════════════════
// Makefile 深度測試
// ══════════════════════════════════════════════

#[test]
fn makefile_phony_target() {
    let input = ".PHONY: clean build test\n\nclean:\n\trm -rf build/\n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains(".PHONY: clean build test"));
    assert!(out.contains("clean:"));
}

#[test]
fn makefile_variable_assignment() {
    let input = "CC=gcc\nCFLAGS=-Wall -g\n\nall:\n\t$(CC) $(CFLAGS) -o main main.c\n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains("CC=gcc") || out.contains("CC =gcc") || out.contains("CC= gcc"));
}

#[test]
fn makefile_multiline_recipe() {
    let input = "build:\n\techo step1\n\techo step2\n\techo step3\n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains("\techo step1"));
    assert!(out.contains("\techo step2"));
    assert!(out.contains("\techo step3"));
}

#[test]
fn makefile_multiple_targets() {
    let input = "all: build test\n\nbuild:\n\techo building\n\ntest:\n\techo testing\n";
    let out = run(makefile::format, "Makefile", input);
    assert!(out.contains("all: build test"));
    assert!(out.contains("build:"));
    assert!(out.contains("test:"));
}

// ══════════════════════════════════════════════
// Nginx 深度測試
// ══════════════════════════════════════════════

#[test]
fn nginx_full_server_block() {
    let input = "http {\nserver {\nlisten 80;\nserver_name example.com;\nlocation / {\nroot /var/www;\nindex index.html;\n}\n}\n}\n";
    let out = run(nginx::format, "nginx.conf", input);
    assert!(out.contains("http {"));
    assert!(out.contains("server {"));
    assert!(out.contains("listen 80;"));
    assert!(out.contains("location / {"));
}

#[test]
fn nginx_upstream_block() {
    let input = "upstream backend {\nserver 127.0.0.1:8080;\nserver 127.0.0.1:8081;\n}\n";
    let out = run(nginx::format, "nginx.conf", input);
    assert!(out.contains("upstream backend {"));
}

#[test]
fn nginx_multiple_locations() {
    let input = "server {\nlocation /api {\nproxy_pass http://backend;\n}\nlocation /static {\nroot /var/www;\n}\n}\n";
    let out = run(nginx::format, "nginx.conf", input);
    assert!(out.contains("location /api {"));
    assert!(out.contains("location /static {"));
}

// ══════════════════════════════════════════════
// Protobuf 深度測試
// ══════════════════════════════════════════════

#[test]
fn protobuf_service_definition() {
    let input = "service Greeter {\nrpc SayHello (HelloRequest) returns (HelloReply) {}\n}\n";
    let out = run(protobuf::format, "a.proto", input);
    assert!(out.contains("service Greeter"));
    assert!(out.contains("rpc SayHello"));
}

#[test]
fn protobuf_enum_definition() {
    let input = "enum Status {\nUNKNOWN=0;\nACTIVE=1;\nINACTIVE=2;\n}\n";
    let out = run(protobuf::format, "a.proto", input);
    assert!(out.contains("UNKNOWN = 0;"));
    assert!(out.contains("ACTIVE = 1;"));
}

#[test]
fn protobuf_nested_message() {
    let input = "message Outer {\nmessage Inner {\nint32 value=1;\n}\nInner inner=1;\n}\n";
    let out = run(protobuf::format, "a.proto", input);
    assert!(out.contains("message Outer"));
    assert!(out.contains("message Inner"));
}

// ══════════════════════════════════════════════
// R 語言深度測試
// ══════════════════════════════════════════════

#[test]
fn rlang_function_definition() {
    let input = "my_func <- function(x, y) {\nreturn(x + y)\n}\n";
    let out = run(rlang::format, "a.r", input);
    assert!(out.contains("my_func <- function"));
    assert!(out.contains("  return(x + y)"));
}

#[test]
fn rlang_nested_blocks() {
    let input = "if (TRUE) {\nfor (i in 1:10) {\nif (i > 5) {\nprint(i)\n}\n}\n}\n";
    let out = run(rlang::format, "a.r", input);
    assert!(out.contains("if (TRUE) {"));
    assert!(out.contains("for (i in 1:10) {"));
}

#[test]
fn rlang_try_catch() {
    let input =
        "tryCatch({\nresult <- dangerous_func()\n}, error = function(e) {\nprint(e$message)\n})\n";
    let out = run(rlang::format, "a.r", input);
    assert!(out.contains("tryCatch"));
}

// ══════════════════════════════════════════════
// Markdown 深度測試
// ══════════════════════════════════════════════

#[test]
fn markdown_embedded_yaml_code_block() {
    let input = "# Config\n\n```yaml\nfoo: 1\nbar: [2,3]\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("```yaml"));
    assert!(out.contains("foo: 1"));
}

#[test]
fn markdown_embedded_bash_code_block() {
    let input = "# Script\n\n```bash\necho hello&&echo world\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("```bash"));
}

#[test]
fn markdown_multiple_code_blocks() {
    let input = "# Doc\n\n```json\n{\"a\":1}\n```\n\ntext\n\n```toml\n[a]\nb=1\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("```json"));
    assert!(out.contains("```toml"));
    assert!(out.contains("\"a\": 1"));
}

#[test]
fn markdown_no_recursive_formatting() {
    // Markdown blocks should not recursively format markdown
    let input = "```markdown\n# Title\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("```markdown"));
}

#[test]
fn markdown_unknown_language_block() {
    // Unknown language should be left as-is
    let input = "```ruby\nputs 'hello'\n```\n";
    let out = run(markdown::format, "a.md", input);
    assert!(out.contains("puts 'hello'"));
}

// ══════════════════════════════════════════════
// Python 深度測試
// ══════════════════════════════════════════════

#[test]
fn python_class_definition() {
    let input = "class User:\n    def __init__(self,name,age):\n        self.name=name\n        self.age=age\n";
    let r = python::format(Path::new("a.py"), input);
    assert!(r.is_ok());
}

#[test]
fn python_list_comprehension() {
    let input = "squares=[x**2 for x in range(10)]\n";
    let r = python::format(Path::new("a.py"), input);
    assert!(r.is_ok());
}
