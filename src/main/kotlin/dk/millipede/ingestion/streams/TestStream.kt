package dk.millipede.ingestion.streams

import dk.millipede.ingestion.config.KafkaConfig
import org.apache.kafka.streams.KafkaStreams
import org.apache.kafka.streams.StreamsBuilder
import org.springframework.stereotype.Service

@Service
class TestStream {

    fun test() {

        val builder = StreamsBuilder()
        val stream = builder.stream<String, String>("test")
        stream.foreach { _, value ->
            println(value)
        }

        val config = KafkaConfig().getConfig()
        KafkaStreams(builder.build(), config).start()
    }
}
