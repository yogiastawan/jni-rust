package com.gix.rust_native

fun main() {

    val app = App()
    println(app.greeting)

    val greeting = app.helloFromRust("Gix")

    println(greeting)
}
