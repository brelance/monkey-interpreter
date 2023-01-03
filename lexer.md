# Monkey-Interpreter


## lexer

monky code
```
let five = 5;
let len = 10;
let add = fn (x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;

```
将代码解析为token流
实现词法解析器
分类 -> 解析 