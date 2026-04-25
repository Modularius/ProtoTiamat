PUT _index_template/user
{
  "priority": 0,
  "template": {
    "settings": {
      "index": {
        "mapping": {
          "nested_fields": {
            "limit": "50"
          }
        },
        "number_of_shards": "1",
        "requests": {
          "cache": {
            "enable": "true"
          }
        },
        "sort": {
          "field": [
            "user_id",
          ],
          "order": [
            "desc",
          ]
        },
        "number_of_replicas": "0"
      }
    },
    "mappings": {
      "dynamic_templates": [
        {
          "span_tags_map": {
            "path_match": "tag.*",
            "mapping": {
              "ignore_above": 256,
              "type": "keyword"
            }
          }
        }
      ],
      "properties": {
        "user_id": {
          "ignore_above": 256,
          "type": "keyword"
        },
        "name": {
          "ignore_above": 256,
          "type": "string"
        },
        "date-joined": {
          "ignore_above": 256,
          "type": "timestamp_ns"
        },
        "groups": {
          "ignore_above": 256,
          "type": "keyword"
        },
        "friends": {
          "ignore_above": 256,
          "type": "keyword"
        },
        "tags": {
          "dynamic": false,
          "type": "nested",
          "properties": {
            "type": {
              "ignore_above": 256,
              "type": "keyword"
            },
            "value": {
              "ignore_above": 256,
              "type": "keyword"
            },
            "key": {
              "ignore_above": 256,
              "type": "keyword"
            }
          }
        },
        "logs": {
          "dynamic": false,
          "type": "nested",
          "properties": {
            "fields": {
              "dynamic": false,
              "type": "nested",
              "properties": {
                "type": {
                  "ignore_above": 256,
                  "type": "keyword"
                },
                "value": {
                  "ignore_above": 256,
                  "type": "keyword"
                },
                "key": {
                  "ignore_above": 256,
                  "type": "keyword"
                }
              }
            },
            "timestamp": {
              "type": "long"
            }
          }
        }
      }
    }
  },
  "index_patterns": [
    "jaeger-span-*"
  ],
  "composed_of": [],
  "ignore_missing_component_templates": []
}