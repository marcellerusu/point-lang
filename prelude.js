let Pnt = {
  proto: Symbol("proto"),
  properties: Symbol("properties"),
  methods: Symbol("methods"),
  patterns: {
    id() {},
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
  match(pattern, arg) {
    if (pattern === arg) {
      return true;
    } else if (pattern === Pnt.patterns.id) {
      return true;
    } else {
      return false;
    }
  },
  call(pnt_object, ...args) {
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
