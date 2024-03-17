import kotlinx.browser.document
import kotlinx.browser.window
import kotlinx.serialization.json.Json
import org.w3c.dom.Element
import org.w3c.dom.HTMLInputElement


class App(private var rowId: Int = 1) {


    fun setupListeners() {
        val generateBtn = document.getElementById("generate-data");
        val clearBtn = document.getElementById("clear");
        val entryCount: HTMLInputElement = document.getElementById("entry-count") as HTMLInputElement;
        val dataItems = document.getElementById("data-items");


        generateBtn?.addEventListener("click") {
            val count = entryCount.value;
            window.fetch("http://localhost:8080/data/$count").then {
                if (it.ok) {
                    it.text().then {
                        try {
                            val entries = Json.decodeFromString<List<Entry>>(it.toString())
                            for (entry in entries) {
                                dataItems?.appendChild(getDataItem(entry))
                            }
                        } catch (exception: Exception) {
                            println("${exception.message}")
                        }
                        null
                    }
                }
                null
            }
        }

        clearBtn?.addEventListener("click") {
            dataItems?.textContent = ""
        }
    }

    private fun getDataItem(entry: Entry): Element {
        val li = document.createElement("li");
        li.innerHTML = "<span>${entry.id}</span> <span>${entry.label}</span>"
        return li;
    }
}