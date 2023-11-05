package main

import (
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
func (self Resource) Get() ([]byte, error) {
	return ioutil.ReadFile(self.Url[7:])
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
	
	app := fiber.New(fiber.Config {
		Prefork:		true,
		CaseSensitive:	false,
		StrictRouting:	true,
		ServerHeader:	"",
		AppName:		"blek! File resourceD",
	})

	app.Use(func (c *fiber.Ctx) error {
		if ! conf.ResourceD.Enabled {
			return c.Status(fiber.StatusNotFound).SendString("ResourceD is disabled")
		}
		return c.Next()
	})

	app.Get("/:id", func (c *fiber.Ctx) error {

		res, exists := conf.Resource[c.Params("id")]
		if ! exists {
			return c.Status(fiber.StatusNotFound).SendString("Resource not found")
		}

		data, err := res.Get()
		if err != nil {
			panic(err)
		}

		c.Response().Header.SetContentType(res.Mime)
		c.Response().Header.SetContentLength(len(data))
		return c.Send(data)
	})

	log.Fatal(app.Listen(conf.ResourceD.ListenURL))
}