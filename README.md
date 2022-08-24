<p align="center"><img height="200" src="assets/cypher_logo.png" alt="go-excel"></p>

<h1 align="center">Neo4j Query Builder</h1>
<p align="center">
    <a href="https://opensource.org/licenses/BSD-3-Clause">
        <img src="https://img.shields.io/github/license/I0HuKc/cypher" alt="Licenses">
    </a>
    <a href="https://t.me/I0HuKc">
        <img src="https://img.shields.io/github/repo-size/I0HuKc/cypher" alt="Size">
    </a>
    <a href="https://t.me/I0HuKc">
        <img src="https://img.shields.io/badge/I0HuKc-Telegram-blue" alt="Telegram">
    </a> 
</p>

A flexible and intuitive query builder for Neo4j and Cypher. Write queries in Rust just as you would write them in Cypher.

## Installation

```toml
cypher = { version = "0.1.0", features=["derive"] }
```


## Usage

### Container attributes

* **#[cypher(rename = "...")]**

This attribute can be used when in your code you wanted to have the name **A**, but in Neo4j you wanted to save the structure as a node with name **B**.


### Field attributes

* **#[cypher(rename = "...")]**

    Rename attribute also can used as s field attribute. The principle of it's operation is similar to that described above. It should be used when you want to automatically change the field name when you build a query with a properties object of Neo4j entity.

* **#[cypher(skip)]**

    Use this attribute when you what to hide some struct field when you build a query.

* **#[cypher(label)]**

    You can use such attribute if you want the field value to be used as the label of the node.
    It is recommended to use enums as a value of such field. 

* **#[cypher(default)]**

    If the field type is some kind of `Option<T>` you can use this att and when field value will be `None`, default value for this type will be set. 

    ***Attention***: For the type that is used with the simple attribute **default**, there must be a mandatory implementation of the trait `Default`

* **#[cypher(default = "...")]**

    If using a default value for the whole type doesn't work for you, you can use the default value for a one field. The default value should always be specified as a string, but it will be cast to the required type when the query is generated.

### Entities

* **Node**

    Create node:

    ```rust
    let node = Entity::node("n", "Profile", None, None);
    ```

    But you can set derive attribute to the struct `#[derive(Debug, Clone, CypQueSet)]` and node will be automatically generated!

* **Relation**
  
    Create relation:

    ```rust
    let rel = Entity::rel("n1", "n2", "TEST", None);
    ```

### Builder

The query builder methods can be represented as a tree like this:

```
.
├── create
│   └── return
│       ├── as
│       │   ├── limit
│       │   │   └── finalize
│       │   └── skip
│       │       └── finalize
│       ├── limit
│       │   ├── skip
│       │   │   └── finalize
│       │   └── finalize
│       ├── skip
│       │   └── finalize
│       └── finalize
│
└── match
    └── where
        ├── and
        │   └── ...
        ├── or
        │   └── ...
        ├── set
        │   └── return
        │       └── ...
        ├── create
        │   └── ...
        ├── match
        │   └── ...
        └── delete
            └── return
                └── ...
```

### Example

#### Node

```rust
use std::fmt::Display;

use cypher::query::match_query::CompOper;
use cypher::CypQueSet;

/// Example of access levels in the system
#[derive(Debug, Clone)]
enum Perm {
    Admin,
    User,
}

/// An example of a structure that should be converted into a Neo4j node.
/// 
/// In your code you use name Account and the Neo4j node label will be Profile.
/// Field `username` will be renamed;
/// Field `secret` will be hidden;
/// Field `perm` will be used as second a node label;
/// Field `level` and `friends` will used a default value if they will be None;
#[derive(Debug, Clone, CypQueSet)]
#[cypher(rename = "Profile")]
struct Account {
    #[cypher(rename = "name")]
    username: String,
    password: String,
    age: i32,
    status: Option<String>,
    online: bool,
    #[cypher(skip)]
    secret: u8,
    #[cypher(label)]
    perm: Perm,
    #[cypher(default = "5")]
    level: Option<u8>,
    #[cypher(default = "['Bob', 'Tom']")]
    friends: Option<Vec<String>>,
}

fn main() {
    // Init some example struct
    let data = Account {
        username: String::from("mi1fhunter"),
        password: String::from("1234f4321"),
        age: 32,
        status: None,
        online: false,
        secret: 1,
        perm: Perm::User,
        level: None,
        friends: Some(vec![
            "Bob".to_string(),
            "Tom".to_string(),
            "Sam".to_string(),
        ]),
    };

    // Let's build some query
    let query = Query::new()
        .create(vec![model.into_entity("n")])
        .r#return("n", None)
        .finalize();

    println!("{}", q);
}
```

So, the query builder automatically generated such query for you:

```sql
CREATE (n:Profile { password: '1234f4321',level: 5,name: 'mi1fhunter',age: 32,friends: ['Bob','Tom','Sam'],online: false })
SET n:User
RETURN n
```


Example of creating a match query:

```rust
let query = Query::new()
    .r#match(model.into_entity("n"))
    .r#where("age", CompOper::Equal, PropType::int(40))
    .or("age", CompOper::Equal, PropType::int(23))
    .and("name", CompOper::Equal, PropType::str("admin"))
    .r#return("n", None)
    .finalize();
```

The result will be like this: 

```sql
MATCH (n:Profile)
WHERE n.age = 40 OR n.age = 23 AND n.name = 'admin' 
RETURN n
```


If you need to return the value of some propertie or get another `var` name, you can write it like this:

```rust
let query = Query::new()
    .create(vec![model.into_entity("n")])
    .r#return("n", Some("age"))
    .finalize();
```

The result will be:

```sql
CREATE (n:Profile { level: 5,age: 32,friends: ['Bob','Tom','Sam'],uname: 'mi1fhunter',password: '1234f4321',online: false })
SET n:User
RETURN n.age

```

OR

```rust
let query = Query::new()
    .create(vec![model.into_entity("n")])
    .r#return("n", None)
    .r#as("node")
    .finalize();
```

Result:

```sql
CREATE (n:Profile { age: 32,uname: 'mi1fhunter',online: false,level: 5,friends: ['Bob','Tom','Sam'],password: '1234f4321' })
SET n:User
RETURN n AS node
```

#### Relation

```rust 
// Of course, instead of None, you can specify an object `Props`.
let rel1 = Entity::rel("n1", "n2", ":SUBSCRIBE", None);
let rel2 = Entity::rel("n2", "n1", ":SUBSCRIBE", None);

let query = Query::init()
    .r#match(&model.into_entity("n1"))
    .r#where("age", CompOper::Equal, PropType::int(1))        
    .r#match(&model.into_entity("n1"))
    .r#where("age", CompOper::Equal, PropType::int(10))
    .create(vec![&rel, &re2])
    .finalize();
```

Result:

```sql
MATCH (n1:Test) WHERE n1.age = 1 AND n1.level = 10 
MATCH (n1:Test) WHERE n1.age = 10
CREATE (n1)-[:TEST]->(n2),
        (n2)-[::SUBSCRIBE]->(n1)
```