package main

import (
	"fmt"
	"log"
	"io/ioutil"
)

import (
	"github.com/BurntSushi/toml"
	"github.com/gofiber/fiber/v2"
)

type Resource struct {
	Url		string `toml:"url"`
	Mime	string `toml:"mime"`
}

type ResourceDConfig struct {
	Enabled		bool `toml:"enabled"`
	ListenURL	string `toml:"listen_url"`
}

type Config struct {
	ResourceD 	ResourceDConfig `toml:"resourceD"`
	Resource map[string]Resource `toml:"resource"`
}

func main() {
	var conf Config
	
	data, err := ioutil.ReadFile("resourced.toml")
	if err != nil { panic(err) }

	a, err := toml.Decode(string(data), &conf)
	if err != nil { panic(err) }
	_ = a
	
	app := fiber.New()

	app.Get("/:id", func (c *fiber.Ctx) error {

		if ! conf.ResourceD.Enabled {
			return c.Status(fiber.StatusNotFound).SendString("Resource not found")
		}

		res, exists := conf.Resource[c.Params("id")]
		if ! exists {
			return c.Status(fiber.StatusNotFound).SendString("Resource not found")
		}

		return c.SendString(fmt.Sprintf("Id: %s\nUrl: %s\nType: %s", c.Params("id"), res.Url, res.Mime))
	})

	log.Fatal(app.Listen(conf.ResourceD.ListenURL))
}