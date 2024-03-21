const express = require("express");
const cors = require('cors');
const app = express();
app.use(cors());
app.use(express.json());

const PORT = process.env.PORT || 3000;


const adjectives = ["pretty", "large", "big", "small", "tall", "short", "long", "handsome", "plain", "quaint", "clean", "elegant", "easy", "angry", "crazy", "helpful", "mushy", "odd", "unsightly", "adorable", "important", "inexpensive", "cheap", "expensive", "fancy"];
const colours = ["red", "yellow", "blue", "green", "pink", "brown", "purple", "brown", "white", "black", "orange"];
const nouns = ["table", "chair", "house", "bbq", "desk", "car", "pony", "cookie", "sandwich", "burger", "pizza", "mouse", "keyboard"];

function getRandomNumber(max) {
    return Math.floor(Math.random() * max);
}

function buildData(count) {
    let data = [];
    for (let i = 0; i < count; i++)
        data.push({id: i+1, label: adjectives[getRandomNumber(adjectives.length)] + " " + colours[getRandomNumber(colours.length)] + " " + nouns[getRandomNumber(nouns.length)] });
    return data;
}

app.get('/data/:id', function (req, res) {
    const response = buildData(req.params.id)
    res.send(response);
  });

  app.listen(PORT, () => {
    console.log("Server Listening on PORT:", PORT);
  });