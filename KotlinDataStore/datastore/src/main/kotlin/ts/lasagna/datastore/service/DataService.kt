package ts.lasagna.datastore.service

import org.springframework.stereotype.Service
import ts.lasagna.datastore.dto.DataDto

@Service
class DataService {
    private val adjectives = arrayOf(
        "pretty",
        "large",
        "big",
        "small",
        "tall",
        "short",
        "long",
        "handsome",
        "plain",
        "quaint",
        "clean",
        "elegant",
        "easy",
        "angry",
        "crazy",
        "helpful",
        "mushy",
        "odd",
        "unsightly",
        "adorable",
        "important",
        "inexpensive",
        "cheap",
        "expensive",
        "fancy"
    )

    private val colors = arrayOf(
        "red",
        "yellow",
        "blue",
        "green",
        "pink",
        "brown",
        "purple",
        "brown",
        "white",
        "black",
        "orange"
    )

    private val nouns = arrayOf(
        "table",
        "chair",
        "house",
        "bbq",
        "desk",
        "car",
        "pony",
        "cookie",
        "sandwich",
        "burger",
        "pizza",
        "mouse",
        "keyboard"
    )

    fun buildData(count: Int): List<DataDto> {
        var rowId = 1
        val result = mutableListOf<DataDto>()
        repeat(count) {
            val data = DataDto(
                ID = rowId++,
                Label = "${adjectives.random()} ${colors.random()} ${nouns.random()}"
            )
            result.add(data)
        }
        return result
    }
}