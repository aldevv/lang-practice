package main

import "testing"

type MockDb struct{}

func (db *MockDb) GetState() string {
	return "changed"
}

func TestAddWorkspacesState(t *testing.T) {
	d := &Data{&MockDb{}}
	ws := []*Workspace{
		{state: "testing"},
		{state: "testing"},
	}

	// pero esta funcion llama a addState
	d.AddWorkspacesState(ws)

	ShowWorkspaces(ws)
}
