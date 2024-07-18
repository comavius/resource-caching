package main

import (
	"encoding/json"
	"log"
	"net/http"
	"os"
	"path/filepath"
)

type StorageContent struct {
	Content string `json:"content"`
}

func main() {
	args := os.Args[1:]
	if len(args) != 2 {
		panic("Usage: mock_object_storage <port> <mock_storage_path>")
	}
	port := args[0]
	mockStoragePath := args[1]
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		query := r.URL.Query()
		key_name := query.Get("key")
		if key_name == "" {
			http.Error(w, "key not provided", http.StatusBadRequest)
			return
		}
		// check if key can not do path traversal
		clean_key := filepath.Clean(key_name)
		if clean_key != key_name {
			http.Error(w, "invalid key", http.StatusBadRequest)
			return
		}
		// check if key is not empty
		if clean_key == "" {
			http.Error(w, "invalid key", http.StatusBadRequest)
			return
		}
		// fetch the content
		path := filepath.Join(mockStoragePath, clean_key)
		// open the file
		file, err := os.Open(path)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		defer file.Close()
		// read the content
		content, err := os.ReadFile(path)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		// send the content
		response := StorageContent{string(content)}
		if err := json.NewEncoder(w).Encode(response); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	})

	log.Fatal(http.ListenAndServe("localhost:"+port, nil))
}
