
```json
{
  "migration_rules": [
    {
      "from_state": "Legacy_Pending",
      "to_state": "Active_AwaitingReview",
      "transforms": [
        {
          "op": "move",
          "from": "old_total",
          "to": "rec.order.total_amount"
        },
        {
          "op": "set",
          "target": "sys.migration_timestamp",
          "value": "now"
        },
        {
          "op": "map",
          "target": "rec.order.priority",
          "from": "legacy_rank",
          "mapping": { "1": "High", "2": "Medium", "3": "Low" }
         },
         {
           "from_state": "Legacy_User",
           "to_state": "Active_User",
           "conditional_transforms": [
           {
              "guards": [
                 { "type": "GreaterThan", "path": "rec.points", "value": 1000 }
               ],
               "ops": [
                 { "op": "set", "target": "rec.tier", "value": "Gold" }
               ]
           }]
         }
      ]
    }
  ]
}
```