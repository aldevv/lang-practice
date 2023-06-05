package main

import (
	"fmt"
)

type myDbClient interface {
	GetState() string
}

type MyDb struct{}

func (db *MyDb) GetState() string {
	return "active"
}

type Data struct {
	db myDbClient
}

type Workspace struct {
	state string
}

func (d *Data) AddWorkspacesState(ws []*Workspace) {
	fmt.Println("Entered AddWorkspacesState")
	for _, w := range ws {
		d.AddState(w)
	}
}

func (d *Data) AddState(w *Workspace) {
	state := d.db.GetState()
	w.state = state
}

func ShowWorkspaces(ws []*Workspace) {
	for _, w := range ws {
		fmt.Println(w.state)
	}
}

func main() {
	d := &Data{&MyDb{}}
	ws := []*Workspace{
		{state: "unknown"},
		{state: "unknown"},
	}

	d.AddWorkspacesState(ws)
	ShowWorkspaces(ws)
}
