# moli
the minimalistic poli interpreter.

## syntax
aims to follow perfect poli syntax

basics
```
~ explicit type sets non-dynamic type
str a = r'a raw string'
a = "another string"

b = .1234
b = 'this one's dynamic'

~ function returning anything
add = [i32 a ->any]
  return [b ->i32] a + b

apply = [f, i32 b ->i32]
  return f(b)

add_10 = add(10)
sub_10 = ([i32 a][b ->i32] a - b)(10)

i32 twenty = apply(add_10, 10)
ten = apply(sub_10, twenty)
```

tables
```
table a = {
  1, 2, 3
  key: 'a named member'
}

b =
  1, 2, 3
  key: 'a named member'

c =
  fun?: [a, b] a + b
```
