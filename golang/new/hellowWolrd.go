package main

import (
        "fmt"
        "os"
        "strings"
)

func main() {

        fmt.Println("Hello, World")
        
        for i := 0; i <  len(os.Args); i++ {
                fmt.Println(os.Args[i])
        }

        var str= "->" 
        for  _, item := range os.Args[1:] {
                fmt.Println(item + str)
        }

        fmt.Println(os.Args[1:])
        fmt.Println(strings.Join(os.Args[1:],"<<"))

}