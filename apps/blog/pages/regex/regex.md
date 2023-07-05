
## 如何实现一个正则引擎

```typescript

interface QuantifierBase {
  type: QuantifierType;
  value?: any;
  fewTimesAsPossible?: boolean; // add a ?
}

// * zero or more times
interface QuantifierZeroOrMore extends QuantifierBase {
  type: 'zeroOrMore';
}

// + one or more
interface QuantifierOneOrMore extends QuantifierBase {
  type: 'oneOrMore';
}

// ? zero or one
interface QuantifierZeroOrOne extends QuantifierBase {
  type: 'zeroOrOne';
}

// {1, 2}
interface QuantifierRange extends QuantifierBase {
  type: 'range';
  value: {
    min?: number;
    max?: number;
  }
}

type Quantifier = QuantifierZeroOrMore
| QuantifierOneOrMore
| QuantifierZeroOrOne
| QuantifierRange


type QuantifierType = keyof QuantifierMap

interface QuantifierMap {
  'zeroOrMore': QuantifierZeroOrMore,
  'oneOrMore': QuantifierOneOrMore,
  'zeroOrOne': QuantifierZeroOrMore,
  'range': QuantifierRange,
}

type NumAny = {
  type: 'any';
}

interface Token {
  expression: string;
  quantifier?: Quantifier;
}


const charQuantifierTypeMap: {
  [char: string]: QuantifierType
} = {
  '*': 'zeroOrMore',
  '+': 'oneOrMore',
  '?': 'zeroOrOne',
}
const quantifierChars = [
  ...Object.keys(charQuantifierTypeMap),
  '{' // range
]


let regex = 'a[a-z]+'
let index = 0
let char
let tokens = []
let previewToken

function addToken(token) {
  tokens.push(token)
  previewToken = token
}

function createQuantifier<T extends QuantifierType>(type: T, value?: any): QuantifierMap[T] {
  return {
    type,
    value,
  }
}

function parseQuantifier(): Quantifier {
  if (!previewToken) {
    throw new Error("nothing to repeat");
  }

  if (char === '{') {
    return parseRangeQuantifier()
  } else {
    return createQuantifier()
  }
}

// 解析范围
function parseRangeQuantifier() {
  let min
  let max
  let minFinished

  read((char) => {
    if (char === ',') {
      minFinished = true
    }

    if (char !== ' ') {
      if (minFinished) {
        max += char
      } else {
        min += char
      }
    }

    // { 5 }
    if (!minFinished) {
      max = min
    }

    return char !== '}'
  })

  return createQuantifier('range', {
    min,
    max
  })
}

function read(cb: () => boolean) {
  index++
  char = regex[index]
  if (!char) {
    return
  }

  if (cb && cb(char)) {
    read(cb)
  }
}

// parse
while(true) {
  char = regex[index]
  if (!char) {
    break
  }

  if (char === '[') { // 是否特殊符号

  } else if (quantifierChars.incudes(char)) {
    parseQuantifier()
  } else {
    addToken(char)
  }

  index++
}

// match
```
