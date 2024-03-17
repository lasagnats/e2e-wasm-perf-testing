
import kotlinx.browser.document
import kotlinx.browser.window
import kotlinx.serialization.json.Json
import org.w3c.dom.HTMLInputElement


class App(private var rowId: Int = 1) {



    fun setupListeners() {
        val generateBtn = document.getElementById("generate-data");
        val clearBtn = document.getElementById("clear");
        val entryCount:HTMLInputElement = document.getElementById("entry-count") as HTMLInputElement;
        val dataItems = document.getElementById("data-items");


        generateBtn?.addEventListener("click") {
            val count = entryCount.value;
            window.fetch("http://localhost:8080/data/$count").then {
                if (it.ok) {
                    it.text().then {
                        try {
                            val obj = Json.decodeFromString<List<Entry>>(it.toString())
                            println(obj.size) // MyModel(a=42, b="42")
                        } catch (exception: Exception) {
                            println("${exception.message}")
                        }

//                        for (row in rows) {
//                                table?.appendChild(getTableRow(row))
//                            }
//                        println(it)
                        null
                    }

                }
                null
            }
//            val rows: List<Entry> = buildData();
//            for (row in rows) {
//                table?.appendChild(getTableRow(row))
//            }
        }

//        addBtn?.addEventListener("click") {
//            val rows: List<Entry> = buildData();
//            for (row in rows) {
//                table?.appendChild(getTableRow(row))
//            }
//        }
//
//        runLotsBtn?.addEventListener("click") {
//            table?.textContent = ""
//            val rows: List<Entry> = buildData(10000);
//            for (row in rows) {
//                table?.appendChild(getTableRow(row))
//            }
//        }

        clearBtn?.addEventListener("click") {
            dataItems?.textContent = ""
        }



    }

//    private fun getTableRow(entry: Entry): Element {
//        val tr = document.createElement("tr");
//        val rowId = "entry" + entry.id.toString();
//        tr.id = rowId;
//        tr.innerHTML =
//            "<td class='col-md-1'></td><td class='col-md-4'><a class='lbl'></a></td><td class='col-md-1'><a class='remove'><span class='remove glyphicon glyphicon-remove' aria-hidden='true'></span></a></td><td class='col-md-6'></td>";
//        tr.getElementsByClassName("remove").get(0)?.addEventListener("click") {
//            delete(entry.id)
//        }
//        tr.addEventListener("click") {
//
//            unselect();
//            selectedRowID = rowId;
//            tr.classList.add("danger")
//        }
//        val td1 = tr.firstChild;
//        val a2 = td1?.nextSibling?.firstChild;
//
//        td1?.textContent = entry.id.toString()
//        a2?.textContent = entry.label
//
//        return tr
//    }


}