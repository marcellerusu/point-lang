let Pnt = {
  proto: Symbol("proto"),
  class_name: Symbol("class_name"),
  properties: Symbol("properties"),
  methods: Symbol("methods"),
  operator: {
    "&&": Symbol("&&"),
    "||": Symbol("||"),
    "**": Symbol("**"),
    "*": Symbol("*"),
    "-": Symbol("-"),
    "+": Symbol("+"),
    "/": Symbol("/"),
    "%": Symbol("%"),
  },
  patterns: {
    id() {},
    record_constructor: function RecordConstructor(name, properties) {
      this.name = name;
      this.properties = properties;
    },
  },
  Keyword: class {
    constructor(name) {
      this.name = name;
    }
  },
  KEYWORDS: new Map([]),
  keyword(name) {
    if (this.KEYWORDS.has(name)) {
      return this.KEYWORDS.get(name);
    } else {
      this.KEYWORDS.set(name, new this.Keyword(name));
      return this.KEYWORDS.get(name);
    }
  },
  construct(class_def, properties) {
    return {
      [Pnt.proto]: class_def,
      [Pnt.properties]: properties,
    };
  },
  is_pnt_object(obj) {
    return Pnt.proto in obj && Pnt.properties in obj;
  },
  match(pattern, arg) {
    if (pattern === arg) {
      return true;
    } else if (pattern === Pnt.patterns.id) {
      return true;
    } else if (pattern instanceof this.patterns.record_constructor) {
      return (
        this.is_pnt_object(arg) &&
        arg[Pnt.proto][Pnt.class_name] === pattern.name &&
        pattern.properties.every((key) => key in arg[Pnt.properties])
      );
    } else {
      return false;
    }
  },
  call(pnt_object, ...args) {
    if (
      args.length === 1 &&
      args[0] instanceof this.Keyword &&
      typeof pnt_object[Pnt.properties][args[0].name] !== "undefined"
    ) {
      return pnt_object[Pnt.properties][args[0].name];
    }

    for (let { patterns, fn } of pnt_object[Pnt.proto][Pnt.methods]) {
      if (
        args.length === patterns.length &&
        args.every((arg, i) => Pnt.match(patterns[i], arg))
      ) {
        return fn(pnt_object, ...args);
      }
    }
    throw "no method found";
  },
};

let Int = {
  [Pnt.methods]: [
    {
      patterns: [Pnt.operator["+"], Pnt.patterns.id],
      fn: (self, _, { [Pnt.properties]: { value } }) =>
        Pnt.construct(Int, { value: self[Pnt.properties].value + value }),
    },
  ],
};
