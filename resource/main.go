package main

import (
	"log"
	"fmt"
	"regexp"
	"strings"
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

func (self Config) Validate() int {
	re, err := regexp.Compile(`^(file|http(s|))://`)
	if err != nil { panic(err) }

	for key, res := range self.Resource {
		if ! re.MatchString(res.Url) {
			panic(fmt.Sprintf("Resource %s has invalid URL: %s\nOnly file://, http:// and https:// URLs are allowed", key, res.Url))
		}
	}

	return 0
}

func main() {
	var conf Config
	
	data, err := ioutil.ReadFile("resourced.toml")
	if err != nil { panic(err) }

	a, err := toml.Decode(string(data), &conf)
	if err != nil { panic(err) }
	_ = a

	conf.Validate()
	
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

		if ! strings.HasPrefix(res.Url, "file://") {
			c.Location(res.Url)
			c.Status(302)
			return nil
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