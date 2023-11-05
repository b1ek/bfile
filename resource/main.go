package main

import (
	"log"
	"fmt"
	"regexp"
	"strings"
	"net/http"
	"io/ioutil"
)

import (
	"github.com/BurntSushi/toml"
	"github.com/gofiber/fiber/v2"
	"github.com/dustin/go-humanize"
)

type Resource struct {
	Url		string 	`toml:"url"`
	Proxied	bool	`toml:"proxied",default:false`
	Mime	string 	`toml:"mime"`
}
func (self Resource) Get() ([]byte, error) {
	return ioutil.ReadFile(self.Url[7:])
}

type ResourceDConfig struct {
	Enabled				bool 	`toml:"enabled"`
	ListenURL			string 	`toml:"listen_url"`
	ProxyCacheMinSize	string	`toml:"proxy_cache_min_size",default:5MB`
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
func (self Resource) GetProxied() ([]byte, error) {

	cached, exists := ProxyResourceCache[self.Url];
	if exists {
		return cached, nil
	}

	resp, err := http.Get(self.Url)
	if err != nil { return make([]byte, 0, 0), err }

	buf, err := ioutil.ReadAll(resp.Body)
	if err != nil { return make([]byte, 0, 0), err }

	// cache only those that are less than 5 mb
	if len(buf) > ProxyCacheMinSize {
		ProxyResourceCache[self.Url] = buf
	}
	
	return buf, nil
}

var ProxyResourceCache map[string][]byte = make(map[string][]byte)
var ProxyCacheMinSize int

func main() {
	var conf Config
	
	data, err := ioutil.ReadFile("resourced.toml")
	if err != nil { panic(err) }

	a, err := toml.Decode(string(data), &conf)
	if err != nil { panic(err) }
	_ = a

	cache_min, err := humanize.ParseBytes(conf.ResourceD.ProxyCacheMinSize)
	if err != nil { panic(err) }
	ProxyCacheMinSize = int(cache_min)

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

			if res.Proxied {
				data, err := res.GetProxied()
				if err != nil {
					log.Fatalln(err)
					// we failed, send a redirect instead
					// (next executed line would be 115)
				} else {
					c.Response().Header.SetContentType(res.Mime)
					c.Response().Header.SetContentLength(len(data))
					return c.Send(data)
				}
			}
			
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