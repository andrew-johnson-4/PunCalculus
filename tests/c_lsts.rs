
//plural applications can be categorized into splits, merges, carries, and narrows
//splits turn singular types into plural types
//merges turn plural types into singular types
//carries apply pluralities of functions and values
//narrows discard some lines of computation entirely

//narrow application argument
//(f: A -> B)(x: A + C): B

//narrow application function
//(f: A -> B + C -> D)(x: A): B

//split application
//(f: A -> B + A -> C)(x: A): B + C

//merge application
//(f: A -> C + B -> C)(x: B + C): C

//carry application
//(F: A -> B + C -> D)(x: A + C): B + D
