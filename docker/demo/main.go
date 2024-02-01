package main

import "net/http"

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("hi docker"))
	})

	http.ListenAndServe("0.0.0.0:8899", nil)
}
