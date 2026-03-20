```json
{
  "domain": "OrderManagement",
  "version": "2.0.0",
  "supersedes": ["1.0.0", "1.1.0"],
  "initial_state": "Draft",

  "migration_rules": [
    {
      "from_state": "Pending",
      "to_state": "AwaitingTaxInfo",
      "guards": [{ "type": "MissingField", "path": "entity.TaxID" }],
      "transforms": [
        { "target": "entity.LegacyMode", "value": true }
      ]
    }
  ],

  "states": [
    {
      "name": "AwaitingPayment",
      "transitions": [
        {
          "action": "SubmitPayment",
          "to": "Processing",
          "priority": 1,
          "guards": [
            { 
              "type": "OccurredWithin", 
              "target_action": "CardValidation", 
              "window_seconds": 3600 
            }
          ],
          "effects": [
            {
              "type": "CrossDomain",
              "target": "Accounting",
              "action": "Debit",
              "on_success": "Confirm",
              "on_failure": "Reject",
              "on_timeout": "HandleStale"
            }
          ]
        }
      ]
    }
  ]
}
```

```json
{
  "process": "LoanApproval",
  "version": "2.1.0",
  "initial_state": "Draft",
  "transitions": [
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path A: Auto-Approve small loans for premium members",
      "to": "Approved",
      "priority": 10,
      "guards": [
        { "type": "LessThan", "path": "entity.Amount", "value": 1000 },
        { "type": "IsEqual", "path": "ext.MemberTier", "value": "Premium" }
      ],
      "effects": ["NotifyCustomer", "TriggerPayout"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path B: Escalate large loans or high-risk cases",
      "to": "PendingVP",
      "priority": 5,
      "guards": [
        { "type": "GreaterThan", "path": "entity.Amount", "value": 50000 }
      ],
      "effects": ["EmailVP", "LogHighValueRisk"]
    },
    {
      "from": "Submitted",
      "action": "Evaluate",
      "comment": "Path C: Default path for standard review",
      "to": "PendingManager",
      "priority": 1,
      "guards": [], 
      "effects": ["AssignToQueue"]
    },
    {
      "from": "PendingManager",
      "action": "Approve",
      "comment": "Temporal Guard: Requires a FraudCheck event in the last 24h",
      "to": "Approved",
      "guards": [
        { 
          "type": "OccurredWithin", 
          "target_action": "FraudCheck", 
          "window_seconds": 86400 
        }
      ]
    }
  ]
}
```

```json
"effects": [
  {
    "type": "WebHook",
    "params": {
      "url": "https://api.partner.com/notify",
      "payload_field": "rec.order_id"
    }
  }
]
```