
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
