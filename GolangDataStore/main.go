package main

import (
	"fmt"
	"math/rand/v2"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

type Data struct {
	ID    int
	Label string
}

var adjectives = []string{"pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", "quaint", "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", "unsightly", "adorable", "important", "inexpensive", "cheap", "expensive", "fancy"}
var colours = []string{"red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black", "orange"}
var nouns = []string{"table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger", "pizza", "mouse", "keyboard"}

func buildData(count int) []Data {
	data := make([]Data, count)
	for i := 0; i < count; i++ {
		data[i] = Data{
			ID:    i + 1,
			Label: fmt.Sprintf("%s %s %s", adjectives[rand.IntN(len(adjectives))], colours[rand.IntN(len(colours))], nouns[rand.IntN(len(nouns))]),
		}
	}
	return data
}

func getDataEntries(c *gin.Context) {
	id := c.Param("id")
	idValue, _ := strconv.Atoi(id)
	c.Header("Access-Control-Allow-Origin", "*")
	c.IndentedJSON(http.StatusOK, buildData(idValue))
}

func main() {
	router := gin.Default()
	router.GET("/data/:id", getDataEntries)
	router.Run("localhost:8080")
}
