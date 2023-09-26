# identconv

Convenient macros that turns `Idents` or ident like
string literals into `Idents` or `Strings` by case converting.

## Macros

| Macro | Case | Output |
| ----- | --- | --- |
| `lower!()` | flatlowercase | `Ident` |
| `upper!()` | FLATUPPERCASE | `Ident` |
| `snake!()` | snake_case | `Ident` |
| `usnake!()` | UPPER_SNAKE_CASE | `Ident` |
| `camel!()` | camelCase | `Ident` |
| `pascal!()` | PascalCase | `Ident` |
| `kebab!()` | kebab-case | `Ident` |
| `ukebab!()` | UPPER-KEBAB-CASE | `Ident` |
| `train!()` | Train-Case | `Ident` |
| `lower_strify!()` | flatlowercase | `&'static str` |
| `upper_strify!()` | FLATUPPERCASE | `&'static str` |
| `snake_strify!()` | snake_case | `&'static str` |
| `usnake_strify!()` | UPPER_SNAKE_CASE | `&'static str` |
| `camel_strify!()` | camelCase | `&'static str` |
| `pascal_strify!()` | PascalCase | `&'static str` |
| `kebab_strify!()` | kebab-case | `&'static str` |
| `ukebab_strify!()` | UPPER-KEBAB-CASE | `&'static str` |
| `train_strify!()` | Train-Case | `&'static str` |
