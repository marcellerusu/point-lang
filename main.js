class Tokenizer {
  static TOKENS = new Map([
    [/^class\b/, "class"],
    [/^def\b/, "def"],
    [/^\{/, "{"],
    [/^\}/, "}"],
    [/^\./, "."],
    [/^\,/, ","],
    [/^\-\>/, "->"],
    [/^\*\*/, "**"],
    [/^\*/, "*"],
    [/^\&\&/, "&&"],
    [/^\|\|/, "||"],
    [/^\+/, "+"],
    [/^\-/, "-"],
    [/^\//, "/"],
    [/^\%/, "%"],
    [/^\d+/, "int"],
    [/^[a-zA-Z0-9?!]+\b/, "id"],
    [/^\:[a-zA-Z0-9?!]+\b/, "keyword"],
    [/^\:/, ":"],
    [/^"[^".]*"/, "string"],
  ]);

  idx = 0;

  constructor(program_string) {
    this.program_string = program_string;
  }

  rest_of_string() {
    return this.program_string.slice(this.idx);
  }

  scan(regex) {
    let result = this.rest_of_string().match(regex);
    if (!result) return false;
    this.match = result[0];

    this.idx += this.match.length;
    return true;
  }

  tokenize() {
    let output = [];
    while (this.idx < this.program_string.length) {
      this.scan(/^\s+/);

      let found = false;
      for (let [regex, type] of Tokenizer.TOKENS) {
        if (this.scan(regex)) {
          output.push({ type, value: this.match });
          found = true;
        }
      }
      if (!found && this.rest_of_string().trim().length > 0) {
        console.log(this.rest_of_string());
        throw "tokenizer failed";
      }
    }
    return output;
  }
}

const OPERATORS = ["&&", "||", "**", "*", "-", "+", "/", "%"];

class Parser {
  index = 0;
  get current_token() {
    return this.tokens[this.index];
  }

  constructor(tokens) {
    this.tokens = tokens;
  }

  scan(...token_types) {
    return token_types.every(
      (type, idx) => this.tokens[this.index + idx].type === type
    );
  }

  consume(type) {
    if (this.current_token.type === type) {
      let token = this.current_token;
      this.index += 1;
      return token.value;
    }
  }

  parse() {
    let ast = [];
    while (this.current_token) {
      let expr = this.parse_expr();
      if (this.scan(".")) {
        this.consume(".");
        ast.push(expr);
      } else {
        let args = [];
        while (!this.scan(".")) {
          args.push(this.parse_expr());
        }
        ast.push({ type: "method_call", lhs: expr, args });
        this.consume(".");
      }
    }
    return ast;
  }

  parse_expr() {
    if (this.scan("class")) {
      return this.parse_class();
    } else if (this.scan("id", "{")) {
      return this.parse_record_constructor();
    } else if (this.scan("id")) {
      return this.parse_id();
    } else if (this.scan("keyword")) {
      return this.parse_keyword();
    } else if (this.scan("def")) {
      return this.parse_def();
    } else if (this.scan("int")) {
      return this.parse_int();
    } else if (this.scan("string")) {
      return this.parse_string();
    } else if (OPERATORS.includes(this.current_token.type)) {
      return this.parse_operator();
    } else {
      console.log(this.index, this.tokens.slice(this.index));
      throw "wtf";
    }
  }

  parse_int() {
    let num = this.consume("int");
    return { type: "int", value: num };
  }

  parse_operator() {
    let { value } = this.current_token;
    this.index += 1;
    return { type: "operator", op: value };
  }

  parse_string() {
    let value = this.consume("string");
    return { type: "string", value: value.slice(1, -1) };
  }

  parse_keyword() {
    let value = this.consume("keyword");
    return { type: "keyword", value: value.slice(1) };
  }

  parse_id() {
    let value = this.consume("id");
    return { type: "id", value };
  }

  parse_record_constructor() {
    let class_name = this.consume("id");
    this.consume("{");
    let kw_args = {};
    while (!this.scan("}")) {
      let name = this.consume("id");
      this.consume(":");
      let value = this.parse_expr();
      kw_args[name] = value;
      this.consume(",");
    }
    // todo: arguments
    this.consume("}");
    return { type: "record_constructor", kw_args, class_name };
  }

  parse_pattern() {
    return this.parse_expr();
  }

  parse_def() {
    this.consume("def");
    let patterns = [];
    while (!this.scan("->")) {
      patterns.push(this.parse_pattern());
    }
    this.consume("->");
    let return_expr = this.parse_expr();
    this.consume(".");
    return { type: "def", patterns, return_expr };
  }

  parse_class() {
    this.consume("class");
    let name = this.consume("id");
    let defs = [];
    while (!this.scan(".")) {
      defs.push(this.parse_def());
    }
    return { type: "class", name, defs };
  }
}

class Compiler {
  static prelude = Deno.readTextFileSync("./prelude.js");

  constructor(ast) {
    this.ast = ast;
  }

  eval() {
    let output = "";
    output += [
      ...new Set(
        this.ast
          .filter(({ type }) => type === "class")
          .map(({ name }) => `let ${name};`)
      ),
    ].join("\n");
    for (let node of ast) output += this.eval_node(node) + ";\n";
    return output;
  }

  eval_node(node) {
    if (node.type === "class") {
      return this.eval_class(node);
    } else if (node.type === "keyword") {
      return this.eval_keyword(node);
    } else if (node.type === "string") {
      return this.eval_string(node);
    } else if (node.type === "id") {
      return this.eval_id(node);
    } else if (node.type === "record_constructor") {
      return this.eval_record_constructor(node);
    } else if (node.type === "method_call") {
      return this.eval_method_call(node);
    } else if (node.type === "operator") {
      return this.eval_operator(node);
    } else if (node.type === "int") {
      return this.eval_int(node);
    } else {
      throw "unmatched case for eval node";
    }
  }

  eval_int({ value }) {
    return value;
  }

  eval_operator({ op }) {
    return `Pnt.operator['${op}']`;
  }

  eval_method_call({ lhs, args }) {
    return `Pnt.call(${this.eval_node(lhs)}, ${args
      .map((arg) => this.eval_node(arg))
      .join(", ")});`;
  }

  eval_record_constructor({ kw_args, class_name }) {
    // TODO: kw_args
    return `Pnt.construct(${class_name}, {${Object.entries(kw_args)
      .map(([name, value]) => `'${name}': ${this.eval_node(value)}`)
      .join(", ")}})`;
  }

  eval_id({ value }) {
    return value;
  }

  eval_string({ value }) {
    return `"${value}"`;
  }

  eval_keyword({ value }) {
    return `Pnt.keyword("${value}")`;
  }

  pattern_to_arg(node) {
    if (node.type === "keyword") {
      return `_${node.value}`;
    } else if (["string", "operator"].includes(node.type)) {
      // just garbage, js doesn't allow multiple _ vars
      return `_${(Math.random() * 100).toFixed(0)}`;
    } else if (node.type === "id") {
      return node.value;
    } else {
      throw "unknown pattern for arg";
    }
  }

  eval_pattern(node) {
    let literals = ["string", "keyword", "int", "float", "operator"];
    if (literals.includes(node.type)) {
      return this.eval_node(node);
    } else if (node.type === "id") {
      return "Pnt.patterns.id";
    } else {
      throw "invalid pattern";
    }
  }

  eval_def({ patterns, return_expr }) {
    return `{ patterns: [${patterns
      .map((pattern) => this.eval_pattern(pattern))
      .join(", ")}], fn: (self, ${patterns
      .map((pattern) => this.pattern_to_arg(pattern))
      .join(", ")}) => ${this.eval_node(return_expr)} }`;
  }

  eval_class({ name, defs }) {
    return `
${name} ||= {[Pnt.methods]: []};
${name}[Pnt.methods].push(${defs.map((def) => this.eval_def(def)).join(", ")})
    `;
  }
}

let program = `
class Point
  def + other -> other.
.

Point{x: 1, y: 2} + Point{x: 2, y: 3}.
`;

let tokens = new Tokenizer(program).tokenize();
let ast = new Parser(tokens).parse();
let output = new Compiler(ast).eval();
// console.log(tokens);
// console.log(ast);
// console.log(output);
console.log(eval(Compiler.prelude + output));
