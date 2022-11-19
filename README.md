# rustPasswordGenerator

a simple program that mixes random numbers, a simple special character and two things you would remember names.

for execution you should run 
`cargo run -- [any email here] [any file path for configuration file] [any file path for the resulting password]`

1)  in the test.json, aka config file, you can have any names and last names that you like to have in your password
an example here so you can understand
```
{
    "names": [
        "any name you want" 
    ],
    "lastNames": [
        "any last name you want"
    ]
}
```

2) by default result.json should contain your new password, as an example:
 ```
 {"password":"juanZambrano@1995","userName":"test"}
 ```
