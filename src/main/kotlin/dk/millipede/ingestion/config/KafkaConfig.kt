package dk.millipede.ingestion.config

import org.apache.kafka.common.serialization.Serdes
import org.apache.kafka.streams.StreamsConfig
import org.springframework.context.annotation.Configuration
import java.util.*

@Configuration
class KafkaConfig {

    fun getConfig(): Properties {
        val props = Properties()

        props.putAll(
                mapOf(
                        StreamsConfig.APPLICATION_ID_CONFIG to "app-id",
                        StreamsConfig.BOOTSTRAP_SERVERS_CONFIG to "kafka.default.svc.cluster.local:9092",
                        StreamsConfig.DEFAULT_KEY_SERDE_CLASS_CONFIG to Serdes.String().javaClass,
                        StreamsConfig.DEFAULT_VALUE_SERDE_CLASS_CONFIG to Serdes.String().javaClass
                )
        )
        return props
    }

}