# Markdown Parser Testing

> foo > [0] = 1

This is a simple Markdown document that you can use 

    to test your Markdown parser.

## Basic Formatting

*Italic Text* or _Italic Text_
**Bold Text** or __Bold Text__
~~Strikethrough Text~~

> foo > [1] > bar = 2

## Headings

# Heading 1
## Heading 2
### Heading 3
> foo > [1] > baz = 3.0

#### Heading 4
##### Heading 5
###### Heading 6

## Lists

Unordered List:
- Item 1
- Item 2
  - Subitem 2.1
  - Subitem 2.2
- Item 3
> foo > [2] = "4"

Ordered List:
1. First Item
2. Second Item
3. Third Item

## Links and Images

[Link to Google](https://www.google.com)
![Image Alt Text](https://via.placeholder.com/150)

## Code

Inline `code` can be inserted.
```
// Code block
function greet(name) {
console.log(Hello, ${name}!);
}
```

## Horizontal Line
---

> foo > [3] = `five`

## Tables

| Header 1 | Header 2 |
|----------|----------|
| Row 1, Col 1 | Row 1, Col 2 |
| Row 2, Col 1 | Row 2, Col 2 |

> foo > [4] = [6]

## Escaping

You can escape Markdown characters like \*italic\* or \[link\] using backslashes.

## References

[Markdown Guide](https://www.markdownguide.org/)

Feel free to modify and use this Markdown text to test your parser.