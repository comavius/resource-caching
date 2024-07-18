package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
)

type ExecDirective struct {
	HockPath string   `json:"hook_path"`
	Args     []string `json:"args"`
}

type ExecResult struct {
	Stdout string `json:"stdout"`
	Stderr string `json:"stderr"`
}

func main() {
	// Listen for incoming HTTP requests
	args := os.Args[1:]
	if len(args) != 2 {
		panic("Usage: execjob_handler <port> <playground_path>")
	}
	port := args[0]
	playgroundPath := args[1]
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		// Decode the request
		var directive ExecDirective
		if err := json.NewDecoder(r.Body).Decode(&directive); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		fmt.Println("Received directive: ", directive)

		// Execute the command
		stdout, stderr, err := execCommand(directive.HockPath, directive.Args, playgroundPath)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		// Encode the response
		response := ExecResult{stdout, stderr}
		if err := json.NewEncoder(w).Encode(response); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
	})
	log.Fatal(http.ListenAndServe("localhost:"+port, nil))
}

func execCommand(hookPath string, args []string, playground_path string) (string, string, error) {
	// Execute the command
	cmd := exec.Command("sh", hookPath)
	cmd.Dir = playground_path
	cmd.Args = append(cmd.Args, args...)
	var stdout, stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		return "", "", err
	}
	return stdout.String(), stderr.String(), nil
}
