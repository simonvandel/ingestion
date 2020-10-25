package dk.millipede.ingestion

import dk.millipede.ingestion.streams.TestStream
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class IngestionApplication

fun main(args: Array<String>) {
    runApplication<IngestionApplication>(*args)

    TestStream().test()
}
