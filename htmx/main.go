package main

import (
	"encoding/json"
	"fmt"
	"html/template"
	"net/http"
	"path"
	"time"

	"github.com/gorilla/websocket"
	"github.com/rs/zerolog/log"
)

func main() {
	dir := path.Join("index.html")
	html, err := template.ParseFiles(dir)

	// Serve assets from /static/assets
	http.Handle("/static/assets/", http.StripPrefix("/static/assets/", http.FileServer(http.Dir("static/assets"))))
	http.HandleFunc("/", func(rw http.ResponseWriter, req *http.Request) {

		if err != nil {
			http.Error(rw, "parse file error", 400)
			return
		}

		content := map[string]string{"name": "john doe"}

		err = html.Execute(rw, content)

		if err != nil {
			http.Error(rw, "render file error", 400)
			return
		}

	})

	http.HandleFunc("/clicked", func(rw http.ResponseWriter, req *http.Request) {
		rw.Write([]byte("<div>devuelto del servidor</div>"))

	})

	http.HandleFunc("/validate", func(rw http.ResponseWriter, req *http.Request) {
		title := req.FormValue("title")
		log.Info().Str("title", title).Msg("validating user input")

	})
	http.HandleFunc("/store", func(rw http.ResponseWriter, req *http.Request) {
		title := req.FormValue("title")
		log.Info().Str("title", title).Msg("storing title...")
		rw.Write([]byte("<div>Stored...</div>"))
	})

	http.HandleFunc("/spin", func(rw http.ResponseWriter, req *http.Request) {
		log.Info().Msg("loading...")
		time.Sleep(time.Second * 2)
		log.Info().Msg("done")
		rw.Write([]byte("<div>My complex data...</div>"))
	})
	http.HandleFunc("/css", func(rw http.ResponseWriter, req *http.Request) {
		button := `<button style="color: red;" hx-delete="/account">sweet!!</button>`
		rw.Write([]byte(button))
	})

	http.HandleFunc("/account", func(rw http.ResponseWriter, req *http.Request) {
		if req.Method == "DELETE" {
			log.Info().Msg("deleting account")
			return
		}
		if req.Method == "PUT" {
			log.Info().Msg("updating account")
			return
		}
	})

	http.HandleFunc("/blog", func(rw http.ResponseWriter, req *http.Request) {
		body := `
			<h1> my first blog entry </h1>
			<p>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.</p>
		`
		rw.Write([]byte(body))
	})

	var upgrader = websocket.Upgrader{}
	var connections []*websocket.Conn
	http.HandleFunc("/chatroom", func(rw http.ResponseWriter, req *http.Request) {
		conn, err := upgrader.Upgrade(rw, req, nil)
		if err != nil {
			fmt.Println(err)
			return
		}
		connections = append(connections, conn)

		log.Info().Msg("Client connected")
		type Msg struct {
			ChatMessage string `json:"chat_message"`
		}

		go func() {
			for {
				_, p, err := conn.ReadMessage()
				if err != nil {
					log.Error().Err(err).Msg("failed to read message")
					return
				}
				var msg Msg
				json.Unmarshal(p, &msg)

				content := fmt.Sprintf(`
				<div hx-swap-oob="beforeend:#chat_room">
					<p>%s</p>
				</div>`, msg.ChatMessage)

				go func() {
					for _, c := range connections {
						if err := c.WriteMessage(websocket.TextMessage, []byte(content)); err != nil {
							log.Error().Err(err).Msg("failed to write message")
							return
						}
					}
				}()
			}
		}()
	})

	http.ListenAndServe(":3000", nil)
}
