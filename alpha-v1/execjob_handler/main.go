package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
)

type ExecDirective struct {
	HookResourceId string   `json:"hook_path"`
	ArgResourceIds []string `json:"args"`
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
		stdout, stderr, err := execCommand(directive.HookResourceId, directive.ArgResourceIds, playgroundPath)
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

func execCommand(HookResourceId string, ArgResourceIds []string, playground_path string) (string, string, error) {
	// Check path traversal
	if filepath.Clean(HookResourceId) != HookResourceId {
		return "", "", fmt.Errorf("invalid hook path")
	}
	for _, arg := range ArgResourceIds {
		if filepath.Clean(arg) != arg {
			return "", "", fmt.Errorf("invalid arg path")
		}
	}
	hook_path := filepath.Join(playground_path, HookResourceId)
	args := make([]string, len(ArgResourceIds))
	for i, arg := range ArgResourceIds {
		args[i] = filepath.Join(playground_path, arg)
	}
	// Execute the command
	cmd := exec.Command("sh", hook_path)
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
