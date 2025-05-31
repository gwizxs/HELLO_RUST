# API Documentation

# How to launch
```bash

# install
curl https://sh.rustup.rs -sSf | sh # on Mac Os or Linux
# run
cargo build
cargo run
```

# Requests  documentation


### Main 
```GET /```

```html request
Input:
{}

Output:
{
     "Hello world"
}
```

### 
```POST /todos```

### Add Todo

```html request
Input:
    {
        "title": "string" - name of todo
    }

Output:
{
"id": "id",
"title": "name",
"completed": boolean
}
```

### Get All Todo
```GET /get-todo```


```html request
Input:
[
{
"id": "id",
"title": "title",
"completed": boolean
}
]
```

### change todo
```GET /todos/{id}```


```html request
Input:
{
"title": "title",
"completed": boolean
}

Output: 
{
{
"id": "id",
"title": "title",
"completed": boolean
}
}
```
