package ts.lasagna.datastore.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import ts.lasagna.datastore.dto.DataDto
import ts.lasagna.datastore.service.DataService

@RestController
@RequestMapping("/data")
class DataController (var dataService: DataService){
    @GetMapping("/{count}")
    fun getData(@PathVariable count: Int): List<DataDto> {
        return dataService.buildData(count);
    }
}
